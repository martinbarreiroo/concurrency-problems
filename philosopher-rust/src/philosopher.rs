use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crate::table::Table;

pub struct Philosopher {
    table: Arc<Table>, // monitor
    id: usize
}
impl Philosopher {
    pub fn new(table: Arc<Table>, id: usize) -> Self {
        Philosopher {
            table,
            id
        }
    }

    pub fn eat(&self) {
        self.table.acquire_forks(self.id);
        sleep(Duration::from_millis(8000));
        self.table.release_fork(self.id);
    }

    pub fn think(&self) {
        sleep(Duration::from_millis(10000));
    }

}