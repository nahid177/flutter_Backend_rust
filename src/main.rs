use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use mongodb::bson::{doc, to_document, oid::ObjectId};
use mongodb::{Client, options::ClientOptions};
use serde_json::json;
use dotenv::dotenv;
use std::env;

mod models;
mod aws_s3;

use models::ProductDocument;
use aws_s3::upload_to_s3;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables

    // Retrieve the MongoDB URL from .env file
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set in .env");

    // Create MongoDB Client
    let client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    // Clone the client to share across threads
    let mongo_client = web::Data::new(client.database("test_db")); // Use your database name here

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(mongo_client.clone())
            .route("/", web::get().to(index))
            .route("/add_product", web::post().to(add_product))
            .route("/change_product/{id}", web::put().to(change_product))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Home Route
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust Mongo API with AWS S3 Integration")
}

// Add Product Handler with Image Upload
async fn add_product(
    db: web::Data<mongodb::Database>, 
    product: web::Json<ProductDocument>, 
    file_path: String,     // Path to the image
    file_name: String      // Image file name for S3
) -> HttpResponse {
    let collection = db.collection::<mongodb::bson::Document>("products");

    // Upload image to S3
    let image_url = match upload_to_s3(&file_path, &file_name).await {
        Ok(url) => url,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Image upload failed: {}", e)),
    };

    let mut product_data = product.into_inner();
    
    // Modify product data to include the image URL
    if let Some(item) = product_data.type_.get_mut(0) {
        if let Some(brand) = item.items.get_mut(0) {
            if let Some(product_details) = brand.brands.get_mut(0) {
                product_details.items[0].images.push(image_url.clone());
            }
        }
    }

    // Convert `ProductDocument` to BSON document
    let bson_product = match to_document(&product_data) {
        Ok(doc) => doc,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error converting to BSON: {}", e)),
    };

    // Insert product into the database
    match collection.insert_one(bson_product, None).await {
        Ok(insert_result) => HttpResponse::Ok().json(json!({"inserted_id": insert_result.inserted_id, "image_url": image_url})),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error inserting product: {}", e)),
    }
}

// Change Product Handler
async fn change_product(
    db: web::Data<mongodb::Database>,
    product_id: web::Path<String>,
    updated_product: web::Json<ProductDocument>,
) -> HttpResponse {
    let collection = db.collection::<mongodb::bson::Document>("products");

    // Convert the product_id to ObjectId
    let product_oid = match ObjectId::parse_str(&product_id.into_inner()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().json(json!({"error": "Invalid product ID"})),
    };

    // Convert updated product data to BSON document
    match to_document(&updated_product.into_inner()) {
        Ok(updated_data) => {
            // Perform the update in MongoDB
            match collection.update_one(doc! { "_id": product_oid }, doc! { "$set": updated_data }, None).await {
                Ok(update_result) => {
                    if update_result.matched_count > 0 {
                        HttpResponse::Ok().json(json!({"status": "Product updated successfully"}))
                    } else {
                        HttpResponse::NotFound().json(json!({"error": "Product not found"}))
                    }
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Error updating product: {}", e)),
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error converting to BSON: {}", e)),
    }
}
