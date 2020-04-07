// f复习：
#[allow(dead_code)]
use std::convert::{From, TryFrom};
use std::string::ToString;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

pub struct Test{
  pub value:i32,
}

pub struct Test2{
  pub value:i32,
}

//联系 toString
#[derive(Debug)]
pub struct Test3{
 pub value:i32,
}

impl From<i32> for Test{
  fn from(value:i32)->Self{
    Test{
      value,
    }
  }
}

impl TryFrom<i32> for Test2{
  type Error = ();
  fn try_from(value:i32)->Result<Self, Self::Error>{
    if value%2==0{
      return Ok(Test2{ value });
    }
    Err(())
  }
}

impl ToString for Test3{
  fn to_string(&self)->String{
    format!(" Test3{{{}}} ",self.value)
  }
}

impl FromStr for Test3{
  type Err = ParseIntError;
  fn from_str(s: &str)->Result<Self, Self::Err>{
    let vecs:Vec<&str> = s.trim_matches(|s|s=='{'||s=='}')
                .split(',')
                .collect();
    let value = vecs[0].parse::<i32>()?;
    return Ok(Test3{ value,});
  }
}


impl fmt::Display for Test{
  fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
    write!(f, "Test{{")?;
    write!(f, "{}", self.value)?;
    write!(f, "}}")
  }
}

impl fmt::Display for Test2{
  fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
    write!(f, "Test2{{")?;
    write!(f, "{}",self.value)?;
    write!(f, "}}")
  }
}