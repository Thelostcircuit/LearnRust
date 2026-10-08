// This code WORKS (the tests pass) but it's badly formatted and not idiomatic Rust.
// 1. Run `cargo fmt` and see how the formatting changes.
// 2. Run `cargo clippy` and fix each warning it prints.
// Don't worry about understanding every line yet — the lessons cover all of it.
#![allow(dead_code)]

pub fn average( numbers:&[i32] )->Option<f64>
{
  if numbers.len()==0 { return None; }
      let mut total=0;
  for i in 0..numbers.len()
  {
    total = total+numbers[i];
  }
  return Some(total as f64/numbers.len() as f64);
}

pub fn is_even(n:i32)->bool{
    if n%2==0
    {
            return true;
    }
    else { return false; }
}

pub fn shout(message:&String)->String { let result=message.to_uppercase(); return result; }

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_average(){
        assert_eq!(average(&[2,4,6]),Some(4.0));
        assert_eq!( average(&[]) , None );
    }

  #[test]
  fn test_is_even() { assert!(is_even(4)); assert!(!is_even(7)); }

    #[test]
    fn test_shout()
    {
        assert_eq!(shout(&String::from("hi")),"HI");
    }
}
