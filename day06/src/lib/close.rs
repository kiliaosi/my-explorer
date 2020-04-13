pub fn test(){
  let mut a = 20;
  let mut handle = move ||{ a+=10;println!("{}",a); };
  handle();
  handle();
}