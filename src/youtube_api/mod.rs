// External imports
use bson::{doc, Document};
use futures::StreamExt;
use reqwest;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};
use serde_json;

use crate::api_service::Channel;
// External constructors
extern crate serde;

pub struct YoutubeApi {
    api_key: String,
}

impl YoutubeApi {
    pub fn new(api_key: String) -> YoutubeApi {
        YoutubeApi { api_key }
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
    pub async fn get_channel_data(&self, channel_url: &String) -> Result<Channel, reqwest::Error> {
        let channel_identifier: String;
        let mut channel_identifier_is_id = false;

        if (channel_url.contains("/channel/")) {
            channel_identifier_is_id = true;
            channel_identifier = channel_url.split("/channel/").last().unwrap().to_string();
        } else {
            channel_identifier = channel_url.split("/c/").last().unwrap().to_string();
        }

        let client = reqwest::Client::new();
        // https://blog.logrocket.com/making-http-requests-rust-reqwest/

        let response: Result<String, reqwest::Error>;
        println!("channel_identifier_is_id {}", channel_identifier_is_id);
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
                let parsedResponse: serde_json::Value = serde_json::from_str(&response.to_string()).unwrap();

                println!("parsedResponse {}", parsedResponse);

                let channel = Channel::new(
                    parsedResponse["items"][0]["id"].to_string(),
                    parsedResponse["items"][0]["snippet"]["title"].to_string(),
                    parsedResponse["items"][0]["snippet"]["description"].to_string(),
                    parsedResponse["items"][0]["snippet"]["thumbnails"]["default"]["url"].to_string(),
                    parsedResponse["items"][0]["brandingSettings"]["image"]["bannerExternalUrl"].to_string(),
                    parsedResponse["items"][0]["snippet"]["country"].to_string(),
                    parsedResponse["items"][0]["contentDetails"]["relatedPlaylists"]["uploads"].to_string(),
                    parsedResponse["items"][0]["statistics"]["subscriberCount"].to_string(),
                    parsedResponse["items"][0]["brandingSettings"]["channel"]["keywords"].to_string(),
                    parsedResponse["items"][0]["brandingSettings"]["channel"]["unsubscribedTrailer"].to_string(),
                    parsedResponse["items"][0]["status"]["madeForKids"].to_string(),
                );
                return Ok(channel);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
