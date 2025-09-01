use winnow::{
    Result,
    ascii::{dec_uint, space1},
    combinator::{delimited, preceded},
    prelude::*,
    seq,
    token::take_till,
};

use crate::{CommonCopyPrefix, CommonPrefix, SmtpdClientLogin, SmtpdClientLoginCopy, TEXT2};

pub fn winnow_sample() {
    let data = TEXT2;
    println!("winnow {:?}", winnow_maillog(data));
    println!("winnow copy {:?}", winnow_maillog_copy(data));
}

pub fn winnow_maillog_copy(s: &str) -> (CommonCopyPrefix, SmtpdClientLoginCopy) {
    let mut data = s;
    let (common, client_login) = (parser_common_copy, parser_client_login_copy)
        .parse_next(&mut data)
        .unwrap();
    (common, client_login)
}

pub fn winnow_maillog(s: &'_ str) -> (CommonPrefix<'_>, SmtpdClientLogin<'_>) {
    let mut data = s;
    let (common, client_login) = (parser_common, parser_client_login)
        .parse_next(&mut data)
        .unwrap();
    (common, client_login)
}

fn parser_common<'a>(s: &mut &'a str) -> Result<CommonPrefix<'a>> {
    seq! {
        CommonPrefix {
            date: take_till(1.., |c| c==' '),
            _:" ",
            server: take_till(1.., |c| c==' '),
            _:" ",
            process: take_till(1.., |x| x =='[' || x == ':'),
            pid: delimited("[", dec_uint, "]"),
            _:":"
        }
    }
    .parse_next(s)
}

fn parser_common_copy(s: &mut &str) -> Result<CommonCopyPrefix> {
    seq! {
        CommonCopyPrefix {
            date: take_till(1.., |c| c==' ').map(|x: &str| x.to_string()),
            _:" ",
            server: take_till(1.., |c| c==' ').map(|x: &str| x.to_string()),
            _:" ",
            process: take_till(1.., |x| x =='[' || x == ':').map(|x: &str| x.to_string()),
            pid: delimited("[", dec_uint , "]"),
            _:":"
        }
    }
    .parse_next(s)
}

fn parser_client_login_copy(s: &mut &str) -> Result<SmtpdClientLoginCopy> {
    seq! {
        SmtpdClientLoginCopy {
            _:space1,
            qid: take_till(1.., |c| c==':').map(|x: &str| x.to_string()),
            _: ": client=",
            domain: take_till(1.., |c| c=='[').map(|x: &str| x.to_string()),
            ip: delimited("[", take_till(1.., |c| c==']'), "]").map(|x: &str| x.to_string()),
            _:(":",take_till(1..,|c| c==','),", "),
            sasl_method: preceded("sasl_method=", take_till(1..,|c| c==',')).map(|x: &str| x.to_string()),
            _: ", ",
            sasl_username: preceded("sasl_username=", take_till(1..,|c| c==',').map(|x: &str| x.to_string()))
        }
    }
    .parse_next(s)
}

fn parser_client_login<'a>(s: &mut &'a str) -> Result<SmtpdClientLogin<'a>> {
    seq! {
        SmtpdClientLogin {
            _:space1,
            qid: take_till(1.., |c| c==':'),
            _: ": client=",
            domain: take_till(1.., |c| c=='['),
            ip: delimited("[", take_till(1.., |c| c==']'), "]"),
            _:(":",take_till(1..,|c| c==','),", "),
            sasl_method: preceded("sasl_method=", take_till(1..,|c| c==',')),
            _: ", ",
            sasl_username: preceded("sasl_username=", take_till(1..,|c| c==','))
        }
    }
    .parse_next(s)
}

#[cfg(test)]
mod tests {
    use crate::{
        TEXT2,
        test_commons::tests::{
            check_common, check_common_copy, check_smtpd_client_login,
            check_smtpd_client_login_copy,
        },
    };

    use super::*;

    #[test]
    fn client_login_winnow() {
        let data = TEXT2;
        let (common, client_login) = winnow_maillog(data);
        check_common(common);
        check_smtpd_client_login(client_login);
    }

    #[test]
    fn client_login_winnow_copy() {
        let data = TEXT2;
        let (common, client_login) = winnow_maillog_copy(data);
        check_common_copy(common);
        check_smtpd_client_login_copy(client_login);
    }
}
