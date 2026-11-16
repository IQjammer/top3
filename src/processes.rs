use std::collections::HashMap;
use std::fs::File;
use std::fs::*;
use std::io;
use std::io::{BufRead, BufReader};

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

#[derive(Debug)]
pub struct Proc {
    pub name: String,
    pub pid: u32,
    pub mem: u64,
    pub uid: u32,
}

impl Proc {
    pub fn read_proc() -> Result<Vec<Proc>, io::Error> {
        let mut procs = Vec::new();
        let mut pids = Vec::new();
        for entry in read_dir(PROC)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                let pid = entry.file_name().to_string_lossy().to_string();
                let default = 99727;
                let pid = pid.parse::<u32>().unwrap_or(default);
                if pid == default {
                    continue;
                } else {
                    pids.push(pid)
                }
            }
        }

        for pid in pids {
            let file = File::open(format!("/proc/{}/status", pid))?;

            let mut line = String::new();
            let mut reader = BufReader::with_capacity(64 * 1024, file);

            let mut name: Option<String> = None;
            let mut uid: Option<u32> = None;
            let mut mem: Option<u64> = None;

            loop {
                line.clear();
                if reader.read_line(&mut line)? == 0 {
                    break;
                }
                if line.trim().is_empty() {
                    continue;
                }

                if line.starts_with("Name:") {
                    name = Some(line.strip_prefix("Name:\t").unwrap().trim().to_string());
                } else if line.starts_with("Uid:") {
                    uid = Some(
                        line.strip_prefix("Uid:\t")
                            .unwrap()
                            .split_once("\t")
                            .map(|(u, _)| u.trim())
                            .unwrap()
                            .parse::<u32>()
                            .unwrap(),
                    );
                } else if line.starts_with("VmRSS:") {
                    mem = Some(
                        line.strip_prefix("VmRSS:\t\t")
                            .unwrap()
                            .split_once(" ")
                            .map(|(m, _)| m.trim())
                            .unwrap()
                            .parse::<u64>()
                            .unwrap(),
                    )
                }

                if mem.unwrap() > 1024 {
                    let mem = mem.unwrap() / 1024;
                }
                if mem.unwrap() > 1048576 {
                    let mem = mem.unwrap() / 1024 * 1024;
                } else {
                    let mem = mem.unwrap();
                }

                let mem = mem.unwrap();
                let name = name.take().unwrap();
                let uid = uid.unwrap();

                procs.push(Proc {
                    pid,
                    name,
                    mem,
                    uid,
                });
            }
        }
        Ok(procs)
    }
}
