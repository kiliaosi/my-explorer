fn call_task(s:String)->bool{
  let revert = |item:char|{
    match item {
      '}'=>Some('{'),
      ')'=>Some('('),
      ']'=>Some('['),
      _=>None,
    }
  };
  
  let mut stack:Vec<char> = Vec::new();
  for item in s.chars(){
    let r1 = revert(item);
    println!("{},{:?}",item, r1);
    match r1 {
      Some(rc1)=>{
        let result = stack.pop();
        match result{
          Some(rc2)=>{
            if rc1 != rc2 {
              return false;
            }
          },
          None=>{return false},
        }
      },
      None=>{
        stack.push(item);
      }
    } 
  }
  if stack.len()!=0{
    return false;
  }
   return true;
}

fn main() {
  let mut vecs = vec![1,2,3];
  vecs.push(12);
  let somes = call_task(String::from("{}{}[]{}"));
  println!("{}",somes);
}
