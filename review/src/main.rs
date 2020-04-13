
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


fn main() {
    let x = vec![1,2,3,4,5];
    
    let some_to_x = move |y|x==y;
    let result = some_to_x(vec![1,2,3,4,5]);
    println!("{}",result);
    // println!("{}",x);
    let name = String::from("张三");
    let test = Info{ name:&name,value:20 };
}
