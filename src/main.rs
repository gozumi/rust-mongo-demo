use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
struct DemoData {
    something: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = "mongodb://localhost:27017".to_string();

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    let demo_data_collection: Collection<DemoData> = client.database("stuff").collection("demo");

    let new_data = DemoData {
        something: "from rust - yahoo - it is working!!!!!".to_string(),
    };

    // let insert_result = movies.insert_one(new_doc.clone(), None).await?;
    let insert_result = demo_data_collection.insert_one(new_data, None).await;
    println!("New document ID: {}", insert_result.unwrap().inserted_id);

    Ok(())
}
