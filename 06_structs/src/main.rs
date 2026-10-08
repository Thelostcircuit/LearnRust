mod exercises;

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // field init shorthand
        email,
        active: true,
        sign_in_count: 1,
    }
}

// Tuple structs
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(i32, i32);

// Newtype pattern
#[derive(Debug)]
struct Meters(f64);

#[derive(Debug, Clone, PartialEq)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self::new(size, size)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    fn into_square(self) -> Rect {
        let side = self.width.max(self.height);
        Rect::square(side)
    }
}

fn main() {
    // --- Creating and using a struct --------------------------------------
    let mut user = build_user(String::from("ferris"), String::from("ferris@example.com"));
    user.sign_in_count += 1;
    println!(
        "{} ({}) has signed in {} times",
        user.username, user.email, user.sign_in_count
    );
    println!("active? {}", user.active);
    println!("{:?}", user);
    println!("{:#?}", user);

    // Struct update syntax
    let user2 = User {
        email: String::from("crab@example.com"),
        ..user // moves `username` out of `user`
    };
    println!("user2 = {:?}", user2);
    // println!("{}", user.username); // <- uncomment: username was moved into user2
    println!("but user.email is still usable: {}", user.email);

    // --- Tuple structs ------------------------------------------------------
    let p = Point(3, 4);
    let q = p; // Point is Copy (we derived it), so p is still usable
    println!("p = {:?}, q.0 = {}, p == q? {}", p, q.0, p == q);

    let distance = Meters(5.5);
    println!("distance = {:?} ({} m)", distance, distance.0);

    // --- Methods --------------------------------------------------------------
    let mut rect = Rect::new(3, 4);
    println!("rect = {:?}, area = {}", rect, rect.area());

    let small = Rect::new(1, 1);
    println!("rect can hold small? {}", rect.can_hold(&small));

    rect.scale(2);
    println!("scaled = {:?}", rect);

    let sq = rect.into_square(); // rect is consumed here
    println!("square = {:?}", sq);
    // println!("{:?}", rect); // <- uncomment: rect was moved by into_square(self)

    // dbg! prints file, line, expression, and value (to stderr), then returns the value
    let area = dbg!(sq.area());
    println!("area = {area}");
}
