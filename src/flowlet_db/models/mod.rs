use serde::Serialize;

use crate::{flowlet_context::FlowletContext, util::FlowletResult};

pub mod auth;
pub mod command;
pub mod user;

/// A trait that all models should implement
pub trait Api: Sized {
    #![allow(async_fn_in_trait)]
    type CreateInput: Serialize;

    async fn create(
        flowlet_context: &FlowletContext,
        input: Self::CreateInput,
    ) -> FlowletResult<Self>;

    type UpdateInput: Serialize;
    async fn update(
        flowlet_context: &FlowletContext,
        input: Self::UpdateInput,
    ) -> FlowletResult<Self>;

    type ReadInput: Serialize;
    async fn read(
        flowlet_context: &FlowletContext,
        input: Self::ReadInput,
    ) -> FlowletResult<Option<Self>>;
}
