use chrono::NaiveDateTime;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct History {
    pid: i64,
    username: String,
    timestamp: i64,
    command: String,
    pwd: PathBuf,
}

impl History {
    pub fn new(
        pid: i64,
        username: String,
        timestamp: i64,
        command: String,
        pwd: PathBuf,
    ) -> History {
        History {
            pid,
            username,
            timestamp,
            command,
            pwd,
        }
    }
}

fn parse_content() {
    let example = String::from(
        "2720 allee 2020-05-09 00:08:32 @ /Users/allee/src/history_search |     23  1589008111 cdd",
    );
    let parts = example.splitn(8, r"\s+");
    println!("{:?}", parts);
}

fn main() {
    let content = fs::read_to_string("toy.txt").expect("Fail to read the file");
    parse_content();

    let a = History::new(
        1234,
        String::from("allee"),
        1589008111,
        String::from("ls"),
        PathBuf::from("/Users/allee/src/history_search"),
    );
    println!("{:?}", a);

    let date_time = NaiveDateTime::from_timestamp(a.timestamp, 0);

    println!("{}", date_time);

    // timestamp conversion
}
