pub fn loops(){
  //打印 1~100之间的所有偶数
  let mut num = 0u32;
  loop{
    num+=1;
    if num%2==0{
        println!("{}",num);
    }
    if num == 100{
        break;
    }
  }
}

pub fn fors(){
  // 打印1~100中所有的奇数
  for item in 1..=100{
    if item %2!=0{
      println!("{}", item);
    }
  }
}