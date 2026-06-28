use aws_config::{BehaviorVersion, Region, SdkConfig};

/// โหลดการตั้งค่า AWS (SdkConfig) โดยสามารถระบุ Region ได้
/// หากไม่ระบุ (`None`) จะทำการโหลดจาก Environment variables หรือ AWS Profile อัตโนมัติ
pub async fn load_cloud_config(region: Option<&str>) -> SdkConfig {
    let mut loader = aws_config::defaults(BehaviorVersion::latest());
    
    if let Some(r) = region {
        loader = loader.region(Region::new(r.to_string()));
    }
    
    loader.load().await
}
