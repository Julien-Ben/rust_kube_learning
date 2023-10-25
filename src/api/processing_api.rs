use std::fs;
use std::str::FromStr;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_web::{post, HttpResponse};
use actix_web::web::Data;
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use crate::models::{processing_model::Processing, image_model::Image};
use crate::repository::mongodb_repo::MongoRepo;
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
    MultipartForm(mut form): MultipartForm<UploadForm>,
) -> HttpResponse {
    // User sends image and user_id
    if form.files.len() != 1 {
       return HttpResponse::BadRequest().body("Exactly one file must be sent")
    }

    // Save base image on file system
    let f = form.files.pop().unwrap();
    let filename = f.file_name.clone().unwrap();
    save_image(f);

    // Insert base image data in DB
    let fullpath = IMAGE_PATH.to_owned() + &filename;
    let new_image = Image {
        id: None,
        user: ObjectId::from_str(&user_id).expect("User id should be parsed correctly"),
        timestamp: DateTime::now(),
        uri: fullpath.clone(),
        filename: filename.clone(),
    };

    let result_image_creation = match db.create_image(new_image).await {
        Ok(r) => r,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
    let base_image_id = result_image_creation.inserted_id;

    // Apply processing to the base image and save it in filesystem
    let img = match image::open(&fullpath) {
        Ok(result) => result,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let gray_image = image::imageops::rotate180(&img);
    let processed_path = fullpath.clone() + "_processed.jpg"; // TODO : remove previous ".jpg"
    gray_image.save(processed_path).expect("Impossible to save processed image"); // TODO : good
                                                                                  // practice
    // Insert processed image data in DB
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

    // Save processing data in DB (images ID, user id, timestamp)
    // TODO : Do it ?
    let processing_document = Processing {}
    let result_image_creation = match db.create_processing(processed_image_document).await {
        Ok(r) => r,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };
    let processed_image_id = result_image_creation.inserted_id;

    // Return processed image ID or link
    HttpResponse::Ok().body(format!("Image correctly saved with IDs {} and {}", base_image_id, processed_image_id))
}

fn save_image(f: TempFile) {
    fs::create_dir_all(IMAGE_PATH).expect("Impossible to create directory");
    let path = IMAGE_PATH.to_owned() + &f.file_name.clone().unwrap();
    println!("{}", path);
    f.file.persist(path).unwrap();
}
