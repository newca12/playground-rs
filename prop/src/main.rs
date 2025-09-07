use std::result;

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::token_factory::CommonTokenFactory;
use antlr4rust::{tree::ParseTree, InputStream};
use prop::{propcalcLexer, propcalcParser};


fn main() {
    let input = r#"( p v q) |- (q v p)"#.to_owned();
    let tf = CommonTokenFactory::default();
    let lexer = propcalcLexer::new_with_token_factory(InputStream::new(&*input), &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = propcalcParser::new(token_source);
    let tree = parser.proposition().expect("parsed unsuccessfully");
    println!("{}", tree.to_string_tree(&*parser));
}
