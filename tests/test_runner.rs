
mod user_test;
mod cloud_storage;
mod idtoken;

#[tokio::test]
async fn user_test() -> Result<(),Box<dyn std::error::Error>>{

    let consumer_name:&str = "consumer";
    let producer_name:&str = "producer";
    let provider_url:&str ="https://rust-server-986655996669.us-central1.run.app";
    
    user_test::push_data( vec![r#"
    {
    "provider_state":"Get User With ID 1",
    "request":{
        "method":"Get",
        "path":"/users/1"
    },
     "response":
        {"statusCode":200,
        "headers":{"Content-Type":"application/json"},
        "body":{"id":1,"user_name":"subhankar","comment":"user_added "}
     }
     }"#,

                               r#"{
        "provider_state":"Get User With ID 2",
        "request":{
            "method":"Get",
            "path":"/users/2"
        },
         "response":
            {"statusCode":200,
            "headers":{"Content-Type":"application/json"},
            "body":{"id":2,"user_name":"biswas","comment":"user_added "}
         }
         }"#


    ],consumer_name,producer_name,provider_url)
        .await.expect("calling init function ");
    user_test::contract_consumer().await.expect("Consumer failed");
    // let err = cloud_storage::uploadFile("pacts/users/consumer-producer.json".to_string()).await;
    // if err.is_err(){

    //     println!("Error Saving file :{:?}",err);
    // }
    // user_test::contract_provider().await.expect("Provider failed");

    
    Ok(())
}