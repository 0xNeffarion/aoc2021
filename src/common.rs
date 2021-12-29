use std::fmt;
use std::io::stdin;
use std::str::FromStr;

pub fn raw_input() -> Vec<String> {
    let mut result: Vec<String> = vec![];
    loop {
        let mut buff = String::new();
        match stdin().read_line(&mut buff) {
            Ok(x) => {
                if x > 0 {
                    let value = buff.trim();

                    if value.is_empty() {
                        break;
                    }

                    result.push(value.to_string());
                }
            }
            Err(_) => {
                break;
            }
        };
    }

    result
}

pub fn parse_input<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    raw_input()
        .iter()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
