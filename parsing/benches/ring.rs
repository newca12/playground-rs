use criterion::{criterion_group, criterion_main, Criterion};
use parsing::{
    regex::{
        named_owned_tuple, named_tuple, regex_maillog, unamed_owned_tuple, unamed_tuple, RE,
        RE_NAMED,
    },
    winnow::{winnow_maillog, winnow_maillog_copy},
    TEXT, TEXT2,
};
use regex::{Captures, Regex};

fn capture<'a>(text: &'a str, re: &'a Regex) -> Captures<'a> {
    re.captures(text).unwrap()
}

fn unamed_regex(b: &mut Criterion) {
    b.bench_function("unamed_regex", |b| b.iter(|| capture(TEXT, &RE)));
}

fn named_regex(b: &mut Criterion) {
    b.bench_function("named_regex", |b| b.iter(|| capture(TEXT, &RE_NAMED)));
}

fn unamed_regex_tuple(b: &mut Criterion) {
    b.bench_function("unamed_regex_tuple", |b| b.iter(|| unamed_tuple(TEXT, &RE)));
}

fn named_regex_tuple(b: &mut Criterion) {
    b.bench_function("named_regex_tuple", |b| {
        b.iter(|| named_tuple(TEXT, &RE_NAMED))
    });
}

fn named_regex_owned_tuple(b: &mut Criterion) {
    b.bench_function("named_owned_tuple", |b| {
        b.iter(|| named_owned_tuple(TEXT, &RE_NAMED))
    });
}

fn unamed_regex_owned_tuple(b: &mut Criterion) {
    b.bench_function("unamed_regex_owned_tuple", |b| {
        b.iter(|| unamed_owned_tuple(TEXT, &RE))
    });
}

fn maillog_regex(b: &mut Criterion) {
    b.bench_function("regex_maillog", |b| b.iter(|| regex_maillog(TEXT2)));
}

fn maillog_winnow(b: &mut Criterion) {
    b.bench_function("winnow_maillog", |b| b.iter(|| winnow_maillog(TEXT2)));
}

fn maillog_winnow_copy(b: &mut Criterion) {
    b.bench_function("winnow_maillog_copy", |b| {
        b.iter(|| winnow_maillog_copy(TEXT2))
    });
}

criterion_group!(
    regex,
    // unamed_regex,
    // named_regex,
    // unamed_regex_tuple,
    // named_regex_tuple,
    // unamed_regex_owned_tuple,
    named_regex_owned_tuple,
    maillog_regex,
    maillog_winnow,
    maillog_winnow_copy,
);
criterion_main!(regex);
