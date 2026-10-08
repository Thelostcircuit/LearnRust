mod exercises;

// A type that announces when it is dropped, so you can SEE ownership in action.
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  (dropping {})", self.0);
    }
}

fn main() {
    // --- Scope and drop -----------------------------------------------------
    println!("== scope ==");
    {
        let _a = Noisy("a");
        println!("  inside the block");
    } // _a dropped here
    println!("  after the block");

    // --- Moves --------------------------------------------------------------
    println!("== moves ==");
    let s1 = String::from("hello");
    let s2 = s1; // moved
    // println!("{s1}"); // <- uncomment: error[E0382]: borrow of moved value
    println!("  s2 = {s2}");

    // --- Moving into a function -------------------------------------------
    println!("== moving into a function ==");
    let n = Noisy("n");
    take_ownership(n);
    println!("  back in main; `n` is gone");
    // take_ownership(n); // <- uncomment: use of moved value

    // --- Getting ownership back from a function ----------------------------
    println!("== returning ownership ==");
    let s = gives_ownership();
    let s = takes_and_gives_back(s);
    println!("  still have: {s}");

    // --- Clone --------------------------------------------------------------
    println!("== clone ==");
    let original = String::from("data");
    let copy = original.clone();
    println!("  original = {original}, copy = {copy}");

    // --- Copy types ---------------------------------------------------------
    println!("== copy types ==");
    let x = 5;
    let y = x; // copied, not moved
    println!("  x = {x}, y = {y}");
    let point = (1, 2);
    let point2 = point; // tuples of Copy types are Copy
    println!("  point = {:?}, point2 = {:?}", point, point2);

    // --- Early drop ---------------------------------------------------------
    println!("== early drop ==");
    let early = Noisy("early");
    let _late = Noisy("late");
    drop(early);
    println!("  dropped `early` manually; `late` drops at end of main");

    // --- Ownership of Vec elements -----------------------------------------
    println!("== vec ==");
    let names = vec![String::from("ann"), String::from("bob")];
    for name in names {
        // `for x in vec` moves each element out of the vec
        println!("  owned name: {name}");
    }
    // println!("{:?}", names); // <- uncomment: `names` was moved by the for loop

    println!("== end of main ==");
}

fn take_ownership(thing: Noisy) {
    println!("  take_ownership got {}", thing.0);
} // `thing` dropped here

fn gives_ownership() -> String {
    String::from("a gift")
}

fn takes_and_gives_back(s: String) -> String {
    s // move it straight back out
}
