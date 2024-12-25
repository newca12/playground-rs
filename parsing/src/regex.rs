use ::regex::Regex;
use std::sync::LazyLock;
use std::collections::HashMap;

use crate::{CommonCopyPrefix, SmtpdClientLoginCopy, TEXT, TEXT2};

pub static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(.{15}) (.{9,12}) (\S+?)\[(\d+)\] to=<([^>]*)>, orig_to=<([^>]*)>.*status=(\S+)")
        .unwrap()
});
pub static RE_NAMED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<date>.{15}) (?P<server>.{9,12}) (?P<service>\S+?)\[(?P<pid>\d+)\] to=<(?P<to>[^>]*)>, orig_to=<(?P<orig_to>[^>]*)>.*status=(?P<status>\S+)").unwrap()
});

pub static RE2: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(.{32}) (.{9,12}) (\S+?)\[(\d+)\]: (.*): client=(.*)\[(.*)\]:\d+(?:, sasl_method=(.*), sasl_username=(.*))?").unwrap()
});

pub fn regex_sample() {
    println!("Unamed tuple: {:?}", unamed_tuple(TEXT, &RE));
    println!("Named tuple: {:?}", named_tuple(TEXT, &RE_NAMED));
    println!("Unamed owned tuple: {:?}", unamed_owned_tuple(TEXT, &RE));
    println!(
        "Named owned tuple: {:?}",
        named_owned_tuple(TEXT, &RE_NAMED)
    );
    println!("regex {:?}", regex_maillog(TEXT2));

    let caps = RE_NAMED.captures(TEXT).unwrap();
    let dict: HashMap<&str, &str> = RE_NAMED
        .capture_names()
        .flatten()
        .filter_map(|n| Some((n, caps.name(n)?.as_str())))
        .collect();
    println!("dico {:#?}", dict);
}

pub fn unamed_tuple<'a>(
    text: &'a str,
    re_named: &'a Regex,
) -> (
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
) {
    let r = re_named.captures(text).unwrap();
    (
        r.get(1).unwrap().as_str(),
        r.get(2).unwrap().as_str(),
        r.get(3).unwrap().as_str(),
        r.get(4).unwrap().as_str(),
        r.get(5).unwrap().as_str(),
        r.get(6).unwrap().as_str(),
        r.get(7).unwrap().as_str(),
    )
}

pub fn unamed_owned_tuple<'a>(
    text: &'a str,
    re_named: &'a Regex,
) -> (String, String, String, u32, String, String, String) {
    let cap = re_named.captures(text).unwrap();
    (
        cap[1].to_owned(),
        cap[2].to_owned(),
        cap[3].to_owned(),
        cap[4].parse().unwrap(),
        cap[5].to_owned(),
        cap[6].to_owned(),
        cap[7].to_owned(),
    )
}

pub fn named_tuple<'a>(
    text: &'a str,
    re_named: &'a Regex,
) -> (
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
) {
    let r = re_named.captures(text).unwrap();
    (
        r.name("date").unwrap().as_str(),
        r.name("server").unwrap().as_str(),
        r.name("service").unwrap().as_str(),
        r.name("pid").unwrap().as_str(),
        r.name("to").unwrap().as_str(),
        r.name("orig_to").unwrap().as_str(),
        r.name("status").unwrap().as_str(),
    )
}

pub fn named_owned_tuple<'a>(
    text: &'a str,
    re_named: &'a Regex,
) -> (String, String, String, u32, String, String, String) {
    let cap = re_named.captures(text).unwrap();
    (
        cap["date"].to_owned(),
        cap["server"].to_owned(),
        cap["service"].to_owned(),
        cap["pid"].parse().unwrap(),
        cap["to"].to_owned(),
        cap["orig_to"].to_owned(),
        cap["status"].to_owned(),
    )
}

pub fn regex_maillog(s: &str) -> (CommonCopyPrefix, SmtpdClientLoginCopy) {
    let cap = RE2.captures(s).unwrap();
    (
        CommonCopyPrefix {
            date: cap[1].to_string(),
            server: cap[2].to_string(),
            process: cap[3].to_string(),
            pid: cap[4].parse().unwrap(),
        },
        SmtpdClientLoginCopy {
            qid: cap[5].to_string(),
            domain: cap[6].to_string(),
            ip: cap[7].to_string(),
            sasl_method: cap[8].to_string(),
            sasl_username: cap[9].to_string(),
        },
    )
}

#[cfg(test)]
mod tests {

    use crate::test_commons::tests::{
        check, check_common_copy, check_s, check_smtpd_client_login_copy,
    };

    use super::*;

    #[test]
    fn test_unamed_tuple() {
        check(unamed_tuple(TEXT, &RE));
    }

    #[test]
    fn test_named_tuple() {
        check(named_tuple(TEXT, &RE_NAMED));
    }

    #[test]
    fn test_owned_named_tuple() {
        check_s(named_owned_tuple(TEXT, &RE_NAMED));
    }

    #[test]
    fn test_owned_unamed_tuple() {
        check_s(unamed_owned_tuple(TEXT, &RE));
    }

    #[test]
    fn client_login_regex() {
        let (common, client_login) = regex_maillog(TEXT2);
        check_common_copy(common);
        check_smtpd_client_login_copy(client_login);
    }
}
