use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt")
        .expect("Should have been able to read the file");

    let mut lines: Vec<i32> = Vec::new();

    let mut sum: i32 = 0;

    for line in contents.lines() {
        if line != "" {
            let num: i32 = line.parse().unwrap();

            sum += num;
        } else {
            lines.push(sum);

            sum = 0;
        }
    }

    let max_value = lines.iter().max().unwrap();

    println!("Max: {max_value}");

    lines.sort_by(|a, b| b.cmp(a));

    let mut top_3: i32 = 0;

    for i in 0..3 {
        top_3 += lines[i];
    }

    println!("Top 3: {top_3}");
}
