// Generated from LabeledExpr.g4 by ANTLR 4.13.2

use super::labeledexprparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by LabeledExprParser.

pub trait LabeledExprBaseListener<'input>:
    ParseTreeListener<'input, LabeledExprParserContextType> {

    /**
     * Enter a parse tree produced by \{@link LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_prog(&mut self, _ctx: &ProgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_prog(&mut self, _ctx: &ProgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_printexpr(&mut self, _ctx: &PrintExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_printexpr(&mut self, _ctx: &PrintExprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_assign(&mut self, _ctx: &AssignContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_assign(&mut self, _ctx: &AssignContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_blank(&mut self, _ctx: &BlankContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_blank(&mut self, _ctx: &BlankContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parens(&mut self, _ctx: &ParensContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parens(&mut self, _ctx: &ParensContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_muldiv(&mut self, _ctx: &MulDivContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_muldiv(&mut self, _ctx: &MulDivContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_addsub(&mut self, _ctx: &AddSubContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_addsub(&mut self, _ctx: &AddSubContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_id(&mut self, _ctx: &IdContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_id(&mut self, _ctx: &IdContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_int(&mut self, _ctx: &IntContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  LabeledExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_int(&mut self, _ctx: &IntContext<'input>) {}


}