mod exercises;

// Constants can live outside functions.
const SECONDS_PER_HOUR: u32 = 60 * 60;

fn main() {
    // --- Mutability -----------------------------------------------------
    let x = 5;
    println!("x = {x}");
    // x = 6; // <- uncomment this line and run `cargo check` to see the error

    let mut y = 5;
    y += 1;
    println!("y = {y}");

    // --- Shadowing ------------------------------------------------------
    let spaces = "   ";
    let spaces = spaces.len(); // new variable, new type
    println!("spaces = {spaces}");

    let z = 1;
    {
        let z = z * 10; // shadows only inside this block
        println!("inner z = {z}");
    }
    println!("outer z = {z}");

    // --- Integers -------------------------------------------------------
    let big = 1_000_000;
    let byte: u8 = 255;
    println!("big = {big}, byte = {byte}");
    println!("i32 range: {} to {}", i32::MIN, i32::MAX);
    println!("u8 range: {} to {}", u8::MIN, u8::MAX);

    // Handling overflow explicitly:
    println!("255u8.wrapping_add(1)   = {}", byte.wrapping_add(1));
    println!("255u8.checked_add(1)    = {:?}", byte.checked_add(1));
    println!("255u8.saturating_add(1) = {}", byte.saturating_add(1));

    // Integer division truncates; % is remainder.
    println!("7 / 2 = {}, 7 % 2 = {}", 7 / 2, 7 % 2);

    // --- Floats ---------------------------------------------------------
    let f = 7.0 / 2.0;
    println!("7.0 / 2.0 = {f}");
    println!("sqrt(2) = {}", 2.0_f64.sqrt());

    // --- Casting --------------------------------------------------------
    let whole = 3.99_f64 as i32; // truncates toward zero
    let as_float = 10 as f64 / 4 as f64;
    println!("3.99 as i32 = {whole}, 10/4 as floats = {as_float}");

    // --- bool & char ----------------------------------------------------
    let is_fun: bool = true;
    let crab: char = '🦀';
    println!("is_fun = {is_fun}, crab = {crab}");
    println!("'a' is alphabetic? {}", 'a'.is_alphabetic());
    println!("'7' as a digit: {:?}", '7'.to_digit(10));

    // --- Tuples ---------------------------------------------------------
    let tup = (500, 6.4, 'z');
    let (a, b, c) = tup;
    println!("destructured: {a} {b} {c}");
    println!("indexed: {} {} {}", tup.0, tup.1, tup.2);

    // --- Arrays ---------------------------------------------------------
    let nums = [10, 20, 30, 40, 50];
    let zeros = [0u8; 4];
    println!("nums = {:?}, len = {}", nums, nums.len());
    println!("zeros = {:?}", zeros);
    println!("first = {}, last = {}", nums[0], nums[nums.len() - 1]);

    // --- Constants ------------------------------------------------------
    println!("There are {SECONDS_PER_HOUR} seconds in an hour");

    // --- Parsing strings into numbers -----------------------------------
    let parsed: i32 = "42".parse().unwrap();
    println!("parsed = {}", parsed + 1);
}
