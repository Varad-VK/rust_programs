use std::collections::HashMap;

fn main() {
    let s1=String::from("hello");
    
    if is_palindrome(&s1){
        println!("{} is palindrome!",s1);
    }else{
        println!("{} is not palindrome",s1);
    }

    if let Some(count) = count_char(&s1) {
        println!("Character counts: {:?}", count);
    } else {
        println!("Failed to count characters.");
    }

    if let Some(uppercase) = uppercase(&s1){
        println!("{} in uppercase letters is {}", s1,uppercase);
    }else{
        println!("Failed to get uppercase string of {}",s1);
    }

    if let Some(lowecase) = lowercase(&s1.to_uppercase()){
        println!("{} in lowecase letters is {}", s1.to_uppercase(),lowecase);
    }else{
        println!("Failed to get lowecase string of {}",s1);
    }
}

fn is_palindrome(org:&String)->bool{
    org==&org.chars().rev().collect::<String>()
}

fn count_char(org:&String)->Option<HashMap<char,u32>>{
    let mut count:HashMap<char,u32> = HashMap::new();
    for c in org.chars(){
        *count.entry(c).or_insert(0)+=1;
    } 

    Some(count)
}

fn uppercase(org:&String)->Option<String>{
    Some(org.to_uppercase())
}

fn lowercase(org:&String)->Option<String>{
    Some(org.to_lowercase())
}