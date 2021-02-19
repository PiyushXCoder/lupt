
pub enum Validation {
    NonEmpty,
    NoGupt,
    NoSpace,
    NoHashtag,
}

pub fn validate(val: Vec<Validation>, dat: &str, entry_name: &str) -> Option<String> {
    for v in val {
        let out = match v {
            Validation::NonEmpty => non_empty(dat, entry_name),
            Validation::NoGupt => is_gupt(dat),
            Validation::NoSpace => no_space(dat, entry_name),
            Validation::NoHashtag => no_hashtag(dat, entry_name)
        };

        if out != None {
            return out;
        }
    }
    None
}

fn non_empty(dat: &str, entry_name: &str) -> Option<String> {
    if dat.len() > 0 {
        None
    } else {
        Some(format!("{} is Required", entry_name))
    }
}

fn is_gupt(dat: &str) -> Option<String> {
    if !dat.starts_with("gupt_") {
        None
    } else {
        Some(format!("Restricted group"))
    }
}

fn no_space(dat: &str, entry_name: &str) -> Option<String> {
    if dat.contains(" ") {
        Some(format!("{} shounld not have space", entry_name))
    } else {
        None
    }
}

fn no_hashtag(dat: &str, entry_name: &str) -> Option<String> {
    if dat.contains("#") {
        Some(format!("{} shounld not have Hashtag(#)", entry_name))
    } else {
        None
    }
}