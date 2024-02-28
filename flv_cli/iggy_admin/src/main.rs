use common::prelude::IggyUser;
use iggy::client::UserClient;
use iggy::users::login_user::LoginUser;

#[tokio::main]
async fn main() {
    println!("Hello, Iggy!");
    let client = iggy_utils::get_iggy_client()
        .await
        .expect("Failed to build iggy client");

    let user = IggyUser::default();
    iggy_utils::init(&client, &user)
        .await
        .expect("Failed to init");

    let new_user = IggyUser::new("qdgw", "qdgw");

    println!("Creating new user...");
    iggy_utils::create_user(&client, &new_user)
        .await
        .expect("Failed to create new user");

    println!("Login new user...");
    client
        .login_user(&LoginUser {
            username: new_user.username().to_string(),
            password: new_user.password().to_string(),
        })
        .await
        .expect("Failed to login user");

    println!("Creating new token...");
    let token = iggy_utils::create_token(&client, "qdgw_token".to_string())
        .await
        .expect("Failed to create token");

    println!("Token: {}", token);

    iggy_utils::shutdown(&client)
        .await
        .expect("Failed to shutdown");
}
