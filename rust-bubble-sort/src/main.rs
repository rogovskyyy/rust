mod bubble;

fn main() {
    let mut arr: Vec<i32> = vec![0; 8192];
    let mut counter: i32 = 8191;

    for item in 0..arr.len() - 1 {
        arr[item] = counter;
        counter = counter - 1;
    }
    
    arr = bubble::sort(arr);

}