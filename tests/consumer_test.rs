
use pact_consumer::prelude::{HttpPartBuilder, PactBuilder, StartMockServer};
use reqwest::Client;
use serde_json::from_str;


#[tokio::test]
async fn test_get_user() -> Result<(),Box<dyn std::error::Error>>{

    let mut pact = PactBuilder::new("consumer", "provider");
    
    let interaction = pact.interaction("Get user by ID", "", |mut builder|{
            builder.given("An user exists with id 1");

            builder.request.path("/users/1");

            builder.request.method("GET");


            builder.response.content_type("application/json").body(r#"{
            "id": 1,
            "user_name": "subhankar",
            "comment": "user added "
            }"#);

        builder

                
    })

    .interaction("Get user by ID", "", |mut builder|{
        builder.given("An user exists with id 2");

        builder.request.path("/users/2");

        builder.request.method("GET");

        builder.response.content_type("application/json").body(r#"{
        "id": 2,
        "user_name": "biswas",
        "comment": "user added "
        }"#);

        builder

                
    })
    
    .start_mock_server(None,None);

    let url = interaction.url();
    let response1 = Client::new().get(url.join("/users/1").unwrap()).send().await?;
    let response2 = Client::new().get(url.join("/users/2").unwrap()).send().await?;
    
    let resp1 = response1.text().await?;
    let _resp2 = response2.text().await?;

    let json_string = serde_json::to_string(&resp1)?;
    let json_data: serde_json::Value = from_str(&json_string)?;
    println!("{:?}",json_data);


    Ok(())

}