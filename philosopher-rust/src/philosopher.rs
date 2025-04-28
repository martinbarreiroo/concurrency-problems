use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crate::table::Table;

pub struct Philosopher {
    table: Arc<Table>, // monitor
}
impl Philosopher {
    pub fn new(table: Arc<Table>) -> Self {
        Philosopher {
            table
        }
    }

    pub fn eat(&self, k: usize) {
        self.table.acquire_forks(k);
        sleep(Duration::from_millis(8000));
        self.table.release_fork(k);
    }

    pub fn think(&self) {
        sleep(Duration::from_millis(10000));
    }

}