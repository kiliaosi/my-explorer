// 链表实现方式参照Lisp语言的cons List
use std::fmt;
use std::fmt::Display;
pub enum Node<T>{
  Cons(T, Box<Node<T>>),
  Nil,
}

pub use Node::{ Cons, Nil };

impl <T>  Node<T>{
  pub fn new()->Node<T>{
    Nil
  }
  pub fn len(&self)->u32{
    match self {
        Cons(ele, ref top) => 1+top.len(),
        Nil => 0,
    }
  }
  pub fn prepend(self, ele:T)->Node<T>{
    Cons(ele, Box::new(self))
  }
}

// 字符串格式化trait
trait Formats{
  fn stringify(&self)->String;
}

impl <T:Display> Formats for Node<T>{
  fn stringify(&self)->String{
    match self {
        Cons(ele, ref top) => format!("{}<-{}", ele, top.stringify()),
        Nil => format!("Nil"),
    }
  }
}

pub struct Mode(i32);
impl fmt::Display for Mode{
  fn fmt(&self,f: &mut fmt::Formatter)->fmt::Result{
    write!(f, "{}",self.0)
  }
}

