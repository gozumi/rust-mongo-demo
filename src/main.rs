use chrono::Utc;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{
    bson::Document,
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
struct DemoData {
    something: String,
    time_stamp: String,
}

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug)]
struct StuffData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    something: Option<String>,
    time_stamp: Option<String>,
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

    // let demo_data_collection: Collection<Document> = client.database("stuff").collection("demo");
    let demo_data_collection: Collection<StuffData> = client.database("stuff").collection("demo");

    let now = Utc::now();
    let new_data = StuffData {
        id: None,
        something: Some("warm".to_string()),
        time_stamp: Some(now.to_rfc3339()),
    };

    let insert_result = demo_data_collection.insert_one(new_data, None).await;
    println!("New document ID: {}", insert_result.unwrap().inserted_id);

    let db_item = demo_data_collection.find_one(doc! {"foo": "bar"}, None).await?;

    println!("Found document is: {:?}", db_item);

    Ok(())
}
