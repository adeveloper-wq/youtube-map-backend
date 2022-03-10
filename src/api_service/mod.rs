use bson::oid::ObjectId;
// External imports
use bson::{doc, DateTime, Document};
use chrono::Utc;
use futures::StreamExt;
use mongodb::options::UpdateOptions;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};

use crate::youtube_api::YoutubeApi;
// External constructors
extern crate serde;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub video_id: String,
    pub video_titel: String,
    /* pub video_description: String, */
    pub video_published_at: String,
    pub video_category_id: String,
    pub video_default_language: String,
    pub video_default_audio_language: String,
    pub video_topics: Vec<YoutubeTopic>,
    pub video_location: Location,
    pub made_for_kids: bool,
}

impl Video {
    pub fn new(
        video_id: String,
        video_titel: String,
        /* video_description: String, */
        video_published_at: String,
        video_category_id: String,
        video_default_language: String,
        video_default_audio_language: String,
        video_topics: Vec<YoutubeTopic>,
        video_location: Location,
        made_for_kids: bool,
    ) -> Video {
        Video {
            video_id,
            video_titel,
            /* video_description, */
            video_published_at,
            video_category_id,
            video_default_language,
            video_default_audio_language,
            video_topics,
            video_location,
            made_for_kids,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct YoutubeTopic {
    pub topic_id: String,
    pub topic_url: String,
}

impl YoutubeTopic {
    pub fn new(topic_id: String, topic_url: String) -> YoutubeTopic {
        YoutubeTopic {
            topic_id,
            topic_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub latitude: String,
    pub longitude: String,
    pub description: String,
}

impl Location {
    pub fn new(latitude: String, longitude: String, description: String) -> Location {
        Location {
            latitude,
            longitude,
            description,
        }
    }
}

// Estructure data for DB
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub channel_id: String,
    pub channel_name: String,
    pub channel_description: String,
    pub channel_profil_image: String,
    pub channel_banner_image: String,
    pub channel_country: String,
    pub channel_uploads_playlist_id: String,
    pub channel_subscriber_count: u32,
    pub channel_topics: Vec<YoutubeTopic>,
    pub channel_keywords: String,
    pub channel_trailer: String,
    pub made_for_kids: bool,
    pub status: String,
    pub channel_videos: Vec<Video>,
    pub map_marker_hex_color: String,
    pub last_updated: DateTime,
    pub video_count: u32,
    pub channel_custom_url: String
}

impl Channel {
    pub fn new(
        channel_id: String,
        channel_name: String,
        channel_description: String,
        channel_profil_image: String,
        channel_banner_image: String,
        channel_country: String,
        channel_uploads_playlist_id: String,
        channel_subscriber_count: u32,
        channel_topics: Vec<YoutubeTopic>,
        channel_keywords: String,
        channel_trailer: String,
        made_for_kids: bool,
        status: String,
        channel_videos: Vec<Video>,
        map_marker_hex_color: String,
        last_updated: DateTime,
        video_count: u32,
        channel_custom_url: String
    ) -> Channel {
        Channel {
            channel_id,
            channel_name,
            channel_description,
            channel_profil_image,
            channel_banner_image,
            channel_country,
            channel_uploads_playlist_id,
            channel_subscriber_count,
            channel_topics,
            channel_keywords,
            channel_trailer,
            made_for_kids,
            status,
            channel_videos,
            map_marker_hex_color,
            last_updated,
            video_count,
            channel_custom_url
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddChannelRequestBody {
    pub channel_url: String,
}

// Reference colection clone
#[derive(Clone)]
pub struct ApiService {
    collection: Collection<Document>,
}

// Transform data to mongo db document
fn data_to_document(data: &Channel) -> Document {
    let Channel {
        channel_id,
        channel_name,
        channel_description,
        channel_profil_image,
        channel_banner_image,
        channel_country,
        channel_uploads_playlist_id,
        channel_subscriber_count,
        channel_topics,
        channel_keywords,
        channel_trailer,
        made_for_kids,
        status,
        channel_videos,
        map_marker_hex_color,
        last_updated,
        video_count,
        channel_custom_url
    } = data;
    doc! {"$setOnInsert": doc! {
        "channel_id": channel_id,
        "channel_name": channel_name,
        "channel_description": channel_description,
        "channel_profil_image": channel_profil_image,
        "channel_banner_image": channel_banner_image,
        "channel_country": channel_country,
        "channel_uploads_playlist_id": channel_uploads_playlist_id,
        "channel_subscriber_count": channel_subscriber_count,
        // https://users.rust-lang.org/t/saving-nested-struct-with-rust-mongodb-returns-error-the-trait-from-t-is-not-implemented-for-bson/58188
        "channel_topics": bson::to_bson(&channel_topics).unwrap(),
        "channel_keywords": channel_keywords,
        "channel_trailer": channel_trailer,
        "made_for_kids": made_for_kids,
        "status": status,
        "channel_videos": bson::to_bson(&channel_videos).unwrap(),
        "map_marker_hex_color": map_marker_hex_color,
        "last_updated": last_updated,
        "video_count": video_count,
        "channel_custom_url": channel_custom_url
    }}
}

// Functions with quieries to Mongo
impl ApiService {
    pub fn new(collection: Collection<Document>) -> ApiService {
        ApiService { collection }
    }

    // Create channel
    pub async fn create(&self, _data: &Channel) -> Result<UpdateResult, Error> {
        let mut options = UpdateOptions::default();
        options.upsert=Some(true);
        self.collection
            .update_one(
                doc! { "channel_id": &_data.channel_id },
                data_to_document(_data),
                options,
            )
            .await
    }

    // Update channel with videos
    pub async fn update_videos(
        &self,
        _videos: &Vec<Video>,
        _channel_id: &String,
    ) -> Result<UpdateResult, Error> {
        //let object_param = bson::oid::ObjectId::parse_str(_param).unwrap();

        self.collection
            .update_one(doc! { "channel_id": _channel_id }, doc!{"$set": doc! {"channel_videos": bson::to_bson(&_videos).unwrap(), "status": "IDLE", "last_updated": bson::DateTime::from_chrono(Utc::now())} }, None)
            .await
    }

    /* // Delete some document
    pub async fn delete(&self, _channel_id: &String) -> Result<DeleteResult, Error> {
        self.collection
            .delete_one(doc! { "channel_id": _channel_id }, None)
            .await
    } */

    // Update videos of channel
    /* pub async fn update(
        &self,
        _videos: &Vec<Video>,
        _channel_id: &String,
    ) -> Result<UpdateResult, Error> {

        self.collection
            .update_one(
                doc! { "channel_id": _channel_id },
                doc! { "$set": { "videos": bson::to_bson(_videos).unwrap(), "status": "FINISHED" }},
                None,
            )
            .await
    } */

    // Get all documents
    pub async fn get_all_channels(
        &self,
    ) -> Result<std::vec::Vec<bson::Document>, mongodb::error::Error> {
        let mut cursor = match self.collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(error) => return Err(error),
        };
        let mut docs: Vec<bson::Document> = Vec::new();
        while let Some(doc) = cursor.next().await {
            docs.push(doc.unwrap());
        }
        Ok(docs)
    }

    // Search channels by name
    pub async fn get_channels_by_name(
        &self,
        param: &String,
    ) -> std::result::Result<std::vec::Vec<bson::Document>, mongodb::error::Error> {
        let mut cursor = match self
            .collection
            .find(doc! { "channel_name": { "$regex": param } }, None)
            .await
        {
            Ok(cursor) => cursor,
            Err(error) => return Err(error),
        };
        let mut docs: Vec<bson::Document> = Vec::new();
        while let Some(doc) = cursor.next().await {
            docs.push(doc.unwrap());
        }
        //let _serialized = serde_json::to_string(&docs).unwrap();
        Ok(docs)
    }

    pub async fn get_channel_by_mongodb_id(
        &self,
        param: &ObjectId,
    ) -> std::result::Result<bson::Document, mongodb::error::Error> {
        let cursor = match self.collection.find_one(doc! { "_id": param }, None).await {
            Ok(cursor) => cursor,
            Err(error) => return Err(error),
        };
        /*         let mut docs: Vec<bson::Document> = Vec::new();
        while let Some(doc) = cursor.next().await {
            docs.push(doc.unwrap());
        } */
        //let _serialized = serde_json::to_string(&cursor.unwrap()).unwrap();

        Ok(cursor.unwrap_or(Document::new()))
    }

    // Get channels by id
    pub async fn get_channel_by_id(
        &self,
        param: &String,
    ) -> std::result::Result<bson::Document, mongodb::error::Error> {
        let cursor = match self
            .collection
            .find_one(doc! { "channel_id": param }, None)
            .await
        {
            Ok(cursor) => cursor,
            Err(error) => return Err(error),
        };
        /*         let mut docs: Vec<bson::Document> = Vec::new();
        while let Some(doc) = cursor.next().await {
            docs.push(doc.unwrap());
        } */
        //let _serialized = serde_json::to_string(&cursor.unwrap()).unwrap();

        Ok(cursor.unwrap_or(Document::new()))
    }

    // Get channels by youtube custom url
    pub async fn get_channel_by_custom_url(
        &self,
        param: &String,
    ) -> std::result::Result<bson::Document, mongodb::error::Error> {
        let cursor = match self
            .collection
            .find_one(doc! { "channel_custom_url": param }, None)
            .await
        {
            Ok(cursor) => cursor,
            Err(error) => return Err(error),
        };
        /*         let mut docs: Vec<bson::Document> = Vec::new();
        while let Some(doc) = cursor.next().await {
            docs.push(doc.unwrap());
        } */
        //let _serialized = serde_json::to_string(&cursor.unwrap()).unwrap();

        Ok(cursor.unwrap_or(Document::new()))
    }
}
