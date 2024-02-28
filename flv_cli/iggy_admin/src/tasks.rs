use iggy::clients::client::IggyClient;
use std::error::Error;
use iggy::personal_access_tokens::create_personal_access_token::CreatePersonalAccessToken;
use iggy::users::create_user::CreateUser;
use iggy::models::user_status::UserStatus;
use iggy::client::{PersonalAccessTokenClient, UserClient};
use crate::types::User;


/// `create_user` command is used to create a new user.
pub(crate) async fn create_user(client: &IggyClient, user: &User) -> Result<(), Box<dyn Error>> {

    match client.create_user(&CreateUser{
        username: user.username().to_string(),
        password: user.password().to_string(),
        status: UserStatus::Active,
        permissions: None,
    }).await
    {
        Ok(_) => println!("User created."),
        Err(_) => println!("User already exists."),
    }

    Ok(())
}

/// `create_token` command is used to create a new personal access token.
pub(crate) async fn create_token(client: &IggyClient, token_name: String) -> Result<(), Box<dyn Error>> {
    match client.create_personal_access_token(&CreatePersonalAccessToken{
        name: token_name,
        expiry: None,
    }).await
    {
        Ok(_) => println!("Token created."),
        Err(_) => println!("Token already exists."),
    }

    Ok(())
}
