// Generated from Math.g4 by ANTLR 4.13.2

use super::mathparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by MathParser.

pub trait MathBaseListener<'input>:
    ParseTreeListener<'input, MathParserContextType> {

    /**
     * Enter a parse tree produced by \{@link MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_compileunit(&mut self, _ctx: &CompileUnitContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_compileunit(&mut self, _ctx: &CompileUnitContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_infixexpr(&mut self, _ctx: &InfixExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_infixexpr(&mut self, _ctx: &InfixExprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unaryexpr(&mut self, _ctx: &UnaryExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unaryexpr(&mut self, _ctx: &UnaryExprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_funcexpr(&mut self, _ctx: &FuncExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_funcexpr(&mut self, _ctx: &FuncExprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numberexpr(&mut self, _ctx: &NumberExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numberexpr(&mut self, _ctx: &NumberExprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parensexpr(&mut self, _ctx: &ParensExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  MathBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parensexpr(&mut self, _ctx: &ParensExprContext<'input>) {}


}