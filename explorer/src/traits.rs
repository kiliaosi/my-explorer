pub trait Test {
    fn calc(&self)->String;
}

pub trait Summer{
    fn calc_inter(&self)->String;
}

pub struct Node{
  value:String,
  age:i32,
}

impl Test for Node{
    fn calc(&self)->String{
        self.value.clone()
    }
}

pub fn calc_test(item:&impl Test)->Option<String>{
    item.calc();
    return None;
}
pub fn calc_test2<T:Test>(item:&T)->Option<T>{
    return None;
}
pub fn calc_inter3<'a,'b,T:Test+Summer>(item:&T)->Option<String>{
    return None;
}