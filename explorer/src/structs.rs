#[derive(Debug)]
enum Sex{
  male,
  female
}

#[derive(Debug)]
pub struct Student{
  username:String,
  age:u32,
  sex:Sex,
}

#[test]
fn test(){
  let stds = Student{
    username:String::from("李四"),
    age:32,
    sex:Sex::male,
  };
  println!("{:?}",stds);
}