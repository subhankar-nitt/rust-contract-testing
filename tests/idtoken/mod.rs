use std::process::Command;


use reqwest::{RequestBuilder, Response};

pub async fn generate_token(url:String,mut request_builder:RequestBuilder) -> Result<Response, Box<dyn std::error::Error>> {
    
    let output =Command::new("curl").arg("-H").arg("Metadata-Flavor: Google").arg(format!("http://metadata/computeMetadata/v1/instance/service-accounts/default/identity?audience={}",url)).output().expect("not found");

    if(!output.status.success()){

        panic!("error {}",String::from_utf8_lossy(&output.stderr));
    
    }
    let body = String::from_utf8_lossy(&output.stdout);
    // let client = Client::new();
    // let mut builder = client.request(Method::GET, "https://rust-server-986655996669.us-central1.run.app/users/1");
    request_builder = request_builder.bearer_auth(body);
    
    let response = request_builder.send().await?;
    println!("{:?}",response.status().as_str());
    return Ok((response));
}
