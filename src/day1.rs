pub fn day1() {
    let strs = include_str!("../day1.txt").lines().collect::<Vec<_>>();

    let mut total = 0;
    for str in &strs {
        total += get_first_number(str) * 10 + get_last_number(str);
    }
    println!("Total: {}", total);
}

const NUMBER_STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn get_first_number(str: &str) -> usize {
    let mut num = 0;
    let mut index = str.find(|s: char| s >= '0' && s <= '9');
    if let Some(index) = index {
        num = str[index..index + 1].parse::<usize>().unwrap();
    }
    for (i, number_str) in NUMBER_STRS.iter().enumerate() {
        if let Some(idx) = str.find(number_str) {
            if index.is_none() || idx < index.unwrap() {
                index = Some(idx);
                num = i + 1;
            }
        }
    }
    //println!("{}: {}", str, num);
    num
}
fn get_last_number(str: &str) -> usize {
    let mut num = 0;
    let mut index = str.rfind(|s: char| s >= '0' && s <= '9');
    if let Some(index) = index {
        num = str[index..index + 1].parse::<usize>().unwrap();
    }
    for (i, number_str) in NUMBER_STRS.iter().enumerate() {
        if let Some(idx) = str.rfind(number_str) {
            if index.is_none() || idx > index.unwrap() {
                index = Some(idx);
                num = i + 1;
            }
        }
    }
    //println!("{}: {}", str, num);
    num
}
