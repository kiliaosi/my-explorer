use std::collections::HashMap;

pub fn test_hash(){
   // let mut colects:HashMap<String,String>;
    let mut hashs = HashMap::new();
    hashs.insert(String::from("make"), 1);
    hashs.insert(String::from("change"),2);
    hashs.insert(String::from("fotal"),3);
    hashs.entry(String::from("mine")).or_insert(100);
    for (key,value) in &hashs{
        println!("key:{},value:{}",key,value);
    }
    println!("{:?}",hashs);
    //测试

    let strs = "hello china,my motherland, i very love you!";
    let mut hashs = HashMap::new();
    for word in strs.split_whitespace(){
        let count = hashs.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:?}",hashs);
    tests();
}

pub fn tests(){
    //给定一个vector  给出平均数，中位数，众数
    let mut vector = vec![3,4,8,91,2,6,8,0,2,4,8,0,4,2];
    //平均数
    let mut sum = 0;
    for item in &vector{
       sum+=item
    }
    println!("{}",sum/vector.len());

    //中位数
    let mut middle = 0;
    //排序
    &vector.sort_by(|a, b|{
        return a.cmp(b);
    });
    //取中位数
    if vector.len()%2==0{
        middle = vector[vector.len()/2];
    }
    println!("{}",middle);

    //众数
    let mut map = HashMap::new();
    for item in &vector{
        let count = map.entry(item).or_insert(0);
        *count+=1;
    }
    println!("{:?}",map);

}