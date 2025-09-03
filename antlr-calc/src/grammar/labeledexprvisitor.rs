#![allow(nonstandard_style)]
// Generated from LabeledExpr.g4 by ANTLR 4.13.2
use antlr4rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
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

pub trait LabeledExprVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= LabeledExprParserContextType>{
	/**
	 * Visit a parse tree produced by {@link LabeledExprParser#prog}.
	 * @param ctx the parse tree
	 */
		fn visit_prog(&mut self, ctx: &ProgContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code printExpr}
	 * labeled alternative in {@link LabeledExprParser#stat}.
	 * @param ctx the parse tree
	 */
		fn visit_printExpr(&mut self, ctx: &PrintExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code assign}
	 * labeled alternative in {@link LabeledExprParser#stat}.
	 * @param ctx the parse tree
	 */
		fn visit_assign(&mut self, ctx: &AssignContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code blank}
	 * labeled alternative in {@link LabeledExprParser#stat}.
	 * @param ctx the parse tree
	 */
		fn visit_blank(&mut self, ctx: &BlankContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parens}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_parens(&mut self, ctx: &ParensContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code MulDiv}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_MulDiv(&mut self, ctx: &MulDivContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code AddSub}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_AddSub(&mut self, ctx: &AddSubContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code id}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_id(&mut self, ctx: &IdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code int}
	 * labeled alternative in {@link LabeledExprParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_int(&mut self, ctx: &IntContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> LabeledExprVisitor<'input> for T
where
	T: LabeledExprVisitorCompat<'input>
{
	fn visit_prog(&mut self, ctx: &ProgContext<'input>){
		let result = <Self as LabeledExprVisitorCompat>::visit_prog(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_printExpr(&mut self, ctx: &PrintExprContext<'input>){
		let result = <Self as LabeledExprVisitorCompat>::visit_printExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assign(&mut self, ctx: &AssignContext<'input>){
		let result = <Self as LabeledExprVisitorCompat>::visit_assign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blank(&mut self, ctx: &BlankContext<'input>){
		let result = <Self as LabeledExprVisitorCompat>::visit_blank(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parens(&mut self, ctx: &ParensContext<'input>){
		let result = <Self as LabeledExprVisitorCompat>::visit_parens(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_MulDiv(&mut self, ctx: &MulDivContext<'input>){
		let result = <Self as LabeledExprVisitorCompat>::visit_MulDiv(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_AddSub(&mut self, ctx: &AddSubContext<'input>){
		let result = <Self as LabeledExprVisitorCompat>::visit_AddSub(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_id(&mut self, ctx: &IdContext<'input>){
		let result = <Self as LabeledExprVisitorCompat>::visit_id(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_int(&mut self, ctx: &IntContext<'input>){
		let result = <Self as LabeledExprVisitorCompat>::visit_int(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}