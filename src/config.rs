use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::mem;
use std::process::Command;
use std::{fs, str};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub remote_name: String,
    pub dev_issue_re: String,
    pub version_compare_re: String,
    pub enable_auto_fetch: bool,
    pub issue_title_filter_re: String,
    pub dev_issue_name_header: String,
    pub commit_append_nodeman_msg: bool,
    pub commit_append_msg: String,
    pub commit_link_description: String,
    pub remote_branch_name_template: String,
    pub commit_custom_params: String,
}

#[cfg(target_os = "windows")]
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let toml_text: String = fs::read_to_string("C:\\etc\\ncommit.toml")?;

    let config_map: HashMap<String, Config> = match toml::from_str(&toml_text) {
        Ok(config_map) => config_map,
        Err(e) => {
            red!("parse config ncommit.toml failed: {}\n", e);
            std::process::exit(1);
        }
    };
    let mut result_config: Option<&Config> = None;
    for (project_path, project_config) in config_map.iter() {
        let current_path: String = get_current_path()?;
        if project_path.to_string() == current_path.to_string() {
            result_config = Some(project_config);
        } else {
            result_config = None;
        };
    }
    if mem::size_of::<Config>() == 0 {
        red!("no match project path config found in ncommit.toml\n");
        std::process::exit(1);
    }
    match result_config {
        Some(config) => {}
        None => {
            red!(
                "no match project path config found in nconfig.toml, current project {}\n",
                get_current_path()?
            );
            std::process::exit(1);
        }
    }

    Ok(result_config.unwrap().clone())
}

#[cfg(not(target_os = "windows"))]
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let toml_text: String = fs::read_to_string("/etc/ncommit.toml")?;

    let config_map: HashMap<String, Config> = match toml::from_str(&toml_text) {
        Ok(config_map) => config_map,
        Err(e) => {
            red!("parse config ncommit.toml failed: {}\n", e);
            std::process::exit(1);
        }
    };
    let mut result_config: Option<&Config> = None;
    let mut project_paths: Vec<String> = Vec::new();
    for (project_path, project_config) in config_map.iter() {
        project_paths.push(project_path.to_string());
        let current_path: String = get_current_path()?;
        let re_str = format!(".*{}.*", project_path.to_string());
        let project_path_re: Regex = Regex::new(&re_str).unwrap();
        if project_path_re.is_match(&current_path) {
            result_config = Some(project_config);
        } else {
            result_config = None;
        };
    }
    match result_config {
        Some(config) => {}
        None => {
            red!("no match project path config found in ncommit.toml, current project {}\n, projects -> {:#?}\n", get_current_path()?, project_paths);
            std::process::exit(1);
        }
    }

    Ok(result_config.unwrap().clone())
}

pub fn get_current_path() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()
        .expect("failed to execute git");
    if !output.status.success() {
        red!("git rev-parse --show-toplevel failed\n");
        std::process::exit(1);
    }
    Ok(str::from_utf8(&output.stdout).unwrap().trim().to_string())
}
