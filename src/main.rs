
#[inline(never)]
fn do_something_with<T>(_data: T) {}

fn getopt_like<T: Iterator<Item = String>>(mut args: T) -> Vec<String> {
    let mut remaining_args: Vec<String> = Vec::new();
    while let Some(arg) = args.next() {
        if arg == "--" {
            remaining_args.extend(args);
            break;
        }
        let mut arg_chars = arg.chars();
        if arg.as_bytes()[0] != b'-' {
            remaining_args.push(arg_chars.collect());
            remaining_args.extend(args);
            break;
        }
        let _ = arg_chars.next();
        while let Some(c) = arg_chars.next() {
            match c {
                'a' => do_something_with("A"),
                'e' => {
                    let mut remainder: String = arg_chars.collect();
                    if remainder.is_empty() {
                        if let Some(next_arg) = args.next() {
                            remainder.push_str(&next_arg);
                        }
                    }
                    remaining_args.push(remainder);
                    break;
                }
                c => do_something_with(c),
            }
        }
    }
    remaining_args
}

#[cfg(not(tarpaulin))]
fn main() {
    use std::env::args;
    do_something_with(getopt_like(args().skip(1)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {
        assert_eq!(
            getopt_like(
                vec![String::from("-a"), String::from("foo"), String::from("bar"),].into_iter()
            ),
            vec![String::from("foo"), String::from("bar")]
        );
        assert_eq!(
            getopt_like(
                vec![
                    String::from("-eo"),
                    String::from("-q"),
                    String::from("-e"),
                    String::from("foo"),
                    String::from("-a"),
                    String::from("--"),
                    String::from("-e"),
                    String::from("-a"),
                    String::from("-e"),
                ]
                .into_iter()
            ),
            vec![
                String::from("o"),
                String::from("foo"),
                String::from("-e"),
                String::from("-a"),
                String::from("-e")
            ]
        );
    }
}
