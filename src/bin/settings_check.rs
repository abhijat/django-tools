use std::collections::HashSet;
use std::fs;

use structopt::StructOpt;

use project_checker as pc;
use project_checker::{Opts, to_refs};

fn main() {
    let opts: Opts = Opts::from_args();
    let settings_path = &opts.settings_path
        .clone()
        .expect("Need settings path to check");

    let data = fs::read_to_string(settings_path).unwrap();
    let pred = |s: &str| s.chars().all(|chr| !chr.is_lowercase());

    let settings = pc::get_declarations_in_file_content(&data, Some(&pred));
    let sources = pc::collect_source_files(
        &to_refs(&opts.source_roots),
        ".py"
    ).unwrap();

    let mut found = HashSet::new();

    for setting in &settings {
        for source_file in &sources {
            if let Ok(data) = fs::read_to_string(source_file) {
                if data.contains(setting) {
                    found.insert(setting.to_owned());
                    break;
                }
            }
        }
    }

    for nf in settings.difference(&found) {
        println!("{}", nf);
    }
}