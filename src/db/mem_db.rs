use crate::models::task::Task;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Types
pub type Db = Arc<Mutex<HashMap<usize, Task>>>;

// Functions
pub fn init_db() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}
