use criterion::{black_box, criterion_group, criterion_main, Criterion};

use regex::Captures;
use regex::{CaptureMatches, Regex};

fn named<'a>(text: &'a str, re_named: &'a Regex) -> Captures<'a> {
    re_named.captures(text).unwrap()
}

fn tuple<'a>(text: &'a str, re: &'a Regex) -> CaptureMatches<'a, 'a> {
    re.captures_iter(text)
}

fn small_test(b: &mut Criterion) {
    let re = Regex::new(
        r"^(.{15}) (.{9,12}) (\S+?)\[(\d+)\] to=<([^>]*)>, orig_to=<([^>]*)>.*status=(\S+)",
    )
    .unwrap();
    let text = "Aug 29 10:01:02 server0303 postfix_auth/smtpd_abo[27339] to=<server000000000000000003435583@1100.back02-mail01-01.null.org>, orig_to=<john@doe.org>, status=sent";
    b.bench_function("small", |b| {
        b.iter(|| tuple(black_box(text), black_box(&re)))
    });
}

fn big_test(b: &mut Criterion) {
    let re_named = Regex::new(r"^(?P<date>.{15}) (?P<server>.{9,12}) (?P<service>\S+?)\[(?P<pid>\d+)\] to=<(?P<to>[^>]*)>, orig_to=<(?P<orig_to>[^>]*)>.*status=(?P<status>\S+)").unwrap();
    let text = "Aug 29 10:01:02 server0303 postfix_auth/smtpd_abo[27339] to=<server000000000000000003435583@1100.back02-mail01-01.null.org>, orig_to=<john@doe.org>, status=sent";
    b.bench_function("big", |b| {
        b.iter(|| named(black_box(text), black_box(&re_named)))
    });
}

criterion_group!(scala, small_test, big_test);
criterion_main!(scala);
