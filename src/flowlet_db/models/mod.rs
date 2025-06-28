use serde::Serialize;

use crate::{flowlet_context::FlowletContext, util::FlowletResult};

pub mod auth;
pub mod user;

/// A trait that all models should implement
pub trait Api: Sized {
    #![allow(async_fn_in_trait)]
    type CreateInput: Serialize;

    async fn create(flowlet_context: &FlowletContext, input: Self::CreateInput) -> FlowletResult<Self>;
}
