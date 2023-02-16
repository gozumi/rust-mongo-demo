use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection, bson::Document,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio;
use mongodb::bson::doc;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
struct DemoData {
    something: String,
    time_stamp: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017".to_string();

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    let demo_data_collection: Collection<Document> = client.database("stuff").collection("demo");

    let now = Utc::now();
    let new_data = doc! {
        "something": "cold".to_string(),
        "time_stamp": now.to_rfc3339()
    };
     
    let insert_result = demo_data_collection.insert_one(new_data, None).await;
    println!("New document ID: {}", insert_result.unwrap().inserted_id);

    let db_item = match demo_data_collection.find_one(doc! {"something": "hot"}, None).await {
        Ok(item_option) => match item_option {
            Some(item) => item,
            None => panic!("ahhh!"),
        },
        Err(error) => panic!("{}", error),
    };

    println!("Found document is: {:?}", db_item); 


    Ok(())
}
