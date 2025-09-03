#![allow(nonstandard_style)]
// Generated from tptp_v7_0_0_0.g4 by ANTLR 4.13.2
use antlr4rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::tptp_v7_0_0_0parser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link tptp_v7_0_0_0Parser}.
 */
pub trait tptp_v7_0_0_0Visitor<'input>: ParseTreeVisitor<'input,tptp_v7_0_0_0ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tptp_file}.
	 * @param ctx the parse tree
	 */
	fn visit_tptp_file(&mut self, ctx: &Tptp_fileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tptp_input}.
	 * @param ctx the parse tree
	 */
	fn visit_tptp_input(&mut self, ctx: &Tptp_inputContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#annotated_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_annotated_formula(&mut self, ctx: &Annotated_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tpi_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_tpi_annotated(&mut self, ctx: &Tpi_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tpi_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tpi_formula(&mut self, ctx: &Tpi_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_annotated(&mut self, ctx: &Thf_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tfx_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_tfx_annotated(&mut self, ctx: &Tfx_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_annotated(&mut self, ctx: &Tff_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tcf_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_tcf_annotated(&mut self, ctx: &Tcf_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_annotated(&mut self, ctx: &Fof_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#cnf_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_cnf_annotated(&mut self, ctx: &Cnf_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#annotations}.
	 * @param ctx the parse tree
	 */
	fn visit_annotations(&mut self, ctx: &AnnotationsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#formula_role}.
	 * @param ctx the parse tree
	 */
	fn visit_formula_role(&mut self, ctx: &Formula_roleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_formula(&mut self, ctx: &Thf_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_logic_formula(&mut self, ctx: &Thf_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_binary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_binary_formula(&mut self, ctx: &Thf_binary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_binary_pair}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_binary_pair(&mut self, ctx: &Thf_binary_pairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_binary_tuple}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_binary_tuple(&mut self, ctx: &Thf_binary_tupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_or_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_or_formula(&mut self, ctx: &Thf_or_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_and_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_and_formula(&mut self, ctx: &Thf_and_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_apply_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_apply_formula(&mut self, ctx: &Thf_apply_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_unitary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unitary_formula(&mut self, ctx: &Thf_unitary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_quantified_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_quantified_formula(&mut self, ctx: &Thf_quantified_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_quantification}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_quantification(&mut self, ctx: &Thf_quantificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_variable_list}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_variable_list(&mut self, ctx: &Thf_variable_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_variable}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_variable(&mut self, ctx: &Thf_variableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_typed_variable}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_typed_variable(&mut self, ctx: &Thf_typed_variableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_unary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unary_formula(&mut self, ctx: &Thf_unary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_atom}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_atom(&mut self, ctx: &Thf_atomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_function}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_function(&mut self, ctx: &Thf_functionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_conn_term}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_conn_term(&mut self, ctx: &Thf_conn_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_conditional}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_conditional(&mut self, ctx: &Thf_conditionalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_let}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_let(&mut self, ctx: &Thf_letContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_arguments(&mut self, ctx: &Thf_argumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_type_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_type_formula(&mut self, ctx: &Thf_type_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_typeable_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_typeable_formula(&mut self, ctx: &Thf_typeable_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_subtype}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_subtype(&mut self, ctx: &Thf_subtypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_top_level_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_top_level_type(&mut self, ctx: &Thf_top_level_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_unitary_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unitary_type(&mut self, ctx: &Thf_unitary_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_apply_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_apply_type(&mut self, ctx: &Thf_apply_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_binary_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_binary_type(&mut self, ctx: &Thf_binary_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_mapping_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_mapping_type(&mut self, ctx: &Thf_mapping_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_xprod_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_xprod_type(&mut self, ctx: &Thf_xprod_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_union_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_union_type(&mut self, ctx: &Thf_union_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_sequent}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_sequent(&mut self, ctx: &Thf_sequentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_tuple}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_tuple(&mut self, ctx: &Thf_tupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_formula_list}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_formula_list(&mut self, ctx: &Thf_formula_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tfx_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tfx_formula(&mut self, ctx: &Tfx_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tfx_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tfx_logic_formula(&mut self, ctx: &Tfx_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_formula(&mut self, ctx: &Tff_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_logic_formula(&mut self, ctx: &Tff_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_binary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_binary_formula(&mut self, ctx: &Tff_binary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_binary_nonassoc(&mut self, ctx: &Tff_binary_nonassocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_binary_assoc}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_binary_assoc(&mut self, ctx: &Tff_binary_assocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_or_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_or_formula(&mut self, ctx: &Tff_or_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_and_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_and_formula(&mut self, ctx: &Tff_and_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_unitary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_unitary_formula(&mut self, ctx: &Tff_unitary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_quantified_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_quantified_formula(&mut self, ctx: &Tff_quantified_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_variable_list}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_variable_list(&mut self, ctx: &Tff_variable_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_variable}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_variable(&mut self, ctx: &Tff_variableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_typed_variable}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_typed_variable(&mut self, ctx: &Tff_typed_variableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_unary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_unary_formula(&mut self, ctx: &Tff_unary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_atomic_formula(&mut self, ctx: &Tff_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_conditional}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_conditional(&mut self, ctx: &Tff_conditionalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let(&mut self, ctx: &Tff_letContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term_defns}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let_term_defns(&mut self, ctx: &Tff_let_term_defnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term_list}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let_term_list(&mut self, ctx: &Tff_let_term_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term_defn}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let_term_defn(&mut self, ctx: &Tff_let_term_defnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term_binding}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let_term_binding(&mut self, ctx: &Tff_let_term_bindingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_formula_defns}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let_formula_defns(&mut self, ctx: &Tff_let_formula_defnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_formula_list}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let_formula_list(&mut self, ctx: &Tff_let_formula_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_formula_defn}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let_formula_defn(&mut self, ctx: &Tff_let_formula_defnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_formula_binding}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let_formula_binding(&mut self, ctx: &Tff_let_formula_bindingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_sequent}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_sequent(&mut self, ctx: &Tff_sequentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_formula_tuple}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_formula_tuple(&mut self, ctx: &Tff_formula_tupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_formula_tuple_list}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_formula_tuple_list(&mut self, ctx: &Tff_formula_tuple_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_typed_atom}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_typed_atom(&mut self, ctx: &Tff_typed_atomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_subtype}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_subtype(&mut self, ctx: &Tff_subtypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_top_level_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_top_level_type(&mut self, ctx: &Tff_top_level_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tf1_quantified_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tf1_quantified_type(&mut self, ctx: &Tf1_quantified_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_monotype}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_monotype(&mut self, ctx: &Tff_monotypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_unitary_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_unitary_type(&mut self, ctx: &Tff_unitary_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_atomic_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_atomic_type(&mut self, ctx: &Tff_atomic_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_type_arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_type_arguments(&mut self, ctx: &Tff_type_argumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_mapping_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_mapping_type(&mut self, ctx: &Tff_mapping_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_xprod_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_xprod_type(&mut self, ctx: &Tff_xprod_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tcf_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tcf_formula(&mut self, ctx: &Tcf_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tcf_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tcf_logic_formula(&mut self, ctx: &Tcf_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tcf_quantified_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tcf_quantified_formula(&mut self, ctx: &Tcf_quantified_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_formula(&mut self, ctx: &Fof_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_logic_formula(&mut self, ctx: &Fof_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_binary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_binary_formula(&mut self, ctx: &Fof_binary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_binary_nonassoc(&mut self, ctx: &Fof_binary_nonassocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_binary_assoc}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_binary_assoc(&mut self, ctx: &Fof_binary_assocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_or_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_or_formula(&mut self, ctx: &Fof_or_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_and_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_and_formula(&mut self, ctx: &Fof_and_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_unitary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_unitary_formula(&mut self, ctx: &Fof_unitary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_quantified_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_quantified_formula(&mut self, ctx: &Fof_quantified_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_variable_list}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_variable_list(&mut self, ctx: &Fof_variable_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_unary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_unary_formula(&mut self, ctx: &Fof_unary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_infix_unary}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_infix_unary(&mut self, ctx: &Fof_infix_unaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_atomic_formula(&mut self, ctx: &Fof_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_plain_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_plain_atomic_formula(&mut self, ctx: &Fof_plain_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_atomic_formula(&mut self, ctx: &Fof_defined_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_plain_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_plain_formula(&mut self, ctx: &Fof_defined_plain_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_infix_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_infix_formula(&mut self, ctx: &Fof_defined_infix_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_system_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_system_atomic_formula(&mut self, ctx: &Fof_system_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_plain_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_plain_term(&mut self, ctx: &Fof_plain_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_term(&mut self, ctx: &Fof_defined_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_atomic_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_atomic_term(&mut self, ctx: &Fof_defined_atomic_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_plain_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_plain_term(&mut self, ctx: &Fof_defined_plain_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_system_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_system_term(&mut self, ctx: &Fof_system_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_arguments(&mut self, ctx: &Fof_argumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_term(&mut self, ctx: &Fof_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_function_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_function_term(&mut self, ctx: &Fof_function_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_conditional_term}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_conditional_term(&mut self, ctx: &Tff_conditional_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_let_term(&mut self, ctx: &Tff_let_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_tuple_term}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_tuple_term(&mut self, ctx: &Tff_tuple_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_sequent}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_sequent(&mut self, ctx: &Fof_sequentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_formula_tuple}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_formula_tuple(&mut self, ctx: &Fof_formula_tupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_formula_tuple_list}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_formula_tuple_list(&mut self, ctx: &Fof_formula_tuple_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#cnf_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_cnf_formula(&mut self, ctx: &Cnf_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#cnf_disjunction}.
	 * @param ctx the parse tree
	 */
	fn visit_cnf_disjunction(&mut self, ctx: &Cnf_disjunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#cnf_literal}.
	 * @param ctx the parse tree
	 */
	fn visit_cnf_literal(&mut self, ctx: &Cnf_literalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_quantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_quantifier(&mut self, ctx: &Thf_quantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#th0_quantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_th0_quantifier(&mut self, ctx: &Th0_quantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#th1_quantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_th1_quantifier(&mut self, ctx: &Th1_quantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_pair_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_pair_connective(&mut self, ctx: &Thf_pair_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_unary_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unary_connective(&mut self, ctx: &Thf_unary_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#th1_unary_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_th1_unary_connective(&mut self, ctx: &Th1_unary_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_pair_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_pair_connective(&mut self, ctx: &Tff_pair_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_quantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_quantifier(&mut self, ctx: &Fof_quantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#binary_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_binary_connective(&mut self, ctx: &Binary_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#assoc_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_assoc_connective(&mut self, ctx: &Assoc_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#unary_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_unary_connective(&mut self, ctx: &Unary_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#type_constant}.
	 * @param ctx the parse tree
	 */
	fn visit_type_constant(&mut self, ctx: &Type_constantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#type_functor}.
	 * @param ctx the parse tree
	 */
	fn visit_type_functor(&mut self, ctx: &Type_functorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_type}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_type(&mut self, ctx: &Defined_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#system_type}.
	 * @param ctx the parse tree
	 */
	fn visit_system_type(&mut self, ctx: &System_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atom(&mut self, ctx: &AtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#untyped_atom}.
	 * @param ctx the parse tree
	 */
	fn visit_untyped_atom(&mut self, ctx: &Untyped_atomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_proposition}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_proposition(&mut self, ctx: &Defined_propositionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_predicate}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_predicate(&mut self, ctx: &Defined_predicateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_infix_pred}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_infix_pred(&mut self, ctx: &Defined_infix_predContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_constant(&mut self, ctx: &ConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#functor}.
	 * @param ctx the parse tree
	 */
	fn visit_functor(&mut self, ctx: &FunctorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#system_constant}.
	 * @param ctx the parse tree
	 */
	fn visit_system_constant(&mut self, ctx: &System_constantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#system_functor}.
	 * @param ctx the parse tree
	 */
	fn visit_system_functor(&mut self, ctx: &System_functorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_constant}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_constant(&mut self, ctx: &Defined_constantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_functor}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_functor(&mut self, ctx: &Defined_functorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_term}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_term(&mut self, ctx: &Defined_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#source}.
	 * @param ctx the parse tree
	 */
	fn visit_source(&mut self, ctx: &SourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#sources}.
	 * @param ctx the parse tree
	 */
	fn visit_sources(&mut self, ctx: &SourcesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#dag_source}.
	 * @param ctx the parse tree
	 */
	fn visit_dag_source(&mut self, ctx: &Dag_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_record}.
	 * @param ctx the parse tree
	 */
	fn visit_inference_record(&mut self, ctx: &Inference_recordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_rule}.
	 * @param ctx the parse tree
	 */
	fn visit_inference_rule(&mut self, ctx: &Inference_ruleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_parents}.
	 * @param ctx the parse tree
	 */
	fn visit_inference_parents(&mut self, ctx: &Inference_parentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#parent_list}.
	 * @param ctx the parse tree
	 */
	fn visit_parent_list(&mut self, ctx: &Parent_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#parent_info}.
	 * @param ctx the parse tree
	 */
	fn visit_parent_info(&mut self, ctx: &Parent_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#parent_details}.
	 * @param ctx the parse tree
	 */
	fn visit_parent_details(&mut self, ctx: &Parent_detailsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#internal_source}.
	 * @param ctx the parse tree
	 */
	fn visit_internal_source(&mut self, ctx: &Internal_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#intro_type}.
	 * @param ctx the parse tree
	 */
	fn visit_intro_type(&mut self, ctx: &Intro_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#external_source}.
	 * @param ctx the parse tree
	 */
	fn visit_external_source(&mut self, ctx: &External_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#file_source}.
	 * @param ctx the parse tree
	 */
	fn visit_file_source(&mut self, ctx: &File_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#file_info}.
	 * @param ctx the parse tree
	 */
	fn visit_file_info(&mut self, ctx: &File_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#theory}.
	 * @param ctx the parse tree
	 */
	fn visit_theory(&mut self, ctx: &TheoryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#theory_name}.
	 * @param ctx the parse tree
	 */
	fn visit_theory_name(&mut self, ctx: &Theory_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#creator_source}.
	 * @param ctx the parse tree
	 */
	fn visit_creator_source(&mut self, ctx: &Creator_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#creator_name}.
	 * @param ctx the parse tree
	 */
	fn visit_creator_name(&mut self, ctx: &Creator_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#optional_info}.
	 * @param ctx the parse tree
	 */
	fn visit_optional_info(&mut self, ctx: &Optional_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#useful_info}.
	 * @param ctx the parse tree
	 */
	fn visit_useful_info(&mut self, ctx: &Useful_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#info_items}.
	 * @param ctx the parse tree
	 */
	fn visit_info_items(&mut self, ctx: &Info_itemsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#info_item}.
	 * @param ctx the parse tree
	 */
	fn visit_info_item(&mut self, ctx: &Info_itemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#formula_item}.
	 * @param ctx the parse tree
	 */
	fn visit_formula_item(&mut self, ctx: &Formula_itemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#description_item}.
	 * @param ctx the parse tree
	 */
	fn visit_description_item(&mut self, ctx: &Description_itemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#iquote_item}.
	 * @param ctx the parse tree
	 */
	fn visit_iquote_item(&mut self, ctx: &Iquote_itemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_item}.
	 * @param ctx the parse tree
	 */
	fn visit_inference_item(&mut self, ctx: &Inference_itemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_status}.
	 * @param ctx the parse tree
	 */
	fn visit_inference_status(&mut self, ctx: &Inference_statusContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#status_value}.
	 * @param ctx the parse tree
	 */
	fn visit_status_value(&mut self, ctx: &Status_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_info}.
	 * @param ctx the parse tree
	 */
	fn visit_inference_info(&mut self, ctx: &Inference_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#assumptions_record}.
	 * @param ctx the parse tree
	 */
	fn visit_assumptions_record(&mut self, ctx: &Assumptions_recordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#refutation}.
	 * @param ctx the parse tree
	 */
	fn visit_refutation(&mut self, ctx: &RefutationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#new_symbol_record}.
	 * @param ctx the parse tree
	 */
	fn visit_new_symbol_record(&mut self, ctx: &New_symbol_recordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#new_symbol_list}.
	 * @param ctx the parse tree
	 */
	fn visit_new_symbol_list(&mut self, ctx: &New_symbol_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#principal_symbol}.
	 * @param ctx the parse tree
	 */
	fn visit_principal_symbol(&mut self, ctx: &Principal_symbolContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#include}.
	 * @param ctx the parse tree
	 */
	fn visit_include(&mut self, ctx: &IncludeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#formula_selection}.
	 * @param ctx the parse tree
	 */
	fn visit_formula_selection(&mut self, ctx: &Formula_selectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#name_list}.
	 * @param ctx the parse tree
	 */
	fn visit_name_list(&mut self, ctx: &Name_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_term}.
	 * @param ctx the parse tree
	 */
	fn visit_general_term(&mut self, ctx: &General_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_data}.
	 * @param ctx the parse tree
	 */
	fn visit_general_data(&mut self, ctx: &General_dataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_function}.
	 * @param ctx the parse tree
	 */
	fn visit_general_function(&mut self, ctx: &General_functionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#formula_data}.
	 * @param ctx the parse tree
	 */
	fn visit_formula_data(&mut self, ctx: &Formula_dataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_list}.
	 * @param ctx the parse tree
	 */
	fn visit_general_list(&mut self, ctx: &General_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_terms}.
	 * @param ctx the parse tree
	 */
	fn visit_general_terms(&mut self, ctx: &General_termsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#name}.
	 * @param ctx the parse tree
	 */
	fn visit_name(&mut self, ctx: &NameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#atomic_word}.
	 * @param ctx the parse tree
	 */
	fn visit_atomic_word(&mut self, ctx: &Atomic_wordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#atomic_defined_word}.
	 * @param ctx the parse tree
	 */
	fn visit_atomic_defined_word(&mut self, ctx: &Atomic_defined_wordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#atomic_system_word}.
	 * @param ctx the parse tree
	 */
	fn visit_atomic_system_word(&mut self, ctx: &Atomic_system_wordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_number(&mut self, ctx: &NumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#file_name}.
	 * @param ctx the parse tree
	 */
	fn visit_file_name(&mut self, ctx: &File_nameContext<'input>) { self.visit_children(ctx) }

}

pub trait tptp_v7_0_0_0VisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= tptp_v7_0_0_0ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tptp_file}.
	 * @param ctx the parse tree
	 */
		fn visit_tptp_file(&mut self, ctx: &Tptp_fileContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tptp_input}.
	 * @param ctx the parse tree
	 */
		fn visit_tptp_input(&mut self, ctx: &Tptp_inputContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#annotated_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_annotated_formula(&mut self, ctx: &Annotated_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tpi_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_tpi_annotated(&mut self, ctx: &Tpi_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tpi_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tpi_formula(&mut self, ctx: &Tpi_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_annotated(&mut self, ctx: &Thf_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tfx_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_tfx_annotated(&mut self, ctx: &Tfx_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_annotated(&mut self, ctx: &Tff_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tcf_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_tcf_annotated(&mut self, ctx: &Tcf_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_annotated(&mut self, ctx: &Fof_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#cnf_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_cnf_annotated(&mut self, ctx: &Cnf_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#annotations}.
	 * @param ctx the parse tree
	 */
		fn visit_annotations(&mut self, ctx: &AnnotationsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#formula_role}.
	 * @param ctx the parse tree
	 */
		fn visit_formula_role(&mut self, ctx: &Formula_roleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_formula(&mut self, ctx: &Thf_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_logic_formula(&mut self, ctx: &Thf_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_binary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_binary_formula(&mut self, ctx: &Thf_binary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_binary_pair}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_binary_pair(&mut self, ctx: &Thf_binary_pairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_binary_tuple}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_binary_tuple(&mut self, ctx: &Thf_binary_tupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_or_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_or_formula(&mut self, ctx: &Thf_or_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_and_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_and_formula(&mut self, ctx: &Thf_and_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_apply_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_apply_formula(&mut self, ctx: &Thf_apply_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_unitary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unitary_formula(&mut self, ctx: &Thf_unitary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_quantified_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_quantified_formula(&mut self, ctx: &Thf_quantified_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_quantification}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_quantification(&mut self, ctx: &Thf_quantificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_variable_list}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_variable_list(&mut self, ctx: &Thf_variable_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_variable}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_variable(&mut self, ctx: &Thf_variableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_typed_variable}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_typed_variable(&mut self, ctx: &Thf_typed_variableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_unary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unary_formula(&mut self, ctx: &Thf_unary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_atom}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_atom(&mut self, ctx: &Thf_atomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_function}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_function(&mut self, ctx: &Thf_functionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_conn_term}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_conn_term(&mut self, ctx: &Thf_conn_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_conditional}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_conditional(&mut self, ctx: &Thf_conditionalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_let}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_let(&mut self, ctx: &Thf_letContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_arguments(&mut self, ctx: &Thf_argumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_type_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_type_formula(&mut self, ctx: &Thf_type_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_typeable_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_typeable_formula(&mut self, ctx: &Thf_typeable_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_subtype}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_subtype(&mut self, ctx: &Thf_subtypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_top_level_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_top_level_type(&mut self, ctx: &Thf_top_level_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_unitary_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unitary_type(&mut self, ctx: &Thf_unitary_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_apply_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_apply_type(&mut self, ctx: &Thf_apply_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_binary_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_binary_type(&mut self, ctx: &Thf_binary_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_mapping_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_mapping_type(&mut self, ctx: &Thf_mapping_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_xprod_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_xprod_type(&mut self, ctx: &Thf_xprod_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_union_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_union_type(&mut self, ctx: &Thf_union_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_sequent}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_sequent(&mut self, ctx: &Thf_sequentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_tuple}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_tuple(&mut self, ctx: &Thf_tupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_formula_list}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_formula_list(&mut self, ctx: &Thf_formula_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tfx_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tfx_formula(&mut self, ctx: &Tfx_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tfx_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tfx_logic_formula(&mut self, ctx: &Tfx_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_formula(&mut self, ctx: &Tff_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_logic_formula(&mut self, ctx: &Tff_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_binary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_binary_formula(&mut self, ctx: &Tff_binary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_binary_nonassoc(&mut self, ctx: &Tff_binary_nonassocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_binary_assoc}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_binary_assoc(&mut self, ctx: &Tff_binary_assocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_or_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_or_formula(&mut self, ctx: &Tff_or_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_and_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_and_formula(&mut self, ctx: &Tff_and_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_unitary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_unitary_formula(&mut self, ctx: &Tff_unitary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_quantified_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_quantified_formula(&mut self, ctx: &Tff_quantified_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_variable_list}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_variable_list(&mut self, ctx: &Tff_variable_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_variable}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_variable(&mut self, ctx: &Tff_variableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_typed_variable}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_typed_variable(&mut self, ctx: &Tff_typed_variableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_unary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_unary_formula(&mut self, ctx: &Tff_unary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_atomic_formula(&mut self, ctx: &Tff_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_conditional}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_conditional(&mut self, ctx: &Tff_conditionalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let(&mut self, ctx: &Tff_letContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term_defns}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let_term_defns(&mut self, ctx: &Tff_let_term_defnsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term_list}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let_term_list(&mut self, ctx: &Tff_let_term_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term_defn}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let_term_defn(&mut self, ctx: &Tff_let_term_defnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term_binding}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let_term_binding(&mut self, ctx: &Tff_let_term_bindingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_formula_defns}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let_formula_defns(&mut self, ctx: &Tff_let_formula_defnsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_formula_list}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let_formula_list(&mut self, ctx: &Tff_let_formula_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_formula_defn}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let_formula_defn(&mut self, ctx: &Tff_let_formula_defnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_formula_binding}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let_formula_binding(&mut self, ctx: &Tff_let_formula_bindingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_sequent}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_sequent(&mut self, ctx: &Tff_sequentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_formula_tuple}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_formula_tuple(&mut self, ctx: &Tff_formula_tupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_formula_tuple_list}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_formula_tuple_list(&mut self, ctx: &Tff_formula_tuple_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_typed_atom}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_typed_atom(&mut self, ctx: &Tff_typed_atomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_subtype}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_subtype(&mut self, ctx: &Tff_subtypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_top_level_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_top_level_type(&mut self, ctx: &Tff_top_level_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tf1_quantified_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tf1_quantified_type(&mut self, ctx: &Tf1_quantified_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_monotype}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_monotype(&mut self, ctx: &Tff_monotypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_unitary_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_unitary_type(&mut self, ctx: &Tff_unitary_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_atomic_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_atomic_type(&mut self, ctx: &Tff_atomic_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_type_arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_type_arguments(&mut self, ctx: &Tff_type_argumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_mapping_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_mapping_type(&mut self, ctx: &Tff_mapping_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_xprod_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_xprod_type(&mut self, ctx: &Tff_xprod_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tcf_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tcf_formula(&mut self, ctx: &Tcf_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tcf_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tcf_logic_formula(&mut self, ctx: &Tcf_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tcf_quantified_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tcf_quantified_formula(&mut self, ctx: &Tcf_quantified_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_formula(&mut self, ctx: &Fof_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_logic_formula(&mut self, ctx: &Fof_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_binary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_binary_formula(&mut self, ctx: &Fof_binary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_binary_nonassoc(&mut self, ctx: &Fof_binary_nonassocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_binary_assoc}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_binary_assoc(&mut self, ctx: &Fof_binary_assocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_or_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_or_formula(&mut self, ctx: &Fof_or_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_and_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_and_formula(&mut self, ctx: &Fof_and_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_unitary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_unitary_formula(&mut self, ctx: &Fof_unitary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_quantified_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_quantified_formula(&mut self, ctx: &Fof_quantified_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_variable_list}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_variable_list(&mut self, ctx: &Fof_variable_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_unary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_unary_formula(&mut self, ctx: &Fof_unary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_infix_unary}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_infix_unary(&mut self, ctx: &Fof_infix_unaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_atomic_formula(&mut self, ctx: &Fof_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_plain_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_plain_atomic_formula(&mut self, ctx: &Fof_plain_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_atomic_formula(&mut self, ctx: &Fof_defined_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_plain_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_plain_formula(&mut self, ctx: &Fof_defined_plain_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_infix_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_infix_formula(&mut self, ctx: &Fof_defined_infix_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_system_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_system_atomic_formula(&mut self, ctx: &Fof_system_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_plain_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_plain_term(&mut self, ctx: &Fof_plain_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_term(&mut self, ctx: &Fof_defined_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_atomic_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_atomic_term(&mut self, ctx: &Fof_defined_atomic_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_defined_plain_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_plain_term(&mut self, ctx: &Fof_defined_plain_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_system_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_system_term(&mut self, ctx: &Fof_system_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_arguments(&mut self, ctx: &Fof_argumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_term(&mut self, ctx: &Fof_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_function_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_function_term(&mut self, ctx: &Fof_function_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_conditional_term}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_conditional_term(&mut self, ctx: &Tff_conditional_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_let_term}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_let_term(&mut self, ctx: &Tff_let_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_tuple_term}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_tuple_term(&mut self, ctx: &Tff_tuple_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_sequent}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_sequent(&mut self, ctx: &Fof_sequentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_formula_tuple}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_formula_tuple(&mut self, ctx: &Fof_formula_tupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_formula_tuple_list}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_formula_tuple_list(&mut self, ctx: &Fof_formula_tuple_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#cnf_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_cnf_formula(&mut self, ctx: &Cnf_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#cnf_disjunction}.
	 * @param ctx the parse tree
	 */
		fn visit_cnf_disjunction(&mut self, ctx: &Cnf_disjunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#cnf_literal}.
	 * @param ctx the parse tree
	 */
		fn visit_cnf_literal(&mut self, ctx: &Cnf_literalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_quantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_quantifier(&mut self, ctx: &Thf_quantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#th0_quantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_th0_quantifier(&mut self, ctx: &Th0_quantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#th1_quantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_th1_quantifier(&mut self, ctx: &Th1_quantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_pair_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_pair_connective(&mut self, ctx: &Thf_pair_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#thf_unary_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unary_connective(&mut self, ctx: &Thf_unary_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#th1_unary_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_th1_unary_connective(&mut self, ctx: &Th1_unary_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#tff_pair_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_pair_connective(&mut self, ctx: &Tff_pair_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#fof_quantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_quantifier(&mut self, ctx: &Fof_quantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#binary_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_binary_connective(&mut self, ctx: &Binary_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#assoc_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_assoc_connective(&mut self, ctx: &Assoc_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#unary_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_unary_connective(&mut self, ctx: &Unary_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#type_constant}.
	 * @param ctx the parse tree
	 */
		fn visit_type_constant(&mut self, ctx: &Type_constantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#type_functor}.
	 * @param ctx the parse tree
	 */
		fn visit_type_functor(&mut self, ctx: &Type_functorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_type}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_type(&mut self, ctx: &Defined_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#system_type}.
	 * @param ctx the parse tree
	 */
		fn visit_system_type(&mut self, ctx: &System_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#untyped_atom}.
	 * @param ctx the parse tree
	 */
		fn visit_untyped_atom(&mut self, ctx: &Untyped_atomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_proposition}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_proposition(&mut self, ctx: &Defined_propositionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_predicate}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_predicate(&mut self, ctx: &Defined_predicateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_infix_pred}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_infix_pred(&mut self, ctx: &Defined_infix_predContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_constant(&mut self, ctx: &ConstantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#functor}.
	 * @param ctx the parse tree
	 */
		fn visit_functor(&mut self, ctx: &FunctorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#system_constant}.
	 * @param ctx the parse tree
	 */
		fn visit_system_constant(&mut self, ctx: &System_constantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#system_functor}.
	 * @param ctx the parse tree
	 */
		fn visit_system_functor(&mut self, ctx: &System_functorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_constant}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_constant(&mut self, ctx: &Defined_constantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_functor}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_functor(&mut self, ctx: &Defined_functorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#defined_term}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_term(&mut self, ctx: &Defined_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#source}.
	 * @param ctx the parse tree
	 */
		fn visit_source(&mut self, ctx: &SourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#sources}.
	 * @param ctx the parse tree
	 */
		fn visit_sources(&mut self, ctx: &SourcesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#dag_source}.
	 * @param ctx the parse tree
	 */
		fn visit_dag_source(&mut self, ctx: &Dag_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_record}.
	 * @param ctx the parse tree
	 */
		fn visit_inference_record(&mut self, ctx: &Inference_recordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_rule}.
	 * @param ctx the parse tree
	 */
		fn visit_inference_rule(&mut self, ctx: &Inference_ruleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_parents}.
	 * @param ctx the parse tree
	 */
		fn visit_inference_parents(&mut self, ctx: &Inference_parentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#parent_list}.
	 * @param ctx the parse tree
	 */
		fn visit_parent_list(&mut self, ctx: &Parent_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#parent_info}.
	 * @param ctx the parse tree
	 */
		fn visit_parent_info(&mut self, ctx: &Parent_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#parent_details}.
	 * @param ctx the parse tree
	 */
		fn visit_parent_details(&mut self, ctx: &Parent_detailsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#internal_source}.
	 * @param ctx the parse tree
	 */
		fn visit_internal_source(&mut self, ctx: &Internal_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#intro_type}.
	 * @param ctx the parse tree
	 */
		fn visit_intro_type(&mut self, ctx: &Intro_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#external_source}.
	 * @param ctx the parse tree
	 */
		fn visit_external_source(&mut self, ctx: &External_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#file_source}.
	 * @param ctx the parse tree
	 */
		fn visit_file_source(&mut self, ctx: &File_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#file_info}.
	 * @param ctx the parse tree
	 */
		fn visit_file_info(&mut self, ctx: &File_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#theory}.
	 * @param ctx the parse tree
	 */
		fn visit_theory(&mut self, ctx: &TheoryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#theory_name}.
	 * @param ctx the parse tree
	 */
		fn visit_theory_name(&mut self, ctx: &Theory_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#creator_source}.
	 * @param ctx the parse tree
	 */
		fn visit_creator_source(&mut self, ctx: &Creator_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#creator_name}.
	 * @param ctx the parse tree
	 */
		fn visit_creator_name(&mut self, ctx: &Creator_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#optional_info}.
	 * @param ctx the parse tree
	 */
		fn visit_optional_info(&mut self, ctx: &Optional_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#useful_info}.
	 * @param ctx the parse tree
	 */
		fn visit_useful_info(&mut self, ctx: &Useful_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#info_items}.
	 * @param ctx the parse tree
	 */
		fn visit_info_items(&mut self, ctx: &Info_itemsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#info_item}.
	 * @param ctx the parse tree
	 */
		fn visit_info_item(&mut self, ctx: &Info_itemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#formula_item}.
	 * @param ctx the parse tree
	 */
		fn visit_formula_item(&mut self, ctx: &Formula_itemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#description_item}.
	 * @param ctx the parse tree
	 */
		fn visit_description_item(&mut self, ctx: &Description_itemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#iquote_item}.
	 * @param ctx the parse tree
	 */
		fn visit_iquote_item(&mut self, ctx: &Iquote_itemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_item}.
	 * @param ctx the parse tree
	 */
		fn visit_inference_item(&mut self, ctx: &Inference_itemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_status}.
	 * @param ctx the parse tree
	 */
		fn visit_inference_status(&mut self, ctx: &Inference_statusContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#status_value}.
	 * @param ctx the parse tree
	 */
		fn visit_status_value(&mut self, ctx: &Status_valueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#inference_info}.
	 * @param ctx the parse tree
	 */
		fn visit_inference_info(&mut self, ctx: &Inference_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#assumptions_record}.
	 * @param ctx the parse tree
	 */
		fn visit_assumptions_record(&mut self, ctx: &Assumptions_recordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#refutation}.
	 * @param ctx the parse tree
	 */
		fn visit_refutation(&mut self, ctx: &RefutationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#new_symbol_record}.
	 * @param ctx the parse tree
	 */
		fn visit_new_symbol_record(&mut self, ctx: &New_symbol_recordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#new_symbol_list}.
	 * @param ctx the parse tree
	 */
		fn visit_new_symbol_list(&mut self, ctx: &New_symbol_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#principal_symbol}.
	 * @param ctx the parse tree
	 */
		fn visit_principal_symbol(&mut self, ctx: &Principal_symbolContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#include}.
	 * @param ctx the parse tree
	 */
		fn visit_include(&mut self, ctx: &IncludeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#formula_selection}.
	 * @param ctx the parse tree
	 */
		fn visit_formula_selection(&mut self, ctx: &Formula_selectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#name_list}.
	 * @param ctx the parse tree
	 */
		fn visit_name_list(&mut self, ctx: &Name_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_term}.
	 * @param ctx the parse tree
	 */
		fn visit_general_term(&mut self, ctx: &General_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_data}.
	 * @param ctx the parse tree
	 */
		fn visit_general_data(&mut self, ctx: &General_dataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_function}.
	 * @param ctx the parse tree
	 */
		fn visit_general_function(&mut self, ctx: &General_functionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#formula_data}.
	 * @param ctx the parse tree
	 */
		fn visit_formula_data(&mut self, ctx: &Formula_dataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_list}.
	 * @param ctx the parse tree
	 */
		fn visit_general_list(&mut self, ctx: &General_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#general_terms}.
	 * @param ctx the parse tree
	 */
		fn visit_general_terms(&mut self, ctx: &General_termsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#name}.
	 * @param ctx the parse tree
	 */
		fn visit_name(&mut self, ctx: &NameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#atomic_word}.
	 * @param ctx the parse tree
	 */
		fn visit_atomic_word(&mut self, ctx: &Atomic_wordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#atomic_defined_word}.
	 * @param ctx the parse tree
	 */
		fn visit_atomic_defined_word(&mut self, ctx: &Atomic_defined_wordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#atomic_system_word}.
	 * @param ctx the parse tree
	 */
		fn visit_atomic_system_word(&mut self, ctx: &Atomic_system_wordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_number(&mut self, ctx: &NumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link tptp_v7_0_0_0Parser#file_name}.
	 * @param ctx the parse tree
	 */
		fn visit_file_name(&mut self, ctx: &File_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> tptp_v7_0_0_0Visitor<'input> for T
where
	T: tptp_v7_0_0_0VisitorCompat<'input>
{
	fn visit_tptp_file(&mut self, ctx: &Tptp_fileContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tptp_file(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tptp_input(&mut self, ctx: &Tptp_inputContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tptp_input(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotated_formula(&mut self, ctx: &Annotated_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_annotated_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tpi_annotated(&mut self, ctx: &Tpi_annotatedContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tpi_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tpi_formula(&mut self, ctx: &Tpi_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tpi_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_annotated(&mut self, ctx: &Thf_annotatedContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tfx_annotated(&mut self, ctx: &Tfx_annotatedContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tfx_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_annotated(&mut self, ctx: &Tff_annotatedContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tcf_annotated(&mut self, ctx: &Tcf_annotatedContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tcf_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_annotated(&mut self, ctx: &Fof_annotatedContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cnf_annotated(&mut self, ctx: &Cnf_annotatedContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_cnf_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotations(&mut self, ctx: &AnnotationsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_annotations(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formula_role(&mut self, ctx: &Formula_roleContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_formula_role(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_formula(&mut self, ctx: &Thf_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_logic_formula(&mut self, ctx: &Thf_logic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_binary_formula(&mut self, ctx: &Thf_binary_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_binary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_binary_pair(&mut self, ctx: &Thf_binary_pairContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_binary_pair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_binary_tuple(&mut self, ctx: &Thf_binary_tupleContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_binary_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_or_formula(&mut self, ctx: &Thf_or_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_or_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_and_formula(&mut self, ctx: &Thf_and_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_and_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_apply_formula(&mut self, ctx: &Thf_apply_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_apply_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unitary_formula(&mut self, ctx: &Thf_unitary_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_unitary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_quantified_formula(&mut self, ctx: &Thf_quantified_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_quantified_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_quantification(&mut self, ctx: &Thf_quantificationContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_quantification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_variable_list(&mut self, ctx: &Thf_variable_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_variable_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_variable(&mut self, ctx: &Thf_variableContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_typed_variable(&mut self, ctx: &Thf_typed_variableContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_typed_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unary_formula(&mut self, ctx: &Thf_unary_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_unary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_atom(&mut self, ctx: &Thf_atomContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_function(&mut self, ctx: &Thf_functionContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_function(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_conn_term(&mut self, ctx: &Thf_conn_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_conn_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_conditional(&mut self, ctx: &Thf_conditionalContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_conditional(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_let(&mut self, ctx: &Thf_letContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_let(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_arguments(&mut self, ctx: &Thf_argumentsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_type_formula(&mut self, ctx: &Thf_type_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_type_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_typeable_formula(&mut self, ctx: &Thf_typeable_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_typeable_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_subtype(&mut self, ctx: &Thf_subtypeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_subtype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_top_level_type(&mut self, ctx: &Thf_top_level_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_top_level_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unitary_type(&mut self, ctx: &Thf_unitary_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_unitary_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_apply_type(&mut self, ctx: &Thf_apply_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_apply_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_binary_type(&mut self, ctx: &Thf_binary_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_binary_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_mapping_type(&mut self, ctx: &Thf_mapping_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_mapping_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_xprod_type(&mut self, ctx: &Thf_xprod_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_xprod_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_union_type(&mut self, ctx: &Thf_union_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_union_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_sequent(&mut self, ctx: &Thf_sequentContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_sequent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_tuple(&mut self, ctx: &Thf_tupleContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_formula_list(&mut self, ctx: &Thf_formula_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_formula_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tfx_formula(&mut self, ctx: &Tfx_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tfx_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tfx_logic_formula(&mut self, ctx: &Tfx_logic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tfx_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_formula(&mut self, ctx: &Tff_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_logic_formula(&mut self, ctx: &Tff_logic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_binary_formula(&mut self, ctx: &Tff_binary_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_binary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_binary_nonassoc(&mut self, ctx: &Tff_binary_nonassocContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_binary_nonassoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_binary_assoc(&mut self, ctx: &Tff_binary_assocContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_binary_assoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_or_formula(&mut self, ctx: &Tff_or_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_or_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_and_formula(&mut self, ctx: &Tff_and_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_and_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_unitary_formula(&mut self, ctx: &Tff_unitary_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_unitary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_quantified_formula(&mut self, ctx: &Tff_quantified_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_quantified_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_variable_list(&mut self, ctx: &Tff_variable_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_variable_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_variable(&mut self, ctx: &Tff_variableContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_typed_variable(&mut self, ctx: &Tff_typed_variableContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_typed_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_unary_formula(&mut self, ctx: &Tff_unary_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_unary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_atomic_formula(&mut self, ctx: &Tff_atomic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_conditional(&mut self, ctx: &Tff_conditionalContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_conditional(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let(&mut self, ctx: &Tff_letContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let_term_defns(&mut self, ctx: &Tff_let_term_defnsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let_term_defns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let_term_list(&mut self, ctx: &Tff_let_term_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let_term_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let_term_defn(&mut self, ctx: &Tff_let_term_defnContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let_term_defn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let_term_binding(&mut self, ctx: &Tff_let_term_bindingContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let_term_binding(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let_formula_defns(&mut self, ctx: &Tff_let_formula_defnsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let_formula_defns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let_formula_list(&mut self, ctx: &Tff_let_formula_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let_formula_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let_formula_defn(&mut self, ctx: &Tff_let_formula_defnContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let_formula_defn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let_formula_binding(&mut self, ctx: &Tff_let_formula_bindingContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let_formula_binding(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_sequent(&mut self, ctx: &Tff_sequentContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_sequent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_formula_tuple(&mut self, ctx: &Tff_formula_tupleContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_formula_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_formula_tuple_list(&mut self, ctx: &Tff_formula_tuple_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_formula_tuple_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_typed_atom(&mut self, ctx: &Tff_typed_atomContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_typed_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_subtype(&mut self, ctx: &Tff_subtypeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_subtype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_top_level_type(&mut self, ctx: &Tff_top_level_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_top_level_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tf1_quantified_type(&mut self, ctx: &Tf1_quantified_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tf1_quantified_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_monotype(&mut self, ctx: &Tff_monotypeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_monotype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_unitary_type(&mut self, ctx: &Tff_unitary_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_unitary_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_atomic_type(&mut self, ctx: &Tff_atomic_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_atomic_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_type_arguments(&mut self, ctx: &Tff_type_argumentsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_type_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_mapping_type(&mut self, ctx: &Tff_mapping_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_mapping_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_xprod_type(&mut self, ctx: &Tff_xprod_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_xprod_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tcf_formula(&mut self, ctx: &Tcf_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tcf_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tcf_logic_formula(&mut self, ctx: &Tcf_logic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tcf_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tcf_quantified_formula(&mut self, ctx: &Tcf_quantified_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tcf_quantified_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_formula(&mut self, ctx: &Fof_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_logic_formula(&mut self, ctx: &Fof_logic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_binary_formula(&mut self, ctx: &Fof_binary_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_binary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_binary_nonassoc(&mut self, ctx: &Fof_binary_nonassocContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_binary_nonassoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_binary_assoc(&mut self, ctx: &Fof_binary_assocContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_binary_assoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_or_formula(&mut self, ctx: &Fof_or_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_or_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_and_formula(&mut self, ctx: &Fof_and_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_and_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_unitary_formula(&mut self, ctx: &Fof_unitary_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_unitary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_quantified_formula(&mut self, ctx: &Fof_quantified_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_quantified_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_variable_list(&mut self, ctx: &Fof_variable_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_variable_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_unary_formula(&mut self, ctx: &Fof_unary_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_unary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_infix_unary(&mut self, ctx: &Fof_infix_unaryContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_infix_unary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_atomic_formula(&mut self, ctx: &Fof_atomic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_plain_atomic_formula(&mut self, ctx: &Fof_plain_atomic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_plain_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_atomic_formula(&mut self, ctx: &Fof_defined_atomic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_defined_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_plain_formula(&mut self, ctx: &Fof_defined_plain_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_defined_plain_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_infix_formula(&mut self, ctx: &Fof_defined_infix_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_defined_infix_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_system_atomic_formula(&mut self, ctx: &Fof_system_atomic_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_system_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_plain_term(&mut self, ctx: &Fof_plain_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_plain_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_term(&mut self, ctx: &Fof_defined_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_defined_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_atomic_term(&mut self, ctx: &Fof_defined_atomic_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_defined_atomic_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_plain_term(&mut self, ctx: &Fof_defined_plain_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_defined_plain_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_system_term(&mut self, ctx: &Fof_system_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_system_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_arguments(&mut self, ctx: &Fof_argumentsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_term(&mut self, ctx: &Fof_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_function_term(&mut self, ctx: &Fof_function_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_function_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_conditional_term(&mut self, ctx: &Tff_conditional_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_conditional_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_let_term(&mut self, ctx: &Tff_let_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_let_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_tuple_term(&mut self, ctx: &Tff_tuple_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_tuple_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_sequent(&mut self, ctx: &Fof_sequentContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_sequent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_formula_tuple(&mut self, ctx: &Fof_formula_tupleContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_formula_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_formula_tuple_list(&mut self, ctx: &Fof_formula_tuple_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_formula_tuple_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cnf_formula(&mut self, ctx: &Cnf_formulaContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_cnf_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cnf_disjunction(&mut self, ctx: &Cnf_disjunctionContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_cnf_disjunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cnf_literal(&mut self, ctx: &Cnf_literalContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_cnf_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_quantifier(&mut self, ctx: &Thf_quantifierContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_quantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_th0_quantifier(&mut self, ctx: &Th0_quantifierContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_th0_quantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_th1_quantifier(&mut self, ctx: &Th1_quantifierContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_th1_quantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_pair_connective(&mut self, ctx: &Thf_pair_connectiveContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_pair_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unary_connective(&mut self, ctx: &Thf_unary_connectiveContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_thf_unary_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_th1_unary_connective(&mut self, ctx: &Th1_unary_connectiveContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_th1_unary_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_pair_connective(&mut self, ctx: &Tff_pair_connectiveContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_tff_pair_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_quantifier(&mut self, ctx: &Fof_quantifierContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_fof_quantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_binary_connective(&mut self, ctx: &Binary_connectiveContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_binary_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assoc_connective(&mut self, ctx: &Assoc_connectiveContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_assoc_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unary_connective(&mut self, ctx: &Unary_connectiveContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_unary_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_constant(&mut self, ctx: &Type_constantContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_type_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_functor(&mut self, ctx: &Type_functorContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_type_functor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_type(&mut self, ctx: &Defined_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_defined_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_system_type(&mut self, ctx: &System_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_system_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atom(&mut self, ctx: &AtomContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_untyped_atom(&mut self, ctx: &Untyped_atomContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_untyped_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_proposition(&mut self, ctx: &Defined_propositionContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_defined_proposition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_predicate(&mut self, ctx: &Defined_predicateContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_defined_predicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_infix_pred(&mut self, ctx: &Defined_infix_predContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_defined_infix_pred(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constant(&mut self, ctx: &ConstantContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functor(&mut self, ctx: &FunctorContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_functor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_system_constant(&mut self, ctx: &System_constantContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_system_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_system_functor(&mut self, ctx: &System_functorContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_system_functor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_constant(&mut self, ctx: &Defined_constantContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_defined_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_functor(&mut self, ctx: &Defined_functorContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_defined_functor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_term(&mut self, ctx: &Defined_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_defined_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_source(&mut self, ctx: &SourceContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sources(&mut self, ctx: &SourcesContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_sources(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dag_source(&mut self, ctx: &Dag_sourceContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_dag_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inference_record(&mut self, ctx: &Inference_recordContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_inference_record(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inference_rule(&mut self, ctx: &Inference_ruleContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_inference_rule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inference_parents(&mut self, ctx: &Inference_parentsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_inference_parents(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parent_list(&mut self, ctx: &Parent_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_parent_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parent_info(&mut self, ctx: &Parent_infoContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_parent_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parent_details(&mut self, ctx: &Parent_detailsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_parent_details(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_internal_source(&mut self, ctx: &Internal_sourceContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_internal_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_intro_type(&mut self, ctx: &Intro_typeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_intro_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_external_source(&mut self, ctx: &External_sourceContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_external_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file_source(&mut self, ctx: &File_sourceContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_file_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file_info(&mut self, ctx: &File_infoContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_file_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_theory(&mut self, ctx: &TheoryContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_theory(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_theory_name(&mut self, ctx: &Theory_nameContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_theory_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_creator_source(&mut self, ctx: &Creator_sourceContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_creator_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_creator_name(&mut self, ctx: &Creator_nameContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_creator_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_optional_info(&mut self, ctx: &Optional_infoContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_optional_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_useful_info(&mut self, ctx: &Useful_infoContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_useful_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_info_items(&mut self, ctx: &Info_itemsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_info_items(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_info_item(&mut self, ctx: &Info_itemContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_info_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formula_item(&mut self, ctx: &Formula_itemContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_formula_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_description_item(&mut self, ctx: &Description_itemContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_description_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_iquote_item(&mut self, ctx: &Iquote_itemContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_iquote_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inference_item(&mut self, ctx: &Inference_itemContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_inference_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inference_status(&mut self, ctx: &Inference_statusContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_inference_status(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_status_value(&mut self, ctx: &Status_valueContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_status_value(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inference_info(&mut self, ctx: &Inference_infoContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_inference_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assumptions_record(&mut self, ctx: &Assumptions_recordContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_assumptions_record(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_refutation(&mut self, ctx: &RefutationContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_refutation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_new_symbol_record(&mut self, ctx: &New_symbol_recordContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_new_symbol_record(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_new_symbol_list(&mut self, ctx: &New_symbol_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_new_symbol_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_principal_symbol(&mut self, ctx: &Principal_symbolContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_principal_symbol(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_include(&mut self, ctx: &IncludeContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_include(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formula_selection(&mut self, ctx: &Formula_selectionContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_formula_selection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_name_list(&mut self, ctx: &Name_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_name_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_term(&mut self, ctx: &General_termContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_general_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_data(&mut self, ctx: &General_dataContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_general_data(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_function(&mut self, ctx: &General_functionContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_general_function(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formula_data(&mut self, ctx: &Formula_dataContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_formula_data(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_list(&mut self, ctx: &General_listContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_general_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_terms(&mut self, ctx: &General_termsContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_general_terms(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_name(&mut self, ctx: &NameContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomic_word(&mut self, ctx: &Atomic_wordContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_atomic_word(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomic_defined_word(&mut self, ctx: &Atomic_defined_wordContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_atomic_defined_word(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomic_system_word(&mut self, ctx: &Atomic_system_wordContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_atomic_system_word(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_number(&mut self, ctx: &NumberContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file_name(&mut self, ctx: &File_nameContext<'input>){
		let result = <Self as tptp_v7_0_0_0VisitorCompat>::visit_file_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}