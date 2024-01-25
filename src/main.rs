use clap::Parser;
use handlebars::Handlebars;
use inquire::Select;
use inquire::Text;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Result;
use std::process::Command;
use std::result::Result as CResult;
use std::str;
use version_compare::Version;
mod config;
mod flow;
mod pr;
// mod tests;
mod view;
use anyhow::Result as Aresult;
use log::{debug, LevelFilter, error};

#[macro_use]
extern crate colour;

#[derive(Parser, Debug, Clone)]
#[clap(author, about, long_about = None)]
pub struct Args {
    /// custom commit message
    #[clap(short = 'm', long, required = false, default_value = "false")]
    message: String,

    /// just print command without exec
    #[clap(
        short = 'p',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    print: String,

    /// 通过浏览器当前匹配到的 issue id 打开浏览器的对应页面，支持 issue/pr, example: ncommit -w pr/issue
    #[clap(
        short = 'w',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    web: String,

    #[clap(
        short = 'n',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    new_branch: String,

    #[clap(
        short = 'f',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    flow: String,

    #[clap(
        short = 'c',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    chooise: String,

    #[clap(
        short = 'd',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    debug: String,

    #[clap(
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    pr: String,

    #[clap(
        short = 'f',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    force: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    number: usize,
    title: String,
}

fn parse_branch_issue_id(config: &config::Config) -> String {
    if let Ok(branch_name) = get_branch() {
        let branch = String::from_utf8_lossy(&branch_name);
        let branch_id_re: Regex = Regex::new(&config.dev_issue_re).unwrap();
        if branch_id_re.is_match(&branch) {
            let issue_id = branch_id_re
                .captures(&branch)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            issue_id
        } else {
            let msg = format!("通过 branch name -> {} 没有匹配到 issue ID", branch);
            error!("{}", msg);
            std::process::exit(1);
        }
    } else {
        error!("没搞到分支名称");
        std::process::exit(1);
    }
}

fn get_branch() -> CResult<Vec<u8>, Box<std::io::Error>> {
    let branch = Command::new("git")
        .args(["branch", "--show-current"])
        .output()?;
    Ok(branch.stdout)
}

fn custom_commit_params(custom_args: &str) -> String {
    let base_params = String::from("-m");
    let mut is_add_custom_args: bool = true;
    if !custom_args.starts_with("-") {
        is_add_custom_args = false;
    }
    if custom_args.len() == 0 {
        is_add_custom_args = false;
    }

    if is_add_custom_args {
        // if to_vec {
        //     let mut result = vec![base_params];
        //     for c in custom_args.split_whitespace() {
        //         result.push(c.to_string());
        //     }
        //     return result
        // }
        return format!("{} {}", custom_args, base_params);
    } else {
        return base_params;
    }
}

fn biggerst_version_number(version_list: Vec<String>) -> (String, usize) {
    debug!("all version list -> {:#?}", version_list);
    if version_list.len() == 0 {
        error!("没有匹配到的版本号\n");
        std::process::exit(1);
    };
    let mut version_numer: String = String::new();
    let numer_version_list = version_list.clone();
    for version in version_list {
        let a = Version::from(&version).unwrap();
        let b = Version::from(&version_numer).unwrap();
        if a > b {
            version_numer = version.to_string();
        }
    }
    debug!("biggerst version number -> {}", version_numer);
    debug!("biggerst version number list -> {:#?}", numer_version_list);
    let number_index = numer_version_list
        .iter()
        .position(|x| x == &version_numer)
        .unwrap();
    debug!("biggerst version number index -> {}", number_index);
    debug!("biggerst version number -> {}", version_numer);
    (version_numer, number_index)
}

fn is_element_in_vec(a: &usize, v: &Vec<usize>) -> bool {
    let result: bool = false;
    for n in v.iter() {
        if a == n {
            let result: bool = true;
            return result;
        }
    }
    result
}

fn render_branch_name_by_tmp(branch_number: &str, tmp: &str) -> String {
    let mut reg = Handlebars::new();
    reg.register_template_string("branch_name", &tmp).unwrap();
    match reg.render("branch_name", &json!({ "number": branch_number })) {
        Ok(rendered) => rendered,
        Err(e) => {
            red!("render branch name failed: {}\n", e);
            std::process::exit(1);
        }
    }
}

fn lower_and_bug_head_replace(source: &str) -> String {
    let replace_re: Regex = Regex::new(r"\[.*\]").unwrap();
    let lower_source = if replace_re.is_match(source) {
        source.replace("[", "").replace("]", "").to_lowercase()
    } else {
        source.to_lowercase()
    };
    use regex::RegexSet;

    let re: RegexSet = RegexSet::new(&[r"bugfix$", r"feature$", r"minor$", r"optimization$", r"sprintfix$", r"refactor$"]).unwrap();
    let matches = re.matches(&lower_source);
    match matches.iter().next() {
        Some(0) => lower_source.replace("bugfix", "fix"),
        Some(1) => lower_source.replace("feature", "feat"),
        Some(2) => lower_source.replace("minor", "docs"),
        Some(3) => lower_source.replace("optimization", "style"),
        Some(4) => lower_source.replace("sprintfix", "refactor"),
        Some(5) => lower_source.replace("refactor", "perf"),
        _ => lower_source
    }
}

fn branch_prefix_strip(branch_name: &str) -> String {
    let mut branch_name = branch_name.to_string();
    if branch_name.starts_with("* ") || branch_name.starts_with("") && branch_name.len() > 1 {
        branch_name = branch_name[2..].to_string();
    }
    branch_name
}

pub fn get_target_issue(
    choice: bool,
    version_re: &str,
    auto_fetch: bool,
    remote_name: &str,
    new_branch_base_format: &str,
    skip_version_re: bool,
) -> (String, String, Vec<String>) {
    let mut re_branchs: Vec<String> = Vec::new();
    let mut all_branchs: Vec<String> = Vec::new();
    let mut branch_pure_number: Vec<String> = Vec::new();
    let nodeman_re: Regex = Regex::new(version_re).unwrap();
    if auto_fetch {
        if let Ok(fetch_result) = Command::new("git").args(["fetch", remote_name]).output() {
            if fetch_result.status.success() {
                yellow!("fetch remote -> {} success!\n", remote_name);
            } else {
                red!(
                    "fetch remote -> {} failed, \n{}",
                    remote_name,
                    str::from_utf8(&fetch_result.stderr).unwrap()
                );
            }
        }
    }

    let branchs = Command::new("git").args(["branch", "-r"]).output().unwrap();

    // list origin branches
    let restr = format!("{}/(.*)", remote_name);
    let target_remote_branch_re = Regex::new(&restr).unwrap();

    let branch_vec = String::from_utf8_lossy(&branchs.stdout);
    let branchs_vec: Vec<&str> = branch_vec.split("\n").collect();
    debug!("branch list -> {:#?}", branchs_vec);

    for branch in branchs_vec {
        let branch_strip = branch_prefix_strip(branch);
        debug!("branch_strip -> {}", branch_strip);
        if target_remote_branch_re.is_match(&branch_strip) {
            let remote_branch_name = target_remote_branch_re
                .captures(&branch_strip)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            all_branchs.push(remote_branch_name);
        }
        // all_branchs.push(branch_strip.clone());
        debug!(
            "all remote {} branch list -> {:#?}",
            remote_name, &all_branchs
        );
        if nodeman_re.is_match(&branch_strip) && !skip_version_re {
            let numer_re = nodeman_re.captures(&branch_strip).unwrap();
            let version_numer: &str = numer_re.get(1).unwrap().as_str();
            branch_pure_number.push(version_numer.to_string());
            re_branchs.push(branch_strip);
        }
    }
    debug!("branch_pure_number -> {:#?}", &branch_pure_number);
    if skip_version_re {
        re_branchs = all_branchs.clone();
    };
    if choice {
        let choice_branch = choose_base_branch(&all_branchs, remote_name);
        (remote_name.to_string(), choice_branch.to_string(), all_branchs)
    } else {
        blue!(
            "match branch list -> {:?}, then choice biggerst version!\n",
            re_branchs
        );
        debug!("all re branch list -> {:#?}", &re_branchs);
        if re_branchs.is_empty() {
            red!("no match branch by Regex: {}\n", version_re);
            std::process::exit(1);
        }
        let mut _latest_re_branch: String = String::new();
        let mut _latest_re_branch_index: usize;
        (_latest_re_branch, _latest_re_branch_index) = biggerst_version_number(branch_pure_number);
        let target_latest_branch_name =
            render_branch_name_by_tmp(&_latest_re_branch, &new_branch_base_format.to_string());
        (
            remote_name.to_string(),
            target_latest_branch_name.to_string(),
            all_branchs
        )
    }
}

fn choose_base_branch(branchs: &Vec<String>, remote_name: &str) -> String {
    let branch_msg = format!(
        "Which branch on remote -> [{}] do you want to choose for add new branch?",
        remote_name
    );
    let choice_branchs = branchs.clone();

    if let Ok(choice) = Select::new(&branch_msg, choice_branchs).prompt() {
        green!("You choose branch -> {}\n", choice);
        choice.to_string()
    } else {
        red!("no choice branch\n");
        std::process::exit(1);
    }
}

fn checkout_branch(target_branch: String) {
    let checkout_result = Command::new("git")
        .args(["checkout", &target_branch])
        .output()
        .unwrap();
    if checkout_result.status.success() {
        green!("checkout branch -> {} success!\n", target_branch);
        std::process::exit(0);
    } else {
        red!(
            "checkout branch -> {} failed, \r

{}",
            target_branch,
            str::from_utf8(&checkout_result.stderr).unwrap()
        );
        std::process::exit(1);
    }
}

fn main() {
    let args = Args::parse();
    //  如果 args.debug == true, 则打印 debug 日志
    let level = if args.debug == "true" {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    simple_logger::SimpleLogger::new()
        .with_level(level)
        .init()
        .unwrap();

    let yaml_config: config::Config;
    match config::load_config(config::CONFIG_PATH) {
        Ok(config) => {
            yaml_config = config;
        }
        Err(e) => {
            red!("parse config /etc/ncommit.yml failed: {}\n", e);
            std::process::exit(1);
        }
    }
    debug!("args -> {:#?}", args);
    if args.web != "false" {
        use view::View;
        let web_handler = view::ViewHandler::new(args.web);
        web_handler.run_command()
    }
    if args.flow == "true" {
        use flow::parse_flow_command;
        let id = parse_branch_issue_id(&yaml_config);
        //  转换为 i32
        let id = id.parse::<i32>().unwrap();
        parse_flow_command(id, "test".to_string())
    }
    let force = args.force == "true";
    if args.pr == "true" {
        use pr::{Pr, PrCommand};
        let p = Pr::new(
            args.chooise.clone(),
            &yaml_config,
            force,
            args.chooise.clone(),
        );
        p.pr();
    }
    // "-m" show maximum number of issues to fetch
    if let Ok(result) = Command::new("gh")
        .args(["issue", "list", "--json", "number,title", "-L", "300"])
        .output()
    {
        if let Ok(branchs) = str::from_utf8(&result.stdout) {
            let mut b_n_vec: Vec<usize> = Vec::new();
            let _load_json: Vec<Foo>;
            match serde_json::from_str::<Vec<Foo>>(branchs) {
                Ok(v) => {
                    _load_json = v;
                }
                Err(e) => {
                    red!("parse issue json failed ->{}", e);
                }
            }
            let load_json: Vec<Foo> = serde_json::from_str(&branchs).unwrap();

            //  interactive select issue
            if args.new_branch == "true" {
                let mut issue_msgs: Vec<&str> = Vec::new();
                let mut issue_numbers: Vec<usize> = Vec::new();
                for issue in load_json.iter() {
                    let issue_pure_re: Regex =
                        Regex::new(&yaml_config.issue_title_filter_re).unwrap();
                    if issue_pure_re.is_match(&issue.title) {
                        let issue_pure = issue_pure_re.captures(&issue.title).unwrap();
                        let pure_msg = issue_pure.get(2).unwrap().as_str();
                        issue_msgs.push(pure_msg);
                        issue_numbers.push(issue.number);
                    }
                }
                let copy_issue = issue_msgs.clone();
                if issue_msgs.len() != 0 {
                    if let Ok(choice) =
                        Select::new("Which issue do you want to choose??", issue_msgs).prompt()
                    {
                        green!("Choice issue -> {}\n", &choice);
                        let index = copy_issue.iter().position(|&r| r == choice).unwrap();
                        let choice_switch: bool;
                        let choice_issue_number = issue_numbers[index];
                        if args.chooise == "true" {
                            choice_switch = true;
                        } else {
                            choice_switch = false;
                        }
                        let (remote_name, branch_name, all_branchs) = get_target_issue(
                            choice_switch,
                            &yaml_config.version_compare_re,
                            yaml_config.enable_auto_fetch,
                            &yaml_config.remote_name,
                            &yaml_config.remote_branch_name_template,
                            false,
                        );
                        let complate_brach_name = format!("{}/{}", remote_name, branch_name);

                        let new_branch = format!(
                            "{}{}",
                            &yaml_config.dev_issue_name_header, choice_issue_number
                        );
                        debug!("new branch name -> {}", new_branch);

                        if all_branchs.contains(&&new_branch) {
                            red!("branch {} already exist, checkout!!!\n", new_branch);
                            if let Ok(checkout_result) =
                                Command::new("git").args(["checkout", &new_branch]).output()
                            {
                                let code = checkout_result.status.code();
                                if code == Some(0) {
                                    green!("checkout to branch {}\n", &new_branch);
                                } else {
                                    red!(
                                        "checkout branch filed! \n{}",
                                        str::from_utf8(&checkout_result.stderr).unwrap()
                                    );
                                    std::process::exit(1);
                                }
                            }
                            std::process::exit(1);
                        }

                        debug!("base branch command -> git checkout -b {} {}", &new_branch, complate_brach_name);
                        if let Ok(add_branch_result) = Command::new("git")
                            .args(["checkout", "-b", &new_branch, &complate_brach_name])
                            .output()
                        {
                            let code = add_branch_result.status.code();
                            if code == Some(0) {
                                green!(
                                    "checkout branch by command -> git checkout -b {} {}\n",
                                    new_branch,
                                    complate_brach_name 
                                );
                            } else {
                                let checkout_result =
                                    str::from_utf8(&add_branch_result.stderr).unwrap();
                                let branch_exists_re = Regex::new(r".*already exists.*").unwrap();
                                if branch_exists_re.is_match(checkout_result) {
                                    let match_stdin = vec![
                                        "y".to_string(),
                                        "yes".to_string(),
                                        "Y".to_string(),
                                        "yes".to_string(),
                                        "Yes".to_string(),
                                    ];
                                    let checkout_msg = format!(
                                        "branch {} already exist, checkout? (y/n)",
                                        new_branch
                                    );
                                    let stdin_result = Text::new(&checkout_msg).prompt();
                                    match stdin_result {
                                        Ok(stdin) => {
                                            if match_stdin.contains(&stdin) {
                                                red!(
                                                    "You press yes. So checkout branch -> {}\n",
                                                    new_branch.clone()
                                                );
                                                checkout_branch(new_branch.clone());
                                            } else {
                                                red!("You press no. So exit!!!\n");
                                                std::process::exit(0);
                                            }
                                        }
                                        Err(e) => {
                                            red!("input error -> {}\n", e);
                                            std::process::exit(1);
                                        }
                                    }
                                }
                                red!(
                                    "checkout branch to {} with base barnch -> {} filed! \n{}",
                                    new_branch,
                                    complate_brach_name,
                                    str::from_utf8(&add_branch_result.stderr).unwrap()
                                );
                                std::process::exit(1);
                            }
                        }
                    }

                    std::process::exit(0)
                }
            }

            if let Ok(result) = get_branch() {
                let branch = String::from_utf8_lossy(&result);
                let issue_re: Regex = Regex::new(&yaml_config.dev_issue_re).unwrap();
                if issue_re.is_match(&branch) {
                    let re_result = issue_re.captures(&branch).unwrap();
                    let branch_number = &re_result[1];
                    let usize_branch = branch_number.parse::<usize>().unwrap();
                    for issue in load_json.iter() {
                        b_n_vec.push(issue.number);
                    }
                    if is_element_in_vec(&usize_branch, &b_n_vec) {
                        for issue in load_json.iter() {
                            if issue.number == usize_branch {
                                let title = &issue.title;
                                let title_re: Regex =
                                    Regex::new(&yaml_config.issue_title_filter_re).unwrap();
                                if title_re.is_match(title) {
                                    let title_result = title_re.captures(&title).unwrap();
                                    let tag = &title_result[1];

                                    let mut message: &str = &String::new();
                                    if args.message != "false" {
                                        message = &args.message;
                                    } else {
                                        message = &title_result[2];
                                    }

                                    let currect_tag = lower_and_bug_head_replace(tag);
                                    let mut c: String = String::new();
                                    let mut d: String = String::new();

                                    if yaml_config.commit_append_nodeman_msg {
                                        c = format!(
                                            "git commit {} \"{}: {} ({} #{}){}\"",
                                            custom_commit_params(&yaml_config.commit_custom_params),
                                            currect_tag,
                                            message,
                                            issue.number,
                                            &yaml_config.commit_link_description,
                                            &yaml_config.commit_append_msg
                                        );
                                        d = format!(
                                            "{}: {} ({} #{}){}",
                                            currect_tag,
                                            message,
                                            issue.number,
                                            &yaml_config.commit_link_description,
                                            &yaml_config.commit_append_msg
                                        );
                                    } else {
                                        c = format!(
                                            "git commit {} \"{}: {} ({} #{})\"",
                                            custom_commit_params(&yaml_config.commit_custom_params),
                                            currect_tag,
                                            message,
                                            &yaml_config.commit_link_description,
                                            issue.number
                                        );
                                        d = format!(
                                            "{}: {} ({} #{})",
                                            currect_tag,
                                            message,
                                            &yaml_config.commit_link_description,
                                            issue.number
                                        );
                                    }

                                    if args.print == "true" {
                                        println!("{}", c);
                                        std::process::exit(1);
                                    }
                                    let mut commit_params = vec!["commit"];
                                    // 把 custom_commit_parmas 里面的参数拆分成列表
                                    let custom_commit_string =
                                        custom_commit_params(&yaml_config.commit_custom_params);
                                    let custom_commit_params: Vec<&str> =
                                        custom_commit_string.split(" ").collect::<Vec<&str>>();
                                    // 把 custom_commit_parmas 里面的参数追加进 commit_params
                                    for custom_commit_param in custom_commit_params {
                                        commit_params.push(&custom_commit_param);
                                    }
                                    commit_params.push(&d);

                                    if let Ok(result) =
                                        Command::new("git").args(commit_params).output()
                                    {
                                        match result.status.code() {
                                            Some(code) => {
                                                if code == 0 {
                                                    let cc =
                                                        String::from_utf8_lossy(&result.stdout);
                                                    println!(" {}\n {}\n ", &c, &cc);
                                                    green!(" - success\n")
                                                } else {
                                                    let cc =
                                                        String::from_utf8_lossy(&result.stderr);
                                                    println!(" {}\n {}\n", &c, &cc);
                                                    red!(" - failed\n")
                                                }
                                            }
                                            None => {
                                                red!("提交失败, 未获取对应返回码");
                                                std::process::exit(1);
                                            }
                                        }
                                    } else {
                                        red!("{} failed !", c);
                                    }
                                }
                            }
                        }
                    } else {
                        red!(
                            "branch number not in remote open issue list -> [{:#?}]\n",
                            &b_n_vec
                        );
                        std::process::exit(1);
                    }
                } else {
                    red!(
                        "branch -> {:?} 不符合匹配规则: {}\n",
                        branch,
                        &yaml_config.dev_issue_re
                    );
                    std::process::exit(1);
                }
            } else {
                println!("{:?}", result)
            }
        }
    } else {
        red!("gh command failed!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn render_shoud_work() {
        let temp = "v{{ number }}-dev";
        let version = "1.0.0";
        assert_eq!(
            render_branch_name_by_tmp(version, temp),
            "v1.0.0-dev".to_string()
        )
    }
    // #[test]
    // fn test_get_current_id() {
    //     simple_logger::SimpleLogger::new().env().init().unwrap();
    //     let id = parse_branch_issue_id();
    //     error!("{}1111", id)
    // }
}
