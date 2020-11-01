use chrono::NaiveDateTime;
use skim::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::{self, BufReader};
use std::path::PathBuf;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use lazy_static::lazy_static;

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"\s*@@@\s*").unwrap();
}
#[derive(Debug)]
pub struct History {
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

    pub fn isodate(&self) -> String {
        let date_time = NaiveDateTime::from_timestamp(self.timestamp, 0);

        date_time.to_string()
    }
}


pub fn parse_content(example: &str) -> Option<History> {
    // let example =
    //     "89563 @@@ 1603443782 @@@ \"/Users/allee/.tmux/plugins/tmux-thumbs\" @@@ echo hello world";

    let parts: Vec<&str> = RE.split(example).collect();

    let pid: i64 = match parts[0].parse() {
        Ok(pid) => pid,
        Err(_) => return None,
    };
    let timestamp: i64 = parts[1].parse().unwrap();
    let pwd = PathBuf::from(parts[2].replace("\"", ""));
    let command = if let Some(command) = parts.get(3) {
        command.to_string()
    } else {
        return None;
    };

    let a = History::new(pid, timestamp, pwd, command);

    Some(a)
}

static ETERNAL_HISTORY_FILE: &str = ".zsh_eternal_history";

fn main() -> io::Result<()> {

    // set up config
    let config_dir_op = std::env::var_os("ETERNAL_HISTORY_FILE")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| dirs::home_dir().map(|d| d.join(ETERNAL_HISTORY_FILE)));

    let history = File::open(config_dir_op.unwrap())?;
    let history = BufReader::new(history);

    let mut my_commands: Vec<String> = Vec::new();
    for line in history.lines() {
        if let Some(parsed) = parse_content(&line?) {
            my_commands.push(parsed.command);
        }
    }

    let my_commands = my_commands.join("\n");

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(my_commands));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let a = if let Some(a) = selected_items.first() {
        a.output()
        .replace("\\n", "\n")
        .to_string()
    } else {
        return Ok(());
    };

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{}", a);
    ctx.set_contents(a.to_owned()).unwrap();

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let line = r##"89563 @@@ 1603443779 @@@ "/Users/allee/.tmux/plugins/tmux-thumbs" @@@ echo hi"##;
        let parsed = parse_content(line);
        let h = parsed.unwrap();
        assert_eq!(h.pid, 89563);
        assert_eq!(h.timestamp, 1603443779);
        assert_eq!(h.pwd, PathBuf::from("/Users/allee/.tmux/plugins/tmux-thumbs"));
        assert_eq!(h.command, "echo hi");
    } 

    #[test]
    fn multiline_history() {
        let line = r##"8508 @@@ 1604165171 @@@ "/Users/allee/src/master-rust/test_things" @@@ echo "hi
my name is
albert""##;
        let parsed = parse_content(line);
        let h = parsed.unwrap();
        assert_eq!(h.pid, 8508);
        assert_eq!(h.timestamp, 1604165171);
        assert_eq!(h.pwd, PathBuf::from("/Users/allee/src/master-rust/test_things"));
        assert_eq!(h.command, "echo \"hi\nmy name is\nalbert\"");

    }
    
    #[test]
    fn ill_defined_line() {
        let line = "89563 @@@ 1603443779 @@@ \"/Users/allee/.tmux/plugins/tmux-thumbs\"";
        let parsed = parse_content(line);
        assert!(parsed.is_none());

        let line = "randomstring";
        let parsed = parse_content(line);
        assert!(parsed.is_none());
    } 

    // #[test]
    // fn multiline_test() {

    //     let hf = File::open("multiline_test_input.txt").unwrap();
    //     let bfs = BufReader::new(hf);


    //     bfs.read_until()
    //     let a = bfs.split(b':');
    //     let stdin = std::io::stdin();
    //     // lock it
    //     let lock = stdin.lock();

        
        

    // }
}
