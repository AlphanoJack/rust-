// simple model layer
// mock store layer

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
// Arc 스레드 간에 데이터 공유를 위해 선언
// Mutex 한번에 하나의 스레드만 접근 가능하도록 보호

// region Ticken type
#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}
// endregion Ticket type

