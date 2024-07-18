use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Command {
    GetUser { uid: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandResp {
    GetUserResp {
        uid: String,
        exists: bool,
        connected: bool,
    }
}
