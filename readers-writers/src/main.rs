use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use std::sync::Condvar;

struct Reader {
    data: Arc<RwLock<usize>>,
    id: usize,
    reader_count: Arc<(Mutex<usize>, Condvar)>,
}

struct Writer {
    data: Arc<RwLock<usize>>,
    id: usize,
    reader_count: Arc<(Mutex<usize>, Condvar)>,
}

impl Reader {
    fn new(data: Arc<RwLock<usize>>, id: usize, reader_count: Arc<(Mutex<usize>, Condvar)>) -> Reader {
        Reader { data, id, reader_count }
    }

    fn read(&self) {
        let (lock, cvar) = &*self.reader_count;

        // Asegurarse de que solo haya 3 lectores concurrentes
        let mut count = lock.lock().unwrap();
        *count += 1;

        if *count == 3 {
            println!("Reader {}: Reached 3 readers, notifying writers", self.id);
            cvar.notify_all(); // Notificar a los escritores que pueden proceder
        }

        drop(count);

        println!("Reader {} waiting to acquire read lock", self.id);
        let data = self.data.read().unwrap();
        println!("Reader {} acquired read lock, reading: {}", self.id, *data);

        thread::sleep(Duration::from_millis(1000)); // Simula lectura

        println!("Reader {} releasing read lock", self.id);

        // Reducir el contador de lectores después de leer
        let mut count = lock.lock().unwrap();
        *count -= 1;
    }
}

impl Writer {
    fn new(data: Arc<RwLock<usize>>, id: usize, reader_count: Arc<(Mutex<usize>, Condvar)>) -> Writer {
        Writer { data, id, reader_count }
    }

    fn write(&self) {
        println!("Writer {} waiting to acquire write lock", self.id);

        // Esperar a que los lectores hayan terminado si hay 3 lectores
        let (lock, cvar) = &*self.reader_count;
        let mut count = lock.lock().unwrap();
        while *count == 3 {
            println!("Writer {}: Waiting for readers to finish...", self.id);
            count = cvar.wait(count).unwrap();
        }

        let mut data = self.data.write().unwrap();
        println!("Writer {} acquired write lock, updating value to {}", self.id, self.id);
        thread::sleep(Duration::from_millis(2000)); // Simula escritura

        *data = self.id;
        println!("Writer {} released write lock", self.id);
    }
}

fn main() {
    let data = Arc::new(RwLock::new(0));
    let reader_count = Arc::new((Mutex::new(0), Condvar::new())); // Contador de lectores y Condvar para sincronización
    let mut handles = vec![];

    // Crear hilos de escritores
    for i in 1..=3 {
        let data_clone = Arc::clone(&data);
        let reader_count_clone = Arc::clone(&reader_count);
        let handle = thread::spawn(move || {
            let writer = Writer::new(data_clone, i * 10, reader_count_clone);
            writer.write();
        });
        handles.push(handle);
    }

    // Crear hilos de lectores
    for i in 1..=10 {
        let data_clone = Arc::clone(&data);
        let reader_count_clone = Arc::clone(&reader_count);
        let handle = thread::spawn(move || {
            let reader = Reader::new(data_clone, i, reader_count_clone);
            reader.read();
        });
        handles.push(handle);
    }

    // Esperar que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All operations completed!");
}