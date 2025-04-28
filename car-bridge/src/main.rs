// logica: se tiene un puente que tiene solo una direccion en la que los autos que lleguen pueden ir al mismo tiempo. Los autos llegan e intentan cruzar el puente
// si el auto q llega va de a->b, pero la direccion del puente en ese momento es b->a, el auto espera
// cuando se llega a 5 autos yendo en la misma direccion, se cambia la direccion en la que el puente acepta los autos

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    AtoB,
    BtoA,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::AtoB => Direction::BtoA,
            Direction::BtoA => Direction::AtoB,
        }
    }
}

struct BridgeState {
    dir: Direction,
    queue: VecDeque<usize>,
    cars_crossed: usize,
    capacity_to_change: usize,
}

struct Bridge {
    state: Mutex<BridgeState>,
    direction_changed: Condvar,
}

struct Car {
    bridge: Arc<Bridge>,
    id: usize,
    direction: Direction,
}

impl Bridge {
    fn new(initial_dir: Direction) -> Self {
        Bridge {
            state: Mutex::new(BridgeState {
                dir: initial_dir,
                queue: VecDeque::new(),
                cars_crossed: 0,
                capacity_to_change: 5,
            }),
            direction_changed: Condvar::new(),
        }
    }

    fn acquire_bridge(&self, car_dir: Direction, car_id: usize) {
        let mut state = self.state.lock().unwrap();

        // Wait if car's direction doesn't match bridge direction
        while state.dir != car_dir {
            println!("Car {} waiting - bridge direction is {:?}", car_id, state.dir);
            state = self.direction_changed.wait(state).unwrap();
        }

        // Add car to queue and increment counter
        state.queue.push_back(car_id);
        state.cars_crossed += 1;
        println!("Car {} crossing bridge (dir: {:?})", car_id, car_dir);

        // Check if we need to change direction after this car
        if state.cars_crossed >= state.capacity_to_change {
            state.dir = state.dir.opposite();
            state.cars_crossed = 0;
            println!("Bridge direction changed to {:?}", state.dir);
            // Notify waiting cars that direction has changed
            self.direction_changed.notify_all();
        }
    }

    fn release_bridge(&self, car_id: usize) {
        let mut state = self.state.lock().unwrap();
        // Remove car from queue
        if let Some(pos) = state.queue.iter().position(|&id| id == car_id) {
            state.queue.remove(pos);
            println!("Car {} exited the bridge", car_id);
            self.direction_changed.notify_all();
        }
    }
}

impl Car {
    fn new(bridge: Arc<Bridge>, id: usize, direction: Direction) -> Self {
        Car { bridge, id, direction }
    }

    fn cross_bridge(self) {
        self.bridge.acquire_bridge(self.direction.clone(), self.id);
        sleep(Duration::from_millis(10000)); // Time to cross bridge
        self.bridge.release_bridge(self.id);
    }
}

fn main() {
    let bridge = Arc::new(Bridge::new(Direction::AtoB));
    let mut handles = vec![];

    // Create cars going A->B
    for i in 1..=8 {
        let car_bridge = Arc::clone(&bridge);
        let car = Car::new(car_bridge, i, Direction::AtoB);
        handles.push(thread::spawn(move || {
            car.cross_bridge();
        }));
        sleep(Duration::from_millis(4000));
    }

    // Create cars going B->A
    for i in 101..=108 {
        let car_bridge = Arc::clone(&bridge);
        let car = Car::new(car_bridge, i, Direction::BtoA);
        handles.push(thread::spawn(move || {
            car.cross_bridge();
        }));
        sleep(Duration::from_millis(7000));
    }

    // Wait for all cars to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

