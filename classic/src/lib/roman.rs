fn roman_parse(item: char) -> Option<i32> {
    match item {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _ => None,
    }
}

pub fn roman(s: String) -> i32 {
    let mut value = 0;
    s.chars()
        .zip(s.chars().skip(1))
        .for_each(|(first, second)| {
            let f = roman_parse(first).unwrap();
            let s = roman_parse(second).unwrap();
            value += if f > s { f } else { -1 * f }
        });
    return value;
}

/// 版本2
pub fn roman2(s: String) -> i32 {
    let mut tmp: i32 = 0;
    let ls = s.chars();
    let mut sum = 0;
    for item in ls {
        let somes = match item {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        // println!("{}",somes);
        sum += if tmp < somes { -1 * tmp } else { tmp };
        tmp = somes;
    }
    // println!("{}", sum+tmp);
    return sum+tmp;
}
