extern crate lib;
use lib::large;

fn main() {
   let list = vec![1,2,3,4,5,6,7,8,1,2,3,4445,5,6,8];
   let num = large::largest(&list);
   println!("{}",num);

   let char_list = vec!['a','b','A','B'];
   let result = large::largest(&char_list);
   println!("{}", result);
}
