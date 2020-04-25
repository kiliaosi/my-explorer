use std::collections::HashMap;

fn main() {
    is_palindrome(101);
    //zip
    let ls = vec![1,2,3,4,5,6,7];
    let ls2 = [8,9,10,11,12];
   let iter =  ls.iter().zip(ls2.iter()).map(|num|{ println!("{:?}",num) ; return num;});
    for  item in iter{
        println!("{:?}", item);
    } 
}


// zhengs

fn  is_palindrome(x: i32) -> bool {
   if x<0{
       return false;
   }
   let mut temp = x;
   let mut sum = 0;
   let mut base = 10;
   loop{
       sum = sum*base+temp%10;
       temp = temp/10;
       if(temp<1){
           break;
       }
   }
   return x == sum;
}

fn  is_palindrome2(x: i32) -> bool {
    let tmp = String::from(x.to_string());
  let  strs:Vec<_> = (&tmp).as_bytes().iter().collect();

  let mut point = strs.len();
  for item in &strs{
     point-=1;
     if strs[point] != *item{
         return false;
     }
     println!("{}",item);
  }
  return true;
}

// fn roman_num(s:String)->i32{
//     let list: = HashMap::new();
// }