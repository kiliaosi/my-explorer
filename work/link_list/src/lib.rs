pub struct Node{
  pub value:i32,
  pub next:Box<Option<Node>>,
}

pub fn gener(){
    let mut link = Box::new(Some(Node{value:0,next:Box::new(None),}));
    let mut refLik = &link;
  for i in (1..10){
    refLik.next = Box::new(Some(Node{ value:i, next:Box::new(None), }));
    refLik = refLik.next;
  }
}