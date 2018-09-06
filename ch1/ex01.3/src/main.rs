use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
enum State {
    Thinking,
    Hungry,
    Eating,
}

struct Philosopher {
    id: u8,
    left: Sender<i32>,
    right: Sender<i32>,
    rx: Arc<Mutex<Receiver<i32>>>,
}

impl Philosopher {
    fn new(
        id: u8,
        left: Sender<i32>,
        right: Sender<i32>,
        rx: Arc<Mutex<Receiver<i32>>>,
    ) -> Philosopher {
        Philosopher {
            id: id,
            left: left,
            right: right,
            rx: rx,
        }
    }

    fn pickup(&self, mutex: &Mutex<Monitor>) -> Result<(), ()> {
        match mutex.try_lock() {
            Ok(ref mut monitor) => {
                let right = ((self.id + 1) % 5) as usize;
                let left = ((self.id + 4) % 5) as usize;
                if monitor.states[self.id as usize] == State::Hungry
                    && monitor.states[right] == State::Eating
                    && monitor.states[left] == State::Eating
                {
                    monitor.states[self.id as usize] = State::Eating;
                };
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn eat(&self) {
        println!("{} is eating", self.id);
        thread::sleep(Duration::from_millis(2000));
    }

    fn wait(&self) {
        let r = self.rx.lock().unwrap();
        r.recv();
    }

    fn putdown(&self, mutex: &Mutex<Monitor>) -> Result<(), ()> {
        let mut lock = mutex.lock().unwrap();
        lock.states[self.id as usize] = State::Thinking;
        self.left.send(0).unwrap();
        self.right.send(0).unwrap();
        Ok(())
    }
}

struct Monitor {
    states: Vec<State>,
}

fn makeChan() -> (Sender<i32>, Arc<Mutex<Receiver<i32>>>) {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();

    (tx, Arc::new(Mutex::new(rx)))
}

fn main() {
    let states = Arc::new(Mutex::new(Monitor {
        states: vec![
            State::Thinking,
            State::Thinking,
            State::Thinking,
            State::Thinking,
            State::Thinking,
        ],
    }));

    let channels = vec![makeChan(), makeChan(), makeChan(), makeChan(), makeChan()];

    let philosophers = vec![
        Philosopher::new(
            0,
            channels[4].0.clone(),
            channels[1].0.clone(),
            channels[0].1.clone(),
        ),
        Philosopher::new(
            1,
            channels[0].0.clone(),
            channels[2].0.clone(),
            channels[1].1.clone(),
        ),
        Philosopher::new(
            2,
            channels[1].0.clone(),
            channels[3].0.clone(),
            channels[2].1.clone(),
        ),
        Philosopher::new(
            3,
            channels[2].0.clone(),
            channels[4].0.clone(),
            channels[3].1.clone(),
        ),
        Philosopher::new(
            4,
            channels[3].0.clone(),
            channels[0].0.clone(),
            channels[4].1.clone(),
        ),
    ];

    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let r_mut = Arc::new(Mutex::new(rx));

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let mutex = states.clone();
            thread::spawn(move || -> () {
                println!("id {} is spawn", p.id);
                loop {
                    match p.pickup(&mutex) {
                        Ok(_) => {
                            p.eat();
                            p.putdown(&mutex);
                        }
                        _ => {
                            p.wait();
                        }
                    }
                }
            })
        }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
