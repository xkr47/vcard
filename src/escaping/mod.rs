use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    static ref NEW_LINE_RE: Regex = { Regex::new(r"\n").unwrap() };
    static ref COMMA_RE: Regex = { Regex::new(r",").unwrap() };
    static ref SEMICOLON_RE: Regex = { Regex::new(r";").unwrap() };
    static ref BACKSLASH_RE: Regex = { Regex::new("\\").unwrap() };
}

pub(crate) fn escape_new_line(s: &str) -> Cow<'_, str> {
    NEW_LINE_RE.replace_all(s, "\\n")
}

pub(crate) fn escape_comma(s: &str) -> Cow<'_, str> {
    COMMA_RE.replace_all(s, "\\,")
}

pub(crate) fn escape_semicolon(s: &str) -> Cow<'_, str> {
    SEMICOLON_RE.replace_all(s, "\\;")
}

pub(crate) fn escape_backslash(s: &str) -> Cow<'_, str> {
    BACKSLASH_RE.replace_all(s, "\\\\")
}