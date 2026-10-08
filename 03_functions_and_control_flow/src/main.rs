mod exercises;

fn main() {
    // --- Functions --------------------------------------------------------
    println!("add(2, 3) = {}", add(2, 3));
    print_labeled("answer", 42);
    println!("absolute(-7) = {}", absolute(-7));

    // --- Blocks are expressions ------------------------------------------
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {y}");

    // --- if / else if / else ----------------------------------------------
    for n in [-5, 0, 5] {
        if n < 0 {
            println!("{n} is negative");
        } else if n == 0 {
            println!("{n} is zero");
        } else {
            println!("{n} is positive");
        }
    }

    // if as an expression
    let n = 7;
    let kind = if n % 2 == 0 { "even" } else { "odd" };
    println!("{n} is {kind}");

    // --- loop with break value --------------------------------------------
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result = {result}");

    // --- while ------------------------------------------------------------
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}...");
        countdown -= 1;
    }
    println!("Liftoff! 🚀");

    // --- for --------------------------------------------------------------
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits {
        println!("I like {fruit}");
    }

    for i in 0..3 {
        print!("{i} ");
    }
    println!("(0..3)");

    for i in 1..=3 {
        print!("{i} ");
    }
    println!("(1..=3)");

    for i in (1..=3).rev() {
        print!("{i} ");
    }
    println!("(reversed)");

    for (index, fruit) in fruits.iter().enumerate() {
        println!("#{index}: {fruit}");
    }

    // continue: skip odd numbers
    for i in 0..10 {
        if i % 2 == 1 {
            continue;
        }
        print!("{i} ");
    }
    println!("(evens)");

    // Labeled break out of nested loops
    'outer: for x in 1..10 {
        for y in 1..10 {
            if x * y == 12 {
                println!("first pair with product 12: {x} x {y}");
                break 'outer;
            }
        }
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon: this is the return value
}

// No `-> Type` means it returns `()`.
fn print_labeled(label: &str, value: i32) {
    println!("{label}: {value}");
}

fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n; // early return uses the `return` keyword
    }
    n
}
