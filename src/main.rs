extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

#[tokio::main]
async fn main() {
    let client = get_dynamodb_local_client();

    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).await {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                if table_name_list.is_empty() {
                    println!("No tables found");
                } else {
                    println!("Tables in database:");
                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                }
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

fn get_dynamodb_local_client() -> DynamoDbClient {
    let region = Region::Custom {
        name: "us-east-1".to_owned(),
        endpoint: "http://localhost:8000".to_owned(),
    };
    DynamoDbClient::new(region)
}
