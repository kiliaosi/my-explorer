pub trait GetName{
  fn get_name(&self)->&String;
}

pub trait GetAge{
  fn get_age(&self)->u32;
}

trait ShowAllInfo{
  fn print_all(&self);
}

pub struct Org {
  name:String,
  age:u32,
}

impl GetAge for Org{
  fn get_age(&self)->u32{
      self.age
  }
}

impl GetName for Org{
  fn get_name(&self)->&String{
      &self.name
  }
}