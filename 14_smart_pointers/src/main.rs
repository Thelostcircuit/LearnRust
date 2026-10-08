mod exercises;

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

// A recursive type needs a Box.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn sum(&self) -> i32 {
        match self {
            Cons(value, rest) => value + rest.sum(),
            Nil => 0,
        }
    }
}

// A tree where children know their parent (weakly) and parents own their children.
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// A struct using Cell to count calls through &self.
struct Api {
    calls: Cell<u32>,
}

impl Api {
    fn request(&self) -> &str {
        self.calls.set(self.calls.get() + 1); // mutating through &self!
        "ok"
    }
}

fn main() {
    // --- Box ------------------------------------------------------------------
    println!("== Box ==");
    let b = Box::new(5);
    println!("b = {b}, *b + 1 = {}", *b + 1);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);
    println!("sum = {}", list.sum());

    // --- Rc -------------------------------------------------------------------
    println!("== Rc ==");
    let a = Rc::new(String::from("shared data"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after cloning into b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count with c in scope = {} (c = {c})", Rc::strong_count(&a));
    }
    println!("count after c dropped = {}", Rc::strong_count(&a));
    println!("a and b point to the same data: {}", Rc::ptr_eq(&a, &b));

    // --- RefCell ---------------------------------------------------------------
    println!("== RefCell ==");
    let cell = RefCell::new(vec![1, 2]);
    cell.borrow_mut().push(3);
    println!("cell = {:?}", cell.borrow());

    // Uncomment to see a runtime panic (two mutable borrows at once):
    // let first = cell.borrow_mut();
    // let second = cell.borrow_mut();

    // try_borrow_mut returns a Result instead of panicking:
    let reader = cell.borrow();
    println!(
        "try_borrow_mut while reading: is_err = {}",
        cell.try_borrow_mut().is_err()
    );
    drop(reader);

    // --- Cell -------------------------------------------------------------------
    let api = Api {
        calls: Cell::new(0),
    };
    api.request();
    api.request();
    println!("api was called {} times", api.calls.get());

    // --- Rc<RefCell<T>> ---------------------------------------------------------
    println!("== Rc<RefCell<T>> ==");
    let shared = Rc::new(RefCell::new(0));
    let x = Rc::clone(&shared);
    let y = Rc::clone(&shared);
    *x.borrow_mut() += 1;
    *y.borrow_mut() += 10;
    println!("shared = {}", shared.borrow());

    // A shared, mutable log that several "components" write to
    let log = Rc::new(RefCell::new(Vec::<String>::new()));
    let components = vec![("ui", Rc::clone(&log)), ("db", Rc::clone(&log))];
    for (name, log) in &components {
        log.borrow_mut().push(format!("{name} started"));
    }
    println!("log = {:?}", log.borrow());

    // --- Weak: a tree with parent pointers --------------------------------------
    println!("== Weak ==");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf parent before = {:?}",
        leaf.parent.borrow().upgrade().map(|p| p.value)
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "leaf parent after  = {:?}",
        leaf.parent.borrow().upgrade().map(|p| p.value)
    );
    println!("branch has {} child(ren)", branch.children.borrow().len());
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
}
