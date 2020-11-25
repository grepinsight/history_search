use chrono::Utc;
use itertools::Itertools;
use skim::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::{self, BufReader};
use std::net::TcpStream;
use std::path::PathBuf;
use std::time;
use std::time::SystemTime;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use lazy_static::lazy_static;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Renaming Tool")]
struct Options {
    #[structopt(short, long)]
    here: bool,

    #[structopt(short, long)]
    today: bool,

    #[structopt(short, long)]
    yesterday: bool,

    #[structopt(short, long)]
    begin: Option<String>,

    #[structopt(short, long)]
    end: Option<String>,

    /// selected commands will be sent to this address
    #[structopt(short, long)]
    socket_addr: Option<String>,
}

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"\s*@@@\s*").unwrap();
}

static ZSH_ETERNAL_HISTORY_FILE: &str = ".zsh_eternal_history";
static BASH_ETERNAL_HISTORY_FILE: &str = ".bash_eternal_history";

#[derive(Debug)]
pub struct History {
    pid: i64,
    timestamp: SystemTime,
    pwd: PathBuf,
    command: String,
}

impl History {
    pub fn new(pid: i64, timestamp: SystemTime, pwd: PathBuf, command: String) -> History {
        History {
            pid,
            timestamp,
            pwd,
            command,
        }
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
    let timestamp_ = parts[1].parse().unwrap();
    let timestamp = time::UNIX_EPOCH + time::Duration::from_secs(timestamp_);

    let pwd = PathBuf::from(parts[2].replace("\"", ""));
    let command = if let Some(command) = parts.get(3) {
        command.to_string()
    } else {
        return None;
    };

    let a = History::new(pid, timestamp, pwd, command);

    Some(a)
}

fn get_commands<I>(lines: I, options: &Options, here_directory: PathBuf) -> Vec<String>
where
    I: Iterator<Item = Result<String, std::io::Error>>,
{
    let mut my_commands: Vec<String> = Vec::new();
    for line in lines {
        let mut include = true;
        if let Some(parsed) = parse_content(&line.unwrap()) {
            // here filter
            if options.here {
                let a = parsed.pwd;
                if a != here_directory {
                    include &= false;
                }
            }

            //  time filter
            if options.yesterday {
                let n = time::SystemTime::now();
                let d = time::Duration::from_secs(86400);
                let y = n - d;

                if parsed.timestamp < y {
                    include &= false;
                }
            }

            if options.today {
                let t = floor_date(time::SystemTime::now());

                if parsed.timestamp < t {
                    include &= false;
                }
            }

            if include {
                my_commands.push(parsed.command);
            }
        }
    }
    my_commands.into_iter().unique().collect()
}

fn floor_date(t: SystemTime) -> SystemTime {
    chrono::prelude::DateTime::<Utc>::from(t)
        .date()
        .and_hms(0, 0, 0)
        .into()
}

fn main() -> io::Result<()> {
    let opts = Options::from_args();

    // check wehther current shell is zsh or bash
    let eternal_history_file = match std::env::var_os("ZSH_VERSION") {
        Some(_) => ZSH_ETERNAL_HISTORY_FILE,
        None => BASH_ETERNAL_HISTORY_FILE,
    };

    let config_dir_op = std::env::var_os("ETERNAL_HISTORY_FILE")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| dirs::home_dir().map(|d| d.join(eternal_history_file)));

    // get commands from history
    let history = File::open(config_dir_op.unwrap())?;
    let history = BufReader::new(history);
    let here_directory = std::env::current_dir().unwrap();

    let my_commands = get_commands(history.lines(), &opts, here_directory).join("\n");

    // fuzzy finding step
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
        a.output().replace("\\n", "\n").to_string()
    } else {
        return Ok(());
    };

    // clipboard step
    println!("{}", a);

    if let Some(socket_addr) = opts.socket_addr {
        let mut stream = TcpStream::connect(socket_addr).unwrap();
        stream.write(a.as_bytes()).unwrap();
    } else {
        let ctx = ClipboardProvider::new();

        if let Ok(ctxr) = ctx {
            // FIXME wierd hack to type annotate
            let mut xx: ClipboardContext = ctxr;
            xx.set_contents(a.to_owned()).unwrap();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let line =
            r##"89563 @@@ 1603443779 @@@ "/Users/allee/.tmux/plugins/tmux-thumbs" @@@ echo hi"##;
        let parsed = parse_content(line);
        let h = parsed.unwrap();
        assert_eq!(h.pid, 89563);
        assert_eq!(
            h.timestamp,
            time::UNIX_EPOCH + time::Duration::from_secs(1603443779)
        );
        assert_eq!(
            h.pwd,
            PathBuf::from("/Users/allee/.tmux/plugins/tmux-thumbs")
        );
        assert_eq!(h.command, "echo hi");
    }
    #[test]
    fn test_get_commands_here() {
        let cmd1 = r##"89563 @@@ 1603443779 @@@ "/my/path/1" @@@ echo path1"##;
        let cmd2 = r##"89563 @@@ 1603443779 @@@ "/my/path/2" @@@ echo path2"##;
        let commands = vec![Ok(String::from(cmd1)), Ok(String::from(cmd2))];
        let options = Options {
            here: true,
            today: false,
            yesterday: false,
            begin: None,
            end: None,
            socket_addr: None,
        };
        let here_directory = PathBuf::from("/my/path/1");

        let cmds = get_commands(commands.into_iter(), &options, here_directory);
        assert_eq!(cmds, vec![String::from("echo path1")])
    }

    #[test]
    fn multiline_history() {
        let line = r##"8508 @@@ 1604165171 @@@ "/Users/allee/src/master-rust/test_things" @@@ echo "hi
my name is
albert""##;
        let parsed = parse_content(line);
        let h = parsed.unwrap();
        assert_eq!(h.pid, 8508);
        assert_eq!(
            h.timestamp,
            time::UNIX_EPOCH + time::Duration::from_secs(1604165171)
        );
        assert_eq!(
            h.pwd,
            PathBuf::from("/Users/allee/src/master-rust/test_things")
        );
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

    #[test]
    fn test_day_filter() {
        // today
        let t = time::UNIX_EPOCH + time::Duration::from_secs(1604424029);

        // yesterday
        let t1d = humantime::parse_duration("1day").unwrap();
        let y = t - t1d;

        let dt = chrono::prelude::DateTime::<Utc>::from(t);
        let dy = chrono::prelude::DateTime::<Utc>::from(y);

        println!("{}", dt.format("%Y-%m-%d").to_string());
        println!("{}", dy.format("%Y-%m-%d").to_string());
        println!("{}", t > y);

        let dt = chrono::prelude::DateTime::<Utc>::from(t);
        let dt = dt.date().and_hms(0, 0, 0);
        println!("{:?}", dt);
        println!("{:?}", floor_date(t));
    }

    #[test]
    fn test_remove_duplicate_cmds() {
        let cmd1 = r##"89563 @@@ 1603443779 @@@ "/my/path/1" @@@ echo path1"##;
        let cmd2 = r##"89563 @@@ 1603443779 @@@ "/my/path/1" @@@ echo path1"##;
        let cmd3 = r##"89563 @@@ 1603443779 @@@ "/my/path/1" @@@ echo path2"##;
        let commands = vec![
            Ok(String::from(cmd1)),
            Ok(String::from(cmd2)),
            Ok(String::from(cmd3)),
        ];
        let options = Options {
            here: false,
            today: false,
            yesterday: false,
            begin: None,
            end: None,
            socket_addr: None,
        };
        let here_directory = PathBuf::from("/my/path/1");

        let cmds = get_commands(commands.into_iter(), &options, here_directory);
        assert_eq!(
            cmds,
            vec![String::from("echo path1"), String::from("echo path2"),]
        )
    }

    #[test]
    fn test_tcp_stuff() {
        let mut stream = TcpStream::connect("127.0.0.1:2224").unwrap();
        let lenz = stream.write(b"hello world").unwrap();
        assert_eq!(lenz, 11)
    }
}
