use std::{error::Error, fs, env, process::Command};
use regex;

pub fn find_outer_config() -> Result<String, Box<dyn Error>> {
    let mut outer_path = "../".to_string();
    'outer: loop {
        let paths = fs::read_dir(&outer_path)?;
        for path in paths {
            let cur_file = path?.path();
            // convert to absolute path and check if dir == home
            if cur_file.parent().ok_or("No parent")?.canonicalize()?.to_str().unwrap() == env::var("HOME")? {
                Err("global config find only")?
            };
            if cur_file.is_file() && cur_file.file_name().unwrap_or("".as_ref()) == ".gitconfig" {
                break 'outer Ok(fs::read_to_string(&cur_file)?)
            };
        };
        outer_path.push_str("../");
    }
}

fn find_inner_config() -> Result<String, std::io::Error> {
    fs::read_to_string(".git/config")
}

pub fn apply_config (config: String) -> Result<(), Box<dyn Error>> {
    let local_config = find_inner_config().unwrap_or("".to_string());
    if local_config == "".to_string() {
        let need_to_apply = ["user.name", "user.email"];
        for field in need_to_apply {
            let prefix_key: Vec<&str> = field.split(".").collect();
            let old_val = get_gitconfig_value(&local_config, prefix_key[0], prefix_key[1]);
            let new_val = get_gitconfig_value(&config, prefix_key[0], prefix_key[1]);

            if old_val != new_val {
                exec_gitconfig_config(field, new_val)?
            }
        }
    }
    Ok(())
}

fn get_gitconfig_value<'a>(config: &'a str, prefix: &'a str, key: &'a str) -> &'a str {
    let mut cur_prefix = "";
    let prefix_regexp = regex::Regex::new(r"\[.*]").unwrap();

    for line in config.lines() {
        if prefix_regexp.is_match(line) {
            cur_prefix = prefix_regexp.find(line).unwrap().as_str();
        }
        if cur_prefix == format!("[{}]", prefix) && line.contains(key) {
            let key_val: Vec<&'a str> = line.split(" = ").collect();
            return key_val[1];
        }
    };
    ""
}

fn exec_gitconfig_config(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    Command::new("git").args(["config", key, value]).output()?;
    Ok(())
}

pub fn exec_git(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    Command::new("git").args(&args[1..]).spawn()?;
    Ok(())
}