use dynamo;

fn main() {
    println!("timereport-api in rust");

    let client = dynamo::get_dynamodb_local_client();
    dynamo::bootstrap(&client)
}
