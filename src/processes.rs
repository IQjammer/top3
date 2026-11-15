use std::collections::HashMap;
use std::fs::*;
use std::io;
use std::num::ParseIntError;

fn load_users() -> HashMap<u32, String> {
    let mut map = HashMap::new();
    if let Ok(s) = read_to_string("/etc/passwd") {
        for line in s.lines() {
            let mut it = line.split(':');
            let name = it.next();
            let uid = it.nth(1).and_then(|u| u.parse::<u32>().ok());
            if let (Some(name), Some(uid)) = (name, uid) {
                map.entry(uid).or_insert_with(|| name.to_string());
            }
        }
    }
    map
}

const PROC: &str = "/proc/";

pub struct Proc {
    pub name: String,
    pub pid: u32,
    pub mem: Option<u64>,
    pub uid: Option<u32>,
}

impl Proc {
    pub fn read_proc() -> Result<(), io::Error> {
        for entry in read_dir(PROC)? {
            let entry = entry.unwrap();
            if !entry.file_type()?.is_file() {
                let name = entry.file_name().to_string_lossy().to_string();
                let Ok(num) = name.parse::<u32>() else {
                    continue;
                };

                for i_entry in read_dir(entry.path())? {
                    let i_entry = i_entry?;
                    if i_entry.file_name().to_string_lossy().to_string() == String::from("status") {
                        let file = read_to_string(i_entry.path());
                    };
                }
            };
        }

        Ok(())
    }
}
