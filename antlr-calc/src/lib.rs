#![feature(try_blocks)]

pub use calc_parser::str_to_calc;

pub use calc_parser_v3::str_to_calc_v3;
use calc_parser_v3::Calc3;
pub use grammar::*;

pub mod calc_parser;
pub mod calc_parser_v3;
pub mod grammar;

pub fn parse(str: &str) -> Calc3 {
    str_to_calc_v3(str)
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn should_parse_assign() {
        let data = r#"a=2
"#;

        let calc = parse(data);
        assert_eq!(2i64, *calc.memory.get("a").unwrap());
    }
}
