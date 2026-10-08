// `mod exercises;` tells Rust to include the file `src/exercises.rs` as a module.
// You'll learn about modules properly in lesson 13 — for now it's just how the exercises
// get compiled alongside this file.
mod exercises;

fn main() {
    // The classic.
    println!("Hello, world!");

    // --- Formatting -------------------------------------------------------
    let name = "Ferris";
    let age = 7;

    println!("{} is {} years old", name, age);
    println!("{name} is {age} years old");
    println!("{0}, {0}, {0}!", name);

    // Debug formatting with {:?} — handy for inspecting values.
    println!("A tuple: {:?}", (1, "two", 3.0));
    println!("Pretty:  {:#?}", (1, "two", 3.0));

    // Numbers.
    let pi = 3.14159265;
    println!("pi to 2 places: {:.2}", pi);
    println!("right  [{:>6}]", 42);
    println!("left   [{:<6}]", 42);
    println!("center [{:^6}]", 42);
    println!("zeros  [{:06}]", 42);
    println!("hex {:x}, binary {:b}", 255, 5);

    // To print a literal brace, double it.
    println!("Braces look like {{ this }}");

    // --- Other printing macros ------------------------------------------
    print!("print! doesn't add a newline... ");
    println!("so this ends up on the same line.");
    eprintln!("eprintln! writes to stderr (you still see it in the terminal).");

    // format! returns a String instead of printing it.
    let message = format!("{name} says hi");
    println!("message = {message}");

    // Your exercise functions can be called from here too:
    // println!("{}", exercises::greet("World"));
}
