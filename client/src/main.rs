use hello::greeter_client::GreeterClient;
use hello::HelloRequest;
use log::{info, warn};
use simple_logger::SimpleLogger;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //logger init
    SimpleLogger::new().init().unwrap();
    // get address
    let mut app_address: String = "0.0.0.0:4000".to_string();
    if std::env::var("SERVER_ADDRESS").is_ok() {
        app_address = std::env::var("SERVER_ADDRESS").unwrap();
        info!("sucess read address from env: {}", app_address);
    }
    else {
        warn!("Cannot read env, set standart address");
    }

    let address = format!("{}{}", "http://", app_address);
    let mut client = GreeterClient::connect(address).await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    info!("server response: {:?}", response);

    Ok(())
}