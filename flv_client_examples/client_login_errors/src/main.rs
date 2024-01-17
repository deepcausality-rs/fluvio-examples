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

    let admin = FluvioAdmin::connect().await.unwrap();

    let name= String::from("test-topic");
    let dry_run = true;
    let common_request = CommonCreateRequest {
        name,
        dry_run,
        ..Default::default()
    };

    let topic_specs = TopicSpec::new_computed(1,1, None);

    admin.create_with_config(common_request, topic_specs)
        .await
        .expect("Failed to create topic");

    Ok(())
}
