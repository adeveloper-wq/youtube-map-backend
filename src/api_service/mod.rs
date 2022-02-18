// External imports
use bson::{doc, Document};
use futures::StreamExt;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};
// External constructors
extern crate serde;
extern crate serde_json;

// Estructure data for DB
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
    pub channel_subscriber_count: String,
    /* pub channel_topics: Vec<YoutubeTopic>, */
    pub channel_keywords: String,
    pub channel_trailer: String,
    pub made_for_kids: String,
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
        channel_subscriber_count: String,
        /* channel_topics: Vec<YoutubeTopic>, */
        channel_keywords: String,
        channel_trailer: String,
        made_for_kids: String,
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
            /* channel_topics, */
            channel_keywords,
            channel_trailer,
            made_for_kids,
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
        /* channel_topics, */
        channel_keywords,
        channel_trailer,
        made_for_kids,
    } = data;
    doc! {
        "channel_id": channel_id,
        "channel_name": channel_name,
        "channel_description": channel_description,
        "channel_profil_image": channel_profil_image,
        "channel_banner_image": channel_banner_image,
        "channel_country": channel_country,
        "channel_uploads_playlist_id": channel_uploads_playlist_id,
        "channel_subscriber_count": channel_subscriber_count,
        // https://users.rust-lang.org/t/saving-nested-struct-with-rust-mongodb-returns-error-the-trait-from-t-is-not-implemented-for-bson/58188
        /* "channel_topics": bson::to_bson(&channel_topics).unwrap(), */
        "channel_keywords": channel_keywords,
        "channel_trailer": channel_trailer,
        "made_for_kids": made_for_kids,
    }
}

// Functions with quieries to Mongo
impl ApiService {
    pub fn new(collection: Collection<Document>) -> ApiService {
        ApiService { collection }
    }

    // Insert data to Mongo DB
    pub async fn create(&self, _data: &Channel) -> Result<InsertOneResult, Error> {
        self.collection
            .insert_one(data_to_document(_data), None)
            .await
    }

    /* // Update an existing document
    pub async fn update(&self, _data: &Channel, _param: &String) -> Result<UpdateResult, Error> {
        //let object_param = bson::oid::ObjectId::parse_str(_param).unwrap();

        //println!("--------------------- {:?}", object_param.to_string());
        self.collection
            .update_one(doc! { "channel_id": _param }, doc!{"$set": data_to_document(_data) }, None)
            .await
    } */

    /* // Delete some document
    pub async fn delete(&self, _channel_id: &String) -> Result<DeleteResult, Error> {
        self.collection
            .delete_one(doc! { "channel_id": _channel_id }, None)
            .await
    } */

    // Get all documents
    pub async fn get_json(&self) -> Result<std::vec::Vec<bson::Document>, mongodb::error::Error> {
        let mut cursor = match self.collection.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_Error) => return Err(_Error),
        };
        let mut docs: Vec<bson::Document> = Vec::new();
        while let Some(doc) = cursor.next().await {
            docs.push(doc.unwrap());
        }
        Ok(docs)
    }

    // Get documents with quiery
    pub async fn get_by(
        &self,
        param: &String,
    ) -> std::result::Result<std::vec::Vec<bson::Document>, mongodb::error::Error> {
        let mut cursor = match self
            .collection
            .find(doc! { "channel_name": { "$regex": param } }, None)
            .await
        {
            Ok(cursor) => cursor,
            Err(_Error) => return Err(_Error),
        };
        let mut docs: Vec<bson::Document> = Vec::new();
        while let Some(doc) = cursor.next().await {
            docs.push(doc.unwrap());
        }
        let _serialized = serde_json::to_string(&docs).unwrap();
        Ok(docs)
    }
}
