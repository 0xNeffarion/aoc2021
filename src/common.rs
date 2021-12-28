use std::fmt;
use std::io::stdin;
use std::str::FromStr;

pub fn parse_input<T>() -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: fmt::Debug,
{
    let mut result: Vec<T> = vec![];
    loop {
        let mut buff = String::new();
        match stdin().read_line(&mut buff) {
            Ok(x) => {
                if x > 0 {
                    let value = buff.trim().parse::<T>();
                    if value.is_err() {
                        break;
                    }

                    result.push(value.unwrap());
                }
            }
            Err(_) => {
                break;
            }
        };
    }

    result
}
