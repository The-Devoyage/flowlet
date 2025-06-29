use dialoguer::{Input, Password};
use serde_json::json;
use thiserror::Error;

use crate::{
    api_client::EmptyData,
    flowlet_context::WithContext,
    flowlet_db::models::{self, Api, auth::CreateAuthInput},
    printer::Printer,
    util::FlowletResult,
};

pub struct Auth;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to register user.")]
    RegisterFailed,
}

impl Auth {
    pub async fn register(ctx: &impl WithContext) -> FlowletResult<()> {
        let client = &ctx.get().api_client;

        // Prompt User
        let email: String = Input::new().with_prompt("Email").interact_text().unwrap();
        let password = Password::new()
            .with_prompt("Password")
            .with_confirmation("Confirm password", "Passwords do not match")
            .interact()
            .unwrap();

        client
            .post::<_, EmptyData>(
                "/auth/register",
                &json!({"email": email, "password": password}),
            )
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                AuthError::RegisterFailed
            })?;

        Ok(())
    }

    pub async fn login(ctx: &impl WithContext) -> FlowletResult<()> {
        let email: String = Input::new().with_prompt("Email").interact_text().unwrap();
        let password = Password::new().with_prompt("Password").interact().unwrap();

        models::auth::Auth::create(ctx.get(), CreateAuthInput { email, password }).await?;

        println!("Login successful!");
        Printer::success("Auth", "Login Successful!");

        Ok(())
    }
}
