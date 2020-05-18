use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::config::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatusState {
    Green,
    Yellow,
    Red,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub config: Config,
    pub monitors: Vec<MonitorState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitorState {
    pub config: MonitorDirConfig,
    pub status: MonitorStatus,
    pub log: Arc<Mutex<VecDeque<String>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitorStatus {
    pub status: StatusState,
    pub metadata: Arc<HashMap<String, String>>,
    pub code: i64,
    pub description: String,
}
