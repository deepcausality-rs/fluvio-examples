use std::error::Error;
use std::process;
use fluvio::FluvioAdmin;
use fluvio::metadata::objects::CommonCreateRequest;
use fluvio::metadata::topic::TopicSpec;
mod handle;
mod handle_login_error;
mod handle_logout_error;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {

    println!("Client Setup!");

    // Create a new admin client
    // Create new client topics:
    // 1. test-topic
    // 2. test-topic-control - For control messages
    // 3. test-topic-data - For receiving data messages

    // OK, I get this one.
    let admin = FluvioAdmin::connect().await.unwrap();

    // Create a new topic
    let name= String::from("test-topic");
    let dry_run = true;
    // WTF is this?
    let common_request = CommonCreateRequest {
        name,
        dry_run,
        ..Default::default()
    };

    // Ok, specify the topic config
    let topic_specs = TopicSpec::new_computed(1,1, None);

    // Create the topic
    admin.create_with_config(common_request, topic_specs)
        .await
        .expect("Failed to create topic");

    Ok(())
}
