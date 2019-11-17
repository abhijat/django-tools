#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub enum Stage {
    PreSave = 0,
    PostSave = 1,
    PostDelete = 2,
    ManyToManyChanged = 3,
}

impl ToString for Stage {
    fn to_string(&self) -> String {
        match self {
            Stage::PreSave => String::from("pre_save"),
            Stage::PostSave => String::from("post_save"),
            Stage::PostDelete => String::from("post_delete"),
            Stage::ManyToManyChanged => String::from("m2m_changed"),
        }
    }
}

impl Stage {
    pub fn from(s: &str) -> Stage {
        match s {
            "post_save" => Stage::PostSave,
            "pre_save" => Stage::PreSave,
            "post_delete" => Stage::PostDelete,
            "m2m_changed" => Stage::ManyToManyChanged,
            _ => panic!("{} is not implemented yet!", s),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Receiver {
    pub subject: String,
    pub stage: Stage,
    pub name: String,
    pub source_path: String,
}

impl ToString for Receiver {
    fn to_string(&self) -> String {
        format!("{} [{}]", self.name, self.source_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_and_from_string() {
        let stage = Stage::ManyToManyChanged;
        assert_eq!(stage, Stage::from(&stage.to_string()));
    }
}
