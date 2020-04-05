use std::fmt;

#[derive(Debug)]
pub struct TestDisplay{
  name: String,
  age:i32,
}

// 自己实现display trait
pub struct TestDisplay2{
 pub name:String,
 pub age:i32,
 pub dos:String,
}

impl fmt::Display for TestDisplay2{
  fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
    write!(f," name:{},age:{} ",self.name, self.age)
  }
}

#[test]
fn test(){
  let node = TestDisplay{
    name:"zhangsan".to_string(),
    age:32,
  };
  println!("{:?}",node);
}