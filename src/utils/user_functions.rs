use actix_web::error::JsonPayloadError;
use serde::{Deserialize, Serialize};
use redis::{AsyncCommands, Connection};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
   pub name: String,
   pub lastname: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformedUser {
   pub id : String, 
   pub name : String,
   pub surname : String,
}


pub async fn create_user(
   connection: &mut redis::aio::Connection,
   user_id: &String,
   user: &User,
) {
   let json_str = serde_json::to_string(user).unwrap();

   let _: () = connection.hset("users", user_id, json_str).await.expect("Failed to create a user");
}

pub async fn update_user(
   connection: &mut redis::aio::Connection,
   user_id: &String,
   user: User,
) -> Option<String> {

   dbg!(user_id.clone());
   let json_str: Option<String> = match connection.hget("users", user_id.clone()).await {
       Ok(result) => result,
       Err(_) => return None,
   };

   dbg!(json_str.clone());
   dbg!(json_str.is_none());
   if json_str.is_none() {
       return None;
   }

   let updated_json_str = serde_json::to_string(&user).expect("Failed to serialize user in update");

   dbg!(updated_json_str.clone());
   dbg!(user_id.clone());

   let _: () = connection
       .hset("users", user_id, updated_json_str)
       .await
       .expect("Failed to update user");


   Some("".to_string())
}

pub async fn delete_user(
   connection: &mut redis::aio::Connection,
   user_id: &String,
) -> Option<String> {

   dbg!(user_id.clone());
   let json_str: Option<String> = match connection.hget("users", user_id.clone()).await {
       Ok(result) => result,
       Err(_) => return None,
   };

   dbg!(json_str.clone());
   dbg!(json_str.is_none());
   if json_str.is_none() {
       return None;
   }

   dbg!(user_id.clone());

   let _: () = connection
      .hdel("users", user_id)
       .await
       .expect("Failed to delete");


   Some("".to_string())
}




pub async fn get_user(
   connection: &mut redis::aio::Connection,
   user_id: String,
) -> Option<User> {
   let json_str: Option<String> = connection.hget("users", user_id).await.unwrap();

   dbg!(&json_str);

   json_str.and_then(|json| serde_json::from_str(&json).ok())
}


// -> Result<Vec<User>, redis::RedisError>
pub async fn get_users(connection: &mut redis::aio::Connection) -> Result<Vec<TransformedUser>, () > {
   // Replace "users" with the actual key you've used for your hash

   // Query all fields and values from the hash
   let user_data: Vec<(String, String)> = connection.hgetall("users").await.unwrap();
   // Transform the data into a vector of User structs
   if user_data.len() == 0 {
       Err(())
   } else {
       let transformed_data: Vec<TransformedUser> = user_data
           .iter()
           .map(|(id, user)| {
               // Deserialize the user string into a User struct
               let user_struct: User = serde_json::from_str(&user).expect("Failed to deserialize user");

               // Create a TransformedUser struct
               TransformedUser {
                   id: id.clone(),
                   name: user_struct.name.clone(),
                   surname: user_struct.lastname.clone(),
               }
           })
           .collect();

       Ok(transformed_data)
   }
}
