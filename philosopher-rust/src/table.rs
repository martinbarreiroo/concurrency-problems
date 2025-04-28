use std::sync::{Arc, Condvar, Mutex};

pub struct Table {
    forks: Arc<Mutex<Vec<bool>>>,
    condvar: Condvar
}


impl Table {
    pub fn new(number_of_forks: usize) -> Self {
        Table {
            forks: Arc::new(Mutex::new(vec![true; number_of_forks])),
            condvar: Condvar::new()
        }
    }

    pub fn acquire_forks(&self, i: usize) {
        let mut forks = self.forks.lock().unwrap();

        let left = Self::left(i);
        let right = Self::right(i);

        while !forks[left] || !forks[right] {
            forks = self.condvar.wait(forks).unwrap();
        }

        forks[left] = false;
        forks[right] = false;
        println!("philosopher {} is eating.", i)
    }

    pub fn release_fork(&self, i: usize) {
        let mut forks = self.forks.lock().unwrap();
        let left = Self::left(i);
        let right = Self::right(i);
        forks[left] = true;
        forks[right] = true;
        self.condvar.notify_one();
    }

    fn right(i: usize) -> usize {
        let right = (i + 1) % 5;
        right
    }

    fn left(i: usize) -> usize {
        let left = (i + 4) % 5;
        left
    }
}