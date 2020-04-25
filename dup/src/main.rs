use std::collections::HashMap;

fn dup(word:&str)->String{
  let mut map:HashMap<char,usize> = HashMap::new();   
  let mut strings = String::new();   
  let tmp = word.to_lowercase().to_string();
  
  let mut index = 0;
  
  for item in tmp.chars(){
    let maps = map.get(&item);
    if let Some(value) = maps{
        strings.push_str(")");     
    }else{
        map.insert(item, index);
        let refs = &tmp[index+1..];
        if refs.contains(item){
            strings.push_str(")");
        }else{
            strings.push_str("(");
        }
    }
    index+=1;
  }
  return strings;
}

fn main() {
  let strings = String::from("hash");
  let test = &strings[0..];
  dup("Success");
  println!("{}",test);
}
