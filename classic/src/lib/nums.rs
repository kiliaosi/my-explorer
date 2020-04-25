/// 1，2，3，4 组成无重复三位数

pub fn run(){
  for item in 1..5{
      for item2 in 1..5{
          for item3 in 1..5{
              if item != item2 && item != item3 && item2 != item3{
                  println!("{}{}{}",item,item2,item3);
              }
          }
      }
  }
}