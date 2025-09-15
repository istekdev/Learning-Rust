use std::rand::Rng;

const diff: u32 = 10000 * 120;
const target: u128 = (u128::MAX) / diff;

fn main() {
  let mut mined: bool = false;
  let mut rng = rand::thread_rng();
  loop {
    let random: u128 = rng.gen::<u128> & ((1u128 << 80) - 1);
    if random <= target {
      println!("[!] - MINED!");
      mined = true;
      break;
    } else {
      println!("[!] - GUESSING: {}", random);
    }
  }
}
  
