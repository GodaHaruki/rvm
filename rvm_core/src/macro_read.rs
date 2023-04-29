use serde::{Deserialize, Serialize};


#[macro_export]
macro_rules! read_as {
  ($t:ty, $s:expr) => { 
    serde_json::from_str::<$t>($s)
  }
}


#[test]
use serde_json;

#[test]
#[derive(PartialEq, Debug, Serialize, Deserialize)]
enum ForT {
  U(usize),
  I(isize),
  S(String),
  N,
}

#[test]
#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct ForTest {
  v: Vec<usize>,
  e: Vec<ForT>,
}

#[test]
fn macro_read_as(){
  let t = ForTest{
    v: vec![1,2,3,4,5],
    e: vec![ForT::U(100), ForT::S("AAA".to_string())],
  };

  let se = serde_json::to_string(&t).unwrap();


  assert_eq!(t, read_as!(ForTest, &se).unwrap());
}