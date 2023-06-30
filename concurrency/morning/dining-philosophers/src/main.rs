use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        let _left_fork = self.left_fork.lock().unwrap();
        let _right_fork = self.right_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

macro_rules! range {
    ($calc:expr, $max:expr) => {
        (($calc) + ($max)) % ($max)
    };
}

fn main() {
    let (sender, receiver) = mpsc::sync_channel(20);

    // Create forks
    let mut fork_array: Vec<Arc<Mutex<Fork>>> = Vec::new();
    for _i in 0..PHILOSOPHERS.len() {
        fork_array.push(Arc::new(Mutex::new(Fork)));
    }

    // Create philosophers
    for i in 0..fork_array.len() {
        let left_fork: Arc<Mutex<Fork>>;
        let right_fork: Arc<Mutex<Fork>>;

        if i == fork_array.len() - 1 {
            left_fork = fork_array[range!(i + 1, fork_array.len())].clone();
            right_fork = fork_array[i].clone();
        } else {
            left_fork = fork_array[i].clone();
            right_fork = fork_array[range!(i + 1, fork_array.len())].clone();
        }

        let philosopher: Philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            left_fork,
            right_fork,
            thoughts: sender.clone(),
        };

        // Make them think and eat
        thread::spawn(move || {
            for _i in 0..50 {
                philosopher.eat();
                philosopher.think();
            }
        });
    }

    // Output their thoughts
    drop(sender);
    for response in receiver {
        println!("{response}");
    }
}
