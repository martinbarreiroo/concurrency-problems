// idea: struct barber-shop -> tenes waiting-room (queue ponele), barber_state (condvar), barber (Barber), y un semaforo para la waiting-room?
// logica de customer: llega un Customer -> intenta acceder a waiting-room (si no puede se muere el thread) -> la condicion para entrar seria que no este lleno el wr -> entra y se queda esperando wait()?
// logica de barber: arranca en sleep -? si la wr no esta vacia -> atiende (que haga vecdeq.pop) -> termina de atender (notify_one()?)
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct BarberShop {
    waiting_room: Mutex<VecDeque<usize>>,
    barber_state: Condvar,
    waiting_room_semaphore: usize,

}

impl BarberShop {
    fn new(n_of_chairs: usize) -> Self {
        BarberShop {
            waiting_room: Mutex::new(VecDeque::new()),
            barber_state: Condvar::new(),
            waiting_room_semaphore: n_of_chairs,
        }
    }

    fn enter_waiting_room(&self, customer: usize) {
        let mut waiting_room = self.waiting_room.lock().unwrap();

        if waiting_room.len() >= self.waiting_room_semaphore {
            println!("room is full");
            return;
        }

        waiting_room.push_back(customer);
        println!("room is now {:?}", waiting_room);
        self.barber_state.notify_one();


    }

    fn exit_waiting_room(&self) -> usize {
        let mut waiting_room = self.waiting_room.lock().unwrap();
        if waiting_room.is_empty() {
            println!("room is empty, barber goes to sleep");
            waiting_room = self.barber_state.wait(waiting_room).unwrap();
        }
        waiting_room.pop_front().unwrap()
    }
}

struct Barber {
    shop: Arc<BarberShop>,
}

impl Barber {
    fn new(barber_shop: Arc<BarberShop>) -> Self {
        Barber { shop: barber_shop }
    }

    fn work(&self) {
        loop {
            let customer = self.shop.exit_waiting_room();
            self.cut_hair(customer);
        }
    }

    fn cut_hair(&self, customer: usize) {
        println!("Barber is cutting hair of Customer {customer} ...");
        thread::sleep(Duration::from_millis(10000));
        println!("Barber finished cutting hair of Customer {customer}.");
    }
}

fn main() {
    let shop = Arc::new(BarberShop::new(3)); // 3 chairs
    let barber = Barber::new(shop.clone());

    // Start the barber thread
    let barber_handle = thread::spawn(move || {
        barber.work();
    });

    // Start customer threads
    let mut customer_handles = vec![];

    for id in 0..100 {
        let shop_clone = shop.clone();
        let handle = thread::spawn(move || {
            println!("Customer {id} arrives.");
            shop_clone.enter_waiting_room(id);
        });
        customer_handles.push(handle);

        thread::sleep(Duration::from_millis(4000)); // New customer every 400ms
    }

    // Wait for all customers to finish trying to enter
    for handle in customer_handles {
        let _ = handle.join();
    }

    // For now, barber loops forever -> optional: you could add a shutdown mechanism

    // Otherwise, we can kill the program manually (Ctrl+C) or let it run.
    let _ = barber_handle.join();
}
