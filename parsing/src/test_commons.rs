#[cfg(test)]
pub mod tests {
    use crate::{Common, CommonCopy, SmtpdClientLogin, SmtpdClientLoginCopy};

    pub fn check(tuple: (&str, &str, &str, &str, &str, &str, &str)) {
        assert_eq!(tuple.0, "Aug 29 10:01:02");
        assert_eq!(tuple.1, "server0303");
        assert_eq!(tuple.2, "postfix_auth/smtpd_abo");
        assert_eq!(tuple.3, "27339");
        assert_eq!(
            tuple.4,
            "server000000000000000003435583@1100.back02-mail01-01.null.org"
        );
        assert_eq!(tuple.5, "john@doe.org");
        assert_eq!(tuple.6, "sent");
    }

    pub fn check_s(tuple: (String, String, String, String, String, String, String)) {
        assert_eq!(tuple.0, "Aug 29 10:01:02");
        assert_eq!(tuple.1, "server0303");
        assert_eq!(tuple.2, "postfix_auth/smtpd_abo");
        assert_eq!(tuple.3, "27339");
        assert_eq!(
            tuple.4,
            "server000000000000000003435583@1100.back02-mail01-01.null.org"
        );
        assert_eq!(tuple.5, "john@doe.org");
        assert_eq!(tuple.6, "sent");
    }

    pub fn check_common(common: Common) {
        assert_eq!(common.timestamp, "2024-03-04T12:02:32.509549+01:00");
        assert_eq!(common.server, "server0101");
        assert_eq!(common.prefix, "postfix_auth");
        assert_eq!(common.process, "smtps/smtpd");
        assert_eq!(common.pid, Some("221317"))
    }

    pub fn check_smtpd_client_login(smtpd_client_login: SmtpdClientLogin) {
        assert_eq!(smtpd_client_login.qid, "4TpG4S3ZxfzFpTZ");
        assert_eq!(
            smtpd_client_login.domain,
            "lfbn-idf2-1-974-19.w86-238.null.org"
        );
        assert_eq!(smtpd_client_login.ip, "1.2.3.4");
        assert_eq!(smtpd_client_login.sasl_method, "PLAIN");
        assert_eq!(smtpd_client_login.sasl_username, "john@doe.org");
    }

    pub fn check_common_copy(common: CommonCopy) {
        assert_eq!(common.timestamp, "2024-03-04T12:02:32.509549+01:00");
        assert_eq!(common.server, "server0101");
        assert_eq!(common.prefix, "postfix_auth");
        assert_eq!(common.process, "smtps/smtpd");
        assert_eq!(common.pid, Some("221317".to_string()))
    }

    pub fn check_smtpd_client_login_copy(smtpd_client_login: SmtpdClientLoginCopy) {
        assert_eq!(smtpd_client_login.qid, "4TpG4S3ZxfzFpTZ");
        assert_eq!(
            smtpd_client_login.domain,
            "lfbn-idf2-1-974-19.w86-238.null.org"
        );
        assert_eq!(smtpd_client_login.ip, "1.2.3.4");
        assert_eq!(smtpd_client_login.sasl_method, "PLAIN");
        assert_eq!(smtpd_client_login.sasl_username, "john@doe.org");
    }
}
