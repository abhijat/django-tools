use std::{fs, io};
use std::collections::BTreeMap;

use rayon::prelude::*;
use regex::Regex;

use crate::paths::collect_source_files;
use crate::receivers::structures::{Receiver, Stage};

pub fn find_receivers_in_paths(
    paths: &Vec<&str>,
    subject: &str,
) -> io::Result<BTreeMap<Stage, Vec<Receiver>>> {
    let paths = collect_source_files(paths, ".py");
    let results = paths
        .par_iter()
        .map(|path| match fs::read_to_string(path) {
            Ok(data) => Some(find_receivers_in_file_content(&data, subject, path)),
            Err(_) => None,
        })
        .collect::<Vec<Option<Vec<Receiver>>>>();

    let map = results
        .into_iter()
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .map(sort_receivers)
        .fold(BTreeMap::new(), |acc, x| acc.into_iter().chain(x).collect());
    Ok(map)
}

pub fn find_receivers_in_file_content(
    data: &str,
    subject: &str,
    source_path: &str,
) -> Vec<Receiver> {
    let pattern = format!(
        r"@receiver\((?P<stage>\w+)\s*,\s*sender=(?P<subject>{})",
        subject
    );
    let expression = Regex::new(&pattern).unwrap();
    let def_expr = Regex::new(r"def (?P<func_name>\w+)\(").unwrap();

    let mut receivers = vec![];
    let mut peekable = data.lines().peekable();
    while let Some(line) = peekable.next() {
        if let Some(captures) = expression.captures(line) {
            let stage = captures.name("stage");
            let subject = captures.name("subject");

            while let Some(line) = peekable.peek() {
                // Skip extra receiver decorators
                if !line.starts_with("def") {
                    peekable.next();
                } else {
                    break;
                }
            }

            let next_line = peekable.next().unwrap();
            if stage.is_some() && subject.is_some() && !next_line.is_empty() {
                if let Some(captures) = def_expr.captures(next_line) {
                    let receiver = Receiver {
                        subject: subject.unwrap().as_str().to_owned(),
                        stage: Stage::from(stage.unwrap().as_str()),
                        name: captures.name("func_name").unwrap().as_str().to_owned(),
                        source_path: source_path.to_owned(),
                    };
                    receivers.push(receiver);
                }
            }
        }
    }
    receivers
}

fn sort_receivers(receivers: Vec<Receiver>) -> BTreeMap<Stage, Vec<Receiver>> {
    let mut stage_receivers: BTreeMap<Stage, Vec<Receiver>> = BTreeMap::new();
    for receiver in receivers {
        stage_receivers
            .entry(receiver.stage)
            .or_default()
            .push(receiver);
    }
    stage_receivers
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"@receiver(post_save, sender=Foo123_Ba)
        def bar()
        @receiver(pre_save, sender=Foo123_Ba)
        def aaaa()
        @receiver(post_delete, sender=Foo123_Ba)
        def bbbbb()
        @receiver(post_save, sender=Foo123_Ba)
        def ccccc()
        @receiver(post_save, sender=Foo123_Ba)
        def xxxxxx()
        @receiver(post_save, sender=Foo123_Ba)
        def nnnnnn()"#;

    #[test]
    fn can_find_receivers() {
        let receivers = find_receivers_in_file_content(DATA, "Foo123_Ba", "1");
        assert_eq!(6, receivers.len());

        let r = &receivers[0];
        let expected = Receiver {
            subject: String::from("Foo123_Ba"),
            stage: Stage::PostSave,
            name: String::from("bar"),
            source_path: String::from("1"),
        };

        assert_eq!(expected, *r);
    }

    #[test]
    fn can_sort_receivers() {
        let stage_receivers =
            sort_receivers(find_receivers_in_file_content(DATA, "Foo123_Ba", "1"));
        assert_eq!(3, stage_receivers.len());
        let (stage, receivers) = stage_receivers.iter().next().unwrap();
        assert_eq!(stage, &Stage::PreSave);
        assert_eq!(1, receivers.len());
    }
}
