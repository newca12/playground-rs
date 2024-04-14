use winnow::{
    ascii::space1,
    combinator::{delimited, opt, preceded},
    error::StrContext,
    prelude::*,
    seq,
    stream::AsChar,
    token::take_till,
};

use crate::{Common, CommonCopy, SmtpdClientLogin, SmtpdClientLoginCopy, TEXT2};

pub fn winnow_sample() {
    let data = TEXT2;
    println!("winnow {:?}", winnow_maillog(data));
    println!("winnow copy {:?}", winnow_maillog_copy(data));
}

pub fn winnow_maillog_copy(s: &str) -> (CommonCopy, SmtpdClientLoginCopy) {
    let mut data = s;
    let (common, client_login) = (parser_common_copy, parser_client_login_copy)
        .parse_next(&mut data)
        .unwrap();
    (common, client_login)
}

pub fn winnow_maillog(s: &str) -> (Common, SmtpdClientLogin) {
    let mut data = s;
    let (common, client_login) = (parser_common, parser_client_login)
        .parse_next(&mut data)
        .unwrap();
    (common, client_login)
}

fn parser_common<'a>(s: &mut &'a str) -> PResult<Common<'a>> {
    seq! {
        Common {
            timestamp: take_till(1.., AsChar::is_space).context(StrContext::Label("timestamp")),
            _: space1,
            server: take_till(1.., AsChar::is_space).context(StrContext::Label("server")),
            _: space1,
            prefix: take_till(1.., |x| x =='/').context(StrContext::Label("prefix")),
            _: "/",
            process: take_till(1.., |x| x =='[' || x == ':').context(StrContext::Label("process")),
            pid: opt(delimited("[", take_till(1.., |c| c==']'), "]")).context(StrContext::Label("pid")),
            _:":"
        }
    }.parse_next(s)
}

fn parser_common_copy(s: &mut &str) -> PResult<CommonCopy> {
    let timestamp = take_till(1.., AsChar::is_space)
        .context(StrContext::Label("timestamp"))
        .parse_next(s)?;
    space1.parse_next(s)?;
    let server = take_till(1.., AsChar::is_space)
        .context(StrContext::Label("server"))
        .parse_next(s)?;
    space1.parse_next(s)?;
    let prefix = take_till(1.., |x| x == '/')
        .context(StrContext::Label("prefix"))
        .parse_next(s)?;
    "/".parse_next(s)?;
    let process = take_till(1.., |x| x == '[' || x == ':')
        .context(StrContext::Label("process"))
        .parse_next(s)?;
    let pid = opt(delimited("[", take_till(1.., |c| c == ']'), "]"))
        .context(StrContext::Label("pid"))
        .parse_next(s)?;
    ":".parse_next(s)?;
    Ok(CommonCopy {
        timestamp: timestamp.to_string(),
        server: server.to_string(),
        prefix: prefix.to_string(),
        process: process.to_string(),
        pid: pid.map(|x| x.to_string()),
    })
}

fn parser_client_login_copy(s: &mut &str) -> PResult<SmtpdClientLoginCopy> {
    space1.parse_next(s)?;
    let qid = take_till(1.., |c| c == ':').parse_next(s)?;
    ": client=".parse_next(s)?;
    let domain = take_till(1.., |c| c == '[')
        .context(StrContext::Label("domain"))
        .parse_next(s)?;
    let ip = delimited("[", take_till(1.., |c| c == ']'), "]")
        .context(StrContext::Label("ip"))
        .parse_next(s)?;
    (":", take_till(1.., |c| c == ','), ", ").parse_next(s)?;
    let sasl_method = preceded("sasl_method=", take_till(1.., |c| c == ',')).parse_next(s)?;
    ", ".parse_next(s)?;
    let sasl_username = preceded("sasl_username=", take_till(1.., |c| c == ',')).parse_next(s)?;
    Ok(SmtpdClientLoginCopy {
        qid: qid.to_string(),
        domain: domain.to_string(),
        ip: ip.to_string(),
        sasl_method: sasl_method.to_string(),
        sasl_username: sasl_username.to_string(),
    })
}

fn parser_client_login<'a>(s: &mut &'a str) -> PResult<SmtpdClientLogin<'a>> {
    seq! {
        SmtpdClientLogin {
            _:space1,
            qid: take_till(1.., |c| c==':'),
            _: ": client=",
            domain: take_till(1.., |c| c=='[').context(StrContext::Label("domain")),
            ip: delimited("[", take_till(1.., |c| c==']'), "]").context(StrContext::Label("ip")),
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
        test_commons::tests::{
            check_common, check_common_copy, check_smtpd_client_login,
            check_smtpd_client_login_copy,
        },
        TEXT2,
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
