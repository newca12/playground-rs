
// Generated from LabeledExpr.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::labeledexprparser::*;

// A complete Visitor for a parse tree produced by LabeledExprParser.

pub trait LabeledExprBaseVisitor<'input>:
    ParseTreeVisitor<'input, LabeledExprParserContextType> {
	// Visit a parse tree produced by LabeledExprParser#prog.
	fn visit_prog(&mut self, ctx: &ProgContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by LabeledExprParser#printExpr.
	fn visit_printexpr(&mut self, ctx: &PrintExprContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by LabeledExprParser#assign.
	fn visit_assign(&mut self, ctx: &AssignContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by LabeledExprParser#blank.
	fn visit_blank(&mut self, ctx: &BlankContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by LabeledExprParser#parens.
	fn visit_parens(&mut self, ctx: &ParensContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by LabeledExprParser#MulDiv.
	fn visit_muldiv(&mut self, ctx: &MulDivContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by LabeledExprParser#AddSub.
	fn visit_addsub(&mut self, ctx: &AddSubContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by LabeledExprParser#id.
	fn visit_id(&mut self, ctx: &IdContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by LabeledExprParser#int.
	fn visit_int(&mut self, ctx: &IntContext<'input>) {
            self.visit_children(ctx)
        }

}