//类型转换之 From: 
use std::convert::From;
use std::fmt;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Number{
  pub value:i32,
}

//实现Form trait后 会自动获得into方法
impl From<i32> for Number{
  fn from(item:i32)->Self{
    Number{
      value: item,
    }
  }
}


impl fmt::Display for Number{
  fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
    write!(f, "Number {{ value: ")?;
    write!(f,"{}",self.value)?;
    write!(f, " }} ")
  }
}

// TryFrom

#[derive(Debug)]
pub struct EvenNumber(pub i32);

impl TryFrom<i32> for EvenNumber{
  type Error = ();
  fn try_from(value:i32)->Result<Self,Self::Error>{
    if value%2==0{
      Ok(EvenNumber(value))
    }else{
      Err(())
    }
  }
}

