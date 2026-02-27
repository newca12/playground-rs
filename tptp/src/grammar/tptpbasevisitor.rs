
// Generated from TPTP.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::tptpparser::*;

// A complete Visitor for a parse tree produced by TPTPParser.

pub trait TPTPBaseVisitor<'input>:
    ParseTreeVisitor<'input, TPTPParserContextType> {
	// Visit a parse tree produced by TPTPParser#tptp_file.
	fn visit_tptp_file(&mut self, ctx: &Tptp_fileContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tptp_input.
	fn visit_tptp_input(&mut self, ctx: &Tptp_inputContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#annotated_formula.
	fn visit_annotated_formula(&mut self, ctx: &Annotated_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tpi_annotated.
	fn visit_tpi_annotated(&mut self, ctx: &Tpi_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tpi_formula.
	fn visit_tpi_formula(&mut self, ctx: &Tpi_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_annotated.
	fn visit_thf_annotated(&mut self, ctx: &Thf_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_annotated.
	fn visit_tff_annotated(&mut self, ctx: &Tff_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tcf_annotated.
	fn visit_tcf_annotated(&mut self, ctx: &Tcf_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_annotated.
	fn visit_fof_annotated(&mut self, ctx: &Fof_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#cnf_annotated.
	fn visit_cnf_annotated(&mut self, ctx: &Cnf_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#annotations.
	fn visit_annotations(&mut self, ctx: &AnnotationsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#formula_role.
	fn visit_formula_role(&mut self, ctx: &Formula_roleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_formula.
	fn visit_thf_formula(&mut self, ctx: &Thf_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_logic_formula.
	fn visit_thf_logic_formula(&mut self, ctx: &Thf_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_binary_formula.
	fn visit_thf_binary_formula(&mut self, ctx: &Thf_binary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_binary_nonassoc.
	fn visit_thf_binary_nonassoc(&mut self, ctx: &Thf_binary_nonassocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_binary_assoc.
	fn visit_thf_binary_assoc(&mut self, ctx: &Thf_binary_assocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_or_formula.
	fn visit_thf_or_formula(&mut self, ctx: &Thf_or_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_and_formula.
	fn visit_thf_and_formula(&mut self, ctx: &Thf_and_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_apply_formula.
	fn visit_thf_apply_formula(&mut self, ctx: &Thf_apply_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_unit_formula.
	fn visit_thf_unit_formula(&mut self, ctx: &Thf_unit_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_preunit_formula.
	fn visit_thf_preunit_formula(&mut self, ctx: &Thf_preunit_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_unitary_formula.
	fn visit_thf_unitary_formula(&mut self, ctx: &Thf_unitary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_quantified_formula.
	fn visit_thf_quantified_formula(&mut self, ctx: &Thf_quantified_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_quantification.
	fn visit_thf_quantification(&mut self, ctx: &Thf_quantificationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_variable_list.
	fn visit_thf_variable_list(&mut self, ctx: &Thf_variable_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_typed_variable.
	fn visit_thf_typed_variable(&mut self, ctx: &Thf_typed_variableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_unary_formula.
	fn visit_thf_unary_formula(&mut self, ctx: &Thf_unary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_prefix_unary.
	fn visit_thf_prefix_unary(&mut self, ctx: &Thf_prefix_unaryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_infix_unary.
	fn visit_thf_infix_unary(&mut self, ctx: &Thf_infix_unaryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_atomic_formula.
	fn visit_thf_atomic_formula(&mut self, ctx: &Thf_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_plain_atomic.
	fn visit_thf_plain_atomic(&mut self, ctx: &Thf_plain_atomicContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_defined_atomic.
	fn visit_thf_defined_atomic(&mut self, ctx: &Thf_defined_atomicContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_defined_term.
	fn visit_thf_defined_term(&mut self, ctx: &Thf_defined_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_defined_infix.
	fn visit_thf_defined_infix(&mut self, ctx: &Thf_defined_infixContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_system_atomic.
	fn visit_thf_system_atomic(&mut self, ctx: &Thf_system_atomicContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_let.
	fn visit_thf_let(&mut self, ctx: &Thf_letContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_let_types.
	fn visit_thf_let_types(&mut self, ctx: &Thf_let_typesContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_atom_typing_list.
	fn visit_thf_atom_typing_list(&mut self, ctx: &Thf_atom_typing_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_let_defns.
	fn visit_thf_let_defns(&mut self, ctx: &Thf_let_defnsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_let_defn.
	fn visit_thf_let_defn(&mut self, ctx: &Thf_let_defnContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_let_defn_list.
	fn visit_thf_let_defn_list(&mut self, ctx: &Thf_let_defn_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_unitary_term.
	fn visit_thf_unitary_term(&mut self, ctx: &Thf_unitary_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_conn_term.
	fn visit_thf_conn_term(&mut self, ctx: &Thf_conn_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_tuple.
	fn visit_thf_tuple(&mut self, ctx: &Thf_tupleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_fof_function.
	fn visit_thf_fof_function(&mut self, ctx: &Thf_fof_functionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_arguments.
	fn visit_thf_arguments(&mut self, ctx: &Thf_argumentsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_formula_list.
	fn visit_thf_formula_list(&mut self, ctx: &Thf_formula_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#comma_thf_logic_formula.
	fn visit_comma_thf_logic_formula(&mut self, ctx: &Comma_thf_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_atom_typing.
	fn visit_thf_atom_typing(&mut self, ctx: &Thf_atom_typingContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_top_level_type.
	fn visit_thf_top_level_type(&mut self, ctx: &Thf_top_level_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_unitary_type.
	fn visit_thf_unitary_type(&mut self, ctx: &Thf_unitary_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_apply_type.
	fn visit_thf_apply_type(&mut self, ctx: &Thf_apply_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_binary_type.
	fn visit_thf_binary_type(&mut self, ctx: &Thf_binary_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_mapping_type.
	fn visit_thf_mapping_type(&mut self, ctx: &Thf_mapping_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_xprod_type.
	fn visit_thf_xprod_type(&mut self, ctx: &Thf_xprod_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_union_type.
	fn visit_thf_union_type(&mut self, ctx: &Thf_union_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_subtype.
	fn visit_thf_subtype(&mut self, ctx: &Thf_subtypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_definition.
	fn visit_thf_definition(&mut self, ctx: &Thf_definitionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_sequent.
	fn visit_thf_sequent(&mut self, ctx: &Thf_sequentContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_formula.
	fn visit_tff_formula(&mut self, ctx: &Tff_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_logic_formula.
	fn visit_tff_logic_formula(&mut self, ctx: &Tff_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_binary_formula.
	fn visit_tff_binary_formula(&mut self, ctx: &Tff_binary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_binary_nonassoc.
	fn visit_tff_binary_nonassoc(&mut self, ctx: &Tff_binary_nonassocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_binary_assoc.
	fn visit_tff_binary_assoc(&mut self, ctx: &Tff_binary_assocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_or_formula.
	fn visit_tff_or_formula(&mut self, ctx: &Tff_or_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_and_formula.
	fn visit_tff_and_formula(&mut self, ctx: &Tff_and_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_unit_formula.
	fn visit_tff_unit_formula(&mut self, ctx: &Tff_unit_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_preunit_formula.
	fn visit_tff_preunit_formula(&mut self, ctx: &Tff_preunit_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_unitary_formula.
	fn visit_tff_unitary_formula(&mut self, ctx: &Tff_unitary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_unitary_formula.
	fn visit_txf_unitary_formula(&mut self, ctx: &Txf_unitary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_quantified_formula.
	fn visit_tff_quantified_formula(&mut self, ctx: &Tff_quantified_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_variable_list.
	fn visit_tff_variable_list(&mut self, ctx: &Tff_variable_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_variable.
	fn visit_tff_variable(&mut self, ctx: &Tff_variableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_typed_variable.
	fn visit_tff_typed_variable(&mut self, ctx: &Tff_typed_variableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_unary_formula.
	fn visit_tff_unary_formula(&mut self, ctx: &Tff_unary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_prefix_unary.
	fn visit_tff_prefix_unary(&mut self, ctx: &Tff_prefix_unaryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_infix_unary.
	fn visit_tff_infix_unary(&mut self, ctx: &Tff_infix_unaryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_atomic_formula.
	fn visit_tff_atomic_formula(&mut self, ctx: &Tff_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_plain_atomic.
	fn visit_tff_plain_atomic(&mut self, ctx: &Tff_plain_atomicContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_defined_atomic.
	fn visit_tff_defined_atomic(&mut self, ctx: &Tff_defined_atomicContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_defined_plain.
	fn visit_tff_defined_plain(&mut self, ctx: &Tff_defined_plainContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_defined_infix.
	fn visit_tff_defined_infix(&mut self, ctx: &Tff_defined_infixContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_system_atomic.
	fn visit_tff_system_atomic(&mut self, ctx: &Tff_system_atomicContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_let.
	fn visit_txf_let(&mut self, ctx: &Txf_letContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_let_types.
	fn visit_txf_let_types(&mut self, ctx: &Txf_let_typesContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_atom_typing_list.
	fn visit_tff_atom_typing_list(&mut self, ctx: &Tff_atom_typing_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_let_defns.
	fn visit_txf_let_defns(&mut self, ctx: &Txf_let_defnsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_let_defn.
	fn visit_txf_let_defn(&mut self, ctx: &Txf_let_defnContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_let_LHS.
	fn visit_txf_let_lhs(&mut self, ctx: &Txf_let_LHSContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_let_defn_list.
	fn visit_txf_let_defn_list(&mut self, ctx: &Txf_let_defn_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nxf_atom.
	fn visit_nxf_atom(&mut self, ctx: &Nxf_atomContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_term.
	fn visit_tff_term(&mut self, ctx: &Tff_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_unitary_term.
	fn visit_tff_unitary_term(&mut self, ctx: &Tff_unitary_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_tuple.
	fn visit_txf_tuple(&mut self, ctx: &Txf_tupleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_arguments.
	fn visit_tff_arguments(&mut self, ctx: &Tff_argumentsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#comma_tff_term.
	fn visit_comma_tff_term(&mut self, ctx: &Comma_tff_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_atom_typing.
	fn visit_tff_atom_typing(&mut self, ctx: &Tff_atom_typingContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_top_level_type.
	fn visit_tff_top_level_type(&mut self, ctx: &Tff_top_level_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_non_atomic_type.
	fn visit_tff_non_atomic_type(&mut self, ctx: &Tff_non_atomic_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tf1_quantified_type.
	fn visit_tf1_quantified_type(&mut self, ctx: &Tf1_quantified_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_monotype.
	fn visit_tff_monotype(&mut self, ctx: &Tff_monotypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_unitary_type.
	fn visit_tff_unitary_type(&mut self, ctx: &Tff_unitary_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_atomic_type.
	fn visit_tff_atomic_type(&mut self, ctx: &Tff_atomic_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_type_arguments.
	fn visit_tff_type_arguments(&mut self, ctx: &Tff_type_argumentsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_mapping_type.
	fn visit_tff_mapping_type(&mut self, ctx: &Tff_mapping_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_xprod_type.
	fn visit_tff_xprod_type(&mut self, ctx: &Tff_xprod_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_tuple_type.
	fn visit_txf_tuple_type(&mut self, ctx: &Txf_tuple_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_type_list.
	fn visit_tff_type_list(&mut self, ctx: &Tff_type_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_subtype.
	fn visit_tff_subtype(&mut self, ctx: &Tff_subtypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_definition.
	fn visit_txf_definition(&mut self, ctx: &Txf_definitionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#txf_sequent.
	fn visit_txf_sequent(&mut self, ctx: &Txf_sequentContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nhf_long_connective.
	fn visit_nhf_long_connective(&mut self, ctx: &Nhf_long_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nhf_parameter_list.
	fn visit_nhf_parameter_list(&mut self, ctx: &Nhf_parameter_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nhf_parameter.
	fn visit_nhf_parameter(&mut self, ctx: &Nhf_parameterContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nhf_key_pair.
	fn visit_nhf_key_pair(&mut self, ctx: &Nhf_key_pairContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nxf_long_connective.
	fn visit_nxf_long_connective(&mut self, ctx: &Nxf_long_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nxf_parameter_list.
	fn visit_nxf_parameter_list(&mut self, ctx: &Nxf_parameter_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nxf_parameter.
	fn visit_nxf_parameter(&mut self, ctx: &Nxf_parameterContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nxf_key_pair.
	fn visit_nxf_key_pair(&mut self, ctx: &Nxf_key_pairContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#ntf_connective_name.
	fn visit_ntf_connective_name(&mut self, ctx: &Ntf_connective_nameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#ntf_index.
	fn visit_ntf_index(&mut self, ctx: &Ntf_indexContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#ntf_short_connective.
	fn visit_ntf_short_connective(&mut self, ctx: &Ntf_short_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tcf_formula.
	fn visit_tcf_formula(&mut self, ctx: &Tcf_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tcf_logic_formula.
	fn visit_tcf_logic_formula(&mut self, ctx: &Tcf_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tcf_quantified_formula.
	fn visit_tcf_quantified_formula(&mut self, ctx: &Tcf_quantified_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_formula.
	fn visit_fof_formula(&mut self, ctx: &Fof_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_logic_formula.
	fn visit_fof_logic_formula(&mut self, ctx: &Fof_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_binary_formula.
	fn visit_fof_binary_formula(&mut self, ctx: &Fof_binary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_binary_nonassoc.
	fn visit_fof_binary_nonassoc(&mut self, ctx: &Fof_binary_nonassocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_binary_assoc.
	fn visit_fof_binary_assoc(&mut self, ctx: &Fof_binary_assocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_or_formula.
	fn visit_fof_or_formula(&mut self, ctx: &Fof_or_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_and_formula.
	fn visit_fof_and_formula(&mut self, ctx: &Fof_and_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_unary_formula.
	fn visit_fof_unary_formula(&mut self, ctx: &Fof_unary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_infix_unary.
	fn visit_fof_infix_unary(&mut self, ctx: &Fof_infix_unaryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_unit_formula.
	fn visit_fof_unit_formula(&mut self, ctx: &Fof_unit_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_unitary_formula.
	fn visit_fof_unitary_formula(&mut self, ctx: &Fof_unitary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_quantified_formula.
	fn visit_fof_quantified_formula(&mut self, ctx: &Fof_quantified_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_variable_list.
	fn visit_fof_variable_list(&mut self, ctx: &Fof_variable_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_atomic_formula.
	fn visit_fof_atomic_formula(&mut self, ctx: &Fof_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_plain_atomic_formula.
	fn visit_fof_plain_atomic_formula(&mut self, ctx: &Fof_plain_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_defined_atomic_formula.
	fn visit_fof_defined_atomic_formula(&mut self, ctx: &Fof_defined_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_defined_plain_formula.
	fn visit_fof_defined_plain_formula(&mut self, ctx: &Fof_defined_plain_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_defined_infix_formula.
	fn visit_fof_defined_infix_formula(&mut self, ctx: &Fof_defined_infix_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_system_atomic_formula.
	fn visit_fof_system_atomic_formula(&mut self, ctx: &Fof_system_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_plain_term.
	fn visit_fof_plain_term(&mut self, ctx: &Fof_plain_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_defined_term.
	fn visit_fof_defined_term(&mut self, ctx: &Fof_defined_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_defined_atomic_term.
	fn visit_fof_defined_atomic_term(&mut self, ctx: &Fof_defined_atomic_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_defined_plain_term.
	fn visit_fof_defined_plain_term(&mut self, ctx: &Fof_defined_plain_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_system_term.
	fn visit_fof_system_term(&mut self, ctx: &Fof_system_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_arguments.
	fn visit_fof_arguments(&mut self, ctx: &Fof_argumentsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_term.
	fn visit_fof_term(&mut self, ctx: &Fof_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_function_term.
	fn visit_fof_function_term(&mut self, ctx: &Fof_function_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_sequent.
	fn visit_fof_sequent(&mut self, ctx: &Fof_sequentContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_formula_tuple.
	fn visit_fof_formula_tuple(&mut self, ctx: &Fof_formula_tupleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_formula_tuple_list.
	fn visit_fof_formula_tuple_list(&mut self, ctx: &Fof_formula_tuple_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#comma_fof_logic_formula.
	fn visit_comma_fof_logic_formula(&mut self, ctx: &Comma_fof_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#cnf_formula.
	fn visit_cnf_formula(&mut self, ctx: &Cnf_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#cnf_disjunction.
	fn visit_cnf_disjunction(&mut self, ctx: &Cnf_disjunctionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#cnf_literal.
	fn visit_cnf_literal(&mut self, ctx: &Cnf_literalContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_quantifier.
	fn visit_thf_quantifier(&mut self, ctx: &Thf_quantifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#thf_unary_connective.
	fn visit_thf_unary_connective(&mut self, ctx: &Thf_unary_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#th0_quantifier.
	fn visit_th0_quantifier(&mut self, ctx: &Th0_quantifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#type_quantifier.
	fn visit_type_quantifier(&mut self, ctx: &Type_quantifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#subtype_sign.
	fn visit_subtype_sign(&mut self, ctx: &Subtype_signContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_unary_connective.
	fn visit_tff_unary_connective(&mut self, ctx: &Tff_unary_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#tff_quantifier.
	fn visit_tff_quantifier(&mut self, ctx: &Tff_quantifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#fof_quantifier.
	fn visit_fof_quantifier(&mut self, ctx: &Fof_quantifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nonassoc_connective.
	fn visit_nonassoc_connective(&mut self, ctx: &Nonassoc_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#assoc_connective.
	fn visit_assoc_connective(&mut self, ctx: &Assoc_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#unary_connective.
	fn visit_unary_connective(&mut self, ctx: &Unary_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#gentzen_arrow.
	fn visit_gentzen_arrow(&mut self, ctx: &Gentzen_arrowContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#assignment.
	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#identical.
	fn visit_identical(&mut self, ctx: &IdenticalContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#type_constant.
	fn visit_type_constant(&mut self, ctx: &Type_constantContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#type_functor.
	fn visit_type_functor(&mut self, ctx: &Type_functorContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#defined_type.
	fn visit_defined_type(&mut self, ctx: &Defined_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#atom.
	fn visit_atom(&mut self, ctx: &AtomContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#untyped_atom.
	fn visit_untyped_atom(&mut self, ctx: &Untyped_atomContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#defined_infix_pred.
	fn visit_defined_infix_pred(&mut self, ctx: &Defined_infix_predContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#infix_equality.
	fn visit_infix_equality(&mut self, ctx: &Infix_equalityContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#infix_inequality.
	fn visit_infix_inequality(&mut self, ctx: &Infix_inequalityContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#constant.
	fn visit_constant(&mut self, ctx: &ConstantContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#functor.
	fn visit_functor(&mut self, ctx: &FunctorContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#defined_constant.
	fn visit_defined_constant(&mut self, ctx: &Defined_constantContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#defined_functor.
	fn visit_defined_functor(&mut self, ctx: &Defined_functorContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#system_constant.
	fn visit_system_constant(&mut self, ctx: &System_constantContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#system_functor.
	fn visit_system_functor(&mut self, ctx: &System_functorContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#def_or_sys_constant.
	fn visit_def_or_sys_constant(&mut self, ctx: &Def_or_sys_constantContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#th1_defined_term.
	fn visit_th1_defined_term(&mut self, ctx: &Th1_defined_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#defined_term.
	fn visit_defined_term(&mut self, ctx: &Defined_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#variable.
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#source.
	fn visit_source(&mut self, ctx: &SourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#sources.
	fn visit_sources(&mut self, ctx: &SourcesContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#dag_source.
	fn visit_dag_source(&mut self, ctx: &Dag_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#inference_record.
	fn visit_inference_record(&mut self, ctx: &Inference_recordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#inference_rule.
	fn visit_inference_rule(&mut self, ctx: &Inference_ruleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#internal_source.
	fn visit_internal_source(&mut self, ctx: &Internal_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#intro_type.
	fn visit_intro_type(&mut self, ctx: &Intro_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#external_source.
	fn visit_external_source(&mut self, ctx: &External_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#file_source.
	fn visit_file_source(&mut self, ctx: &File_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#file_info.
	fn visit_file_info(&mut self, ctx: &File_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#theory.
	fn visit_theory(&mut self, ctx: &TheoryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#theory_name.
	fn visit_theory_name(&mut self, ctx: &Theory_nameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#creator_source.
	fn visit_creator_source(&mut self, ctx: &Creator_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#creator_name.
	fn visit_creator_name(&mut self, ctx: &Creator_nameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#parents.
	fn visit_parents(&mut self, ctx: &ParentsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#parent_list.
	fn visit_parent_list(&mut self, ctx: &Parent_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#comma_parent_info.
	fn visit_comma_parent_info(&mut self, ctx: &Comma_parent_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#parent_info.
	fn visit_parent_info(&mut self, ctx: &Parent_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#parent_details.
	fn visit_parent_details(&mut self, ctx: &Parent_detailsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#optional_info.
	fn visit_optional_info(&mut self, ctx: &Optional_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#useful_info.
	fn visit_useful_info(&mut self, ctx: &Useful_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#include.
	fn visit_include(&mut self, ctx: &IncludeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#include_optionals.
	fn visit_include_optionals(&mut self, ctx: &Include_optionalsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#formula_selection.
	fn visit_formula_selection(&mut self, ctx: &Formula_selectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#name_list.
	fn visit_name_list(&mut self, ctx: &Name_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#space_name.
	fn visit_space_name(&mut self, ctx: &Space_nameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#general_term.
	fn visit_general_term(&mut self, ctx: &General_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#general_data.
	fn visit_general_data(&mut self, ctx: &General_dataContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#general_function.
	fn visit_general_function(&mut self, ctx: &General_functionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#formula_data.
	fn visit_formula_data(&mut self, ctx: &Formula_dataContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#general_list.
	fn visit_general_list(&mut self, ctx: &General_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#general_terms.
	fn visit_general_terms(&mut self, ctx: &General_termsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#comma_general_term.
	fn visit_comma_general_term(&mut self, ctx: &Comma_general_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#name.
	fn visit_name(&mut self, ctx: &NameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#atomic_word.
	fn visit_atomic_word(&mut self, ctx: &Atomic_wordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#atomic_defined_word.
	fn visit_atomic_defined_word(&mut self, ctx: &Atomic_defined_wordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#atomic_system_word.
	fn visit_atomic_system_word(&mut self, ctx: &Atomic_system_wordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#number.
	fn visit_number(&mut self, ctx: &NumberContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#file_name.
	fn visit_file_name(&mut self, ctx: &File_nameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by TPTPParser#nothing.
	fn visit_nothing(&mut self, ctx: &NothingContext<'input>) {
            self.visit_children(ctx)
        }

}