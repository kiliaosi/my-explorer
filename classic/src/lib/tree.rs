use std::rc::Rc;
use std::cell::RefCell;
/// 完全二叉树的节点个数
pub struct TreeNode{
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode{
  fn new(item:i32)->Self{
    TreeNode{
      value:item,
      left:None,
      right:None,
    }
  }
}

pub fn cout(node:Option<Rc<RefCell<TreeNode>>>)->i32{
  match node{
    None=>0,
    Some(**nodes)=>{
        return cout(nodes.left)+1+count(nodes.right);
    }
  }
}
