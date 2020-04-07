#[allow(dead_code)]
extern crate lib;
use lib::*;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::str::FromStr;

fn main(){
    // 一个类型 如果实现了Form 那么会自动实现into
  let num = preview::Test::from(10);
  let num2:preview::Test = 89.into();

  //try_from

  let even = preview::Test2::try_from(20).unwrap();
  println!("{}",even);
  let even2:preview::Test2 = 80.try_into().unwrap();
  println!("{}",even2);

  //ToString he fromSTr
  let test = preview::Test3::from_str("{100}").unwrap();
  println!("{:?}",&test);
  println!("{}", test.to_string());
}