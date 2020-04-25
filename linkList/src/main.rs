#[derive(Debug)]
struct ListNode {
  val:i32,
  next:Option<Box<ListNode>>,
}

impl ListNode{
  fn new(value:i32)->Self{
    ListNode{
      val:value,
      next:None,
    }
  }

}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  
  if let None = l1{
    return l2;
  }
  if let None = l2{
    return l1;
  }
  let mut ptr1 = &l1;
  let mut ptr2 = &l2; 
  let mut ls:ListNode = ListNode::new(ptr1.unwrap().val+ptr2.unwrap().val);
  let mut refCells = ls;
  
  loop{
    let mut sum:Option<i32> = None;
    let tmp1 = &ptr1.unwrap().next;
    let tmp2 = &ptr2.unwrap().next;
    match tmp1 {
        Some(node) => {
          sum = Some(node.val);
          ptr1 = &ptr1.unwrap().next;
        },
        None => {},
    }
    match tmp2 {
        Some(node) => {
          if let Some(item) = sum{
            sum = Some(item+node.val);
          }
          ptr2 = &ptr2.unwrap().next;
        },
        None => {},
    }
    if let Some(v) = sum{
      refCells.next = Some(Box::new(ListNode::new(v)));
      refCells = *refCells.next.unwrap();
    }else{
        return Some(Box::new(ls))
    }
  }
}

fn main() {
    println!("Hello, world!");
}
