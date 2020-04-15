///二叉树

enum BinaryTree{
  Node(Box<BinaryTree>, i32, Box<BinaryTree>),
  Nil,
}

use BinaryTree::*;

impl BinaryTree{
  fn new()->BinaryTree{
    Nil
  }
 
  ///先序遍历
  fn preorder(&self){
    match self{
      Nil=>{
          println!("结束");
      },
      Node(left,value,right)=>{
          println!("{}",value);
         left.preorder();
         right.preorder();
      },
    }
  }
  ///中序遍历
  fn seqorder(&self){
    match self{
      Nil=>println!("end"),
      Node(left, value, right)=>{
        left.seqorder();
        println!("{}",value);
        right.seqorder();
      }
    }
  }
  /// 后续遍历
  fn postorder(&self){
    match self{
        Nil=>println!("end"),
        Node(left,value,right)=>{
          left.postorder();
          right.postorder();
          println!("{}",value);
        }
    }
  }
}

fn main() {
    let mut root = 12;
    //1
    //2 3
    //4 5 6 7
    //8 9 10 11 12 13 14 15
    let list = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let b_tree = Node(
        Box::new(Node( Box::new(Node(Box::new(Nil),4, Box::new(Nil) )), 2 ,Box::new( Node( Box::new(Nil), 5 ,Box::new(Nil)) ))),
        1,
        Box::new(Node( Box::new(Node(Box::new(Nil),6, Box::new(Nil) )), 3 ,Box::new( Node( Box::new(Nil), 7 ,Box::new(Nil)) ))),
    );
    //先序遍历
    b_tree.preorder();
    println!("###########################################");
    //中序遍历
    b_tree.seqorder();
    println!("###########################################");
    //后序遍历
    b_tree.postorder();
}