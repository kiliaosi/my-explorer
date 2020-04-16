use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List{
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

use List::*;

fn main(){
  let value = Rc::new(RefCell::new(10));
  let value1 = Rc::new(Cons( Rc::clone(&value) , Rc::new(Nil)));
  let a1 = Cons( Rc::new(RefCell::new(1)), Rc::clone(&value1));
  let a2 = Cons( Rc::new(RefCell::new(2)), Rc::clone(&value1));
  //在运行时候，借用RefCell进行可变借用
  *value.borrow_mut()+=1;
  println!("{:?}",a1);
}