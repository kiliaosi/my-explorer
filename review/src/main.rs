
//关联类型及默认类型
#[allow(dead_code)]
trait Test<Types = String>{
  type Item;
  fn show(&self,add:Types);
}

struct Info<'a>{
  value:i32,
  name:&'a str,
}

//使用默认类型
// impl <'a> Test for Info<'a>{
//     type Item = i32;
//     fn show(&self,add:String){
//       println!("{}",add);
//     } 
// }

impl <'a> Test<i32> for Info<'a>{
    type Item = String;
    fn show(&self,add:i32){
        println!("{}",add);
    }
}

//迭代器
struct Node{
  value:i32,
}

impl Iterator for Node{
  type Item = i32;
  fn next(&mut self)->Option<Self::Item>{
    if(&self.value>=&5){
      return None
    }
    self.value+=1;
    Some(self.value)
  }
}

use std::ops::Deref;

#[derive(Debug)]
struct Mybox<T>(T);

impl <T> Mybox<T>{
  fn new(item:T)->Mybox<T>{
    Mybox(item)
  }
}

impl <T> Deref for Mybox<T>{
  type Target = T;
  fn deref(&self)->&T{
    &self.0
  }
}

struct TestDrop{
  value:String,
}
impl Drop for TestDrop{
  fn drop(&mut self){
    println!("{} drop",self.value);
  }
}


fn main() {
    let x = vec![1,2,3,4,5];
    
    let some_to_x = move |y|x==y;
    let result = some_to_x(vec![1,2,3,4,5]);
    println!("{}",result);
    // println!("{}",x);
    let name = String::from("张三");
    let test = Info{ name:&name,value:20 };


    //迭代器
    let user = Node{ value:100 };

    let somes = Mybox::new(10);
    println!("{}",*somes);

    //drop实验
    let value1 = TestDrop{ value:String::from("name space is error") };
    drop(value1);
    {
    let value2 = TestDrop{ value:String::from("test Interface") };
      
    }
}
