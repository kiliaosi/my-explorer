pub fn reverse_string(s: &mut Vec<char>) {
    let length = s.len();
    let index = length/2;
    let mut i = 0;
    while i<index{
        let tmp = s[i];
        let index2 = length-1-i;
        s[i] = s[index2];
        s[index2] = tmp;
        // s.swap(i,length-1-i);
        i +=1;
    }
}

pub fn check(item:&str)->bool{
    match &item.to_lowercase()[..]{
        "a"|"e"|"i"|"o"|"u"=>true,
        _=>false,
      }
}
pub fn reverse_mut(input:String)->String{
  let mut coll = input.split("").collect::<Vec<_>>();
  let mut left = 0;
  let mut right = coll.len()-1;
  
  while left<right{
    if check(coll[left]){
        if check(coll[right]){
          coll.swap(right,left);
          left+=1;
        }
        right-=1;
    }else{
        left+=1;
    }
  }
  
  return coll.join("").to_string();
}


pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut tmp = magazine.clone();
    for item in ransom_note.chars(){
        let s = tmp.find(item);
        if let Some(node) = s{
          tmp.remove(node);
        }else{
            return false;
        }
    }   
    return true;
}


pub fn first_uniq_char(s: String) -> i32 {
         
}