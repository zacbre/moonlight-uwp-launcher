use std::fmt::{Display, Formatter};
use std::process::{Command, exit};
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};
use winparsingtools::structs::shell_items::ShellItemTypes;

#[cfg(debug_assertions)]
pub fn get_args() -> Vec<String> {
    vec![String::from(""), String::from("C:\\Users\\zac\\Desktop\\Age of Empires IV.lnk")]
}

#[cfg(not(debug_assertions))]
pub fn get_args() -> Vec<String> {
    std::env::args().collect()
}

fn monitor() {
    let target = std::env::current_exe().unwrap();
    let target = target.with_extension("");
    let target = target.file_name();
    let target = target.unwrap();
    let target = target.to_str().unwrap();

    let keyword: Vec<&str> = target.split("_").collect();
    let path = &keyword[0].to_string(); //

    println!("Child!: {} {}", target, path);

    locate_and_wait_for_process(target.to_string(), "SOMETHING_THAT_WILL_NEVER_CONTAIN".to_string(), ChildOrParent::Child, false);
    println!("Child waited for process...");
    locate_and_kill_process(path.to_string(), target.to_string());
    println!("Child killed process...");
    exit(0);
}

pub fn launch() {
    // allow someone to drag a shortcut onto this and it will generate one with the apps name and all.
    let args = get_args();
    if args.len() == 2 {
        if args[1].starts_with("--monitor") {
            monitor();
        } else {
            // create shortcut
            parse_lnk(&args[1]);
        }
    } else if args.len() == 1 {
        let target = std::env::current_exe().unwrap();
        let target = target.with_extension("");
        let target = target.file_name();
        let target = target.unwrap();
        let target = target.to_str().unwrap();

        if !target.contains("!") {
            return;
        }

        let mut str = String::new();
        str.push_str("shell:appsFolder\\");
        str.push_str(target);

        // launch
        let _ = Command::new("explorer.exe")
            .args([str])
            .spawn()
            .unwrap()
            .wait()
            .expect("Child stopped running!");

        let _ = Command::new(std::env::current_exe().unwrap())
            .args(["--monitor"])
            .spawn()
            .unwrap();

        // check for process running with roughly the same path.
        let keyword: Vec<&str> = target.split("_").collect();
        let path = &keyword[0].to_string();
        locate_and_wait_for_process(path.to_string(), target.to_string(), ChildOrParent::Parent, true);
    }
    //Get-AppxPackage
}

fn locate_and_kill_process(path: String, target: String) {
    let mut processes = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));
    {
        processes.refresh_processes();
    }
    for (_, process) in processes.processes() {
        let process_cmd = process.cmd();
        for item in process_cmd {
            if item.contains(&path) && !item.contains(&target) && process.pid().as_u32() != std::process::id() {
                process.kill();
            }
        }
    }
}

#[derive(Debug)]
enum ChildOrParent {
    Child,
    Parent
}

impl Display for ChildOrParent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChildOrParent::Child => write!(f, "Child"),
            ChildOrParent::Parent => write!(f, "Parent")
        }
    }
}

fn locate_and_wait_for_process(path: String, target: String, child_or_parent: ChildOrParent, wait_for_all: bool) {
    let mut processes = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));
    for _ in 0..10 {
        {
            processes.refresh_processes();
        }
        for (_, process) in processes.processes() {
            let process_cmd = process.cmd();
            for item in process_cmd {
                // make sure it's not our current process?
                if item.contains(&path) && !item.contains(&target) && process.pid().as_u32() != std::process::id() {
                    println!("[{}] Waiting for PID {} - {:?}", child_or_parent, process.pid().as_u32(), process_cmd);
                    process.wait();
                    if !wait_for_all {
                        return;
                    }
                }
            }
        }
        sleep(Duration::from_secs(1));
    }
}

fn skip_sections(x: &Vec<u8>, i: &mut usize) -> Option<String> {
    loop {
        let next_section_length = x[*i] as usize;
        if next_section_length <= 17 {
            *i += next_section_length;
        } else {
            let cloned_i = i.clone();
            let str = extract_string(x, cloned_i, next_section_length);
            if str.contains("!") {
                return Some(str);
            }
            *i += next_section_length;
        }
        if *i >= x.len() {
            break;
        }
    }
    return None;
}

fn parse_lnk(link: &String) {
    let lnkres = lnk_parser::LNKParser::from_path(link.as_str());
    if let Ok(lnk) = lnkres {
        let lnk_target_list = &lnk.get_link_target_id_list().as_ref().clone().unwrap().id_list;
        for item in lnk_target_list.items() {
            match &item.shell_item_data {
                Some(ShellItemTypes::Unimplemented(u)) => {
                    let x: Vec<u8> = u.0.clone();
                    let mut j = 0;
                    loop {
                        match x[j..j+5] {
                            [0x31, 0x53, 0x50, 0x53, 0x55] => {
                                break;
                            },
                            _ => {}
                        }
                        j += 1;
                    }
                    let mut i = j + 20;
                    let potential_str = skip_sections(&x, &mut i);
                    if potential_str.is_some() {
                        create_shortcut(link, potential_str.unwrap());
                    } else {
                        //println!("Cannot locate app title!");
                    }
                }
                _ => {}
            }
        }
    }
}

fn extract_string(x: &Vec<u8>, begin_x: usize, data_section_length: usize) -> String {
    let mut str = String::new();
    let start_section = begin_x + 17;
    let output = std::str::from_utf8(&x[start_section..start_section + data_section_length - 17]);
    match output {
        Ok(o) => str.push_str(o),
        Err(_) => {}//println!("{:?}", e)
    };
    str.replace(char::from(0), "")
}

fn create_shortcut(lnk_path: &String, mut target_app: String) {
    let target = std::env::current_exe().unwrap();
    let target = target.into_os_string();
    let target = target.to_str().unwrap();
    let modified_lnk_path = std::path::Path::new(lnk_path.as_str());

    target_app.push_str(".exe");

    let file_name = modified_lnk_path.with_file_name(target_app);
    let file_name = file_name.to_str().unwrap();
    copy_file(target, file_name);
}

#[cfg(not(debug_assertions))]
fn copy_file(src: &str, dest: &str) {
    let _ = std::fs::copy(src, dest);
}

#[cfg(debug_assertions)]
fn copy_file(src: &str, dest: &str) {
    println!("{} -> {}", src, dest);    
}