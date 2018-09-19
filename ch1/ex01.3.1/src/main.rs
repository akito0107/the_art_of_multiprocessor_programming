use std::cell::Cell;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
enum State {
    Thinking,
    Eating,
}

struct Philosopher {
    id: u8,
    w_count: Cell<u8>,
}

impl Philosopher {
    fn new(id: u8) -> Philosopher {
        Philosopher {
            id: id,
            w_count: Cell::new(0),
        }
    }

    fn pickup(&self, mutex: &Mutex<Monitor>) -> Result<(), ()> {
        match mutex.try_lock() {
            Ok(ref mut monitor) => {
                let right = ((self.id + 1) % 5) as usize;
                let left = ((self.id + 4) % 5) as usize;
                if monitor.states[self.id as usize] == State::Thinking
                    && monitor.states[right] != State::Eating
                    && monitor.states[left] != State::Eating
                {
                    monitor.states[self.id as usize] = State::Eating;
                    return Ok(());
                };
                return Err(());
            }
            _ => Err(()),
        }
    }

    fn eat(&self) {
        println!("{} is eating", self.id);
        thread::sleep(Duration::from_millis(2000));
    }

    fn wait(&self, mutex: &Mutex<Monitor>, cvar: &Condvar) {
        let lock = mutex.lock().unwrap();
        cvar.wait(lock).unwrap();
    }

    fn putdown(&self, mutex: &Mutex<Monitor>) -> Result<(), ()> {
        let mut lock = mutex.lock().unwrap();
        lock.states[self.id as usize] = State::Thinking;
        println!("{} is done eating", self.id);
        Ok(())
    }
}

struct Monitor {
    states: Vec<State>,
}

fn main() {
    let states = Arc::new((
        Mutex::new(Monitor {
            states: vec![
                State::Thinking,
                State::Thinking,
                State::Thinking,
                State::Thinking,
                State::Thinking,
            ],
        }),
        Condvar::new(),
    ));

    let philosophers = vec![
        Philosopher::new(0),
        Philosopher::new(1),
        Philosopher::new(2),
        Philosopher::new(3),
        Philosopher::new(4),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let pair = states.clone();
            thread::spawn(move || -> () {
                let &(ref lock, ref cvar) = &*pair;
                loop {
                    match p.pickup(lock) {
                        Ok(_) => {
                            p.eat();
                            cvar.notify_all();
                            p.putdown(lock).unwrap();
                            cvar.notify_all();
                        }
                        _ => {
                            println!("id {} is waiting", p.id);
                            p.wait(lock, cvar);
                        }
                    }
                }
            })
        }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
