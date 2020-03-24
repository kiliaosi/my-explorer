mod calc;

pub fn show_make(){
    print!("day02\n");
    let user = calc::User::new(String::from("zhangsna"),18,calc::Sex::Male);
    println!("{:?}",user);
    let data = Some(190);
    if let calc::Sex::Male=user.sex{
      print!("190");
    }

    let mut vecs:Vec<i32> = vec![1,2,3,4,5,6,7];
    //更新vector
    vecs.push(10);
    println!("{:?}",vecs);
    let test = vecs.get(2).expect("out of range");
}

#[test]
fn test(){
    show_make();
}