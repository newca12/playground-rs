
// Generated from Math.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::mathparser::*;

// A complete Visitor for a parse tree produced by MathParser.

pub trait MathBaseVisitor<'input>:
    ParseTreeVisitor<'input, MathParserContextType> {
	// Visit a parse tree produced by MathParser#compileUnit.
	fn visit_compileunit(&mut self, ctx: &CompileUnitContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by MathParser#infixExpr.
	fn visit_infixexpr(&mut self, ctx: &InfixExprContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by MathParser#unaryExpr.
	fn visit_unaryexpr(&mut self, ctx: &UnaryExprContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by MathParser#funcExpr.
	fn visit_funcexpr(&mut self, ctx: &FuncExprContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by MathParser#numberExpr.
	fn visit_numberexpr(&mut self, ctx: &NumberExprContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by MathParser#parensExpr.
	fn visit_parensexpr(&mut self, ctx: &ParensExprContext<'input>) {
            self.visit_children(ctx)
        }

}