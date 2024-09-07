use std::collections::HashMap;
use std::panic;
use std::str::FromStr;
use std::{fs::File, io::Read};

use axum::body::Bytes;
use axum::http::{ HeaderName, HeaderValue};
use pact_matching::headers::match_headers;

use pact_models::pact::load_pact_from_json;

use pact_models::prelude::MatchingRuleCategory;
use reqwest::{Client, Method};

use serde_json::{from_str, to_string, Value};

use pact_matching::{ match_status, match_text, CoreMatchingContext, MatchingContext};

#[tokio::test]
async fn test_provider() -> Result<(),Box<dyn std::error::Error>> {
    let  provider_url = "http://localhost:7878";
    let mut contract_file = File::open("target/pacts/consumer-provider.json")?;
    let mut contract_content = String::new();

    
    contract_file.read_to_string(&mut contract_content).unwrap();
    let pact_json:Value = from_str(&contract_content).expect("Failed to parse");



    let pact = load_pact_from_json("target/pacts/consumer-provider.json", &pact_json)?;

    for inter in pact.interactions(){

        let req_res = inter.as_request_response();
        if let Some(req_res) = req_res{
            let req = req_res.request;
            let res= req_res.response;

            let client = Client::new();
            let mut builder = client.request(Method::from_str(&req.method)?, format!("{}{}",provider_url,req.path));
            
            if let Some(headers) = req.headers{
                for (key,value) in headers{

                    let header = value.get(0);
                    if let Some(header) =header{

                        builder = builder.header(HeaderName::from_str(&key)?,HeaderValue::from_str(&header)? );
                    }                    
                }   
                // builder = builder.headers(headers)
            }

            let body = req.body;

            if body.is_present(){
                let body_json = body.to_string();

                builder = builder.body(body_json);
            }

            let  response = builder.send().await?;

            // let dup = response.borrow_mut();

            // let mut pact_response = Response::default_response();
            let mut expec_header :HashMap<String,Vec<String>>= HashMap::new(); 
            let mut actual_headers:HashMap<String,Vec<String>> = HashMap::new();

            let matching_context = CoreMatchingContext::default() ;
            let boxed: Box<dyn MatchingContext> = Box::new(matching_context);

            if let Some(expected_header) = res.headers{
                expec_header.clone_from(&expected_header);
            }

            for (key,header) in response.headers(){

                let header_value = header.to_str()?;
                let header_vec: Vec<String> = vec![header_value.to_string()];

                actual_headers.insert(key.as_str().to_string(), header_vec);

                // pact_response.add_header(key.as_str(), header_vec);
            }

            let header_match = match_headers(Some(expec_header), Some(actual_headers), boxed.as_ref().clone_with(&MatchingRuleCategory::empty("rule")).as_ref());

            if !header_match.is_empty(){
                for (key,value) in header_match{
                    if !value.is_empty(){

                        panic!("header value {:?} mismatch {:?}",key,value);
                        
                    }
                }
            }

            if response.status().is_success(){

                let status_code = response.status();
                // pact_response.status=status_code.as_u16();

                let status_match = match_status(status_code.as_u16(),res.status,boxed.as_ref());
                if status_match.is_err(){
                    println!("Status Error");
                    panic!("status error {:?}",status_match.err());
                }
            }else{
                println!("request not successful");
            }

            
            let body = response.text().await?;
            // let json_string = serde_json::to_string(&body)?;
            let mut json_data: serde_json::Value = from_str(&body).unwrap();

            if let Value::Object(ref mut obj)= json_data{
                let mut keys: Vec<&String> = obj.keys().collect();
                keys.sort();

                let mut soreted_obj = serde_json::Map::new();

                for key in keys {
                    soreted_obj.insert(key.to_string(), obj.get(key).unwrap().clone());
                }
                *obj=soreted_obj;
            }

            let res_body = to_string(&json_data).unwrap();
            let expected_body = res.body.str_value().to_string();

            // println!("{:?},  {:?}",res_body,expected_body);
            // let are_equal = res_body==expected_body;

            let body_match = match_text(&Some(Bytes::from(res_body)), &Some(Bytes::from(expected_body)), boxed.as_ref());

            
            if body_match.is_err(){
                panic!("Body Error {:?}",body_match.err());
                // body_match.err();
            }


            // pact_response.body=OptionalBody::from(body);

            // let bytes_expected = Bytes::from(body);

            // let bytest_actual = res.body.value();

            // let body_match = match_text(&Some(bytes_expected), &bytest_actual, boxed.as_ref());

            // let body_match = match
            
            // if body_match.is_err(){
            //     println!("Body Error {:?}",body_match.err());
            // }

            // let headers = dup.headers();

            // println!("{:?}",pact_response);

            // println!("#####################################################################################################");

            // println!("{:?}",res);

            // println!("{:?}",body);




        }
        
    }
    // let pact:Value = from_str(&contract_content).expect("Failed to parse");

    // let interactions = pact.get("interactions");

    // if let Some(interactions) = interactions{
    //     let interactions = interactions.clone();
        
    //     let array = interactions.as_array();
        
    //     if let Some(array) = array{
    //         for ele in array{
    //             let request = ele.get("request").expect("Interaction should have a request object");
    //             let response = ele.get("response").expect("Interaction should have a response object");
                
    //             let client = Client::new();

                

                
    //         }
    //     }
        
    // }
    
    // for interaction in interactions{

    //     let request = interaction.get("request").expect("Interaction should have a request object");

    //     println!("{:?}",request);
    // }


    // print!("{:?}",interactions);
    
    Ok(())
    
}