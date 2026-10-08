mod exercises;

use std::fmt;
use std::ops::Add;

// --- A generic struct ---------------------------------------------------------
#[derive(Debug)]
struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

// Methods that only exist when T supports comparison + printing.
impl<T: PartialOrd + fmt::Display> Pair<T> {
    fn print_larger(&self) {
        if self.a >= self.b {
            println!("larger is a = {}", self.a);
        } else {
            println!("larger is b = {}", self.b);
        }
    }
}

// --- A trait ------------------------------------------------------------------
trait Describe {
    fn describe(&self) -> String;

    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}

struct Dog {
    name: String,
}

struct Cat {
    lives: u8,
}

impl Describe for Dog {
    fn describe(&self) -> String {
        format!("{} the dog", self.name)
    }
}

impl Describe for Cat {
    fn describe(&self) -> String {
        format!("a cat with {} lives", self.lives)
    }
    fn shout(&self) -> String {
        String::from("HISSSS") // overriding the default
    }
}

// Implementing our trait for a built-in type
impl Describe for i32 {
    fn describe(&self) -> String {
        format!("the number {self}")
    }
}

// --- Display ------------------------------------------------------------------
impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "🐶 {}", self.name)
    }
}

// --- Operator overloading ------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// --- From / Into ---------------------------------------------------------------
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

// --- Generic functions with bounds -----------------------------------------------
fn largest<T: PartialOrd>(items: &[T]) -> &T {
    let mut max = &items[0];
    for item in items {
        if item > max {
            max = item;
        }
    }
    max
}

fn announce(item: &impl Describe) {
    println!("Announcing: {}", item.describe());
}

fn print_all<T>(items: &[T])
where
    T: Describe,
{
    for item in items {
        println!("  - {}", item.describe());
    }
}

fn make_number() -> impl Describe {
    7
}

fn main() {
    let p = Pair::new(3, 8);
    println!("{:?}", p);
    p.print_larger();
    Pair::new("apple", "banana").print_larger();

    println!("largest int = {}", largest(&[4, 9, 2]));
    println!("largest char = {}", largest(&['x', 'b', 'q']));
    println!("largest str = {}", largest(&["hi", "zebra", "apple"]));

    let rex = Dog {
        name: String::from("Rex"),
    };
    let tom = Cat { lives: 9 };
    announce(&rex);
    announce(&tom);
    announce(&42);
    println!("rex shouts: {}", rex.shout());
    println!("tom shouts: {}", tom.shout());
    println!("Display: {rex}");
    println!("to_string: {}", rex.to_string());

    print_all(&[1, 2, 3]);
    println!("make_number(): {}", make_number().describe());

    // Trait objects: a Vec holding different types
    let zoo: Vec<Box<dyn Describe>> = vec![
        Box::new(Dog {
            name: String::from("Fido"),
        }),
        Box::new(Cat { lives: 3 }),
        Box::new(100),
    ];
    for animal in &zoo {
        println!("zoo: {}", animal.describe());
    }

    let v = Vec2 { x: 1.0, y: 2.0 } + Vec2 { x: 3.0, y: 4.0 };
    println!("vector sum = {:?}", v);

    let boiling: Fahrenheit = Celsius(100.0).into(); // Into comes free from From
    println!("100C = {}F", boiling.0);
}
