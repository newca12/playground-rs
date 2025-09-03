use antlr4rust::tree::{ParseTree, ParseTreeVisitor, Visitable};
use antlr4rust::InputStream;
use antlr4rust::{common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{AddSubContextAttrs, MulDivContextAttrs};

use crate::{
    AddSubContext, AssignContext, IdContext, IntContext, LabeledExprLexer, LabeledExprParser,
    LabeledExprParserContextType, LabeledExprVisitor, MulDivContext,
};

use crate::PrintExprContext;

use crate::grammar::labeledexprparser::AssignContextAttrs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Calc {
    pub memory: HashMap<String, i64>,
    pub stack: Vec<i64>,
}

impl Default for Calc {
    fn default() -> Self {
        Calc {
            memory: HashMap::new(),
            stack: Vec::new(),
        }
    }
}

pub struct CalcParser {
    pub(crate) calc: Calc,
}

pub fn str_to_calc<'input>(data: &str) -> Calc {
    let tf = CommonTokenFactory::default();
    let lexer = LabeledExprLexer::new_with_token_factory(InputStream::new(data.into()), &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = LabeledExprParser::new(token_source);
    let result = parser.prog().expect("parsed unsuccessfully");

    let mut calc_parser = CalcParser {
        calc: Default::default(),
    };

    result.accept(&mut calc_parser);

    calc_parser.calc
}

impl<'i> ParseTreeVisitor<'i, LabeledExprParserContextType> for CalcParser {}

impl<'i> LabeledExprVisitor<'i> for CalcParser {
    fn visit_printExpr(&mut self, ctx: &PrintExprContext) {
        let value = ctx.get_text();
        self.visit_children(ctx);
        println!("{} = {:?}", value, self.calc.stack.pop().unwrap());
        ()
    }

    //immutable assign is implemented
    fn visit_assign(&mut self, ctx: &AssignContext) {
        let id = ctx.ID().unwrap();
        let value = ctx.expr().unwrap().get_text();
        println!("{:?} = {}", id, value);
        self.visit_children(ctx);
        self.calc.stack.pop();
        self.calc.memory.insert(
            id.get_text(),
            if self.calc.memory.get(&value).is_none() {
                value.parse().unwrap()
            } else {
                *self.calc.memory.get(&value).unwrap()
            },
        );
        ()
    }

    fn visit_MulDiv(&mut self, ctx: &MulDivContext) {
        self.visit_children(ctx);
        let (right, left) = (self.calc.stack.pop(), self.calc.stack.pop());
        if ctx.MUL().is_some() {
            self.calc.stack.push(left.unwrap() * right.unwrap());
        } else {
            self.calc.stack.push(left.unwrap() / right.unwrap())
        }
        ()
    }

    fn visit_AddSub(&mut self, ctx: &AddSubContext) {
        self.visit_children(ctx);
        let (right, left) = (self.calc.stack.pop(), self.calc.stack.pop());
        if ctx.ADD().is_some() {
            self.calc.stack.push(left.unwrap() + right.unwrap());
        } else {
            self.calc.stack.push(left.unwrap() - right.unwrap())
        }
        ()
    }

    fn visit_id(&mut self, ctx: &IdContext) {
        let value = ctx.get_text();
        self.calc.stack.push(*self.calc.memory.get(&value).unwrap());
        self.visit_children(ctx);
        ()
    }

    fn visit_int(&mut self, ctx: &IntContext) {
        let value = ctx.get_text();
        self.calc.stack.push(value.parse::<i64>().unwrap());
        self.visit_children(ctx);
        ()
    }
}
