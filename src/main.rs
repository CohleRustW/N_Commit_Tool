use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use core::num;
use std::{process::Command, sync::Arc};
use std::result::Result as CResult;
use std::str;
use inquire::{Text, validator::{StringValidator, Validation}};
use inquire::{error::InquireError, Select};
use version_compare::{Cmp, Version};
#[macro_use]
extern crate colour;

static REMOTE_NAME: &'static str = "origin";

#[derive(Parser, Debug)]
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

    /// Open issue on chrome
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
        short = 'c',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    chooise: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    number: usize,
    title: String,
}

fn get_branch() -> CResult<Vec<u8>, Box<std::io::Error>> {
    let branch = Command::new("git")
        .args(["branch", "--show-current"])
        .output()?;
    Ok(branch.stdout)
}

fn biggerst_version_number (version_list: Vec<String>) -> (String, usize) {
    let mut version_numer: String = String::new();
    let numer_version_list = version_list.clone();
    for version in version_list {
        let a = Version::from(&version).unwrap();
        let b = Version::from(&version_numer).unwrap();
        if a > b {
            version_numer = version.to_string();
        }
    }
    let number_index = numer_version_list.iter().position(|x| x == &version_numer).unwrap();
    (version_numer,  number_index)

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

fn lower_and_bug_head_replace(source: &str) -> String {
    let lower_source = source.to_lowercase();
    let bug_re: Regex = Regex::new(r"bug$").unwrap();
    if bug_re.is_match(&lower_source) {
        let result: String = lower_source.replace("bug", "bugfix");
        result
    } else {
        lower_source
    }
}

fn branch_prefix_strip(branch_name: &str) -> String {
    let mut branch_name = branch_name.to_string();
    if branch_name.starts_with("* ") || branch_name.starts_with("") && branch_name.len() > 1 {
        branch_name = branch_name[2..].to_string();
    }
    branch_name
}

#[warn(unused_assignments)]
fn get_latest_issue(choice: bool) -> (String, Vec<String>) {
    let mut re_branchs: Vec<String> = Vec::new();
    let mut all_branchs: Vec<String> = Vec::new();
    let mut branch_pure_number: Vec<String> = Vec::new();
    let nodeman_re: Regex = Regex::new(r"V(\d{1,2}\.\d{1,2}\.\d{1,2})-rc").unwrap();
    if let Ok(fetch_result) = Command::new("git").args(["fetch", REMOTE_NAME]).output() {
        if fetch_result.status.success() {
            yellow/!("fetch remote -> {} success!\n", REMOTE_NAME);
        } else {
            red!("fetch remote -> {} failed, \n{}", REMOTE_NAME, str::from_utf8(&fetch_result.stderr).unwrap());
        }
    }
    let branchs = Command::new("git").args(["branch",  "-r"]).output().unwrap();

    // list origin branches
    let restr = format!("{}/(.*)", REMOTE_NAME);
    let target_remote_branch_re = Regex::new(&restr).unwrap();
    // let target_remote_branchs = Vec::new();

    let branch_vec  =  String::from_utf8_lossy(&branchs.stdout);
    let branchs_vec: Vec<&str> = branch_vec.split("\n").collect();


    for branch in branchs_vec {
        let branch_strip = branch_prefix_strip(branch);
        if target_remote_branch_re.is_match(&branch_strip) {
            let remote_branch_name = target_remote_branch_re.captures(&branch_strip).unwrap().get(1).unwrap().as_str().to_string();
            all_branchs.push(remote_branch_name);
        }
        // all_branchs.push(branch_strip.clone());
        if nodeman_re.is_match(&branch_strip) {
            let numer_re = nodeman_re.captures(&branch_strip).unwrap();
            let version_numer: &str = numer_re.get(1).unwrap().as_str();
            branch_pure_number.push(version_numer.to_string());
            re_branchs.push(branch_strip);
        }
    }
    if choice {
        let choice_branch = choose_base_branch(&all_branchs);
        (format!("{}/{}", REMOTE_NAME, choice_branch), all_branchs)
    } else {
        blue!("match branch list -> {:?}, then choice biggerst version!\n", re_branchs);
        if re_branchs.is_empty() {
            red!("no match branch\n");
            std::process::exit(1);
        }
        let mut _latest_re_branch: String = String::new();
        let mut _latest_re_branch_index: usize;
        (_latest_re_branch, _latest_re_branch_index) = biggerst_version_number(branch_pure_number);
        (format!("{}/V{}-rc", REMOTE_NAME, _latest_re_branch), all_branchs)
    }
}

fn choose_base_branch (branchs: &Vec<String>) -> String {
    let choice_branchs = branchs.clone();

    if let Ok(choice) = Select::new("Which branch do you want to choose for branch?", choice_branchs).prompt() {
        green!("You choose branch -> {}\n", choice);
        choice.to_string()
    } else {
        red!("no choice branch\n");
        std::process::exit(1);
    }
}


fn main() {
    let args = Args::parse();
    if args.web == "true" {
        if let Ok(result) = Command::new("gh").args(["issue", "list", "--web"]).output() {
            let code = result.status.code();
            if code != Some(0) {
                println!("{}", str::from_utf8(&result.stdout).unwrap());
            } else {
                std::process::exit(1);
            }
        }
    }
    // "-m" show maximum number of issues to fetch
    if let Ok(result) = Command::new("gh")
        .args(["issue", "list", "--json", "number,title", "-L", "200"])
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
                    let issue_pure_re: Regex = Regex::new(r"\[(.*)\](.*)").unwrap();
                    if issue_pure_re.is_match(&issue.title) {
                        let issue_pure = issue_pure_re.captures(&issue.title).unwrap();
                        let pure_msg = issue_pure.get(2).unwrap().as_str();
                        issue_msgs.push(pure_msg);
                        issue_numbers.push(issue.number);
                    }
                }
                let copy_issue = issue_msgs.clone();
                if issue_msgs.len() != 0 {
                    if let Ok(choice) = Select::new("Which issue do you want to choose??", issue_msgs).prompt() {
                        green!("Choice issue -> {}\n", &choice);
                        let index = copy_issue.iter().position(|&r| r == choice).unwrap();
                        let choice_switch: bool;
                        let choice_issue_number = issue_numbers[index];
                        if args.chooise == "true" {
                            choice_switch = true;
                        } else {
                            choice_switch = false;
                        }
                        let (latest_issue , all_branchs )= get_latest_issue(choice_switch);

                        let new_branch = format!("dev_issue#{}", choice_issue_number);

                        if all_branchs.contains(&&new_branch) {
                            red!("branch {} already exist, checkout!!!\n", new_branch);
                            if let Ok(checkout_result) = Command::new("git").args(["checkout", &new_branch]).output() {
                                let code = checkout_result.status.code();
                                if code == Some(0) {
                                    green!("checkout to branch {}\n", &new_branch);
                                } else {
                                    red!("checkout branch filed! \n{}", str::from_utf8(&checkout_result.stderr).unwrap());
                                    std::process::exit(1);
                                }
                            }
                            std::process::exit(1);
                        }

                        if let Ok(add_branch_result) = Command::new("git")
                            .args(["checkout", "-b" , &new_branch,  &latest_issue])
                            .output()
                        {
                            let code = add_branch_result.status.code();
                            if code == Some(0) {
                                green!("checkout branch by command -> git checkout -b {} {}\n", new_branch, latest_issue, latest_issue);
                            } else {
                                red!("checkout branch to {} with base barnch -> {} filed! \n{}", new_branch, latest_issue, str::from_utf8(&add_branch_result.stderr).unwrap());
                                std::process::exit(1);
                            }
                        }

                    }

                std::process::exit(0)
                }
            }

            if let Ok(result) = get_branch() {
                let branch = String::from_utf8_lossy(&result);
                let issue_re: Regex = Regex::new(r".*issue#?(\d+).*").unwrap();
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
                                let title_re: Regex = Regex::new(r"\[(.*)\] (.*)").unwrap();
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
                                    let c = format!(
                                        "git commit -m \"{}: {} (closed #{})(wf -l)\"",
                                        currect_tag, message, issue.number
                                    );
                                    let d = format!(
                                        "{}: {} (closed #{})(wf -l)",
                                        currect_tag, message, issue.number
                                    );
                                    if args.print == "true" {
                                        println!("{}", c);
                                        std::process::exit(1);
                                    }
                                    if let Ok(result) =
                                        Command::new("git").args(["commit", "-m", &d]).output()
                                    {
                                        match result.status.code() {
                                            Some(code) => {
                                                if code == 0 {
                                                    let cc = String::from_utf8_lossy(&result.stdout);
                                                    println!(" {}\n {}\n ", &c, &cc);
                                                    green!(" - success\n")
                                                } else {
                                                    let cc = String::from_utf8_lossy(&result.stderr);
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
                                } else {
                                    red!("miss title re");
                                    std::process::exit(1);
                                }
                            }
                        }
                    } else {
                        red!("branch number not in issue list");
                        std::process::exit(1);
                    }
                } else {
                    red!(
                        "branch -> {:?} 不符合匹配规则 like: (*issue#666*|*issue666*)\n",
                        branch
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
