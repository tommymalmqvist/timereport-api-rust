extern crate futures;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use futures::future::Future;
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, AttributeValue, CreateTableInput, CreateTableOutput, DynamoDb,
    DynamoDbClient, GetItemError, GetItemInput, GetItemOutput, KeySchemaElement, ListTablesInput,
    ProvisionedThroughput, UpdateItemInput, UpdateItemOutput,
};

#[tokio::main]
async fn main() {
    // create client
    let client = get_dynamodb_local_client();

    // get all existing tables
    let list_tables_input: ListTablesInput = Default::default();

    // prepare input if tables doesn't exist
    let event_table = create_table_input("dev_event".to_string());
    let lock_table = create_table_input("dev_lock".to_string());

    // get all tables
    match client.list_tables(list_tables_input).await {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                if table_name_list.is_empty() {
                    // create missing tables
                    println!("No tables found, creating");
                    match client.create_table(event_table).await {
                        Ok(val) => val,
                        Err(e) => panic!("Could not create table, {}", e),
                    };
                    match client.create_table(lock_table).await {
                        Ok(val) => val,
                        Err(e) => panic!("Could not create table, {}", e),
                    };
                } else {
                    println!("Tables in database:");
                    for table_name in table_name_list {
                        println!("{}", table_name);
                    }
                }
            }
            None => println!("No result"),
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

fn create_table_input(table_name: String) -> CreateTableInput {
    let provisioned_throughput = ProvisionedThroughput {
        read_capacity_units: 1,
        write_capacity_units: 1,
    };

    let attr_user_id = AttributeDefinition {
        attribute_name: "user_id".to_string(),
        attribute_type: "S".to_string(),
    };

    let attr_event_date = AttributeDefinition {
        attribute_name: "event_date".to_string(),
        attribute_type: "S".to_string(),
    };

    let key_user_id = KeySchemaElement {
        attribute_name: "user_id".to_string(),
        key_type: "HASH".to_string(),
    };
    let key_event_date = KeySchemaElement {
        attribute_name: "event_date".to_string(),
        key_type: "RANGE".to_string(), // case sensitive
    };

    let table_input = CreateTableInput {
        table_name: table_name,
        attribute_definitions: vec![attr_user_id, attr_event_date],
        key_schema: vec![key_user_id, key_event_date],
        billing_mode: Some("PROVISIONED".to_string()),
        provisioned_throughput: Some(provisioned_throughput),
        ..Default::default()
    };

    table_input
}
