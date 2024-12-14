use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtitle {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub titledetail: String,
    pub subtitle: Option<Vec<Subtitle>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductItemDetails {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub product_name: String,
    pub title: Vec<String>,
    pub subtitle: Vec<Subtitle>,
    pub description: String,
    pub amount: f64,
    pub discount_amount: f64,
    pub quantity: i32,
    pub images: Vec<String>,  // Store image URLs from S3
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductBrand {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub brand_name: String,
    pub items: Vec<ProductItemDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub item_name: String,
    pub brands: Vec<ProductBrand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductType {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub type_name: String,
    pub items: Vec<ProductItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDocument {
    pub type_: Vec<ProductType>,
}
