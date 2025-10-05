fn main() {
    let mut arr = [40, 20, 10, 30, 50];
    selection_sort(&mut arr); // time complexity : O(n^2) in all cases
    println!("sorted arr: {:?}", arr);
}

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..(n - 1) {
        let mut smalllest = i;
        for j in (i + 1)..n {
            if arr[j] < arr[smalllest] {
                smalllest = j;
            }
        }
        if smalllest != i {
            arr.swap(i, smalllest);
        }
    }
}
