// ### **Productor-Consumidor (Bounded Buffer)**
//
// - **Descripción:** Unos hilos producen datos y otros los consumen. El buffer tiene capacidad limitada.
// - **Conceptos:** sincronización, semáforos para controlar el acceso a la cola, condición de espera si está llena o vacía.
// si hay algun hilo produciendo, los consumers esperan

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;

struct Buffer {
    buffer: Mutex<VecDeque<usize>>,
    not_full: Condvar,
    not_empty: Condvar,
    capacity: usize
}

struct Producer {
    buffer: Arc<Buffer>,
    data: usize
}

struct Consumer {
    buffer: Arc<Buffer>
}


impl Buffer {
    fn new(capacity: usize) -> Buffer {
        Buffer{
            buffer: Mutex::new(VecDeque::new()),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
            capacity
        }
    }

    fn producer_access_buffer(&self, data: usize) {

        let mut buffer = self.buffer.lock().unwrap();

        while buffer.len() == self.capacity {
            buffer = self.not_full.wait(buffer).unwrap();
        }

        buffer.push_back(data);
        self.not_empty.notify_one();
    }

    fn consumer_access_buffer(&self) -> usize {
        let mut buffer = self.buffer.lock().unwrap();
        while buffer.is_empty() {
            buffer = self.not_empty.wait(buffer).unwrap();
        }

        let popped = buffer.pop_front().unwrap();
        self.not_full.notify_one();
        popped

    }


}

impl Producer {
    fn new(data: usize, buffer: Arc<Buffer>) -> Producer {
        Producer{buffer, data}
    }

    fn produce(&self) {
        self.buffer.producer_access_buffer(self.data);
        sleep(Duration::new(1, 0));
    }
}

impl Consumer {
    fn new(buffer: Arc<Buffer>) -> Consumer {
        Consumer{buffer}
    }

    fn consume(&self) -> usize {
        let consumed = self.buffer.consumer_access_buffer();
        sleep(Duration::new(1, 0));
        consumed
    }
}


fn main() {
    use std::thread;

    // Create shared buffer with limited capacity
    let buffer = Arc::new(Buffer::new(3));

    // Create producers
    let producer1 = Arc::new(Producer::new(10, Arc::clone(&buffer)));
    let producer2 = Arc::new(Producer::new(20, Arc::clone(&buffer)));

    // Create producer threads
    let p1 = {
        let producer = Arc::clone(&producer1);
        thread::spawn(move || {
            for i in 1..=5 {
                println!("Producer 1 producing item {}", i);
                producer.produce();
                println!("Producer 1 finished producing item {}", i);
            }
        })
    };

    let p2 = {
        let producer = Arc::clone(&producer2);
        thread::spawn(move || {
            for i in 1..=5 {
                println!("Producer 2 producing item {}", i);
                producer.produce();
                println!("Producer 2 finished producing item {}", i);
            }
        })
    };

    // Small delay to let producers start first
    sleep(Duration::from_millis(500));

    // Create consumer threads
    let c1 = {
        let buffer_clone = Arc::clone(&buffer); // Clone before moving
        thread::spawn(move || {
            let consumer = Consumer::new(buffer_clone);
            for i in 1..=5 {
                println!("Consumer 1 waiting for item {}", i);
                let value = consumer.consume();
                println!("Consumer 1 consumed: {}", value);
            }
        })
    };

    let c2 = {
        let buffer_clone = Arc::clone(&buffer); // Use original buffer again
        thread::spawn(move || {
            let consumer = Consumer::new(buffer_clone);
            for i in 1..=5 {
                println!("Consumer 2 waiting for item {}", i);
                let value = consumer.consume();
                println!("Consumer 2 consumed: {}", value);
            }
        })
    };

    // Wait for all threads to complete
    p1.join().unwrap();
    p2.join().unwrap();
    c1.join().unwrap();
    c2.join().unwrap();

    println!("All operations completed successfully!");
}
