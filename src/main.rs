use chrono::NaiveDateTime;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct History {
    pid: i64,
    timestamp: i64,
    pwd: PathBuf,
    command: String,
}

impl History {
    pub fn new(pid: i64, timestamp: i64, pwd: PathBuf, command: String) -> History {
        History {
            pid,
            timestamp,
            pwd,
            command,
        }
    }
}

// 89563 @@@ 1603443779 @@@ "/Users/allee/.tmux/plugins/tmux-thumbs" @@@ echo hi
// 89563 @@@ 1603443782 @@@ "/Users/allee/.tmux/plugins/tmux-thumbs" @@@ echo hello world

fn parse_content() {
    let example =
        "89563 @@@ 1603443782 @@@ \"/Users/allee/.tmux/plugins/tmux-thumbs\" @@@ echo hello world";
    let re = regex::Regex::new(r"\s*@@@\s*").unwrap();
    let parts: Vec<&str> = re.split(example).collect();
    println!("{:?}", parts);
}

fn main() {
    let content = fs::read_to_string("toy.txt").expect("Fail to read the file");
    parse_content();

    // let a = History::new(
    //     1234,
    //     String::from("allee"),
    //     1589008111,
    //     String::from("ls"),
    //     PathBuf::from("/Users/allee/src/history_search"),
    // );
    // println!("{:?}", a);

    // let date_time = NaiveDateTime::from_timestamp(a.timestamp, 0);

    // println!("{}", date_time);

    // timestamp conversion
}
