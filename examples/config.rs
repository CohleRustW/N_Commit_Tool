// use anyhow::Result;

// use serde::{Serialize, Deserialize};

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// struct Point {
//     remote_name: String,
// }

// fn main()  -> Result<()> {
//     // let content = include_str!("../fixfures/config.yml");
//     // let point = Point{ remote_name: "origin".to_string() };
//     let content = "x: 1.0\ny: 2.0\n";
//     let des_point = serde_json::from_str(content)?;
//     println!("{:#?}", des_point);
//     println!("content: {}", content);
//     Ok(())
// }
use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    pub remote_name: String,
    pub dev_issue_re: String,
    pub version_compare_re: String,
    pub enable_auto_fetch: bool,
    pub commit_append_nodeman_msg: bool,
    pub commit_append_msg: String,
}

fn main() -> Result<(), serde_yaml::Error> {
    let yaml = include_str!("../fixfures/config.yml");
    let deserialized_point: Point = serde_yaml::from_str(&yaml)?;
    let test_re = Regex::new(&deserialized_point.dev_issue_re).unwrap();
    let test_str = "xxxissue#111xxx";
    println!("{:#?}", deserialized_point.dev_issue_re);
    if test_re.is_match(test_str) {
        let c = test_re.captures(test_str).unwrap().get(1).unwrap().as_str();
        println!("c: {}", c);
    } else {
        println!("not match");
    }
    Ok(())
}