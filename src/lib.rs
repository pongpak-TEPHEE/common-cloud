use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use async_trait::async_trait;


#[async_trait]
pub trait CloudStorage {
    async fn upload(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<u16, String>;
}

pub struct AwsClient {
    client: Client,
}

impl AwsClient {
    pub async fn new() -> Self {
        // ใช้ defaults(BehaviorVersion::latest()) ตามมาตรฐานใหม่
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region("ap-southeast-7") // กำหนด Region ที่นี่ได้เลย
            .load()
            .await;
        
        let client = Client::new(&config);
        AwsClient { client }
    }
}

#[async_trait]
impl CloudStorage for AwsClient {
    async fn upload(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<u16, String> {
        match self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(data.into())
            .send()
            .await
        {
            Ok(_) => Ok(200), // HTTP 200 OK for successful PutObject
            Err(e) => {
                let status = e.raw_response().map(|r| r.status().as_u16()).unwrap_or(0);
                Err(format!("AWS Upload Failed (Status {}): {}", status, e))
            }
        }
    }
}