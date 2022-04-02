use crate::{
    api_service::{Channel},
    youtube_api::YoutubeApi,
};
use actix::Arbiter;
use actix_web::{get, web, HttpResponse, Responder};
use bson::DateTime;
use chrono::Utc;
use regex::Regex;

fn was_in_the_last_7_days(last_updated: DateTime) -> bool {
    let current_date_time: chrono::DateTime<Utc> = Utc::now();
    let last_updated_date_time: chrono::DateTime<Utc> = last_updated.to_chrono();
    let difference_in_days = current_date_time.signed_duration_since(last_updated_date_time);
    difference_in_days.num_days() <= 7
}

async fn get_channel_id(channel_url: String) -> String {
    let response_channel_page = reqwest::get(channel_url.to_owned() + &"/videos".to_string())
        .await
        .unwrap();

    if response_channel_page.status() == 404 {
        return "".to_string();
    }

    let body = response_channel_page.text().await.unwrap();

    let re = Regex::new("(canonical\" href=\"https://www.youtube.com/channel/)[^\"]*").unwrap();

    let caps = re.captures(body.as_str()).unwrap();

    let channel_identifier = caps
        .get(0)
        .unwrap()
        .as_str()
        .split("/channel/")
        .last()
        .unwrap()
        .to_string();

    return channel_identifier;
}

async fn get_channel_from_youtube(
    channel_id: String,
    app_data: web::Data<crate::AppState>,
) -> HttpResponse {
    let client = reqwest::Client::new();
    let action = app_data
        .service_manager
        .youtube_api
        .get_channel_data(channel_id.to_string(), &client)
        .await;
    let result = web::block(move || action).await;
    match result {
        Ok(result_channel) => {
            let channel_ref = result_channel.as_ref().unwrap();
            /* let channel = result_channel.unwrap(); */
            if channel_ref.channel_name == "" {
                return HttpResponse::NotFound().finish();
            }

            let channel2 = channel_ref.clone();
            let channel3 = channel_ref.clone();
            let app_data2 = app_data.clone();

            let channel = result_channel.unwrap();

            let action = app_data.service_manager.api.create(&channel).await;
            let result_mongodb_update = web::block(move || action).await;
            match result_mongodb_update {
                Ok(result_mongodb_update) => {
                    println!("{:?}", result_mongodb_update.unwrap());
                    let arbiter = Arbiter::new();
                    arbiter.spawn(async move {
                        YoutubeApi::add_playlist_videos(&channel2, &client, &app_data2).await
                    });
              
                    let action = app_data
                        .service_manager
                        .api
                        .get_channel_by_id(&channel3.channel_id)
                        .await;
                    let result = web::block(move || action).await;
                    match result {
                        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
                        Err(e) => {
                            println!("Error while getting, {:?}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(e) => {
                    println!("Error while getting, {:?}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            }
        }
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/")]
async fn get_all_channels(app_data: web::Data<crate::AppState>) -> impl Responder {
    let action = app_data.service_manager.api.get_all_channels().await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/channel/search/{param}")]
async fn channel_search(
    app_data: web::Data<crate::AppState>,
    param: web::Path<String>,
) -> impl Responder {
    let action = app_data
        .service_manager
        .api
        .get_channels_by_name(&param)
        .await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/channel/{param}")]
async fn get_channel_by_id(
    app_data: web::Data<crate::AppState>,
    param: web::Path<String>,
) -> impl Responder {
    let action = app_data.service_manager.api.get_channel_by_id(&param).await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => {
            let channel_doc = result.unwrap();
            if channel_doc.is_empty() {
                let response = get_channel_from_youtube(param.to_string(), app_data).await;
                return response;
            } else {
                let channel_struct: Channel = bson::from_document(channel_doc.clone()).unwrap();
                if !was_in_the_last_7_days(channel_struct.last_updated) && channel_struct.status != "LOADING" && channel_struct.status != "UPDATING"{
                    let response = get_channel_from_youtube(param.to_string(), app_data).await;
                    return response;
                }
                HttpResponse::Ok().json(channel_doc)
            }
        }
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/c/{param}")]
async fn get_channel_by_custom_url(
    app_data: web::Data<crate::AppState>,
    param: web::Path<String>,
) -> impl Responder {
    let action = app_data
        .service_manager
        .api
        .get_channel_by_custom_url(&param)
        .await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => {
            let channel_doc = result.unwrap();
            if channel_doc.is_empty() {
                let channel_id =
                    get_channel_id("https://www.youtube.com/c/".to_string() + &param).await;
                if channel_id == "".to_string() {
                    return HttpResponse::NotFound().finish();
                }
                let response = get_channel_from_youtube(channel_id, app_data).await;
                return response;
            } else {
                let channel_struct: Channel = bson::from_document(channel_doc.clone()).unwrap();
                if !was_in_the_last_7_days(channel_struct.last_updated)  && channel_struct.status != "LOADING" && channel_struct.status != "UPDATING" {
                    let channel_id =
                        get_channel_id("https://www.youtube.com/c/".to_string() + &param).await;
                    if channel_id == "".to_string() {
                        return HttpResponse::NotFound().finish();
                    }
                    let response = get_channel_from_youtube(channel_id, app_data).await;
                    return response;
                }
                HttpResponse::Ok().json(channel_doc)
            }
        }
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{param}")]
async fn get_channel_by_username(
    app_data: web::Data<crate::AppState>,
    param: web::Path<String>,
) -> impl Responder {
    let channel_id = get_channel_id("https://www.youtube.com/".to_string() + &param).await;
    if channel_id == "".to_string() {
        return HttpResponse::NotFound().finish();
    }

    let action = app_data
        .service_manager
        .api
        .get_channel_by_id(&channel_id)
        .await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => {
            let channel_doc = result.unwrap();
            if channel_doc.is_empty() {
                let response = get_channel_from_youtube(channel_id, app_data).await;
                return response;
            } else {
                let channel_struct: Channel = bson::from_document(channel_doc.clone()).unwrap();
                if !was_in_the_last_7_days(channel_struct.last_updated) && channel_struct.status != "LOADING" && channel_struct.status != "UPDATING" {
                    let response = get_channel_from_youtube(channel_id, app_data).await;
                    return response;
                }
                HttpResponse::Ok().json(channel_doc)
            }
        }
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(channel_search);
    cfg.service(get_channel_by_custom_url);
    cfg.service(get_channel_by_username);
    cfg.service(get_all_channels);
    cfg.service(get_channel_by_id);
}
