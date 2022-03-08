use crate::{
    api_service::{AddChannelRequestBody, Channel},
    youtube_api::YoutubeApi,
};
use actix::Arbiter;
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::TryFutureExt;

#[get("/channel")]
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
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/channel")]
async fn add_channel(
    app_data: web::Data<crate::AppState>,
    data: web::Json<AddChannelRequestBody>,
) -> impl Responder {
    let channel_url = data.channel_url.trim();
    let url_check_result = app_data
        .service_manager
        .youtube_api
        .check_url(&channel_url.to_string());
    match url_check_result {
        Ok(url_check_result_youtube_url) => {
            if (url_check_result_youtube_url) {
                let client = reqwest::Client::new();
                let action = app_data
                    .service_manager
                    .youtube_api
                    .get_channel_data(&channel_url.to_string(), &client)
                    .await;
                let result = web::block(move || action).await;
                match result {
                    Ok(result_channel) => {
                        let channel_ref = result_channel.as_ref().unwrap();
                        /* let channel = result_channel.unwrap(); */

                        let channel2 = channel_ref.clone();
                        let channel3 = channel_ref.clone();
                        let app_data2 = app_data.clone();

                        let arbiter = Arbiter::new();

                        /*  Arbiter::spawn(async {
                            app_data.service_manager.youtube_api.add_playlist_videos(channel_ref, &client, &app_data);
                            /* let result_get_all_videos = app_data
                                .service_manager
                                .youtube_api
                                .get_playlist_videos(
                                    &channel_ref.channel_uploads_playlist_id,
                                    "FIRST_PAGE".to_string(),
                                    channel_ref.video_count,
                                    &client,
                                )
                                .await;

                            match result_get_all_videos {
                                Ok(result_get_all_videos) => {
                                    let action =
                                        app_data.service_manager.api.update_videos(&result_get_all_videos, &channel_ref.channel_id).await;
                                    let result_mongodb_update = web::block(move || action).await;
                                    /* match result_mongodb_update {
                                        Ok(result_mongodb_update) => {

                                        }
                                        Err(e) => {
                                            println!("Error while getting, {:?}", e);
                                        }
                                    } */
                                }
                                /* Err(e) => {
                                    println!("Error while getting, {:?}", e);
                                    return HttpResponse::InternalServerError().finish();
                                } */
                            } */
                        }); */

                        let channel = result_channel.unwrap();

                        let action = app_data.service_manager.api.create(&channel).await;
                        let result_mongodb_update = web::block(move || action).await;
                        match result_mongodb_update {
                            Ok(result_mongodb_update) => {
                                if result_mongodb_update.unwrap().matched_count == (0 as u64) {
                                    arbiter.spawn(async move {
                                        YoutubeApi::add_playlist_videos(
                                            &channel2, &client, &app_data2,
                                        )
                                        .await
                                    });
                                }
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
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
        Err(e) => {
            println!("Error while parsing, {:?}", e);
            HttpResponse::BadRequest().finish()
        }
    }

    /* let action = app_data.service_manager.api.create(&data).await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    } */
}

/* #[post("/update/{param}")]
async fn update_user(app_data: web::Data<crate::AppState>, data: web::Json<Channel>, param: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager.api.update(&data, &param).await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
} */

/* #[delete("/delete")]
async fn delete_user(app_data: web::Data<crate::AppState>, data: web::Json<Channel>) -> impl Responder {
    let action = app_data.service_manager.api.delete(&data.channel_id).await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
} */

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(add_channel);
    cfg.service(channel_search);
    /* cfg.service(update_user);
    cfg.service(delete_user); */
    cfg.service(get_all_channels);
    cfg.service(get_channel_by_id);
}
