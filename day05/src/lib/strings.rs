use std::string::ToString;
use std::str::FromStr;

pub struct FmtString{
  value:String,
  mock:i32,
}

impl ToString for FmtString{
  fn to_string(&self)->String{
    format!("FmtString{{ value:{},mock:{} }}",self.value, self.mock)
  }
}

// 将字符串转换为目标类型
impl FromStr for FmtString{
  type Err = ();
  fn from_str(s:&str)->Result<Self, Self::Err>{
   let coord:Vec<&str> =  s.trim_matches(|strs|strs=='{' || strs=='}' || strs==' ')
                           .split(",")
                           .collect();
    
  }
}

#[test]
fn to_string(){
  let somes = FmtString{
    value:String::from("make change"),
    mock:10,
  };
  somes.to_string();
}