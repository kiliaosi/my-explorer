trait GetName{
  fn get_name(&self)->&String;
} 

trait  ShowInfo {
  fn prints(&self);
}

impl <T:GetName>  ShowInfo for T{
    fn prints(&self){
        println!("name={}",self.get_name());
    }
}


fn main() {
    println!("Hello, world!");
}
