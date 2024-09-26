use std::io;

fn shell_sort(array: &mut Vec<i32>) {
    let len_array: usize = array.len();
    let mut gap: usize = len_array / 2;
    while gap > 0 {
        for i in gap..len_array {
            let temp: i32 = array[i];
            let mut j: usize = i;
            while (j >= gap) && (array[j - gap] > temp) {
                array[j] = array[j - gap];
                j -= gap;
            }
            array[j] = temp;
        }
        gap /= 2;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut array: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    shell_sort(&mut array);
    let output = array
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}
