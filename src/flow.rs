use log::{debug, error, info, trace};
use std::collections::HashMap;
use crate::Args;
use std::process::Command;
use crate::config::Config;

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

struct Flow {
    flow_config: HashMap<String, HashMap<String, String>>,
}

impl Flow {
    fn new() {
        todo!();
    }
}

struct GitFlowCommand {
    id: i32,
    git_command: String,
    git_args: Vec<String>,
}

impl GitFlowCommand {
    fn new(issue_id: i32, git_command: &str, git_args: &str) -> GitFlowCommand {
        let git_args: Vec<String> = git_args.split(" ").map(str::to_string).collect();
        GitFlowCommand {
            id: issue_id,
            git_command: git_command.to_string(),
            git_args: git_args,
        }
    }
}

trait GitCommand {
    fn run_command_with_quit(&self);
    fn error_msg_transcate(&self, output: &str, msg: &str);
}

impl GitCommand for GitFlowCommand {
    fn run_command_with_quit(&self) {
        let complate_command: &str = &format!("{} {}", self.git_command, self.git_args.join(" "));
        if let Ok(output) = Command::new(&self.git_command)
            .args(&self.git_args)
            .output()
        {
            let excute_code = output.status.code().unwrap();
            match excute_code {
                0 => {
                    // 如果执行成功，应该打印结果, 说明是 git 输出的并且退出
                    debug!("execute git command : {} ", complate_command);
                    info!("{}", String::from_utf8_lossy(&output.stdout));
                    return;
                }
                _ => {
                    error!("execute git command : {} failed!", complate_command);
                    // self.git_args 取最后一个的值
                    let last_args = self.git_args.last().unwrap();
                    self.error_msg_transcate(&String::from_utf8_lossy(&output.stderr), last_args);
                    std::process::exit(1);
                }
            }
        }
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

pub fn parse_flow_command(args: &Args, config: &Config){
    // 先解析 flow 的配置
    let mut step_ids: Vec<i32> = config.git_flow.iter().flat_map(|hashmap| hashmap.keys()).map(|key|key.parse::<i32>().unwrap()).collect();
    step_ids.sort_unstable();
    let last_step = step_ids[step_ids.len() - 1];

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flow_pars() {
        simple_logger::SimpleLogger::new().env().init().unwrap();
        println!("{:?}", parse_flow_config());
        let flow_command = GitFlowCommand::new(15, "gh", "issue edit 27 --add-label bockon");
        flow_command.run_command_with_quit();
    }
    #[test]
    fn test_flow_config_parse() {

        use crate::config::load_config;
        let config = load_config("/home/murphy/rust/N_Commit_Tool/fixfures/ncommit.toml").unwrap();
        let mut keys: Vec<i32> = config.git_flow.iter().flat_map(|hashmap| hashmap.keys()).map(|key|key.parse::<i32>().unwrap()).collect();
        keys.sort_unstable();
        println!("{:?}", keys);
        println!("{:?}", keys[keys.len() -1]);

        for i in config.git_flow {
            println!("{:?}", i)
        }
        // let sorted_hashmap: Vec<_> = config.git_flow.iter();
    }
}
