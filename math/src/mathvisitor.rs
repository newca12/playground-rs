#![allow(nonstandard_style)]
// Generated from Math.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::mathparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link MathParser}.
 */
pub trait MathVisitor<'input>: ParseTreeVisitor<'input,MathParserContextType>{
	/**
	 * Visit a parse tree produced by {@link MathParser#compileUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_compileUnit(&mut self, ctx: &CompileUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code infixExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_infixExpr(&mut self, ctx: &InfixExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code unaryExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpr(&mut self, ctx: &UnaryExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code funcExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_funcExpr(&mut self, ctx: &FuncExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numberExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_numberExpr(&mut self, ctx: &NumberExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parensExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_parensExpr(&mut self, ctx: &ParensExprContext<'input>) { self.visit_children(ctx) }

}

pub trait MathVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= MathParserContextType>{
	/**
	 * Visit a parse tree produced by {@link MathParser#compileUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_compileUnit(&mut self, ctx: &CompileUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code infixExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_infixExpr(&mut self, ctx: &InfixExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code unaryExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExpr(&mut self, ctx: &UnaryExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code funcExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_funcExpr(&mut self, ctx: &FuncExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numberExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_numberExpr(&mut self, ctx: &NumberExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parensExpr}
	 * labeled alternative in {@link MathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_parensExpr(&mut self, ctx: &ParensExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> MathVisitor<'input> for T
where
	T: MathVisitorCompat<'input>
{
	fn visit_compileUnit(&mut self, ctx: &CompileUnitContext<'input>){
		let result = <Self as MathVisitorCompat>::visit_compileUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_infixExpr(&mut self, ctx: &InfixExprContext<'input>){
		let result = <Self as MathVisitorCompat>::visit_infixExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExpr(&mut self, ctx: &UnaryExprContext<'input>){
		let result = <Self as MathVisitorCompat>::visit_unaryExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_funcExpr(&mut self, ctx: &FuncExprContext<'input>){
		let result = <Self as MathVisitorCompat>::visit_funcExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberExpr(&mut self, ctx: &NumberExprContext<'input>){
		let result = <Self as MathVisitorCompat>::visit_numberExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parensExpr(&mut self, ctx: &ParensExprContext<'input>){
		let result = <Self as MathVisitorCompat>::visit_parensExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}