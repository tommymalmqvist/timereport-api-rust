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
    let client = get_dynamodb_local_client();

    let list_tables_input: ListTablesInput = Default::default();

    let event_table = create_event_table_input();

    match client.list_tables(list_tables_input).await {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                if table_name_list.is_empty() {
                    println!("No tables found");
                    match client.create_table(event_table).await {
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

fn create_event_table_input() -> CreateTableInput {
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

    let event_table_input = CreateTableInput {
        table_name: "dev_event".to_string(),
        attribute_definitions: vec![attr_user_id, attr_event_date],
        key_schema: vec![key_user_id, key_event_date],
        billing_mode: Some("PROVISIONED".to_string()),
        provisioned_throughput: Some(provisioned_throughput),
        ..Default::default()
    };

    event_table_input
}

// fn create_event_table(input: CreateTableInput) ->
