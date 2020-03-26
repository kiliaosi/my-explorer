mod types;
mod func;
mod ownership;
mod structs;
mod day02;
mod day03;
mod error_handle;
mod review324;
mod traits;
mod life;
mod day04;
mod iter;
fn main() {
   //types::show_type();
   //func::nine();
   //func::block_expression(1,1,1);
   //func::control_flow(3);
   //ownership::owner();
   //day02::show_make();
   // day03::call_str();
   // day03::day::test_hash();
   // error_handle::call_error();
   // let config = day04::Config{
   //    path:String::from("test.txt"),
   //    key:String::from("hello"),
   // };
   // day04::grep(&config);
   iter::test_iter();
}
