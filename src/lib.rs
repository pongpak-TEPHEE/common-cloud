use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use async_trait::async_trait;


#[async_trait]
pub trait CloudStorage {
    async fn upload(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<(), String>;
}

pub struct AwsClient {
    client: Client,
}

impl AwsClient {
    pub async fn new() -> Self {
        // ใช้ defaults(BehaviorVersion::latest()) ตามมาตรฐานใหม่
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region("ap-southeast-1") // กำหนด Region ที่นี่ได้เลย
            .load()
            .await;
        
        let client = Client::new(&config);
        AwsClient { client }
    }
}

#[async_trait]
impl CloudStorage for AwsClient {
    async fn upload(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<(), String> {
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(data.into())
            .send()
            .await
            .map_err(|e| format!("AWS Upload Failed: {}", e))?;

        Ok(())
    }
}