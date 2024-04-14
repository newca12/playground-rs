pub mod regex;
pub mod winnow;

#[cfg(test)]
mod test_commons;

pub static TEXT: &str = "Aug 29 10:01:02 server0303 postfix_auth/smtpd_abo[27339] to=<server000000000000000003435583@1100.back02-mail01-01.null.org>, orig_to=<john@doe.org>, status=sent";
pub static TEXT2: &str = "2024-03-04T12:02:32.509549+01:00 server0101 postfix_auth/smtps/smtpd[221317]: 4TpG4S3ZxfzFpTZ: client=lfbn-idf2-1-974-19.w86-238.null.org[1.2.3.4]:53192, sasl_method=PLAIN, sasl_username=john@doe.org";

#[derive(Debug, PartialEq)]
pub struct CommonPrefix<'a> {
    pub date: &'a str,
    pub server: &'a str,
    pub process: &'a str,
    pub pid: u32,
}

#[derive(Debug, PartialEq)]
pub struct CommonCopyPrefix {
    pub date: String,
    pub server: String,
    pub process: String,
    pub pid: u32,
}

#[derive(Debug, PartialEq)]
pub struct SmtpdClientLogin<'a> {
    pub qid: &'a str,
    pub domain: &'a str,
    pub ip: &'a str,
    pub sasl_method: &'a str,
    pub sasl_username: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct SmtpdClientLoginCopy {
    pub qid: String,
    pub domain: String,
    pub ip: String,
    pub sasl_method: String,
    pub sasl_username: String,
}
