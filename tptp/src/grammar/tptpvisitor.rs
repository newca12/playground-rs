#![allow(nonstandard_style)]
// Generated from TPTP.g4 by ANTLR 4.13.2
use antlr4rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::tptpparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link TPTPParser}.
 */
pub trait TPTPVisitor<'input>: ParseTreeVisitor<'input,TPTPParserContextType>{
	/**
	 * Visit a parse tree produced by {@link TPTPParser#tptp_file}.
	 * @param ctx the parse tree
	 */
	fn visit_tptp_file(&mut self, ctx: &Tptp_fileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tptp_input}.
	 * @param ctx the parse tree
	 */
	fn visit_tptp_input(&mut self, ctx: &Tptp_inputContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#annotated_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_annotated_formula(&mut self, ctx: &Annotated_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tpi_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_tpi_annotated(&mut self, ctx: &Tpi_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tpi_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tpi_formula(&mut self, ctx: &Tpi_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_annotated(&mut self, ctx: &Thf_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_annotated(&mut self, ctx: &Tff_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tcf_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_tcf_annotated(&mut self, ctx: &Tcf_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_annotated(&mut self, ctx: &Fof_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#cnf_annotated}.
	 * @param ctx the parse tree
	 */
	fn visit_cnf_annotated(&mut self, ctx: &Cnf_annotatedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#annotations}.
	 * @param ctx the parse tree
	 */
	fn visit_annotations(&mut self, ctx: &AnnotationsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#formula_role}.
	 * @param ctx the parse tree
	 */
	fn visit_formula_role(&mut self, ctx: &Formula_roleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_formula(&mut self, ctx: &Thf_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_logic_formula(&mut self, ctx: &Thf_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_binary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_binary_formula(&mut self, ctx: &Thf_binary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_binary_nonassoc(&mut self, ctx: &Thf_binary_nonassocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_binary_assoc}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_binary_assoc(&mut self, ctx: &Thf_binary_assocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_or_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_or_formula(&mut self, ctx: &Thf_or_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_and_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_and_formula(&mut self, ctx: &Thf_and_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_apply_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_apply_formula(&mut self, ctx: &Thf_apply_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unit_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unit_formula(&mut self, ctx: &Thf_unit_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_preunit_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_preunit_formula(&mut self, ctx: &Thf_preunit_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unitary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unitary_formula(&mut self, ctx: &Thf_unitary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_quantified_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_quantified_formula(&mut self, ctx: &Thf_quantified_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_quantification}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_quantification(&mut self, ctx: &Thf_quantificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_variable_list}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_variable_list(&mut self, ctx: &Thf_variable_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_typed_variable}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_typed_variable(&mut self, ctx: &Thf_typed_variableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unary_formula(&mut self, ctx: &Thf_unary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_prefix_unary}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_prefix_unary(&mut self, ctx: &Thf_prefix_unaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_infix_unary}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_infix_unary(&mut self, ctx: &Thf_infix_unaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_atomic_formula(&mut self, ctx: &Thf_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_plain_atomic}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_plain_atomic(&mut self, ctx: &Thf_plain_atomicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_defined_atomic}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_defined_atomic(&mut self, ctx: &Thf_defined_atomicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_defined_term}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_defined_term(&mut self, ctx: &Thf_defined_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_defined_infix}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_defined_infix(&mut self, ctx: &Thf_defined_infixContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_system_atomic}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_system_atomic(&mut self, ctx: &Thf_system_atomicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_let(&mut self, ctx: &Thf_letContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let_types}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_let_types(&mut self, ctx: &Thf_let_typesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_atom_typing_list}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_atom_typing_list(&mut self, ctx: &Thf_atom_typing_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let_defns}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_let_defns(&mut self, ctx: &Thf_let_defnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let_defn}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_let_defn(&mut self, ctx: &Thf_let_defnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let_defn_list}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_let_defn_list(&mut self, ctx: &Thf_let_defn_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unitary_term}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unitary_term(&mut self, ctx: &Thf_unitary_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_conn_term}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_conn_term(&mut self, ctx: &Thf_conn_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_tuple}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_tuple(&mut self, ctx: &Thf_tupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_fof_function}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_fof_function(&mut self, ctx: &Thf_fof_functionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_arguments(&mut self, ctx: &Thf_argumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_formula_list}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_formula_list(&mut self, ctx: &Thf_formula_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_thf_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_comma_thf_logic_formula(&mut self, ctx: &Comma_thf_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_atom_typing}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_atom_typing(&mut self, ctx: &Thf_atom_typingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_top_level_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_top_level_type(&mut self, ctx: &Thf_top_level_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unitary_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unitary_type(&mut self, ctx: &Thf_unitary_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_apply_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_apply_type(&mut self, ctx: &Thf_apply_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_binary_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_binary_type(&mut self, ctx: &Thf_binary_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_mapping_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_mapping_type(&mut self, ctx: &Thf_mapping_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_xprod_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_xprod_type(&mut self, ctx: &Thf_xprod_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_union_type}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_union_type(&mut self, ctx: &Thf_union_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_subtype}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_subtype(&mut self, ctx: &Thf_subtypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_definition}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_definition(&mut self, ctx: &Thf_definitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_sequent}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_sequent(&mut self, ctx: &Thf_sequentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_formula(&mut self, ctx: &Tff_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_logic_formula(&mut self, ctx: &Tff_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_binary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_binary_formula(&mut self, ctx: &Tff_binary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_binary_nonassoc(&mut self, ctx: &Tff_binary_nonassocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_binary_assoc}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_binary_assoc(&mut self, ctx: &Tff_binary_assocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_or_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_or_formula(&mut self, ctx: &Tff_or_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_and_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_and_formula(&mut self, ctx: &Tff_and_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unit_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_unit_formula(&mut self, ctx: &Tff_unit_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_preunit_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_preunit_formula(&mut self, ctx: &Tff_preunit_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unitary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_unitary_formula(&mut self, ctx: &Tff_unitary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_unitary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_unitary_formula(&mut self, ctx: &Txf_unitary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_quantified_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_quantified_formula(&mut self, ctx: &Tff_quantified_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_variable_list}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_variable_list(&mut self, ctx: &Tff_variable_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_variable}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_variable(&mut self, ctx: &Tff_variableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_typed_variable}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_typed_variable(&mut self, ctx: &Tff_typed_variableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_unary_formula(&mut self, ctx: &Tff_unary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_prefix_unary}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_prefix_unary(&mut self, ctx: &Tff_prefix_unaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_infix_unary}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_infix_unary(&mut self, ctx: &Tff_infix_unaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_atomic_formula(&mut self, ctx: &Tff_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_plain_atomic}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_plain_atomic(&mut self, ctx: &Tff_plain_atomicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_defined_atomic}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_defined_atomic(&mut self, ctx: &Tff_defined_atomicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_defined_plain}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_defined_plain(&mut self, ctx: &Tff_defined_plainContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_defined_infix}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_defined_infix(&mut self, ctx: &Tff_defined_infixContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_system_atomic}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_system_atomic(&mut self, ctx: &Tff_system_atomicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_let(&mut self, ctx: &Txf_letContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_types}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_let_types(&mut self, ctx: &Txf_let_typesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_atom_typing_list}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_atom_typing_list(&mut self, ctx: &Tff_atom_typing_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_defns}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_let_defns(&mut self, ctx: &Txf_let_defnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_defn}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_let_defn(&mut self, ctx: &Txf_let_defnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_LHS}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_let_LHS(&mut self, ctx: &Txf_let_LHSContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_defn_list}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_let_defn_list(&mut self, ctx: &Txf_let_defn_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_atom}.
	 * @param ctx the parse tree
	 */
	fn visit_nxf_atom(&mut self, ctx: &Nxf_atomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_term}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_term(&mut self, ctx: &Tff_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unitary_term}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_unitary_term(&mut self, ctx: &Tff_unitary_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_tuple}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_tuple(&mut self, ctx: &Txf_tupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_arguments(&mut self, ctx: &Tff_argumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_tff_term}.
	 * @param ctx the parse tree
	 */
	fn visit_comma_tff_term(&mut self, ctx: &Comma_tff_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_atom_typing}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_atom_typing(&mut self, ctx: &Tff_atom_typingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_top_level_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_top_level_type(&mut self, ctx: &Tff_top_level_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_non_atomic_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_non_atomic_type(&mut self, ctx: &Tff_non_atomic_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tf1_quantified_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tf1_quantified_type(&mut self, ctx: &Tf1_quantified_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_monotype}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_monotype(&mut self, ctx: &Tff_monotypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unitary_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_unitary_type(&mut self, ctx: &Tff_unitary_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_atomic_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_atomic_type(&mut self, ctx: &Tff_atomic_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_type_arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_type_arguments(&mut self, ctx: &Tff_type_argumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_mapping_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_mapping_type(&mut self, ctx: &Tff_mapping_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_xprod_type}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_xprod_type(&mut self, ctx: &Tff_xprod_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_tuple_type}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_tuple_type(&mut self, ctx: &Txf_tuple_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_type_list}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_type_list(&mut self, ctx: &Tff_type_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_subtype}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_subtype(&mut self, ctx: &Tff_subtypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_definition}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_definition(&mut self, ctx: &Txf_definitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_sequent}.
	 * @param ctx the parse tree
	 */
	fn visit_txf_sequent(&mut self, ctx: &Txf_sequentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nhf_long_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_nhf_long_connective(&mut self, ctx: &Nhf_long_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nhf_parameter_list}.
	 * @param ctx the parse tree
	 */
	fn visit_nhf_parameter_list(&mut self, ctx: &Nhf_parameter_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nhf_parameter}.
	 * @param ctx the parse tree
	 */
	fn visit_nhf_parameter(&mut self, ctx: &Nhf_parameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nhf_key_pair}.
	 * @param ctx the parse tree
	 */
	fn visit_nhf_key_pair(&mut self, ctx: &Nhf_key_pairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_long_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_nxf_long_connective(&mut self, ctx: &Nxf_long_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_parameter_list}.
	 * @param ctx the parse tree
	 */
	fn visit_nxf_parameter_list(&mut self, ctx: &Nxf_parameter_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_parameter}.
	 * @param ctx the parse tree
	 */
	fn visit_nxf_parameter(&mut self, ctx: &Nxf_parameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_key_pair}.
	 * @param ctx the parse tree
	 */
	fn visit_nxf_key_pair(&mut self, ctx: &Nxf_key_pairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#ntf_connective_name}.
	 * @param ctx the parse tree
	 */
	fn visit_ntf_connective_name(&mut self, ctx: &Ntf_connective_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#ntf_index}.
	 * @param ctx the parse tree
	 */
	fn visit_ntf_index(&mut self, ctx: &Ntf_indexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#ntf_short_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_ntf_short_connective(&mut self, ctx: &Ntf_short_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tcf_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tcf_formula(&mut self, ctx: &Tcf_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tcf_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tcf_logic_formula(&mut self, ctx: &Tcf_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tcf_quantified_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_tcf_quantified_formula(&mut self, ctx: &Tcf_quantified_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_formula(&mut self, ctx: &Fof_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_logic_formula(&mut self, ctx: &Fof_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_binary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_binary_formula(&mut self, ctx: &Fof_binary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_binary_nonassoc(&mut self, ctx: &Fof_binary_nonassocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_binary_assoc}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_binary_assoc(&mut self, ctx: &Fof_binary_assocContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_or_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_or_formula(&mut self, ctx: &Fof_or_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_and_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_and_formula(&mut self, ctx: &Fof_and_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_unary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_unary_formula(&mut self, ctx: &Fof_unary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_infix_unary}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_infix_unary(&mut self, ctx: &Fof_infix_unaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_unit_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_unit_formula(&mut self, ctx: &Fof_unit_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_unitary_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_unitary_formula(&mut self, ctx: &Fof_unitary_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_quantified_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_quantified_formula(&mut self, ctx: &Fof_quantified_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_variable_list}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_variable_list(&mut self, ctx: &Fof_variable_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_atomic_formula(&mut self, ctx: &Fof_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_plain_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_plain_atomic_formula(&mut self, ctx: &Fof_plain_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_atomic_formula(&mut self, ctx: &Fof_defined_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_plain_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_plain_formula(&mut self, ctx: &Fof_defined_plain_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_infix_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_infix_formula(&mut self, ctx: &Fof_defined_infix_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_system_atomic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_system_atomic_formula(&mut self, ctx: &Fof_system_atomic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_plain_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_plain_term(&mut self, ctx: &Fof_plain_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_term(&mut self, ctx: &Fof_defined_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_atomic_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_atomic_term(&mut self, ctx: &Fof_defined_atomic_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_plain_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_defined_plain_term(&mut self, ctx: &Fof_defined_plain_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_system_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_system_term(&mut self, ctx: &Fof_system_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_arguments(&mut self, ctx: &Fof_argumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_term(&mut self, ctx: &Fof_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_function_term}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_function_term(&mut self, ctx: &Fof_function_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_sequent}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_sequent(&mut self, ctx: &Fof_sequentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_formula_tuple}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_formula_tuple(&mut self, ctx: &Fof_formula_tupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_formula_tuple_list}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_formula_tuple_list(&mut self, ctx: &Fof_formula_tuple_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_fof_logic_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_comma_fof_logic_formula(&mut self, ctx: &Comma_fof_logic_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#cnf_formula}.
	 * @param ctx the parse tree
	 */
	fn visit_cnf_formula(&mut self, ctx: &Cnf_formulaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#cnf_disjunction}.
	 * @param ctx the parse tree
	 */
	fn visit_cnf_disjunction(&mut self, ctx: &Cnf_disjunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#cnf_literal}.
	 * @param ctx the parse tree
	 */
	fn visit_cnf_literal(&mut self, ctx: &Cnf_literalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_quantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_quantifier(&mut self, ctx: &Thf_quantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unary_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_thf_unary_connective(&mut self, ctx: &Thf_unary_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#th0_quantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_th0_quantifier(&mut self, ctx: &Th0_quantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#type_quantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_type_quantifier(&mut self, ctx: &Type_quantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#subtype_sign}.
	 * @param ctx the parse tree
	 */
	fn visit_subtype_sign(&mut self, ctx: &Subtype_signContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unary_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_unary_connective(&mut self, ctx: &Tff_unary_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_quantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_tff_quantifier(&mut self, ctx: &Tff_quantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_quantifier}.
	 * @param ctx the parse tree
	 */
	fn visit_fof_quantifier(&mut self, ctx: &Fof_quantifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nonassoc_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_nonassoc_connective(&mut self, ctx: &Nonassoc_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#assoc_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_assoc_connective(&mut self, ctx: &Assoc_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#unary_connective}.
	 * @param ctx the parse tree
	 */
	fn visit_unary_connective(&mut self, ctx: &Unary_connectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#gentzen_arrow}.
	 * @param ctx the parse tree
	 */
	fn visit_gentzen_arrow(&mut self, ctx: &Gentzen_arrowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#identical}.
	 * @param ctx the parse tree
	 */
	fn visit_identical(&mut self, ctx: &IdenticalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#type_constant}.
	 * @param ctx the parse tree
	 */
	fn visit_type_constant(&mut self, ctx: &Type_constantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#type_functor}.
	 * @param ctx the parse tree
	 */
	fn visit_type_functor(&mut self, ctx: &Type_functorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_type}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_type(&mut self, ctx: &Defined_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atom(&mut self, ctx: &AtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#untyped_atom}.
	 * @param ctx the parse tree
	 */
	fn visit_untyped_atom(&mut self, ctx: &Untyped_atomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_infix_pred}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_infix_pred(&mut self, ctx: &Defined_infix_predContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#infix_equality}.
	 * @param ctx the parse tree
	 */
	fn visit_infix_equality(&mut self, ctx: &Infix_equalityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#infix_inequality}.
	 * @param ctx the parse tree
	 */
	fn visit_infix_inequality(&mut self, ctx: &Infix_inequalityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_constant(&mut self, ctx: &ConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#functor}.
	 * @param ctx the parse tree
	 */
	fn visit_functor(&mut self, ctx: &FunctorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_constant}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_constant(&mut self, ctx: &Defined_constantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_functor}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_functor(&mut self, ctx: &Defined_functorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#system_constant}.
	 * @param ctx the parse tree
	 */
	fn visit_system_constant(&mut self, ctx: &System_constantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#system_functor}.
	 * @param ctx the parse tree
	 */
	fn visit_system_functor(&mut self, ctx: &System_functorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#def_or_sys_constant}.
	 * @param ctx the parse tree
	 */
	fn visit_def_or_sys_constant(&mut self, ctx: &Def_or_sys_constantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#th1_defined_term}.
	 * @param ctx the parse tree
	 */
	fn visit_th1_defined_term(&mut self, ctx: &Th1_defined_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_term}.
	 * @param ctx the parse tree
	 */
	fn visit_defined_term(&mut self, ctx: &Defined_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#source}.
	 * @param ctx the parse tree
	 */
	fn visit_source(&mut self, ctx: &SourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#sources}.
	 * @param ctx the parse tree
	 */
	fn visit_sources(&mut self, ctx: &SourcesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#dag_source}.
	 * @param ctx the parse tree
	 */
	fn visit_dag_source(&mut self, ctx: &Dag_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#inference_record}.
	 * @param ctx the parse tree
	 */
	fn visit_inference_record(&mut self, ctx: &Inference_recordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#inference_rule}.
	 * @param ctx the parse tree
	 */
	fn visit_inference_rule(&mut self, ctx: &Inference_ruleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#internal_source}.
	 * @param ctx the parse tree
	 */
	fn visit_internal_source(&mut self, ctx: &Internal_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#intro_type}.
	 * @param ctx the parse tree
	 */
	fn visit_intro_type(&mut self, ctx: &Intro_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#external_source}.
	 * @param ctx the parse tree
	 */
	fn visit_external_source(&mut self, ctx: &External_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#file_source}.
	 * @param ctx the parse tree
	 */
	fn visit_file_source(&mut self, ctx: &File_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#file_info}.
	 * @param ctx the parse tree
	 */
	fn visit_file_info(&mut self, ctx: &File_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#theory}.
	 * @param ctx the parse tree
	 */
	fn visit_theory(&mut self, ctx: &TheoryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#theory_name}.
	 * @param ctx the parse tree
	 */
	fn visit_theory_name(&mut self, ctx: &Theory_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#creator_source}.
	 * @param ctx the parse tree
	 */
	fn visit_creator_source(&mut self, ctx: &Creator_sourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#creator_name}.
	 * @param ctx the parse tree
	 */
	fn visit_creator_name(&mut self, ctx: &Creator_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#parents}.
	 * @param ctx the parse tree
	 */
	fn visit_parents(&mut self, ctx: &ParentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#parent_list}.
	 * @param ctx the parse tree
	 */
	fn visit_parent_list(&mut self, ctx: &Parent_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_parent_info}.
	 * @param ctx the parse tree
	 */
	fn visit_comma_parent_info(&mut self, ctx: &Comma_parent_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#parent_info}.
	 * @param ctx the parse tree
	 */
	fn visit_parent_info(&mut self, ctx: &Parent_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#parent_details}.
	 * @param ctx the parse tree
	 */
	fn visit_parent_details(&mut self, ctx: &Parent_detailsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#optional_info}.
	 * @param ctx the parse tree
	 */
	fn visit_optional_info(&mut self, ctx: &Optional_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#useful_info}.
	 * @param ctx the parse tree
	 */
	fn visit_useful_info(&mut self, ctx: &Useful_infoContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#include}.
	 * @param ctx the parse tree
	 */
	fn visit_include(&mut self, ctx: &IncludeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#include_optionals}.
	 * @param ctx the parse tree
	 */
	fn visit_include_optionals(&mut self, ctx: &Include_optionalsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#formula_selection}.
	 * @param ctx the parse tree
	 */
	fn visit_formula_selection(&mut self, ctx: &Formula_selectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#name_list}.
	 * @param ctx the parse tree
	 */
	fn visit_name_list(&mut self, ctx: &Name_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#space_name}.
	 * @param ctx the parse tree
	 */
	fn visit_space_name(&mut self, ctx: &Space_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_term}.
	 * @param ctx the parse tree
	 */
	fn visit_general_term(&mut self, ctx: &General_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_data}.
	 * @param ctx the parse tree
	 */
	fn visit_general_data(&mut self, ctx: &General_dataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_function}.
	 * @param ctx the parse tree
	 */
	fn visit_general_function(&mut self, ctx: &General_functionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#formula_data}.
	 * @param ctx the parse tree
	 */
	fn visit_formula_data(&mut self, ctx: &Formula_dataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_list}.
	 * @param ctx the parse tree
	 */
	fn visit_general_list(&mut self, ctx: &General_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_terms}.
	 * @param ctx the parse tree
	 */
	fn visit_general_terms(&mut self, ctx: &General_termsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_general_term}.
	 * @param ctx the parse tree
	 */
	fn visit_comma_general_term(&mut self, ctx: &Comma_general_termContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#name}.
	 * @param ctx the parse tree
	 */
	fn visit_name(&mut self, ctx: &NameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#atomic_word}.
	 * @param ctx the parse tree
	 */
	fn visit_atomic_word(&mut self, ctx: &Atomic_wordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#atomic_defined_word}.
	 * @param ctx the parse tree
	 */
	fn visit_atomic_defined_word(&mut self, ctx: &Atomic_defined_wordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#atomic_system_word}.
	 * @param ctx the parse tree
	 */
	fn visit_atomic_system_word(&mut self, ctx: &Atomic_system_wordContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_number(&mut self, ctx: &NumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#file_name}.
	 * @param ctx the parse tree
	 */
	fn visit_file_name(&mut self, ctx: &File_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nothing}.
	 * @param ctx the parse tree
	 */
	fn visit_nothing(&mut self, ctx: &NothingContext<'input>) { self.visit_children(ctx) }

}

pub trait TPTPVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= TPTPParserContextType>{
	/**
	 * Visit a parse tree produced by {@link TPTPParser#tptp_file}.
	 * @param ctx the parse tree
	 */
		fn visit_tptp_file(&mut self, ctx: &Tptp_fileContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tptp_input}.
	 * @param ctx the parse tree
	 */
		fn visit_tptp_input(&mut self, ctx: &Tptp_inputContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#annotated_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_annotated_formula(&mut self, ctx: &Annotated_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tpi_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_tpi_annotated(&mut self, ctx: &Tpi_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tpi_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tpi_formula(&mut self, ctx: &Tpi_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_annotated(&mut self, ctx: &Thf_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_annotated(&mut self, ctx: &Tff_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tcf_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_tcf_annotated(&mut self, ctx: &Tcf_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_annotated(&mut self, ctx: &Fof_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#cnf_annotated}.
	 * @param ctx the parse tree
	 */
		fn visit_cnf_annotated(&mut self, ctx: &Cnf_annotatedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#annotations}.
	 * @param ctx the parse tree
	 */
		fn visit_annotations(&mut self, ctx: &AnnotationsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#formula_role}.
	 * @param ctx the parse tree
	 */
		fn visit_formula_role(&mut self, ctx: &Formula_roleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_formula(&mut self, ctx: &Thf_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_logic_formula(&mut self, ctx: &Thf_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_binary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_binary_formula(&mut self, ctx: &Thf_binary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_binary_nonassoc(&mut self, ctx: &Thf_binary_nonassocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_binary_assoc}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_binary_assoc(&mut self, ctx: &Thf_binary_assocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_or_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_or_formula(&mut self, ctx: &Thf_or_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_and_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_and_formula(&mut self, ctx: &Thf_and_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_apply_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_apply_formula(&mut self, ctx: &Thf_apply_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unit_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unit_formula(&mut self, ctx: &Thf_unit_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_preunit_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_preunit_formula(&mut self, ctx: &Thf_preunit_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unitary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unitary_formula(&mut self, ctx: &Thf_unitary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_quantified_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_quantified_formula(&mut self, ctx: &Thf_quantified_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_quantification}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_quantification(&mut self, ctx: &Thf_quantificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_variable_list}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_variable_list(&mut self, ctx: &Thf_variable_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_typed_variable}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_typed_variable(&mut self, ctx: &Thf_typed_variableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unary_formula(&mut self, ctx: &Thf_unary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_prefix_unary}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_prefix_unary(&mut self, ctx: &Thf_prefix_unaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_infix_unary}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_infix_unary(&mut self, ctx: &Thf_infix_unaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_atomic_formula(&mut self, ctx: &Thf_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_plain_atomic}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_plain_atomic(&mut self, ctx: &Thf_plain_atomicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_defined_atomic}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_defined_atomic(&mut self, ctx: &Thf_defined_atomicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_defined_term}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_defined_term(&mut self, ctx: &Thf_defined_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_defined_infix}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_defined_infix(&mut self, ctx: &Thf_defined_infixContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_system_atomic}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_system_atomic(&mut self, ctx: &Thf_system_atomicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_let(&mut self, ctx: &Thf_letContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let_types}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_let_types(&mut self, ctx: &Thf_let_typesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_atom_typing_list}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_atom_typing_list(&mut self, ctx: &Thf_atom_typing_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let_defns}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_let_defns(&mut self, ctx: &Thf_let_defnsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let_defn}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_let_defn(&mut self, ctx: &Thf_let_defnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_let_defn_list}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_let_defn_list(&mut self, ctx: &Thf_let_defn_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unitary_term}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unitary_term(&mut self, ctx: &Thf_unitary_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_conn_term}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_conn_term(&mut self, ctx: &Thf_conn_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_tuple}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_tuple(&mut self, ctx: &Thf_tupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_fof_function}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_fof_function(&mut self, ctx: &Thf_fof_functionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_arguments(&mut self, ctx: &Thf_argumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_formula_list}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_formula_list(&mut self, ctx: &Thf_formula_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_thf_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_comma_thf_logic_formula(&mut self, ctx: &Comma_thf_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_atom_typing}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_atom_typing(&mut self, ctx: &Thf_atom_typingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_top_level_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_top_level_type(&mut self, ctx: &Thf_top_level_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unitary_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unitary_type(&mut self, ctx: &Thf_unitary_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_apply_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_apply_type(&mut self, ctx: &Thf_apply_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_binary_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_binary_type(&mut self, ctx: &Thf_binary_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_mapping_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_mapping_type(&mut self, ctx: &Thf_mapping_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_xprod_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_xprod_type(&mut self, ctx: &Thf_xprod_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_union_type}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_union_type(&mut self, ctx: &Thf_union_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_subtype}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_subtype(&mut self, ctx: &Thf_subtypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_definition}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_definition(&mut self, ctx: &Thf_definitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_sequent}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_sequent(&mut self, ctx: &Thf_sequentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_formula(&mut self, ctx: &Tff_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_logic_formula(&mut self, ctx: &Tff_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_binary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_binary_formula(&mut self, ctx: &Tff_binary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_binary_nonassoc(&mut self, ctx: &Tff_binary_nonassocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_binary_assoc}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_binary_assoc(&mut self, ctx: &Tff_binary_assocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_or_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_or_formula(&mut self, ctx: &Tff_or_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_and_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_and_formula(&mut self, ctx: &Tff_and_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unit_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_unit_formula(&mut self, ctx: &Tff_unit_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_preunit_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_preunit_formula(&mut self, ctx: &Tff_preunit_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unitary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_unitary_formula(&mut self, ctx: &Tff_unitary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_unitary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_unitary_formula(&mut self, ctx: &Txf_unitary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_quantified_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_quantified_formula(&mut self, ctx: &Tff_quantified_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_variable_list}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_variable_list(&mut self, ctx: &Tff_variable_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_variable}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_variable(&mut self, ctx: &Tff_variableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_typed_variable}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_typed_variable(&mut self, ctx: &Tff_typed_variableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_unary_formula(&mut self, ctx: &Tff_unary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_prefix_unary}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_prefix_unary(&mut self, ctx: &Tff_prefix_unaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_infix_unary}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_infix_unary(&mut self, ctx: &Tff_infix_unaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_atomic_formula(&mut self, ctx: &Tff_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_plain_atomic}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_plain_atomic(&mut self, ctx: &Tff_plain_atomicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_defined_atomic}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_defined_atomic(&mut self, ctx: &Tff_defined_atomicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_defined_plain}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_defined_plain(&mut self, ctx: &Tff_defined_plainContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_defined_infix}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_defined_infix(&mut self, ctx: &Tff_defined_infixContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_system_atomic}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_system_atomic(&mut self, ctx: &Tff_system_atomicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_let(&mut self, ctx: &Txf_letContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_types}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_let_types(&mut self, ctx: &Txf_let_typesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_atom_typing_list}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_atom_typing_list(&mut self, ctx: &Tff_atom_typing_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_defns}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_let_defns(&mut self, ctx: &Txf_let_defnsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_defn}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_let_defn(&mut self, ctx: &Txf_let_defnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_LHS}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_let_LHS(&mut self, ctx: &Txf_let_LHSContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_let_defn_list}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_let_defn_list(&mut self, ctx: &Txf_let_defn_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_atom}.
	 * @param ctx the parse tree
	 */
		fn visit_nxf_atom(&mut self, ctx: &Nxf_atomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_term}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_term(&mut self, ctx: &Tff_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unitary_term}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_unitary_term(&mut self, ctx: &Tff_unitary_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_tuple}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_tuple(&mut self, ctx: &Txf_tupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_arguments(&mut self, ctx: &Tff_argumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_tff_term}.
	 * @param ctx the parse tree
	 */
		fn visit_comma_tff_term(&mut self, ctx: &Comma_tff_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_atom_typing}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_atom_typing(&mut self, ctx: &Tff_atom_typingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_top_level_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_top_level_type(&mut self, ctx: &Tff_top_level_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_non_atomic_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_non_atomic_type(&mut self, ctx: &Tff_non_atomic_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tf1_quantified_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tf1_quantified_type(&mut self, ctx: &Tf1_quantified_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_monotype}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_monotype(&mut self, ctx: &Tff_monotypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unitary_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_unitary_type(&mut self, ctx: &Tff_unitary_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_atomic_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_atomic_type(&mut self, ctx: &Tff_atomic_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_type_arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_type_arguments(&mut self, ctx: &Tff_type_argumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_mapping_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_mapping_type(&mut self, ctx: &Tff_mapping_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_xprod_type}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_xprod_type(&mut self, ctx: &Tff_xprod_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_tuple_type}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_tuple_type(&mut self, ctx: &Txf_tuple_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_type_list}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_type_list(&mut self, ctx: &Tff_type_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_subtype}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_subtype(&mut self, ctx: &Tff_subtypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_definition}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_definition(&mut self, ctx: &Txf_definitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#txf_sequent}.
	 * @param ctx the parse tree
	 */
		fn visit_txf_sequent(&mut self, ctx: &Txf_sequentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nhf_long_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_nhf_long_connective(&mut self, ctx: &Nhf_long_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nhf_parameter_list}.
	 * @param ctx the parse tree
	 */
		fn visit_nhf_parameter_list(&mut self, ctx: &Nhf_parameter_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nhf_parameter}.
	 * @param ctx the parse tree
	 */
		fn visit_nhf_parameter(&mut self, ctx: &Nhf_parameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nhf_key_pair}.
	 * @param ctx the parse tree
	 */
		fn visit_nhf_key_pair(&mut self, ctx: &Nhf_key_pairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_long_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_nxf_long_connective(&mut self, ctx: &Nxf_long_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_parameter_list}.
	 * @param ctx the parse tree
	 */
		fn visit_nxf_parameter_list(&mut self, ctx: &Nxf_parameter_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_parameter}.
	 * @param ctx the parse tree
	 */
		fn visit_nxf_parameter(&mut self, ctx: &Nxf_parameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nxf_key_pair}.
	 * @param ctx the parse tree
	 */
		fn visit_nxf_key_pair(&mut self, ctx: &Nxf_key_pairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#ntf_connective_name}.
	 * @param ctx the parse tree
	 */
		fn visit_ntf_connective_name(&mut self, ctx: &Ntf_connective_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#ntf_index}.
	 * @param ctx the parse tree
	 */
		fn visit_ntf_index(&mut self, ctx: &Ntf_indexContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#ntf_short_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_ntf_short_connective(&mut self, ctx: &Ntf_short_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tcf_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tcf_formula(&mut self, ctx: &Tcf_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tcf_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tcf_logic_formula(&mut self, ctx: &Tcf_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tcf_quantified_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_tcf_quantified_formula(&mut self, ctx: &Tcf_quantified_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_formula(&mut self, ctx: &Fof_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_logic_formula(&mut self, ctx: &Fof_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_binary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_binary_formula(&mut self, ctx: &Fof_binary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_binary_nonassoc}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_binary_nonassoc(&mut self, ctx: &Fof_binary_nonassocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_binary_assoc}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_binary_assoc(&mut self, ctx: &Fof_binary_assocContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_or_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_or_formula(&mut self, ctx: &Fof_or_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_and_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_and_formula(&mut self, ctx: &Fof_and_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_unary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_unary_formula(&mut self, ctx: &Fof_unary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_infix_unary}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_infix_unary(&mut self, ctx: &Fof_infix_unaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_unit_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_unit_formula(&mut self, ctx: &Fof_unit_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_unitary_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_unitary_formula(&mut self, ctx: &Fof_unitary_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_quantified_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_quantified_formula(&mut self, ctx: &Fof_quantified_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_variable_list}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_variable_list(&mut self, ctx: &Fof_variable_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_atomic_formula(&mut self, ctx: &Fof_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_plain_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_plain_atomic_formula(&mut self, ctx: &Fof_plain_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_atomic_formula(&mut self, ctx: &Fof_defined_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_plain_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_plain_formula(&mut self, ctx: &Fof_defined_plain_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_infix_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_infix_formula(&mut self, ctx: &Fof_defined_infix_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_system_atomic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_system_atomic_formula(&mut self, ctx: &Fof_system_atomic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_plain_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_plain_term(&mut self, ctx: &Fof_plain_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_term(&mut self, ctx: &Fof_defined_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_atomic_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_atomic_term(&mut self, ctx: &Fof_defined_atomic_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_defined_plain_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_defined_plain_term(&mut self, ctx: &Fof_defined_plain_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_system_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_system_term(&mut self, ctx: &Fof_system_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_arguments(&mut self, ctx: &Fof_argumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_term(&mut self, ctx: &Fof_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_function_term}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_function_term(&mut self, ctx: &Fof_function_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_sequent}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_sequent(&mut self, ctx: &Fof_sequentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_formula_tuple}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_formula_tuple(&mut self, ctx: &Fof_formula_tupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_formula_tuple_list}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_formula_tuple_list(&mut self, ctx: &Fof_formula_tuple_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_fof_logic_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_comma_fof_logic_formula(&mut self, ctx: &Comma_fof_logic_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#cnf_formula}.
	 * @param ctx the parse tree
	 */
		fn visit_cnf_formula(&mut self, ctx: &Cnf_formulaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#cnf_disjunction}.
	 * @param ctx the parse tree
	 */
		fn visit_cnf_disjunction(&mut self, ctx: &Cnf_disjunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#cnf_literal}.
	 * @param ctx the parse tree
	 */
		fn visit_cnf_literal(&mut self, ctx: &Cnf_literalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_quantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_quantifier(&mut self, ctx: &Thf_quantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#thf_unary_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_thf_unary_connective(&mut self, ctx: &Thf_unary_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#th0_quantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_th0_quantifier(&mut self, ctx: &Th0_quantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#type_quantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_type_quantifier(&mut self, ctx: &Type_quantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#subtype_sign}.
	 * @param ctx the parse tree
	 */
		fn visit_subtype_sign(&mut self, ctx: &Subtype_signContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_unary_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_unary_connective(&mut self, ctx: &Tff_unary_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#tff_quantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_tff_quantifier(&mut self, ctx: &Tff_quantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#fof_quantifier}.
	 * @param ctx the parse tree
	 */
		fn visit_fof_quantifier(&mut self, ctx: &Fof_quantifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nonassoc_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_nonassoc_connective(&mut self, ctx: &Nonassoc_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#assoc_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_assoc_connective(&mut self, ctx: &Assoc_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#unary_connective}.
	 * @param ctx the parse tree
	 */
		fn visit_unary_connective(&mut self, ctx: &Unary_connectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#gentzen_arrow}.
	 * @param ctx the parse tree
	 */
		fn visit_gentzen_arrow(&mut self, ctx: &Gentzen_arrowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#assignment}.
	 * @param ctx the parse tree
	 */
		fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#identical}.
	 * @param ctx the parse tree
	 */
		fn visit_identical(&mut self, ctx: &IdenticalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#type_constant}.
	 * @param ctx the parse tree
	 */
		fn visit_type_constant(&mut self, ctx: &Type_constantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#type_functor}.
	 * @param ctx the parse tree
	 */
		fn visit_type_functor(&mut self, ctx: &Type_functorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_type}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_type(&mut self, ctx: &Defined_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#untyped_atom}.
	 * @param ctx the parse tree
	 */
		fn visit_untyped_atom(&mut self, ctx: &Untyped_atomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_infix_pred}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_infix_pred(&mut self, ctx: &Defined_infix_predContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#infix_equality}.
	 * @param ctx the parse tree
	 */
		fn visit_infix_equality(&mut self, ctx: &Infix_equalityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#infix_inequality}.
	 * @param ctx the parse tree
	 */
		fn visit_infix_inequality(&mut self, ctx: &Infix_inequalityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_constant(&mut self, ctx: &ConstantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#functor}.
	 * @param ctx the parse tree
	 */
		fn visit_functor(&mut self, ctx: &FunctorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_constant}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_constant(&mut self, ctx: &Defined_constantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_functor}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_functor(&mut self, ctx: &Defined_functorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#system_constant}.
	 * @param ctx the parse tree
	 */
		fn visit_system_constant(&mut self, ctx: &System_constantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#system_functor}.
	 * @param ctx the parse tree
	 */
		fn visit_system_functor(&mut self, ctx: &System_functorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#def_or_sys_constant}.
	 * @param ctx the parse tree
	 */
		fn visit_def_or_sys_constant(&mut self, ctx: &Def_or_sys_constantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#th1_defined_term}.
	 * @param ctx the parse tree
	 */
		fn visit_th1_defined_term(&mut self, ctx: &Th1_defined_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#defined_term}.
	 * @param ctx the parse tree
	 */
		fn visit_defined_term(&mut self, ctx: &Defined_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#source}.
	 * @param ctx the parse tree
	 */
		fn visit_source(&mut self, ctx: &SourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#sources}.
	 * @param ctx the parse tree
	 */
		fn visit_sources(&mut self, ctx: &SourcesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#dag_source}.
	 * @param ctx the parse tree
	 */
		fn visit_dag_source(&mut self, ctx: &Dag_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#inference_record}.
	 * @param ctx the parse tree
	 */
		fn visit_inference_record(&mut self, ctx: &Inference_recordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#inference_rule}.
	 * @param ctx the parse tree
	 */
		fn visit_inference_rule(&mut self, ctx: &Inference_ruleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#internal_source}.
	 * @param ctx the parse tree
	 */
		fn visit_internal_source(&mut self, ctx: &Internal_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#intro_type}.
	 * @param ctx the parse tree
	 */
		fn visit_intro_type(&mut self, ctx: &Intro_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#external_source}.
	 * @param ctx the parse tree
	 */
		fn visit_external_source(&mut self, ctx: &External_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#file_source}.
	 * @param ctx the parse tree
	 */
		fn visit_file_source(&mut self, ctx: &File_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#file_info}.
	 * @param ctx the parse tree
	 */
		fn visit_file_info(&mut self, ctx: &File_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#theory}.
	 * @param ctx the parse tree
	 */
		fn visit_theory(&mut self, ctx: &TheoryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#theory_name}.
	 * @param ctx the parse tree
	 */
		fn visit_theory_name(&mut self, ctx: &Theory_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#creator_source}.
	 * @param ctx the parse tree
	 */
		fn visit_creator_source(&mut self, ctx: &Creator_sourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#creator_name}.
	 * @param ctx the parse tree
	 */
		fn visit_creator_name(&mut self, ctx: &Creator_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#parents}.
	 * @param ctx the parse tree
	 */
		fn visit_parents(&mut self, ctx: &ParentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#parent_list}.
	 * @param ctx the parse tree
	 */
		fn visit_parent_list(&mut self, ctx: &Parent_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_parent_info}.
	 * @param ctx the parse tree
	 */
		fn visit_comma_parent_info(&mut self, ctx: &Comma_parent_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#parent_info}.
	 * @param ctx the parse tree
	 */
		fn visit_parent_info(&mut self, ctx: &Parent_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#parent_details}.
	 * @param ctx the parse tree
	 */
		fn visit_parent_details(&mut self, ctx: &Parent_detailsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#optional_info}.
	 * @param ctx the parse tree
	 */
		fn visit_optional_info(&mut self, ctx: &Optional_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#useful_info}.
	 * @param ctx the parse tree
	 */
		fn visit_useful_info(&mut self, ctx: &Useful_infoContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#include}.
	 * @param ctx the parse tree
	 */
		fn visit_include(&mut self, ctx: &IncludeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#include_optionals}.
	 * @param ctx the parse tree
	 */
		fn visit_include_optionals(&mut self, ctx: &Include_optionalsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#formula_selection}.
	 * @param ctx the parse tree
	 */
		fn visit_formula_selection(&mut self, ctx: &Formula_selectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#name_list}.
	 * @param ctx the parse tree
	 */
		fn visit_name_list(&mut self, ctx: &Name_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#space_name}.
	 * @param ctx the parse tree
	 */
		fn visit_space_name(&mut self, ctx: &Space_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_term}.
	 * @param ctx the parse tree
	 */
		fn visit_general_term(&mut self, ctx: &General_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_data}.
	 * @param ctx the parse tree
	 */
		fn visit_general_data(&mut self, ctx: &General_dataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_function}.
	 * @param ctx the parse tree
	 */
		fn visit_general_function(&mut self, ctx: &General_functionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#formula_data}.
	 * @param ctx the parse tree
	 */
		fn visit_formula_data(&mut self, ctx: &Formula_dataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_list}.
	 * @param ctx the parse tree
	 */
		fn visit_general_list(&mut self, ctx: &General_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#general_terms}.
	 * @param ctx the parse tree
	 */
		fn visit_general_terms(&mut self, ctx: &General_termsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#comma_general_term}.
	 * @param ctx the parse tree
	 */
		fn visit_comma_general_term(&mut self, ctx: &Comma_general_termContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#name}.
	 * @param ctx the parse tree
	 */
		fn visit_name(&mut self, ctx: &NameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#atomic_word}.
	 * @param ctx the parse tree
	 */
		fn visit_atomic_word(&mut self, ctx: &Atomic_wordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#atomic_defined_word}.
	 * @param ctx the parse tree
	 */
		fn visit_atomic_defined_word(&mut self, ctx: &Atomic_defined_wordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#atomic_system_word}.
	 * @param ctx the parse tree
	 */
		fn visit_atomic_system_word(&mut self, ctx: &Atomic_system_wordContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_number(&mut self, ctx: &NumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#file_name}.
	 * @param ctx the parse tree
	 */
		fn visit_file_name(&mut self, ctx: &File_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link TPTPParser#nothing}.
	 * @param ctx the parse tree
	 */
		fn visit_nothing(&mut self, ctx: &NothingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> TPTPVisitor<'input> for T
where
	T: TPTPVisitorCompat<'input>
{
	fn visit_tptp_file(&mut self, ctx: &Tptp_fileContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tptp_file(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tptp_input(&mut self, ctx: &Tptp_inputContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tptp_input(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotated_formula(&mut self, ctx: &Annotated_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_annotated_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tpi_annotated(&mut self, ctx: &Tpi_annotatedContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tpi_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tpi_formula(&mut self, ctx: &Tpi_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tpi_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_annotated(&mut self, ctx: &Thf_annotatedContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_annotated(&mut self, ctx: &Tff_annotatedContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tcf_annotated(&mut self, ctx: &Tcf_annotatedContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tcf_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_annotated(&mut self, ctx: &Fof_annotatedContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cnf_annotated(&mut self, ctx: &Cnf_annotatedContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_cnf_annotated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotations(&mut self, ctx: &AnnotationsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_annotations(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formula_role(&mut self, ctx: &Formula_roleContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_formula_role(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_formula(&mut self, ctx: &Thf_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_logic_formula(&mut self, ctx: &Thf_logic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_binary_formula(&mut self, ctx: &Thf_binary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_binary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_binary_nonassoc(&mut self, ctx: &Thf_binary_nonassocContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_binary_nonassoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_binary_assoc(&mut self, ctx: &Thf_binary_assocContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_binary_assoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_or_formula(&mut self, ctx: &Thf_or_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_or_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_and_formula(&mut self, ctx: &Thf_and_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_and_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_apply_formula(&mut self, ctx: &Thf_apply_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_apply_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unit_formula(&mut self, ctx: &Thf_unit_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_unit_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_preunit_formula(&mut self, ctx: &Thf_preunit_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_preunit_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unitary_formula(&mut self, ctx: &Thf_unitary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_unitary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_quantified_formula(&mut self, ctx: &Thf_quantified_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_quantified_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_quantification(&mut self, ctx: &Thf_quantificationContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_quantification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_variable_list(&mut self, ctx: &Thf_variable_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_variable_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_typed_variable(&mut self, ctx: &Thf_typed_variableContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_typed_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unary_formula(&mut self, ctx: &Thf_unary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_unary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_prefix_unary(&mut self, ctx: &Thf_prefix_unaryContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_prefix_unary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_infix_unary(&mut self, ctx: &Thf_infix_unaryContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_infix_unary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_atomic_formula(&mut self, ctx: &Thf_atomic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_plain_atomic(&mut self, ctx: &Thf_plain_atomicContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_plain_atomic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_defined_atomic(&mut self, ctx: &Thf_defined_atomicContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_defined_atomic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_defined_term(&mut self, ctx: &Thf_defined_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_defined_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_defined_infix(&mut self, ctx: &Thf_defined_infixContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_defined_infix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_system_atomic(&mut self, ctx: &Thf_system_atomicContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_system_atomic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_let(&mut self, ctx: &Thf_letContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_let(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_let_types(&mut self, ctx: &Thf_let_typesContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_let_types(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_atom_typing_list(&mut self, ctx: &Thf_atom_typing_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_atom_typing_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_let_defns(&mut self, ctx: &Thf_let_defnsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_let_defns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_let_defn(&mut self, ctx: &Thf_let_defnContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_let_defn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_let_defn_list(&mut self, ctx: &Thf_let_defn_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_let_defn_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unitary_term(&mut self, ctx: &Thf_unitary_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_unitary_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_conn_term(&mut self, ctx: &Thf_conn_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_conn_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_tuple(&mut self, ctx: &Thf_tupleContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_fof_function(&mut self, ctx: &Thf_fof_functionContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_fof_function(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_arguments(&mut self, ctx: &Thf_argumentsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_formula_list(&mut self, ctx: &Thf_formula_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_formula_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comma_thf_logic_formula(&mut self, ctx: &Comma_thf_logic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_comma_thf_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_atom_typing(&mut self, ctx: &Thf_atom_typingContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_atom_typing(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_top_level_type(&mut self, ctx: &Thf_top_level_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_top_level_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unitary_type(&mut self, ctx: &Thf_unitary_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_unitary_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_apply_type(&mut self, ctx: &Thf_apply_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_apply_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_binary_type(&mut self, ctx: &Thf_binary_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_binary_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_mapping_type(&mut self, ctx: &Thf_mapping_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_mapping_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_xprod_type(&mut self, ctx: &Thf_xprod_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_xprod_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_union_type(&mut self, ctx: &Thf_union_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_union_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_subtype(&mut self, ctx: &Thf_subtypeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_subtype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_definition(&mut self, ctx: &Thf_definitionContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_definition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_sequent(&mut self, ctx: &Thf_sequentContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_sequent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_formula(&mut self, ctx: &Tff_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_logic_formula(&mut self, ctx: &Tff_logic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_binary_formula(&mut self, ctx: &Tff_binary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_binary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_binary_nonassoc(&mut self, ctx: &Tff_binary_nonassocContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_binary_nonassoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_binary_assoc(&mut self, ctx: &Tff_binary_assocContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_binary_assoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_or_formula(&mut self, ctx: &Tff_or_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_or_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_and_formula(&mut self, ctx: &Tff_and_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_and_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_unit_formula(&mut self, ctx: &Tff_unit_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_unit_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_preunit_formula(&mut self, ctx: &Tff_preunit_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_preunit_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_unitary_formula(&mut self, ctx: &Tff_unitary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_unitary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_unitary_formula(&mut self, ctx: &Txf_unitary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_unitary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_quantified_formula(&mut self, ctx: &Tff_quantified_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_quantified_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_variable_list(&mut self, ctx: &Tff_variable_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_variable_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_variable(&mut self, ctx: &Tff_variableContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_typed_variable(&mut self, ctx: &Tff_typed_variableContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_typed_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_unary_formula(&mut self, ctx: &Tff_unary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_unary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_prefix_unary(&mut self, ctx: &Tff_prefix_unaryContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_prefix_unary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_infix_unary(&mut self, ctx: &Tff_infix_unaryContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_infix_unary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_atomic_formula(&mut self, ctx: &Tff_atomic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_plain_atomic(&mut self, ctx: &Tff_plain_atomicContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_plain_atomic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_defined_atomic(&mut self, ctx: &Tff_defined_atomicContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_defined_atomic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_defined_plain(&mut self, ctx: &Tff_defined_plainContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_defined_plain(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_defined_infix(&mut self, ctx: &Tff_defined_infixContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_defined_infix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_system_atomic(&mut self, ctx: &Tff_system_atomicContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_system_atomic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_let(&mut self, ctx: &Txf_letContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_let(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_let_types(&mut self, ctx: &Txf_let_typesContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_let_types(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_atom_typing_list(&mut self, ctx: &Tff_atom_typing_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_atom_typing_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_let_defns(&mut self, ctx: &Txf_let_defnsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_let_defns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_let_defn(&mut self, ctx: &Txf_let_defnContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_let_defn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_let_LHS(&mut self, ctx: &Txf_let_LHSContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_let_LHS(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_let_defn_list(&mut self, ctx: &Txf_let_defn_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_let_defn_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nxf_atom(&mut self, ctx: &Nxf_atomContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nxf_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_term(&mut self, ctx: &Tff_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_unitary_term(&mut self, ctx: &Tff_unitary_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_unitary_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_tuple(&mut self, ctx: &Txf_tupleContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_arguments(&mut self, ctx: &Tff_argumentsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comma_tff_term(&mut self, ctx: &Comma_tff_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_comma_tff_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_atom_typing(&mut self, ctx: &Tff_atom_typingContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_atom_typing(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_top_level_type(&mut self, ctx: &Tff_top_level_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_top_level_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_non_atomic_type(&mut self, ctx: &Tff_non_atomic_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_non_atomic_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tf1_quantified_type(&mut self, ctx: &Tf1_quantified_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tf1_quantified_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_monotype(&mut self, ctx: &Tff_monotypeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_monotype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_unitary_type(&mut self, ctx: &Tff_unitary_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_unitary_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_atomic_type(&mut self, ctx: &Tff_atomic_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_atomic_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_type_arguments(&mut self, ctx: &Tff_type_argumentsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_type_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_mapping_type(&mut self, ctx: &Tff_mapping_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_mapping_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_xprod_type(&mut self, ctx: &Tff_xprod_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_xprod_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_tuple_type(&mut self, ctx: &Txf_tuple_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_tuple_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_type_list(&mut self, ctx: &Tff_type_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_type_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_subtype(&mut self, ctx: &Tff_subtypeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_subtype(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_definition(&mut self, ctx: &Txf_definitionContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_definition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_txf_sequent(&mut self, ctx: &Txf_sequentContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_txf_sequent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nhf_long_connective(&mut self, ctx: &Nhf_long_connectiveContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nhf_long_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nhf_parameter_list(&mut self, ctx: &Nhf_parameter_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nhf_parameter_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nhf_parameter(&mut self, ctx: &Nhf_parameterContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nhf_parameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nhf_key_pair(&mut self, ctx: &Nhf_key_pairContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nhf_key_pair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nxf_long_connective(&mut self, ctx: &Nxf_long_connectiveContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nxf_long_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nxf_parameter_list(&mut self, ctx: &Nxf_parameter_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nxf_parameter_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nxf_parameter(&mut self, ctx: &Nxf_parameterContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nxf_parameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nxf_key_pair(&mut self, ctx: &Nxf_key_pairContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nxf_key_pair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ntf_connective_name(&mut self, ctx: &Ntf_connective_nameContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_ntf_connective_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ntf_index(&mut self, ctx: &Ntf_indexContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_ntf_index(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ntf_short_connective(&mut self, ctx: &Ntf_short_connectiveContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_ntf_short_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tcf_formula(&mut self, ctx: &Tcf_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tcf_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tcf_logic_formula(&mut self, ctx: &Tcf_logic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tcf_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tcf_quantified_formula(&mut self, ctx: &Tcf_quantified_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tcf_quantified_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_formula(&mut self, ctx: &Fof_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_logic_formula(&mut self, ctx: &Fof_logic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_binary_formula(&mut self, ctx: &Fof_binary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_binary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_binary_nonassoc(&mut self, ctx: &Fof_binary_nonassocContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_binary_nonassoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_binary_assoc(&mut self, ctx: &Fof_binary_assocContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_binary_assoc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_or_formula(&mut self, ctx: &Fof_or_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_or_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_and_formula(&mut self, ctx: &Fof_and_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_and_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_unary_formula(&mut self, ctx: &Fof_unary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_unary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_infix_unary(&mut self, ctx: &Fof_infix_unaryContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_infix_unary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_unit_formula(&mut self, ctx: &Fof_unit_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_unit_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_unitary_formula(&mut self, ctx: &Fof_unitary_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_unitary_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_quantified_formula(&mut self, ctx: &Fof_quantified_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_quantified_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_variable_list(&mut self, ctx: &Fof_variable_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_variable_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_atomic_formula(&mut self, ctx: &Fof_atomic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_plain_atomic_formula(&mut self, ctx: &Fof_plain_atomic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_plain_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_atomic_formula(&mut self, ctx: &Fof_defined_atomic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_defined_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_plain_formula(&mut self, ctx: &Fof_defined_plain_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_defined_plain_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_infix_formula(&mut self, ctx: &Fof_defined_infix_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_defined_infix_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_system_atomic_formula(&mut self, ctx: &Fof_system_atomic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_system_atomic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_plain_term(&mut self, ctx: &Fof_plain_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_plain_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_term(&mut self, ctx: &Fof_defined_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_defined_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_atomic_term(&mut self, ctx: &Fof_defined_atomic_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_defined_atomic_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_defined_plain_term(&mut self, ctx: &Fof_defined_plain_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_defined_plain_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_system_term(&mut self, ctx: &Fof_system_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_system_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_arguments(&mut self, ctx: &Fof_argumentsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_term(&mut self, ctx: &Fof_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_function_term(&mut self, ctx: &Fof_function_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_function_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_sequent(&mut self, ctx: &Fof_sequentContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_sequent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_formula_tuple(&mut self, ctx: &Fof_formula_tupleContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_formula_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_formula_tuple_list(&mut self, ctx: &Fof_formula_tuple_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_formula_tuple_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comma_fof_logic_formula(&mut self, ctx: &Comma_fof_logic_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_comma_fof_logic_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cnf_formula(&mut self, ctx: &Cnf_formulaContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_cnf_formula(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cnf_disjunction(&mut self, ctx: &Cnf_disjunctionContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_cnf_disjunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cnf_literal(&mut self, ctx: &Cnf_literalContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_cnf_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_quantifier(&mut self, ctx: &Thf_quantifierContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_quantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_thf_unary_connective(&mut self, ctx: &Thf_unary_connectiveContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_thf_unary_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_th0_quantifier(&mut self, ctx: &Th0_quantifierContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_th0_quantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_quantifier(&mut self, ctx: &Type_quantifierContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_type_quantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subtype_sign(&mut self, ctx: &Subtype_signContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_subtype_sign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_unary_connective(&mut self, ctx: &Tff_unary_connectiveContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_unary_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tff_quantifier(&mut self, ctx: &Tff_quantifierContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_tff_quantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fof_quantifier(&mut self, ctx: &Fof_quantifierContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_fof_quantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nonassoc_connective(&mut self, ctx: &Nonassoc_connectiveContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nonassoc_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assoc_connective(&mut self, ctx: &Assoc_connectiveContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_assoc_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unary_connective(&mut self, ctx: &Unary_connectiveContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_unary_connective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gentzen_arrow(&mut self, ctx: &Gentzen_arrowContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_gentzen_arrow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identical(&mut self, ctx: &IdenticalContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_identical(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_constant(&mut self, ctx: &Type_constantContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_type_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_functor(&mut self, ctx: &Type_functorContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_type_functor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_type(&mut self, ctx: &Defined_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_defined_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atom(&mut self, ctx: &AtomContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_untyped_atom(&mut self, ctx: &Untyped_atomContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_untyped_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_infix_pred(&mut self, ctx: &Defined_infix_predContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_defined_infix_pred(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_infix_equality(&mut self, ctx: &Infix_equalityContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_infix_equality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_infix_inequality(&mut self, ctx: &Infix_inequalityContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_infix_inequality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constant(&mut self, ctx: &ConstantContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functor(&mut self, ctx: &FunctorContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_functor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_constant(&mut self, ctx: &Defined_constantContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_defined_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_functor(&mut self, ctx: &Defined_functorContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_defined_functor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_system_constant(&mut self, ctx: &System_constantContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_system_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_system_functor(&mut self, ctx: &System_functorContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_system_functor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_def_or_sys_constant(&mut self, ctx: &Def_or_sys_constantContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_def_or_sys_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_th1_defined_term(&mut self, ctx: &Th1_defined_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_th1_defined_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defined_term(&mut self, ctx: &Defined_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_defined_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_source(&mut self, ctx: &SourceContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sources(&mut self, ctx: &SourcesContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_sources(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dag_source(&mut self, ctx: &Dag_sourceContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_dag_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inference_record(&mut self, ctx: &Inference_recordContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_inference_record(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inference_rule(&mut self, ctx: &Inference_ruleContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_inference_rule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_internal_source(&mut self, ctx: &Internal_sourceContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_internal_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_intro_type(&mut self, ctx: &Intro_typeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_intro_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_external_source(&mut self, ctx: &External_sourceContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_external_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file_source(&mut self, ctx: &File_sourceContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_file_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file_info(&mut self, ctx: &File_infoContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_file_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_theory(&mut self, ctx: &TheoryContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_theory(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_theory_name(&mut self, ctx: &Theory_nameContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_theory_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_creator_source(&mut self, ctx: &Creator_sourceContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_creator_source(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_creator_name(&mut self, ctx: &Creator_nameContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_creator_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parents(&mut self, ctx: &ParentsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_parents(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parent_list(&mut self, ctx: &Parent_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_parent_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comma_parent_info(&mut self, ctx: &Comma_parent_infoContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_comma_parent_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parent_info(&mut self, ctx: &Parent_infoContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_parent_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parent_details(&mut self, ctx: &Parent_detailsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_parent_details(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_optional_info(&mut self, ctx: &Optional_infoContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_optional_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_useful_info(&mut self, ctx: &Useful_infoContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_useful_info(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_include(&mut self, ctx: &IncludeContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_include(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_include_optionals(&mut self, ctx: &Include_optionalsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_include_optionals(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formula_selection(&mut self, ctx: &Formula_selectionContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_formula_selection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_name_list(&mut self, ctx: &Name_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_name_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_space_name(&mut self, ctx: &Space_nameContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_space_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_term(&mut self, ctx: &General_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_general_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_data(&mut self, ctx: &General_dataContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_general_data(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_function(&mut self, ctx: &General_functionContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_general_function(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formula_data(&mut self, ctx: &Formula_dataContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_formula_data(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_list(&mut self, ctx: &General_listContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_general_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_general_terms(&mut self, ctx: &General_termsContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_general_terms(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comma_general_term(&mut self, ctx: &Comma_general_termContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_comma_general_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_name(&mut self, ctx: &NameContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomic_word(&mut self, ctx: &Atomic_wordContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_atomic_word(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomic_defined_word(&mut self, ctx: &Atomic_defined_wordContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_atomic_defined_word(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomic_system_word(&mut self, ctx: &Atomic_system_wordContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_atomic_system_word(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_number(&mut self, ctx: &NumberContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file_name(&mut self, ctx: &File_nameContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_file_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nothing(&mut self, ctx: &NothingContext<'input>){
		let result = <Self as TPTPVisitorCompat>::visit_nothing(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}