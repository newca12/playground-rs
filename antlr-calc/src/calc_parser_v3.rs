use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, Visitable};
use antlr_rust::InputStream;
use antlr_rust::{common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{AddSubContext, AddSubContextAttrs, MulDivContextAttrs, PrintExprContextAttrs};

use crate::{
    AssignContext, IdContext, IntContext, LabeledExprLexer, LabeledExprParser,
    LabeledExprParserContextType, LabeledExprVisitorCompat, MulDivContext,
};

use crate::PrintExprContext;

use crate::grammar::labeledexprparser::AssignContextAttrs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Calc3 {
    pub memory: HashMap<String, i64>,
    pub res: i64,
}

impl Default for Calc3 {
    fn default() -> Self {
        Calc3 {
            memory: HashMap::new(),
            res: 0,
        }
    }
}

pub struct CalcParser {
    pub(crate) calc: Calc3,
}

pub fn str_to_calc_v3<'input>(data: &str) -> Calc3 {
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

impl ParseTreeVisitorCompat<'_> for CalcParser {
    type Node = LabeledExprParserContextType;
    type Return = i64;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.calc.res
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        aggregate + &next
    }
}

impl LabeledExprVisitorCompat<'_> for CalcParser {
    fn visit_printExpr(&mut self, ctx: &PrintExprContext) -> Self::Return {
        let value = ctx.get_text();
        let res = self.visit(&*ctx.expr().unwrap());
        println!("{} = {:?}", value, res);
        self.calc.res
    }

    fn visit_assign(&mut self, ctx: &AssignContext) -> Self::Return {
        let id = ctx.ID().unwrap();
        let value = ctx.expr().unwrap().get_text();
        println!("{:?} = {}", id, value);
        self.visit(&*ctx.expr().unwrap());
        let v = if self.calc.memory.get(&value).is_none() {
            value.parse().unwrap()
        } else {
            *self.calc.memory.get(&value).unwrap()
        };
        self.calc.memory.insert(id.get_text(), v);
        v
    }

    fn visit_MulDiv(&mut self, ctx: &MulDivContext) -> Self::Return {
        let left = self.visit(&*ctx.expr(0).unwrap());
        let right = self.visit(&*ctx.expr(1).unwrap());
        if ctx.MUL().is_some() {
            left * right
        } else {
            left / right
        }
    }

    fn visit_AddSub(&mut self, ctx: &AddSubContext) -> Self::Return {
        let left = self.visit(&*ctx.expr(0).unwrap());
        let right = self.visit(&*ctx.expr(1).unwrap());
        if ctx.ADD().is_some() {
            left + right
        } else {
            left - right
        }
    }

    fn visit_id(&mut self, ctx: &IdContext) -> Self::Return {
        let value = ctx.get_text();
        if self.calc.memory.get(&value).is_none() {
            0
        } else {
            *self.calc.memory.get(&value).unwrap()
        }
    }

    fn visit_int(&mut self, ctx: &IntContext) -> Self::Return {
        ctx.get_text().parse().unwrap()
    }
}
