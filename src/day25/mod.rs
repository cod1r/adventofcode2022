fn conv_to_dec(s: String) -> i128 {
    s.trim().lines().fold(0_i128, |acc, line| {
        let bytes = line.as_bytes();
        let mut ans = 0;
        for i in (0..bytes.len()).rev() {
            ans += match bytes[i] {
                b'2' => 5_i128.pow((bytes.len() - 1 - i).try_into().unwrap()) * 2,
                b'1' => 5_i128.pow((bytes.len() - 1 - i).try_into().unwrap()),
                b'0' => 0_i128,
                b'-' => -(5_i128.pow((bytes.len() - 1 - i).try_into().unwrap())),
                b'=' => 5_i128.pow((bytes.len() - 1 - i).try_into().unwrap()) * -2,
                _ => unreachable!(),
            }
        }
        acc + ans
    })
}
fn conv_to_snafu(mut n: i128) -> String {
    let mut s = String::new();
    let mut carry: i128 = 0;
    while n > 0 {
        let rem = n % 5;
        let added = rem + carry;
        if added > 2 {
            match added {
                3 => s += "=",
                4 => s += "-",
                5 => s += "0",
                _ => unreachable!(),
            }
            carry = 1;
        } else {
            s += &added.to_string();
            carry = 0;
        }
        n /= 5;
    }
    if carry == 1 {
        s += "1";
    }
    s.chars().rev().collect::<String>()
}
pub fn day25() {
    let input_str = include_str!("input.txt");
    let part1 = conv_to_dec(input_str.to_string());
    println!("part1: {}", conv_to_snafu(part1));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one() {
        assert!(conv_to_dec("10-0".to_string()) == 120);
    }
    #[test]
    fn two() {
        assert!(conv_to_snafu(120) == "10-0");
    }
    #[test]
    fn three() {
        assert!(conv_to_snafu(33) == "12=");
    }
    #[test]
    fn four() {
        assert!(conv_to_dec(conv_to_snafu(33)) == 33);
    }
    #[test]
    fn five() {
        assert!(conv_to_dec(conv_to_snafu(120)) == 120);
    }
    #[test]
    fn six() {
        assert!(conv_to_dec(conv_to_snafu(12345)) == 12345);
    }
}
