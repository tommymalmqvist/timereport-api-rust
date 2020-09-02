extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, CreateTableInput, CreateTableOutput, DynamoDb, DynamoDbClient,
    KeySchemaElement, ListTablesInput,
};

async fn create_table(
    client: &DynamoDbClient,
    table: CreateTableInput,
) -> Result<CreateTableOutput, String> {
    let create = client.create_table(table);
}

fn create_table_input(s: &str) -> CreateTableInput {
    let attributes: Vec<AttributeDefinition> = vec![
        AttributeDefinition {
            attribute_name: "user_id".to_string(),
            attribute_type: "S".to_string(),
        },
        AttributeDefinition {
            attribute_name: "event_date".to_string(),
            attribute_type: "S".to_string(),
        },
        AttributeDefinition {
            attribute_name: "reason".to_string(),
            attribute_type: "S".to_string(),
        },
        AttributeDefinition {
            attribute_name: "hours".to_string(),
            attribute_type: "S".to_string(),
        },
    ];

    let key_schema: Vec<KeySchemaElement> = vec![
        KeySchemaElement {
            attribute_name: "user_id".to_string(),
            key_type: "HASH".to_string(),
        },
        KeySchemaElement {
            attribute_name: "event_date".to_string(),
            key_type: "RANGE".to_string(),
        },
    ];

    let table_request = CreateTableInput {
        table_name: String::from(s),
        attribute_definitions: attributes,
        key_schema: key_schema,
        ..Default::default()
    };

    table_request
}

#[tokio::main]
async fn main() {
    let client = DynamoDbClient::new(Region::Custom {
        name: "eu-east-1".to_owned(),
        endpoint: "http://localhost:8000".to_owned(),
    });
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).await {
        Ok(o) => match o.table_names {
            Some(table_names) => {
                if table_names.len() > 0 {
                    println!("Tables in database:");
                    for table_name in table_names {
                        println!("{}", table_name);
                    }
                } else {
                    println!("No tables in database!")
                }
            }
            None => println!("Could not retrieve tables!"),
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
