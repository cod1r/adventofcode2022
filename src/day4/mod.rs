pub fn day4() {
    let bytes = include_bytes!("input.txt");
    let answers = bytes[0..bytes.len() - 1]
        .split(|b| *b == b'\n')
        .fold((0, 0), |acc, elem| {
            let mut parts = elem
                .split(|byte| *byte == b',' || *byte == b'-')
                .map(|ee| {
                    ee.iter()
                        .fold(String::new(), |acc, e| acc + &(*e as char).to_string())
                })
                .map(|s| s.parse::<u8>().expect("a number"));
            let one = parts.next().expect("some parts are not correct");
            let two = parts.next().expect("some parts are not correct");
            let three = parts.next().expect("some parts are not correct");
            let four = parts.next().expect("some parts are not correct");
            let mut first_part = acc.0;
            let mut second_part = acc.1;
            if one <= four && three <= two {
                second_part += 1;
            }
            if (one >= three && two <= four) || (three >= one && four <= two) {
                first_part += 1;
            }
            (first_part, second_part)
        });
    println!("day4");
    println!("{}\n{}", answers.0, answers.1);
}
