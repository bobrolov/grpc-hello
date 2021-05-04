
use tokio_postgres::{NoTls, Error, Connection};
use simple_logger::SimpleLogger;
use log::{info, warn};
/*
pub struct postgres_connection{
    postgres_client: Option<Client>,
    postgres_write: Option<Statement>,
}





pub fn create_setting_connect() -> Result<Config, Error> {
    let param = "host=localhost user=postgres password=somePassword";
    tokio_postgres::Config::from_str(param)
}
pub fn create_table() {

}

pub fn write_to_table(message: String, client: String, datatime: String) {

}

fn postgres_write(message: String, client: String) -> Result<(), Box<dyn std::error::Error>> {
    let config = "host=localhost, user=postgres";
    let mut client = Client::connect(config, NoTls)?;

    client.batch_execute("\
    CREATE TABLE grpc_messages (\
       id       SERIAL PRIMARY KEY,\
       message  TEXT NOT NULL,\
       client   TEXT NOT NULL\
       )"
    )?;

    let message = "message";
    let clients = "client";
    client.execute("\
    INSERT INTO grpc_messages (message, client) VALUES ($1, $2",
                   &[&message, &clients]
    )?;

    Ok(())
}
*/

#[tokio::main]
pub async fn postgres_work(message: String, client_address: String) -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();
    let mut postgres_config;
    if std::env::var("POSTGRES_CONFIG").is_ok() {
        info!("config for postgres in env OK");
        postgres_config = std::env::var("POSTGRES_CONFIG").unwrap();
        info!("read from env OK");
    }
    else {
        warn!("config for postgres in env NOT OK");
        postgres_config = "host=localhost, user=postgres".to_string();
    }

    let (client, connection) = tokio_postgres::connect(postgres_config.as_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("INSERT INTO grpc_table (message, client) VALUES ($1, $2)", &[&message, &client_address])
        .await?;

    let value: &str = rows[0].get(0);

    assert_eq!(value, "hello World");

    Ok(())
}