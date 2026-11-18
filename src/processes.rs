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

pub struct Proc {
    pub name: String,
    pub pid: u32,
    pub mem: (u64, String),
    pub user: String,
}

impl Proc {
    pub fn convert(procs: Vec<Proc>) -> Vec<Proc> {
        procs
            .into_iter()
            .map(|mut p| {
                if p.mem.0 > 1024 * 1024 {
                    p.mem.0 /= 1024 * 1024;
                    p.mem.1 = "gB".to_string();
                } else if p.mem.0 > 1024 {
                    p.mem.0 /= 1024;
                    p.mem.1 = "mB".to_string();
                }
                p
            })
            .collect()
    }

    pub fn read_proc() -> Result<Vec<Proc>, io::Error> {
        let mut procs = Vec::new();
        for entry in read_dir("/proc/")? {
            let entry = entry?;
            let pid = entry.file_name().to_string_lossy().to_string();
            let Ok(pid) = pid.parse::<u32>() else {
                continue;
            };
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
                        line.strip_prefix("VmRSS:\t")
                            .unwrap()
                            .trim()
                            .to_string()
                            .strip_suffix(" kB")
                            .unwrap()
                            .parse::<u64>()
                            .unwrap(),
                    )
                } else {
                    continue;
                }
            }

            let mem = (mem.unwrap_or_default(), "kB".to_string());
            let name = name.take().unwrap();
            let uid = uid.unwrap();
            let user = load_users()
                .get(&uid)
                .cloned()
                .unwrap_or_else(|| uid.to_string());

            procs.push(Proc {
                pid,
                name,
                mem,
                user,
            });
        }
        procs.sort_unstable_by_key(|p| std::cmp::Reverse(p.mem.0));
        procs = Proc::convert(procs);
        Ok(procs)
    }
}
