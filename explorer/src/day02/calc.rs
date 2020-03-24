#[derive(Debug)]
pub enum Sex{
    Male,
    Female,
}

#[derive(Debug)]
pub struct User{
  pub username:String,
  pub age:u32,
  pub sex:Sex,
}
impl User{
  pub  fn new(username:String,age:u32,sex:Sex)->User{
        User{
            username,
            age,
            sex,
        }
    }
    pub fn show_info(&self){
        println!("{},{},{:?}",self.username, self.age, self.sex);
    }
    pub fn show_sex(&self)->String{
        match self.sex{
            Sex::Male=>{
                return String::from("male");
            },
            Sex::Female=>{
                return String::from("Female");
            }
        }
    }
}