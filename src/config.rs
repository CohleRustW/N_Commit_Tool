use serde::{Serialize, Deserialize};
use anyhow::Result;
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
}


pub fn load_config () -> Result<Config, ()>{
    let yaml_text = include_str!("nconfig.yml");
    let config: Config = match serde_yaml::from_str(&yaml_text) {
        Ok(c) => c,
        Err(e) => {
            red!("parse config nconfig.yml failed: {}\n", e);
            return Err(());
        }
    };
    Ok(config)
}