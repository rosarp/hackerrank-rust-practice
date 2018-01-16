use std::io;

fn main() {
  let mut buff1 = String::new();
  let mut buff2 = String::new();
  let mut buff3 = String::new();
  io::stdin().read_line(&mut buff1).ok().expect("read error");
  io::stdin().read_line(&mut buff2).ok().expect("read error");
  io::stdin().read_line(&mut buff3).ok().expect("read error");

  let mut meal_cost: f32 = 0.00;
  let mut tip: f32 = 0.00;
  let mut tax: f32 = 0.00;
  match buff1.trim().parse::<f32>() {
    Ok(i) => meal_cost = i,
    Err(_) => println!("Error while parsing"),
  }

  match buff2.trim().parse::<f32>() {
    Ok(i) => tip = meal_cost * i / 100 as f32,
    Err(_) => println!("Error while parsing"),
  }

  match buff3.trim().parse::<f32>() {
    Ok(i) => tax = meal_cost * i / 100 as f32,
    Err(_) => println!("Error while parsing"),
  }

  println!("The total meal cost is {} dollars.", (meal_cost + tip + tax).round());
}
