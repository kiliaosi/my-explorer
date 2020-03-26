//硬编码模拟命令行参数
use std::fs;

pub struct Config{
   pub path:String,
   pub key:String,
}

pub fn grep(config:&Config){
    let content = fs::read_to_string(&config.path).expect("no such file");
    for line in content.lines(){
        if line.contains(&config.key){
            println!("{}",line);
        }
    }
}