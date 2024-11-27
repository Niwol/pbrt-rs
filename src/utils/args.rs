use std::{fmt::format, vec::IntoIter};

fn normalize_arg(string: String) -> String {
    let mut result = String::new();
    for c in string.chars() {
        if c != '_' || c != '-' {
            result.push(c.to_lowercase().next().unwrap());
        }
    }

    result
}

fn match_prefix(string: String, prefix: String) -> bool {
    string.starts_with(prefix.as_str())
}

trait InitArg {
    fn init(&mut self, value: String) -> bool;
}

impl InitArg for bool {
    fn init(&mut self, value: String) -> bool {
        if value == "false" {
            *self = false;
            return true;
        }

        return false;
    }
}

pub fn parse_arg<T: InitArg, F>(
    mut arg: String,
    arg_iter: &mut IntoIter<String>,
    name: &str,
    out: &mut T,
    on_error: F,
) -> bool
where
    F: Fn(String),
{
    if arg.starts_with("--") {
        arg.drain(..2);
    } else if arg.starts_with("-") {
        arg.drain(..1);
    }

    if match_prefix(arg.clone(), format!("{name}=")) {
        let pos = arg.find('=').unwrap();
        let value: String = arg.drain(..pos).collect();

        return true;
    }

    false
}


