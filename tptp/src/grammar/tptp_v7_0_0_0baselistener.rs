// Generated from tptp_v7_0_0_0.g4 by ANTLR 4.13.2

use super::tptp_v7_0_0_0parser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by tptp_v7_0_0_0Parser.

pub trait tptp_v7_0_0_0BaseListener<'input>:
    ParseTreeListener<'input, tptp_v7_0_0_0ParserContextType> {

    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tptp_file(&mut self, _ctx: &Tptp_fileContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tptp_file(&mut self, _ctx: &Tptp_fileContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tptp_input(&mut self, _ctx: &Tptp_inputContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tptp_input(&mut self, _ctx: &Tptp_inputContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_annotated_formula(&mut self, _ctx: &Annotated_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_annotated_formula(&mut self, _ctx: &Annotated_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tpi_annotated(&mut self, _ctx: &Tpi_annotatedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tpi_annotated(&mut self, _ctx: &Tpi_annotatedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tpi_formula(&mut self, _ctx: &Tpi_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tpi_formula(&mut self, _ctx: &Tpi_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_annotated(&mut self, _ctx: &Thf_annotatedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_annotated(&mut self, _ctx: &Thf_annotatedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tfx_annotated(&mut self, _ctx: &Tfx_annotatedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tfx_annotated(&mut self, _ctx: &Tfx_annotatedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_annotated(&mut self, _ctx: &Tff_annotatedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_annotated(&mut self, _ctx: &Tff_annotatedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tcf_annotated(&mut self, _ctx: &Tcf_annotatedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tcf_annotated(&mut self, _ctx: &Tcf_annotatedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_annotated(&mut self, _ctx: &Fof_annotatedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_annotated(&mut self, _ctx: &Fof_annotatedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_cnf_annotated(&mut self, _ctx: &Cnf_annotatedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cnf_annotated(&mut self, _ctx: &Cnf_annotatedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_annotations(&mut self, _ctx: &AnnotationsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_annotations(&mut self, _ctx: &AnnotationsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_formula_role(&mut self, _ctx: &Formula_roleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_formula_role(&mut self, _ctx: &Formula_roleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_formula(&mut self, _ctx: &Thf_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_formula(&mut self, _ctx: &Thf_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_logic_formula(&mut self, _ctx: &Thf_logic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_logic_formula(&mut self, _ctx: &Thf_logic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_binary_formula(&mut self, _ctx: &Thf_binary_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_binary_formula(&mut self, _ctx: &Thf_binary_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_binary_pair(&mut self, _ctx: &Thf_binary_pairContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_binary_pair(&mut self, _ctx: &Thf_binary_pairContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_binary_tuple(&mut self, _ctx: &Thf_binary_tupleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_binary_tuple(&mut self, _ctx: &Thf_binary_tupleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_or_formula(&mut self, _ctx: &Thf_or_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_or_formula(&mut self, _ctx: &Thf_or_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_and_formula(&mut self, _ctx: &Thf_and_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_and_formula(&mut self, _ctx: &Thf_and_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_apply_formula(&mut self, _ctx: &Thf_apply_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_apply_formula(&mut self, _ctx: &Thf_apply_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_unitary_formula(&mut self, _ctx: &Thf_unitary_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_unitary_formula(&mut self, _ctx: &Thf_unitary_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_quantified_formula(&mut self, _ctx: &Thf_quantified_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_quantified_formula(&mut self, _ctx: &Thf_quantified_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_quantification(&mut self, _ctx: &Thf_quantificationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_quantification(&mut self, _ctx: &Thf_quantificationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_variable_list(&mut self, _ctx: &Thf_variable_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_variable_list(&mut self, _ctx: &Thf_variable_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_variable(&mut self, _ctx: &Thf_variableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_variable(&mut self, _ctx: &Thf_variableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_typed_variable(&mut self, _ctx: &Thf_typed_variableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_typed_variable(&mut self, _ctx: &Thf_typed_variableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_unary_formula(&mut self, _ctx: &Thf_unary_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_unary_formula(&mut self, _ctx: &Thf_unary_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_atom(&mut self, _ctx: &Thf_atomContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_atom(&mut self, _ctx: &Thf_atomContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_function(&mut self, _ctx: &Thf_functionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_function(&mut self, _ctx: &Thf_functionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_conn_term(&mut self, _ctx: &Thf_conn_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_conn_term(&mut self, _ctx: &Thf_conn_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_conditional(&mut self, _ctx: &Thf_conditionalContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_conditional(&mut self, _ctx: &Thf_conditionalContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_let(&mut self, _ctx: &Thf_letContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_let(&mut self, _ctx: &Thf_letContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_arguments(&mut self, _ctx: &Thf_argumentsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_arguments(&mut self, _ctx: &Thf_argumentsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_type_formula(&mut self, _ctx: &Thf_type_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_type_formula(&mut self, _ctx: &Thf_type_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_typeable_formula(&mut self, _ctx: &Thf_typeable_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_typeable_formula(&mut self, _ctx: &Thf_typeable_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_subtype(&mut self, _ctx: &Thf_subtypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_subtype(&mut self, _ctx: &Thf_subtypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_top_level_type(&mut self, _ctx: &Thf_top_level_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_top_level_type(&mut self, _ctx: &Thf_top_level_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_unitary_type(&mut self, _ctx: &Thf_unitary_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_unitary_type(&mut self, _ctx: &Thf_unitary_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_apply_type(&mut self, _ctx: &Thf_apply_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_apply_type(&mut self, _ctx: &Thf_apply_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_binary_type(&mut self, _ctx: &Thf_binary_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_binary_type(&mut self, _ctx: &Thf_binary_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_mapping_type(&mut self, _ctx: &Thf_mapping_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_mapping_type(&mut self, _ctx: &Thf_mapping_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_xprod_type(&mut self, _ctx: &Thf_xprod_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_xprod_type(&mut self, _ctx: &Thf_xprod_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_union_type(&mut self, _ctx: &Thf_union_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_union_type(&mut self, _ctx: &Thf_union_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_sequent(&mut self, _ctx: &Thf_sequentContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_sequent(&mut self, _ctx: &Thf_sequentContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_tuple(&mut self, _ctx: &Thf_tupleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_tuple(&mut self, _ctx: &Thf_tupleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_formula_list(&mut self, _ctx: &Thf_formula_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_formula_list(&mut self, _ctx: &Thf_formula_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tfx_formula(&mut self, _ctx: &Tfx_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tfx_formula(&mut self, _ctx: &Tfx_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tfx_logic_formula(&mut self, _ctx: &Tfx_logic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tfx_logic_formula(&mut self, _ctx: &Tfx_logic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_formula(&mut self, _ctx: &Tff_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_formula(&mut self, _ctx: &Tff_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_logic_formula(&mut self, _ctx: &Tff_logic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_logic_formula(&mut self, _ctx: &Tff_logic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_binary_formula(&mut self, _ctx: &Tff_binary_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_binary_formula(&mut self, _ctx: &Tff_binary_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_binary_nonassoc(&mut self, _ctx: &Tff_binary_nonassocContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_binary_nonassoc(&mut self, _ctx: &Tff_binary_nonassocContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_binary_assoc(&mut self, _ctx: &Tff_binary_assocContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_binary_assoc(&mut self, _ctx: &Tff_binary_assocContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_or_formula(&mut self, _ctx: &Tff_or_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_or_formula(&mut self, _ctx: &Tff_or_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_and_formula(&mut self, _ctx: &Tff_and_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_and_formula(&mut self, _ctx: &Tff_and_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_unitary_formula(&mut self, _ctx: &Tff_unitary_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_unitary_formula(&mut self, _ctx: &Tff_unitary_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_quantified_formula(&mut self, _ctx: &Tff_quantified_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_quantified_formula(&mut self, _ctx: &Tff_quantified_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_variable_list(&mut self, _ctx: &Tff_variable_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_variable_list(&mut self, _ctx: &Tff_variable_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_variable(&mut self, _ctx: &Tff_variableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_variable(&mut self, _ctx: &Tff_variableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_typed_variable(&mut self, _ctx: &Tff_typed_variableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_typed_variable(&mut self, _ctx: &Tff_typed_variableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_unary_formula(&mut self, _ctx: &Tff_unary_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_unary_formula(&mut self, _ctx: &Tff_unary_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_atomic_formula(&mut self, _ctx: &Tff_atomic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_atomic_formula(&mut self, _ctx: &Tff_atomic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_conditional(&mut self, _ctx: &Tff_conditionalContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_conditional(&mut self, _ctx: &Tff_conditionalContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let(&mut self, _ctx: &Tff_letContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let(&mut self, _ctx: &Tff_letContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let_term_defns(&mut self, _ctx: &Tff_let_term_defnsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let_term_defns(&mut self, _ctx: &Tff_let_term_defnsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let_term_list(&mut self, _ctx: &Tff_let_term_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let_term_list(&mut self, _ctx: &Tff_let_term_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let_term_defn(&mut self, _ctx: &Tff_let_term_defnContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let_term_defn(&mut self, _ctx: &Tff_let_term_defnContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let_term_binding(&mut self, _ctx: &Tff_let_term_bindingContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let_term_binding(&mut self, _ctx: &Tff_let_term_bindingContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let_formula_defns(&mut self, _ctx: &Tff_let_formula_defnsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let_formula_defns(&mut self, _ctx: &Tff_let_formula_defnsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let_formula_list(&mut self, _ctx: &Tff_let_formula_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let_formula_list(&mut self, _ctx: &Tff_let_formula_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let_formula_defn(&mut self, _ctx: &Tff_let_formula_defnContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let_formula_defn(&mut self, _ctx: &Tff_let_formula_defnContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let_formula_binding(&mut self, _ctx: &Tff_let_formula_bindingContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let_formula_binding(&mut self, _ctx: &Tff_let_formula_bindingContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_sequent(&mut self, _ctx: &Tff_sequentContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_sequent(&mut self, _ctx: &Tff_sequentContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_formula_tuple(&mut self, _ctx: &Tff_formula_tupleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_formula_tuple(&mut self, _ctx: &Tff_formula_tupleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_formula_tuple_list(&mut self, _ctx: &Tff_formula_tuple_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_formula_tuple_list(&mut self, _ctx: &Tff_formula_tuple_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_typed_atom(&mut self, _ctx: &Tff_typed_atomContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_typed_atom(&mut self, _ctx: &Tff_typed_atomContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_subtype(&mut self, _ctx: &Tff_subtypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_subtype(&mut self, _ctx: &Tff_subtypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_top_level_type(&mut self, _ctx: &Tff_top_level_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_top_level_type(&mut self, _ctx: &Tff_top_level_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tf1_quantified_type(&mut self, _ctx: &Tf1_quantified_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tf1_quantified_type(&mut self, _ctx: &Tf1_quantified_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_monotype(&mut self, _ctx: &Tff_monotypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_monotype(&mut self, _ctx: &Tff_monotypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_unitary_type(&mut self, _ctx: &Tff_unitary_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_unitary_type(&mut self, _ctx: &Tff_unitary_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_atomic_type(&mut self, _ctx: &Tff_atomic_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_atomic_type(&mut self, _ctx: &Tff_atomic_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_type_arguments(&mut self, _ctx: &Tff_type_argumentsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_type_arguments(&mut self, _ctx: &Tff_type_argumentsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_mapping_type(&mut self, _ctx: &Tff_mapping_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_mapping_type(&mut self, _ctx: &Tff_mapping_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_xprod_type(&mut self, _ctx: &Tff_xprod_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_xprod_type(&mut self, _ctx: &Tff_xprod_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tcf_formula(&mut self, _ctx: &Tcf_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tcf_formula(&mut self, _ctx: &Tcf_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tcf_logic_formula(&mut self, _ctx: &Tcf_logic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tcf_logic_formula(&mut self, _ctx: &Tcf_logic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tcf_quantified_formula(&mut self, _ctx: &Tcf_quantified_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tcf_quantified_formula(&mut self, _ctx: &Tcf_quantified_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_formula(&mut self, _ctx: &Fof_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_formula(&mut self, _ctx: &Fof_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_logic_formula(&mut self, _ctx: &Fof_logic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_logic_formula(&mut self, _ctx: &Fof_logic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_binary_formula(&mut self, _ctx: &Fof_binary_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_binary_formula(&mut self, _ctx: &Fof_binary_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_binary_nonassoc(&mut self, _ctx: &Fof_binary_nonassocContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_binary_nonassoc(&mut self, _ctx: &Fof_binary_nonassocContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_binary_assoc(&mut self, _ctx: &Fof_binary_assocContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_binary_assoc(&mut self, _ctx: &Fof_binary_assocContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_or_formula(&mut self, _ctx: &Fof_or_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_or_formula(&mut self, _ctx: &Fof_or_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_and_formula(&mut self, _ctx: &Fof_and_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_and_formula(&mut self, _ctx: &Fof_and_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_unitary_formula(&mut self, _ctx: &Fof_unitary_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_unitary_formula(&mut self, _ctx: &Fof_unitary_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_quantified_formula(&mut self, _ctx: &Fof_quantified_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_quantified_formula(&mut self, _ctx: &Fof_quantified_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_variable_list(&mut self, _ctx: &Fof_variable_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_variable_list(&mut self, _ctx: &Fof_variable_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_unary_formula(&mut self, _ctx: &Fof_unary_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_unary_formula(&mut self, _ctx: &Fof_unary_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_infix_unary(&mut self, _ctx: &Fof_infix_unaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_infix_unary(&mut self, _ctx: &Fof_infix_unaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_atomic_formula(&mut self, _ctx: &Fof_atomic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_atomic_formula(&mut self, _ctx: &Fof_atomic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_plain_atomic_formula(&mut self, _ctx: &Fof_plain_atomic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_plain_atomic_formula(&mut self, _ctx: &Fof_plain_atomic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_defined_atomic_formula(&mut self, _ctx: &Fof_defined_atomic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_defined_atomic_formula(&mut self, _ctx: &Fof_defined_atomic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_defined_plain_formula(&mut self, _ctx: &Fof_defined_plain_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_defined_plain_formula(&mut self, _ctx: &Fof_defined_plain_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_defined_infix_formula(&mut self, _ctx: &Fof_defined_infix_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_defined_infix_formula(&mut self, _ctx: &Fof_defined_infix_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_system_atomic_formula(&mut self, _ctx: &Fof_system_atomic_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_system_atomic_formula(&mut self, _ctx: &Fof_system_atomic_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_plain_term(&mut self, _ctx: &Fof_plain_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_plain_term(&mut self, _ctx: &Fof_plain_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_defined_term(&mut self, _ctx: &Fof_defined_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_defined_term(&mut self, _ctx: &Fof_defined_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_defined_atomic_term(&mut self, _ctx: &Fof_defined_atomic_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_defined_atomic_term(&mut self, _ctx: &Fof_defined_atomic_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_defined_plain_term(&mut self, _ctx: &Fof_defined_plain_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_defined_plain_term(&mut self, _ctx: &Fof_defined_plain_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_system_term(&mut self, _ctx: &Fof_system_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_system_term(&mut self, _ctx: &Fof_system_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_arguments(&mut self, _ctx: &Fof_argumentsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_arguments(&mut self, _ctx: &Fof_argumentsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_term(&mut self, _ctx: &Fof_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_term(&mut self, _ctx: &Fof_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_function_term(&mut self, _ctx: &Fof_function_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_function_term(&mut self, _ctx: &Fof_function_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_conditional_term(&mut self, _ctx: &Tff_conditional_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_conditional_term(&mut self, _ctx: &Tff_conditional_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_let_term(&mut self, _ctx: &Tff_let_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_let_term(&mut self, _ctx: &Tff_let_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_tuple_term(&mut self, _ctx: &Tff_tuple_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_tuple_term(&mut self, _ctx: &Tff_tuple_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_sequent(&mut self, _ctx: &Fof_sequentContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_sequent(&mut self, _ctx: &Fof_sequentContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_formula_tuple(&mut self, _ctx: &Fof_formula_tupleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_formula_tuple(&mut self, _ctx: &Fof_formula_tupleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_formula_tuple_list(&mut self, _ctx: &Fof_formula_tuple_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_formula_tuple_list(&mut self, _ctx: &Fof_formula_tuple_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_cnf_formula(&mut self, _ctx: &Cnf_formulaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cnf_formula(&mut self, _ctx: &Cnf_formulaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_cnf_disjunction(&mut self, _ctx: &Cnf_disjunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cnf_disjunction(&mut self, _ctx: &Cnf_disjunctionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_cnf_literal(&mut self, _ctx: &Cnf_literalContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cnf_literal(&mut self, _ctx: &Cnf_literalContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_quantifier(&mut self, _ctx: &Thf_quantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_quantifier(&mut self, _ctx: &Thf_quantifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_th0_quantifier(&mut self, _ctx: &Th0_quantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_th0_quantifier(&mut self, _ctx: &Th0_quantifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_th1_quantifier(&mut self, _ctx: &Th1_quantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_th1_quantifier(&mut self, _ctx: &Th1_quantifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_pair_connective(&mut self, _ctx: &Thf_pair_connectiveContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_pair_connective(&mut self, _ctx: &Thf_pair_connectiveContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_thf_unary_connective(&mut self, _ctx: &Thf_unary_connectiveContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_thf_unary_connective(&mut self, _ctx: &Thf_unary_connectiveContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_th1_unary_connective(&mut self, _ctx: &Th1_unary_connectiveContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_th1_unary_connective(&mut self, _ctx: &Th1_unary_connectiveContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tff_pair_connective(&mut self, _ctx: &Tff_pair_connectiveContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tff_pair_connective(&mut self, _ctx: &Tff_pair_connectiveContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fof_quantifier(&mut self, _ctx: &Fof_quantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fof_quantifier(&mut self, _ctx: &Fof_quantifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_binary_connective(&mut self, _ctx: &Binary_connectiveContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_binary_connective(&mut self, _ctx: &Binary_connectiveContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_assoc_connective(&mut self, _ctx: &Assoc_connectiveContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_assoc_connective(&mut self, _ctx: &Assoc_connectiveContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unary_connective(&mut self, _ctx: &Unary_connectiveContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unary_connective(&mut self, _ctx: &Unary_connectiveContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_type_constant(&mut self, _ctx: &Type_constantContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_type_constant(&mut self, _ctx: &Type_constantContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_type_functor(&mut self, _ctx: &Type_functorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_type_functor(&mut self, _ctx: &Type_functorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_defined_type(&mut self, _ctx: &Defined_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_defined_type(&mut self, _ctx: &Defined_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_system_type(&mut self, _ctx: &System_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_system_type(&mut self, _ctx: &System_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atom(&mut self, _ctx: &AtomContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom(&mut self, _ctx: &AtomContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_untyped_atom(&mut self, _ctx: &Untyped_atomContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_untyped_atom(&mut self, _ctx: &Untyped_atomContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_defined_proposition(&mut self, _ctx: &Defined_propositionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_defined_proposition(&mut self, _ctx: &Defined_propositionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_defined_predicate(&mut self, _ctx: &Defined_predicateContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_defined_predicate(&mut self, _ctx: &Defined_predicateContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_defined_infix_pred(&mut self, _ctx: &Defined_infix_predContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_defined_infix_pred(&mut self, _ctx: &Defined_infix_predContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_constant(&mut self, _ctx: &ConstantContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_constant(&mut self, _ctx: &ConstantContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_functor(&mut self, _ctx: &FunctorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_functor(&mut self, _ctx: &FunctorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_system_constant(&mut self, _ctx: &System_constantContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_system_constant(&mut self, _ctx: &System_constantContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_system_functor(&mut self, _ctx: &System_functorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_system_functor(&mut self, _ctx: &System_functorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_defined_constant(&mut self, _ctx: &Defined_constantContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_defined_constant(&mut self, _ctx: &Defined_constantContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_defined_functor(&mut self, _ctx: &Defined_functorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_defined_functor(&mut self, _ctx: &Defined_functorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_defined_term(&mut self, _ctx: &Defined_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_defined_term(&mut self, _ctx: &Defined_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_variable(&mut self, _ctx: &VariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_source(&mut self, _ctx: &SourceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_source(&mut self, _ctx: &SourceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_sources(&mut self, _ctx: &SourcesContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_sources(&mut self, _ctx: &SourcesContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dag_source(&mut self, _ctx: &Dag_sourceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dag_source(&mut self, _ctx: &Dag_sourceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inference_record(&mut self, _ctx: &Inference_recordContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inference_record(&mut self, _ctx: &Inference_recordContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inference_rule(&mut self, _ctx: &Inference_ruleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inference_rule(&mut self, _ctx: &Inference_ruleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inference_parents(&mut self, _ctx: &Inference_parentsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inference_parents(&mut self, _ctx: &Inference_parentsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parent_list(&mut self, _ctx: &Parent_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parent_list(&mut self, _ctx: &Parent_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parent_info(&mut self, _ctx: &Parent_infoContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parent_info(&mut self, _ctx: &Parent_infoContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parent_details(&mut self, _ctx: &Parent_detailsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parent_details(&mut self, _ctx: &Parent_detailsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_internal_source(&mut self, _ctx: &Internal_sourceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_internal_source(&mut self, _ctx: &Internal_sourceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intro_type(&mut self, _ctx: &Intro_typeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intro_type(&mut self, _ctx: &Intro_typeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_external_source(&mut self, _ctx: &External_sourceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_external_source(&mut self, _ctx: &External_sourceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_file_source(&mut self, _ctx: &File_sourceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_file_source(&mut self, _ctx: &File_sourceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_file_info(&mut self, _ctx: &File_infoContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_file_info(&mut self, _ctx: &File_infoContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_theory(&mut self, _ctx: &TheoryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_theory(&mut self, _ctx: &TheoryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_theory_name(&mut self, _ctx: &Theory_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_theory_name(&mut self, _ctx: &Theory_nameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_creator_source(&mut self, _ctx: &Creator_sourceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_creator_source(&mut self, _ctx: &Creator_sourceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_creator_name(&mut self, _ctx: &Creator_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_creator_name(&mut self, _ctx: &Creator_nameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_optional_info(&mut self, _ctx: &Optional_infoContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_optional_info(&mut self, _ctx: &Optional_infoContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_useful_info(&mut self, _ctx: &Useful_infoContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_useful_info(&mut self, _ctx: &Useful_infoContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_info_items(&mut self, _ctx: &Info_itemsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_info_items(&mut self, _ctx: &Info_itemsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_info_item(&mut self, _ctx: &Info_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_info_item(&mut self, _ctx: &Info_itemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_formula_item(&mut self, _ctx: &Formula_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_formula_item(&mut self, _ctx: &Formula_itemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_description_item(&mut self, _ctx: &Description_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_description_item(&mut self, _ctx: &Description_itemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_iquote_item(&mut self, _ctx: &Iquote_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_iquote_item(&mut self, _ctx: &Iquote_itemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inference_item(&mut self, _ctx: &Inference_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inference_item(&mut self, _ctx: &Inference_itemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inference_status(&mut self, _ctx: &Inference_statusContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inference_status(&mut self, _ctx: &Inference_statusContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_status_value(&mut self, _ctx: &Status_valueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_status_value(&mut self, _ctx: &Status_valueContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inference_info(&mut self, _ctx: &Inference_infoContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inference_info(&mut self, _ctx: &Inference_infoContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_assumptions_record(&mut self, _ctx: &Assumptions_recordContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_assumptions_record(&mut self, _ctx: &Assumptions_recordContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_refutation(&mut self, _ctx: &RefutationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_refutation(&mut self, _ctx: &RefutationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_new_symbol_record(&mut self, _ctx: &New_symbol_recordContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_new_symbol_record(&mut self, _ctx: &New_symbol_recordContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_new_symbol_list(&mut self, _ctx: &New_symbol_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_new_symbol_list(&mut self, _ctx: &New_symbol_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_principal_symbol(&mut self, _ctx: &Principal_symbolContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_principal_symbol(&mut self, _ctx: &Principal_symbolContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_include(&mut self, _ctx: &IncludeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_include(&mut self, _ctx: &IncludeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_formula_selection(&mut self, _ctx: &Formula_selectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_formula_selection(&mut self, _ctx: &Formula_selectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_name_list(&mut self, _ctx: &Name_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_name_list(&mut self, _ctx: &Name_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_general_term(&mut self, _ctx: &General_termContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_general_term(&mut self, _ctx: &General_termContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_general_data(&mut self, _ctx: &General_dataContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_general_data(&mut self, _ctx: &General_dataContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_general_function(&mut self, _ctx: &General_functionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_general_function(&mut self, _ctx: &General_functionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_formula_data(&mut self, _ctx: &Formula_dataContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_formula_data(&mut self, _ctx: &Formula_dataContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_general_list(&mut self, _ctx: &General_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_general_list(&mut self, _ctx: &General_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_general_terms(&mut self, _ctx: &General_termsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_general_terms(&mut self, _ctx: &General_termsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_name(&mut self, _ctx: &NameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_name(&mut self, _ctx: &NameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atomic_word(&mut self, _ctx: &Atomic_wordContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atomic_word(&mut self, _ctx: &Atomic_wordContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atomic_defined_word(&mut self, _ctx: &Atomic_defined_wordContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atomic_defined_word(&mut self, _ctx: &Atomic_defined_wordContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atomic_system_word(&mut self, _ctx: &Atomic_system_wordContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atomic_system_word(&mut self, _ctx: &Atomic_system_wordContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_number(&mut self, _ctx: &NumberContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_number(&mut self, _ctx: &NumberContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_file_name(&mut self, _ctx: &File_nameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  tptp_v7_0_0_0BaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_file_name(&mut self, _ctx: &File_nameContext<'input>) {}


}