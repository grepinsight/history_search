use chrono::NaiveDateTime;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct History {
    username: String,
    timestamp: i64,
    command: String,
    pwd: PathBuf,
}

impl History {
    pub fn new(username: String, timestamp: i64, command: String, pwd: PathBuf) -> History {
        History {
            username,
            timestamp,
            command,
            pwd,
        }
    }
}
fn main() {
    let content = fs::read_to_string("toy.txt").expect("Fail to read the file");

    let a = History::new(
        String::from("allee"),
        1589008111,
        String::from("ls"),
        PathBuf::from("/Users/allee/src/history_search"),
    );
    println!("{:?}", a);
}
