use std::fs;
use std::str::FromStr;
use std::time::SystemTime;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_web::{post, HttpResponse, Responder, Error};
use actix_web::web::Data;
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use crate::{models::image_model::Image, models::processing_model::Processing, repository::mongodb_repo::MongoRepo};
use image;

const IMAGE_PATH: &str = "./tmp/images/";

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/process_image")]
async fn process_image(
    db: Data<MongoRepo>,
    user_id: String,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> HttpResponse {

    /*
    User sends image and user_id
    Save base image on file system
    Insert base image data in DB
    */
    if form.files.len() != 1 {
       return HttpResponse::BadRequest().body("Only one file must be sent")
    }
    let f = form.files.get(0).unwrap();
    save_image(f);
    let filename = f.file_name.clone().unwrap();
    let fullpath = IMAGE_PATH.to_owned() + &filename;
    let new_image = Image {
        id: None,
        user: ObjectId::from_str(&user_id).unwrap(),
        timestamp: DateTime::now(),
        uri: fullpath.clone(),
        filename: filename.clone(),
    };

    let result_image_creation = match db.create_image(new_image).await {
        Ok(r) => r,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
    let base_image_id = result_image_creation.inserted_id;

    /*
    Apply processing to the base image
    Save processed image in file system
    Insert processed image data in DB
    Save processing data in DB (images ID, user id, timestamp)

    Return processed image ID or link
    */

    let img = match image::open(&fullpath) {
        Ok(result) => result,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let gray_image = image::imageops::rotate180(&img);
    let processed_path = fullpath.clone() + "_processed.jpg";
    gray_image.save(processed_path).expect("Impossible to save processed image");

    let processed_image_document = Image {
        id: None,
        user: ObjectId::from_str(&user_id).unwrap(),
        timestamp: DateTime::now(),
        uri: fullpath.clone(),
        filename: filename + "_processed.jpg",
    };

    let result_image_creation = match db.create_image(processed_image_document).await {
        Ok(r) => r,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
    let processed_image_id = result_image_creation.inserted_id;

    HttpResponse::Ok().body(format!("Image correctly saved with IDs {} and {}", base_image_id, processed_image_id))
}

fn save_image(f: &TempFile) {
    fs::create_dir_all(IMAGE_PATH).expect("Impossible to create directory");
    let path = IMAGE_PATH.to_owned() + &f.file_name.clone().unwrap();
    println!("{}", path);
    f.file.persist(path).unwrap();
}
