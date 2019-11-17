use std::collections::HashSet;

use structopt::StructOpt;

pub use declarations::*;
pub use paths::*;
pub use receivers::operations::*;
pub use receivers::structures::*;

mod paths;
mod receivers;
mod declarations;


#[derive(StructOpt, Debug)]
#[structopt(version = "0.1.0", author = "Abhijat Malviya")]
pub struct Opts {
    #[structopt(required = true)]
    pub source_roots: Vec<String>,

    #[structopt(short, long)]
    pub settings_path: Option<String>,
}

pub fn to_refs(v: &Vec<String>) -> Vec<&str> {
    v.iter().map(|s| s.as_str()).collect()
}

pub fn distances(words: &HashSet<String>) {
    for word in words {
        for other in words.iter()
            .filter(|w| w.len() == word.len() && *w != word) {
            let distance = strsim::damerau_levenshtein(word, other);
            if distance == 1 {
                println!("{} <==> {}", word, other);
            }
        }
    }
}
