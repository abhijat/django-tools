use std::collections::HashSet;
use std::fs;

use rayon::prelude::*;

pub fn get_declarations_in_file_content(
    data: &str,
    pred: Option<&dyn Fn(&str) -> bool>,
) -> HashSet<String> {
    let mut declarations = HashSet::new();
    for line in data.lines() {
        if line.contains(" = ") {
            let declaration = line.splitn(2, " = ").nth(0).unwrap().trim();
            if !declaration.contains(" ") && declaration.len() > 2 {
                declarations.insert(declaration.to_owned());
            }
        }
    }

    if let Some(pred) = pred {
        declarations.into_iter().filter(|s| pred(s)).collect()
    } else {
        declarations
    }
}

pub fn get_declarations(paths: &HashSet<String>) -> HashSet<String> {
    let results = paths
        .into_par_iter()
        .map(|path| match fs::read_to_string(path) {
            Ok(data) => Some(get_declarations_in_file_content(&data, None)),
            Err(_) => None,
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<HashSet<String>>>();

    results
        .into_iter()
        .fold(HashSet::new(), |acc, x| acc.into_iter().chain(x).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_declarations() {
        let declarations = get_declarations_in_file_content(
            r#"
        testing = 1
        foo and bar
        "#,
            None,
        );
        assert_eq!(1, declarations.len());
        let only = declarations.iter().next().unwrap().as_str();
        assert_eq!("testing", only);
    }

    #[test]
    fn non_declarations_are_skipped() {
        let declarations = get_declarations_in_file_content(
            r#"
            let foo = bar
            while a = = x
            "#,
            None,
        );
        assert!(declarations.is_empty());
    }
}
