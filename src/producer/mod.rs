
use std::{collections::HashMap, sync::Mutex};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};

use serde::{ Deserialize, Serialize};

use lazy_static::lazy_static;


lazy_static!{
    static ref MAP_INSTANCE: StoreData = StoreData::new();
}

pub async fn producer() {
    
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/user/add", post(create_user));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn get_user(Path(id): Path<String>) -> impl IntoResponse{
    
    let usr = MAP_INSTANCE.get(&id);
    Json(usr)

}

async fn create_user(Json(payload): Json<CreateUser>,) -> (StatusCode,Json<HashMap<String,User>>){
   

    let  key = MAP_INSTANCE.len()+1;
  
    let user = User{
        id: key.try_into().unwrap(),
        user_name: payload.user_name,
        comment: Some("user added ".to_string())
        
    };
    

    MAP_INSTANCE.insert(key.to_string(),user);

    println!("{:?}",MAP_INSTANCE.data);

    (StatusCode::CREATED,Json(MAP_INSTANCE.get_dictionary()))
}

#[derive(Deserialize,Clone)]
struct CreateUser{
    user_name:String
}

#[derive(Serialize,Debug,Clone,PartialEq)]
struct User{
    id: u64,
    user_name: String,
    comment: Option<String>
}

#[derive(Debug)]
struct StoreData{
    data: Mutex<HashMap<String,User>>,
}

impl StoreData {
    fn new() -> Self{
        StoreData{
            data: Mutex::new(HashMap::new()),
        }
    }

    fn insert( &self, key:String, value:User){
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    fn get(&self, key:&str) ->  Option<User>{

        let data = self.data.lock().unwrap();
        let response =data.get(key);
        
        response.cloned()
    }

    fn len(&self) -> usize{
        let data =self.data.lock().unwrap();
        data.len()

    }

    fn get_dictionary(&self) -> HashMap<String,User>{
        let data = self.data.lock().unwrap();
        data.clone()
    }
}