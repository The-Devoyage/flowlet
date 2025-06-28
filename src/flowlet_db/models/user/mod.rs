use deeb::Collection;
use deeb::*;
use serde::{Deserialize, Serialize};

#[derive(Collection, Deserialize, Serialize)]
pub struct User {
    pub _id: ulid::Ulid,
}
