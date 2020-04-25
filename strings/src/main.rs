// 回文对

fn test(item:&String, item2:&String)->bool{
  let compose:String = item.clone()+&item2.clone();
  return compose.as_bytes().iter().zip(compose.as_bytes().iter().rev()).all(|(first,second)|{
    println!("sum={},{}",first,second);
    return first==second;
  });
}

pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
  let mut tmp:Vec<Vec<i32>> = Vec::new();
  let mut index = 0;
  for item in words.iter(){
    let mut index2 = 0;
    for item2 in words.iter(){
      println!("{},{}",item,item2);
      if item.as_ptr()!=item2.as_ptr(){
        println!("{},{},index={},index2={}",item,item2,index,index2);
        if test(item, item2){
          tmp.push(vec![ index, index2 ]);
        }
      }
      index2+=1;
    }
    index+=1;
  }
  return tmp;
}

fn main() {
    // let ts = test("abcdedba");
    // let result = palindrome_pairs(vec!["abcd".to_string(),"dcba".to_string(),"lls".to_string(),"s".to_string(),"sssll".to_string()]);
    // println!("{}","abcd".to_string().as_ptr()=="abcd".to_string().as_ptr());
    // let result = test(&String::from("sssll"),&String::from("s"));
    // println!("{:?}", result);
    let mut s = String::from("hello wolrd");
    
    println!("{}",s.split("").rev().collect::<Vec<_>>());
  }
