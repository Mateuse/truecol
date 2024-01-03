use mongodb::{Client, options::ClientOptions, Database};
use std::error::Error;

pub async fn connect() -> Result<Database, Box<dyn Error>> {
    let mongo_uri = "mongodb+srv://templates-dev-mac:Arfini21!@cluster0.3acbi0i.mongodb.net/?retryWrites=true&w=majority";

    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database("truecol"))
}