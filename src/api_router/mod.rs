use crate::api_service::{AddChannelRequestBody, Channel};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/get-all")]
async fn get_all_json(app_data: web::Data<crate::AppState>) -> impl Responder {
    let action = app_data.service_manager.api.get_json().await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/get-by/{param}")]
async fn get_user_email(
    app_data: web::Data<crate::AppState>,
    param: web::Path<String>,
) -> impl Responder {
    let action = app_data.service_manager.api.get_by(&param).await;
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/add")]
async fn add_user(
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
                let action = app_data
                    .service_manager
                    .youtube_api
                    .get_channel_data(&channel_url.to_string())
                    .await;
                let result = web::block(move || action).await;
                match result {
                    Ok(resultChannel) => {
                        let action = app_data.service_manager.api.create(&resultChannel.unwrap()).await;
                        let result = web::block(move || action).await;
                        match result {
                            Ok(resultMongoDB) => HttpResponse::Ok().json(resultMongoDB.unwrap()),
                            Err(e) => {
                                println!("Error while getting, {:?}", e);
                                HttpResponse::InternalServerError().finish()
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
            HttpResponse::BadRequest().finish()
        },
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
    cfg.service(get_user_email);
    cfg.service(add_user);
    /* cfg.service(update_user);
    cfg.service(delete_user); */
    cfg.service(get_all_json);
}
