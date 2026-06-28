pub mod config;

use aws_config::SdkConfig;
use aws_sdk_s3::Client;
use async_trait::async_trait;

#[async_trait]
pub trait CloudStorage {
    /// อัปโหลดไฟล์ขึ้นสู่ Cloud Storage
    async fn upload(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<u16, String>;
    
    /// ดาวน์โหลดหรืออ่านเนื้อหาไฟล์จาก Bucket (Get/Download file)
    async fn download_file(&self, bucket: &str, key: &str) -> Result<Vec<u8>, String>;
    
    /// ดึงรายชื่อไฟล์ทั้งหมดใน Bucket (Select all files)
    async fn list_files(&self, bucket: &str) -> Result<Vec<String>, String>;
    
    /// ลบไฟล์ออกจาก Bucket (Delete file)
    async fn delete_file(&self, bucket: &str, key: &str) -> Result<u16, String>;
}

pub struct AwsClient {
    client: Client,
}

impl AwsClient {
    /// สร้าง AwsClient โดยรับ SdkConfig จากภายนอก (Dependency Injection)
    pub fn new(config: &SdkConfig) -> Self {
        let client = Client::new(config);
        AwsClient { client }
    }

    /// Helper เสริม: สร้าง AwsClient โดยโหลด Config ตาม Region ที่ระบุทันที
    pub async fn from_region(region: &str) -> Self {
        let config = config::load_cloud_config(Some(region)).await;
        Self::new(&config)
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
            Ok(_) => Ok(200),
            Err(e) => {
                let status = e.raw_response().map(|r| r.status().as_u16()).unwrap_or(0);
                Err(format!("AWS Upload Failed (Status {}):\n{:#?}", status, e))
            }
        }
    }

    async fn list_files(&self, bucket: &str) -> Result<Vec<String>, String> {
        match self.client
            .list_objects_v2()
            .bucket(bucket)
            .send()
            .await
        {
            Ok(output) => {
                let keys = output
                    .contents()
                    .iter()
                    .filter_map(|obj| obj.key().map(|k| k.to_string()))
                    .collect();
                Ok(keys)
            }
            Err(e) => {
                let status = e.raw_response().map(|r| r.status().as_u16()).unwrap_or(0);
                Err(format!("AWS List Files Failed (Status {}):\n{:#?}", status, e))
            }
        }
    }

    async fn delete_file(&self, bucket: &str, key: &str) -> Result<u16, String> {
        match self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(200),
            Err(e) => {
                let status = e.raw_response().map(|r| r.status().as_u16()).unwrap_or(0);
                Err(format!("AWS Delete Failed (Status {}):\n{:#?}", status, e))
            }
        }
    }

    async fn download_file(&self, bucket: &str, key: &str) -> Result<Vec<u8>, String> {
        match self.client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
        {
            Ok(output) => {
                match output.body.collect().await {
                    Ok(data) => Ok(data.into_bytes().to_vec()),
                    Err(e) => Err(format!("Failed to read stream body:\n{:#?}", e)),
                }
            }
            Err(e) => {
                let status = e.raw_response().map(|r| r.status().as_u16()).unwrap_or(0);
                Err(format!("AWS Download Failed (Status {}):\n{:#?}", status, e))
            }
        }
    }
}