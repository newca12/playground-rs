
// Generated from propcalc.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::propcalcparser::*;

// A complete Visitor for a parse tree produced by propcalcParser.

pub trait propcalcBaseVisitor<'input>:
    ParseTreeVisitor<'input, propcalcParserContextType> {
	// Visit a parse tree produced by propcalcParser#proposition.
	fn visit_proposition(&mut self, ctx: &PropositionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by propcalcParser#expression.
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by propcalcParser#relExpression.
	fn visit_relexpression(&mut self, ctx: &RelExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by propcalcParser#atoms.
	fn visit_atoms(&mut self, ctx: &AtomsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by propcalcParser#atom.
	fn visit_atom(&mut self, ctx: &AtomContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by propcalcParser#equiv.
	fn visit_equiv(&mut self, ctx: &EquivContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by propcalcParser#implies.
	fn visit_implies(&mut self, ctx: &ImpliesContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by propcalcParser#variable.
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
            self.visit_children(ctx)
        }

}