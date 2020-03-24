use std::fs::File;
use std::io::ErrorKind;

pub fn call_error(){
  let file_handle = File::open("./test.txt");
  match &file_handle{
      Ok(info)=>{
          println!("打开文件成功:{:?}",info);
      },
      Err(info)=>{
        println!("打开文件失败");
      },
  }
  
  let file_handle = File::open("target.txt").unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound{
          println!("not found");
          File::create("target.txt").expect("handl error")
      }else{
          println!("未知错误");
          panic!("miss")
      }
  });
}