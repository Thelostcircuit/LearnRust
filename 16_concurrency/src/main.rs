mod exercises;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // --- Basic spawn & join ----------------------------------------------------
    println!("== spawn & join ==");
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  thread: {i}");
            thread::sleep(Duration::from_millis(5));
        }
        "thread finished" // the return value comes back through join()
    });
    for i in 1..=3 {
        println!("  main:   {i}");
        thread::sleep(Duration::from_millis(5));
    }
    let result = handle.join().unwrap();
    println!("  joined: {result}");

    // --- move closures -----------------------------------------------------------
    println!("== move ==");
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        let sum: i32 = v.iter().sum();
        sum
    });
    // println!("{:?}", v); // <- uncomment: v was moved into the thread
    println!("  sum computed in thread = {}", handle.join().unwrap());

    // --- scoped threads (can borrow) ---------------------------------------------
    println!("== scope ==");
    let data = vec![10, 20, 30, 40];
    let (left, right) = data.split_at(2);
    let (a, b) = thread::scope(|s| {
        let h1 = s.spawn(|| left.iter().sum::<i32>());
        let h2 = s.spawn(|| right.iter().sum::<i32>());
        (h1.join().unwrap(), h2.join().unwrap())
    });
    println!("  halves summed in parallel: {a} + {b} = {}", a + b);
    println!("  data still usable here: {:?}", data);

    // --- channels -------------------------------------------------------------------
    println!("== channels ==");
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone(); // one sender per producer thread
        thread::spawn(move || {
            for j in 0..2 {
                tx.send(format!("worker {id} says {j}")).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });
    }
    drop(tx); // drop the original sender so the loop below can end
    let mut messages: Vec<String> = rx.iter().collect(); // receives until all senders are gone
    messages.sort(); // arrival order varies between runs
    for m in &messages {
        println!("  {m}");
    }

    // --- Arc<Mutex<T>> ---------------------------------------------------------------
    println!("== Arc<Mutex> ==");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!(
        "  counter = {} (always exactly 10000)",
        *counter.lock().unwrap()
    );

    // A shared Vec
    let results = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = (1..=4)
        .map(|n| {
            let results = Arc::clone(&results);
            thread::spawn(move || {
                results.lock().unwrap().push(n * n);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let mut squares = results.lock().unwrap().clone();
    squares.sort();
    println!("  squares = {:?}", squares);

    // --- RwLock ---------------------------------------------------------------------
    println!("== RwLock ==");
    let config = Arc::new(RwLock::new(String::from("v1")));
    {
        let r1 = config.read().unwrap();
        let r2 = config.read().unwrap(); // many readers at once is fine
        println!("  readers see {r1} and {r2}");
    }
    config.write().unwrap().push_str("-updated");
    println!("  after write: {}", config.read().unwrap());

    // --- Atomics ----------------------------------------------------------------------
    println!("== atomics ==");
    let hits = AtomicUsize::new(0); // no Arc needed: scoped threads can borrow it
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..500 {
                    hits.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    println!("  hits = {}", hits.load(Ordering::Relaxed));

    // --- Send / Sync: uncomment to see the compiler stop a bug ---------------------------
    // let rc = std::rc::Rc::new(5);
    // thread::spawn(move || println!("{rc}")); // error: `Rc<i32>` cannot be sent between threads safely
}
