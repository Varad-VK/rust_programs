use std::collections::HashMap;
fn main(){
    let word = String::from("hello");
    find_most_occurance_char(&word);

}

fn find_most_occurance_char(org:&String){
    let mut counter:HashMap<char, i32> = HashMap::new();
    for c in org.chars(){
        *counter.entry(c).or_insert(0)+=1;
    }
    print!("HashMap => ");
    for (key,value) in &counter{
        print!("{}:{} | ",key,value);
    }

    let mut max_char = ' ';
    let mut max_count = 0;

    for (key, &value) in &counter {
        if value > max_count {
            max_count = value;
            max_char = *key;
        }
    }
    
    println!("\nMost occurring character: '{}' with {} occurrences", max_char, max_count);

}