use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

// a chequear ni idea si esta bien

struct Producer {
    sender: mpsc::Sender<usize>,
    data: usize,
    id: usize
}

struct Consumer {
    receiver: Arc<Mutex<mpsc::Receiver<usize>>>,
    id: usize
}

impl Producer {
    fn new(id: usize, sender: mpsc::Sender<usize>, data: usize) -> Self {
        Producer { id, sender, data }
    }

    fn produce(&self) {
        println!("Producer {} producing {}", self.id, self.data);
        sleep(Duration::from_millis(3000));
        self.sender.send(self.data).unwrap();
        println!("Producer {} produced {}", self.id, self.data);
    }
}

impl Consumer {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<usize>>>) -> Self {
        Consumer { id, receiver }
    }

    fn consume(&self) {
        loop {
            let message = self.receiver.lock().unwrap().recv();
            match message {
                Ok(data) => {
                    sleep(Duration::from_millis(4000));
                    println!("Consumer {} consumed {}", self.id, data);
                }
                Err(_) => {
                    println!("No more data, consumer {} exiting.", self.id);
                    break;
                }
            }
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    // Crear productores
    for i in 0..3 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let producer = Producer::new(i, tx, i);
            producer.produce();
        });
        handles.push(handle);
    }

    // Crear consumidores
    for j in 0..3 {
        let rx = rx.clone();
        let handle = thread::spawn(move || {
            let consumer = Consumer::new(j, rx);
            consumer.consume();
        });
        handles.push(handle);
    }

    // Dropear el último sender (importante para que se cierre el canal cuando terminen todos los producers)
    drop(tx);

    // Esperar a que todos los threads terminen
    for handle in handles {
        handle.join().unwrap();
    }
}