fn main() {
    let n = 100;
    println!("Occurance of 0 is :{}", find_0_occrance(n));
}

fn find_0_occrance(n: i32) -> i32 {
    let mut count = 0;
    for i in 0..=n {
        let num = i.to_string();
        for c in num.chars() {
            if c == '0' {
                count += 1;
                // println!("{}, ",num);
            }
        }
    }
    count
}
