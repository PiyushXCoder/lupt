/*
    This file is part of Lupt.

    Lupt is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Lupt is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Lupt.  If not, see <https://www.gnu.org/licenses/>
*/
pub enum Validation {
    NonEmpty,
    NoGupt,
    NoSpace,
    NoHashtag,
    NoAndOrQuestion,
}

pub fn validate(val: Vec<Validation>, dat: &str, entry_name: &str) -> Option<String> {
    for v in val {
        let out = match v {
            Validation::NonEmpty => non_empty(dat, entry_name),
            Validation::NoGupt => is_gupt(dat),
            Validation::NoSpace => no_space(dat, entry_name),
            Validation::NoHashtag => no_hashtag(dat, entry_name),
            Validation::NoAndOrQuestion => no_and_or_question(dat, entry_name),
        };

        if out != None {
            return out;
        }
    }
    None
}

fn non_empty(dat: &str, entry_name: &str) -> Option<String> {
    if dat.len() > 0 || dat != "" {
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

fn no_and_or_question(dat: &str, entry_name: &str) -> Option<String> {
    if dat.contains("&") {
        Some(format!("{} shounld not have &", entry_name))
    } else if dat.contains("?") {
        Some(format!("{} shounld not have ?", entry_name))
    } else {
        None
    }
}
