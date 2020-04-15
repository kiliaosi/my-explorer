//文档注释
//! 这是一个测试模块
//! 
/// Add one to the number
/// 
/// #Example
/// 
/// ```
/// let mut num = 200;
/// let result = add_one(num);
/// assert_eq!(201, num);
pub fn add_one(num:i32)->i32{
  num+1
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
