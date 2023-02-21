use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkToken {
    pub expiration: String,
    pub link_token: String,
    pub request_id: String,
}
