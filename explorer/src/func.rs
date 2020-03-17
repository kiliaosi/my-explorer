//学习函数相关的模块
//第一个练习，99乘法表


pub fn nine(){
    for  i in 1..=9{
        for j in 1..=i{
            print!("{}*{}={}\t",j,i,j*i);
        }
        print!("\n");
    }
}

//块表达式,例子，计算空间直角坐标系中点到原点的距离
pub fn block_expression(x:i32,y:i32,z:i32)->f64{
    let squre:f64 = {
        let sum = x*x+y*y+z*z;
        sum as f64
    };
    let num = squre.sqrt();
    println!("{}",num);
    return 3.14;
}

//控制流
pub fn control_flow(flag:i32){
  //if 表达式
  if flag==3{
      print!("flag is three");
  }else{
      print!("bad input");
  }
  //可以有返回值
  let num = if flag==3{
      10
  }else{
      9
  };
  
  println!("{}",num);
  //循环
  let mut start = 0;
  loop{
    start= start+2;
    if(start==4){
        break;
    }
    println!("flow or :{}",start);
  }
  //while
  while start<=10{
      start=start+2;
      print!("excute or :{}\n",start);
  }
  //for
  let arr:[i32;4] = [1,2,3,4];
  for i in arr.iter(){
      println!("{}",i);
  }
}

pub fn calc_fib(num:i32)->i32{
    if num<=2{
      return num;
    }
    return calc_fib(num-1)+calc_fib(num-2);
  }

#[test]
fn test(){
   let nums:i32 = calc_fib(10);
   nine();
   block_expression(1, 2, 3);
   control_flow(100);
   println!("{}",nums);
}