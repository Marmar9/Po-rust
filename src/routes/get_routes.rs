
use actix_web::{
   web, HttpResponse, error::ErrorInternalServerError ,
};
use crate::utils::user_functions::User;
use crate::AppState;

use crate::utils::user_functions::{get_users , get_user};

pub async fn user(info: web::Path<(String,)>, data: web::Data<AppState>) -> HttpResponse {
   let connection_pool = &data.connection_pool;
   let mut connection = connection_pool.get_connection().await;
   let id = &info.0;

   match get_user(&mut connection,  id.clone()).await {
      Some(result) => {
          let user_json = serde_json::to_string(&result).expect("Failed to serialize response");
          connection_pool.return_connection(connection).await;
        return HttpResponse::Ok()
              .content_type("application/json")
              .body(user_json)
      }
      None => {
          connection_pool.return_connection(connection).await;
          return HttpResponse::BadRequest().content_type("plaintext").body("User does not exist")

      }
  }
}
pub async fn users(data: web::Data<AppState>) -> HttpResponse {
   let connection_pool = &data.connection_pool;
   let mut connection = connection_pool.get_connection().await;

   match get_users(&mut connection).await {
      Ok(result) => {
         let json_response = serde_json::to_string(&result).expect("Failed to serialize response");
         connection_pool.return_connection(connection).await;
         HttpResponse::Ok().content_type("application/json").body(json_response)
      },
      Err(_) => {
         connection_pool.return_connection(connection).await;
         HttpResponse::Ok().content_type("application/json").body("[]")
      }
   }




   
}








// pub async fn get_user(
//    connection: &mut redis::aio::Connection,
//    hash_key: &str,
//    user_id: String,
// ) -> Option<User> {
//    let json_str: Option<String> = connection.hget(hash_key, user_id).await.unwrap();

//    json_str.and_then(|json| serde_json::from_str(&json).ok())
// }
