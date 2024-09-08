

use google_cloud_storage::{client::{Client, ClientConfig}, http::objects::upload::{Media, UploadObjectRequest, UploadType}};
use tokio::fs::File;


pub async fn uploadFile(file_name:String) ->Result<(),Box<dyn std::error::Error>>{

    let file = File::open("report.json").await?;
    

    let config = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(config);
    let upload_type = UploadType::Simple(Media::new(file_name));

    let uploaded = client.upload_object(&UploadObjectRequest {
        bucket: "contract-testing-report".to_string(),
        ..Default::default()
    }, file, &upload_type).await?;

    println!("{:?}",uploaded);

    Ok(())
}