// Generated from propcalc.g4 by ANTLR 4.13.2

use super::propcalcparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by propcalcParser.

pub trait propcalcBaseListener<'input>:
    ParseTreeListener<'input, propcalcParserContextType> {

    /**
     * Enter a parse tree produced by \{@link propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_proposition(&mut self, _ctx: &PropositionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_proposition(&mut self, _ctx: &PropositionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_relexpression(&mut self, _ctx: &RelExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_relexpression(&mut self, _ctx: &RelExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atoms(&mut self, _ctx: &AtomsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atoms(&mut self, _ctx: &AtomsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atom(&mut self, _ctx: &AtomContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom(&mut self, _ctx: &AtomContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_equiv(&mut self, _ctx: &EquivContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_equiv(&mut self, _ctx: &EquivContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_implies(&mut self, _ctx: &ImpliesContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_implies(&mut self, _ctx: &ImpliesContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  propcalcBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_variable(&mut self, _ctx: &VariableContext<'input>) {}


}