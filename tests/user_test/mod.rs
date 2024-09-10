
use pact_consumer::prelude::{HttpPartBuilder, PactBuilder, StartMockServer};
use reqwest::Client;
use serde_json::{from_str, Map};

use std::collections::HashMap;
use std::panic;
use std::str::FromStr;
use std::{fs::File, io::Read};

use axum::body::Bytes;
use axum::http::{HeaderName, HeaderValue};

use pact_matching::headers::match_headers;

use pact_models::pact::load_pact_from_json;

use http::response::Builder;
use pact_models::prelude::MatchingRuleCategory;
use reqwest::{Method, Response};

use serde_json::{to_string, Value};

use pact_matching::{match_status, match_text, CoreMatchingContext, MatchingContext};
use crate::idtoken;

use std::sync::{ Mutex};
use http::StatusCode;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize)]
struct Requests{

    method: String,
    path: String,
    headers: Option<Map<String,Value>>,
    body: Option<Map<String,Value>>

}

#[derive(Debug)]
struct Responses{

    status_code: StatusCode,
    headers: Map<String,Value>,
    body: Map<String,Value>,

}

#[derive(Debug)]
struct RequestResponsePair{
    provider_state: String,
    req: Requests,
    res: Responses,
}

#[derive(Debug)]
struct BasicData{
    consumer: String,
    producer: String,
    url:String
}

impl BasicData {
    fn assign_value(&mut self, consumer:&str, provider:&str, url:&str) -> &BasicData {
        self.producer= provider.to_string();
        self.consumer= consumer.to_string();
        self.url = url.to_string();

        (self)
    }

    fn get_consumer(&self)->&str{
        &self.consumer.as_str()
    }

    fn get_provider(&self)->&str{
        &self.producer.as_str()
    }
    fn get_url(&self)->&str{
        &self.url.as_str()
    }
}

impl RequestResponsePair {
    pub fn get_request(&self) -> &Requests {
        &self.req
    }

    pub fn get_respones(&self) ->&Responses{
        &self.res
    }

    pub fn get_provider_state(&self) ->&String{
        &self.provider_state
    }
}

lazy_static!{
    static ref REQ_RES_LIST: Mutex<Vec<RequestResponsePair>> = Mutex::new(Vec::new());
    static ref PATHS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref BASIC_DATA: Mutex<BasicData> = Mutex::new(BasicData{consumer:String::new(),producer:String::new(),url:String::new()});
}


pub async fn push_data(data_list: Vec<&str>,consumer:&str,producer:&str,provider_url:&str) ->Result<(), Box<dyn std::error::Error>>{
    let mut vec = REQ_RES_LIST.lock().unwrap();
    let mut paths = PATHS.lock().unwrap();
    let mut basic_data = BASIC_DATA.lock().unwrap();

    basic_data.assign_value(consumer, producer, provider_url);

    for item in data_list{
        let data: Value = from_str(item).unwrap();

        let provider_state = data["provider_state"].as_str().unwrap().to_owned();
        let method = data["request"]["method"].as_str().unwrap().to_owned();
        let path = data["request"]["path"].as_str().unwrap().to_owned();
        paths.push(path.clone());
        let headers = match data["request"]["headers"].as_object()==None{
            true => None,
            false => Some(data["request"]["headers"].as_object().unwrap().clone()),
        };
        let body = match data["request"]["headers"].as_object()==None{
            true => None,
            false => Some(data["request"]["body"].as_object().unwrap().clone()),
        };


        let request = Requests{method,path,headers,body };

        let status_code = data["response"]["statusCode"].as_u64().unwrap() as u16;
        let response_headers = data["response"]["headers"].as_object().unwrap().clone();
        let response_body = data["response"]["body"].as_object().unwrap().clone();

        let statusCode = StatusCode::from_u16(status_code)?;

        let response = Responses{status_code:statusCode,headers:response_headers,body:response_body};
        let req_response = RequestResponsePair{
            provider_state,
            req:request,
            res: response
        };

        
        vec.push(req_response);
    }


    Ok(())
}
pub async fn contract_consumer() -> Result<(),Box<dyn std::error::Error>>{

    let basic_data = BASIC_DATA.lock().unwrap();
    let vec = REQ_RES_LIST.lock().unwrap();
    println!("{:?}",basic_data.get_consumer());
    let mut pact = PactBuilder::new(basic_data.get_consumer(), basic_data.get_provider());


     let mut pact = pact.with_output_dir("pacts/users");

    for interaction in vec.iter() {
        let provider_state = interaction.get_provider_state();
        let req = interaction.get_request();
        let res = interaction.get_respones();
        pact = pact.interaction("Get User_test by ID","", |mut builder|{
            builder.given(provider_state);
            builder.request.path(req.path.as_str());
            builder.request.method(req.method.as_str());
            
            if let Some(headers) = req.headers.clone(){

                for (key,value) in headers.iter(){
                    builder.request.header(key,value.to_string());
                }
            }

            if let Some(body) = req.body.clone(){

                builder.request.json_body(body);
            }

            for (key,value) in res.headers.iter(){
                builder.response.header(key,value.to_string());
            }
            builder.response.json_body(res.body.clone());
            builder.response.status(res.status_code.as_u16());

            builder
        });
    }

    let pact = pact.start_mock_server(None,None);

    // let mut pact =pact.interaction("Get user_test by ID", "", |mut builder|{
    //     builder.given("An unit exists with id 1");
    //
    //     builder.request.path("/users/1");
    //
    //     builder.request.method("GET");
    //
    //     builder.response.content_type("application/json").body(r#"{
    //     "id": 1,
    //     "user_name": "subhankar",
    //     "comment": "user added "
    //     }"#);
    //
    //     builder
    //
    //
    // })
    //
    //     .interaction("Get user_test by ID", "", |mut builder|{
    //         builder.given("An unit exists with id 2");
    //
    //         builder.request.path("/users/2");
    //
    //         builder.request.method("GET");
    //
    //         builder.response.content_type("application/json").body(r#"{
    // "id": 2,
    // "user_name": "biswas",
    // "comment": "user added "
    // }"#);
    //
    //         builder
    //
    //
    //     })
    //
    //     .start_mock_server(None,None);

    let url = pact.url();

    let paths = PATHS.lock().unwrap();
    for path in paths.iter(){
        let  response = Client::new().get(url.join(&path).unwrap()).send().await?;
        let resp = response.text().await?;
        let json_string = serde_json::to_string(&resp)?;
        let _json_data: serde_json::Value = from_str(&json_string)?;
        // println!("{:?}",json_data);
        
    }
    // let response1 = Client::new().get(url.join("/users/1").unwrap()).send().await?;
    // let response2 = Client::new().get(url.join("/users/2").unwrap()).send().await?;

    // let resp1 = response1.text().await?;
    // let _resp2 = response2.text().await?;

    // let json_string = serde_json::to_string(&resp1)?;
    // let json_data: serde_json::Value = from_str(&json_string)?;
    // println!("{:?}",json_data);

    Ok(())

}


pub async fn contract_provider() -> Result<(),Box<dyn std::error::Error>> {
    let basic_data = BASIC_DATA.lock().unwrap();
    let  provider_url = basic_data.get_url();
    let mut contract_file = File::open("pacts/users/consumer-producer.json")?;
    let mut contract_content = String::new();


    contract_file.read_to_string(&mut contract_content).unwrap();
    let pact_json:Value = from_str(&contract_content).expect("Failed to parse");



    let pact = load_pact_from_json("pacts/users/consumer-producer.json", &pact_json)?;

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
            let response = Builder::new()
                .status(200)
                .body("foo")
                .unwrap();

            let mut response = Response::from(response);

            if body.is_present(){
                let body_json = body.to_string();

                builder = builder.body(body_json);
            }

            if provider_url.to_string().contains(".run.app"){
                response = idtoken::generate_token(provider_url.to_string(), builder).await?;

            }else{

                response = builder.send().await?;
            }

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
                println!("Request not successful");
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

            let body_match = match_text(&Some(Bytes::from(res_body)), &Some(Bytes::from(expected_body)), boxed.as_ref());


            if body_match.is_err(){
                panic!("Body Error {:?}",body_match.err());
            }

        }

    }
    Ok(())

}



