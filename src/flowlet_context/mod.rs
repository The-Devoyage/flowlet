use thiserror::Error;

use crate::{api_client::ApiClient, flowlet_db::FlowletDb, util::FlowletResult};

pub struct FlowletContext {
    pub flowlet_db: FlowletDb,
    pub api_client: ApiClient,
}

pub trait WithContext {
    fn get(&self) -> &FlowletContext;
}

#[derive(Debug, Error)]
pub enum FlowletContextError {}

impl FlowletContext {
    pub async fn new() -> FlowletResult<Self> {
        // Start Local DB
        let flowlet_db = FlowletDb::new().await?;

        // GET AUTH TOKEN FROM LOCAL DEEB DB
        // let auth...

        // CREATE A CLIENT WITH AUTH TOKEN IN HEADERS
        // Start Cloud DB
        let api_client = ApiClient::new("http://localhost:8080")?;

        Ok(Self { flowlet_db, api_client })
    }
}
