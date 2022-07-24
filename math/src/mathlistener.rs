#![allow(nonstandard_style)]
// Generated from Math.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::mathparser::*;

pub trait MathListener<'input> : ParseTreeListener<'input,MathParserContextType>{
/**
 * Enter a parse tree produced by {@link MathParser#compileUnit}.
 * @param ctx the parse tree
 */
fn enter_compileUnit(&mut self, _ctx: &CompileUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link MathParser#compileUnit}.
 * @param ctx the parse tree
 */
fn exit_compileUnit(&mut self, _ctx: &CompileUnitContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code infixExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_infixExpr(&mut self, _ctx: &InfixExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code infixExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_infixExpr(&mut self, _ctx: &InfixExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unaryExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_unaryExpr(&mut self, _ctx: &UnaryExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unaryExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_unaryExpr(&mut self, _ctx: &UnaryExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code funcExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_funcExpr(&mut self, _ctx: &FuncExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code funcExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_funcExpr(&mut self, _ctx: &FuncExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numberExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_numberExpr(&mut self, _ctx: &NumberExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numberExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_numberExpr(&mut self, _ctx: &NumberExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parensExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn enter_parensExpr(&mut self, _ctx: &ParensExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parensExpr}
 * labeled alternative in {@link MathParser#expr}.
 * @param ctx the parse tree
 */
fn exit_parensExpr(&mut self, _ctx: &ParensExprContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : MathListener<'input> }


