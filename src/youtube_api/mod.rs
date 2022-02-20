// External imports
use async_recursion::async_recursion;
use bson::{doc, Document};
use futures::StreamExt;
use reqwest;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json;
use url::{ParseError, Url};

use crate::api_service::Channel;
use crate::api_service::Location;
use crate::api_service::Video;
use crate::api_service::YoutubeTopic;
// External constructors
extern crate serde;

pub struct YoutubeApi {
    api_key: String,
}

impl YoutubeApi {
    pub fn new(api_key: String) -> YoutubeApi {
        YoutubeApi { api_key }
    }

    fn rem_first_and_last(value: &str) -> &str {
        if value != "null" && value.ends_with('"') && value.starts_with('"') {
            let mut chars = value.chars();
            chars.next();
            chars.next_back();
            chars.as_str()
        } else {
            return value;
        }
    }

    pub fn check_url(&self, channel_url: &String) -> Result<bool, ParseError> {
        match Url::parse(channel_url) {
            Ok(url) => {
                if (url.host_str() == Some("www.youtube.com")) {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            Err(e) => {
                println!("Could not parse '{}'. {}.", channel_url, e);
                return Err(e);
            }
        }
    }

    // Get Channel data by URL
    pub async fn get_channel_data(
        &self,
        channel_url: &String,
        client: &reqwest::Client,
    ) -> Result<Channel, reqwest::Error> {
        let channel_identifier: String;
        let mut channel_identifier_is_id = false;

        if (channel_url.contains("/channel/")) {
            channel_identifier_is_id = true;
            channel_identifier = channel_url.split("/channel/").last().unwrap().to_string();
        } else {
            channel_identifier = channel_url.split("/c/").last().unwrap().to_string();
        }
        // https://blog.logrocket.com/making-http-requests-rust-reqwest/

        let response: Result<String, reqwest::Error>;

        if channel_identifier_is_id {
            response = client
                .get("https://youtube.googleapis.com/youtube/v3/channels")
                .header(CONTENT_TYPE, "application/json")
                .header(ACCEPT, "application/json")
                .query(&[
                    ("part", "contentDetails,brandingSettings,contentOwnerDetails,localizations,snippet,statistics,status,topicDetails".to_string()),
                    ("key", self.api_key.to_string()),
                    ("id", channel_identifier)
                ])
                .send()
                .await
                .unwrap()
                .text()
                .await;
        } else {
            response = client
                .get("https://youtube.googleapis.com/youtube/v3/channels")
                .header(CONTENT_TYPE, "application/json")
                .header(ACCEPT, "application/json")
                .query(&[
                    ("part", "contentDetails,brandingSettings,contentOwnerDetails,localizations,snippet,statistics,status,topicDetails".to_string()),
                    ("key", self.api_key.to_string()),
                    ("forUsername", channel_identifier)
                ])
                .send()
                .await
                .unwrap()
                .text()
                .await;
        }
        match response {
            Ok(response) => {
                let parsed_response: serde_json::Value =
                    serde_json::from_str(&response.to_string()).unwrap();

                let subscriber_count_string = parsed_response["items"][0]["statistics"]
                    ["subscriberCount"]
                    .to_string()
                    .replace('"', "");
                let subscriber_count: u32;
                if subscriber_count_string != "null" {
                    subscriber_count = subscriber_count_string.parse::<u32>().unwrap();
                } else {
                    subscriber_count = 0;
                }

                let mut topic_vector: Vec<YoutubeTopic> = Vec::new();
                let mut i = 0;
                while i < parsed_response["items"][0]["topicDetails"]["topicIds"]
                    .as_array()
                    .unwrap()
                    .len()
                {
                    topic_vector.push(YoutubeTopic::new(
                        parsed_response["items"][0]["topicDetails"]["topicIds"][i].to_string(),
                        parsed_response["items"][0]["topicDetails"]["topicCategories"][i]
                            .to_string(),
                    ));
                    i = i + 1;
                }

                /* println!(
                    "topicDetails{:?}",
                    parsed_response["items"][0]["topicDetails"]
                ); */

                let channel = Channel::new(
                    YoutubeApi::rem_first_and_last(&parsed_response["items"][0]["id"].to_string())
                        .to_string(),
                    YoutubeApi::rem_first_and_last(
                        &parsed_response["items"][0]["snippet"]["title"].to_string(),
                    )
                    .to_string(),
                    YoutubeApi::rem_first_and_last(
                        &parsed_response["items"][0]["snippet"]["description"].to_string(),
                    )
                    .to_string(),
                    YoutubeApi::rem_first_and_last(
                        &parsed_response["items"][0]["snippet"]["thumbnails"]["default"]["url"]
                            .to_string(),
                    )
                    .to_string(),
                    YoutubeApi::rem_first_and_last(
                        &parsed_response["items"][0]["brandingSettings"]["image"]
                            ["bannerExternalUrl"]
                            .to_string(),
                    )
                    .to_string(),
                    YoutubeApi::rem_first_and_last(
                        &parsed_response["items"][0]["snippet"]["country"].to_string(),
                    )
                    .to_string(),
                    YoutubeApi::rem_first_and_last(
                        &parsed_response["items"][0]["contentDetails"]["relatedPlaylists"]
                            ["uploads"]
                            .to_string(),
                    )
                    .to_string(),
                    subscriber_count,
                    topic_vector,
                    YoutubeApi::rem_first_and_last(
                        &parsed_response["items"][0]["brandingSettings"]["channel"]["keywords"]
                            .to_string(),
                    )
                    .to_string(),
                    YoutubeApi::rem_first_and_last(
                        &parsed_response["items"][0]["brandingSettings"]["channel"]
                            ["unsubscribedTrailer"]
                            .to_string(),
                    )
                    .to_string(),
                    parsed_response["items"][0]["status"]["madeForKids"]
                        .to_string()
                        .parse::<bool>()
                        .unwrap(),
                    "LOADING".to_string(),
                    Vec::new()
                );
                return Ok(channel);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    /* pub async fn get_videos(
        &self,
        playlist_id: &String,
        video_amount: u16,
        client: &reqwest::Client,
    ) -> bool {
        let videos_ids_result = YoutubeApi::get_playlist_videos(
            &self,
            playlist_id,
            "FIRST_PAGE".to_string(),
            video_amount,
            client,
        )
        .await;

        match videos_ids {
            Ok(video_ids) => for video_id in video_ids.into_iter() {},
            Err(e) => {}
        }

        return true;
    } */

    /* pub async fn get_video(&self, video_id: &String) -> Result<Video, reqwest::Error> {

    } */

    #[async_recursion]
    pub async fn get_playlist_videos(
        &self,
        playlist_id: &String,
        page_token: String,
        mut open_video_amount: u16,
        client: &reqwest::Client
    ) -> Result<Vec<Video>, reqwest::Error> {
        let mut max_results: u16 = 50;
        if open_video_amount <= max_results {
            max_results = open_video_amount;
            open_video_amount = 0;
        } else if open_video_amount > max_results {
            open_video_amount = open_video_amount - max_results;
        }
        let response_video_ids = client
            .get("https://youtube.googleapis.com/youtube/v3/playlistItems")
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .query(&[
                ("part", "contentDetails".to_string()),
                ("key", self.api_key.to_string()),
                ("playlistId", playlist_id.to_string()),
                (if page_token != "FIRST_PAGE" {
                    ("pageToken", page_token)
                } else {
                    ("maxResults", max_results.to_string())
                }),
                ("maxResults", max_results.to_string()),
            ])
            .send()
            .await
            .unwrap()
            .text()
            .await;
            
        match response_video_ids {
            Ok(response_video_ids) => {
                let parsed_response: serde_json::Value =
                    serde_json::from_str(&response_video_ids.to_string()).unwrap();

                let mut video_ids: Vec<String> = Vec::new();

                let mut i = 0;
                while i < parsed_response["items"].as_array().unwrap().len() {
                    video_ids
                        .push(parsed_response["items"][i]["contentDetails"]["videoId"].to_string());
                    i = i + 1;
                }
                let mut video_ids_string = video_ids
                    .iter()
                    .map(|x| x.to_string() + ",")
                    .collect::<String>();
                video_ids_string = video_ids_string.trim_end_matches(",").to_string().replace('"', "").replace('\\', "");
                let response_video_details = client
                    .get("https://youtube.googleapis.com/youtube/v3/videos")
                    .header(CONTENT_TYPE, "application/json")
                    .header(ACCEPT, "application/json")
                    .query(&[
                        (
                            "part",
                            "snippet,status,topicDetails,recordingDetails".to_string(),
                        ),
                        ("key", self.api_key.to_string()),
                        ("id", video_ids_string)
                    ])
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await;

                match response_video_details {
                    Ok(response_video_details) => {
                        let parsed_video_details_response: serde_json::Value =
                            serde_json::from_str(&response_video_details.to_string()).unwrap();
                        let mut videos: Vec<Video> = Vec::new();

                        let mut i = 0;
                        while i < parsed_video_details_response["items"]
                            .as_array()
                            .unwrap()
                            .len()
                        {
                            let mut topic_vector: Vec<YoutubeTopic> = Vec::new();
                            let mut j = 0;
                            while j < parsed_video_details_response
                                ["items"][i]["topicDetails"]
                                ["topicCategories"]
                                .as_array()
                                .unwrap()
                                .len()
                            {
                                topic_vector.push(YoutubeTopic::new(
                                    "".to_string(),
                                    YoutubeApi::rem_first_and_last(
                                        &parsed_video_details_response["items"][i]["topicDetails"]
                                            ["topicCategories"][j]
                                            .to_string(),
                                    ).to_string(),
                                ));
                                j = j + 1;
                            }

                            videos.push(Video::new(
                                YoutubeApi::rem_first_and_last(&
                                    parsed_video_details_response["items"][i]["id"].to_string(),
                                ).to_string(),
                                YoutubeApi::rem_first_and_last(&
                                    parsed_video_details_response["items"][i]["snippet"]["title"]
                                        .to_string(),
                                ).to_string(),
                                YoutubeApi::rem_first_and_last(&
                                    parsed_video_details_response["items"][i]["snippet"]
                                        ["description"]
                                        .to_string(),
                                ).to_string(),
                                YoutubeApi::rem_first_and_last(&
                                    parsed_video_details_response["items"][i]["snippet"]
                                        ["publishedAt"]
                                        .to_string(),
                                ).to_string(),
                                YoutubeApi::rem_first_and_last(&
                                    parsed_video_details_response["items"][i]["snippet"]
                                        ["categoryId"]
                                        .to_string(),
                                ).to_string(),
                                YoutubeApi::rem_first_and_last(&
                                    parsed_video_details_response["items"][i]["snippet"]
                                        ["defaultLanguage"]
                                        .to_string(),
                                ).to_string(),
                                YoutubeApi::rem_first_and_last(&
                                    parsed_video_details_response["items"][i]["snippet"]
                                        ["defaultAudioLanguage"]
                                        .to_string(),
                                ).to_string(),
                                topic_vector,
                                Location::new(
                                    YoutubeApi::rem_first_and_last(&
                                        parsed_video_details_response["items"][i]
                                            ["recordingDetails"]["location"]["latitude"]
                                            .to_string(),
                                    ).to_string(),
                                    YoutubeApi::rem_first_and_last(&
                                        parsed_video_details_response["items"][i]
                                            ["recordingDetails"]["location"]["longitude"]
                                            .to_string(),
                                    ).to_string(),
                                    YoutubeApi::rem_first_and_last(&
                                        parsed_video_details_response["items"][i]
                                            ["recordingDetails"]["locationDescription"]
                                            .to_string(),
                                    ).to_string(),
                                ),
                                YoutubeApi::rem_first_and_last(&
                                    parsed_video_details_response["items"][i]["status"]
                                        ["madeForKids"]
                                        .to_string(),
                                )
                                .parse::<bool>()
                                .unwrap(),
                            ));
                            i = i + 1;
                        }
                        if open_video_amount == 0 {
                            return Ok(videos);
                        } else {
                            let next_page_token = parsed_response["nextPageToken"].as_str();
        
                            if next_page_token != None {
                                let further_request = YoutubeApi::get_playlist_videos(
                                    &self,
                                    playlist_id,
                                    next_page_token.unwrap().to_string(),
                                    open_video_amount,
                                    &client
                                )
                                .await;
        
                                match further_request {
                                    Ok(mut response2) => {
                                        videos.append(&mut response2);
                                        return Ok(videos)
                                    }
                                    Err(e2) => {
                                        return Err(e2);
                                    }
                                }
                            } else {
                                return Ok(videos)
                            }
                        }
                    }
                    Err(e) => {
                        return Err(e)
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
