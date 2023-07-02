pub fn print_a_Z() {
  println!("print_a_Z: ");

  for c in (b'Z'..=b'a') { 
    print!("{}", c as char);
  }
}