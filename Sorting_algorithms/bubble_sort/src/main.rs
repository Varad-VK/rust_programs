fn main() {
    let mut arr = [50,20,40,30,10,80,60,90,70];
    let n= arr.len();

    for i in 0..n{
        let mut is_swap = false; // to achive O(n) time complexity if array is already sorted
        for j in 0..(n-i-1){
            if arr[j]>arr[j+1]{
                arr.swap(j, j+1);
                is_swap = true;
            }
        }
        if !is_swap{
            break;
        }
        
    }
    println!("{arr:?}");
}
