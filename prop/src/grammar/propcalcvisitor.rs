#![allow(nonstandard_style)]
// Generated from propcalc.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::propcalcparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link propcalcParser}.
 */
pub trait propcalcVisitor<'input>: ParseTreeVisitor<'input,propcalcParserContextType>{
	/**
	 * Visit a parse tree produced by {@link propcalcParser#proposition}.
	 * @param ctx the parse tree
	 */
	fn visit_proposition(&mut self, ctx: &PropositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link propcalcParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link propcalcParser#relExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_relExpression(&mut self, ctx: &RelExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link propcalcParser#atoms}.
	 * @param ctx the parse tree
	 */
	fn visit_atoms(&mut self, ctx: &AtomsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link propcalcParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atom(&mut self, ctx: &AtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link propcalcParser#equiv}.
	 * @param ctx the parse tree
	 */
	fn visit_equiv(&mut self, ctx: &EquivContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link propcalcParser#implies}.
	 * @param ctx the parse tree
	 */
	fn visit_implies(&mut self, ctx: &ImpliesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link propcalcParser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

}

pub trait propcalcVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= propcalcParserContextType>{
	/**
	 * Visit a parse tree produced by {@link propcalcParser#proposition}.
	 * @param ctx the parse tree
	 */
		fn visit_proposition(&mut self, ctx: &PropositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link propcalcParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link propcalcParser#relExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_relExpression(&mut self, ctx: &RelExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link propcalcParser#atoms}.
	 * @param ctx the parse tree
	 */
		fn visit_atoms(&mut self, ctx: &AtomsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link propcalcParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link propcalcParser#equiv}.
	 * @param ctx the parse tree
	 */
		fn visit_equiv(&mut self, ctx: &EquivContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link propcalcParser#implies}.
	 * @param ctx the parse tree
	 */
		fn visit_implies(&mut self, ctx: &ImpliesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link propcalcParser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> propcalcVisitor<'input> for T
where
	T: propcalcVisitorCompat<'input>
{
	fn visit_proposition(&mut self, ctx: &PropositionContext<'input>){
		let result = <Self as propcalcVisitorCompat>::visit_proposition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as propcalcVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relExpression(&mut self, ctx: &RelExpressionContext<'input>){
		let result = <Self as propcalcVisitorCompat>::visit_relExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atoms(&mut self, ctx: &AtomsContext<'input>){
		let result = <Self as propcalcVisitorCompat>::visit_atoms(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atom(&mut self, ctx: &AtomContext<'input>){
		let result = <Self as propcalcVisitorCompat>::visit_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equiv(&mut self, ctx: &EquivContext<'input>){
		let result = <Self as propcalcVisitorCompat>::visit_equiv(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_implies(&mut self, ctx: &ImpliesContext<'input>){
		let result = <Self as propcalcVisitorCompat>::visit_implies(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as propcalcVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}