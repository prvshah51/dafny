pub mod additional_module;

fn main() {
  let x = additional_module::_module::_default::reverse(10);
  println!("Intermediate result is {:?}", x);
  let y = additional_module::_module::_default::reverse(x);
  println!("Result should be 10, got {:?}", y);
}