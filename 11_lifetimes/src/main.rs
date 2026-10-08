mod exercises;

// Two inputs, one output that could come from either → need 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Output only ever comes from x, so only x needs to be tied to it.
fn first_of<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// One input → elision rule 2 handles it, no annotation needed.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// A struct holding a reference.
#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    // elision rule 3: output tied to &self
    fn part(&self) -> &str {
        self.part
    }

    fn word_count(&self) -> usize {
        self.part.split_whitespace().count()
    }
}

// A struct holding several borrowed pieces of one text.
struct KeyValue<'a> {
    key: &'a str,
    value: &'a str,
}

fn parse_kv(line: &str) -> Option<KeyValue<'_>> {
    // '_ means "the lifetime elision would pick" — here, line's lifetime
    let (key, value) = line.split_once('=')?;
    Some(KeyValue {
        key: key.trim(),
        value: value.trim(),
    })
}

fn main() {
    let a = String::from("a long string");
    let b = "short";
    println!("longest = {}", longest(&a, b));

    // Works: both live long enough.
    let result;
    {
        let c = String::from("xyz");
        result = first_of(&a, &c); // result only borrows from `a`, so c can die
    }
    println!("first_of = {result}");

    // Uncomment to see the lifetime error:
    // let result2;
    // {
    //     let c = String::from("xyz");
    //     result2 = longest(&a, &c);
    // }
    // println!("{result2}");

    println!("first_word = {}", first_word("hello there"));

    let novel = String::from("Call me Ishmael. Some years ago, never mind how long...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!(
        "{:?} -> '{}' has {} words",
        excerpt,
        excerpt.part(),
        excerpt.word_count()
    );

    let config = "name = ferris\ncolor = orange\ninvalid line";
    for line in config.lines() {
        match parse_kv(line) {
            Some(kv) => println!("key '{}' -> value '{}'", kv.key, kv.value),
            None => println!("skipping '{line}'"),
        }
    }

    let forever: &'static str = "I am baked into the binary";
    println!("{forever}");
}
