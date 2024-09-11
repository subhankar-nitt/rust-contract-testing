use std::fmt::format;

mod user_test;
mod cloud_storage;
mod idtoken;

#[tokio::test]
async fn user_test() -> Result<(),Box<dyn std::error::Error>>{

    let consumer_name:&str = "consumer";
    let producer_name:&str = "producer";
    let provider_url:&str ="https://rust-server-986655996669.us-central1.run.app";
    let test_name:&str = "user";

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
        "body":{"id":1,"user_name":"subhankar","comment":"user added "}
     }
     }"#,

    r#"{
        "provider_state":"Get User With ID 2",
        "request":{
            "method":"Get",
            "path":"/users/2",
            "headers": {
                "Authorization": ""
            }
        },
         "response":
            {"statusCode":200,
            "headers":{"Content-Type":"application/json"},
            "body":{"id":2,"user_name":"biswas","comment":"user added "}
         }
         }"#


    ],consumer_name,producer_name,provider_url,test_name)
        .await.expect("calling init function ");

    user_test::contract_consumer().await.expect("Consumer failed");
    let err = cloud_storage::uploadFile(format!("pacts/{}_test/{}-{}.json",test_name,consumer_name,producer_name)).await;
    if err.is_err(){

        println!("Error Saving file :{:?}",err);
    }
    user_test::contract_provider().await.expect("Provider failed");


    Ok(())
}
