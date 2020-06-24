use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Display;

struct Events<T>{
  maps: HashMap<String, Vec<Rc<Fn(&Vec<T>)->()>>>,
}

impl<T> Events<T>{
  fn new()->Self{
    let maps:HashMap<String, Vec<Rc<Fn(&Vec<T>)->()>>> = HashMap::new();
    Events{
      maps,
    }
  }

  fn emit(&mut self, event_name: String, args:Vec<T>)
  where T:Display
  {
    let fns = &self.maps.get(&event_name).unwrap();
    for item in fns.iter() {
      item(&args);
    }
  }

  fn on(&mut self, event_name:String, handle:Rc<Fn(&Vec<T>)->()>){
    let ls:Vec<Rc<Fn(&Vec<T>)->()>> = Vec::new();
    self.maps.entry(event_name.clone()).or_insert(ls);
    let ref mut list = self.maps.get_mut(&event_name).unwrap();
    list.push(handle);
  }
}

fn main(){
  let mut somes:Events<i32> = Events::new();
  somes.on("change".to_string(), Rc::new(|arr:&Vec<i32>|{
    println!("change one {}", 1);
  }));
  somes.on("change".to_string(), Rc::new(|arr:&Vec<i32>|{
    println!("change two {}", 2);
  }));
  somes.on("change".to_string(), Rc::new(|arr:&Vec<i32>|{
    println!("change three {}", 3);
  }));
  somes.emit("change".to_string(), vec![1,2,3,4,5]);
}
