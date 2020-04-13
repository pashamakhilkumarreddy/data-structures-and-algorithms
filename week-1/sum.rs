use std::io::{stdin};

fn main() {
  let mut buff = String::new();
  println!("Please enter the values to add separated by a white space");
  stdin().read_line(&mut buff).expect("Unable to read the input");
  let mut words = buff.split_whitespace();
  let a: i64 = words.next().unwrap().parse().unwrap();
  let b: i64 = words.next().unwrap().parse().unwrap();
  println!("The sum of {} and {} is {}", a, b, sum_of_two_digits(a, b))
}

fn sum_of_two_digits(a: i64, b: i64) -> i64 {
  a + b
}

