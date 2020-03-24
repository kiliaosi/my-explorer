//ownership
//栈上的数据，不会移动，不会移交所有权
//堆上的数据行为，被所有权系统支配
pub fn owner(){
  let str_owner = String::from("make some changes!");
  let owner2 = str_owner;
  //错误：所有权已被移动
  //println!("{}",str_owner); 
  //函数在传值时，会移交所有权，  如果不想移交，使用引用
  //如果想更改原始值，请使用可变引用，不可变数据不能拥有可变引用，在拥有一个不可变引用时候，同时不嫩能共有一个可变引用
  let mut strs = String::from("test ownership in function");
  test_owner(&mut strs);
  println!("{}",strs);

  let strs = String::from("make some");
  let index:usize = slice_test(&strs);
  println!("{}",&strs[..index]);
}

//slice
fn slice_test(strs:&String)->usize{
    let lists = strs.as_bytes().iter().enumerate();
    for (i ,&j) in lists{
        if j==b' '{
            return i;
        }
    }
    return strs.len();
}

fn test_owner(strs:&mut String){
    strs.push_str(";new str");
    print!("{}",strs);
}

#[test]
fn test(){
  owner();
}