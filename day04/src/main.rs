// 格式化输出
extern crate lib;
use lib::displays;

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
}
