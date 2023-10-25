use std::env;

extern crate dotenv;
use dotenv::dotenv;
use futures::TryStreamExt;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};
use crate::models::{
    user_model::User,
    processing_model::Processing,
    image_model::Image,
};

pub struct MongoRepo {
    user_col: Collection<User>,
    processing_col: Collection<Processing>,
    image_col: Collection<Image>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let user_col: Collection<User> = db.collection("User");
        let processing_col: Collection<Processing> = db.collection("Procesing");
        let image_col: Collection<Image> = db.collection("Image");
        MongoRepo {user_col, processing_col, image_col}
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
        };
        let user = self
            .user_col
            .insert_one(new_doc, None)
            .await
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user_col
            .find_one(filter, None)
            .await
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
                "$set":
                    {
                        "id": new_user.id,
                        "name": new_user.name,
                    },
            };
        let updated_doc = self
            .user_col
            .update_one(filter, new_doc, None)
            .await
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user_col
            .delete_one(filter, None)
            .await
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .user_col
            .find(None, None)
            .await
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }

    pub async fn create_image(&self, new_image: Image) -> Result<InsertOneResult, Error> {
        let new_doc = Image {
            id: None,
            uri: new_image.uri,
            user: new_image.user,
            filename: new_image.filename,
            timestamp: new_image.timestamp,
        };
        let image = self
            .image_col
            .insert_one(new_doc, None)
            .await
            .expect("Error creating image");
        Ok(image)
    }

    pub async fn get_image(&self, id: &String) -> Result<Image, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let image_detail = self
            .image_col
            .find_one(filter, None)
            .await
            .expect("Error getting image");
        Ok(image_detail.unwrap())
    }

    pub async fn get_user_images(&self, user_id: &String) -> Result<Vec<Image>, Error> {
        let filter_id = ObjectId::parse_str(user_id).unwrap();
        let filter = doc! {"user": filter_id};
        let mut cursors = self
            .image_col
            .find(filter, None)
            .await
            .expect("Error getting user images");
        let mut images: Vec<Image> = Vec::new();
        while let Some(image) = cursors
            .try_next()
            .await
            .expect("Error mapping through cursor, impossible to retrieve user images")
        {
            images.push(image)
        }
        Ok(images)
    }

    pub async fn create_processing(&self, new_processing: Processing) -> Result<InsertOneResult, Error> {
        let new_doc = Processing {
            id: None,
            timestamp: new_processing.timestamp,
            base_image: new_processing.base_image,
            processed_image: new_processing.processed_image,
            user: new_processing.user
        };
        let processing = self
            .processing_col
            .insert_one(new_doc, None)
            .await
            .expect("Error creating processing");
        Ok(processing)
    }

    pub async fn get_processing(&self, id: &String) -> Result<Processing, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let processing_detail = self
            .processing_col
            .find_one(filter, None)
            .await
            .expect("Error getting processing");
        Ok(processing_detail.unwrap())
    }

    pub async fn get_user_processings(&self, user_id: &String) -> Result<Vec<Processing>, Error> {
        let filter_id = ObjectId::parse_str(user_id).unwrap();
        let filter = doc! {"user": filter_id};
        let mut cursors = self
            .processing_col
            .find(filter, None)
            .await
            .expect("Error getting user processings");
        let mut processings: Vec<Processing> = Vec::new();
        while let Some(processing) = cursors
            .try_next()
            .await
            .expect("Error mapping through cursor, impossible to retrieve user processings")
        {
            processings.push(processing)
        }
        Ok(processings)
    }

    pub async fn update_processing(&self, id: &String, new_processing: Processing) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
                "$set":
                    {
                        "id": new_processing.id,
                        "timestamp": new_processing.timestamp,
                        "base_image": new_processing.base_image,
                        "processed_image": new_processing.processed_image,
                        "user": new_processing.user,
                    },
            };
        let updated_doc = self
            .user_col
            .update_one(filter, new_doc, None)
            .await
            .expect("Error updating processing");
        Ok(updated_doc)
    }
}
