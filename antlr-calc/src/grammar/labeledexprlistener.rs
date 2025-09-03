#![allow(nonstandard_style)]
// Generated from LabeledExpr.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::labeledexprparser::*;

pub trait LabeledExprListener<'input> : ParseTreeListener<'input,LabeledExprParserContextType>{
/**
 * Enter a parse tree produced by {@link LabeledExprParser#prog}.
 * @param ctx the parse tree
 */
fn enter_prog(&mut self, _ctx: &ProgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LabeledExprParser#prog}.
 * @param ctx the parse tree
 */
fn exit_prog(&mut self, _ctx: &ProgContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code printExpr}
 * labeled alternative in {@link LabeledExprParser#stat}.
 * @param ctx the parse tree
 */
fn enter_printExpr(&mut self, _ctx: &PrintExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code printExpr}
 * labeled alternative in {@link LabeledExprParser#stat}.
 * @param ctx the parse tree
 */
fn exit_printExpr(&mut self, _ctx: &PrintExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code assign}
 * labeled alternative in {@link LabeledExprParser#stat}.
 * @param ctx the parse tree
 */
fn enter_assign(&mut self, _ctx: &AssignContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code assign}
 * labeled alternative in {@link LabeledExprParser#stat}.
 * @param ctx the parse tree
 */
fn exit_assign(&mut self, _ctx: &AssignContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code blank}
 * labeled alternative in {@link LabeledExprParser#stat}.
 * @param ctx the parse tree
 */
fn enter_blank(&mut self, _ctx: &BlankContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code blank}
 * labeled alternative in {@link LabeledExprParser#stat}.
 * @param ctx the parse tree
 */
fn exit_blank(&mut self, _ctx: &BlankContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parens}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn enter_parens(&mut self, _ctx: &ParensContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parens}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn exit_parens(&mut self, _ctx: &ParensContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code MulDiv}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn enter_MulDiv(&mut self, _ctx: &MulDivContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code MulDiv}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn exit_MulDiv(&mut self, _ctx: &MulDivContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AddSub}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn enter_AddSub(&mut self, _ctx: &AddSubContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AddSub}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn exit_AddSub(&mut self, _ctx: &AddSubContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code id}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn enter_id(&mut self, _ctx: &IdContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code id}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn exit_id(&mut self, _ctx: &IdContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code int}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn enter_int(&mut self, _ctx: &IntContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code int}
 * labeled alternative in {@link LabeledExprParser#expr}.
 * @param ctx the parse tree
 */
fn exit_int(&mut self, _ctx: &IntContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : LabeledExprListener<'input> }


