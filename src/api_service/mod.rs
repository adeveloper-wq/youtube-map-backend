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
#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: String,
    pub channel_name: String,
    pub channel_profil_image: String,
    pub channel_banner_image: String,
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
        channel_profil_image,
        channel_banner_image,
        channel_url,
    } = data;
    doc! {
        "channel_id": channel_id,
        "channel_name": channel_name,
        "channel_profil_image": channel_profil_image,
        "channel_banner_image": channel_banner_image,
        "channel_url": channel_url,
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
