
mod user_test;
mod cloud_storage;
mod idtoken;

#[tokio::test]
async fn user_test() -> Result<(),Box<dyn std::error::Error>>{
    user_test::contract_consumer().await.expect("Consumer failed");
    user_test::contract_provider().await.expect("Provider failed");

    
    Ok(())
}