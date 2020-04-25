use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Test{
  left:Option<Rc<RefCell<Test>>>,
  val:i32,
}

fn main(){
  let someTree = Some( Rc::new(RefCell::new(Test{ val:1,left:None  })) );

  match someTree {
      Some(ref rcs) => {
        let somes = &rcs.borrow().left;
        println!("{:?}", somes);
      },
      None => {
        print!("none");
      },
  }
}
