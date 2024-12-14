use aws_sdk_s3::{Client, Error}; // Correct imports
use aws_sdk_s3::primitives::ByteStream;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn upload_to_s3(file_path: &str, file_name: &str) -> Result<String, Error> {
    // Load AWS configuration using default environment loader
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    
    // Get the bucket name from the environment
    let bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set in .env");

    // Read the file from the file system
    let mut file = File::open(file_path).await.expect("File not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.expect("Failed to read file");

    // Upload the file to S3
    let body = ByteStream::from(buffer);
    let _resp = client.put_object()
        .bucket(bucket_name.clone())  // Clone to avoid move
        .key(file_name)
        .body(body)
        .send()
        .await?;

    // Return the file URL
    Ok(format!("https://{}.s3.amazonaws.com/{}", bucket_name, file_name))
}
