use crate::request::user_param::UserParam;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;



    pub async fn get_user(query: web::Query<UserParam>) -> impl Responder {
        HttpResponse::Ok().body(println!("userId:{:?}", query.user_id))
     }




     