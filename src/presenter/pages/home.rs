use anyhow::Result;
use std::io;

pub fn home_page() ->  {
  let mut username = String::new();
  let stdin = io::stdin();

  loop {
    println!("Seu Nome: (minimo 2 caracters)");
    stdin.read_line(&mut username)?;

    if username.len() >= 3 {
      println!("{}", username.len());
      break;
    }
  }

  Ok(username)
}
