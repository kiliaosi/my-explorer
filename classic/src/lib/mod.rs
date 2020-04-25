pub mod nums;
pub mod roman;
pub mod prefix;
pub mod reverses;
pub fn init(){
  // nums::run();
  // roman::roman(String::from("helloworld"));
  // roman::roman2(String::from("IV"));

  // let strs = String::from("1423423");
  // let s = &strs[0..0];
  // // println!("{}",s);
  // // let fix = prefix::prefix(vec!["flower".to_string(), "flow".to_string(),"flight".to_string()]);
  // let fix = prefix::prefix(vec!["c".to_string(), "c".to_string(),"c".to_string()]);
 
   let mut strings = vec!['h','e','l','o'];
   reverses::reverse_string(&mut strings);
  //  println!("{:?}", strings);
   let result = reverses::reverse_mut("hello".to_string());
   println!("{}",result);
}