#![allow(nonstandard_style)]
// Generated from LabeledExpr.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::labeledexprparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link LabeledExprParser}.
 */
pub trait LabeledExprVisitor<'input>: ParseTreeVisitor<'input,LabeledExprParserContextType>{
	/**
	 * Visit a parse tree produced by {@link LabeledExprParser#prog}.
	 * @param ctx the parse tree
	 */
	fn visit_prog(&mut self, ctx: &ProgContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code printExpr}
	 * labeled alternative in {@link LabeledExprParser#stat}.
	 * @param ctx the parse tree
	 */
	fn visit_printExpr(&mut self, ctx: &PrintExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code assign}
	 * labeled alternative in {@link LabeledExprParser#stat}.
	 * @param ctx the parse tree
	 */
	fn visit_assign(&mut self, ctx: &AssignContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code blank}
	 * labeled alternative in {@link LabeledExprParser#stat}.
	 * @param ctx the parse tree
	 */
	fn visit_blank(&mut self, ctx: &BlankContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parens}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_parens(&mut self, ctx: &ParensContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code MulDiv}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_MulDiv(&mut self, ctx: &MulDivContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code AddSub}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_AddSub(&mut self, ctx: &AddSubContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code id}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_id(&mut self, ctx: &IdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code int}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_int(&mut self, ctx: &IntContext<'input>) { self.visit_children(ctx) }


}