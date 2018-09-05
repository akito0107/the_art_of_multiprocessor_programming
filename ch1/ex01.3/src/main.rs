use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

enum State {
    Thinking,
    Hungry,
    Eating,
}

struct Philosopher {
    id: u8,
}

impl Philosopher {
    fn new(id: u8) -> Philosopher {
        Philosopher { id: id }
    }

    fn eat(&self, mutex: &Mutex<Monitor>) {
        match mutex.try_lock() {
            Ok(ref mut mutex) => {
                if mutex.states[self.id as usize] == State::Thinking {};
                return;
            }
            _ => (),
        };
    }
}

struct Monitor {
    states: Vec<State>,
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

    let philosophers = vec![
        Philosopher::new(0),
        Philosopher::new(1),
        Philosopher::new(2),
        Philosopher::new(3),
        Philosopher::new(4),
    ];

    // let (tx, rx) = channel();
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let mutex = states.clone();
            thread::spawn(move || -> () {
                println!("id {} is spawn", p.id);
                loop {
                    p.eat(&mutex);
                }
            })
        }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
