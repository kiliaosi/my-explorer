pub enum Node{
  Cons(i32, Box<Node>),
  Nil,
}

impl Node{
  fn new()->Node{
     Node::Nil
  }
  fn prepend(self,ele:i32)->Node{
    Node::Cons(ele, Box::new(self))
  }
}