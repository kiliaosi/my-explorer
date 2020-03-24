pub mod day;

pub fn call_str(){
    let mut strs = "test string".to_string();
    strs.push_str("handle click");
    let sli = "make change";
    strs.push_str(sli);
    println!("{}",sli);
    for item in strs.chars(){
        println!("{}",item);
    }
}

#[test]
fn calc(){
  println!("calc");
}