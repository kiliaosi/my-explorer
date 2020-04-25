pub fn prefix(strs:Vec<String>)->String{
    let mut fix = 0;
    let mut container:&str = "";
    if strs.len() <=0 {
      return "".to_string();
    }
    if strs.len() == 1{
      return strs[0].clone();
    }
    loop{
        let mut tmp:&str="";
        let mut is_break = false;
        for item in &strs {
            
            if item.len()<=0 {
                
                return String::from("");
            }
            
            if fix>item.len(){
                is_break = true;
                break;
            }

            if tmp.len()==0{
                tmp = &item[0..fix];
                continue;
            }
            
            if tmp != &item[0..fix] {
                println!("怎么不对了{},{},{}",tmp, fix, container);
                return String::from(container);
            }
        }
        if is_break{
            break;
        }
        fix+=1;
        println!("{}",fix);
        container = tmp;
    }
    println!("{}",container);
    return String::from(container);
}