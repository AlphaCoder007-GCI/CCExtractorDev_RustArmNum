pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let len = s.len() as u32;
    s.chars().map(|c| (c as u32 - '0' as u32).pow(len)).sum::<u32>() == num
}

fn main() {
    
    println!("It's the list of all Armstrong numbers between 0 and 1000.");

    for my_num in 0..1000 {
        // let my_num = 15;
        let ans = is_armstrong_number(my_num);
        
        if ans == true {
            println!("The given number {} is an Armstrong Number.", my_num);
        // } else {
        //     println!("The given number {} is not an Armstrong Number.", my_num);
        }
    }
}
