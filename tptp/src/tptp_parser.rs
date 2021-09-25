#![feature(try_blocks)]
#![feature(inner_deref)]
#![feature(specialization)]
#![feature(coerce_unsized)]

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token::Token;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::{ParseTree, ParseTreeListener, ParseTreeVisitor};
use antlr_rust::InputStream;

use crate::grammar::tptp_v7_0_0_0parser::tptp_v7_0_0_0ParserContextType;
use crate::{
    tptp_v7_0_0_0Lexer, tptp_v7_0_0_0Listener, tptp_v7_0_0_0Parser, tptp_v7_0_0_0ParserContext,
    tptp_v7_0_0_0parser, Tptp_inputContextAttrs,
};

pub fn str_to_visit<'input>(input: &str) -> () {
    let tf = CommonTokenFactory::default();
    let mut lexer = tptp_v7_0_0_0Lexer::new_with_token_factory(InputStream::new(&*input), &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = tptp_v7_0_0_0Parser::new(token_source);
    parser.add_parse_listener(Box::new(TptpListener));
    let result = parser.tptp_input().expect("parsed unsuccessfully");
    println!(
        "{:#?}",
        result.annotated_formula().unwrap().to_string_tree(&*parser)
    );
    //let listener = tptp_v7_0_0_0TreeWalker::walk(Box::new(TptpListener), &*result);
    ()
}

/*
type Row = Vec<String>;

#[derive(Debug)]
struct CSV {
    header: Row,
    rows: Vec<Row>,
}

struct Listener {
    csv: Box<CSV>,
}

impl Listener {
    fn hdr(&self, ctx: &Tptp_fileContext) -> Row {
        let row_ctx = ctx.tptp_input_all().unwrap();
        self.row(&row_ctx)
    }

    fn row(&self, ctx: &Tptp_fileContext) -> Row {
        let mut row = Row::new();
        let field_ctx_list = ctx.tptp_input_all();
        for (_i, field_ctx) in field_ctx_list.iter().enumerate() {
            let field = self.field(&field_ctx);
            row.push(field);
        }
        row
    }

    fn field(&self, ctx: &Tptp_fileContext) -> String {
        ctx.get_text()
    }
}


impl ParseTreeListener for Listener {}

impl tptp_v7_0_0_0Listener for Listener {
    fn exit_csvFile(&mut self, ctx: &CsvFileContext) {
        let hdr_ctx = ctx.hdr().unwrap();
        let header = self.hdr(&hdr_ctx);
        self.csv.header = header;
        let row_ctx_list = ctx.row_all();
        for (_i, row_ctx) in row_ctx_list.iter().enumerate() {
            let row = self.row(&row_ctx);
            self.csv.rows.push(row);
        }
    }
}
*/

struct TptpListener;

impl<'input> ParseTreeListener<'input, tptp_v7_0_0_0ParserContextType> for TptpListener {
    fn visit_terminal(
        &mut self,
        node: &antlr_rust::tree::TerminalNode<'input, tptp_v7_0_0_0ParserContextType>,
    ) {
        println!("terminal node {:#?}", node.symbol.get_text());
    }

    fn enter_every_rule(&mut self, ctx: &dyn tptp_v7_0_0_0ParserContext<'input>) {
        println!(
            "rule entered {}",
            tptp_v7_0_0_0parser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }

    fn exit_every_rule(&mut self, ctx: &dyn tptp_v7_0_0_0ParserContext<'input>) {
        println!(
            "rule exited {}",
            tptp_v7_0_0_0parser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }
}

impl<'input> tptp_v7_0_0_0Listener<'input> for TptpListener {}
