// 对vec数据实现display trait
use std::fmt;

pub struct List(pub Vec<i32>);

impl fmt::Display for List{
  fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
    let ls = &self.0;
    write!(f, "[")?;
    for item in ls{
      write!(f, "{},",item)?;
    }
    write!(f, "]")
  }
}