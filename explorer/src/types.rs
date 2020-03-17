
//rust 基本类型
//int  float  bool char
//tuple  array

pub fn show_type(){
     //i8 i16 i32 i6 i128
     let num1:i32 = -189;
    
     //u8 ~
     let num2:u32 = 190;
     //f32 f64
     let num3:f64 = 198.178;
     //bool
     let falg:bool = false;
     //char
     let strs:char = 'a';
 
     println!("Hello, world!{},{},{},{},{}",num1,num2,num3,falg,strs);

     let tup:(i32,f32,char,bool) = (10,20.2,'a',false);
     println!("{:?}",tup);

     //数组
     let arr:[i32;3] = [1,2,3];
     let arr2 = [3;5];
     println!("{:?}\n{:?}",arr,arr2);
}