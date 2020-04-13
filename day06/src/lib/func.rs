//方法是依附于对象的函数

//例子   给出一个描述长方形的结构体， 给出计算面积和周长的函数

pub struct Rect{
  width:f64,
  height:f64,
}

impl Rect{
  fn new(width:f64, height:f64)->Rect{
    Rect{
      width,
      height,
    }
  }
  fn calc_per(&self)->f64{
    self.width*2.0+self.height*2.0
  }
  fn calc_area(&self)->f64{
    self.width*self.height
  }
}

#[cfg(test)]

#[test]
fn test_some(){
  let rect = Rect{
    width: 199f64,
    height:199f64,
  };
  rect.calc_per();
  rect.calc_area();
}