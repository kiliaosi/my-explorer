pub fn test_iter(){
    let col:Vec<u32> = vec![1,2,3,4,5,6,7,8,9,0];
    let mut iter1:Vec<u32> =  col.iter().map(|item|item+1).collect();
    let mut iter2:Vec<u32> = col.into_iter().filter(|num|*num%2==0).collect();
    println!("{:?}",iter2);
}

struct Items{
  size:u32,
}

impl Iterator for Items{
    type Item = u32;
    fn next(&mut self)->Option<Self::Item>{
        self.size+=1;
        if self.size<=10{
            return Some(self.size);
        }
        return None;
    }
}