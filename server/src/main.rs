use tonic::{transport::Server, Request, Response, Status};
use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};
use log::{info};
use simple_logger::SimpleLogger;

mod postgres_commands;
pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,)
        -> Result<Response<HelloReply>, Status> {

        info!("Got a request from {:?}", request.remote_addr().unwrap());
        println!("GOTCHA");
        // data for postgres
        let client_address = request.remote_addr().unwrap();
        let message_to_client = format!("Hello {}!", request.into_inner().name);
        // send reply
        let reply = hello::HelloReply {
            message: message_to_client.clone(),
        };
        // send data to postgres
        //postgres_commands::postgres_work(message_to_client, client_address.to_string());

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // logger init
    SimpleLogger::new().init().unwrap();
    // подключение grpc сервера
    let mut app_address: String = "0.0.0.0:4000".to_string();
    if std::env::var("APP_ADDRESS").is_ok() {
        app_address = std::env::var("APP_ADDRESS").unwrap();
        info!("sucess read address from env: {}", app_address);
    }
    else {
        info!("Cannot read env, set standart address");
    }

    //let addr = app_address.parse()?;
    let addr = "[::1]:4000".parse()?;
    let greeter = MyGreeter::default();

    info!("Server work well and listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}