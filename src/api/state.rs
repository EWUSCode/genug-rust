use std::sync::{Arc, Mutex};

use axum::extract::FromRef;

use crate::db::ConnectionPool;

#[derive(Clone)]
pub struct AppState {
    counter: Counter,
    pool: ConnectionPool,
}

impl AppState {
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            counter: Counter::new(),
            pool,
        }
    }
}

#[derive(Clone)]
pub struct Counter {
    value: Arc<Mutex<u64>>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }

    pub fn inc_and_get(self) -> u64 {
        let r: u64;
        {
            let mut num = self.value.lock().unwrap();
            *num += 1;
            r = *num;
        }
        r
    }
}

impl FromRef<AppState> for Counter {
    fn from_ref(app_state: &AppState) -> Counter {
        app_state.counter.clone()
    }
}

impl FromRef<AppState> for ConnectionPool {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.pool.clone()
    }
}
