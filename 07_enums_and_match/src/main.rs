mod exercises;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn describe(msg: &Message) -> String {
    match msg {
        Message::Quit => String::from("quit"),
        Message::Move { x, y } => format!("move to {x},{y}"),
        Message::Write(text) => format!("write: {text}"),
        Message::ChangeColor(r, g, b) => format!("color #{r:02x}{g:02x}{b:02x}"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}

fn size_word(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 | 2 => "one or two",
        3..=9 => "a few",
        n if n < 0 => "negative",
        _ => "lots",
    }
}

fn parse_age(s: &str) -> Option<u32> {
    let Ok(age) = s.parse::<u32>() else {
        return None;
    };
    Some(age)
}

fn main() {
    // --- Simple enums -------------------------------------------------------
    let d = Direction::North;
    println!("{:?} is opposite {:?}", d, d.opposite());
    println!("East == East? {}", Direction::East == Direction::East);
    let _unused = Direction::West;

    // --- Enums with data ----------------------------------------------------
    let messages = [
        Message::Quit,
        Message::Move { x: 1, y: -2 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ];
    for m in &messages {
        println!("{:?} -> {}", m, describe(m));
    }

    // --- Option -------------------------------------------------------------
    println!("plus_one(Some(5)) = {:?}", plus_one(Some(5)));
    println!("plus_one(None)    = {:?}", plus_one(None));

    let names = ["ann", "bob"];
    let third: Option<&&str> = names.get(2); // .get returns an Option instead of panicking
    println!("names.get(2) = {:?}", third);

    // --- More patterns ------------------------------------------------------
    for n in [0, 2, 5, -3, 100] {
        println!("{n} -> {}", size_word(n));
    }

    let point = (0, 7);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("on the y axis at {y}"),
        (x, 0) => println!("on the x axis at {x}"),
        _ => println!("somewhere else"),
    }

    // --- if let ---------------------------------------------------------------
    let maybe = Some(42);
    if let Some(n) = maybe {
        println!("if let got {n}");
    }

    let msg = Message::Write(String::from("only care about Write"));
    if let Message::Write(text) = &msg {
        println!("text = {text}");
    } else {
        println!("not a Write");
    }

    // --- let else -------------------------------------------------------------
    println!("parse_age(\"30\")  = {:?}", parse_age("30"));
    println!("parse_age(\"old\") = {:?}", parse_age("old"));

    // --- Option helpers ---------------------------------------------------------
    let some: Option<i32> = Some(3);
    let none: Option<i32> = None;
    println!("unwrap_or: {} {}", some.unwrap_or(0), none.unwrap_or(0));
    println!("map: {:?} {:?}", some.map(|x| x * 10), none.map(|x| x * 10));
    println!("is_some: {} {}", some.is_some(), none.is_some());
}
