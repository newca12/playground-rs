
// Generated from tptp_v7_0_0_0.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::tptp_v7_0_0_0parser::*;

// A complete Visitor for a parse tree produced by tptp_v7_0_0_0Parser.

pub trait tptp_v7_0_0_0BaseVisitor<'input>:
    ParseTreeVisitor<'input, tptp_v7_0_0_0ParserContextType> {
	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tptp_file.
	fn visit_tptp_file(&mut self, ctx: &Tptp_fileContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tptp_input.
	fn visit_tptp_input(&mut self, ctx: &Tptp_inputContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#annotated_formula.
	fn visit_annotated_formula(&mut self, ctx: &Annotated_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tpi_annotated.
	fn visit_tpi_annotated(&mut self, ctx: &Tpi_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tpi_formula.
	fn visit_tpi_formula(&mut self, ctx: &Tpi_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_annotated.
	fn visit_thf_annotated(&mut self, ctx: &Thf_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tfx_annotated.
	fn visit_tfx_annotated(&mut self, ctx: &Tfx_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_annotated.
	fn visit_tff_annotated(&mut self, ctx: &Tff_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tcf_annotated.
	fn visit_tcf_annotated(&mut self, ctx: &Tcf_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_annotated.
	fn visit_fof_annotated(&mut self, ctx: &Fof_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#cnf_annotated.
	fn visit_cnf_annotated(&mut self, ctx: &Cnf_annotatedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#annotations.
	fn visit_annotations(&mut self, ctx: &AnnotationsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#formula_role.
	fn visit_formula_role(&mut self, ctx: &Formula_roleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_formula.
	fn visit_thf_formula(&mut self, ctx: &Thf_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_logic_formula.
	fn visit_thf_logic_formula(&mut self, ctx: &Thf_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_binary_formula.
	fn visit_thf_binary_formula(&mut self, ctx: &Thf_binary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_binary_pair.
	fn visit_thf_binary_pair(&mut self, ctx: &Thf_binary_pairContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_binary_tuple.
	fn visit_thf_binary_tuple(&mut self, ctx: &Thf_binary_tupleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_or_formula.
	fn visit_thf_or_formula(&mut self, ctx: &Thf_or_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_and_formula.
	fn visit_thf_and_formula(&mut self, ctx: &Thf_and_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_apply_formula.
	fn visit_thf_apply_formula(&mut self, ctx: &Thf_apply_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_unitary_formula.
	fn visit_thf_unitary_formula(&mut self, ctx: &Thf_unitary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_quantified_formula.
	fn visit_thf_quantified_formula(&mut self, ctx: &Thf_quantified_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_quantification.
	fn visit_thf_quantification(&mut self, ctx: &Thf_quantificationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_variable_list.
	fn visit_thf_variable_list(&mut self, ctx: &Thf_variable_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_variable.
	fn visit_thf_variable(&mut self, ctx: &Thf_variableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_typed_variable.
	fn visit_thf_typed_variable(&mut self, ctx: &Thf_typed_variableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_unary_formula.
	fn visit_thf_unary_formula(&mut self, ctx: &Thf_unary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_atom.
	fn visit_thf_atom(&mut self, ctx: &Thf_atomContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_function.
	fn visit_thf_function(&mut self, ctx: &Thf_functionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_conn_term.
	fn visit_thf_conn_term(&mut self, ctx: &Thf_conn_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_conditional.
	fn visit_thf_conditional(&mut self, ctx: &Thf_conditionalContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_let.
	fn visit_thf_let(&mut self, ctx: &Thf_letContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_arguments.
	fn visit_thf_arguments(&mut self, ctx: &Thf_argumentsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_type_formula.
	fn visit_thf_type_formula(&mut self, ctx: &Thf_type_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_typeable_formula.
	fn visit_thf_typeable_formula(&mut self, ctx: &Thf_typeable_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_subtype.
	fn visit_thf_subtype(&mut self, ctx: &Thf_subtypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_top_level_type.
	fn visit_thf_top_level_type(&mut self, ctx: &Thf_top_level_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_unitary_type.
	fn visit_thf_unitary_type(&mut self, ctx: &Thf_unitary_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_apply_type.
	fn visit_thf_apply_type(&mut self, ctx: &Thf_apply_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_binary_type.
	fn visit_thf_binary_type(&mut self, ctx: &Thf_binary_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_mapping_type.
	fn visit_thf_mapping_type(&mut self, ctx: &Thf_mapping_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_xprod_type.
	fn visit_thf_xprod_type(&mut self, ctx: &Thf_xprod_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_union_type.
	fn visit_thf_union_type(&mut self, ctx: &Thf_union_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_sequent.
	fn visit_thf_sequent(&mut self, ctx: &Thf_sequentContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_tuple.
	fn visit_thf_tuple(&mut self, ctx: &Thf_tupleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_formula_list.
	fn visit_thf_formula_list(&mut self, ctx: &Thf_formula_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tfx_formula.
	fn visit_tfx_formula(&mut self, ctx: &Tfx_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tfx_logic_formula.
	fn visit_tfx_logic_formula(&mut self, ctx: &Tfx_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_formula.
	fn visit_tff_formula(&mut self, ctx: &Tff_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_logic_formula.
	fn visit_tff_logic_formula(&mut self, ctx: &Tff_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_binary_formula.
	fn visit_tff_binary_formula(&mut self, ctx: &Tff_binary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_binary_nonassoc.
	fn visit_tff_binary_nonassoc(&mut self, ctx: &Tff_binary_nonassocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_binary_assoc.
	fn visit_tff_binary_assoc(&mut self, ctx: &Tff_binary_assocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_or_formula.
	fn visit_tff_or_formula(&mut self, ctx: &Tff_or_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_and_formula.
	fn visit_tff_and_formula(&mut self, ctx: &Tff_and_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_unitary_formula.
	fn visit_tff_unitary_formula(&mut self, ctx: &Tff_unitary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_quantified_formula.
	fn visit_tff_quantified_formula(&mut self, ctx: &Tff_quantified_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_variable_list.
	fn visit_tff_variable_list(&mut self, ctx: &Tff_variable_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_variable.
	fn visit_tff_variable(&mut self, ctx: &Tff_variableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_typed_variable.
	fn visit_tff_typed_variable(&mut self, ctx: &Tff_typed_variableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_unary_formula.
	fn visit_tff_unary_formula(&mut self, ctx: &Tff_unary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_atomic_formula.
	fn visit_tff_atomic_formula(&mut self, ctx: &Tff_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_conditional.
	fn visit_tff_conditional(&mut self, ctx: &Tff_conditionalContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let.
	fn visit_tff_let(&mut self, ctx: &Tff_letContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let_term_defns.
	fn visit_tff_let_term_defns(&mut self, ctx: &Tff_let_term_defnsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let_term_list.
	fn visit_tff_let_term_list(&mut self, ctx: &Tff_let_term_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let_term_defn.
	fn visit_tff_let_term_defn(&mut self, ctx: &Tff_let_term_defnContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let_term_binding.
	fn visit_tff_let_term_binding(&mut self, ctx: &Tff_let_term_bindingContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let_formula_defns.
	fn visit_tff_let_formula_defns(&mut self, ctx: &Tff_let_formula_defnsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let_formula_list.
	fn visit_tff_let_formula_list(&mut self, ctx: &Tff_let_formula_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let_formula_defn.
	fn visit_tff_let_formula_defn(&mut self, ctx: &Tff_let_formula_defnContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let_formula_binding.
	fn visit_tff_let_formula_binding(&mut self, ctx: &Tff_let_formula_bindingContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_sequent.
	fn visit_tff_sequent(&mut self, ctx: &Tff_sequentContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_formula_tuple.
	fn visit_tff_formula_tuple(&mut self, ctx: &Tff_formula_tupleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_formula_tuple_list.
	fn visit_tff_formula_tuple_list(&mut self, ctx: &Tff_formula_tuple_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_typed_atom.
	fn visit_tff_typed_atom(&mut self, ctx: &Tff_typed_atomContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_subtype.
	fn visit_tff_subtype(&mut self, ctx: &Tff_subtypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_top_level_type.
	fn visit_tff_top_level_type(&mut self, ctx: &Tff_top_level_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tf1_quantified_type.
	fn visit_tf1_quantified_type(&mut self, ctx: &Tf1_quantified_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_monotype.
	fn visit_tff_monotype(&mut self, ctx: &Tff_monotypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_unitary_type.
	fn visit_tff_unitary_type(&mut self, ctx: &Tff_unitary_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_atomic_type.
	fn visit_tff_atomic_type(&mut self, ctx: &Tff_atomic_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_type_arguments.
	fn visit_tff_type_arguments(&mut self, ctx: &Tff_type_argumentsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_mapping_type.
	fn visit_tff_mapping_type(&mut self, ctx: &Tff_mapping_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_xprod_type.
	fn visit_tff_xprod_type(&mut self, ctx: &Tff_xprod_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tcf_formula.
	fn visit_tcf_formula(&mut self, ctx: &Tcf_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tcf_logic_formula.
	fn visit_tcf_logic_formula(&mut self, ctx: &Tcf_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tcf_quantified_formula.
	fn visit_tcf_quantified_formula(&mut self, ctx: &Tcf_quantified_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_formula.
	fn visit_fof_formula(&mut self, ctx: &Fof_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_logic_formula.
	fn visit_fof_logic_formula(&mut self, ctx: &Fof_logic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_binary_formula.
	fn visit_fof_binary_formula(&mut self, ctx: &Fof_binary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_binary_nonassoc.
	fn visit_fof_binary_nonassoc(&mut self, ctx: &Fof_binary_nonassocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_binary_assoc.
	fn visit_fof_binary_assoc(&mut self, ctx: &Fof_binary_assocContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_or_formula.
	fn visit_fof_or_formula(&mut self, ctx: &Fof_or_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_and_formula.
	fn visit_fof_and_formula(&mut self, ctx: &Fof_and_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_unitary_formula.
	fn visit_fof_unitary_formula(&mut self, ctx: &Fof_unitary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_quantified_formula.
	fn visit_fof_quantified_formula(&mut self, ctx: &Fof_quantified_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_variable_list.
	fn visit_fof_variable_list(&mut self, ctx: &Fof_variable_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_unary_formula.
	fn visit_fof_unary_formula(&mut self, ctx: &Fof_unary_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_infix_unary.
	fn visit_fof_infix_unary(&mut self, ctx: &Fof_infix_unaryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_atomic_formula.
	fn visit_fof_atomic_formula(&mut self, ctx: &Fof_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_plain_atomic_formula.
	fn visit_fof_plain_atomic_formula(&mut self, ctx: &Fof_plain_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_defined_atomic_formula.
	fn visit_fof_defined_atomic_formula(&mut self, ctx: &Fof_defined_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_defined_plain_formula.
	fn visit_fof_defined_plain_formula(&mut self, ctx: &Fof_defined_plain_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_defined_infix_formula.
	fn visit_fof_defined_infix_formula(&mut self, ctx: &Fof_defined_infix_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_system_atomic_formula.
	fn visit_fof_system_atomic_formula(&mut self, ctx: &Fof_system_atomic_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_plain_term.
	fn visit_fof_plain_term(&mut self, ctx: &Fof_plain_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_defined_term.
	fn visit_fof_defined_term(&mut self, ctx: &Fof_defined_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_defined_atomic_term.
	fn visit_fof_defined_atomic_term(&mut self, ctx: &Fof_defined_atomic_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_defined_plain_term.
	fn visit_fof_defined_plain_term(&mut self, ctx: &Fof_defined_plain_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_system_term.
	fn visit_fof_system_term(&mut self, ctx: &Fof_system_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_arguments.
	fn visit_fof_arguments(&mut self, ctx: &Fof_argumentsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_term.
	fn visit_fof_term(&mut self, ctx: &Fof_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_function_term.
	fn visit_fof_function_term(&mut self, ctx: &Fof_function_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_conditional_term.
	fn visit_tff_conditional_term(&mut self, ctx: &Tff_conditional_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_let_term.
	fn visit_tff_let_term(&mut self, ctx: &Tff_let_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_tuple_term.
	fn visit_tff_tuple_term(&mut self, ctx: &Tff_tuple_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_sequent.
	fn visit_fof_sequent(&mut self, ctx: &Fof_sequentContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_formula_tuple.
	fn visit_fof_formula_tuple(&mut self, ctx: &Fof_formula_tupleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_formula_tuple_list.
	fn visit_fof_formula_tuple_list(&mut self, ctx: &Fof_formula_tuple_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#cnf_formula.
	fn visit_cnf_formula(&mut self, ctx: &Cnf_formulaContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#cnf_disjunction.
	fn visit_cnf_disjunction(&mut self, ctx: &Cnf_disjunctionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#cnf_literal.
	fn visit_cnf_literal(&mut self, ctx: &Cnf_literalContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_quantifier.
	fn visit_thf_quantifier(&mut self, ctx: &Thf_quantifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#th0_quantifier.
	fn visit_th0_quantifier(&mut self, ctx: &Th0_quantifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#th1_quantifier.
	fn visit_th1_quantifier(&mut self, ctx: &Th1_quantifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_pair_connective.
	fn visit_thf_pair_connective(&mut self, ctx: &Thf_pair_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#thf_unary_connective.
	fn visit_thf_unary_connective(&mut self, ctx: &Thf_unary_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#th1_unary_connective.
	fn visit_th1_unary_connective(&mut self, ctx: &Th1_unary_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#tff_pair_connective.
	fn visit_tff_pair_connective(&mut self, ctx: &Tff_pair_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#fof_quantifier.
	fn visit_fof_quantifier(&mut self, ctx: &Fof_quantifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#binary_connective.
	fn visit_binary_connective(&mut self, ctx: &Binary_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#assoc_connective.
	fn visit_assoc_connective(&mut self, ctx: &Assoc_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#unary_connective.
	fn visit_unary_connective(&mut self, ctx: &Unary_connectiveContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#type_constant.
	fn visit_type_constant(&mut self, ctx: &Type_constantContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#type_functor.
	fn visit_type_functor(&mut self, ctx: &Type_functorContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#defined_type.
	fn visit_defined_type(&mut self, ctx: &Defined_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#system_type.
	fn visit_system_type(&mut self, ctx: &System_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#atom.
	fn visit_atom(&mut self, ctx: &AtomContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#untyped_atom.
	fn visit_untyped_atom(&mut self, ctx: &Untyped_atomContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#defined_proposition.
	fn visit_defined_proposition(&mut self, ctx: &Defined_propositionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#defined_predicate.
	fn visit_defined_predicate(&mut self, ctx: &Defined_predicateContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#defined_infix_pred.
	fn visit_defined_infix_pred(&mut self, ctx: &Defined_infix_predContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#constant.
	fn visit_constant(&mut self, ctx: &ConstantContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#functor.
	fn visit_functor(&mut self, ctx: &FunctorContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#system_constant.
	fn visit_system_constant(&mut self, ctx: &System_constantContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#system_functor.
	fn visit_system_functor(&mut self, ctx: &System_functorContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#defined_constant.
	fn visit_defined_constant(&mut self, ctx: &Defined_constantContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#defined_functor.
	fn visit_defined_functor(&mut self, ctx: &Defined_functorContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#defined_term.
	fn visit_defined_term(&mut self, ctx: &Defined_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#variable.
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#source.
	fn visit_source(&mut self, ctx: &SourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#sources.
	fn visit_sources(&mut self, ctx: &SourcesContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#dag_source.
	fn visit_dag_source(&mut self, ctx: &Dag_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#inference_record.
	fn visit_inference_record(&mut self, ctx: &Inference_recordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#inference_rule.
	fn visit_inference_rule(&mut self, ctx: &Inference_ruleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#inference_parents.
	fn visit_inference_parents(&mut self, ctx: &Inference_parentsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#parent_list.
	fn visit_parent_list(&mut self, ctx: &Parent_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#parent_info.
	fn visit_parent_info(&mut self, ctx: &Parent_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#parent_details.
	fn visit_parent_details(&mut self, ctx: &Parent_detailsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#internal_source.
	fn visit_internal_source(&mut self, ctx: &Internal_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#intro_type.
	fn visit_intro_type(&mut self, ctx: &Intro_typeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#external_source.
	fn visit_external_source(&mut self, ctx: &External_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#file_source.
	fn visit_file_source(&mut self, ctx: &File_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#file_info.
	fn visit_file_info(&mut self, ctx: &File_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#theory.
	fn visit_theory(&mut self, ctx: &TheoryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#theory_name.
	fn visit_theory_name(&mut self, ctx: &Theory_nameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#creator_source.
	fn visit_creator_source(&mut self, ctx: &Creator_sourceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#creator_name.
	fn visit_creator_name(&mut self, ctx: &Creator_nameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#optional_info.
	fn visit_optional_info(&mut self, ctx: &Optional_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#useful_info.
	fn visit_useful_info(&mut self, ctx: &Useful_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#info_items.
	fn visit_info_items(&mut self, ctx: &Info_itemsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#info_item.
	fn visit_info_item(&mut self, ctx: &Info_itemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#formula_item.
	fn visit_formula_item(&mut self, ctx: &Formula_itemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#description_item.
	fn visit_description_item(&mut self, ctx: &Description_itemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#iquote_item.
	fn visit_iquote_item(&mut self, ctx: &Iquote_itemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#inference_item.
	fn visit_inference_item(&mut self, ctx: &Inference_itemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#inference_status.
	fn visit_inference_status(&mut self, ctx: &Inference_statusContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#status_value.
	fn visit_status_value(&mut self, ctx: &Status_valueContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#inference_info.
	fn visit_inference_info(&mut self, ctx: &Inference_infoContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#assumptions_record.
	fn visit_assumptions_record(&mut self, ctx: &Assumptions_recordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#refutation.
	fn visit_refutation(&mut self, ctx: &RefutationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#new_symbol_record.
	fn visit_new_symbol_record(&mut self, ctx: &New_symbol_recordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#new_symbol_list.
	fn visit_new_symbol_list(&mut self, ctx: &New_symbol_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#principal_symbol.
	fn visit_principal_symbol(&mut self, ctx: &Principal_symbolContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#include.
	fn visit_include(&mut self, ctx: &IncludeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#formula_selection.
	fn visit_formula_selection(&mut self, ctx: &Formula_selectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#name_list.
	fn visit_name_list(&mut self, ctx: &Name_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#general_term.
	fn visit_general_term(&mut self, ctx: &General_termContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#general_data.
	fn visit_general_data(&mut self, ctx: &General_dataContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#general_function.
	fn visit_general_function(&mut self, ctx: &General_functionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#formula_data.
	fn visit_formula_data(&mut self, ctx: &Formula_dataContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#general_list.
	fn visit_general_list(&mut self, ctx: &General_listContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#general_terms.
	fn visit_general_terms(&mut self, ctx: &General_termsContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#name.
	fn visit_name(&mut self, ctx: &NameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#atomic_word.
	fn visit_atomic_word(&mut self, ctx: &Atomic_wordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#atomic_defined_word.
	fn visit_atomic_defined_word(&mut self, ctx: &Atomic_defined_wordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#atomic_system_word.
	fn visit_atomic_system_word(&mut self, ctx: &Atomic_system_wordContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#number.
	fn visit_number(&mut self, ctx: &NumberContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by tptp_v7_0_0_0Parser#file_name.
	fn visit_file_name(&mut self, ctx: &File_nameContext<'input>) {
            self.visit_children(ctx)
        }

}