extern crate nom;
use regex::Captures;
use regex::{CaptureMatches, Regex};
use std::collections::HashMap;

enum Process {
    LmtpProcess(String, u64),
    ReinjectProcess(String, u64),
    MeFilter(String, u64),
    UnknownProcess(String, u64),
}

struct PostfixTypedLine {
    date: String,
    server: String,
    process: Process,
    qid: Option<String>,
    remaining: String,
    uuid: Option<String>,
}

fn main() {
    let re_named = Regex::new(r"^(?P<date>.{15}) (?P<server>.{9,12}) (?P<service>\S+?)\[(?P<pid>\d+)\] to=<(?P<to>[^>]*)>, orig_to=<(?P<orig_to>[^>]*)>.*status=(?P<status>\S+)").unwrap();
    let re = Regex::new(
        r"^(.{15}) (.{9,12}) (\S+?)\[(\d+)\] to=<([^>]*)>, orig_to=<([^>]*)>.*status=(\S+)",
    )
    .unwrap();
    let text = "Aug 29 10:01:02 server0303 postfix_auth/smtpd_abo[27339] to=<server000000000000000003435583@1100.back02-mail01-01.null.org>, orig_to=<john@doe.org>, status=sent";
    for cap in re.captures_iter(text) {
        println!(
            "{} {} {} {} {} {} {}",
            &cap[1], &cap[2], &cap[3], &cap[4], &cap[5], &cap[6], &cap[7]
        );
    }
    for cap in re_named.captures_iter(text) {
        println!(
            "{} {} {} {} {} {} {}",
            &cap[1], &cap[2], &cap[3], &cap[4], &cap[5], &cap[6], &cap[7]
        );
    }

    let caps = re_named.captures(text).unwrap();
    let dict: HashMap<&str, &str> = re_named
        .capture_names()
        .flatten()
        .filter_map(|n| Some((n, caps.name(n)?.as_str())))
        .collect();
    println!("{:#?}", dict);
}
