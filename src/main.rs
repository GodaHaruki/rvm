use rvm_core::session::Session;
use rvm_core::save::Save;

fn main() -> Result<(), Box<dyn std::error::Error>>{
  let s = Session::init();
  println!("{:?}", &s);
  s.save(&(std::env::current_dir()?));
  Ok(())
}
