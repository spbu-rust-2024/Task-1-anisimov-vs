use std::io;

fn shell_sort(array: &mut Vec<i16>) {
    let len_array = array.len();
    let mut gap = len_array / 2;

    while gap > 0 {
        for i in gap..len_array {
            let temp = array[i];
            let mut j = i;

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

    let mut array: Vec<i16> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    shell_sort(&mut array);

    println!(
        "{}",
        array
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
