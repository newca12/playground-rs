use antlr_rust::{
    common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory, InputStream,
};

use crate::mathlistener::MathListener;
use crate::mathvisitor::MathVisitor;
use mathlexer::MathLexer;
use mathparser::*;

mod mathlexer;
mod mathlistener;
mod mathparser;
mod mathvisitor;

fn main() {
    println!("Start math");
    let input = r#"(2+3)*4"#.to_owned();
    let tf = CommonTokenFactory::default();
    let mut lexer = MathLexer::new_with_token_factory(InputStream::new(&*input), &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = MathParser::new(token_source);
    //parser.add_parse_listener(Box::new(MathListener));
    let result = parser.compileUnit().expect("parsed unsuccessfully");
    println!("{:#?}", result.expr().unwrap().to_string_tree(&*parser));
    //let listener = MathTreeWalker::walk(parser, &*result);
}
