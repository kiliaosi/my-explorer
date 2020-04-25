fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut vss:Vec<T> = Vec::new();
    for item in a{
        if !b.contains(&item){
            &vss.push(item);
        }
    } 
    return vss;
  }

fn main() {
   let vecs = vec![1,2,3,4,5,6];
   let somes = vecs.iter().filter(|item|{
       return **item != 2;
   }).collect::<Vec<_>>();
  println!("{:?}",somes);
}
