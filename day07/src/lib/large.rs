pub fn largest<T:PartialOrd+Copy>(lists:&[T])->T{
  let mut larger = lists[0];
  for &item in lists.iter(){
    if item>larger{
        larger = item;
    }
  }
  larger
}