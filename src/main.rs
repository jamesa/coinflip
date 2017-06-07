use std::io;

extern crate rand;
use rand::Rng;

extern crate separator;
use separator::Separatable;

fn main() {
  println!("Coin flip!");
  println!("How many times should we flip the coin? ");

  let mut num = String::new();
  io::stdin().read_line(&mut num)
    .expect("Didn't catch that...");

  let num: f32 = num.trim().parse::<f32>()
    .expect("Please put in a number!");

  let results = flip_coin(num);

  let pct_heads = (results[0] / num) * 100.0;
  let pct_tails = (results[1] / num) * 100.0;

  println!("It landed on heads {} times. ({:.1}%)", results[0].separated_string(), pct_heads);
  println!("It landed on tails {} times. ({:.1}%)", results[1].separated_string(), pct_tails);
}

fn flip_coin(n: f32) -> [f32; 2] {
  let mut num_heads = 0.0;
  let mut num_tails = 0.0;

  while num_heads + num_tails < n {
    let flip = rand::thread_rng().gen_range(0, 2);

    match flip {
      0 => num_heads+=1.0,
      1 => num_tails+=1.0,
      _ => println!("Coin landed on its edge :O")
    }
  }

  return [num_heads, num_tails];
}

#[cfg(test)]
mod tests {
  use super::flip_coin;

  #[test]
  fn test_no_flips() {
    assert_eq!([0.0, 0.0], flip_coin(0.0));
  }

  #[test]
  fn test_100_flips() {
    let results = flip_coin(100.0);
    assert_eq!(100.0, results[0] + results[1]);
  }
}
