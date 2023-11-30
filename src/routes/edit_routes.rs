use actix_web::{web , HttpResponse};
use uuid::Uuid;



use crate::AppState;

use crate::utils::user_functions::{User , update_user , create_user,delete_user };



pub async fn user_post(user: actix_web::web::Json<User> , info: web::Path<(String,)>, data: web::Data<AppState>) -> HttpResponse{
   let connection_pool = &data.connection_pool;
   let mut connection = connection_pool.get_connection().await;

   let id = &info.0;

   let post_data = user.into_inner();

   // Perform some logic with the user data (e.g., store it in a database)
   // For simplicity, let's just print the received user information
   let new_user = User {
      name : post_data.name,
      lastname : post_data.lastname,
   };

   create_user(&mut connection, id, &new_user).await;
   connection_pool.return_connection(connection).await;

   HttpResponse::NoContent().finish()

}
pub async fn user_put(user: actix_web::web::Json<User> , info: web::Path<(String,)>, data: web::Data<AppState>) -> HttpResponse{
   // Access the user data from the request
   let connection_pool = &data.connection_pool;
   let mut connection = connection_pool.get_connection().await;

   let id = &info.0;

   let post_data = user.into_inner();

   // Perform some logic with the user data (e.g., store it in a database)
   // For simplicity, let's just print the received user information
   let new_user = User {
      name : post_data.name,
      lastname : post_data.lastname,
   };

   create_user(&mut connection, id, &new_user).await;
   connection_pool.return_connection(connection).await;


   // Return a success response
   HttpResponse::NoContent().finish()
}

pub async fn user_patch(user: actix_web::web::Json<User> , info: web::Path<(String,)>, data: web::Data<AppState>) -> HttpResponse{
   // Access the user data from the request

   let connection_pool = &data.connection_pool;
   let mut connection = connection_pool.get_connection().await;

   let id = &info.0;

   let post_data = user.into_inner();
   
   if post_data.name.is_empty() || post_data.lastname.is_empty() {
      connection_pool.return_connection(connection).await;

      return HttpResponse::BadRequest().content_type("plaintext").body("Name and lastname are required")
   }
   // Perform some logic with the user data (e.g., store it in a database)
   // For simplicity, let's just print the received user information
   let new_user = User {
      name : post_data.name,
      lastname : post_data.lastname,
   };

   match update_user(&mut connection,  id,  new_user).await {
      Some(_) => {
         connection_pool.return_connection(connection).await;

         return HttpResponse::NoContent().finish()
      }
      None => {
         connection_pool.return_connection(connection).await;

         return HttpResponse::BadRequest().content_type("plaintext").body("User does not exist")
      }
   }
   // Return a success response

}
pub async fn user_delete( info: web::Path<(String,)>, data: web::Data<AppState>) -> HttpResponse{
   let connection_pool = &data.connection_pool;
   let mut connection = connection_pool.get_connection().await;

   let id = &info.0;

   // Perform some logic with the user data (e.g., store it in a database)
   // For simplicity, let's just print the received user information

   match delete_user(&mut connection,  id).await {
      Some(_) => {
         connection_pool.return_connection(connection).await;

         return HttpResponse::NoContent().finish()
      }
      None => {
         connection_pool.return_connection(connection).await;

         return HttpResponse::BadRequest().content_type("plaintext").body("User not exist")
      }
   }
}