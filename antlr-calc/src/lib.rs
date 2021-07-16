#![feature(try_blocks)]

pub use calc_parser::*;
pub use grammar::*;

pub mod calc_parser;
pub mod grammar;

pub fn parse(str: &str) -> Calc {
    str_to_calc(str)
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
