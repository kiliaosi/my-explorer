// 格式化输出
#[allow(dead_code)]
extern crate lib;
use lib::displays;
use lib::list;
use lib::enums;
use lib::link_list;
use std::fmt;

fn format_date(year:&str, mouth:&str, day:&str)->String{
    format!("{}-{}-{}", year, mouth, day)
}

fn main() {
    let strs = format_date("1994", "04", "05");
    println!("{}",strs);
    println!("stydy {0}- learn {0}- visit {0}- eating {1}",
      "rust", 
      "dinner");
    print!("{name}-{subject}-{dos}-{make}",
      name="张三", 
      subject = "study rust", 
      dos = "eating dinner", 
      make="every day");
      print!("PI is {:.2}",3.1415);
    
    let dis = displays::TestDisplay2{
          name: String::from("zansan"),
          age: 32,
          dos:String::from("make"),
        };
    println!("\n{}",dis);

    let ls = list::List(vec![1,2,3,4,5,6,7,8,9,0]);
    println!("{}",ls);
    
    // 十六进制
    println!("0x{:x}",90);
    // 八进制
    println!("0o{:o}",90);
    // 二进制
    println!("ob{:b}",90);

    let zero = enums::Test::Two;
    println!("{}", zero as i32);

    
    use lib::link_list::Mode;
    let mut list:link_list::Node<Mode> = link_list::Node::new();
    
    list = list.prepend(Mode(1));
    list = list.prepend(Mode(2));
    list = list.prepend(Mode(3));
    list = list.prepend(Mode(4));

    println!("{}", list.len());
    println!("{}", list.stringify());
}
