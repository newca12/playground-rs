#![allow(nonstandard_style)]
// Generated from TPTP.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::tptpparser::*;

pub trait TPTPListener<'input> : ParseTreeListener<'input,TPTPParserContextType>{
/**
 * Enter a parse tree produced by {@link TPTPParser#tptp_file}.
 * @param ctx the parse tree
 */
fn enter_tptp_file(&mut self, _ctx: &Tptp_fileContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tptp_file}.
 * @param ctx the parse tree
 */
fn exit_tptp_file(&mut self, _ctx: &Tptp_fileContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tptp_input}.
 * @param ctx the parse tree
 */
fn enter_tptp_input(&mut self, _ctx: &Tptp_inputContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tptp_input}.
 * @param ctx the parse tree
 */
fn exit_tptp_input(&mut self, _ctx: &Tptp_inputContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#annotated_formula}.
 * @param ctx the parse tree
 */
fn enter_annotated_formula(&mut self, _ctx: &Annotated_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#annotated_formula}.
 * @param ctx the parse tree
 */
fn exit_annotated_formula(&mut self, _ctx: &Annotated_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tpi_annotated}.
 * @param ctx the parse tree
 */
fn enter_tpi_annotated(&mut self, _ctx: &Tpi_annotatedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tpi_annotated}.
 * @param ctx the parse tree
 */
fn exit_tpi_annotated(&mut self, _ctx: &Tpi_annotatedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tpi_formula}.
 * @param ctx the parse tree
 */
fn enter_tpi_formula(&mut self, _ctx: &Tpi_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tpi_formula}.
 * @param ctx the parse tree
 */
fn exit_tpi_formula(&mut self, _ctx: &Tpi_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_annotated}.
 * @param ctx the parse tree
 */
fn enter_thf_annotated(&mut self, _ctx: &Thf_annotatedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_annotated}.
 * @param ctx the parse tree
 */
fn exit_thf_annotated(&mut self, _ctx: &Thf_annotatedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_annotated}.
 * @param ctx the parse tree
 */
fn enter_tff_annotated(&mut self, _ctx: &Tff_annotatedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_annotated}.
 * @param ctx the parse tree
 */
fn exit_tff_annotated(&mut self, _ctx: &Tff_annotatedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tcf_annotated}.
 * @param ctx the parse tree
 */
fn enter_tcf_annotated(&mut self, _ctx: &Tcf_annotatedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tcf_annotated}.
 * @param ctx the parse tree
 */
fn exit_tcf_annotated(&mut self, _ctx: &Tcf_annotatedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_annotated}.
 * @param ctx the parse tree
 */
fn enter_fof_annotated(&mut self, _ctx: &Fof_annotatedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_annotated}.
 * @param ctx the parse tree
 */
fn exit_fof_annotated(&mut self, _ctx: &Fof_annotatedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#cnf_annotated}.
 * @param ctx the parse tree
 */
fn enter_cnf_annotated(&mut self, _ctx: &Cnf_annotatedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#cnf_annotated}.
 * @param ctx the parse tree
 */
fn exit_cnf_annotated(&mut self, _ctx: &Cnf_annotatedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#annotations}.
 * @param ctx the parse tree
 */
fn enter_annotations(&mut self, _ctx: &AnnotationsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#annotations}.
 * @param ctx the parse tree
 */
fn exit_annotations(&mut self, _ctx: &AnnotationsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#formula_role}.
 * @param ctx the parse tree
 */
fn enter_formula_role(&mut self, _ctx: &Formula_roleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#formula_role}.
 * @param ctx the parse tree
 */
fn exit_formula_role(&mut self, _ctx: &Formula_roleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_formula(&mut self, _ctx: &Thf_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_formula(&mut self, _ctx: &Thf_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_logic_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_logic_formula(&mut self, _ctx: &Thf_logic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_logic_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_logic_formula(&mut self, _ctx: &Thf_logic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_binary_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_binary_formula(&mut self, _ctx: &Thf_binary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_binary_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_binary_formula(&mut self, _ctx: &Thf_binary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_binary_nonassoc}.
 * @param ctx the parse tree
 */
fn enter_thf_binary_nonassoc(&mut self, _ctx: &Thf_binary_nonassocContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_binary_nonassoc}.
 * @param ctx the parse tree
 */
fn exit_thf_binary_nonassoc(&mut self, _ctx: &Thf_binary_nonassocContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_binary_assoc}.
 * @param ctx the parse tree
 */
fn enter_thf_binary_assoc(&mut self, _ctx: &Thf_binary_assocContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_binary_assoc}.
 * @param ctx the parse tree
 */
fn exit_thf_binary_assoc(&mut self, _ctx: &Thf_binary_assocContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_or_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_or_formula(&mut self, _ctx: &Thf_or_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_or_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_or_formula(&mut self, _ctx: &Thf_or_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_and_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_and_formula(&mut self, _ctx: &Thf_and_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_and_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_and_formula(&mut self, _ctx: &Thf_and_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_apply_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_apply_formula(&mut self, _ctx: &Thf_apply_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_apply_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_apply_formula(&mut self, _ctx: &Thf_apply_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_unit_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_unit_formula(&mut self, _ctx: &Thf_unit_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_unit_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_unit_formula(&mut self, _ctx: &Thf_unit_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_preunit_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_preunit_formula(&mut self, _ctx: &Thf_preunit_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_preunit_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_preunit_formula(&mut self, _ctx: &Thf_preunit_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_unitary_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_unitary_formula(&mut self, _ctx: &Thf_unitary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_unitary_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_unitary_formula(&mut self, _ctx: &Thf_unitary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_quantified_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_quantified_formula(&mut self, _ctx: &Thf_quantified_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_quantified_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_quantified_formula(&mut self, _ctx: &Thf_quantified_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_quantification}.
 * @param ctx the parse tree
 */
fn enter_thf_quantification(&mut self, _ctx: &Thf_quantificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_quantification}.
 * @param ctx the parse tree
 */
fn exit_thf_quantification(&mut self, _ctx: &Thf_quantificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_variable_list}.
 * @param ctx the parse tree
 */
fn enter_thf_variable_list(&mut self, _ctx: &Thf_variable_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_variable_list}.
 * @param ctx the parse tree
 */
fn exit_thf_variable_list(&mut self, _ctx: &Thf_variable_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_typed_variable}.
 * @param ctx the parse tree
 */
fn enter_thf_typed_variable(&mut self, _ctx: &Thf_typed_variableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_typed_variable}.
 * @param ctx the parse tree
 */
fn exit_thf_typed_variable(&mut self, _ctx: &Thf_typed_variableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_unary_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_unary_formula(&mut self, _ctx: &Thf_unary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_unary_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_unary_formula(&mut self, _ctx: &Thf_unary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_prefix_unary}.
 * @param ctx the parse tree
 */
fn enter_thf_prefix_unary(&mut self, _ctx: &Thf_prefix_unaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_prefix_unary}.
 * @param ctx the parse tree
 */
fn exit_thf_prefix_unary(&mut self, _ctx: &Thf_prefix_unaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_infix_unary}.
 * @param ctx the parse tree
 */
fn enter_thf_infix_unary(&mut self, _ctx: &Thf_infix_unaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_infix_unary}.
 * @param ctx the parse tree
 */
fn exit_thf_infix_unary(&mut self, _ctx: &Thf_infix_unaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_atomic_formula}.
 * @param ctx the parse tree
 */
fn enter_thf_atomic_formula(&mut self, _ctx: &Thf_atomic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_atomic_formula}.
 * @param ctx the parse tree
 */
fn exit_thf_atomic_formula(&mut self, _ctx: &Thf_atomic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_plain_atomic}.
 * @param ctx the parse tree
 */
fn enter_thf_plain_atomic(&mut self, _ctx: &Thf_plain_atomicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_plain_atomic}.
 * @param ctx the parse tree
 */
fn exit_thf_plain_atomic(&mut self, _ctx: &Thf_plain_atomicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_defined_atomic}.
 * @param ctx the parse tree
 */
fn enter_thf_defined_atomic(&mut self, _ctx: &Thf_defined_atomicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_defined_atomic}.
 * @param ctx the parse tree
 */
fn exit_thf_defined_atomic(&mut self, _ctx: &Thf_defined_atomicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_defined_term}.
 * @param ctx the parse tree
 */
fn enter_thf_defined_term(&mut self, _ctx: &Thf_defined_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_defined_term}.
 * @param ctx the parse tree
 */
fn exit_thf_defined_term(&mut self, _ctx: &Thf_defined_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_defined_infix}.
 * @param ctx the parse tree
 */
fn enter_thf_defined_infix(&mut self, _ctx: &Thf_defined_infixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_defined_infix}.
 * @param ctx the parse tree
 */
fn exit_thf_defined_infix(&mut self, _ctx: &Thf_defined_infixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_system_atomic}.
 * @param ctx the parse tree
 */
fn enter_thf_system_atomic(&mut self, _ctx: &Thf_system_atomicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_system_atomic}.
 * @param ctx the parse tree
 */
fn exit_thf_system_atomic(&mut self, _ctx: &Thf_system_atomicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_let}.
 * @param ctx the parse tree
 */
fn enter_thf_let(&mut self, _ctx: &Thf_letContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_let}.
 * @param ctx the parse tree
 */
fn exit_thf_let(&mut self, _ctx: &Thf_letContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_let_types}.
 * @param ctx the parse tree
 */
fn enter_thf_let_types(&mut self, _ctx: &Thf_let_typesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_let_types}.
 * @param ctx the parse tree
 */
fn exit_thf_let_types(&mut self, _ctx: &Thf_let_typesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_atom_typing_list}.
 * @param ctx the parse tree
 */
fn enter_thf_atom_typing_list(&mut self, _ctx: &Thf_atom_typing_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_atom_typing_list}.
 * @param ctx the parse tree
 */
fn exit_thf_atom_typing_list(&mut self, _ctx: &Thf_atom_typing_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_let_defns}.
 * @param ctx the parse tree
 */
fn enter_thf_let_defns(&mut self, _ctx: &Thf_let_defnsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_let_defns}.
 * @param ctx the parse tree
 */
fn exit_thf_let_defns(&mut self, _ctx: &Thf_let_defnsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_let_defn}.
 * @param ctx the parse tree
 */
fn enter_thf_let_defn(&mut self, _ctx: &Thf_let_defnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_let_defn}.
 * @param ctx the parse tree
 */
fn exit_thf_let_defn(&mut self, _ctx: &Thf_let_defnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_let_defn_list}.
 * @param ctx the parse tree
 */
fn enter_thf_let_defn_list(&mut self, _ctx: &Thf_let_defn_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_let_defn_list}.
 * @param ctx the parse tree
 */
fn exit_thf_let_defn_list(&mut self, _ctx: &Thf_let_defn_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_unitary_term}.
 * @param ctx the parse tree
 */
fn enter_thf_unitary_term(&mut self, _ctx: &Thf_unitary_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_unitary_term}.
 * @param ctx the parse tree
 */
fn exit_thf_unitary_term(&mut self, _ctx: &Thf_unitary_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_conn_term}.
 * @param ctx the parse tree
 */
fn enter_thf_conn_term(&mut self, _ctx: &Thf_conn_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_conn_term}.
 * @param ctx the parse tree
 */
fn exit_thf_conn_term(&mut self, _ctx: &Thf_conn_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_tuple}.
 * @param ctx the parse tree
 */
fn enter_thf_tuple(&mut self, _ctx: &Thf_tupleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_tuple}.
 * @param ctx the parse tree
 */
fn exit_thf_tuple(&mut self, _ctx: &Thf_tupleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_fof_function}.
 * @param ctx the parse tree
 */
fn enter_thf_fof_function(&mut self, _ctx: &Thf_fof_functionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_fof_function}.
 * @param ctx the parse tree
 */
fn exit_thf_fof_function(&mut self, _ctx: &Thf_fof_functionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_arguments}.
 * @param ctx the parse tree
 */
fn enter_thf_arguments(&mut self, _ctx: &Thf_argumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_arguments}.
 * @param ctx the parse tree
 */
fn exit_thf_arguments(&mut self, _ctx: &Thf_argumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_formula_list}.
 * @param ctx the parse tree
 */
fn enter_thf_formula_list(&mut self, _ctx: &Thf_formula_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_formula_list}.
 * @param ctx the parse tree
 */
fn exit_thf_formula_list(&mut self, _ctx: &Thf_formula_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#comma_thf_logic_formula}.
 * @param ctx the parse tree
 */
fn enter_comma_thf_logic_formula(&mut self, _ctx: &Comma_thf_logic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#comma_thf_logic_formula}.
 * @param ctx the parse tree
 */
fn exit_comma_thf_logic_formula(&mut self, _ctx: &Comma_thf_logic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_atom_typing}.
 * @param ctx the parse tree
 */
fn enter_thf_atom_typing(&mut self, _ctx: &Thf_atom_typingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_atom_typing}.
 * @param ctx the parse tree
 */
fn exit_thf_atom_typing(&mut self, _ctx: &Thf_atom_typingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_top_level_type}.
 * @param ctx the parse tree
 */
fn enter_thf_top_level_type(&mut self, _ctx: &Thf_top_level_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_top_level_type}.
 * @param ctx the parse tree
 */
fn exit_thf_top_level_type(&mut self, _ctx: &Thf_top_level_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_unitary_type}.
 * @param ctx the parse tree
 */
fn enter_thf_unitary_type(&mut self, _ctx: &Thf_unitary_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_unitary_type}.
 * @param ctx the parse tree
 */
fn exit_thf_unitary_type(&mut self, _ctx: &Thf_unitary_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_apply_type}.
 * @param ctx the parse tree
 */
fn enter_thf_apply_type(&mut self, _ctx: &Thf_apply_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_apply_type}.
 * @param ctx the parse tree
 */
fn exit_thf_apply_type(&mut self, _ctx: &Thf_apply_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_binary_type}.
 * @param ctx the parse tree
 */
fn enter_thf_binary_type(&mut self, _ctx: &Thf_binary_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_binary_type}.
 * @param ctx the parse tree
 */
fn exit_thf_binary_type(&mut self, _ctx: &Thf_binary_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_mapping_type}.
 * @param ctx the parse tree
 */
fn enter_thf_mapping_type(&mut self, _ctx: &Thf_mapping_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_mapping_type}.
 * @param ctx the parse tree
 */
fn exit_thf_mapping_type(&mut self, _ctx: &Thf_mapping_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_xprod_type}.
 * @param ctx the parse tree
 */
fn enter_thf_xprod_type(&mut self, _ctx: &Thf_xprod_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_xprod_type}.
 * @param ctx the parse tree
 */
fn exit_thf_xprod_type(&mut self, _ctx: &Thf_xprod_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_union_type}.
 * @param ctx the parse tree
 */
fn enter_thf_union_type(&mut self, _ctx: &Thf_union_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_union_type}.
 * @param ctx the parse tree
 */
fn exit_thf_union_type(&mut self, _ctx: &Thf_union_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_subtype}.
 * @param ctx the parse tree
 */
fn enter_thf_subtype(&mut self, _ctx: &Thf_subtypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_subtype}.
 * @param ctx the parse tree
 */
fn exit_thf_subtype(&mut self, _ctx: &Thf_subtypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_definition}.
 * @param ctx the parse tree
 */
fn enter_thf_definition(&mut self, _ctx: &Thf_definitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_definition}.
 * @param ctx the parse tree
 */
fn exit_thf_definition(&mut self, _ctx: &Thf_definitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_sequent}.
 * @param ctx the parse tree
 */
fn enter_thf_sequent(&mut self, _ctx: &Thf_sequentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_sequent}.
 * @param ctx the parse tree
 */
fn exit_thf_sequent(&mut self, _ctx: &Thf_sequentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_formula(&mut self, _ctx: &Tff_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_formula(&mut self, _ctx: &Tff_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_logic_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_logic_formula(&mut self, _ctx: &Tff_logic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_logic_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_logic_formula(&mut self, _ctx: &Tff_logic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_binary_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_binary_formula(&mut self, _ctx: &Tff_binary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_binary_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_binary_formula(&mut self, _ctx: &Tff_binary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_binary_nonassoc}.
 * @param ctx the parse tree
 */
fn enter_tff_binary_nonassoc(&mut self, _ctx: &Tff_binary_nonassocContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_binary_nonassoc}.
 * @param ctx the parse tree
 */
fn exit_tff_binary_nonassoc(&mut self, _ctx: &Tff_binary_nonassocContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_binary_assoc}.
 * @param ctx the parse tree
 */
fn enter_tff_binary_assoc(&mut self, _ctx: &Tff_binary_assocContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_binary_assoc}.
 * @param ctx the parse tree
 */
fn exit_tff_binary_assoc(&mut self, _ctx: &Tff_binary_assocContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_or_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_or_formula(&mut self, _ctx: &Tff_or_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_or_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_or_formula(&mut self, _ctx: &Tff_or_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_and_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_and_formula(&mut self, _ctx: &Tff_and_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_and_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_and_formula(&mut self, _ctx: &Tff_and_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_unit_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_unit_formula(&mut self, _ctx: &Tff_unit_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_unit_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_unit_formula(&mut self, _ctx: &Tff_unit_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_preunit_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_preunit_formula(&mut self, _ctx: &Tff_preunit_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_preunit_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_preunit_formula(&mut self, _ctx: &Tff_preunit_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_unitary_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_unitary_formula(&mut self, _ctx: &Tff_unitary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_unitary_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_unitary_formula(&mut self, _ctx: &Tff_unitary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_unitary_formula}.
 * @param ctx the parse tree
 */
fn enter_txf_unitary_formula(&mut self, _ctx: &Txf_unitary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_unitary_formula}.
 * @param ctx the parse tree
 */
fn exit_txf_unitary_formula(&mut self, _ctx: &Txf_unitary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_quantified_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_quantified_formula(&mut self, _ctx: &Tff_quantified_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_quantified_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_quantified_formula(&mut self, _ctx: &Tff_quantified_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_variable_list}.
 * @param ctx the parse tree
 */
fn enter_tff_variable_list(&mut self, _ctx: &Tff_variable_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_variable_list}.
 * @param ctx the parse tree
 */
fn exit_tff_variable_list(&mut self, _ctx: &Tff_variable_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_variable}.
 * @param ctx the parse tree
 */
fn enter_tff_variable(&mut self, _ctx: &Tff_variableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_variable}.
 * @param ctx the parse tree
 */
fn exit_tff_variable(&mut self, _ctx: &Tff_variableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_typed_variable}.
 * @param ctx the parse tree
 */
fn enter_tff_typed_variable(&mut self, _ctx: &Tff_typed_variableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_typed_variable}.
 * @param ctx the parse tree
 */
fn exit_tff_typed_variable(&mut self, _ctx: &Tff_typed_variableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_unary_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_unary_formula(&mut self, _ctx: &Tff_unary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_unary_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_unary_formula(&mut self, _ctx: &Tff_unary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_prefix_unary}.
 * @param ctx the parse tree
 */
fn enter_tff_prefix_unary(&mut self, _ctx: &Tff_prefix_unaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_prefix_unary}.
 * @param ctx the parse tree
 */
fn exit_tff_prefix_unary(&mut self, _ctx: &Tff_prefix_unaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_infix_unary}.
 * @param ctx the parse tree
 */
fn enter_tff_infix_unary(&mut self, _ctx: &Tff_infix_unaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_infix_unary}.
 * @param ctx the parse tree
 */
fn exit_tff_infix_unary(&mut self, _ctx: &Tff_infix_unaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_atomic_formula}.
 * @param ctx the parse tree
 */
fn enter_tff_atomic_formula(&mut self, _ctx: &Tff_atomic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_atomic_formula}.
 * @param ctx the parse tree
 */
fn exit_tff_atomic_formula(&mut self, _ctx: &Tff_atomic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_plain_atomic}.
 * @param ctx the parse tree
 */
fn enter_tff_plain_atomic(&mut self, _ctx: &Tff_plain_atomicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_plain_atomic}.
 * @param ctx the parse tree
 */
fn exit_tff_plain_atomic(&mut self, _ctx: &Tff_plain_atomicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_defined_atomic}.
 * @param ctx the parse tree
 */
fn enter_tff_defined_atomic(&mut self, _ctx: &Tff_defined_atomicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_defined_atomic}.
 * @param ctx the parse tree
 */
fn exit_tff_defined_atomic(&mut self, _ctx: &Tff_defined_atomicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_defined_plain}.
 * @param ctx the parse tree
 */
fn enter_tff_defined_plain(&mut self, _ctx: &Tff_defined_plainContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_defined_plain}.
 * @param ctx the parse tree
 */
fn exit_tff_defined_plain(&mut self, _ctx: &Tff_defined_plainContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_defined_infix}.
 * @param ctx the parse tree
 */
fn enter_tff_defined_infix(&mut self, _ctx: &Tff_defined_infixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_defined_infix}.
 * @param ctx the parse tree
 */
fn exit_tff_defined_infix(&mut self, _ctx: &Tff_defined_infixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_system_atomic}.
 * @param ctx the parse tree
 */
fn enter_tff_system_atomic(&mut self, _ctx: &Tff_system_atomicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_system_atomic}.
 * @param ctx the parse tree
 */
fn exit_tff_system_atomic(&mut self, _ctx: &Tff_system_atomicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_let}.
 * @param ctx the parse tree
 */
fn enter_txf_let(&mut self, _ctx: &Txf_letContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_let}.
 * @param ctx the parse tree
 */
fn exit_txf_let(&mut self, _ctx: &Txf_letContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_let_types}.
 * @param ctx the parse tree
 */
fn enter_txf_let_types(&mut self, _ctx: &Txf_let_typesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_let_types}.
 * @param ctx the parse tree
 */
fn exit_txf_let_types(&mut self, _ctx: &Txf_let_typesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_atom_typing_list}.
 * @param ctx the parse tree
 */
fn enter_tff_atom_typing_list(&mut self, _ctx: &Tff_atom_typing_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_atom_typing_list}.
 * @param ctx the parse tree
 */
fn exit_tff_atom_typing_list(&mut self, _ctx: &Tff_atom_typing_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_let_defns}.
 * @param ctx the parse tree
 */
fn enter_txf_let_defns(&mut self, _ctx: &Txf_let_defnsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_let_defns}.
 * @param ctx the parse tree
 */
fn exit_txf_let_defns(&mut self, _ctx: &Txf_let_defnsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_let_defn}.
 * @param ctx the parse tree
 */
fn enter_txf_let_defn(&mut self, _ctx: &Txf_let_defnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_let_defn}.
 * @param ctx the parse tree
 */
fn exit_txf_let_defn(&mut self, _ctx: &Txf_let_defnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_let_LHS}.
 * @param ctx the parse tree
 */
fn enter_txf_let_LHS(&mut self, _ctx: &Txf_let_LHSContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_let_LHS}.
 * @param ctx the parse tree
 */
fn exit_txf_let_LHS(&mut self, _ctx: &Txf_let_LHSContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_let_defn_list}.
 * @param ctx the parse tree
 */
fn enter_txf_let_defn_list(&mut self, _ctx: &Txf_let_defn_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_let_defn_list}.
 * @param ctx the parse tree
 */
fn exit_txf_let_defn_list(&mut self, _ctx: &Txf_let_defn_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nxf_atom}.
 * @param ctx the parse tree
 */
fn enter_nxf_atom(&mut self, _ctx: &Nxf_atomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nxf_atom}.
 * @param ctx the parse tree
 */
fn exit_nxf_atom(&mut self, _ctx: &Nxf_atomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_term}.
 * @param ctx the parse tree
 */
fn enter_tff_term(&mut self, _ctx: &Tff_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_term}.
 * @param ctx the parse tree
 */
fn exit_tff_term(&mut self, _ctx: &Tff_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_unitary_term}.
 * @param ctx the parse tree
 */
fn enter_tff_unitary_term(&mut self, _ctx: &Tff_unitary_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_unitary_term}.
 * @param ctx the parse tree
 */
fn exit_tff_unitary_term(&mut self, _ctx: &Tff_unitary_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_tuple}.
 * @param ctx the parse tree
 */
fn enter_txf_tuple(&mut self, _ctx: &Txf_tupleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_tuple}.
 * @param ctx the parse tree
 */
fn exit_txf_tuple(&mut self, _ctx: &Txf_tupleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_arguments}.
 * @param ctx the parse tree
 */
fn enter_tff_arguments(&mut self, _ctx: &Tff_argumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_arguments}.
 * @param ctx the parse tree
 */
fn exit_tff_arguments(&mut self, _ctx: &Tff_argumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#comma_tff_term}.
 * @param ctx the parse tree
 */
fn enter_comma_tff_term(&mut self, _ctx: &Comma_tff_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#comma_tff_term}.
 * @param ctx the parse tree
 */
fn exit_comma_tff_term(&mut self, _ctx: &Comma_tff_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_atom_typing}.
 * @param ctx the parse tree
 */
fn enter_tff_atom_typing(&mut self, _ctx: &Tff_atom_typingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_atom_typing}.
 * @param ctx the parse tree
 */
fn exit_tff_atom_typing(&mut self, _ctx: &Tff_atom_typingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_top_level_type}.
 * @param ctx the parse tree
 */
fn enter_tff_top_level_type(&mut self, _ctx: &Tff_top_level_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_top_level_type}.
 * @param ctx the parse tree
 */
fn exit_tff_top_level_type(&mut self, _ctx: &Tff_top_level_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_non_atomic_type}.
 * @param ctx the parse tree
 */
fn enter_tff_non_atomic_type(&mut self, _ctx: &Tff_non_atomic_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_non_atomic_type}.
 * @param ctx the parse tree
 */
fn exit_tff_non_atomic_type(&mut self, _ctx: &Tff_non_atomic_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tf1_quantified_type}.
 * @param ctx the parse tree
 */
fn enter_tf1_quantified_type(&mut self, _ctx: &Tf1_quantified_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tf1_quantified_type}.
 * @param ctx the parse tree
 */
fn exit_tf1_quantified_type(&mut self, _ctx: &Tf1_quantified_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_monotype}.
 * @param ctx the parse tree
 */
fn enter_tff_monotype(&mut self, _ctx: &Tff_monotypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_monotype}.
 * @param ctx the parse tree
 */
fn exit_tff_monotype(&mut self, _ctx: &Tff_monotypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_unitary_type}.
 * @param ctx the parse tree
 */
fn enter_tff_unitary_type(&mut self, _ctx: &Tff_unitary_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_unitary_type}.
 * @param ctx the parse tree
 */
fn exit_tff_unitary_type(&mut self, _ctx: &Tff_unitary_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_atomic_type}.
 * @param ctx the parse tree
 */
fn enter_tff_atomic_type(&mut self, _ctx: &Tff_atomic_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_atomic_type}.
 * @param ctx the parse tree
 */
fn exit_tff_atomic_type(&mut self, _ctx: &Tff_atomic_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_type_arguments}.
 * @param ctx the parse tree
 */
fn enter_tff_type_arguments(&mut self, _ctx: &Tff_type_argumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_type_arguments}.
 * @param ctx the parse tree
 */
fn exit_tff_type_arguments(&mut self, _ctx: &Tff_type_argumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_mapping_type}.
 * @param ctx the parse tree
 */
fn enter_tff_mapping_type(&mut self, _ctx: &Tff_mapping_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_mapping_type}.
 * @param ctx the parse tree
 */
fn exit_tff_mapping_type(&mut self, _ctx: &Tff_mapping_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_xprod_type}.
 * @param ctx the parse tree
 */
fn enter_tff_xprod_type(&mut self, _ctx: &Tff_xprod_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_xprod_type}.
 * @param ctx the parse tree
 */
fn exit_tff_xprod_type(&mut self, _ctx: &Tff_xprod_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_tuple_type}.
 * @param ctx the parse tree
 */
fn enter_txf_tuple_type(&mut self, _ctx: &Txf_tuple_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_tuple_type}.
 * @param ctx the parse tree
 */
fn exit_txf_tuple_type(&mut self, _ctx: &Txf_tuple_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_type_list}.
 * @param ctx the parse tree
 */
fn enter_tff_type_list(&mut self, _ctx: &Tff_type_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_type_list}.
 * @param ctx the parse tree
 */
fn exit_tff_type_list(&mut self, _ctx: &Tff_type_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_subtype}.
 * @param ctx the parse tree
 */
fn enter_tff_subtype(&mut self, _ctx: &Tff_subtypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_subtype}.
 * @param ctx the parse tree
 */
fn exit_tff_subtype(&mut self, _ctx: &Tff_subtypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_definition}.
 * @param ctx the parse tree
 */
fn enter_txf_definition(&mut self, _ctx: &Txf_definitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_definition}.
 * @param ctx the parse tree
 */
fn exit_txf_definition(&mut self, _ctx: &Txf_definitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#txf_sequent}.
 * @param ctx the parse tree
 */
fn enter_txf_sequent(&mut self, _ctx: &Txf_sequentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#txf_sequent}.
 * @param ctx the parse tree
 */
fn exit_txf_sequent(&mut self, _ctx: &Txf_sequentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nhf_long_connective}.
 * @param ctx the parse tree
 */
fn enter_nhf_long_connective(&mut self, _ctx: &Nhf_long_connectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nhf_long_connective}.
 * @param ctx the parse tree
 */
fn exit_nhf_long_connective(&mut self, _ctx: &Nhf_long_connectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nhf_parameter_list}.
 * @param ctx the parse tree
 */
fn enter_nhf_parameter_list(&mut self, _ctx: &Nhf_parameter_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nhf_parameter_list}.
 * @param ctx the parse tree
 */
fn exit_nhf_parameter_list(&mut self, _ctx: &Nhf_parameter_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nhf_parameter}.
 * @param ctx the parse tree
 */
fn enter_nhf_parameter(&mut self, _ctx: &Nhf_parameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nhf_parameter}.
 * @param ctx the parse tree
 */
fn exit_nhf_parameter(&mut self, _ctx: &Nhf_parameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nhf_key_pair}.
 * @param ctx the parse tree
 */
fn enter_nhf_key_pair(&mut self, _ctx: &Nhf_key_pairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nhf_key_pair}.
 * @param ctx the parse tree
 */
fn exit_nhf_key_pair(&mut self, _ctx: &Nhf_key_pairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nxf_long_connective}.
 * @param ctx the parse tree
 */
fn enter_nxf_long_connective(&mut self, _ctx: &Nxf_long_connectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nxf_long_connective}.
 * @param ctx the parse tree
 */
fn exit_nxf_long_connective(&mut self, _ctx: &Nxf_long_connectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nxf_parameter_list}.
 * @param ctx the parse tree
 */
fn enter_nxf_parameter_list(&mut self, _ctx: &Nxf_parameter_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nxf_parameter_list}.
 * @param ctx the parse tree
 */
fn exit_nxf_parameter_list(&mut self, _ctx: &Nxf_parameter_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nxf_parameter}.
 * @param ctx the parse tree
 */
fn enter_nxf_parameter(&mut self, _ctx: &Nxf_parameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nxf_parameter}.
 * @param ctx the parse tree
 */
fn exit_nxf_parameter(&mut self, _ctx: &Nxf_parameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nxf_key_pair}.
 * @param ctx the parse tree
 */
fn enter_nxf_key_pair(&mut self, _ctx: &Nxf_key_pairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nxf_key_pair}.
 * @param ctx the parse tree
 */
fn exit_nxf_key_pair(&mut self, _ctx: &Nxf_key_pairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#ntf_connective_name}.
 * @param ctx the parse tree
 */
fn enter_ntf_connective_name(&mut self, _ctx: &Ntf_connective_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#ntf_connective_name}.
 * @param ctx the parse tree
 */
fn exit_ntf_connective_name(&mut self, _ctx: &Ntf_connective_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#ntf_index}.
 * @param ctx the parse tree
 */
fn enter_ntf_index(&mut self, _ctx: &Ntf_indexContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#ntf_index}.
 * @param ctx the parse tree
 */
fn exit_ntf_index(&mut self, _ctx: &Ntf_indexContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#ntf_short_connective}.
 * @param ctx the parse tree
 */
fn enter_ntf_short_connective(&mut self, _ctx: &Ntf_short_connectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#ntf_short_connective}.
 * @param ctx the parse tree
 */
fn exit_ntf_short_connective(&mut self, _ctx: &Ntf_short_connectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tcf_formula}.
 * @param ctx the parse tree
 */
fn enter_tcf_formula(&mut self, _ctx: &Tcf_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tcf_formula}.
 * @param ctx the parse tree
 */
fn exit_tcf_formula(&mut self, _ctx: &Tcf_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tcf_logic_formula}.
 * @param ctx the parse tree
 */
fn enter_tcf_logic_formula(&mut self, _ctx: &Tcf_logic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tcf_logic_formula}.
 * @param ctx the parse tree
 */
fn exit_tcf_logic_formula(&mut self, _ctx: &Tcf_logic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tcf_quantified_formula}.
 * @param ctx the parse tree
 */
fn enter_tcf_quantified_formula(&mut self, _ctx: &Tcf_quantified_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tcf_quantified_formula}.
 * @param ctx the parse tree
 */
fn exit_tcf_quantified_formula(&mut self, _ctx: &Tcf_quantified_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_formula(&mut self, _ctx: &Fof_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_formula(&mut self, _ctx: &Fof_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_logic_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_logic_formula(&mut self, _ctx: &Fof_logic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_logic_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_logic_formula(&mut self, _ctx: &Fof_logic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_binary_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_binary_formula(&mut self, _ctx: &Fof_binary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_binary_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_binary_formula(&mut self, _ctx: &Fof_binary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_binary_nonassoc}.
 * @param ctx the parse tree
 */
fn enter_fof_binary_nonassoc(&mut self, _ctx: &Fof_binary_nonassocContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_binary_nonassoc}.
 * @param ctx the parse tree
 */
fn exit_fof_binary_nonassoc(&mut self, _ctx: &Fof_binary_nonassocContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_binary_assoc}.
 * @param ctx the parse tree
 */
fn enter_fof_binary_assoc(&mut self, _ctx: &Fof_binary_assocContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_binary_assoc}.
 * @param ctx the parse tree
 */
fn exit_fof_binary_assoc(&mut self, _ctx: &Fof_binary_assocContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_or_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_or_formula(&mut self, _ctx: &Fof_or_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_or_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_or_formula(&mut self, _ctx: &Fof_or_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_and_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_and_formula(&mut self, _ctx: &Fof_and_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_and_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_and_formula(&mut self, _ctx: &Fof_and_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_unary_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_unary_formula(&mut self, _ctx: &Fof_unary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_unary_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_unary_formula(&mut self, _ctx: &Fof_unary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_infix_unary}.
 * @param ctx the parse tree
 */
fn enter_fof_infix_unary(&mut self, _ctx: &Fof_infix_unaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_infix_unary}.
 * @param ctx the parse tree
 */
fn exit_fof_infix_unary(&mut self, _ctx: &Fof_infix_unaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_unit_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_unit_formula(&mut self, _ctx: &Fof_unit_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_unit_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_unit_formula(&mut self, _ctx: &Fof_unit_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_unitary_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_unitary_formula(&mut self, _ctx: &Fof_unitary_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_unitary_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_unitary_formula(&mut self, _ctx: &Fof_unitary_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_quantified_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_quantified_formula(&mut self, _ctx: &Fof_quantified_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_quantified_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_quantified_formula(&mut self, _ctx: &Fof_quantified_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_variable_list}.
 * @param ctx the parse tree
 */
fn enter_fof_variable_list(&mut self, _ctx: &Fof_variable_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_variable_list}.
 * @param ctx the parse tree
 */
fn exit_fof_variable_list(&mut self, _ctx: &Fof_variable_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_atomic_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_atomic_formula(&mut self, _ctx: &Fof_atomic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_atomic_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_atomic_formula(&mut self, _ctx: &Fof_atomic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_plain_atomic_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_plain_atomic_formula(&mut self, _ctx: &Fof_plain_atomic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_plain_atomic_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_plain_atomic_formula(&mut self, _ctx: &Fof_plain_atomic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_defined_atomic_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_defined_atomic_formula(&mut self, _ctx: &Fof_defined_atomic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_defined_atomic_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_defined_atomic_formula(&mut self, _ctx: &Fof_defined_atomic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_defined_plain_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_defined_plain_formula(&mut self, _ctx: &Fof_defined_plain_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_defined_plain_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_defined_plain_formula(&mut self, _ctx: &Fof_defined_plain_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_defined_infix_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_defined_infix_formula(&mut self, _ctx: &Fof_defined_infix_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_defined_infix_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_defined_infix_formula(&mut self, _ctx: &Fof_defined_infix_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_system_atomic_formula}.
 * @param ctx the parse tree
 */
fn enter_fof_system_atomic_formula(&mut self, _ctx: &Fof_system_atomic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_system_atomic_formula}.
 * @param ctx the parse tree
 */
fn exit_fof_system_atomic_formula(&mut self, _ctx: &Fof_system_atomic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_plain_term}.
 * @param ctx the parse tree
 */
fn enter_fof_plain_term(&mut self, _ctx: &Fof_plain_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_plain_term}.
 * @param ctx the parse tree
 */
fn exit_fof_plain_term(&mut self, _ctx: &Fof_plain_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_defined_term}.
 * @param ctx the parse tree
 */
fn enter_fof_defined_term(&mut self, _ctx: &Fof_defined_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_defined_term}.
 * @param ctx the parse tree
 */
fn exit_fof_defined_term(&mut self, _ctx: &Fof_defined_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_defined_atomic_term}.
 * @param ctx the parse tree
 */
fn enter_fof_defined_atomic_term(&mut self, _ctx: &Fof_defined_atomic_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_defined_atomic_term}.
 * @param ctx the parse tree
 */
fn exit_fof_defined_atomic_term(&mut self, _ctx: &Fof_defined_atomic_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_defined_plain_term}.
 * @param ctx the parse tree
 */
fn enter_fof_defined_plain_term(&mut self, _ctx: &Fof_defined_plain_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_defined_plain_term}.
 * @param ctx the parse tree
 */
fn exit_fof_defined_plain_term(&mut self, _ctx: &Fof_defined_plain_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_system_term}.
 * @param ctx the parse tree
 */
fn enter_fof_system_term(&mut self, _ctx: &Fof_system_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_system_term}.
 * @param ctx the parse tree
 */
fn exit_fof_system_term(&mut self, _ctx: &Fof_system_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_arguments}.
 * @param ctx the parse tree
 */
fn enter_fof_arguments(&mut self, _ctx: &Fof_argumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_arguments}.
 * @param ctx the parse tree
 */
fn exit_fof_arguments(&mut self, _ctx: &Fof_argumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_term}.
 * @param ctx the parse tree
 */
fn enter_fof_term(&mut self, _ctx: &Fof_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_term}.
 * @param ctx the parse tree
 */
fn exit_fof_term(&mut self, _ctx: &Fof_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_function_term}.
 * @param ctx the parse tree
 */
fn enter_fof_function_term(&mut self, _ctx: &Fof_function_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_function_term}.
 * @param ctx the parse tree
 */
fn exit_fof_function_term(&mut self, _ctx: &Fof_function_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_sequent}.
 * @param ctx the parse tree
 */
fn enter_fof_sequent(&mut self, _ctx: &Fof_sequentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_sequent}.
 * @param ctx the parse tree
 */
fn exit_fof_sequent(&mut self, _ctx: &Fof_sequentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_formula_tuple}.
 * @param ctx the parse tree
 */
fn enter_fof_formula_tuple(&mut self, _ctx: &Fof_formula_tupleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_formula_tuple}.
 * @param ctx the parse tree
 */
fn exit_fof_formula_tuple(&mut self, _ctx: &Fof_formula_tupleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_formula_tuple_list}.
 * @param ctx the parse tree
 */
fn enter_fof_formula_tuple_list(&mut self, _ctx: &Fof_formula_tuple_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_formula_tuple_list}.
 * @param ctx the parse tree
 */
fn exit_fof_formula_tuple_list(&mut self, _ctx: &Fof_formula_tuple_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#comma_fof_logic_formula}.
 * @param ctx the parse tree
 */
fn enter_comma_fof_logic_formula(&mut self, _ctx: &Comma_fof_logic_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#comma_fof_logic_formula}.
 * @param ctx the parse tree
 */
fn exit_comma_fof_logic_formula(&mut self, _ctx: &Comma_fof_logic_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#cnf_formula}.
 * @param ctx the parse tree
 */
fn enter_cnf_formula(&mut self, _ctx: &Cnf_formulaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#cnf_formula}.
 * @param ctx the parse tree
 */
fn exit_cnf_formula(&mut self, _ctx: &Cnf_formulaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#cnf_disjunction}.
 * @param ctx the parse tree
 */
fn enter_cnf_disjunction(&mut self, _ctx: &Cnf_disjunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#cnf_disjunction}.
 * @param ctx the parse tree
 */
fn exit_cnf_disjunction(&mut self, _ctx: &Cnf_disjunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#cnf_literal}.
 * @param ctx the parse tree
 */
fn enter_cnf_literal(&mut self, _ctx: &Cnf_literalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#cnf_literal}.
 * @param ctx the parse tree
 */
fn exit_cnf_literal(&mut self, _ctx: &Cnf_literalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_quantifier}.
 * @param ctx the parse tree
 */
fn enter_thf_quantifier(&mut self, _ctx: &Thf_quantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_quantifier}.
 * @param ctx the parse tree
 */
fn exit_thf_quantifier(&mut self, _ctx: &Thf_quantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#thf_unary_connective}.
 * @param ctx the parse tree
 */
fn enter_thf_unary_connective(&mut self, _ctx: &Thf_unary_connectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#thf_unary_connective}.
 * @param ctx the parse tree
 */
fn exit_thf_unary_connective(&mut self, _ctx: &Thf_unary_connectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#th0_quantifier}.
 * @param ctx the parse tree
 */
fn enter_th0_quantifier(&mut self, _ctx: &Th0_quantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#th0_quantifier}.
 * @param ctx the parse tree
 */
fn exit_th0_quantifier(&mut self, _ctx: &Th0_quantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#type_quantifier}.
 * @param ctx the parse tree
 */
fn enter_type_quantifier(&mut self, _ctx: &Type_quantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#type_quantifier}.
 * @param ctx the parse tree
 */
fn exit_type_quantifier(&mut self, _ctx: &Type_quantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#subtype_sign}.
 * @param ctx the parse tree
 */
fn enter_subtype_sign(&mut self, _ctx: &Subtype_signContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#subtype_sign}.
 * @param ctx the parse tree
 */
fn exit_subtype_sign(&mut self, _ctx: &Subtype_signContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_unary_connective}.
 * @param ctx the parse tree
 */
fn enter_tff_unary_connective(&mut self, _ctx: &Tff_unary_connectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_unary_connective}.
 * @param ctx the parse tree
 */
fn exit_tff_unary_connective(&mut self, _ctx: &Tff_unary_connectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#tff_quantifier}.
 * @param ctx the parse tree
 */
fn enter_tff_quantifier(&mut self, _ctx: &Tff_quantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#tff_quantifier}.
 * @param ctx the parse tree
 */
fn exit_tff_quantifier(&mut self, _ctx: &Tff_quantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#fof_quantifier}.
 * @param ctx the parse tree
 */
fn enter_fof_quantifier(&mut self, _ctx: &Fof_quantifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#fof_quantifier}.
 * @param ctx the parse tree
 */
fn exit_fof_quantifier(&mut self, _ctx: &Fof_quantifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nonassoc_connective}.
 * @param ctx the parse tree
 */
fn enter_nonassoc_connective(&mut self, _ctx: &Nonassoc_connectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nonassoc_connective}.
 * @param ctx the parse tree
 */
fn exit_nonassoc_connective(&mut self, _ctx: &Nonassoc_connectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#assoc_connective}.
 * @param ctx the parse tree
 */
fn enter_assoc_connective(&mut self, _ctx: &Assoc_connectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#assoc_connective}.
 * @param ctx the parse tree
 */
fn exit_assoc_connective(&mut self, _ctx: &Assoc_connectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#unary_connective}.
 * @param ctx the parse tree
 */
fn enter_unary_connective(&mut self, _ctx: &Unary_connectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#unary_connective}.
 * @param ctx the parse tree
 */
fn exit_unary_connective(&mut self, _ctx: &Unary_connectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#gentzen_arrow}.
 * @param ctx the parse tree
 */
fn enter_gentzen_arrow(&mut self, _ctx: &Gentzen_arrowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#gentzen_arrow}.
 * @param ctx the parse tree
 */
fn exit_gentzen_arrow(&mut self, _ctx: &Gentzen_arrowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#identical}.
 * @param ctx the parse tree
 */
fn enter_identical(&mut self, _ctx: &IdenticalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#identical}.
 * @param ctx the parse tree
 */
fn exit_identical(&mut self, _ctx: &IdenticalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#type_constant}.
 * @param ctx the parse tree
 */
fn enter_type_constant(&mut self, _ctx: &Type_constantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#type_constant}.
 * @param ctx the parse tree
 */
fn exit_type_constant(&mut self, _ctx: &Type_constantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#type_functor}.
 * @param ctx the parse tree
 */
fn enter_type_functor(&mut self, _ctx: &Type_functorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#type_functor}.
 * @param ctx the parse tree
 */
fn exit_type_functor(&mut self, _ctx: &Type_functorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#defined_type}.
 * @param ctx the parse tree
 */
fn enter_defined_type(&mut self, _ctx: &Defined_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#defined_type}.
 * @param ctx the parse tree
 */
fn exit_defined_type(&mut self, _ctx: &Defined_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#untyped_atom}.
 * @param ctx the parse tree
 */
fn enter_untyped_atom(&mut self, _ctx: &Untyped_atomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#untyped_atom}.
 * @param ctx the parse tree
 */
fn exit_untyped_atom(&mut self, _ctx: &Untyped_atomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#defined_infix_pred}.
 * @param ctx the parse tree
 */
fn enter_defined_infix_pred(&mut self, _ctx: &Defined_infix_predContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#defined_infix_pred}.
 * @param ctx the parse tree
 */
fn exit_defined_infix_pred(&mut self, _ctx: &Defined_infix_predContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#infix_equality}.
 * @param ctx the parse tree
 */
fn enter_infix_equality(&mut self, _ctx: &Infix_equalityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#infix_equality}.
 * @param ctx the parse tree
 */
fn exit_infix_equality(&mut self, _ctx: &Infix_equalityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#infix_inequality}.
 * @param ctx the parse tree
 */
fn enter_infix_inequality(&mut self, _ctx: &Infix_inequalityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#infix_inequality}.
 * @param ctx the parse tree
 */
fn exit_infix_inequality(&mut self, _ctx: &Infix_inequalityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#constant}.
 * @param ctx the parse tree
 */
fn enter_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#constant}.
 * @param ctx the parse tree
 */
fn exit_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#functor}.
 * @param ctx the parse tree
 */
fn enter_functor(&mut self, _ctx: &FunctorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#functor}.
 * @param ctx the parse tree
 */
fn exit_functor(&mut self, _ctx: &FunctorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#defined_constant}.
 * @param ctx the parse tree
 */
fn enter_defined_constant(&mut self, _ctx: &Defined_constantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#defined_constant}.
 * @param ctx the parse tree
 */
fn exit_defined_constant(&mut self, _ctx: &Defined_constantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#defined_functor}.
 * @param ctx the parse tree
 */
fn enter_defined_functor(&mut self, _ctx: &Defined_functorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#defined_functor}.
 * @param ctx the parse tree
 */
fn exit_defined_functor(&mut self, _ctx: &Defined_functorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#system_constant}.
 * @param ctx the parse tree
 */
fn enter_system_constant(&mut self, _ctx: &System_constantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#system_constant}.
 * @param ctx the parse tree
 */
fn exit_system_constant(&mut self, _ctx: &System_constantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#system_functor}.
 * @param ctx the parse tree
 */
fn enter_system_functor(&mut self, _ctx: &System_functorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#system_functor}.
 * @param ctx the parse tree
 */
fn exit_system_functor(&mut self, _ctx: &System_functorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#def_or_sys_constant}.
 * @param ctx the parse tree
 */
fn enter_def_or_sys_constant(&mut self, _ctx: &Def_or_sys_constantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#def_or_sys_constant}.
 * @param ctx the parse tree
 */
fn exit_def_or_sys_constant(&mut self, _ctx: &Def_or_sys_constantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#th1_defined_term}.
 * @param ctx the parse tree
 */
fn enter_th1_defined_term(&mut self, _ctx: &Th1_defined_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#th1_defined_term}.
 * @param ctx the parse tree
 */
fn exit_th1_defined_term(&mut self, _ctx: &Th1_defined_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#defined_term}.
 * @param ctx the parse tree
 */
fn enter_defined_term(&mut self, _ctx: &Defined_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#defined_term}.
 * @param ctx the parse tree
 */
fn exit_defined_term(&mut self, _ctx: &Defined_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#variable}.
 * @param ctx the parse tree
 */
fn enter_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#variable}.
 * @param ctx the parse tree
 */
fn exit_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#source}.
 * @param ctx the parse tree
 */
fn enter_source(&mut self, _ctx: &SourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#source}.
 * @param ctx the parse tree
 */
fn exit_source(&mut self, _ctx: &SourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#sources}.
 * @param ctx the parse tree
 */
fn enter_sources(&mut self, _ctx: &SourcesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#sources}.
 * @param ctx the parse tree
 */
fn exit_sources(&mut self, _ctx: &SourcesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#dag_source}.
 * @param ctx the parse tree
 */
fn enter_dag_source(&mut self, _ctx: &Dag_sourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#dag_source}.
 * @param ctx the parse tree
 */
fn exit_dag_source(&mut self, _ctx: &Dag_sourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#inference_record}.
 * @param ctx the parse tree
 */
fn enter_inference_record(&mut self, _ctx: &Inference_recordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#inference_record}.
 * @param ctx the parse tree
 */
fn exit_inference_record(&mut self, _ctx: &Inference_recordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#inference_rule}.
 * @param ctx the parse tree
 */
fn enter_inference_rule(&mut self, _ctx: &Inference_ruleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#inference_rule}.
 * @param ctx the parse tree
 */
fn exit_inference_rule(&mut self, _ctx: &Inference_ruleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#internal_source}.
 * @param ctx the parse tree
 */
fn enter_internal_source(&mut self, _ctx: &Internal_sourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#internal_source}.
 * @param ctx the parse tree
 */
fn exit_internal_source(&mut self, _ctx: &Internal_sourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#intro_type}.
 * @param ctx the parse tree
 */
fn enter_intro_type(&mut self, _ctx: &Intro_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#intro_type}.
 * @param ctx the parse tree
 */
fn exit_intro_type(&mut self, _ctx: &Intro_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#external_source}.
 * @param ctx the parse tree
 */
fn enter_external_source(&mut self, _ctx: &External_sourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#external_source}.
 * @param ctx the parse tree
 */
fn exit_external_source(&mut self, _ctx: &External_sourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#file_source}.
 * @param ctx the parse tree
 */
fn enter_file_source(&mut self, _ctx: &File_sourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#file_source}.
 * @param ctx the parse tree
 */
fn exit_file_source(&mut self, _ctx: &File_sourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#file_info}.
 * @param ctx the parse tree
 */
fn enter_file_info(&mut self, _ctx: &File_infoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#file_info}.
 * @param ctx the parse tree
 */
fn exit_file_info(&mut self, _ctx: &File_infoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#theory}.
 * @param ctx the parse tree
 */
fn enter_theory(&mut self, _ctx: &TheoryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#theory}.
 * @param ctx the parse tree
 */
fn exit_theory(&mut self, _ctx: &TheoryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#theory_name}.
 * @param ctx the parse tree
 */
fn enter_theory_name(&mut self, _ctx: &Theory_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#theory_name}.
 * @param ctx the parse tree
 */
fn exit_theory_name(&mut self, _ctx: &Theory_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#creator_source}.
 * @param ctx the parse tree
 */
fn enter_creator_source(&mut self, _ctx: &Creator_sourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#creator_source}.
 * @param ctx the parse tree
 */
fn exit_creator_source(&mut self, _ctx: &Creator_sourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#creator_name}.
 * @param ctx the parse tree
 */
fn enter_creator_name(&mut self, _ctx: &Creator_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#creator_name}.
 * @param ctx the parse tree
 */
fn exit_creator_name(&mut self, _ctx: &Creator_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#parents}.
 * @param ctx the parse tree
 */
fn enter_parents(&mut self, _ctx: &ParentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#parents}.
 * @param ctx the parse tree
 */
fn exit_parents(&mut self, _ctx: &ParentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#parent_list}.
 * @param ctx the parse tree
 */
fn enter_parent_list(&mut self, _ctx: &Parent_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#parent_list}.
 * @param ctx the parse tree
 */
fn exit_parent_list(&mut self, _ctx: &Parent_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#comma_parent_info}.
 * @param ctx the parse tree
 */
fn enter_comma_parent_info(&mut self, _ctx: &Comma_parent_infoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#comma_parent_info}.
 * @param ctx the parse tree
 */
fn exit_comma_parent_info(&mut self, _ctx: &Comma_parent_infoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#parent_info}.
 * @param ctx the parse tree
 */
fn enter_parent_info(&mut self, _ctx: &Parent_infoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#parent_info}.
 * @param ctx the parse tree
 */
fn exit_parent_info(&mut self, _ctx: &Parent_infoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#parent_details}.
 * @param ctx the parse tree
 */
fn enter_parent_details(&mut self, _ctx: &Parent_detailsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#parent_details}.
 * @param ctx the parse tree
 */
fn exit_parent_details(&mut self, _ctx: &Parent_detailsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#optional_info}.
 * @param ctx the parse tree
 */
fn enter_optional_info(&mut self, _ctx: &Optional_infoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#optional_info}.
 * @param ctx the parse tree
 */
fn exit_optional_info(&mut self, _ctx: &Optional_infoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#useful_info}.
 * @param ctx the parse tree
 */
fn enter_useful_info(&mut self, _ctx: &Useful_infoContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#useful_info}.
 * @param ctx the parse tree
 */
fn exit_useful_info(&mut self, _ctx: &Useful_infoContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#include}.
 * @param ctx the parse tree
 */
fn enter_include(&mut self, _ctx: &IncludeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#include}.
 * @param ctx the parse tree
 */
fn exit_include(&mut self, _ctx: &IncludeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#include_optionals}.
 * @param ctx the parse tree
 */
fn enter_include_optionals(&mut self, _ctx: &Include_optionalsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#include_optionals}.
 * @param ctx the parse tree
 */
fn exit_include_optionals(&mut self, _ctx: &Include_optionalsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#formula_selection}.
 * @param ctx the parse tree
 */
fn enter_formula_selection(&mut self, _ctx: &Formula_selectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#formula_selection}.
 * @param ctx the parse tree
 */
fn exit_formula_selection(&mut self, _ctx: &Formula_selectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#name_list}.
 * @param ctx the parse tree
 */
fn enter_name_list(&mut self, _ctx: &Name_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#name_list}.
 * @param ctx the parse tree
 */
fn exit_name_list(&mut self, _ctx: &Name_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#space_name}.
 * @param ctx the parse tree
 */
fn enter_space_name(&mut self, _ctx: &Space_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#space_name}.
 * @param ctx the parse tree
 */
fn exit_space_name(&mut self, _ctx: &Space_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#general_term}.
 * @param ctx the parse tree
 */
fn enter_general_term(&mut self, _ctx: &General_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#general_term}.
 * @param ctx the parse tree
 */
fn exit_general_term(&mut self, _ctx: &General_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#general_data}.
 * @param ctx the parse tree
 */
fn enter_general_data(&mut self, _ctx: &General_dataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#general_data}.
 * @param ctx the parse tree
 */
fn exit_general_data(&mut self, _ctx: &General_dataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#general_function}.
 * @param ctx the parse tree
 */
fn enter_general_function(&mut self, _ctx: &General_functionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#general_function}.
 * @param ctx the parse tree
 */
fn exit_general_function(&mut self, _ctx: &General_functionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#formula_data}.
 * @param ctx the parse tree
 */
fn enter_formula_data(&mut self, _ctx: &Formula_dataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#formula_data}.
 * @param ctx the parse tree
 */
fn exit_formula_data(&mut self, _ctx: &Formula_dataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#general_list}.
 * @param ctx the parse tree
 */
fn enter_general_list(&mut self, _ctx: &General_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#general_list}.
 * @param ctx the parse tree
 */
fn exit_general_list(&mut self, _ctx: &General_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#general_terms}.
 * @param ctx the parse tree
 */
fn enter_general_terms(&mut self, _ctx: &General_termsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#general_terms}.
 * @param ctx the parse tree
 */
fn exit_general_terms(&mut self, _ctx: &General_termsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#comma_general_term}.
 * @param ctx the parse tree
 */
fn enter_comma_general_term(&mut self, _ctx: &Comma_general_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#comma_general_term}.
 * @param ctx the parse tree
 */
fn exit_comma_general_term(&mut self, _ctx: &Comma_general_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#name}.
 * @param ctx the parse tree
 */
fn enter_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#name}.
 * @param ctx the parse tree
 */
fn exit_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#atomic_word}.
 * @param ctx the parse tree
 */
fn enter_atomic_word(&mut self, _ctx: &Atomic_wordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#atomic_word}.
 * @param ctx the parse tree
 */
fn exit_atomic_word(&mut self, _ctx: &Atomic_wordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#atomic_defined_word}.
 * @param ctx the parse tree
 */
fn enter_atomic_defined_word(&mut self, _ctx: &Atomic_defined_wordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#atomic_defined_word}.
 * @param ctx the parse tree
 */
fn exit_atomic_defined_word(&mut self, _ctx: &Atomic_defined_wordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#atomic_system_word}.
 * @param ctx the parse tree
 */
fn enter_atomic_system_word(&mut self, _ctx: &Atomic_system_wordContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#atomic_system_word}.
 * @param ctx the parse tree
 */
fn exit_atomic_system_word(&mut self, _ctx: &Atomic_system_wordContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#number}.
 * @param ctx the parse tree
 */
fn enter_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#number}.
 * @param ctx the parse tree
 */
fn exit_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#file_name}.
 * @param ctx the parse tree
 */
fn enter_file_name(&mut self, _ctx: &File_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#file_name}.
 * @param ctx the parse tree
 */
fn exit_file_name(&mut self, _ctx: &File_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TPTPParser#nothing}.
 * @param ctx the parse tree
 */
fn enter_nothing(&mut self, _ctx: &NothingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TPTPParser#nothing}.
 * @param ctx the parse tree
 */
fn exit_nothing(&mut self, _ctx: &NothingContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : TPTPListener<'input> }


