pub fn sort(mut data: Vec<i32>) -> Vec<i32> {
    let mut temp: i32;
    for _ in 0..data.len() - 1 {
        for i in 0..data.len() - 1 {
            if data[i] > data[i + 1] {
                temp = data[i + 1];
                data[i + 1] = data[i];
                data[i] = temp;
            }
        }
    }
    data
}