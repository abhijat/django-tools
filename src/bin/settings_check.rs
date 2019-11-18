use std::collections::HashSet;
use std::fs;

use structopt::StructOpt;

use project_checker as pc;
use project_checker::to_refs;
use std::collections::hash_set::Difference;
use std::collections::hash_map::RandomState;

#[derive(StructOpt, Debug)]
#[structopt(version = "0.1.0", author = "Abhijat Malviya")]
pub struct Opts {
    #[structopt(required = true, long, short = "r")]
    pub source_roots: Vec<String>,

    #[structopt(required = true, short, long)]
    pub settings_path: String,
}

fn is_valid_setting(s: &str) -> bool {
    s.chars().all(|chr| !chr.is_lowercase())
}

fn find_settings(settings: &HashSet<String>, sources: &HashSet<String>) -> HashSet<String> {
    let mut found = HashSet::new();

    let mut all_settings_found = false;
    for source_file in sources {
        if all_settings_found {
            break;
        }

        match fs::read_to_string(source_file) {
            Ok(data) => {
                for setting in settings {
                    if data.contains(setting) {
                        found.insert(setting.to_owned());
                        if found.len() == settings.len() {
                            all_settings_found = true;
                            break;
                        }
                    }
                }
            }
            Err(err) => {
                println!("failed to read {} with error {}", source_file, err);
            }
        }
    }

    found
}

fn present_diff(diff: Difference<String, RandomState>) {
    let mut v: Vec<String> = diff.map(|s| s.to_owned()).collect();
    v.sort();
    for value in v {
        println!("{}", value);
    }
}

fn main() {
    let opts: Opts = Opts::from_args();
    let settings_data =
        fs::read_to_string(&opts.settings_path).expect("Failed to read settings path");
    let settings = pc::get_declarations_in_file_content(&settings_data, Some(&is_valid_setting));
    let mut sources = pc::collect_source_files(&to_refs(&opts.source_roots), ".py");

    sources.remove(&opts.settings_path);

    let found = find_settings(&settings, &sources);
    present_diff(settings.difference(&found));
}
