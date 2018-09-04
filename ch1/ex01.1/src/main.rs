use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    id: u8,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(id: u8, left: usize, right: usize) -> Philosopher {
        Philosopher {
            id: id,
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating,", self.id);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating,", self.id);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosophers = vec![
        Philosopher::new(0, 0, 1),
        Philosopher::new(1, 1, 2),
        Philosopher::new(2, 2, 3),
        Philosopher::new(3, 3, 4),
        Philosopher::new(4, 0, 4), // ex1.2
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();
            thread::spawn(move || loop {
                p.eat(&table);
            })
        }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
