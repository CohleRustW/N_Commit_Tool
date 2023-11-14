use crate::config::load_config;
use crate::config::Config;
use crate::config::CONFIG_PATH;
use log::{debug, error, info};
use std::collections::HashMap;
use std::process::Command;

fn parse_flow_config(
) -> HashMap<std::string::String, HashMap<std::string::String, std::string::String>> {
    let mut configs = HashMap::new();
    for i in 1..4 {
        let mut lable_config = HashMap::new();
        let lable_name = format!("{}{}", "lable", i);
        lable_config.insert("label".to_string(), lable_name);
        configs.insert(i.to_string(), lable_config);
    }
    configs
}

struct GitFlowCommand {
    id: i32,
    command: String,
    config: Config,
}

impl GitFlowCommand {
    fn new(issue_id: i32, command: String) -> GitFlowCommand {
        // let git_args: Vec<String> = git_args.split(" ").map(str::to_string).collect();
        let config = load_config(CONFIG_PATH).unwrap();

        GitFlowCommand {
            id: issue_id,
            command: command,
            config: config,
        }
    }
}

trait GitCommand {
    fn error_msg_transcate(&self, output: &str, msg: &str);
    fn run_git_command_with_string(
        &self,
        command: &str,
    ) -> Result<std::process::Output, std::io::Error> {
        let commands = command.split(" ").collect::<Vec<&str>>();
        let git_command = commands[0];
        let git_args = &commands[1..];
        Command::new(git_command).args(git_args).output()
    }
    fn run_command_and_check_code(&self, command: &str) -> Vec<u8> {
        if let Ok(output) = self.run_git_command_with_string(command) {
            let code = output.status.code().unwrap();
            if code != 0 {
                let last_args = command.split(" ").last().unwrap();
                self.error_msg_transcate(&String::from_utf8_lossy(&output.stderr), last_args);
                std::process::exit(code);
            } else {
                debug!("{}", String::from_utf8_lossy(&output.stdout));
                output.stdout
            }
        } else {
            error!("execute git command : {} failed!", command);
            std::process::exit(1);
        }
    }
    fn get_labels(&self) -> Vec<String>;
    fn rebase_to_target_label(&self, target_label: &str);
}

impl GitCommand for GitFlowCommand {
    fn get_labels(&self) -> Vec<String> {
        let command = format!("gh issue view {} --json labels", self.id);
        let output = self.run_command_and_check_code(command.as_str());
        // 把 output 转换成 json
        let json = String::from_utf8_lossy(&output);
        let json: serde_json::Value = serde_json::from_str(&json).unwrap();
        let labels = json["labels"].as_array().unwrap();
        let labels: Vec<String> = labels
            .iter()
            .map(|label| label["name"].as_str().unwrap().to_string())
            .collect();
        labels
    }

    fn rebase_to_target_label(&self, target_label: &str) {
        let current_labels = self.get_labels();
        // 判断当前的 target_label 是否包括在 current_labels
        if current_labels.contains(&target_label.to_string()) {
            info!("当前分支已经包含了 {} 标签", target_label);
            std::process::exit(0);
        }
        use inquire::Select;
        let mut choice_labels = Vec::new();
        let mut keys: Vec<i32> = self
            .config
            .git_flow
            .iter()
            .flat_map(|hashmap| hashmap.keys())
            .map(|key| key.parse::<i32>().unwrap())
            .collect();
        keys.sort_unstable();

        for key in keys {
            for iter in self.config.git_flow.iter() {
                if iter.contains_key(&key.to_string()) {
                    let label = iter.get(&key.to_string()).unwrap();
                    choice_labels.push(label);
                }
            }
        }
        let label_msg = format!("选择你想要切换到的标签, 当前 issue id: {}", self.id);
        if let Ok(choice) = Select::new(&label_msg, choice_labels.clone()).prompt() {
            info!("选择了 {}", choice);
            self.run_command_and_check_code(
                format!("gh issue edit {} --add-label {}", self.id, choice).as_str(),
            );
            // 删除其他的标签，保留当前的标签
            let mut to_be_delete_labels = current_labels
                .iter()
                .filter(|label| choice_labels.contains(label))
                .collect::<Vec<&String>>();
            to_be_delete_labels.retain(|label| label != &choice);
            debug!("需要删除的标签: {:?}", to_be_delete_labels);
            for delete_label in to_be_delete_labels {
                let remove_command =
                    format!("gh issue edit {} --remove-label {}", self.id, delete_label);
                let output = self.run_command_and_check_code(&remove_command);
                info!(
                    "删除标签 {} \n {:?}",
                    delete_label,
                    String::from_utf8_lossy(&output)
                );
            }
        } else {
            red!("no choice label\n");
            std::process::exit(1);
        }
        std::process::exit(0);
    }
    fn error_msg_transcate(&self, output: &str, msg: &str) {
        use regex::Regex;
        let label_nout_found = Regex::new(r"failed to update.* not found").unwrap();
        if label_nout_found.is_match(output) {
            let log = format!("不存在的标签: {}", msg.to_string());
            info!("{}", log);
        } else {
            error!("{}", output)
        }
    }
}

pub fn parse_flow_command(id: i32, command: String) {
    let flow_command = GitFlowCommand::new(id, command);
    flow_command.rebase_to_target_label("lable1");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flow_pars() {
        simple_logger::SimpleLogger::new().env().init().unwrap();
        println!("{:?}", parse_flow_config());
        let flow_command = GitFlowCommand::new(27, "test".to_string());
        let labels = flow_command.get_labels();
        println!("{:?}", labels);
    }
    #[test]
    fn test_flow_config_parse() {
        use crate::config::load_config;
        let config = load_config("/home/murphy/rust/N_Commit_Tool/fixfures/ncommit.toml").unwrap();
        let mut keys: Vec<i32> = config
            .git_flow
            .iter()
            .flat_map(|hashmap| hashmap.keys())
            .map(|key| key.parse::<i32>().unwrap())
            .collect();
        keys.sort_unstable();
        println!("{:?}", keys);
        println!("{:?}", keys[keys.len() - 1]);

        for i in config.git_flow {
            println!("{:?}", i)
        }
        // let sorted_hashmap: Vec<_> = config.git_flow.iter();
    }
    #[test]
    fn test_branch_re() {
        use regex::Regex;
        use crate::config;
        let config = config::load_config(config::CONFIG_PATH).unwrap();
        use crate::parse_branch_issue_id;
        let re = Regex::new(r".*issue#?(\d+).*").unwrap();
        let branch_name = "_V2.3.X/dev_issue#27".to_string();
        if re.is_match(&branch_name) {
            let caps = re.captures(&branch_name).unwrap();
            println!("{:?}", caps.get(1).unwrap().as_str());
        }else {
            println!("no match");
            std::process::exit(1);
        }
        let id = parse_branch_issue_id(&config);
        println!("{:?}", id);
    }
}
