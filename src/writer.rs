use crate::Item;

pub fn convert_items_to_string(items: &Vec<Item>) -> String {
    let items: Vec<String> = items.iter().map(|item| item.to_string()).collect();
    items.join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Time;

    #[test]
    fn test_convert_items_to_string() {
        let items = vec![
            Item {
                pos: 1,
                start_time: Time {
                    hours: 0,
                    minutes: 0,
                    seconds: 5,
                    milliseconds: 200,
                },
                end_time: Time {
                    hours: 0,
                    minutes: 0,
                    seconds: 6,
                    milliseconds: 300,
                },
                text: String::from("test"),
            },
            Item {
                pos: 1,
                start_time: Time {
                    hours: 0,
                    minutes: 0,
                    seconds: 5,
                    milliseconds: 200,
                },
                end_time: Time {
                    hours: 0,
                    minutes: 0,
                    seconds: 6,
                    milliseconds: 300,
                },
                text: String::from("test"),
            },
        ];
        println!("{}", convert_items_to_string(&items));
        assert_eq!(
            convert_items_to_string(&items),
            "1\n00:00:05,200-->00:00:06,300\ntest\n\n1\n00:00:05,200-->00:00:06,300\ntest"
        );
    }
}
