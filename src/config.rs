use std::path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub dir: path::PathBuf,
    pub db_filename: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dir: ".".into(),
            db_filename: "store.rdb".into(),
        }
    }
}

impl From<&[String]> for Config {
    fn from(value: &[String]) -> Self {
        let mut cfg = Self::default();
        let mut args = value.iter();

        while let Some(cmd) = args.next() {
            if let Some(val) = args.next() {
                match cmd.as_str() {
                    "--dir" => cfg.dir = PathBuf::from(val),
                    "--dbfilename" => cfg.db_filename = val.to_string(),
                    cmd => println!("Unknown command: {cmd}"),
                }
            }
        }

        cfg
    }
}
