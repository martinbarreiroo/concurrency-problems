use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

struct Data {
    data: RwLock<Vec<usize>>
}

struct Reader {
    data: Arc<Data>
}

struct Writer {
    data: Arc<Data>,
    id: usize
}

impl Data {
    
    fn new() -> Self {
        Self {
            data: RwLock::new(Vec::new())
        }
    }
    
    fn read(&self) {
        self.data.read().unwrap();
    }
    
    fn write(&self, item: usize) {
       let mut data = self.data.write().unwrap();
       data.push(item); 
    }
}

impl Reader {
    fn new(data: Arc<Data>) -> Self {
        Self {
            data
        }
    }
    
    fn read(&self) {
        self.data.read();
        sleep(Duration::from_millis(1000));
    }
}

impl Writer {
    fn new(data: Arc<Data>, i: usize) -> Self {
        Self {
            data, id: i
        }
    }
    
    fn write(&self) {
        self.data.write(self.id);
        sleep(Duration::from_millis(1000));
    }
}
fn main() {
    
}