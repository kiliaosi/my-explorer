extern crate lib;
use lib::from;
use std::convert::TryFrom;
use std::convert::TryInto;
fn main() {
  let stru = from::Number::from(10);
  println!("{}", stru);
  
  // 使用自动获得的into
  let somes:from::Number = 200.into();
  println!("{}", somes);
  // try_into
  let try_some = from::EvenNumber::try_from(20).unwrap();
  println!("{:?}",try_some);
  let try_some:from::EvenNumber = 50.try_into().unwrap();
  println!("{:?}", try_some);
}
