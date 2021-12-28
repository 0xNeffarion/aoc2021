use std::io::stdin;

pub fn parse_input() -> Vec<i32> {
    let mut result:Vec<i32> = vec![];
    loop {
        let mut buff = String::new();
        match stdin().read_line(&mut buff) {
            Ok(x) => {
                if x > 0 {
                    let value = buff.trim().parse::<i32>();
                    if value.is_err(){
                        break;
                    }

                    result.push(value.unwrap());
                }
            },
            Err(_) => {
                break;
            }
        };
    }

    result
}