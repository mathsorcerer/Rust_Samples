fn main() {
    let in_str = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. ");
    println!("Result: {}", count_sub_str(&in_str, "is is"));
}

fn count_sub_str(in_str: &str, in_pattern: &str) -> u32 {
    let n = in_str.len();
    let m = in_pattern.len();
    let mut res = 0;

    let in_str = in_str.as_bytes();
    let in_pattern = in_pattern.as_bytes();

    for i in 0..(n-m+1) {
        let mut j = 0;
        while j < m {
            if in_str[i+j] != in_pattern[j] {
                break;
            }
            j += 1;
        }
        if j == m {
            res += 1;
        }
    }
    res
}