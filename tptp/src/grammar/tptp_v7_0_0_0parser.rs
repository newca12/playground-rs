// Generated from tptp_v7_0_0_0.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::tptp_v7_0_0_0listener::*;
use super::tptp_v7_0_0_0visitor::*;

use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const tptp_v7_0_0_0_T__0:i32=1; 
		pub const tptp_v7_0_0_0_T__1:i32=2; 
		pub const tptp_v7_0_0_0_T__2:i32=3; 
		pub const tptp_v7_0_0_0_T__3:i32=4; 
		pub const tptp_v7_0_0_0_T__4:i32=5; 
		pub const tptp_v7_0_0_0_T__5:i32=6; 
		pub const tptp_v7_0_0_0_T__6:i32=7; 
		pub const tptp_v7_0_0_0_T__7:i32=8; 
		pub const tptp_v7_0_0_0_T__8:i32=9; 
		pub const tptp_v7_0_0_0_T__9:i32=10; 
		pub const tptp_v7_0_0_0_T__10:i32=11; 
		pub const tptp_v7_0_0_0_T__11:i32=12; 
		pub const tptp_v7_0_0_0_T__12:i32=13; 
		pub const tptp_v7_0_0_0_T__13:i32=14; 
		pub const tptp_v7_0_0_0_T__14:i32=15; 
		pub const tptp_v7_0_0_0_T__15:i32=16; 
		pub const tptp_v7_0_0_0_T__16:i32=17; 
		pub const tptp_v7_0_0_0_T__17:i32=18; 
		pub const tptp_v7_0_0_0_T__18:i32=19; 
		pub const tptp_v7_0_0_0_T__19:i32=20; 
		pub const tptp_v7_0_0_0_T__20:i32=21; 
		pub const tptp_v7_0_0_0_T__21:i32=22; 
		pub const tptp_v7_0_0_0_T__22:i32=23; 
		pub const tptp_v7_0_0_0_T__23:i32=24; 
		pub const tptp_v7_0_0_0_T__24:i32=25; 
		pub const tptp_v7_0_0_0_T__25:i32=26; 
		pub const tptp_v7_0_0_0_T__26:i32=27; 
		pub const tptp_v7_0_0_0_T__27:i32=28; 
		pub const tptp_v7_0_0_0_T__28:i32=29; 
		pub const tptp_v7_0_0_0_T__29:i32=30; 
		pub const tptp_v7_0_0_0_T__30:i32=31; 
		pub const tptp_v7_0_0_0_T__31:i32=32; 
		pub const tptp_v7_0_0_0_T__32:i32=33; 
		pub const tptp_v7_0_0_0_T__33:i32=34; 
		pub const tptp_v7_0_0_0_T__34:i32=35; 
		pub const tptp_v7_0_0_0_T__35:i32=36; 
		pub const tptp_v7_0_0_0_T__36:i32=37; 
		pub const tptp_v7_0_0_0_T__37:i32=38; 
		pub const tptp_v7_0_0_0_T__38:i32=39; 
		pub const tptp_v7_0_0_0_T__39:i32=40; 
		pub const tptp_v7_0_0_0_T__40:i32=41; 
		pub const tptp_v7_0_0_0_T__41:i32=42; 
		pub const tptp_v7_0_0_0_T__42:i32=43; 
		pub const tptp_v7_0_0_0_Or:i32=44; 
		pub const tptp_v7_0_0_0_And:i32=45; 
		pub const tptp_v7_0_0_0_Iff:i32=46; 
		pub const tptp_v7_0_0_0_Impl:i32=47; 
		pub const tptp_v7_0_0_0_If:i32=48; 
		pub const tptp_v7_0_0_0_Niff:i32=49; 
		pub const tptp_v7_0_0_0_Nor:i32=50; 
		pub const tptp_v7_0_0_0_Nand:i32=51; 
		pub const tptp_v7_0_0_0_Not:i32=52; 
		pub const tptp_v7_0_0_0_ForallComb:i32=53; 
		pub const tptp_v7_0_0_0_TyForall:i32=54; 
		pub const tptp_v7_0_0_0_Infix_inequality:i32=55; 
		pub const tptp_v7_0_0_0_Infix_equality:i32=56; 
		pub const tptp_v7_0_0_0_Forall:i32=57; 
		pub const tptp_v7_0_0_0_ExistsComb:i32=58; 
		pub const tptp_v7_0_0_0_TyExists:i32=59; 
		pub const tptp_v7_0_0_0_Exists:i32=60; 
		pub const tptp_v7_0_0_0_Lambda:i32=61; 
		pub const tptp_v7_0_0_0_ChoiceComb:i32=62; 
		pub const tptp_v7_0_0_0_Choice:i32=63; 
		pub const tptp_v7_0_0_0_DescriptionComb:i32=64; 
		pub const tptp_v7_0_0_0_Description:i32=65; 
		pub const tptp_v7_0_0_0_EqComb:i32=66; 
		pub const tptp_v7_0_0_0_App:i32=67; 
		pub const tptp_v7_0_0_0_Assignment:i32=68; 
		pub const tptp_v7_0_0_0_Arrow:i32=69; 
		pub const tptp_v7_0_0_0_Star:i32=70; 
		pub const tptp_v7_0_0_0_Plus:i32=71; 
		pub const tptp_v7_0_0_0_Subtype_sign:i32=72; 
		pub const tptp_v7_0_0_0_Gentzen_arrow:i32=73; 
		pub const tptp_v7_0_0_0_Real:i32=74; 
		pub const tptp_v7_0_0_0_Signed_real:i32=75; 
		pub const tptp_v7_0_0_0_Unsigned_real:i32=76; 
		pub const tptp_v7_0_0_0_Rational:i32=77; 
		pub const tptp_v7_0_0_0_Signed_rational:i32=78; 
		pub const tptp_v7_0_0_0_Unsigned_rational:i32=79; 
		pub const tptp_v7_0_0_0_Integer:i32=80; 
		pub const tptp_v7_0_0_0_Signed_integer:i32=81; 
		pub const tptp_v7_0_0_0_Unsigned_integer:i32=82; 
		pub const tptp_v7_0_0_0_Decimal:i32=83; 
		pub const tptp_v7_0_0_0_Positive_decimal:i32=84; 
		pub const tptp_v7_0_0_0_Decimal_exponent:i32=85; 
		pub const tptp_v7_0_0_0_Decimal_fraction:i32=86; 
		pub const tptp_v7_0_0_0_Dot_decimal:i32=87; 
		pub const tptp_v7_0_0_0_Exp_integer:i32=88; 
		pub const tptp_v7_0_0_0_Signed_exp_integer:i32=89; 
		pub const tptp_v7_0_0_0_Unsigned_exp_integer:i32=90; 
		pub const tptp_v7_0_0_0_Dollar_word:i32=91; 
		pub const tptp_v7_0_0_0_Dollar_dollar_word:i32=92; 
		pub const tptp_v7_0_0_0_Upper_word:i32=93; 
		pub const tptp_v7_0_0_0_Lower_word:i32=94; 
		pub const tptp_v7_0_0_0_Single_quoted:i32=95; 
		pub const tptp_v7_0_0_0_Distinct_object:i32=96; 
		pub const tptp_v7_0_0_0_WS:i32=97; 
		pub const tptp_v7_0_0_0_Line_comment:i32=98; 
		pub const tptp_v7_0_0_0_Block_comment:i32=99;
	pub const tptp_v7_0_0_0_EOF:i32=EOF;
	pub const RULE_tptp_file:usize = 0; 
	pub const RULE_tptp_input:usize = 1; 
	pub const RULE_annotated_formula:usize = 2; 
	pub const RULE_tpi_annotated:usize = 3; 
	pub const RULE_tpi_formula:usize = 4; 
	pub const RULE_thf_annotated:usize = 5; 
	pub const RULE_tfx_annotated:usize = 6; 
	pub const RULE_tff_annotated:usize = 7; 
	pub const RULE_tcf_annotated:usize = 8; 
	pub const RULE_fof_annotated:usize = 9; 
	pub const RULE_cnf_annotated:usize = 10; 
	pub const RULE_annotations:usize = 11; 
	pub const RULE_formula_role:usize = 12; 
	pub const RULE_thf_formula:usize = 13; 
	pub const RULE_thf_logic_formula:usize = 14; 
	pub const RULE_thf_binary_formula:usize = 15; 
	pub const RULE_thf_binary_pair:usize = 16; 
	pub const RULE_thf_binary_tuple:usize = 17; 
	pub const RULE_thf_or_formula:usize = 18; 
	pub const RULE_thf_and_formula:usize = 19; 
	pub const RULE_thf_apply_formula:usize = 20; 
	pub const RULE_thf_unitary_formula:usize = 21; 
	pub const RULE_thf_quantified_formula:usize = 22; 
	pub const RULE_thf_quantification:usize = 23; 
	pub const RULE_thf_variable_list:usize = 24; 
	pub const RULE_thf_variable:usize = 25; 
	pub const RULE_thf_typed_variable:usize = 26; 
	pub const RULE_thf_unary_formula:usize = 27; 
	pub const RULE_thf_atom:usize = 28; 
	pub const RULE_thf_function:usize = 29; 
	pub const RULE_thf_conn_term:usize = 30; 
	pub const RULE_thf_conditional:usize = 31; 
	pub const RULE_thf_let:usize = 32; 
	pub const RULE_thf_arguments:usize = 33; 
	pub const RULE_thf_type_formula:usize = 34; 
	pub const RULE_thf_typeable_formula:usize = 35; 
	pub const RULE_thf_subtype:usize = 36; 
	pub const RULE_thf_top_level_type:usize = 37; 
	pub const RULE_thf_unitary_type:usize = 38; 
	pub const RULE_thf_apply_type:usize = 39; 
	pub const RULE_thf_binary_type:usize = 40; 
	pub const RULE_thf_mapping_type:usize = 41; 
	pub const RULE_thf_xprod_type:usize = 42; 
	pub const RULE_thf_union_type:usize = 43; 
	pub const RULE_thf_sequent:usize = 44; 
	pub const RULE_thf_tuple:usize = 45; 
	pub const RULE_thf_formula_list:usize = 46; 
	pub const RULE_tfx_formula:usize = 47; 
	pub const RULE_tfx_logic_formula:usize = 48; 
	pub const RULE_tff_formula:usize = 49; 
	pub const RULE_tff_logic_formula:usize = 50; 
	pub const RULE_tff_binary_formula:usize = 51; 
	pub const RULE_tff_binary_nonassoc:usize = 52; 
	pub const RULE_tff_binary_assoc:usize = 53; 
	pub const RULE_tff_or_formula:usize = 54; 
	pub const RULE_tff_and_formula:usize = 55; 
	pub const RULE_tff_unitary_formula:usize = 56; 
	pub const RULE_tff_quantified_formula:usize = 57; 
	pub const RULE_tff_variable_list:usize = 58; 
	pub const RULE_tff_variable:usize = 59; 
	pub const RULE_tff_typed_variable:usize = 60; 
	pub const RULE_tff_unary_formula:usize = 61; 
	pub const RULE_tff_atomic_formula:usize = 62; 
	pub const RULE_tff_conditional:usize = 63; 
	pub const RULE_tff_let:usize = 64; 
	pub const RULE_tff_let_term_defns:usize = 65; 
	pub const RULE_tff_let_term_list:usize = 66; 
	pub const RULE_tff_let_term_defn:usize = 67; 
	pub const RULE_tff_let_term_binding:usize = 68; 
	pub const RULE_tff_let_formula_defns:usize = 69; 
	pub const RULE_tff_let_formula_list:usize = 70; 
	pub const RULE_tff_let_formula_defn:usize = 71; 
	pub const RULE_tff_let_formula_binding:usize = 72; 
	pub const RULE_tff_sequent:usize = 73; 
	pub const RULE_tff_formula_tuple:usize = 74; 
	pub const RULE_tff_formula_tuple_list:usize = 75; 
	pub const RULE_tff_typed_atom:usize = 76; 
	pub const RULE_tff_subtype:usize = 77; 
	pub const RULE_tff_top_level_type:usize = 78; 
	pub const RULE_tf1_quantified_type:usize = 79; 
	pub const RULE_tff_monotype:usize = 80; 
	pub const RULE_tff_unitary_type:usize = 81; 
	pub const RULE_tff_atomic_type:usize = 82; 
	pub const RULE_tff_type_arguments:usize = 83; 
	pub const RULE_tff_mapping_type:usize = 84; 
	pub const RULE_tff_xprod_type:usize = 85; 
	pub const RULE_tcf_formula:usize = 86; 
	pub const RULE_tcf_logic_formula:usize = 87; 
	pub const RULE_tcf_quantified_formula:usize = 88; 
	pub const RULE_fof_formula:usize = 89; 
	pub const RULE_fof_logic_formula:usize = 90; 
	pub const RULE_fof_binary_formula:usize = 91; 
	pub const RULE_fof_binary_nonassoc:usize = 92; 
	pub const RULE_fof_binary_assoc:usize = 93; 
	pub const RULE_fof_or_formula:usize = 94; 
	pub const RULE_fof_and_formula:usize = 95; 
	pub const RULE_fof_unitary_formula:usize = 96; 
	pub const RULE_fof_quantified_formula:usize = 97; 
	pub const RULE_fof_variable_list:usize = 98; 
	pub const RULE_fof_unary_formula:usize = 99; 
	pub const RULE_fof_infix_unary:usize = 100; 
	pub const RULE_fof_atomic_formula:usize = 101; 
	pub const RULE_fof_plain_atomic_formula:usize = 102; 
	pub const RULE_fof_defined_atomic_formula:usize = 103; 
	pub const RULE_fof_defined_plain_formula:usize = 104; 
	pub const RULE_fof_defined_infix_formula:usize = 105; 
	pub const RULE_fof_system_atomic_formula:usize = 106; 
	pub const RULE_fof_plain_term:usize = 107; 
	pub const RULE_fof_defined_term:usize = 108; 
	pub const RULE_fof_defined_atomic_term:usize = 109; 
	pub const RULE_fof_defined_plain_term:usize = 110; 
	pub const RULE_fof_system_term:usize = 111; 
	pub const RULE_fof_arguments:usize = 112; 
	pub const RULE_fof_term:usize = 113; 
	pub const RULE_fof_function_term:usize = 114; 
	pub const RULE_tff_conditional_term:usize = 115; 
	pub const RULE_tff_let_term:usize = 116; 
	pub const RULE_tff_tuple_term:usize = 117; 
	pub const RULE_fof_sequent:usize = 118; 
	pub const RULE_fof_formula_tuple:usize = 119; 
	pub const RULE_fof_formula_tuple_list:usize = 120; 
	pub const RULE_cnf_formula:usize = 121; 
	pub const RULE_cnf_disjunction:usize = 122; 
	pub const RULE_cnf_literal:usize = 123; 
	pub const RULE_thf_quantifier:usize = 124; 
	pub const RULE_th0_quantifier:usize = 125; 
	pub const RULE_th1_quantifier:usize = 126; 
	pub const RULE_thf_pair_connective:usize = 127; 
	pub const RULE_thf_unary_connective:usize = 128; 
	pub const RULE_th1_unary_connective:usize = 129; 
	pub const RULE_tff_pair_connective:usize = 130; 
	pub const RULE_fof_quantifier:usize = 131; 
	pub const RULE_binary_connective:usize = 132; 
	pub const RULE_assoc_connective:usize = 133; 
	pub const RULE_unary_connective:usize = 134; 
	pub const RULE_type_constant:usize = 135; 
	pub const RULE_type_functor:usize = 136; 
	pub const RULE_defined_type:usize = 137; 
	pub const RULE_system_type:usize = 138; 
	pub const RULE_atom:usize = 139; 
	pub const RULE_untyped_atom:usize = 140; 
	pub const RULE_defined_proposition:usize = 141; 
	pub const RULE_defined_predicate:usize = 142; 
	pub const RULE_defined_infix_pred:usize = 143; 
	pub const RULE_constant:usize = 144; 
	pub const RULE_functor:usize = 145; 
	pub const RULE_system_constant:usize = 146; 
	pub const RULE_system_functor:usize = 147; 
	pub const RULE_defined_constant:usize = 148; 
	pub const RULE_defined_functor:usize = 149; 
	pub const RULE_defined_term:usize = 150; 
	pub const RULE_variable:usize = 151; 
	pub const RULE_source:usize = 152; 
	pub const RULE_sources:usize = 153; 
	pub const RULE_dag_source:usize = 154; 
	pub const RULE_inference_record:usize = 155; 
	pub const RULE_inference_rule:usize = 156; 
	pub const RULE_inference_parents:usize = 157; 
	pub const RULE_parent_list:usize = 158; 
	pub const RULE_parent_info:usize = 159; 
	pub const RULE_parent_details:usize = 160; 
	pub const RULE_internal_source:usize = 161; 
	pub const RULE_intro_type:usize = 162; 
	pub const RULE_external_source:usize = 163; 
	pub const RULE_file_source:usize = 164; 
	pub const RULE_file_info:usize = 165; 
	pub const RULE_theory:usize = 166; 
	pub const RULE_theory_name:usize = 167; 
	pub const RULE_creator_source:usize = 168; 
	pub const RULE_creator_name:usize = 169; 
	pub const RULE_optional_info:usize = 170; 
	pub const RULE_useful_info:usize = 171; 
	pub const RULE_info_items:usize = 172; 
	pub const RULE_info_item:usize = 173; 
	pub const RULE_formula_item:usize = 174; 
	pub const RULE_description_item:usize = 175; 
	pub const RULE_iquote_item:usize = 176; 
	pub const RULE_inference_item:usize = 177; 
	pub const RULE_inference_status:usize = 178; 
	pub const RULE_status_value:usize = 179; 
	pub const RULE_inference_info:usize = 180; 
	pub const RULE_assumptions_record:usize = 181; 
	pub const RULE_refutation:usize = 182; 
	pub const RULE_new_symbol_record:usize = 183; 
	pub const RULE_new_symbol_list:usize = 184; 
	pub const RULE_principal_symbol:usize = 185; 
	pub const RULE_include:usize = 186; 
	pub const RULE_formula_selection:usize = 187; 
	pub const RULE_name_list:usize = 188; 
	pub const RULE_general_term:usize = 189; 
	pub const RULE_general_data:usize = 190; 
	pub const RULE_general_function:usize = 191; 
	pub const RULE_formula_data:usize = 192; 
	pub const RULE_general_list:usize = 193; 
	pub const RULE_general_terms:usize = 194; 
	pub const RULE_name:usize = 195; 
	pub const RULE_atomic_word:usize = 196; 
	pub const RULE_atomic_defined_word:usize = 197; 
	pub const RULE_atomic_system_word:usize = 198; 
	pub const RULE_number:usize = 199; 
	pub const RULE_file_name:usize = 200;
	pub const ruleNames: [&'static str; 201] =  [
		"tptp_file", "tptp_input", "annotated_formula", "tpi_annotated", "tpi_formula", 
		"thf_annotated", "tfx_annotated", "tff_annotated", "tcf_annotated", "fof_annotated", 
		"cnf_annotated", "annotations", "formula_role", "thf_formula", "thf_logic_formula", 
		"thf_binary_formula", "thf_binary_pair", "thf_binary_tuple", "thf_or_formula", 
		"thf_and_formula", "thf_apply_formula", "thf_unitary_formula", "thf_quantified_formula", 
		"thf_quantification", "thf_variable_list", "thf_variable", "thf_typed_variable", 
		"thf_unary_formula", "thf_atom", "thf_function", "thf_conn_term", "thf_conditional", 
		"thf_let", "thf_arguments", "thf_type_formula", "thf_typeable_formula", 
		"thf_subtype", "thf_top_level_type", "thf_unitary_type", "thf_apply_type", 
		"thf_binary_type", "thf_mapping_type", "thf_xprod_type", "thf_union_type", 
		"thf_sequent", "thf_tuple", "thf_formula_list", "tfx_formula", "tfx_logic_formula", 
		"tff_formula", "tff_logic_formula", "tff_binary_formula", "tff_binary_nonassoc", 
		"tff_binary_assoc", "tff_or_formula", "tff_and_formula", "tff_unitary_formula", 
		"tff_quantified_formula", "tff_variable_list", "tff_variable", "tff_typed_variable", 
		"tff_unary_formula", "tff_atomic_formula", "tff_conditional", "tff_let", 
		"tff_let_term_defns", "tff_let_term_list", "tff_let_term_defn", "tff_let_term_binding", 
		"tff_let_formula_defns", "tff_let_formula_list", "tff_let_formula_defn", 
		"tff_let_formula_binding", "tff_sequent", "tff_formula_tuple", "tff_formula_tuple_list", 
		"tff_typed_atom", "tff_subtype", "tff_top_level_type", "tf1_quantified_type", 
		"tff_monotype", "tff_unitary_type", "tff_atomic_type", "tff_type_arguments", 
		"tff_mapping_type", "tff_xprod_type", "tcf_formula", "tcf_logic_formula", 
		"tcf_quantified_formula", "fof_formula", "fof_logic_formula", "fof_binary_formula", 
		"fof_binary_nonassoc", "fof_binary_assoc", "fof_or_formula", "fof_and_formula", 
		"fof_unitary_formula", "fof_quantified_formula", "fof_variable_list", 
		"fof_unary_formula", "fof_infix_unary", "fof_atomic_formula", "fof_plain_atomic_formula", 
		"fof_defined_atomic_formula", "fof_defined_plain_formula", "fof_defined_infix_formula", 
		"fof_system_atomic_formula", "fof_plain_term", "fof_defined_term", "fof_defined_atomic_term", 
		"fof_defined_plain_term", "fof_system_term", "fof_arguments", "fof_term", 
		"fof_function_term", "tff_conditional_term", "tff_let_term", "tff_tuple_term", 
		"fof_sequent", "fof_formula_tuple", "fof_formula_tuple_list", "cnf_formula", 
		"cnf_disjunction", "cnf_literal", "thf_quantifier", "th0_quantifier", 
		"th1_quantifier", "thf_pair_connective", "thf_unary_connective", "th1_unary_connective", 
		"tff_pair_connective", "fof_quantifier", "binary_connective", "assoc_connective", 
		"unary_connective", "type_constant", "type_functor", "defined_type", "system_type", 
		"atom", "untyped_atom", "defined_proposition", "defined_predicate", "defined_infix_pred", 
		"constant", "functor", "system_constant", "system_functor", "defined_constant", 
		"defined_functor", "defined_term", "variable", "source", "sources", "dag_source", 
		"inference_record", "inference_rule", "inference_parents", "parent_list", 
		"parent_info", "parent_details", "internal_source", "intro_type", "external_source", 
		"file_source", "file_info", "theory", "theory_name", "creator_source", 
		"creator_name", "optional_info", "useful_info", "info_items", "info_item", 
		"formula_item", "description_item", "iquote_item", "inference_item", "inference_status", 
		"status_value", "inference_info", "assumptions_record", "refutation", 
		"new_symbol_record", "new_symbol_list", "principal_symbol", "include", 
		"formula_selection", "name_list", "general_term", "general_data", "general_function", 
		"formula_data", "general_list", "general_terms", "name", "atomic_word", 
		"atomic_defined_word", "atomic_system_word", "number", "file_name"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;74] = [
		None, Some("'tpi('"), Some("','"), Some("').'"), Some("'thf('"), Some("'tfx('"), 
		Some("'tff('"), Some("'tcf('"), Some("'fof('"), Some("'cnf('"), Some("'('"), 
		Some("')'"), Some("'['"), Some("']'"), Some("':'"), Some("'$ite('"), Some("'$let('"), 
		Some("'[]'"), Some("'{}'"), Some("'{'"), Some("'}'"), Some("'$ite_f('"), 
		Some("'$let_tf('"), Some("'$let_ff('"), Some("'$ite_t('"), Some("'$let_ft('"), 
		Some("'$let_tt('"), Some("'inference('"), Some("'introduced('"), Some("'file('"), 
		Some("'theory('"), Some("'creator('"), Some("'description('"), Some("'iquote('"), 
		Some("'status('"), Some("'assumptions('"), Some("'refutation('"), Some("'new_symbols('"), 
		Some("'include('"), Some("'$thf('"), Some("'$tff('"), Some("'$fof('"), 
		Some("'$cnf('"), Some("'$fot('"), Some("'|'"), Some("'&'"), Some("'<=>'"), 
		Some("'=>'"), Some("'<='"), Some("'<~>'"), Some("'~|'"), Some("'~&'"), 
		Some("'~'"), Some("'!!'"), Some("'!>'"), Some("'!='"), Some("'='"), Some("'!'"), 
		Some("'??'"), Some("'?*'"), Some("'?'"), Some("'^'"), Some("'@@+'"), Some("'@+'"), 
		Some("'@@-'"), Some("'@-'"), Some("'@='"), Some("'@'"), Some("':='"), 
		Some("'>'"), Some("'*'"), Some("'+'"), Some("'<<'"), Some("'-->'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;100]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, Some("Or"), Some("And"), 
		Some("Iff"), Some("Impl"), Some("If"), Some("Niff"), Some("Nor"), Some("Nand"), 
		Some("Not"), Some("ForallComb"), Some("TyForall"), Some("Infix_inequality"), 
		Some("Infix_equality"), Some("Forall"), Some("ExistsComb"), Some("TyExists"), 
		Some("Exists"), Some("Lambda"), Some("ChoiceComb"), Some("Choice"), Some("DescriptionComb"), 
		Some("Description"), Some("EqComb"), Some("App"), Some("Assignment"), 
		Some("Arrow"), Some("Star"), Some("Plus"), Some("Subtype_sign"), Some("Gentzen_arrow"), 
		Some("Real"), Some("Signed_real"), Some("Unsigned_real"), Some("Rational"), 
		Some("Signed_rational"), Some("Unsigned_rational"), Some("Integer"), Some("Signed_integer"), 
		Some("Unsigned_integer"), Some("Decimal"), Some("Positive_decimal"), Some("Decimal_exponent"), 
		Some("Decimal_fraction"), Some("Dot_decimal"), Some("Exp_integer"), Some("Signed_exp_integer"), 
		Some("Unsigned_exp_integer"), Some("Dollar_word"), Some("Dollar_dollar_word"), 
		Some("Upper_word"), Some("Lower_word"), Some("Single_quoted"), Some("Distinct_object"), 
		Some("WS"), Some("Line_comment"), Some("Block_comment")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,tptp_v7_0_0_0ParserExt<'input>, I, tptp_v7_0_0_0ParserContextType , dyn tptp_v7_0_0_0Listener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type tptp_v7_0_0_0TreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, tptp_v7_0_0_0ParserContextType , dyn tptp_v7_0_0_0Listener<'input> + 'a>;

/// Parser for tptp_v7_0_0_0 grammar
pub struct tptp_v7_0_0_0Parser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr4rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				tptp_v7_0_0_0ParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> tptp_v7_0_0_0Parser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> tptp_v7_0_0_0Parser<'input, I, DefaultErrorStrategy<'input,tptp_v7_0_0_0ParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for tptp_v7_0_0_0Parser
pub trait tptp_v7_0_0_0ParserContext<'input>:
	for<'x> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'x > + 
	for<'x> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=tptp_v7_0_0_0ParserContextType>
{}

antlr4rust::coerce_from!{ 'input : tptp_v7_0_0_0ParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn tptp_v7_0_0_0ParserContext<'input> + 'input
where
    T: tptp_v7_0_0_0Visitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'x))
    }
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for TerminalNode<'input,tptp_v7_0_0_0ParserContextType> {}
impl<'input> tptp_v7_0_0_0ParserContext<'input> for ErrorNode<'input,tptp_v7_0_0_0ParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn tptp_v7_0_0_0ParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn tptp_v7_0_0_0Listener<'input> + 'input }

pub struct tptp_v7_0_0_0ParserContextType;
antlr4rust::tid!{tptp_v7_0_0_0ParserContextType}

impl<'input> ParserNodeType<'input> for tptp_v7_0_0_0ParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn tptp_v7_0_0_0ParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct tptp_v7_0_0_0ParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> tptp_v7_0_0_0ParserExt<'input>{
}
antlr4rust::tid! { tptp_v7_0_0_0ParserExt<'a> }

impl<'input> TokenAware<'input> for tptp_v7_0_0_0ParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for tptp_v7_0_0_0ParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for tptp_v7_0_0_0ParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "tptp_v7_0_0_0.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn tptp_v7_0_0_0ParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					18 => tptp_v7_0_0_0Parser::<'input,I,_>::thf_or_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					19 => tptp_v7_0_0_0Parser::<'input,I,_>::thf_and_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					20 => tptp_v7_0_0_0Parser::<'input,I,_>::thf_apply_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					42 => tptp_v7_0_0_0Parser::<'input,I,_>::thf_xprod_type_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					43 => tptp_v7_0_0_0Parser::<'input,I,_>::thf_union_type_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					54 => tptp_v7_0_0_0Parser::<'input,I,_>::tff_or_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					55 => tptp_v7_0_0_0Parser::<'input,I,_>::tff_and_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					85 => tptp_v7_0_0_0Parser::<'input,I,_>::tff_xprod_type_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					94 => tptp_v7_0_0_0Parser::<'input,I,_>::fof_or_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					95 => tptp_v7_0_0_0Parser::<'input,I,_>::fof_and_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					122 => tptp_v7_0_0_0Parser::<'input,I,_>::cnf_disjunction_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> tptp_v7_0_0_0Parser<'input, I, DefaultErrorStrategy<'input,tptp_v7_0_0_0ParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn thf_or_formula_sempred(_localctx: Option<&Thf_or_formulaContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn thf_and_formula_sempred(_localctx: Option<&Thf_and_formulaContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				1=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn thf_apply_formula_sempred(_localctx: Option<&Thf_apply_formulaContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				2=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn thf_xprod_type_sempred(_localctx: Option<&Thf_xprod_typeContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				3=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn thf_union_type_sempred(_localctx: Option<&Thf_union_typeContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				4=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn tff_or_formula_sempred(_localctx: Option<&Tff_or_formulaContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				5=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn tff_and_formula_sempred(_localctx: Option<&Tff_and_formulaContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				6=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn tff_xprod_type_sempred(_localctx: Option<&Tff_xprod_typeContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				7=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn fof_or_formula_sempred(_localctx: Option<&Fof_or_formulaContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				8=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn fof_and_formula_sempred(_localctx: Option<&Fof_and_formulaContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				9=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn cnf_disjunction_sempred(_localctx: Option<&Cnf_disjunctionContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				10=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- tptp_file ----------------
pub type Tptp_fileContextAll<'input> = Tptp_fileContext<'input>;


pub type Tptp_fileContext<'input> = BaseParserRuleContext<'input,Tptp_fileContextExt<'input>>;

#[derive(Clone)]
pub struct Tptp_fileContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tptp_fileContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tptp_fileContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tptp_file(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tptp_file(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tptp_fileContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tptp_file(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tptp_fileContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tptp_file }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tptp_file }
}
antlr4rust::tid!{Tptp_fileContextExt<'a>}

impl<'input> Tptp_fileContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tptp_fileContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tptp_fileContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tptp_fileContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tptp_fileContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_EOF, 0)
}
fn tptp_input_all(&self) ->  Vec<Rc<Tptp_inputContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tptp_input(&self, i: usize) -> Option<Rc<Tptp_inputContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tptp_fileContextAttrs<'input> for Tptp_fileContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tptp_file(&mut self,)
	-> Result<Rc<Tptp_fileContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tptp_fileContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_tptp_file);
        let mut _localctx: Rc<Tptp_fileContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(405);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 1010) != 0) || _la==tptp_v7_0_0_0_T__37 {
				{
				{
				/*InvokeRule tptp_input*/
				recog.base.set_state(402);
				recog.tptp_input()?;

				}
				}
				recog.base.set_state(407);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(408);
			recog.base.match_token(tptp_v7_0_0_0_EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tptp_input ----------------
pub type Tptp_inputContextAll<'input> = Tptp_inputContext<'input>;


pub type Tptp_inputContext<'input> = BaseParserRuleContext<'input,Tptp_inputContextExt<'input>>;

#[derive(Clone)]
pub struct Tptp_inputContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tptp_inputContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tptp_inputContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tptp_input(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tptp_input(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tptp_inputContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tptp_input(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tptp_inputContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tptp_input }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tptp_input }
}
antlr4rust::tid!{Tptp_inputContextExt<'a>}

impl<'input> Tptp_inputContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tptp_inputContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tptp_inputContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tptp_inputContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tptp_inputContextExt<'input>>{

fn annotated_formula(&self) -> Option<Rc<Annotated_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn include(&self) -> Option<Rc<IncludeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tptp_inputContextAttrs<'input> for Tptp_inputContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tptp_input(&mut self,)
	-> Result<Rc<Tptp_inputContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tptp_inputContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_tptp_input);
        let mut _localctx: Rc<Tptp_inputContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(412);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__0 |tptp_v7_0_0_0_T__3 |tptp_v7_0_0_0_T__4 |tptp_v7_0_0_0_T__5 |
			tptp_v7_0_0_0_T__6 |tptp_v7_0_0_0_T__7 |tptp_v7_0_0_0_T__8 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule annotated_formula*/
					recog.base.set_state(410);
					recog.annotated_formula()?;

					}
				}

			tptp_v7_0_0_0_T__37 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule include*/
					recog.base.set_state(411);
					recog.include()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotated_formula ----------------
pub type Annotated_formulaContextAll<'input> = Annotated_formulaContext<'input>;


pub type Annotated_formulaContext<'input> = BaseParserRuleContext<'input,Annotated_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Annotated_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Annotated_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Annotated_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotated_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_annotated_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Annotated_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_annotated_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Annotated_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotated_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotated_formula }
}
antlr4rust::tid!{Annotated_formulaContextExt<'a>}

impl<'input> Annotated_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Annotated_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Annotated_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Annotated_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Annotated_formulaContextExt<'input>>{

fn thf_annotated(&self) -> Option<Rc<Thf_annotatedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tfx_annotated(&self) -> Option<Rc<Tfx_annotatedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_annotated(&self) -> Option<Rc<Tff_annotatedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tcf_annotated(&self) -> Option<Rc<Tcf_annotatedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_annotated(&self) -> Option<Rc<Fof_annotatedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cnf_annotated(&self) -> Option<Rc<Cnf_annotatedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tpi_annotated(&self) -> Option<Rc<Tpi_annotatedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Annotated_formulaContextAttrs<'input> for Annotated_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotated_formula(&mut self,)
	-> Result<Rc<Annotated_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Annotated_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_annotated_formula);
        let mut _localctx: Rc<Annotated_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(421);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_annotated*/
					recog.base.set_state(414);
					recog.thf_annotated()?;

					}
				}

			tptp_v7_0_0_0_T__4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tfx_annotated*/
					recog.base.set_state(415);
					recog.tfx_annotated()?;

					}
				}

			tptp_v7_0_0_0_T__5 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule tff_annotated*/
					recog.base.set_state(416);
					recog.tff_annotated()?;

					}
				}

			tptp_v7_0_0_0_T__6 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule tcf_annotated*/
					recog.base.set_state(417);
					recog.tcf_annotated()?;

					}
				}

			tptp_v7_0_0_0_T__7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule fof_annotated*/
					recog.base.set_state(418);
					recog.fof_annotated()?;

					}
				}

			tptp_v7_0_0_0_T__8 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule cnf_annotated*/
					recog.base.set_state(419);
					recog.cnf_annotated()?;

					}
				}

			tptp_v7_0_0_0_T__0 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule tpi_annotated*/
					recog.base.set_state(420);
					recog.tpi_annotated()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tpi_annotated ----------------
pub type Tpi_annotatedContextAll<'input> = Tpi_annotatedContext<'input>;


pub type Tpi_annotatedContext<'input> = BaseParserRuleContext<'input,Tpi_annotatedContextExt<'input>>;

#[derive(Clone)]
pub struct Tpi_annotatedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tpi_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tpi_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tpi_annotated(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tpi_annotated(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tpi_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tpi_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tpi_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tpi_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tpi_annotated }
}
antlr4rust::tid!{Tpi_annotatedContextExt<'a>}

impl<'input> Tpi_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tpi_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tpi_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tpi_annotatedContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tpi_annotatedContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formula_role(&self) -> Option<Rc<Formula_roleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tpi_formula(&self) -> Option<Rc<Tpi_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotations(&self) -> Option<Rc<AnnotationsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tpi_annotatedContextAttrs<'input> for Tpi_annotatedContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tpi_annotated(&mut self,)
	-> Result<Rc<Tpi_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tpi_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_tpi_annotated);
        let mut _localctx: Rc<Tpi_annotatedContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(423);
			recog.base.match_token(tptp_v7_0_0_0_T__0,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(424);
			recog.name()?;

			recog.base.set_state(425);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(426);
			recog.formula_role()?;

			recog.base.set_state(427);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule tpi_formula*/
			recog.base.set_state(428);
			recog.tpi_formula()?;

			recog.base.set_state(430);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(429);
				recog.annotations()?;

				}
			}

			recog.base.set_state(432);
			recog.base.match_token(tptp_v7_0_0_0_T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tpi_formula ----------------
pub type Tpi_formulaContextAll<'input> = Tpi_formulaContext<'input>;


pub type Tpi_formulaContext<'input> = BaseParserRuleContext<'input,Tpi_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tpi_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tpi_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tpi_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tpi_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tpi_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tpi_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tpi_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tpi_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tpi_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tpi_formula }
}
antlr4rust::tid!{Tpi_formulaContextExt<'a>}

impl<'input> Tpi_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tpi_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tpi_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tpi_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tpi_formulaContextExt<'input>>{

fn fof_formula(&self) -> Option<Rc<Fof_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tpi_formulaContextAttrs<'input> for Tpi_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tpi_formula(&mut self,)
	-> Result<Rc<Tpi_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tpi_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_tpi_formula);
        let mut _localctx: Rc<Tpi_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_formula*/
			recog.base.set_state(434);
			recog.fof_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_annotated ----------------
pub type Thf_annotatedContextAll<'input> = Thf_annotatedContext<'input>;


pub type Thf_annotatedContext<'input> = BaseParserRuleContext<'input,Thf_annotatedContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_annotatedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_annotated(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_annotated(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_annotated }
}
antlr4rust::tid!{Thf_annotatedContextExt<'a>}

impl<'input> Thf_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_annotatedContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_annotatedContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formula_role(&self) -> Option<Rc<Formula_roleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_formula(&self) -> Option<Rc<Thf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotations(&self) -> Option<Rc<AnnotationsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_annotatedContextAttrs<'input> for Thf_annotatedContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_annotated(&mut self,)
	-> Result<Rc<Thf_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_thf_annotated);
        let mut _localctx: Rc<Thf_annotatedContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(436);
			recog.base.match_token(tptp_v7_0_0_0_T__3,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(437);
			recog.name()?;

			recog.base.set_state(438);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(439);
			recog.formula_role()?;

			recog.base.set_state(440);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_formula*/
			recog.base.set_state(441);
			recog.thf_formula()?;

			recog.base.set_state(443);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(442);
				recog.annotations()?;

				}
			}

			recog.base.set_state(445);
			recog.base.match_token(tptp_v7_0_0_0_T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tfx_annotated ----------------
pub type Tfx_annotatedContextAll<'input> = Tfx_annotatedContext<'input>;


pub type Tfx_annotatedContext<'input> = BaseParserRuleContext<'input,Tfx_annotatedContextExt<'input>>;

#[derive(Clone)]
pub struct Tfx_annotatedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tfx_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tfx_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tfx_annotated(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tfx_annotated(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tfx_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tfx_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tfx_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tfx_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tfx_annotated }
}
antlr4rust::tid!{Tfx_annotatedContextExt<'a>}

impl<'input> Tfx_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tfx_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tfx_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tfx_annotatedContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tfx_annotatedContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formula_role(&self) -> Option<Rc<Formula_roleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tfx_formula(&self) -> Option<Rc<Tfx_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotations(&self) -> Option<Rc<AnnotationsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tfx_annotatedContextAttrs<'input> for Tfx_annotatedContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tfx_annotated(&mut self,)
	-> Result<Rc<Tfx_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tfx_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_tfx_annotated);
        let mut _localctx: Rc<Tfx_annotatedContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(447);
			recog.base.match_token(tptp_v7_0_0_0_T__4,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(448);
			recog.name()?;

			recog.base.set_state(449);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(450);
			recog.formula_role()?;

			recog.base.set_state(451);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule tfx_formula*/
			recog.base.set_state(452);
			recog.tfx_formula()?;

			recog.base.set_state(454);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(453);
				recog.annotations()?;

				}
			}

			recog.base.set_state(456);
			recog.base.match_token(tptp_v7_0_0_0_T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_annotated ----------------
pub type Tff_annotatedContextAll<'input> = Tff_annotatedContext<'input>;


pub type Tff_annotatedContext<'input> = BaseParserRuleContext<'input,Tff_annotatedContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_annotatedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_annotated(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_annotated(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_annotated }
}
antlr4rust::tid!{Tff_annotatedContextExt<'a>}

impl<'input> Tff_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_annotatedContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_annotatedContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formula_role(&self) -> Option<Rc<Formula_roleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_formula(&self) -> Option<Rc<Tff_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotations(&self) -> Option<Rc<AnnotationsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_annotatedContextAttrs<'input> for Tff_annotatedContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_annotated(&mut self,)
	-> Result<Rc<Tff_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_tff_annotated);
        let mut _localctx: Rc<Tff_annotatedContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(458);
			recog.base.match_token(tptp_v7_0_0_0_T__5,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(459);
			recog.name()?;

			recog.base.set_state(460);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(461);
			recog.formula_role()?;

			recog.base.set_state(462);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_formula*/
			recog.base.set_state(463);
			recog.tff_formula()?;

			recog.base.set_state(465);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(464);
				recog.annotations()?;

				}
			}

			recog.base.set_state(467);
			recog.base.match_token(tptp_v7_0_0_0_T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tcf_annotated ----------------
pub type Tcf_annotatedContextAll<'input> = Tcf_annotatedContext<'input>;


pub type Tcf_annotatedContext<'input> = BaseParserRuleContext<'input,Tcf_annotatedContextExt<'input>>;

#[derive(Clone)]
pub struct Tcf_annotatedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tcf_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tcf_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tcf_annotated(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tcf_annotated(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tcf_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tcf_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tcf_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tcf_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tcf_annotated }
}
antlr4rust::tid!{Tcf_annotatedContextExt<'a>}

impl<'input> Tcf_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tcf_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tcf_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tcf_annotatedContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tcf_annotatedContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formula_role(&self) -> Option<Rc<Formula_roleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tcf_formula(&self) -> Option<Rc<Tcf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotations(&self) -> Option<Rc<AnnotationsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tcf_annotatedContextAttrs<'input> for Tcf_annotatedContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tcf_annotated(&mut self,)
	-> Result<Rc<Tcf_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tcf_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_tcf_annotated);
        let mut _localctx: Rc<Tcf_annotatedContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(469);
			recog.base.match_token(tptp_v7_0_0_0_T__6,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(470);
			recog.name()?;

			recog.base.set_state(471);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(472);
			recog.formula_role()?;

			recog.base.set_state(473);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule tcf_formula*/
			recog.base.set_state(474);
			recog.tcf_formula()?;

			recog.base.set_state(476);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(475);
				recog.annotations()?;

				}
			}

			recog.base.set_state(478);
			recog.base.match_token(tptp_v7_0_0_0_T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_annotated ----------------
pub type Fof_annotatedContextAll<'input> = Fof_annotatedContext<'input>;


pub type Fof_annotatedContext<'input> = BaseParserRuleContext<'input,Fof_annotatedContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_annotatedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_annotated(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_annotated(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_annotated }
}
antlr4rust::tid!{Fof_annotatedContextExt<'a>}

impl<'input> Fof_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_annotatedContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_annotatedContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formula_role(&self) -> Option<Rc<Formula_roleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_formula(&self) -> Option<Rc<Fof_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotations(&self) -> Option<Rc<AnnotationsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_annotatedContextAttrs<'input> for Fof_annotatedContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_annotated(&mut self,)
	-> Result<Rc<Fof_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_fof_annotated);
        let mut _localctx: Rc<Fof_annotatedContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(480);
			recog.base.match_token(tptp_v7_0_0_0_T__7,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(481);
			recog.name()?;

			recog.base.set_state(482);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(483);
			recog.formula_role()?;

			recog.base.set_state(484);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_formula*/
			recog.base.set_state(485);
			recog.fof_formula()?;

			recog.base.set_state(487);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(486);
				recog.annotations()?;

				}
			}

			recog.base.set_state(489);
			recog.base.match_token(tptp_v7_0_0_0_T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- cnf_annotated ----------------
pub type Cnf_annotatedContextAll<'input> = Cnf_annotatedContext<'input>;


pub type Cnf_annotatedContext<'input> = BaseParserRuleContext<'input,Cnf_annotatedContextExt<'input>>;

#[derive(Clone)]
pub struct Cnf_annotatedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Cnf_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Cnf_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cnf_annotated(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_cnf_annotated(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Cnf_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_cnf_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Cnf_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cnf_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cnf_annotated }
}
antlr4rust::tid!{Cnf_annotatedContextExt<'a>}

impl<'input> Cnf_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Cnf_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Cnf_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Cnf_annotatedContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Cnf_annotatedContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formula_role(&self) -> Option<Rc<Formula_roleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cnf_formula(&self) -> Option<Rc<Cnf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotations(&self) -> Option<Rc<AnnotationsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Cnf_annotatedContextAttrs<'input> for Cnf_annotatedContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cnf_annotated(&mut self,)
	-> Result<Rc<Cnf_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Cnf_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_cnf_annotated);
        let mut _localctx: Rc<Cnf_annotatedContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(491);
			recog.base.match_token(tptp_v7_0_0_0_T__8,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(492);
			recog.name()?;

			recog.base.set_state(493);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(494);
			recog.formula_role()?;

			recog.base.set_state(495);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule cnf_formula*/
			recog.base.set_state(496);
			recog.cnf_formula()?;

			recog.base.set_state(498);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(497);
				recog.annotations()?;

				}
			}

			recog.base.set_state(500);
			recog.base.match_token(tptp_v7_0_0_0_T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotations ----------------
pub type AnnotationsContextAll<'input> = AnnotationsContext<'input>;


pub type AnnotationsContext<'input> = BaseParserRuleContext<'input,AnnotationsContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for AnnotationsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for AnnotationsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotations(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_annotations(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for AnnotationsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_annotations(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnnotationsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotations }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotations }
}
antlr4rust::tid!{AnnotationsContextExt<'a>}

impl<'input> AnnotationsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AnnotationsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<AnnotationsContextExt<'input>>{

fn source(&self) -> Option<Rc<SourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn optional_info(&self) -> Option<Rc<Optional_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationsContextAttrs<'input> for AnnotationsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotations(&mut self,)
	-> Result<Rc<AnnotationsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_annotations);
        let mut _localctx: Rc<AnnotationsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(502);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule source*/
			recog.base.set_state(503);
			recog.source()?;

			recog.base.set_state(505);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule optional_info*/
				recog.base.set_state(504);
				recog.optional_info()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- formula_role ----------------
pub type Formula_roleContextAll<'input> = Formula_roleContext<'input>;


pub type Formula_roleContext<'input> = BaseParserRuleContext<'input,Formula_roleContextExt<'input>>;

#[derive(Clone)]
pub struct Formula_roleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Formula_roleContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Formula_roleContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_formula_role(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_formula_role(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Formula_roleContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_formula_role(self);
	}
}

impl<'input> CustomRuleContext<'input> for Formula_roleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formula_role }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formula_role }
}
antlr4rust::tid!{Formula_roleContextExt<'a>}

impl<'input> Formula_roleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Formula_roleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Formula_roleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Formula_roleContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Formula_roleContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Lower_word
/// Returns `None` if there is no child corresponding to token Lower_word
fn Lower_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Lower_word, 0)
}

}

impl<'input> Formula_roleContextAttrs<'input> for Formula_roleContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn formula_role(&mut self,)
	-> Result<Rc<Formula_roleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Formula_roleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_formula_role);
        let mut _localctx: Rc<Formula_roleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(507);
			recog.base.match_token(tptp_v7_0_0_0_Lower_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_formula ----------------
pub type Thf_formulaContextAll<'input> = Thf_formulaContext<'input>;


pub type Thf_formulaContext<'input> = BaseParserRuleContext<'input,Thf_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_formula }
}
antlr4rust::tid!{Thf_formulaContextExt<'a>}

impl<'input> Thf_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_formulaContextExt<'input>>{

fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_sequent(&self) -> Option<Rc<Thf_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_formulaContextAttrs<'input> for Thf_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_formula(&mut self,)
	-> Result<Rc<Thf_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_thf_formula);
        let mut _localctx: Rc<Thf_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(511);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(509);
					recog.thf_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule thf_sequent*/
					recog.base.set_state(510);
					recog.thf_sequent()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_logic_formula ----------------
pub type Thf_logic_formulaContextAll<'input> = Thf_logic_formulaContext<'input>;


pub type Thf_logic_formulaContext<'input> = BaseParserRuleContext<'input,Thf_logic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_logic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_logic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_logic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_logic_formula }
}
antlr4rust::tid!{Thf_logic_formulaContextExt<'a>}

impl<'input> Thf_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_logic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_logic_formulaContextExt<'input>>{

fn thf_binary_formula(&self) -> Option<Rc<Thf_binary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_unitary_formula(&self) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_type_formula(&self) -> Option<Rc<Thf_type_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_subtype(&self) -> Option<Rc<Thf_subtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_logic_formulaContextAttrs<'input> for Thf_logic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_logic_formula(&mut self,)
	-> Result<Rc<Thf_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_thf_logic_formula);
        let mut _localctx: Rc<Thf_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(517);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_binary_formula*/
					recog.base.set_state(513);
					recog.thf_binary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(514);
					recog.thf_unitary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule thf_type_formula*/
					recog.base.set_state(515);
					recog.thf_type_formula()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule thf_subtype*/
					recog.base.set_state(516);
					recog.thf_subtype()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_binary_formula ----------------
pub type Thf_binary_formulaContextAll<'input> = Thf_binary_formulaContext<'input>;


pub type Thf_binary_formulaContext<'input> = BaseParserRuleContext<'input,Thf_binary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_binary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_binary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_binary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_binary_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_binary_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_binary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_binary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_formula }
}
antlr4rust::tid!{Thf_binary_formulaContextExt<'a>}

impl<'input> Thf_binary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_binary_formulaContextExt<'input>>{

fn thf_binary_pair(&self) -> Option<Rc<Thf_binary_pairContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_binary_tuple(&self) -> Option<Rc<Thf_binary_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_binary_type(&self) -> Option<Rc<Thf_binary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_binary_formulaContextAttrs<'input> for Thf_binary_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_binary_formula(&mut self,)
	-> Result<Rc<Thf_binary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_thf_binary_formula);
        let mut _localctx: Rc<Thf_binary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(522);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_binary_pair*/
					recog.base.set_state(519);
					recog.thf_binary_pair()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule thf_binary_tuple*/
					recog.base.set_state(520);
					recog.thf_binary_tuple()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule thf_binary_type*/
					recog.base.set_state(521);
					recog.thf_binary_type()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_binary_pair ----------------
pub type Thf_binary_pairContextAll<'input> = Thf_binary_pairContext<'input>;


pub type Thf_binary_pairContext<'input> = BaseParserRuleContext<'input,Thf_binary_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_binary_pairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_binary_pairContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_binary_pairContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_binary_pair(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_binary_pair(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_binary_pairContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_binary_pair(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_pairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_pair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_pair }
}
antlr4rust::tid!{Thf_binary_pairContextExt<'a>}

impl<'input> Thf_binary_pairContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_pairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_pairContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_pairContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_binary_pairContextExt<'input>>{

fn thf_unitary_formula_all(&self) ->  Vec<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_formula(&self, i: usize) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn thf_pair_connective(&self) -> Option<Rc<Thf_pair_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_binary_pairContextAttrs<'input> for Thf_binary_pairContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_binary_pair(&mut self,)
	-> Result<Rc<Thf_binary_pairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_thf_binary_pair);
        let mut _localctx: Rc<Thf_binary_pairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(524);
			recog.thf_unitary_formula()?;

			/*InvokeRule thf_pair_connective*/
			recog.base.set_state(525);
			recog.thf_pair_connective()?;

			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(526);
			recog.thf_unitary_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_binary_tuple ----------------
pub type Thf_binary_tupleContextAll<'input> = Thf_binary_tupleContext<'input>;


pub type Thf_binary_tupleContext<'input> = BaseParserRuleContext<'input,Thf_binary_tupleContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_binary_tupleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_binary_tupleContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_binary_tupleContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_binary_tuple(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_binary_tuple(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_binary_tupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_binary_tuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_tupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_tuple }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_tuple }
}
antlr4rust::tid!{Thf_binary_tupleContextExt<'a>}

impl<'input> Thf_binary_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_tupleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_tupleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_tupleContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_binary_tupleContextExt<'input>>{

fn thf_or_formula(&self) -> Option<Rc<Thf_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_and_formula(&self) -> Option<Rc<Thf_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_apply_formula(&self) -> Option<Rc<Thf_apply_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_binary_tupleContextAttrs<'input> for Thf_binary_tupleContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_binary_tuple(&mut self,)
	-> Result<Rc<Thf_binary_tupleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_tupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_thf_binary_tuple);
        let mut _localctx: Rc<Thf_binary_tupleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(531);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(14,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_or_formula*/
					recog.base.set_state(528);
					recog.thf_or_formula_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule thf_and_formula*/
					recog.base.set_state(529);
					recog.thf_and_formula_rec(0)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule thf_apply_formula*/
					recog.base.set_state(530);
					recog.thf_apply_formula_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_or_formula ----------------
pub type Thf_or_formulaContextAll<'input> = Thf_or_formulaContext<'input>;


pub type Thf_or_formulaContext<'input> = BaseParserRuleContext<'input,Thf_or_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_or_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_or_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_or_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_or_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_or_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_or_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_or_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_or_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_or_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_or_formula }
}
antlr4rust::tid!{Thf_or_formulaContextExt<'a>}

impl<'input> Thf_or_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_or_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_or_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_or_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_or_formulaContextExt<'input>>{

fn thf_unitary_formula_all(&self) ->  Vec<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_formula(&self, i: usize) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Or
/// Returns `None` if there is no child corresponding to token Or
fn Or(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Or, 0)
}
fn thf_or_formula(&self) -> Option<Rc<Thf_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_or_formulaContextAttrs<'input> for Thf_or_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  thf_or_formula(&mut self,)
	-> Result<Rc<Thf_or_formulaContextAll<'input>>,ANTLRError> {
		self.thf_or_formula_rec(0)
	}

	fn thf_or_formula_rec(&mut self, _p: i32)
	-> Result<Rc<Thf_or_formulaContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Thf_or_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 36, RULE_thf_or_formula, _p);
	    let mut _localctx: Rc<Thf_or_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 36;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(534);
			recog.thf_unitary_formula()?;

			recog.base.set_state(535);
			recog.base.match_token(tptp_v7_0_0_0_Or,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(536);
			recog.thf_unitary_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(543);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_or_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_or_formula);
					_localctx = tmp;
					recog.base.set_state(538);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(539);
					recog.base.match_token(tptp_v7_0_0_0_Or,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(540);
					recog.thf_unitary_formula()?;

					}
					} 
				}
				recog.base.set_state(545);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- thf_and_formula ----------------
pub type Thf_and_formulaContextAll<'input> = Thf_and_formulaContext<'input>;


pub type Thf_and_formulaContext<'input> = BaseParserRuleContext<'input,Thf_and_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_and_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_and_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_and_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_and_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_and_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_and_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_and_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_and_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_and_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_and_formula }
}
antlr4rust::tid!{Thf_and_formulaContextExt<'a>}

impl<'input> Thf_and_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_and_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_and_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_and_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_and_formulaContextExt<'input>>{

fn thf_unitary_formula_all(&self) ->  Vec<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_formula(&self, i: usize) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token And
/// Returns `None` if there is no child corresponding to token And
fn And(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_And, 0)
}
fn thf_and_formula(&self) -> Option<Rc<Thf_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_and_formulaContextAttrs<'input> for Thf_and_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  thf_and_formula(&mut self,)
	-> Result<Rc<Thf_and_formulaContextAll<'input>>,ANTLRError> {
		self.thf_and_formula_rec(0)
	}

	fn thf_and_formula_rec(&mut self, _p: i32)
	-> Result<Rc<Thf_and_formulaContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Thf_and_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 38, RULE_thf_and_formula, _p);
	    let mut _localctx: Rc<Thf_and_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 38;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(547);
			recog.thf_unitary_formula()?;

			recog.base.set_state(548);
			recog.base.match_token(tptp_v7_0_0_0_And,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(549);
			recog.thf_unitary_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(556);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_and_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_and_formula);
					_localctx = tmp;
					recog.base.set_state(551);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(552);
					recog.base.match_token(tptp_v7_0_0_0_And,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(553);
					recog.thf_unitary_formula()?;

					}
					} 
				}
				recog.base.set_state(558);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- thf_apply_formula ----------------
pub type Thf_apply_formulaContextAll<'input> = Thf_apply_formulaContext<'input>;


pub type Thf_apply_formulaContext<'input> = BaseParserRuleContext<'input,Thf_apply_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_apply_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_apply_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_apply_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_apply_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_apply_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_apply_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_apply_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_apply_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_apply_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_apply_formula }
}
antlr4rust::tid!{Thf_apply_formulaContextExt<'a>}

impl<'input> Thf_apply_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_apply_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_apply_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_apply_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_apply_formulaContextExt<'input>>{

fn thf_unitary_formula_all(&self) ->  Vec<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_formula(&self, i: usize) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token App
/// Returns `None` if there is no child corresponding to token App
fn App(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_App, 0)
}
fn thf_apply_formula(&self) -> Option<Rc<Thf_apply_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_apply_formulaContextAttrs<'input> for Thf_apply_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  thf_apply_formula(&mut self,)
	-> Result<Rc<Thf_apply_formulaContextAll<'input>>,ANTLRError> {
		self.thf_apply_formula_rec(0)
	}

	fn thf_apply_formula_rec(&mut self, _p: i32)
	-> Result<Rc<Thf_apply_formulaContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Thf_apply_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 40, RULE_thf_apply_formula, _p);
	    let mut _localctx: Rc<Thf_apply_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 40;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(560);
			recog.thf_unitary_formula()?;

			recog.base.set_state(561);
			recog.base.match_token(tptp_v7_0_0_0_App,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(562);
			recog.thf_unitary_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(569);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_apply_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_apply_formula);
					_localctx = tmp;
					recog.base.set_state(564);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(565);
					recog.base.match_token(tptp_v7_0_0_0_App,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(566);
					recog.thf_unitary_formula()?;

					}
					} 
				}
				recog.base.set_state(571);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- thf_unitary_formula ----------------
pub type Thf_unitary_formulaContextAll<'input> = Thf_unitary_formulaContext<'input>;


pub type Thf_unitary_formulaContext<'input> = BaseParserRuleContext<'input,Thf_unitary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_unitary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_unitary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_unitary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_unitary_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_unitary_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_unitary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_unitary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unitary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unitary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unitary_formula }
}
antlr4rust::tid!{Thf_unitary_formulaContextExt<'a>}

impl<'input> Thf_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unitary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unitary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unitary_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_unitary_formulaContextExt<'input>>{

fn thf_quantified_formula(&self) -> Option<Rc<Thf_quantified_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_unary_formula(&self) -> Option<Rc<Thf_unary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_atom(&self) -> Option<Rc<Thf_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_conditional(&self) -> Option<Rc<Thf_conditionalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_let(&self) -> Option<Rc<Thf_letContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_tuple(&self) -> Option<Rc<Thf_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unitary_formulaContextAttrs<'input> for Thf_unitary_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_unitary_formula(&mut self,)
	-> Result<Rc<Thf_unitary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unitary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_thf_unitary_formula);
        let mut _localctx: Rc<Thf_unitary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(582);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(18,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_quantified_formula*/
					recog.base.set_state(572);
					recog.thf_quantified_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule thf_unary_formula*/
					recog.base.set_state(573);
					recog.thf_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule thf_atom*/
					recog.base.set_state(574);
					recog.thf_atom()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule thf_conditional*/
					recog.base.set_state(575);
					recog.thf_conditional()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule thf_let*/
					recog.base.set_state(576);
					recog.thf_let()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule thf_tuple*/
					recog.base.set_state(577);
					recog.thf_tuple()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(578);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(579);
					recog.thf_logic_formula()?;

					recog.base.set_state(580);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_quantified_formula ----------------
pub type Thf_quantified_formulaContextAll<'input> = Thf_quantified_formulaContext<'input>;


pub type Thf_quantified_formulaContext<'input> = BaseParserRuleContext<'input,Thf_quantified_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_quantified_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_quantified_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_quantified_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_quantified_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_quantified_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_quantified_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_quantified_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_quantified_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_quantified_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_quantified_formula }
}
antlr4rust::tid!{Thf_quantified_formulaContextExt<'a>}

impl<'input> Thf_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_quantified_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_quantified_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_quantified_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_quantified_formulaContextExt<'input>>{

fn thf_quantification(&self) -> Option<Rc<Thf_quantificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_unitary_formula(&self) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_quantified_formulaContextAttrs<'input> for Thf_quantified_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_quantified_formula(&mut self,)
	-> Result<Rc<Thf_quantified_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_quantified_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_thf_quantified_formula);
        let mut _localctx: Rc<Thf_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_quantification*/
			recog.base.set_state(584);
			recog.thf_quantification()?;

			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(585);
			recog.thf_unitary_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_quantification ----------------
pub type Thf_quantificationContextAll<'input> = Thf_quantificationContext<'input>;


pub type Thf_quantificationContext<'input> = BaseParserRuleContext<'input,Thf_quantificationContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_quantificationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_quantificationContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_quantificationContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_quantification(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_quantification(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_quantificationContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_quantification(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_quantificationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_quantification }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_quantification }
}
antlr4rust::tid!{Thf_quantificationContextExt<'a>}

impl<'input> Thf_quantificationContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_quantificationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_quantificationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_quantificationContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_quantificationContextExt<'input>>{

fn thf_quantifier(&self) -> Option<Rc<Thf_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_variable_list(&self) -> Option<Rc<Thf_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_quantificationContextAttrs<'input> for Thf_quantificationContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_quantification(&mut self,)
	-> Result<Rc<Thf_quantificationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_quantificationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_thf_quantification);
        let mut _localctx: Rc<Thf_quantificationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_quantifier*/
			recog.base.set_state(587);
			recog.thf_quantifier()?;

			recog.base.set_state(588);
			recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

			/*InvokeRule thf_variable_list*/
			recog.base.set_state(589);
			recog.thf_variable_list()?;

			recog.base.set_state(590);
			recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

			recog.base.set_state(591);
			recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_variable_list ----------------
pub type Thf_variable_listContextAll<'input> = Thf_variable_listContext<'input>;


pub type Thf_variable_listContext<'input> = BaseParserRuleContext<'input,Thf_variable_listContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_variable_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_variable_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_variable_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_variable_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_variable_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_variable_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_variable_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_variable_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_variable_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_variable_list }
}
antlr4rust::tid!{Thf_variable_listContextExt<'a>}

impl<'input> Thf_variable_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_variable_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_variable_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_variable_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_variable_listContextExt<'input>>{

fn thf_variable_all(&self) ->  Vec<Rc<Thf_variableContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_variable(&self, i: usize) -> Option<Rc<Thf_variableContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Thf_variable_listContextAttrs<'input> for Thf_variable_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_variable_list(&mut self,)
	-> Result<Rc<Thf_variable_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_variable_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_thf_variable_list);
        let mut _localctx: Rc<Thf_variable_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_variable*/
			recog.base.set_state(593);
			recog.thf_variable()?;

			recog.base.set_state(598);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(594);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule thf_variable*/
				recog.base.set_state(595);
				recog.thf_variable()?;

				}
				}
				recog.base.set_state(600);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_variable ----------------
pub type Thf_variableContextAll<'input> = Thf_variableContext<'input>;


pub type Thf_variableContext<'input> = BaseParserRuleContext<'input,Thf_variableContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_variableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_variableContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_variableContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_variable(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_variable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_variableContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_variableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_variable }
}
antlr4rust::tid!{Thf_variableContextExt<'a>}

impl<'input> Thf_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_variableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_variableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_variableContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_variableContextExt<'input>>{

fn thf_typed_variable(&self) -> Option<Rc<Thf_typed_variableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_variableContextAttrs<'input> for Thf_variableContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_variable(&mut self,)
	-> Result<Rc<Thf_variableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_variableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_thf_variable);
        let mut _localctx: Rc<Thf_variableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(603);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_typed_variable*/
					recog.base.set_state(601);
					recog.thf_typed_variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable*/
					recog.base.set_state(602);
					recog.variable()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_typed_variable ----------------
pub type Thf_typed_variableContextAll<'input> = Thf_typed_variableContext<'input>;


pub type Thf_typed_variableContext<'input> = BaseParserRuleContext<'input,Thf_typed_variableContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_typed_variableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_typed_variableContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_typed_variableContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_typed_variable(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_typed_variable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_typed_variableContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_typed_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_typed_variableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_typed_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_typed_variable }
}
antlr4rust::tid!{Thf_typed_variableContextExt<'a>}

impl<'input> Thf_typed_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_typed_variableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_typed_variableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_typed_variableContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_typed_variableContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_top_level_type(&self) -> Option<Rc<Thf_top_level_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_typed_variableContextAttrs<'input> for Thf_typed_variableContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_typed_variable(&mut self,)
	-> Result<Rc<Thf_typed_variableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_typed_variableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_thf_typed_variable);
        let mut _localctx: Rc<Thf_typed_variableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variable*/
			recog.base.set_state(605);
			recog.variable()?;

			recog.base.set_state(606);
			recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

			/*InvokeRule thf_top_level_type*/
			recog.base.set_state(607);
			recog.thf_top_level_type()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_unary_formula ----------------
pub type Thf_unary_formulaContextAll<'input> = Thf_unary_formulaContext<'input>;


pub type Thf_unary_formulaContext<'input> = BaseParserRuleContext<'input,Thf_unary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_unary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_unary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_unary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_unary_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_unary_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_unary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_unary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unary_formula }
}
antlr4rust::tid!{Thf_unary_formulaContextExt<'a>}

impl<'input> Thf_unary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unary_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_unary_formulaContextExt<'input>>{

fn thf_unary_connective(&self) -> Option<Rc<Thf_unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unary_formulaContextAttrs<'input> for Thf_unary_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_unary_formula(&mut self,)
	-> Result<Rc<Thf_unary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_thf_unary_formula);
        let mut _localctx: Rc<Thf_unary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_unary_connective*/
			recog.base.set_state(609);
			recog.thf_unary_connective()?;

			recog.base.set_state(610);
			recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(611);
			recog.thf_logic_formula()?;

			recog.base.set_state(612);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_atom ----------------
pub type Thf_atomContextAll<'input> = Thf_atomContext<'input>;


pub type Thf_atomContext<'input> = BaseParserRuleContext<'input,Thf_atomContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_atomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_atomContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_atomContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_atom(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_atom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_atomContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_atomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_atom }
}
antlr4rust::tid!{Thf_atomContextExt<'a>}

impl<'input> Thf_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_atomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_atomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_atomContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_atomContextExt<'input>>{

fn thf_function(&self) -> Option<Rc<Thf_functionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_term(&self) -> Option<Rc<Defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_conn_term(&self) -> Option<Rc<Thf_conn_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_atomContextAttrs<'input> for Thf_atomContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_atom(&mut self,)
	-> Result<Rc<Thf_atomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_atomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_thf_atom);
        let mut _localctx: Rc<Thf_atomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(618);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Dollar_word |tptp_v7_0_0_0_Dollar_dollar_word |tptp_v7_0_0_0_Lower_word |
			tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_function*/
					recog.base.set_state(614);
					recog.thf_function()?;

					}
				}

			tptp_v7_0_0_0_Upper_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable*/
					recog.base.set_state(615);
					recog.variable()?;

					}
				}

			tptp_v7_0_0_0_Real |tptp_v7_0_0_0_Rational |tptp_v7_0_0_0_Integer |tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(616);
					recog.defined_term()?;

					}
				}

			tptp_v7_0_0_0_Or |tptp_v7_0_0_0_And |tptp_v7_0_0_0_Iff |tptp_v7_0_0_0_Impl |
			tptp_v7_0_0_0_If |tptp_v7_0_0_0_Niff |tptp_v7_0_0_0_Nor |tptp_v7_0_0_0_Nand |
			tptp_v7_0_0_0_Not |tptp_v7_0_0_0_ForallComb |tptp_v7_0_0_0_Infix_inequality |
			tptp_v7_0_0_0_Infix_equality |tptp_v7_0_0_0_ExistsComb |tptp_v7_0_0_0_ChoiceComb |
			tptp_v7_0_0_0_DescriptionComb |tptp_v7_0_0_0_EqComb |tptp_v7_0_0_0_Assignment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule thf_conn_term*/
					recog.base.set_state(617);
					recog.thf_conn_term()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_function ----------------
pub type Thf_functionContextAll<'input> = Thf_functionContext<'input>;


pub type Thf_functionContext<'input> = BaseParserRuleContext<'input,Thf_functionContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_functionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_functionContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_functionContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_function(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_function(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_functionContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_function(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_functionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_function }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_function }
}
antlr4rust::tid!{Thf_functionContextExt<'a>}

impl<'input> Thf_functionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_functionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_functionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_functionContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_functionContextExt<'input>>{

fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functor(&self) -> Option<Rc<FunctorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_arguments(&self) -> Option<Rc<Thf_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_functor(&self) -> Option<Rc<Defined_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn system_functor(&self) -> Option<Rc<System_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_functionContextAttrs<'input> for Thf_functionContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_function(&mut self,)
	-> Result<Rc<Thf_functionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_functionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_thf_function);
        let mut _localctx: Rc<Thf_functionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(636);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule atom*/
					recog.base.set_state(620);
					recog.atom()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule functor*/
					recog.base.set_state(621);
					recog.functor()?;

					recog.base.set_state(622);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(623);
					recog.thf_arguments()?;

					recog.base.set_state(624);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule defined_functor*/
					recog.base.set_state(626);
					recog.defined_functor()?;

					recog.base.set_state(627);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(628);
					recog.thf_arguments()?;

					recog.base.set_state(629);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule system_functor*/
					recog.base.set_state(631);
					recog.system_functor()?;

					recog.base.set_state(632);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(633);
					recog.thf_arguments()?;

					recog.base.set_state(634);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_conn_term ----------------
pub type Thf_conn_termContextAll<'input> = Thf_conn_termContext<'input>;


pub type Thf_conn_termContext<'input> = BaseParserRuleContext<'input,Thf_conn_termContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_conn_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_conn_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_conn_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_conn_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_conn_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_conn_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_conn_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_conn_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_conn_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_conn_term }
}
antlr4rust::tid!{Thf_conn_termContextExt<'a>}

impl<'input> Thf_conn_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_conn_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_conn_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_conn_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_conn_termContextExt<'input>>{

fn thf_pair_connective(&self) -> Option<Rc<Thf_pair_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assoc_connective(&self) -> Option<Rc<Assoc_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_unary_connective(&self) -> Option<Rc<Thf_unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_conn_termContextAttrs<'input> for Thf_conn_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_conn_term(&mut self,)
	-> Result<Rc<Thf_conn_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_conn_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_thf_conn_term);
        let mut _localctx: Rc<Thf_conn_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(641);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Iff |tptp_v7_0_0_0_Impl |tptp_v7_0_0_0_If |tptp_v7_0_0_0_Niff |
			tptp_v7_0_0_0_Nor |tptp_v7_0_0_0_Nand |tptp_v7_0_0_0_Infix_inequality |
			tptp_v7_0_0_0_Infix_equality |tptp_v7_0_0_0_Assignment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_pair_connective*/
					recog.base.set_state(638);
					recog.thf_pair_connective()?;

					}
				}

			tptp_v7_0_0_0_Or |tptp_v7_0_0_0_And 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assoc_connective*/
					recog.base.set_state(639);
					recog.assoc_connective()?;

					}
				}

			tptp_v7_0_0_0_Not |tptp_v7_0_0_0_ForallComb |tptp_v7_0_0_0_ExistsComb |
			tptp_v7_0_0_0_ChoiceComb |tptp_v7_0_0_0_DescriptionComb |tptp_v7_0_0_0_EqComb 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule thf_unary_connective*/
					recog.base.set_state(640);
					recog.thf_unary_connective()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_conditional ----------------
pub type Thf_conditionalContextAll<'input> = Thf_conditionalContext<'input>;


pub type Thf_conditionalContext<'input> = BaseParserRuleContext<'input,Thf_conditionalContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_conditionalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_conditionalContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_conditionalContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_conditional(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_conditional(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_conditionalContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_conditional(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_conditionalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_conditional }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_conditional }
}
antlr4rust::tid!{Thf_conditionalContextExt<'a>}

impl<'input> Thf_conditionalContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_conditionalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_conditionalContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_conditionalContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_conditionalContextExt<'input>>{

fn thf_logic_formula_all(&self) ->  Vec<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_logic_formula(&self, i: usize) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Thf_conditionalContextAttrs<'input> for Thf_conditionalContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_conditional(&mut self,)
	-> Result<Rc<Thf_conditionalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_conditionalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_thf_conditional);
        let mut _localctx: Rc<Thf_conditionalContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(643);
			recog.base.match_token(tptp_v7_0_0_0_T__14,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(644);
			recog.thf_logic_formula()?;

			recog.base.set_state(645);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(646);
			recog.thf_logic_formula()?;

			recog.base.set_state(647);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(648);
			recog.thf_logic_formula()?;

			recog.base.set_state(649);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_let ----------------
pub type Thf_letContextAll<'input> = Thf_letContext<'input>;


pub type Thf_letContext<'input> = BaseParserRuleContext<'input,Thf_letContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_letContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_letContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_letContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_let(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_let(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_letContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_let(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_letContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_let }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_let }
}
antlr4rust::tid!{Thf_letContextExt<'a>}

impl<'input> Thf_letContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_letContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_letContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_letContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_letContextExt<'input>>{

fn thf_unitary_formula(&self) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_formula(&self) -> Option<Rc<Thf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_letContextAttrs<'input> for Thf_letContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_let(&mut self,)
	-> Result<Rc<Thf_letContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_letContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_thf_let);
        let mut _localctx: Rc<Thf_letContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(651);
			recog.base.match_token(tptp_v7_0_0_0_T__15,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(652);
			recog.thf_unitary_formula()?;

			recog.base.set_state(653);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_formula*/
			recog.base.set_state(654);
			recog.thf_formula()?;

			recog.base.set_state(655);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_arguments ----------------
pub type Thf_argumentsContextAll<'input> = Thf_argumentsContext<'input>;


pub type Thf_argumentsContext<'input> = BaseParserRuleContext<'input,Thf_argumentsContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_argumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_argumentsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_argumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_arguments(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_arguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_argumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_arguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_argumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_arguments }
}
antlr4rust::tid!{Thf_argumentsContextExt<'a>}

impl<'input> Thf_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_argumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_argumentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_argumentsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_argumentsContextExt<'input>>{

fn thf_formula_list(&self) -> Option<Rc<Thf_formula_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_argumentsContextAttrs<'input> for Thf_argumentsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_arguments(&mut self,)
	-> Result<Rc<Thf_argumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_argumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_thf_arguments);
        let mut _localctx: Rc<Thf_argumentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_formula_list*/
			recog.base.set_state(657);
			recog.thf_formula_list()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_type_formula ----------------
pub type Thf_type_formulaContextAll<'input> = Thf_type_formulaContext<'input>;


pub type Thf_type_formulaContext<'input> = BaseParserRuleContext<'input,Thf_type_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_type_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_type_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_type_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_type_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_type_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_type_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_type_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_type_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_type_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_type_formula }
}
antlr4rust::tid!{Thf_type_formulaContextExt<'a>}

impl<'input> Thf_type_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_type_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_type_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_type_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_type_formulaContextExt<'input>>{

fn thf_typeable_formula(&self) -> Option<Rc<Thf_typeable_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_top_level_type(&self) -> Option<Rc<Thf_top_level_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_type_formulaContextAttrs<'input> for Thf_type_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_type_formula(&mut self,)
	-> Result<Rc<Thf_type_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_type_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_thf_type_formula);
        let mut _localctx: Rc<Thf_type_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_typeable_formula*/
			recog.base.set_state(659);
			recog.thf_typeable_formula()?;

			recog.base.set_state(660);
			recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

			/*InvokeRule thf_top_level_type*/
			recog.base.set_state(661);
			recog.thf_top_level_type()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_typeable_formula ----------------
pub type Thf_typeable_formulaContextAll<'input> = Thf_typeable_formulaContext<'input>;


pub type Thf_typeable_formulaContext<'input> = BaseParserRuleContext<'input,Thf_typeable_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_typeable_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_typeable_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_typeable_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_typeable_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_typeable_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_typeable_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_typeable_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_typeable_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_typeable_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_typeable_formula }
}
antlr4rust::tid!{Thf_typeable_formulaContextExt<'a>}

impl<'input> Thf_typeable_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_typeable_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_typeable_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_typeable_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_typeable_formulaContextExt<'input>>{

fn thf_atom(&self) -> Option<Rc<Thf_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_typeable_formulaContextAttrs<'input> for Thf_typeable_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_typeable_formula(&mut self,)
	-> Result<Rc<Thf_typeable_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_typeable_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_thf_typeable_formula);
        let mut _localctx: Rc<Thf_typeable_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(668);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Or |tptp_v7_0_0_0_And |tptp_v7_0_0_0_Iff |tptp_v7_0_0_0_Impl |
			tptp_v7_0_0_0_If |tptp_v7_0_0_0_Niff |tptp_v7_0_0_0_Nor |tptp_v7_0_0_0_Nand |
			tptp_v7_0_0_0_Not |tptp_v7_0_0_0_ForallComb |tptp_v7_0_0_0_Infix_inequality |
			tptp_v7_0_0_0_Infix_equality |tptp_v7_0_0_0_ExistsComb |tptp_v7_0_0_0_ChoiceComb |
			tptp_v7_0_0_0_DescriptionComb |tptp_v7_0_0_0_EqComb |tptp_v7_0_0_0_Assignment |
			tptp_v7_0_0_0_Real |tptp_v7_0_0_0_Rational |tptp_v7_0_0_0_Integer |tptp_v7_0_0_0_Dollar_word |
			tptp_v7_0_0_0_Dollar_dollar_word |tptp_v7_0_0_0_Upper_word |tptp_v7_0_0_0_Lower_word |
			tptp_v7_0_0_0_Single_quoted |tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_atom*/
					recog.base.set_state(663);
					recog.thf_atom()?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(664);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(665);
					recog.thf_logic_formula()?;

					recog.base.set_state(666);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_subtype ----------------
pub type Thf_subtypeContextAll<'input> = Thf_subtypeContext<'input>;


pub type Thf_subtypeContext<'input> = BaseParserRuleContext<'input,Thf_subtypeContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_subtypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_subtypeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_subtypeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_subtype(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_subtype(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_subtypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_subtype(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_subtypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_subtype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_subtype }
}
antlr4rust::tid!{Thf_subtypeContextExt<'a>}

impl<'input> Thf_subtypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_subtypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_subtypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_subtypeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_subtypeContextExt<'input>>{

fn thf_atom_all(&self) ->  Vec<Rc<Thf_atomContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_atom(&self, i: usize) -> Option<Rc<Thf_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Subtype_sign
/// Returns `None` if there is no child corresponding to token Subtype_sign
fn Subtype_sign(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Subtype_sign, 0)
}

}

impl<'input> Thf_subtypeContextAttrs<'input> for Thf_subtypeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_subtype(&mut self,)
	-> Result<Rc<Thf_subtypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_subtypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_thf_subtype);
        let mut _localctx: Rc<Thf_subtypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_atom*/
			recog.base.set_state(670);
			recog.thf_atom()?;

			recog.base.set_state(671);
			recog.base.match_token(tptp_v7_0_0_0_Subtype_sign,&mut recog.err_handler)?;

			/*InvokeRule thf_atom*/
			recog.base.set_state(672);
			recog.thf_atom()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_top_level_type ----------------
pub type Thf_top_level_typeContextAll<'input> = Thf_top_level_typeContext<'input>;


pub type Thf_top_level_typeContext<'input> = BaseParserRuleContext<'input,Thf_top_level_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_top_level_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_top_level_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_top_level_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_top_level_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_top_level_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_top_level_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_top_level_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_top_level_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_top_level_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_top_level_type }
}
antlr4rust::tid!{Thf_top_level_typeContextExt<'a>}

impl<'input> Thf_top_level_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_top_level_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_top_level_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_top_level_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_top_level_typeContextExt<'input>>{

fn thf_unitary_type(&self) -> Option<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_mapping_type(&self) -> Option<Rc<Thf_mapping_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_apply_type(&self) -> Option<Rc<Thf_apply_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_top_level_typeContextAttrs<'input> for Thf_top_level_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_top_level_type(&mut self,)
	-> Result<Rc<Thf_top_level_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_top_level_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_thf_top_level_type);
        let mut _localctx: Rc<Thf_top_level_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(677);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(25,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(674);
					recog.thf_unitary_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule thf_mapping_type*/
					recog.base.set_state(675);
					recog.thf_mapping_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule thf_apply_type*/
					recog.base.set_state(676);
					recog.thf_apply_type()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_unitary_type ----------------
pub type Thf_unitary_typeContextAll<'input> = Thf_unitary_typeContext<'input>;


pub type Thf_unitary_typeContext<'input> = BaseParserRuleContext<'input,Thf_unitary_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_unitary_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_unitary_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_unitary_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_unitary_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_unitary_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_unitary_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_unitary_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unitary_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unitary_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unitary_type }
}
antlr4rust::tid!{Thf_unitary_typeContextExt<'a>}

impl<'input> Thf_unitary_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unitary_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unitary_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unitary_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_unitary_typeContextExt<'input>>{

fn thf_unitary_formula(&self) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unitary_typeContextAttrs<'input> for Thf_unitary_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_unitary_type(&mut self,)
	-> Result<Rc<Thf_unitary_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unitary_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_thf_unitary_type);
        let mut _localctx: Rc<Thf_unitary_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(679);
			recog.thf_unitary_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_apply_type ----------------
pub type Thf_apply_typeContextAll<'input> = Thf_apply_typeContext<'input>;


pub type Thf_apply_typeContext<'input> = BaseParserRuleContext<'input,Thf_apply_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_apply_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_apply_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_apply_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_apply_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_apply_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_apply_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_apply_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_apply_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_apply_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_apply_type }
}
antlr4rust::tid!{Thf_apply_typeContextExt<'a>}

impl<'input> Thf_apply_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_apply_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_apply_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_apply_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_apply_typeContextExt<'input>>{

fn thf_apply_formula(&self) -> Option<Rc<Thf_apply_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_apply_typeContextAttrs<'input> for Thf_apply_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_apply_type(&mut self,)
	-> Result<Rc<Thf_apply_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_apply_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_thf_apply_type);
        let mut _localctx: Rc<Thf_apply_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_apply_formula*/
			recog.base.set_state(681);
			recog.thf_apply_formula_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_binary_type ----------------
pub type Thf_binary_typeContextAll<'input> = Thf_binary_typeContext<'input>;


pub type Thf_binary_typeContext<'input> = BaseParserRuleContext<'input,Thf_binary_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_binary_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_binary_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_binary_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_binary_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_binary_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_binary_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_binary_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_type }
}
antlr4rust::tid!{Thf_binary_typeContextExt<'a>}

impl<'input> Thf_binary_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_binary_typeContextExt<'input>>{

fn thf_mapping_type(&self) -> Option<Rc<Thf_mapping_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_xprod_type(&self) -> Option<Rc<Thf_xprod_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_union_type(&self) -> Option<Rc<Thf_union_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_binary_typeContextAttrs<'input> for Thf_binary_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_binary_type(&mut self,)
	-> Result<Rc<Thf_binary_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_thf_binary_type);
        let mut _localctx: Rc<Thf_binary_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(686);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_mapping_type*/
					recog.base.set_state(683);
					recog.thf_mapping_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule thf_xprod_type*/
					recog.base.set_state(684);
					recog.thf_xprod_type_rec(0)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule thf_union_type*/
					recog.base.set_state(685);
					recog.thf_union_type_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_mapping_type ----------------
pub type Thf_mapping_typeContextAll<'input> = Thf_mapping_typeContext<'input>;


pub type Thf_mapping_typeContext<'input> = BaseParserRuleContext<'input,Thf_mapping_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_mapping_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_mapping_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_mapping_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_mapping_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_mapping_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_mapping_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_mapping_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_mapping_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_mapping_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_mapping_type }
}
antlr4rust::tid!{Thf_mapping_typeContextExt<'a>}

impl<'input> Thf_mapping_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_mapping_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_mapping_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_mapping_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_mapping_typeContextExt<'input>>{

fn thf_unitary_type_all(&self) ->  Vec<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_type(&self, i: usize) -> Option<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Arrow
/// Returns `None` if there is no child corresponding to token Arrow
fn Arrow(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Arrow, 0)
}
fn thf_mapping_type(&self) -> Option<Rc<Thf_mapping_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_mapping_typeContextAttrs<'input> for Thf_mapping_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_mapping_type(&mut self,)
	-> Result<Rc<Thf_mapping_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_mapping_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_thf_mapping_type);
        let mut _localctx: Rc<Thf_mapping_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(696);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(688);
					recog.thf_unitary_type()?;

					recog.base.set_state(689);
					recog.base.match_token(tptp_v7_0_0_0_Arrow,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(690);
					recog.thf_unitary_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(692);
					recog.thf_unitary_type()?;

					recog.base.set_state(693);
					recog.base.match_token(tptp_v7_0_0_0_Arrow,&mut recog.err_handler)?;

					/*InvokeRule thf_mapping_type*/
					recog.base.set_state(694);
					recog.thf_mapping_type()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_xprod_type ----------------
pub type Thf_xprod_typeContextAll<'input> = Thf_xprod_typeContext<'input>;


pub type Thf_xprod_typeContext<'input> = BaseParserRuleContext<'input,Thf_xprod_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_xprod_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_xprod_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_xprod_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_xprod_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_xprod_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_xprod_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_xprod_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_xprod_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_xprod_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_xprod_type }
}
antlr4rust::tid!{Thf_xprod_typeContextExt<'a>}

impl<'input> Thf_xprod_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_xprod_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_xprod_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_xprod_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_xprod_typeContextExt<'input>>{

fn thf_unitary_type_all(&self) ->  Vec<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_type(&self, i: usize) -> Option<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Star, 0)
}
fn thf_xprod_type(&self) -> Option<Rc<Thf_xprod_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_xprod_typeContextAttrs<'input> for Thf_xprod_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  thf_xprod_type(&mut self,)
	-> Result<Rc<Thf_xprod_typeContextAll<'input>>,ANTLRError> {
		self.thf_xprod_type_rec(0)
	}

	fn thf_xprod_type_rec(&mut self, _p: i32)
	-> Result<Rc<Thf_xprod_typeContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Thf_xprod_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 84, RULE_thf_xprod_type, _p);
	    let mut _localctx: Rc<Thf_xprod_typeContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 84;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(699);
			recog.thf_unitary_type()?;

			recog.base.set_state(700);
			recog.base.match_token(tptp_v7_0_0_0_Star,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(701);
			recog.thf_unitary_type()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(708);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(28,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_xprod_typeContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_xprod_type);
					_localctx = tmp;
					recog.base.set_state(703);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(704);
					recog.base.match_token(tptp_v7_0_0_0_Star,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(705);
					recog.thf_unitary_type()?;

					}
					} 
				}
				recog.base.set_state(710);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(28,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- thf_union_type ----------------
pub type Thf_union_typeContextAll<'input> = Thf_union_typeContext<'input>;


pub type Thf_union_typeContext<'input> = BaseParserRuleContext<'input,Thf_union_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_union_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_union_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_union_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_union_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_union_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_union_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_union_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_union_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_union_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_union_type }
}
antlr4rust::tid!{Thf_union_typeContextExt<'a>}

impl<'input> Thf_union_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_union_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_union_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_union_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_union_typeContextExt<'input>>{

fn thf_unitary_type_all(&self) ->  Vec<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_type(&self, i: usize) -> Option<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Plus
/// Returns `None` if there is no child corresponding to token Plus
fn Plus(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Plus, 0)
}
fn thf_union_type(&self) -> Option<Rc<Thf_union_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_union_typeContextAttrs<'input> for Thf_union_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  thf_union_type(&mut self,)
	-> Result<Rc<Thf_union_typeContextAll<'input>>,ANTLRError> {
		self.thf_union_type_rec(0)
	}

	fn thf_union_type_rec(&mut self, _p: i32)
	-> Result<Rc<Thf_union_typeContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Thf_union_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 86, RULE_thf_union_type, _p);
	    let mut _localctx: Rc<Thf_union_typeContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 86;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(712);
			recog.thf_unitary_type()?;

			recog.base.set_state(713);
			recog.base.match_token(tptp_v7_0_0_0_Plus,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(714);
			recog.thf_unitary_type()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(721);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(29,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_union_typeContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_union_type);
					_localctx = tmp;
					recog.base.set_state(716);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(717);
					recog.base.match_token(tptp_v7_0_0_0_Plus,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(718);
					recog.thf_unitary_type()?;

					}
					} 
				}
				recog.base.set_state(723);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(29,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- thf_sequent ----------------
pub type Thf_sequentContextAll<'input> = Thf_sequentContext<'input>;


pub type Thf_sequentContext<'input> = BaseParserRuleContext<'input,Thf_sequentContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_sequentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_sequentContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_sequentContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_sequent(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_sequent(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_sequentContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_sequent(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_sequentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_sequent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_sequent }
}
antlr4rust::tid!{Thf_sequentContextExt<'a>}

impl<'input> Thf_sequentContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_sequentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_sequentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_sequentContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_sequentContextExt<'input>>{

fn thf_tuple_all(&self) ->  Vec<Rc<Thf_tupleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_tuple(&self, i: usize) -> Option<Rc<Thf_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Gentzen_arrow
/// Returns `None` if there is no child corresponding to token Gentzen_arrow
fn Gentzen_arrow(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Gentzen_arrow, 0)
}
fn thf_sequent(&self) -> Option<Rc<Thf_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_sequentContextAttrs<'input> for Thf_sequentContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_sequent(&mut self,)
	-> Result<Rc<Thf_sequentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_sequentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_thf_sequent);
        let mut _localctx: Rc<Thf_sequentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(732);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__11 |tptp_v7_0_0_0_T__16 |tptp_v7_0_0_0_T__17 |tptp_v7_0_0_0_T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_tuple*/
					recog.base.set_state(724);
					recog.thf_tuple()?;

					recog.base.set_state(725);
					recog.base.match_token(tptp_v7_0_0_0_Gentzen_arrow,&mut recog.err_handler)?;

					/*InvokeRule thf_tuple*/
					recog.base.set_state(726);
					recog.thf_tuple()?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(728);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_sequent*/
					recog.base.set_state(729);
					recog.thf_sequent()?;

					recog.base.set_state(730);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_tuple ----------------
pub type Thf_tupleContextAll<'input> = Thf_tupleContext<'input>;


pub type Thf_tupleContext<'input> = BaseParserRuleContext<'input,Thf_tupleContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_tupleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_tupleContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_tupleContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_tuple(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_tuple(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_tupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_tuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_tupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_tuple }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_tuple }
}
antlr4rust::tid!{Thf_tupleContextExt<'a>}

impl<'input> Thf_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_tupleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_tupleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_tupleContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_tupleContextExt<'input>>{

fn thf_formula_list(&self) -> Option<Rc<Thf_formula_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_tupleContextAttrs<'input> for Thf_tupleContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_tuple(&mut self,)
	-> Result<Rc<Thf_tupleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_tupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_thf_tuple);
        let mut _localctx: Rc<Thf_tupleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(744);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(734);
					recog.base.match_token(tptp_v7_0_0_0_T__16,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(735);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_formula_list*/
					recog.base.set_state(736);
					recog.thf_formula_list()?;

					recog.base.set_state(737);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(739);
					recog.base.match_token(tptp_v7_0_0_0_T__17,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(740);
					recog.base.match_token(tptp_v7_0_0_0_T__18,&mut recog.err_handler)?;

					/*InvokeRule thf_formula_list*/
					recog.base.set_state(741);
					recog.thf_formula_list()?;

					recog.base.set_state(742);
					recog.base.match_token(tptp_v7_0_0_0_T__19,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_formula_list ----------------
pub type Thf_formula_listContextAll<'input> = Thf_formula_listContext<'input>;


pub type Thf_formula_listContext<'input> = BaseParserRuleContext<'input,Thf_formula_listContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_formula_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_formula_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_formula_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_formula_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_formula_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_formula_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_formula_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_formula_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_formula_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_formula_list }
}
antlr4rust::tid!{Thf_formula_listContextExt<'a>}

impl<'input> Thf_formula_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_formula_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_formula_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_formula_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_formula_listContextExt<'input>>{

fn thf_logic_formula_all(&self) ->  Vec<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_logic_formula(&self, i: usize) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Thf_formula_listContextAttrs<'input> for Thf_formula_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_formula_list(&mut self,)
	-> Result<Rc<Thf_formula_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_formula_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_thf_formula_list);
        let mut _localctx: Rc<Thf_formula_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(746);
			recog.thf_logic_formula()?;

			recog.base.set_state(751);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(747);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule thf_logic_formula*/
				recog.base.set_state(748);
				recog.thf_logic_formula()?;

				}
				}
				recog.base.set_state(753);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tfx_formula ----------------
pub type Tfx_formulaContextAll<'input> = Tfx_formulaContext<'input>;


pub type Tfx_formulaContext<'input> = BaseParserRuleContext<'input,Tfx_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tfx_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tfx_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tfx_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tfx_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tfx_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tfx_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tfx_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tfx_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tfx_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tfx_formula }
}
antlr4rust::tid!{Tfx_formulaContextExt<'a>}

impl<'input> Tfx_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tfx_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tfx_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tfx_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tfx_formulaContextExt<'input>>{

fn tfx_logic_formula(&self) -> Option<Rc<Tfx_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_sequent(&self) -> Option<Rc<Thf_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tfx_formulaContextAttrs<'input> for Tfx_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tfx_formula(&mut self,)
	-> Result<Rc<Tfx_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tfx_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_tfx_formula);
        let mut _localctx: Rc<Tfx_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(756);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tfx_logic_formula*/
					recog.base.set_state(754);
					recog.tfx_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule thf_sequent*/
					recog.base.set_state(755);
					recog.thf_sequent()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tfx_logic_formula ----------------
pub type Tfx_logic_formulaContextAll<'input> = Tfx_logic_formulaContext<'input>;


pub type Tfx_logic_formulaContext<'input> = BaseParserRuleContext<'input,Tfx_logic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tfx_logic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tfx_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tfx_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tfx_logic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tfx_logic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tfx_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tfx_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tfx_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tfx_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tfx_logic_formula }
}
antlr4rust::tid!{Tfx_logic_formulaContextExt<'a>}

impl<'input> Tfx_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tfx_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tfx_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tfx_logic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tfx_logic_formulaContextExt<'input>>{

fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tfx_logic_formulaContextAttrs<'input> for Tfx_logic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tfx_logic_formula(&mut self,)
	-> Result<Rc<Tfx_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tfx_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_tfx_logic_formula);
        let mut _localctx: Rc<Tfx_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(758);
			recog.thf_logic_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_formula ----------------
pub type Tff_formulaContextAll<'input> = Tff_formulaContext<'input>;


pub type Tff_formulaContext<'input> = BaseParserRuleContext<'input,Tff_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_formula }
}
antlr4rust::tid!{Tff_formulaContextExt<'a>}

impl<'input> Tff_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_formulaContextExt<'input>>{

fn tff_logic_formula(&self) -> Option<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_typed_atom(&self) -> Option<Rc<Tff_typed_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_sequent(&self) -> Option<Rc<Tff_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_formulaContextAttrs<'input> for Tff_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_formula(&mut self,)
	-> Result<Rc<Tff_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_tff_formula);
        let mut _localctx: Rc<Tff_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(763);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(34,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(760);
					recog.tff_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tff_typed_atom*/
					recog.base.set_state(761);
					recog.tff_typed_atom()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule tff_sequent*/
					recog.base.set_state(762);
					recog.tff_sequent()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_logic_formula ----------------
pub type Tff_logic_formulaContextAll<'input> = Tff_logic_formulaContext<'input>;


pub type Tff_logic_formulaContext<'input> = BaseParserRuleContext<'input,Tff_logic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_logic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_logic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_logic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_logic_formula }
}
antlr4rust::tid!{Tff_logic_formulaContextExt<'a>}

impl<'input> Tff_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_logic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_logic_formulaContextExt<'input>>{

fn tff_binary_formula(&self) -> Option<Rc<Tff_binary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_unitary_formula(&self) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_subtype(&self) -> Option<Rc<Tff_subtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_logic_formulaContextAttrs<'input> for Tff_logic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_logic_formula(&mut self,)
	-> Result<Rc<Tff_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_tff_logic_formula);
        let mut _localctx: Rc<Tff_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(768);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(35,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_binary_formula*/
					recog.base.set_state(765);
					recog.tff_binary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(766);
					recog.tff_unitary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule tff_subtype*/
					recog.base.set_state(767);
					recog.tff_subtype()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_binary_formula ----------------
pub type Tff_binary_formulaContextAll<'input> = Tff_binary_formulaContext<'input>;


pub type Tff_binary_formulaContext<'input> = BaseParserRuleContext<'input,Tff_binary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_binary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_binary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_binary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_binary_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_binary_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_binary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_binary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_binary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_binary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_binary_formula }
}
antlr4rust::tid!{Tff_binary_formulaContextExt<'a>}

impl<'input> Tff_binary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_binary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_binary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_binary_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_binary_formulaContextExt<'input>>{

fn tff_binary_nonassoc(&self) -> Option<Rc<Tff_binary_nonassocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_binary_assoc(&self) -> Option<Rc<Tff_binary_assocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_binary_formulaContextAttrs<'input> for Tff_binary_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_binary_formula(&mut self,)
	-> Result<Rc<Tff_binary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_binary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_tff_binary_formula);
        let mut _localctx: Rc<Tff_binary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(772);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(36,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_binary_nonassoc*/
					recog.base.set_state(770);
					recog.tff_binary_nonassoc()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tff_binary_assoc*/
					recog.base.set_state(771);
					recog.tff_binary_assoc()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_binary_nonassoc ----------------
pub type Tff_binary_nonassocContextAll<'input> = Tff_binary_nonassocContext<'input>;


pub type Tff_binary_nonassocContext<'input> = BaseParserRuleContext<'input,Tff_binary_nonassocContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_binary_nonassocContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_binary_nonassocContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_binary_nonassocContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_binary_nonassoc(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_binary_nonassoc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_binary_nonassocContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_binary_nonassoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_binary_nonassocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_binary_nonassoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_binary_nonassoc }
}
antlr4rust::tid!{Tff_binary_nonassocContextExt<'a>}

impl<'input> Tff_binary_nonassocContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_binary_nonassocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_binary_nonassocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_binary_nonassocContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_binary_nonassocContextExt<'input>>{

fn tff_unitary_formula_all(&self) ->  Vec<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_unitary_formula(&self, i: usize) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn binary_connective(&self) -> Option<Rc<Binary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_binary_nonassocContextAttrs<'input> for Tff_binary_nonassocContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_binary_nonassoc(&mut self,)
	-> Result<Rc<Tff_binary_nonassocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_binary_nonassocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_tff_binary_nonassoc);
        let mut _localctx: Rc<Tff_binary_nonassocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule tff_unitary_formula*/
			recog.base.set_state(774);
			recog.tff_unitary_formula()?;

			/*InvokeRule binary_connective*/
			recog.base.set_state(775);
			recog.binary_connective()?;

			/*InvokeRule tff_unitary_formula*/
			recog.base.set_state(776);
			recog.tff_unitary_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_binary_assoc ----------------
pub type Tff_binary_assocContextAll<'input> = Tff_binary_assocContext<'input>;


pub type Tff_binary_assocContext<'input> = BaseParserRuleContext<'input,Tff_binary_assocContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_binary_assocContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_binary_assocContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_binary_assocContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_binary_assoc(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_binary_assoc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_binary_assocContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_binary_assoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_binary_assocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_binary_assoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_binary_assoc }
}
antlr4rust::tid!{Tff_binary_assocContextExt<'a>}

impl<'input> Tff_binary_assocContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_binary_assocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_binary_assocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_binary_assocContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_binary_assocContextExt<'input>>{

fn tff_or_formula(&self) -> Option<Rc<Tff_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_and_formula(&self) -> Option<Rc<Tff_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_binary_assocContextAttrs<'input> for Tff_binary_assocContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_binary_assoc(&mut self,)
	-> Result<Rc<Tff_binary_assocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_binary_assocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_tff_binary_assoc);
        let mut _localctx: Rc<Tff_binary_assocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(780);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_or_formula*/
					recog.base.set_state(778);
					recog.tff_or_formula_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tff_and_formula*/
					recog.base.set_state(779);
					recog.tff_and_formula_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_or_formula ----------------
pub type Tff_or_formulaContextAll<'input> = Tff_or_formulaContext<'input>;


pub type Tff_or_formulaContext<'input> = BaseParserRuleContext<'input,Tff_or_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_or_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_or_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_or_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_or_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_or_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_or_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_or_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_or_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_or_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_or_formula }
}
antlr4rust::tid!{Tff_or_formulaContextExt<'a>}

impl<'input> Tff_or_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_or_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_or_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_or_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_or_formulaContextExt<'input>>{

fn tff_unitary_formula_all(&self) ->  Vec<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_unitary_formula(&self, i: usize) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Or
/// Returns `None` if there is no child corresponding to token Or
fn Or(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Or, 0)
}
fn tff_or_formula(&self) -> Option<Rc<Tff_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_or_formulaContextAttrs<'input> for Tff_or_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  tff_or_formula(&mut self,)
	-> Result<Rc<Tff_or_formulaContextAll<'input>>,ANTLRError> {
		self.tff_or_formula_rec(0)
	}

	fn tff_or_formula_rec(&mut self, _p: i32)
	-> Result<Rc<Tff_or_formulaContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Tff_or_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 108, RULE_tff_or_formula, _p);
	    let mut _localctx: Rc<Tff_or_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 108;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule tff_unitary_formula*/
			recog.base.set_state(783);
			recog.tff_unitary_formula()?;

			recog.base.set_state(784);
			recog.base.match_token(tptp_v7_0_0_0_Or,&mut recog.err_handler)?;

			/*InvokeRule tff_unitary_formula*/
			recog.base.set_state(785);
			recog.tff_unitary_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(792);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(38,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Tff_or_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_tff_or_formula);
					_localctx = tmp;
					recog.base.set_state(787);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(788);
					recog.base.match_token(tptp_v7_0_0_0_Or,&mut recog.err_handler)?;

					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(789);
					recog.tff_unitary_formula()?;

					}
					} 
				}
				recog.base.set_state(794);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(38,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- tff_and_formula ----------------
pub type Tff_and_formulaContextAll<'input> = Tff_and_formulaContext<'input>;


pub type Tff_and_formulaContext<'input> = BaseParserRuleContext<'input,Tff_and_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_and_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_and_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_and_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_and_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_and_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_and_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_and_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_and_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_and_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_and_formula }
}
antlr4rust::tid!{Tff_and_formulaContextExt<'a>}

impl<'input> Tff_and_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_and_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_and_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_and_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_and_formulaContextExt<'input>>{

fn tff_unitary_formula_all(&self) ->  Vec<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_unitary_formula(&self, i: usize) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token And
/// Returns `None` if there is no child corresponding to token And
fn And(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_And, 0)
}
fn tff_and_formula(&self) -> Option<Rc<Tff_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_and_formulaContextAttrs<'input> for Tff_and_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  tff_and_formula(&mut self,)
	-> Result<Rc<Tff_and_formulaContextAll<'input>>,ANTLRError> {
		self.tff_and_formula_rec(0)
	}

	fn tff_and_formula_rec(&mut self, _p: i32)
	-> Result<Rc<Tff_and_formulaContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Tff_and_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 110, RULE_tff_and_formula, _p);
	    let mut _localctx: Rc<Tff_and_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 110;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule tff_unitary_formula*/
			recog.base.set_state(796);
			recog.tff_unitary_formula()?;

			recog.base.set_state(797);
			recog.base.match_token(tptp_v7_0_0_0_And,&mut recog.err_handler)?;

			/*InvokeRule tff_unitary_formula*/
			recog.base.set_state(798);
			recog.tff_unitary_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(805);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(39,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Tff_and_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_tff_and_formula);
					_localctx = tmp;
					recog.base.set_state(800);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(801);
					recog.base.match_token(tptp_v7_0_0_0_And,&mut recog.err_handler)?;

					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(802);
					recog.tff_unitary_formula()?;

					}
					} 
				}
				recog.base.set_state(807);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(39,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- tff_unitary_formula ----------------
pub type Tff_unitary_formulaContextAll<'input> = Tff_unitary_formulaContext<'input>;


pub type Tff_unitary_formulaContext<'input> = BaseParserRuleContext<'input,Tff_unitary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_unitary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_unitary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_unitary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_unitary_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_unitary_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_unitary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_unitary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_unitary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_unitary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_unitary_formula }
}
antlr4rust::tid!{Tff_unitary_formulaContextExt<'a>}

impl<'input> Tff_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_unitary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_unitary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_unitary_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_unitary_formulaContextExt<'input>>{

fn tff_quantified_formula(&self) -> Option<Rc<Tff_quantified_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_unary_formula(&self) -> Option<Rc<Tff_unary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atomic_formula(&self) -> Option<Rc<Tff_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_conditional(&self) -> Option<Rc<Tff_conditionalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let(&self) -> Option<Rc<Tff_letContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_logic_formula(&self) -> Option<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_unitary_formulaContextAttrs<'input> for Tff_unitary_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_unitary_formula(&mut self,)
	-> Result<Rc<Tff_unitary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_unitary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_tff_unitary_formula);
        let mut _localctx: Rc<Tff_unitary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(817);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(40,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_quantified_formula*/
					recog.base.set_state(808);
					recog.tff_quantified_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tff_unary_formula*/
					recog.base.set_state(809);
					recog.tff_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule tff_atomic_formula*/
					recog.base.set_state(810);
					recog.tff_atomic_formula()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule tff_conditional*/
					recog.base.set_state(811);
					recog.tff_conditional()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule tff_let*/
					recog.base.set_state(812);
					recog.tff_let()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(813);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(814);
					recog.tff_logic_formula()?;

					recog.base.set_state(815);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_quantified_formula ----------------
pub type Tff_quantified_formulaContextAll<'input> = Tff_quantified_formulaContext<'input>;


pub type Tff_quantified_formulaContext<'input> = BaseParserRuleContext<'input,Tff_quantified_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_quantified_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_quantified_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_quantified_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_quantified_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_quantified_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_quantified_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_quantified_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_quantified_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_quantified_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_quantified_formula }
}
antlr4rust::tid!{Tff_quantified_formulaContextExt<'a>}

impl<'input> Tff_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_quantified_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_quantified_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_quantified_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_quantified_formulaContextExt<'input>>{

fn fof_quantifier(&self) -> Option<Rc<Fof_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_variable_list(&self) -> Option<Rc<Tff_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_unitary_formula(&self) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_quantified_formulaContextAttrs<'input> for Tff_quantified_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_quantified_formula(&mut self,)
	-> Result<Rc<Tff_quantified_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_quantified_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_tff_quantified_formula);
        let mut _localctx: Rc<Tff_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_quantifier*/
			recog.base.set_state(819);
			recog.fof_quantifier()?;

			recog.base.set_state(820);
			recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(821);
			recog.tff_variable_list()?;

			recog.base.set_state(822);
			recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

			recog.base.set_state(823);
			recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_unitary_formula*/
			recog.base.set_state(824);
			recog.tff_unitary_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_variable_list ----------------
pub type Tff_variable_listContextAll<'input> = Tff_variable_listContext<'input>;


pub type Tff_variable_listContext<'input> = BaseParserRuleContext<'input,Tff_variable_listContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_variable_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_variable_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_variable_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_variable_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_variable_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_variable_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_variable_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_variable_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_variable_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_variable_list }
}
antlr4rust::tid!{Tff_variable_listContextExt<'a>}

impl<'input> Tff_variable_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_variable_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_variable_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_variable_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_variable_listContextExt<'input>>{

fn tff_variable_all(&self) ->  Vec<Rc<Tff_variableContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_variable(&self, i: usize) -> Option<Rc<Tff_variableContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tff_variable_listContextAttrs<'input> for Tff_variable_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_variable_list(&mut self,)
	-> Result<Rc<Tff_variable_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_variable_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_tff_variable_list);
        let mut _localctx: Rc<Tff_variable_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule tff_variable*/
			recog.base.set_state(826);
			recog.tff_variable()?;

			recog.base.set_state(831);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(827);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule tff_variable*/
				recog.base.set_state(828);
				recog.tff_variable()?;

				}
				}
				recog.base.set_state(833);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_variable ----------------
pub type Tff_variableContextAll<'input> = Tff_variableContext<'input>;


pub type Tff_variableContext<'input> = BaseParserRuleContext<'input,Tff_variableContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_variableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_variableContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_variableContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_variable(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_variable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_variableContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_variableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_variable }
}
antlr4rust::tid!{Tff_variableContextExt<'a>}

impl<'input> Tff_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_variableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_variableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_variableContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_variableContextExt<'input>>{

fn tff_typed_variable(&self) -> Option<Rc<Tff_typed_variableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_variableContextAttrs<'input> for Tff_variableContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_variable(&mut self,)
	-> Result<Rc<Tff_variableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_variableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_tff_variable);
        let mut _localctx: Rc<Tff_variableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(836);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(42,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_typed_variable*/
					recog.base.set_state(834);
					recog.tff_typed_variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable*/
					recog.base.set_state(835);
					recog.variable()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_typed_variable ----------------
pub type Tff_typed_variableContextAll<'input> = Tff_typed_variableContext<'input>;


pub type Tff_typed_variableContext<'input> = BaseParserRuleContext<'input,Tff_typed_variableContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_typed_variableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_typed_variableContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_typed_variableContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_typed_variable(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_typed_variable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_typed_variableContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_typed_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_typed_variableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_typed_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_typed_variable }
}
antlr4rust::tid!{Tff_typed_variableContextExt<'a>}

impl<'input> Tff_typed_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_typed_variableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_typed_variableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_typed_variableContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_typed_variableContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_typed_variableContextAttrs<'input> for Tff_typed_variableContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_typed_variable(&mut self,)
	-> Result<Rc<Tff_typed_variableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_typed_variableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_tff_typed_variable);
        let mut _localctx: Rc<Tff_typed_variableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variable*/
			recog.base.set_state(838);
			recog.variable()?;

			recog.base.set_state(839);
			recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(840);
			recog.tff_atomic_type()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_unary_formula ----------------
pub type Tff_unary_formulaContextAll<'input> = Tff_unary_formulaContext<'input>;


pub type Tff_unary_formulaContext<'input> = BaseParserRuleContext<'input,Tff_unary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_unary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_unary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_unary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_unary_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_unary_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_unary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_unary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_unary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_unary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_unary_formula }
}
antlr4rust::tid!{Tff_unary_formulaContextExt<'a>}

impl<'input> Tff_unary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_unary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_unary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_unary_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_unary_formulaContextExt<'input>>{

fn unary_connective(&self) -> Option<Rc<Unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_unitary_formula(&self) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_infix_unary(&self) -> Option<Rc<Fof_infix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_unary_formulaContextAttrs<'input> for Tff_unary_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_unary_formula(&mut self,)
	-> Result<Rc<Tff_unary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_unary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_tff_unary_formula);
        let mut _localctx: Rc<Tff_unary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(846);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Not 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(842);
					recog.unary_connective()?;

					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(843);
					recog.tff_unitary_formula()?;

					}
				}

			tptp_v7_0_0_0_T__17 |tptp_v7_0_0_0_T__18 |tptp_v7_0_0_0_T__23 |tptp_v7_0_0_0_T__24 |
			tptp_v7_0_0_0_T__25 |tptp_v7_0_0_0_Real |tptp_v7_0_0_0_Rational |tptp_v7_0_0_0_Integer |
			tptp_v7_0_0_0_Dollar_word |tptp_v7_0_0_0_Dollar_dollar_word |tptp_v7_0_0_0_Upper_word |
			tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted |tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_infix_unary*/
					recog.base.set_state(845);
					recog.fof_infix_unary()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_atomic_formula ----------------
pub type Tff_atomic_formulaContextAll<'input> = Tff_atomic_formulaContext<'input>;


pub type Tff_atomic_formulaContext<'input> = BaseParserRuleContext<'input,Tff_atomic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_atomic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_atomic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_atomic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_atomic_formula }
}
antlr4rust::tid!{Tff_atomic_formulaContextExt<'a>}

impl<'input> Tff_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_atomic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_atomic_formulaContextExt<'input>>{

fn fof_atomic_formula(&self) -> Option<Rc<Fof_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_atomic_formulaContextAttrs<'input> for Tff_atomic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_atomic_formula(&mut self,)
	-> Result<Rc<Tff_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_tff_atomic_formula);
        let mut _localctx: Rc<Tff_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_atomic_formula*/
			recog.base.set_state(848);
			recog.fof_atomic_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_conditional ----------------
pub type Tff_conditionalContextAll<'input> = Tff_conditionalContext<'input>;


pub type Tff_conditionalContext<'input> = BaseParserRuleContext<'input,Tff_conditionalContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_conditionalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_conditionalContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_conditionalContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_conditional(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_conditional(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_conditionalContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_conditional(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_conditionalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_conditional }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_conditional }
}
antlr4rust::tid!{Tff_conditionalContextExt<'a>}

impl<'input> Tff_conditionalContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_conditionalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_conditionalContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_conditionalContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_conditionalContextExt<'input>>{

fn tff_logic_formula_all(&self) ->  Vec<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_logic_formula(&self, i: usize) -> Option<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tff_conditionalContextAttrs<'input> for Tff_conditionalContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_conditional(&mut self,)
	-> Result<Rc<Tff_conditionalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_conditionalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_tff_conditional);
        let mut _localctx: Rc<Tff_conditionalContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(850);
			recog.base.match_token(tptp_v7_0_0_0_T__20,&mut recog.err_handler)?;

			/*InvokeRule tff_logic_formula*/
			recog.base.set_state(851);
			recog.tff_logic_formula()?;

			recog.base.set_state(852);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_logic_formula*/
			recog.base.set_state(853);
			recog.tff_logic_formula()?;

			recog.base.set_state(854);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_logic_formula*/
			recog.base.set_state(855);
			recog.tff_logic_formula()?;

			recog.base.set_state(856);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let ----------------
pub type Tff_letContextAll<'input> = Tff_letContext<'input>;


pub type Tff_letContext<'input> = BaseParserRuleContext<'input,Tff_letContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_letContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_letContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_letContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_letContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_letContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let }
}
antlr4rust::tid!{Tff_letContextExt<'a>}

impl<'input> Tff_letContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_letContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_letContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_letContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_letContextExt<'input>>{

fn tff_let_term_defns(&self) -> Option<Rc<Tff_let_term_defnsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_formula(&self) -> Option<Rc<Tff_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_formula_defns(&self) -> Option<Rc<Tff_let_formula_defnsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_letContextAttrs<'input> for Tff_letContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let(&mut self,)
	-> Result<Rc<Tff_letContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_letContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_tff_let);
        let mut _localctx: Rc<Tff_letContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(870);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__21 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(858);
					recog.base.match_token(tptp_v7_0_0_0_T__21,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_defns*/
					recog.base.set_state(859);
					recog.tff_let_term_defns()?;

					recog.base.set_state(860);
					recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_formula*/
					recog.base.set_state(861);
					recog.tff_formula()?;

					recog.base.set_state(862);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__22 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(864);
					recog.base.match_token(tptp_v7_0_0_0_T__22,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_defns*/
					recog.base.set_state(865);
					recog.tff_let_formula_defns()?;

					recog.base.set_state(866);
					recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_formula*/
					recog.base.set_state(867);
					recog.tff_formula()?;

					recog.base.set_state(868);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let_term_defns ----------------
pub type Tff_let_term_defnsContextAll<'input> = Tff_let_term_defnsContext<'input>;


pub type Tff_let_term_defnsContext<'input> = BaseParserRuleContext<'input,Tff_let_term_defnsContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_let_term_defnsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_let_term_defnsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_let_term_defnsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let_term_defns(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let_term_defns(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_let_term_defnsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let_term_defns(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_let_term_defnsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let_term_defns }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let_term_defns }
}
antlr4rust::tid!{Tff_let_term_defnsContextExt<'a>}

impl<'input> Tff_let_term_defnsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_let_term_defnsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_let_term_defnsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_let_term_defnsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_let_term_defnsContextExt<'input>>{

fn tff_let_term_defn(&self) -> Option<Rc<Tff_let_term_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_term_list(&self) -> Option<Rc<Tff_let_term_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_let_term_defnsContextAttrs<'input> for Tff_let_term_defnsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let_term_defns(&mut self,)
	-> Result<Rc<Tff_let_term_defnsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_let_term_defnsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_tff_let_term_defns);
        let mut _localctx: Rc<Tff_let_term_defnsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(877);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__9 |tptp_v7_0_0_0_Forall |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_let_term_defn*/
					recog.base.set_state(872);
					recog.tff_let_term_defn()?;

					}
				}

			tptp_v7_0_0_0_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(873);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_list*/
					recog.base.set_state(874);
					recog.tff_let_term_list()?;

					recog.base.set_state(875);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let_term_list ----------------
pub type Tff_let_term_listContextAll<'input> = Tff_let_term_listContext<'input>;


pub type Tff_let_term_listContext<'input> = BaseParserRuleContext<'input,Tff_let_term_listContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_let_term_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_let_term_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_let_term_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let_term_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let_term_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_let_term_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let_term_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_let_term_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let_term_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let_term_list }
}
antlr4rust::tid!{Tff_let_term_listContextExt<'a>}

impl<'input> Tff_let_term_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_let_term_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_let_term_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_let_term_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_let_term_listContextExt<'input>>{

fn tff_let_term_defn_all(&self) ->  Vec<Rc<Tff_let_term_defnContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_let_term_defn(&self, i: usize) -> Option<Rc<Tff_let_term_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tff_let_term_listContextAttrs<'input> for Tff_let_term_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let_term_list(&mut self,)
	-> Result<Rc<Tff_let_term_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_let_term_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_tff_let_term_list);
        let mut _localctx: Rc<Tff_let_term_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule tff_let_term_defn*/
			recog.base.set_state(879);
			recog.tff_let_term_defn()?;

			recog.base.set_state(884);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(880);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule tff_let_term_defn*/
				recog.base.set_state(881);
				recog.tff_let_term_defn()?;

				}
				}
				recog.base.set_state(886);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let_term_defn ----------------
pub type Tff_let_term_defnContextAll<'input> = Tff_let_term_defnContext<'input>;


pub type Tff_let_term_defnContext<'input> = BaseParserRuleContext<'input,Tff_let_term_defnContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_let_term_defnContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_let_term_defnContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_let_term_defnContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let_term_defn(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let_term_defn(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_let_term_defnContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let_term_defn(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_let_term_defnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let_term_defn }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let_term_defn }
}
antlr4rust::tid!{Tff_let_term_defnContextExt<'a>}

impl<'input> Tff_let_term_defnContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_let_term_defnContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_let_term_defnContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_let_term_defnContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_let_term_defnContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Forall
/// Returns `None` if there is no child corresponding to token Forall
fn Forall(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Forall, 0)
}
fn tff_variable_list(&self) -> Option<Rc<Tff_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_term_defn(&self) -> Option<Rc<Tff_let_term_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_term_binding(&self) -> Option<Rc<Tff_let_term_bindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_let_term_defnContextAttrs<'input> for Tff_let_term_defnContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let_term_defn(&mut self,)
	-> Result<Rc<Tff_let_term_defnContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_let_term_defnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_tff_let_term_defn);
        let mut _localctx: Rc<Tff_let_term_defnContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(895);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Forall 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(887);
					recog.base.match_token(tptp_v7_0_0_0_Forall,&mut recog.err_handler)?;

					recog.base.set_state(888);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_variable_list*/
					recog.base.set_state(889);
					recog.tff_variable_list()?;

					recog.base.set_state(890);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					recog.base.set_state(891);
					recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_defn*/
					recog.base.set_state(892);
					recog.tff_let_term_defn()?;

					}
				}

			tptp_v7_0_0_0_T__9 |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tff_let_term_binding*/
					recog.base.set_state(894);
					recog.tff_let_term_binding()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let_term_binding ----------------
pub type Tff_let_term_bindingContextAll<'input> = Tff_let_term_bindingContext<'input>;


pub type Tff_let_term_bindingContext<'input> = BaseParserRuleContext<'input,Tff_let_term_bindingContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_let_term_bindingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_let_term_bindingContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_let_term_bindingContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let_term_binding(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let_term_binding(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_let_term_bindingContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let_term_binding(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_let_term_bindingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let_term_binding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let_term_binding }
}
antlr4rust::tid!{Tff_let_term_bindingContextExt<'a>}

impl<'input> Tff_let_term_bindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_let_term_bindingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_let_term_bindingContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_let_term_bindingContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_let_term_bindingContextExt<'input>>{

fn fof_plain_term(&self) -> Option<Rc<Fof_plain_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Infix_equality
/// Returns `None` if there is no child corresponding to token Infix_equality
fn Infix_equality(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Infix_equality, 0)
}
fn fof_term(&self) -> Option<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_term_binding(&self) -> Option<Rc<Tff_let_term_bindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_let_term_bindingContextAttrs<'input> for Tff_let_term_bindingContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let_term_binding(&mut self,)
	-> Result<Rc<Tff_let_term_bindingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_let_term_bindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_tff_let_term_binding);
        let mut _localctx: Rc<Tff_let_term_bindingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(905);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_plain_term*/
					recog.base.set_state(897);
					recog.fof_plain_term()?;

					recog.base.set_state(898);
					recog.base.match_token(tptp_v7_0_0_0_Infix_equality,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(899);
					recog.fof_term()?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(901);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_binding*/
					recog.base.set_state(902);
					recog.tff_let_term_binding()?;

					recog.base.set_state(903);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let_formula_defns ----------------
pub type Tff_let_formula_defnsContextAll<'input> = Tff_let_formula_defnsContext<'input>;


pub type Tff_let_formula_defnsContext<'input> = BaseParserRuleContext<'input,Tff_let_formula_defnsContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_let_formula_defnsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_let_formula_defnsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_let_formula_defnsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let_formula_defns(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let_formula_defns(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_let_formula_defnsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let_formula_defns(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_let_formula_defnsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let_formula_defns }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let_formula_defns }
}
antlr4rust::tid!{Tff_let_formula_defnsContextExt<'a>}

impl<'input> Tff_let_formula_defnsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_let_formula_defnsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_let_formula_defnsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_let_formula_defnsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_let_formula_defnsContextExt<'input>>{

fn tff_let_formula_defn(&self) -> Option<Rc<Tff_let_formula_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_formula_list(&self) -> Option<Rc<Tff_let_formula_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_let_formula_defnsContextAttrs<'input> for Tff_let_formula_defnsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let_formula_defns(&mut self,)
	-> Result<Rc<Tff_let_formula_defnsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_let_formula_defnsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_tff_let_formula_defns);
        let mut _localctx: Rc<Tff_let_formula_defnsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(912);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__9 |tptp_v7_0_0_0_Forall |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_let_formula_defn*/
					recog.base.set_state(907);
					recog.tff_let_formula_defn()?;

					}
				}

			tptp_v7_0_0_0_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(908);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_list*/
					recog.base.set_state(909);
					recog.tff_let_formula_list()?;

					recog.base.set_state(910);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let_formula_list ----------------
pub type Tff_let_formula_listContextAll<'input> = Tff_let_formula_listContext<'input>;


pub type Tff_let_formula_listContext<'input> = BaseParserRuleContext<'input,Tff_let_formula_listContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_let_formula_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_let_formula_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_let_formula_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let_formula_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let_formula_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_let_formula_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let_formula_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_let_formula_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let_formula_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let_formula_list }
}
antlr4rust::tid!{Tff_let_formula_listContextExt<'a>}

impl<'input> Tff_let_formula_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_let_formula_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_let_formula_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_let_formula_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_let_formula_listContextExt<'input>>{

fn tff_let_formula_defn_all(&self) ->  Vec<Rc<Tff_let_formula_defnContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_let_formula_defn(&self, i: usize) -> Option<Rc<Tff_let_formula_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tff_let_formula_listContextAttrs<'input> for Tff_let_formula_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let_formula_list(&mut self,)
	-> Result<Rc<Tff_let_formula_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_let_formula_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_tff_let_formula_list);
        let mut _localctx: Rc<Tff_let_formula_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule tff_let_formula_defn*/
			recog.base.set_state(914);
			recog.tff_let_formula_defn()?;

			recog.base.set_state(919);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(915);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule tff_let_formula_defn*/
				recog.base.set_state(916);
				recog.tff_let_formula_defn()?;

				}
				}
				recog.base.set_state(921);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let_formula_defn ----------------
pub type Tff_let_formula_defnContextAll<'input> = Tff_let_formula_defnContext<'input>;


pub type Tff_let_formula_defnContext<'input> = BaseParserRuleContext<'input,Tff_let_formula_defnContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_let_formula_defnContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_let_formula_defnContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_let_formula_defnContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let_formula_defn(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let_formula_defn(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_let_formula_defnContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let_formula_defn(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_let_formula_defnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let_formula_defn }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let_formula_defn }
}
antlr4rust::tid!{Tff_let_formula_defnContextExt<'a>}

impl<'input> Tff_let_formula_defnContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_let_formula_defnContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_let_formula_defnContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_let_formula_defnContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_let_formula_defnContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Forall
/// Returns `None` if there is no child corresponding to token Forall
fn Forall(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Forall, 0)
}
fn tff_variable_list(&self) -> Option<Rc<Tff_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_formula_defn(&self) -> Option<Rc<Tff_let_formula_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_formula_binding(&self) -> Option<Rc<Tff_let_formula_bindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_let_formula_defnContextAttrs<'input> for Tff_let_formula_defnContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let_formula_defn(&mut self,)
	-> Result<Rc<Tff_let_formula_defnContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_let_formula_defnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_tff_let_formula_defn);
        let mut _localctx: Rc<Tff_let_formula_defnContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(930);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Forall 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(922);
					recog.base.match_token(tptp_v7_0_0_0_Forall,&mut recog.err_handler)?;

					recog.base.set_state(923);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_variable_list*/
					recog.base.set_state(924);
					recog.tff_variable_list()?;

					recog.base.set_state(925);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					recog.base.set_state(926);
					recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_defn*/
					recog.base.set_state(927);
					recog.tff_let_formula_defn()?;

					}
				}

			tptp_v7_0_0_0_T__9 |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tff_let_formula_binding*/
					recog.base.set_state(929);
					recog.tff_let_formula_binding()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let_formula_binding ----------------
pub type Tff_let_formula_bindingContextAll<'input> = Tff_let_formula_bindingContext<'input>;


pub type Tff_let_formula_bindingContext<'input> = BaseParserRuleContext<'input,Tff_let_formula_bindingContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_let_formula_bindingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_let_formula_bindingContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_let_formula_bindingContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let_formula_binding(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let_formula_binding(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_let_formula_bindingContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let_formula_binding(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_let_formula_bindingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let_formula_binding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let_formula_binding }
}
antlr4rust::tid!{Tff_let_formula_bindingContextExt<'a>}

impl<'input> Tff_let_formula_bindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_let_formula_bindingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_let_formula_bindingContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_let_formula_bindingContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_let_formula_bindingContextExt<'input>>{

fn fof_plain_atomic_formula(&self) -> Option<Rc<Fof_plain_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Iff
/// Returns `None` if there is no child corresponding to token Iff
fn Iff(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Iff, 0)
}
fn tff_unitary_formula(&self) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_formula_binding(&self) -> Option<Rc<Tff_let_formula_bindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_let_formula_bindingContextAttrs<'input> for Tff_let_formula_bindingContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let_formula_binding(&mut self,)
	-> Result<Rc<Tff_let_formula_bindingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_let_formula_bindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_tff_let_formula_binding);
        let mut _localctx: Rc<Tff_let_formula_bindingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(940);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_plain_atomic_formula*/
					recog.base.set_state(932);
					recog.fof_plain_atomic_formula()?;

					recog.base.set_state(933);
					recog.base.match_token(tptp_v7_0_0_0_Iff,&mut recog.err_handler)?;

					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(934);
					recog.tff_unitary_formula()?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(936);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_binding*/
					recog.base.set_state(937);
					recog.tff_let_formula_binding()?;

					recog.base.set_state(938);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_sequent ----------------
pub type Tff_sequentContextAll<'input> = Tff_sequentContext<'input>;


pub type Tff_sequentContext<'input> = BaseParserRuleContext<'input,Tff_sequentContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_sequentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_sequentContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_sequentContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_sequent(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_sequent(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_sequentContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_sequent(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_sequentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_sequent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_sequent }
}
antlr4rust::tid!{Tff_sequentContextExt<'a>}

impl<'input> Tff_sequentContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_sequentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_sequentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_sequentContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_sequentContextExt<'input>>{

fn tff_formula_tuple_all(&self) ->  Vec<Rc<Tff_formula_tupleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_formula_tuple(&self, i: usize) -> Option<Rc<Tff_formula_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Gentzen_arrow
/// Returns `None` if there is no child corresponding to token Gentzen_arrow
fn Gentzen_arrow(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Gentzen_arrow, 0)
}
fn tff_sequent(&self) -> Option<Rc<Tff_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_sequentContextAttrs<'input> for Tff_sequentContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_sequent(&mut self,)
	-> Result<Rc<Tff_sequentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_sequentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_tff_sequent);
        let mut _localctx: Rc<Tff_sequentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(950);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__11 |tptp_v7_0_0_0_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_formula_tuple*/
					recog.base.set_state(942);
					recog.tff_formula_tuple()?;

					recog.base.set_state(943);
					recog.base.match_token(tptp_v7_0_0_0_Gentzen_arrow,&mut recog.err_handler)?;

					/*InvokeRule tff_formula_tuple*/
					recog.base.set_state(944);
					recog.tff_formula_tuple()?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(946);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_sequent*/
					recog.base.set_state(947);
					recog.tff_sequent()?;

					recog.base.set_state(948);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_formula_tuple ----------------
pub type Tff_formula_tupleContextAll<'input> = Tff_formula_tupleContext<'input>;


pub type Tff_formula_tupleContext<'input> = BaseParserRuleContext<'input,Tff_formula_tupleContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_formula_tupleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_formula_tupleContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_formula_tupleContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_formula_tuple(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_formula_tuple(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_formula_tupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_formula_tuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_formula_tupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_formula_tuple }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_formula_tuple }
}
antlr4rust::tid!{Tff_formula_tupleContextExt<'a>}

impl<'input> Tff_formula_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_formula_tupleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_formula_tupleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_formula_tupleContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_formula_tupleContextExt<'input>>{

fn tff_formula_tuple_list(&self) -> Option<Rc<Tff_formula_tuple_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_formula_tupleContextAttrs<'input> for Tff_formula_tupleContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_formula_tuple(&mut self,)
	-> Result<Rc<Tff_formula_tupleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_formula_tupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_tff_formula_tuple);
        let mut _localctx: Rc<Tff_formula_tupleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(957);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(952);
					recog.base.match_token(tptp_v7_0_0_0_T__16,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(953);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_formula_tuple_list*/
					recog.base.set_state(954);
					recog.tff_formula_tuple_list()?;

					recog.base.set_state(955);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_formula_tuple_list ----------------
pub type Tff_formula_tuple_listContextAll<'input> = Tff_formula_tuple_listContext<'input>;


pub type Tff_formula_tuple_listContext<'input> = BaseParserRuleContext<'input,Tff_formula_tuple_listContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_formula_tuple_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_formula_tuple_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_formula_tuple_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_formula_tuple_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_formula_tuple_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_formula_tuple_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_formula_tuple_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_formula_tuple_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_formula_tuple_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_formula_tuple_list }
}
antlr4rust::tid!{Tff_formula_tuple_listContextExt<'a>}

impl<'input> Tff_formula_tuple_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_formula_tuple_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_formula_tuple_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_formula_tuple_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_formula_tuple_listContextExt<'input>>{

fn tff_logic_formula_all(&self) ->  Vec<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_logic_formula(&self, i: usize) -> Option<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tff_formula_tuple_listContextAttrs<'input> for Tff_formula_tuple_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_formula_tuple_list(&mut self,)
	-> Result<Rc<Tff_formula_tuple_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_formula_tuple_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_tff_formula_tuple_list);
        let mut _localctx: Rc<Tff_formula_tuple_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule tff_logic_formula*/
			recog.base.set_state(959);
			recog.tff_logic_formula()?;

			recog.base.set_state(964);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(960);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule tff_logic_formula*/
				recog.base.set_state(961);
				recog.tff_logic_formula()?;

				}
				}
				recog.base.set_state(966);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_typed_atom ----------------
pub type Tff_typed_atomContextAll<'input> = Tff_typed_atomContext<'input>;


pub type Tff_typed_atomContext<'input> = BaseParserRuleContext<'input,Tff_typed_atomContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_typed_atomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_typed_atomContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_typed_atomContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_typed_atom(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_typed_atom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_typed_atomContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_typed_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_typed_atomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_typed_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_typed_atom }
}
antlr4rust::tid!{Tff_typed_atomContextExt<'a>}

impl<'input> Tff_typed_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_typed_atomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_typed_atomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_typed_atomContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_typed_atomContextExt<'input>>{

fn untyped_atom(&self) -> Option<Rc<Untyped_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_top_level_type(&self) -> Option<Rc<Tff_top_level_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_typed_atom(&self) -> Option<Rc<Tff_typed_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_typed_atomContextAttrs<'input> for Tff_typed_atomContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_typed_atom(&mut self,)
	-> Result<Rc<Tff_typed_atomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_typed_atomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_tff_typed_atom);
        let mut _localctx: Rc<Tff_typed_atomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(975);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Dollar_dollar_word |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule untyped_atom*/
					recog.base.set_state(967);
					recog.untyped_atom()?;

					recog.base.set_state(968);
					recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(969);
					recog.tff_top_level_type()?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(971);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_typed_atom*/
					recog.base.set_state(972);
					recog.tff_typed_atom()?;

					recog.base.set_state(973);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_subtype ----------------
pub type Tff_subtypeContextAll<'input> = Tff_subtypeContext<'input>;


pub type Tff_subtypeContext<'input> = BaseParserRuleContext<'input,Tff_subtypeContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_subtypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_subtypeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_subtypeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_subtype(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_subtype(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_subtypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_subtype(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_subtypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_subtype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_subtype }
}
antlr4rust::tid!{Tff_subtypeContextExt<'a>}

impl<'input> Tff_subtypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_subtypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_subtypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_subtypeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_subtypeContextExt<'input>>{

fn untyped_atom(&self) -> Option<Rc<Untyped_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Subtype_sign
/// Returns `None` if there is no child corresponding to token Subtype_sign
fn Subtype_sign(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Subtype_sign, 0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_subtypeContextAttrs<'input> for Tff_subtypeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_subtype(&mut self,)
	-> Result<Rc<Tff_subtypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_subtypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_tff_subtype);
        let mut _localctx: Rc<Tff_subtypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule untyped_atom*/
			recog.base.set_state(977);
			recog.untyped_atom()?;

			recog.base.set_state(978);
			recog.base.match_token(tptp_v7_0_0_0_Subtype_sign,&mut recog.err_handler)?;

			/*InvokeRule atom*/
			recog.base.set_state(979);
			recog.atom()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_top_level_type ----------------
pub type Tff_top_level_typeContextAll<'input> = Tff_top_level_typeContext<'input>;


pub type Tff_top_level_typeContext<'input> = BaseParserRuleContext<'input,Tff_top_level_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_top_level_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_top_level_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_top_level_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_top_level_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_top_level_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_top_level_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_top_level_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_top_level_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_top_level_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_top_level_type }
}
antlr4rust::tid!{Tff_top_level_typeContextExt<'a>}

impl<'input> Tff_top_level_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_top_level_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_top_level_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_top_level_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_top_level_typeContextExt<'input>>{

fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_mapping_type(&self) -> Option<Rc<Tff_mapping_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tf1_quantified_type(&self) -> Option<Rc<Tf1_quantified_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_top_level_type(&self) -> Option<Rc<Tff_top_level_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_top_level_typeContextAttrs<'input> for Tff_top_level_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_top_level_type(&mut self,)
	-> Result<Rc<Tff_top_level_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_top_level_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_tff_top_level_type);
        let mut _localctx: Rc<Tff_top_level_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(988);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(57,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(981);
					recog.tff_atomic_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tff_mapping_type*/
					recog.base.set_state(982);
					recog.tff_mapping_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule tf1_quantified_type*/
					recog.base.set_state(983);
					recog.tf1_quantified_type()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(984);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(985);
					recog.tff_top_level_type()?;

					recog.base.set_state(986);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tf1_quantified_type ----------------
pub type Tf1_quantified_typeContextAll<'input> = Tf1_quantified_typeContext<'input>;


pub type Tf1_quantified_typeContext<'input> = BaseParserRuleContext<'input,Tf1_quantified_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Tf1_quantified_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tf1_quantified_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tf1_quantified_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tf1_quantified_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tf1_quantified_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tf1_quantified_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tf1_quantified_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tf1_quantified_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tf1_quantified_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tf1_quantified_type }
}
antlr4rust::tid!{Tf1_quantified_typeContextExt<'a>}

impl<'input> Tf1_quantified_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tf1_quantified_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tf1_quantified_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tf1_quantified_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tf1_quantified_typeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TyForall
/// Returns `None` if there is no child corresponding to token TyForall
fn TyForall(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_TyForall, 0)
}
fn tff_variable_list(&self) -> Option<Rc<Tff_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_monotype(&self) -> Option<Rc<Tff_monotypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tf1_quantified_typeContextAttrs<'input> for Tf1_quantified_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tf1_quantified_type(&mut self,)
	-> Result<Rc<Tf1_quantified_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tf1_quantified_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_tf1_quantified_type);
        let mut _localctx: Rc<Tf1_quantified_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(990);
			recog.base.match_token(tptp_v7_0_0_0_TyForall,&mut recog.err_handler)?;

			recog.base.set_state(991);
			recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(992);
			recog.tff_variable_list()?;

			recog.base.set_state(993);
			recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

			recog.base.set_state(994);
			recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_monotype*/
			recog.base.set_state(995);
			recog.tff_monotype()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_monotype ----------------
pub type Tff_monotypeContextAll<'input> = Tff_monotypeContext<'input>;


pub type Tff_monotypeContext<'input> = BaseParserRuleContext<'input,Tff_monotypeContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_monotypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_monotypeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_monotypeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_monotype(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_monotype(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_monotypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_monotype(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_monotypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_monotype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_monotype }
}
antlr4rust::tid!{Tff_monotypeContextExt<'a>}

impl<'input> Tff_monotypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_monotypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_monotypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_monotypeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_monotypeContextExt<'input>>{

fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_mapping_type(&self) -> Option<Rc<Tff_mapping_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_monotypeContextAttrs<'input> for Tff_monotypeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_monotype(&mut self,)
	-> Result<Rc<Tff_monotypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_monotypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_tff_monotype);
        let mut _localctx: Rc<Tff_monotypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1002);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Dollar_word |tptp_v7_0_0_0_Upper_word |tptp_v7_0_0_0_Lower_word |
			tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(997);
					recog.tff_atomic_type()?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(998);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_mapping_type*/
					recog.base.set_state(999);
					recog.tff_mapping_type()?;

					recog.base.set_state(1000);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_unitary_type ----------------
pub type Tff_unitary_typeContextAll<'input> = Tff_unitary_typeContext<'input>;


pub type Tff_unitary_typeContext<'input> = BaseParserRuleContext<'input,Tff_unitary_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_unitary_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_unitary_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_unitary_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_unitary_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_unitary_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_unitary_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_unitary_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_unitary_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_unitary_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_unitary_type }
}
antlr4rust::tid!{Tff_unitary_typeContextExt<'a>}

impl<'input> Tff_unitary_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_unitary_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_unitary_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_unitary_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_unitary_typeContextExt<'input>>{

fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_xprod_type(&self) -> Option<Rc<Tff_xprod_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_unitary_typeContextAttrs<'input> for Tff_unitary_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_unitary_type(&mut self,)
	-> Result<Rc<Tff_unitary_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_unitary_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_tff_unitary_type);
        let mut _localctx: Rc<Tff_unitary_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1009);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Dollar_word |tptp_v7_0_0_0_Upper_word |tptp_v7_0_0_0_Lower_word |
			tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1004);
					recog.tff_atomic_type()?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1005);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_xprod_type*/
					recog.base.set_state(1006);
					recog.tff_xprod_type_rec(0)?;

					recog.base.set_state(1007);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_atomic_type ----------------
pub type Tff_atomic_typeContextAll<'input> = Tff_atomic_typeContext<'input>;


pub type Tff_atomic_typeContext<'input> = BaseParserRuleContext<'input,Tff_atomic_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_atomic_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_atomic_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_atomic_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_atomic_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_atomic_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_atomic_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_atomic_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_atomic_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_atomic_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_atomic_type }
}
antlr4rust::tid!{Tff_atomic_typeContextExt<'a>}

impl<'input> Tff_atomic_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_atomic_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_atomic_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_atomic_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_atomic_typeContextExt<'input>>{

fn type_constant(&self) -> Option<Rc<Type_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_type(&self) -> Option<Rc<Defined_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type_functor(&self) -> Option<Rc<Type_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_type_arguments(&self) -> Option<Rc<Tff_type_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_atomic_typeContextAttrs<'input> for Tff_atomic_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_atomic_type(&mut self,)
	-> Result<Rc<Tff_atomic_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_atomic_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_tff_atomic_type);
        let mut _localctx: Rc<Tff_atomic_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1019);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(60,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule type_constant*/
					recog.base.set_state(1011);
					recog.type_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule defined_type*/
					recog.base.set_state(1012);
					recog.defined_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule type_functor*/
					recog.base.set_state(1013);
					recog.type_functor()?;

					recog.base.set_state(1014);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_type_arguments*/
					recog.base.set_state(1015);
					recog.tff_type_arguments()?;

					recog.base.set_state(1016);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule variable*/
					recog.base.set_state(1018);
					recog.variable()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_type_arguments ----------------
pub type Tff_type_argumentsContextAll<'input> = Tff_type_argumentsContext<'input>;


pub type Tff_type_argumentsContext<'input> = BaseParserRuleContext<'input,Tff_type_argumentsContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_type_argumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_type_argumentsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_type_argumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_type_arguments(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_type_arguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_type_argumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_type_arguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_type_argumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_type_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_type_arguments }
}
antlr4rust::tid!{Tff_type_argumentsContextExt<'a>}

impl<'input> Tff_type_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_type_argumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_type_argumentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_type_argumentsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_type_argumentsContextExt<'input>>{

fn tff_atomic_type_all(&self) ->  Vec<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_atomic_type(&self, i: usize) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tff_type_argumentsContextAttrs<'input> for Tff_type_argumentsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_type_arguments(&mut self,)
	-> Result<Rc<Tff_type_argumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_type_argumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_tff_type_arguments);
        let mut _localctx: Rc<Tff_type_argumentsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(1021);
			recog.tff_atomic_type()?;

			recog.base.set_state(1026);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1022);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule tff_atomic_type*/
				recog.base.set_state(1023);
				recog.tff_atomic_type()?;

				}
				}
				recog.base.set_state(1028);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_mapping_type ----------------
pub type Tff_mapping_typeContextAll<'input> = Tff_mapping_typeContext<'input>;


pub type Tff_mapping_typeContext<'input> = BaseParserRuleContext<'input,Tff_mapping_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_mapping_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_mapping_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_mapping_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_mapping_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_mapping_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_mapping_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_mapping_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_mapping_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_mapping_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_mapping_type }
}
antlr4rust::tid!{Tff_mapping_typeContextExt<'a>}

impl<'input> Tff_mapping_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_mapping_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_mapping_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_mapping_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_mapping_typeContextExt<'input>>{

fn tff_unitary_type(&self) -> Option<Rc<Tff_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Arrow
/// Returns `None` if there is no child corresponding to token Arrow
fn Arrow(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Arrow, 0)
}
fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_mapping_typeContextAttrs<'input> for Tff_mapping_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_mapping_type(&mut self,)
	-> Result<Rc<Tff_mapping_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_mapping_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_tff_mapping_type);
        let mut _localctx: Rc<Tff_mapping_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule tff_unitary_type*/
			recog.base.set_state(1029);
			recog.tff_unitary_type()?;

			recog.base.set_state(1030);
			recog.base.match_token(tptp_v7_0_0_0_Arrow,&mut recog.err_handler)?;

			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(1031);
			recog.tff_atomic_type()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_xprod_type ----------------
pub type Tff_xprod_typeContextAll<'input> = Tff_xprod_typeContext<'input>;


pub type Tff_xprod_typeContext<'input> = BaseParserRuleContext<'input,Tff_xprod_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_xprod_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_xprod_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_xprod_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_xprod_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_xprod_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_xprod_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_xprod_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_xprod_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_xprod_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_xprod_type }
}
antlr4rust::tid!{Tff_xprod_typeContextExt<'a>}

impl<'input> Tff_xprod_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_xprod_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_xprod_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_xprod_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_xprod_typeContextExt<'input>>{

fn tff_unitary_type(&self) -> Option<Rc<Tff_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Star, 0)
}
fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_xprod_type(&self) -> Option<Rc<Tff_xprod_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_xprod_typeContextAttrs<'input> for Tff_xprod_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  tff_xprod_type(&mut self,)
	-> Result<Rc<Tff_xprod_typeContextAll<'input>>,ANTLRError> {
		self.tff_xprod_type_rec(0)
	}

	fn tff_xprod_type_rec(&mut self, _p: i32)
	-> Result<Rc<Tff_xprod_typeContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Tff_xprod_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 170, RULE_tff_xprod_type, _p);
	    let mut _localctx: Rc<Tff_xprod_typeContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 170;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule tff_unitary_type*/
			recog.base.set_state(1034);
			recog.tff_unitary_type()?;

			recog.base.set_state(1035);
			recog.base.match_token(tptp_v7_0_0_0_Star,&mut recog.err_handler)?;

			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(1036);
			recog.tff_atomic_type()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1043);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Tff_xprod_typeContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_tff_xprod_type);
					_localctx = tmp;
					recog.base.set_state(1038);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1039);
					recog.base.match_token(tptp_v7_0_0_0_Star,&mut recog.err_handler)?;

					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1040);
					recog.tff_atomic_type()?;

					}
					} 
				}
				recog.base.set_state(1045);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- tcf_formula ----------------
pub type Tcf_formulaContextAll<'input> = Tcf_formulaContext<'input>;


pub type Tcf_formulaContext<'input> = BaseParserRuleContext<'input,Tcf_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tcf_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tcf_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tcf_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tcf_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tcf_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tcf_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tcf_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tcf_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tcf_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tcf_formula }
}
antlr4rust::tid!{Tcf_formulaContextExt<'a>}

impl<'input> Tcf_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tcf_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tcf_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tcf_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tcf_formulaContextExt<'input>>{

fn tcf_logic_formula(&self) -> Option<Rc<Tcf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_typed_atom(&self) -> Option<Rc<Tff_typed_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tcf_formulaContextAttrs<'input> for Tcf_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tcf_formula(&mut self,)
	-> Result<Rc<Tcf_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tcf_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_tcf_formula);
        let mut _localctx: Rc<Tcf_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1048);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(63,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tcf_logic_formula*/
					recog.base.set_state(1046);
					recog.tcf_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tff_typed_atom*/
					recog.base.set_state(1047);
					recog.tff_typed_atom()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tcf_logic_formula ----------------
pub type Tcf_logic_formulaContextAll<'input> = Tcf_logic_formulaContext<'input>;


pub type Tcf_logic_formulaContext<'input> = BaseParserRuleContext<'input,Tcf_logic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tcf_logic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tcf_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tcf_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tcf_logic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tcf_logic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tcf_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tcf_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tcf_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tcf_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tcf_logic_formula }
}
antlr4rust::tid!{Tcf_logic_formulaContextExt<'a>}

impl<'input> Tcf_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tcf_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tcf_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tcf_logic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tcf_logic_formulaContextExt<'input>>{

fn tcf_quantified_formula(&self) -> Option<Rc<Tcf_quantified_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cnf_formula(&self) -> Option<Rc<Cnf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tcf_logic_formulaContextAttrs<'input> for Tcf_logic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tcf_logic_formula(&mut self,)
	-> Result<Rc<Tcf_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tcf_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_tcf_logic_formula);
        let mut _localctx: Rc<Tcf_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1052);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Forall 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tcf_quantified_formula*/
					recog.base.set_state(1050);
					recog.tcf_quantified_formula()?;

					}
				}

			tptp_v7_0_0_0_T__9 |tptp_v7_0_0_0_T__17 |tptp_v7_0_0_0_T__18 |tptp_v7_0_0_0_T__23 |
			tptp_v7_0_0_0_T__24 |tptp_v7_0_0_0_T__25 |tptp_v7_0_0_0_Not |tptp_v7_0_0_0_Real |
			tptp_v7_0_0_0_Rational |tptp_v7_0_0_0_Integer |tptp_v7_0_0_0_Dollar_word |
			tptp_v7_0_0_0_Dollar_dollar_word |tptp_v7_0_0_0_Upper_word |tptp_v7_0_0_0_Lower_word |
			tptp_v7_0_0_0_Single_quoted |tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule cnf_formula*/
					recog.base.set_state(1051);
					recog.cnf_formula()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tcf_quantified_formula ----------------
pub type Tcf_quantified_formulaContextAll<'input> = Tcf_quantified_formulaContext<'input>;


pub type Tcf_quantified_formulaContext<'input> = BaseParserRuleContext<'input,Tcf_quantified_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tcf_quantified_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tcf_quantified_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tcf_quantified_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tcf_quantified_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tcf_quantified_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tcf_quantified_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tcf_quantified_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tcf_quantified_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tcf_quantified_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tcf_quantified_formula }
}
antlr4rust::tid!{Tcf_quantified_formulaContextExt<'a>}

impl<'input> Tcf_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tcf_quantified_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tcf_quantified_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tcf_quantified_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tcf_quantified_formulaContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Forall
/// Returns `None` if there is no child corresponding to token Forall
fn Forall(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Forall, 0)
}
fn tff_variable_list(&self) -> Option<Rc<Tff_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cnf_formula(&self) -> Option<Rc<Cnf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tcf_quantified_formulaContextAttrs<'input> for Tcf_quantified_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tcf_quantified_formula(&mut self,)
	-> Result<Rc<Tcf_quantified_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tcf_quantified_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_tcf_quantified_formula);
        let mut _localctx: Rc<Tcf_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1054);
			recog.base.match_token(tptp_v7_0_0_0_Forall,&mut recog.err_handler)?;

			recog.base.set_state(1055);
			recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(1056);
			recog.tff_variable_list()?;

			recog.base.set_state(1057);
			recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

			recog.base.set_state(1058);
			recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

			/*InvokeRule cnf_formula*/
			recog.base.set_state(1059);
			recog.cnf_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_formula ----------------
pub type Fof_formulaContextAll<'input> = Fof_formulaContext<'input>;


pub type Fof_formulaContext<'input> = BaseParserRuleContext<'input,Fof_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_formula }
}
antlr4rust::tid!{Fof_formulaContextExt<'a>}

impl<'input> Fof_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_formulaContextExt<'input>>{

fn fof_logic_formula(&self) -> Option<Rc<Fof_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_sequent(&self) -> Option<Rc<Fof_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_formulaContextAttrs<'input> for Fof_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_formula(&mut self,)
	-> Result<Rc<Fof_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_fof_formula);
        let mut _localctx: Rc<Fof_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1063);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(65,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_logic_formula*/
					recog.base.set_state(1061);
					recog.fof_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_sequent*/
					recog.base.set_state(1062);
					recog.fof_sequent()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_logic_formula ----------------
pub type Fof_logic_formulaContextAll<'input> = Fof_logic_formulaContext<'input>;


pub type Fof_logic_formulaContext<'input> = BaseParserRuleContext<'input,Fof_logic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_logic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_logic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_logic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_logic_formula }
}
antlr4rust::tid!{Fof_logic_formulaContextExt<'a>}

impl<'input> Fof_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_logic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_logic_formulaContextExt<'input>>{

fn fof_binary_formula(&self) -> Option<Rc<Fof_binary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_unitary_formula(&self) -> Option<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_logic_formulaContextAttrs<'input> for Fof_logic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_logic_formula(&mut self,)
	-> Result<Rc<Fof_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_fof_logic_formula);
        let mut _localctx: Rc<Fof_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1067);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(66,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_binary_formula*/
					recog.base.set_state(1065);
					recog.fof_binary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_unitary_formula*/
					recog.base.set_state(1066);
					recog.fof_unitary_formula()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_binary_formula ----------------
pub type Fof_binary_formulaContextAll<'input> = Fof_binary_formulaContext<'input>;


pub type Fof_binary_formulaContext<'input> = BaseParserRuleContext<'input,Fof_binary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_binary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_binary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_binary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_binary_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_binary_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_binary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_binary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_binary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_binary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_binary_formula }
}
antlr4rust::tid!{Fof_binary_formulaContextExt<'a>}

impl<'input> Fof_binary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_binary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_binary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_binary_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_binary_formulaContextExt<'input>>{

fn fof_binary_nonassoc(&self) -> Option<Rc<Fof_binary_nonassocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_binary_assoc(&self) -> Option<Rc<Fof_binary_assocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_binary_formulaContextAttrs<'input> for Fof_binary_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_binary_formula(&mut self,)
	-> Result<Rc<Fof_binary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_binary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_fof_binary_formula);
        let mut _localctx: Rc<Fof_binary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1071);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(67,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_binary_nonassoc*/
					recog.base.set_state(1069);
					recog.fof_binary_nonassoc()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_binary_assoc*/
					recog.base.set_state(1070);
					recog.fof_binary_assoc()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_binary_nonassoc ----------------
pub type Fof_binary_nonassocContextAll<'input> = Fof_binary_nonassocContext<'input>;


pub type Fof_binary_nonassocContext<'input> = BaseParserRuleContext<'input,Fof_binary_nonassocContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_binary_nonassocContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_binary_nonassocContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_binary_nonassocContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_binary_nonassoc(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_binary_nonassoc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_binary_nonassocContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_binary_nonassoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_binary_nonassocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_binary_nonassoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_binary_nonassoc }
}
antlr4rust::tid!{Fof_binary_nonassocContextExt<'a>}

impl<'input> Fof_binary_nonassocContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_binary_nonassocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_binary_nonassocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_binary_nonassocContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_binary_nonassocContextExt<'input>>{

fn fof_unitary_formula_all(&self) ->  Vec<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_unitary_formula(&self, i: usize) -> Option<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn binary_connective(&self) -> Option<Rc<Binary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_binary_nonassocContextAttrs<'input> for Fof_binary_nonassocContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_binary_nonassoc(&mut self,)
	-> Result<Rc<Fof_binary_nonassocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_binary_nonassocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_fof_binary_nonassoc);
        let mut _localctx: Rc<Fof_binary_nonassocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_unitary_formula*/
			recog.base.set_state(1073);
			recog.fof_unitary_formula()?;

			/*InvokeRule binary_connective*/
			recog.base.set_state(1074);
			recog.binary_connective()?;

			/*InvokeRule fof_unitary_formula*/
			recog.base.set_state(1075);
			recog.fof_unitary_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_binary_assoc ----------------
pub type Fof_binary_assocContextAll<'input> = Fof_binary_assocContext<'input>;


pub type Fof_binary_assocContext<'input> = BaseParserRuleContext<'input,Fof_binary_assocContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_binary_assocContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_binary_assocContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_binary_assocContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_binary_assoc(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_binary_assoc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_binary_assocContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_binary_assoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_binary_assocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_binary_assoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_binary_assoc }
}
antlr4rust::tid!{Fof_binary_assocContextExt<'a>}

impl<'input> Fof_binary_assocContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_binary_assocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_binary_assocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_binary_assocContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_binary_assocContextExt<'input>>{

fn fof_or_formula(&self) -> Option<Rc<Fof_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_and_formula(&self) -> Option<Rc<Fof_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_binary_assocContextAttrs<'input> for Fof_binary_assocContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_binary_assoc(&mut self,)
	-> Result<Rc<Fof_binary_assocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_binary_assocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_fof_binary_assoc);
        let mut _localctx: Rc<Fof_binary_assocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1079);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(68,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_or_formula*/
					recog.base.set_state(1077);
					recog.fof_or_formula_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_and_formula*/
					recog.base.set_state(1078);
					recog.fof_and_formula_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_or_formula ----------------
pub type Fof_or_formulaContextAll<'input> = Fof_or_formulaContext<'input>;


pub type Fof_or_formulaContext<'input> = BaseParserRuleContext<'input,Fof_or_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_or_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_or_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_or_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_or_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_or_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_or_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_or_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_or_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_or_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_or_formula }
}
antlr4rust::tid!{Fof_or_formulaContextExt<'a>}

impl<'input> Fof_or_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_or_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_or_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_or_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_or_formulaContextExt<'input>>{

fn fof_unitary_formula_all(&self) ->  Vec<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_unitary_formula(&self, i: usize) -> Option<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Or
/// Returns `None` if there is no child corresponding to token Or
fn Or(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Or, 0)
}
fn fof_or_formula(&self) -> Option<Rc<Fof_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_or_formulaContextAttrs<'input> for Fof_or_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  fof_or_formula(&mut self,)
	-> Result<Rc<Fof_or_formulaContextAll<'input>>,ANTLRError> {
		self.fof_or_formula_rec(0)
	}

	fn fof_or_formula_rec(&mut self, _p: i32)
	-> Result<Rc<Fof_or_formulaContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Fof_or_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 188, RULE_fof_or_formula, _p);
	    let mut _localctx: Rc<Fof_or_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 188;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule fof_unitary_formula*/
			recog.base.set_state(1082);
			recog.fof_unitary_formula()?;

			recog.base.set_state(1083);
			recog.base.match_token(tptp_v7_0_0_0_Or,&mut recog.err_handler)?;

			/*InvokeRule fof_unitary_formula*/
			recog.base.set_state(1084);
			recog.fof_unitary_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1091);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(69,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Fof_or_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_fof_or_formula);
					_localctx = tmp;
					recog.base.set_state(1086);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1087);
					recog.base.match_token(tptp_v7_0_0_0_Or,&mut recog.err_handler)?;

					/*InvokeRule fof_unitary_formula*/
					recog.base.set_state(1088);
					recog.fof_unitary_formula()?;

					}
					} 
				}
				recog.base.set_state(1093);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(69,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- fof_and_formula ----------------
pub type Fof_and_formulaContextAll<'input> = Fof_and_formulaContext<'input>;


pub type Fof_and_formulaContext<'input> = BaseParserRuleContext<'input,Fof_and_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_and_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_and_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_and_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_and_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_and_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_and_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_and_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_and_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_and_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_and_formula }
}
antlr4rust::tid!{Fof_and_formulaContextExt<'a>}

impl<'input> Fof_and_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_and_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_and_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_and_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_and_formulaContextExt<'input>>{

fn fof_unitary_formula_all(&self) ->  Vec<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_unitary_formula(&self, i: usize) -> Option<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token And
/// Returns `None` if there is no child corresponding to token And
fn And(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_And, 0)
}
fn fof_and_formula(&self) -> Option<Rc<Fof_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_and_formulaContextAttrs<'input> for Fof_and_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  fof_and_formula(&mut self,)
	-> Result<Rc<Fof_and_formulaContextAll<'input>>,ANTLRError> {
		self.fof_and_formula_rec(0)
	}

	fn fof_and_formula_rec(&mut self, _p: i32)
	-> Result<Rc<Fof_and_formulaContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Fof_and_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 190, RULE_fof_and_formula, _p);
	    let mut _localctx: Rc<Fof_and_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 190;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule fof_unitary_formula*/
			recog.base.set_state(1095);
			recog.fof_unitary_formula()?;

			recog.base.set_state(1096);
			recog.base.match_token(tptp_v7_0_0_0_And,&mut recog.err_handler)?;

			/*InvokeRule fof_unitary_formula*/
			recog.base.set_state(1097);
			recog.fof_unitary_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1104);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(70,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Fof_and_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_fof_and_formula);
					_localctx = tmp;
					recog.base.set_state(1099);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1100);
					recog.base.match_token(tptp_v7_0_0_0_And,&mut recog.err_handler)?;

					/*InvokeRule fof_unitary_formula*/
					recog.base.set_state(1101);
					recog.fof_unitary_formula()?;

					}
					} 
				}
				recog.base.set_state(1106);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(70,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- fof_unitary_formula ----------------
pub type Fof_unitary_formulaContextAll<'input> = Fof_unitary_formulaContext<'input>;


pub type Fof_unitary_formulaContext<'input> = BaseParserRuleContext<'input,Fof_unitary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_unitary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_unitary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_unitary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_unitary_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_unitary_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_unitary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_unitary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_unitary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_unitary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_unitary_formula }
}
antlr4rust::tid!{Fof_unitary_formulaContextExt<'a>}

impl<'input> Fof_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_unitary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_unitary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_unitary_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_unitary_formulaContextExt<'input>>{

fn fof_quantified_formula(&self) -> Option<Rc<Fof_quantified_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_unary_formula(&self) -> Option<Rc<Fof_unary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_atomic_formula(&self) -> Option<Rc<Fof_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_logic_formula(&self) -> Option<Rc<Fof_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_unitary_formulaContextAttrs<'input> for Fof_unitary_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_unitary_formula(&mut self,)
	-> Result<Rc<Fof_unitary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_unitary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_fof_unitary_formula);
        let mut _localctx: Rc<Fof_unitary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1114);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(71,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_quantified_formula*/
					recog.base.set_state(1107);
					recog.fof_quantified_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_unary_formula*/
					recog.base.set_state(1108);
					recog.fof_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1109);
					recog.fof_atomic_formula()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1110);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_logic_formula*/
					recog.base.set_state(1111);
					recog.fof_logic_formula()?;

					recog.base.set_state(1112);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_quantified_formula ----------------
pub type Fof_quantified_formulaContextAll<'input> = Fof_quantified_formulaContext<'input>;


pub type Fof_quantified_formulaContext<'input> = BaseParserRuleContext<'input,Fof_quantified_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_quantified_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_quantified_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_quantified_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_quantified_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_quantified_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_quantified_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_quantified_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_quantified_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_quantified_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_quantified_formula }
}
antlr4rust::tid!{Fof_quantified_formulaContextExt<'a>}

impl<'input> Fof_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_quantified_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_quantified_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_quantified_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_quantified_formulaContextExt<'input>>{

fn fof_quantifier(&self) -> Option<Rc<Fof_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_variable_list(&self) -> Option<Rc<Fof_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_unitary_formula(&self) -> Option<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_quantified_formulaContextAttrs<'input> for Fof_quantified_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_quantified_formula(&mut self,)
	-> Result<Rc<Fof_quantified_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_quantified_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_fof_quantified_formula);
        let mut _localctx: Rc<Fof_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_quantifier*/
			recog.base.set_state(1116);
			recog.fof_quantifier()?;

			recog.base.set_state(1117);
			recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

			/*InvokeRule fof_variable_list*/
			recog.base.set_state(1118);
			recog.fof_variable_list()?;

			recog.base.set_state(1119);
			recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

			recog.base.set_state(1120);
			recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

			/*InvokeRule fof_unitary_formula*/
			recog.base.set_state(1121);
			recog.fof_unitary_formula()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_variable_list ----------------
pub type Fof_variable_listContextAll<'input> = Fof_variable_listContext<'input>;


pub type Fof_variable_listContext<'input> = BaseParserRuleContext<'input,Fof_variable_listContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_variable_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_variable_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_variable_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_variable_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_variable_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_variable_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_variable_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_variable_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_variable_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_variable_list }
}
antlr4rust::tid!{Fof_variable_listContextExt<'a>}

impl<'input> Fof_variable_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_variable_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_variable_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_variable_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_variable_listContextExt<'input>>{

fn variable_all(&self) ->  Vec<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variable(&self, i: usize) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Fof_variable_listContextAttrs<'input> for Fof_variable_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_variable_list(&mut self,)
	-> Result<Rc<Fof_variable_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_variable_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_fof_variable_list);
        let mut _localctx: Rc<Fof_variable_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variable*/
			recog.base.set_state(1123);
			recog.variable()?;

			recog.base.set_state(1128);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1124);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule variable*/
				recog.base.set_state(1125);
				recog.variable()?;

				}
				}
				recog.base.set_state(1130);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_unary_formula ----------------
pub type Fof_unary_formulaContextAll<'input> = Fof_unary_formulaContext<'input>;


pub type Fof_unary_formulaContext<'input> = BaseParserRuleContext<'input,Fof_unary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_unary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_unary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_unary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_unary_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_unary_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_unary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_unary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_unary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_unary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_unary_formula }
}
antlr4rust::tid!{Fof_unary_formulaContextExt<'a>}

impl<'input> Fof_unary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_unary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_unary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_unary_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_unary_formulaContextExt<'input>>{

fn unary_connective(&self) -> Option<Rc<Unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_unitary_formula(&self) -> Option<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_infix_unary(&self) -> Option<Rc<Fof_infix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_unary_formulaContextAttrs<'input> for Fof_unary_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_unary_formula(&mut self,)
	-> Result<Rc<Fof_unary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_unary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_fof_unary_formula);
        let mut _localctx: Rc<Fof_unary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1135);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Not 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(1131);
					recog.unary_connective()?;

					/*InvokeRule fof_unitary_formula*/
					recog.base.set_state(1132);
					recog.fof_unitary_formula()?;

					}
				}

			tptp_v7_0_0_0_T__17 |tptp_v7_0_0_0_T__18 |tptp_v7_0_0_0_T__23 |tptp_v7_0_0_0_T__24 |
			tptp_v7_0_0_0_T__25 |tptp_v7_0_0_0_Real |tptp_v7_0_0_0_Rational |tptp_v7_0_0_0_Integer |
			tptp_v7_0_0_0_Dollar_word |tptp_v7_0_0_0_Dollar_dollar_word |tptp_v7_0_0_0_Upper_word |
			tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted |tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_infix_unary*/
					recog.base.set_state(1134);
					recog.fof_infix_unary()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_infix_unary ----------------
pub type Fof_infix_unaryContextAll<'input> = Fof_infix_unaryContext<'input>;


pub type Fof_infix_unaryContext<'input> = BaseParserRuleContext<'input,Fof_infix_unaryContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_infix_unaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_infix_unaryContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_infix_unaryContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_infix_unary(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_infix_unary(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_infix_unaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_infix_unary(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_infix_unaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_infix_unary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_infix_unary }
}
antlr4rust::tid!{Fof_infix_unaryContextExt<'a>}

impl<'input> Fof_infix_unaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_infix_unaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_infix_unaryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_infix_unaryContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_infix_unaryContextExt<'input>>{

fn fof_term_all(&self) ->  Vec<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_term(&self, i: usize) -> Option<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Infix_inequality
/// Returns `None` if there is no child corresponding to token Infix_inequality
fn Infix_inequality(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Infix_inequality, 0)
}

}

impl<'input> Fof_infix_unaryContextAttrs<'input> for Fof_infix_unaryContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_infix_unary(&mut self,)
	-> Result<Rc<Fof_infix_unaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_infix_unaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_fof_infix_unary);
        let mut _localctx: Rc<Fof_infix_unaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_term*/
			recog.base.set_state(1137);
			recog.fof_term()?;

			recog.base.set_state(1138);
			recog.base.match_token(tptp_v7_0_0_0_Infix_inequality,&mut recog.err_handler)?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1139);
			recog.fof_term()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_atomic_formula ----------------
pub type Fof_atomic_formulaContextAll<'input> = Fof_atomic_formulaContext<'input>;


pub type Fof_atomic_formulaContext<'input> = BaseParserRuleContext<'input,Fof_atomic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_atomic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_atomic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_atomic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_atomic_formula }
}
antlr4rust::tid!{Fof_atomic_formulaContextExt<'a>}

impl<'input> Fof_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_atomic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_atomic_formulaContextExt<'input>>{

fn fof_plain_atomic_formula(&self) -> Option<Rc<Fof_plain_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_defined_atomic_formula(&self) -> Option<Rc<Fof_defined_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_system_atomic_formula(&self) -> Option<Rc<Fof_system_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_atomic_formulaContextAttrs<'input> for Fof_atomic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_atomic_formula(&mut self,)
	-> Result<Rc<Fof_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 202, RULE_fof_atomic_formula);
        let mut _localctx: Rc<Fof_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1144);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(74,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_plain_atomic_formula*/
					recog.base.set_state(1141);
					recog.fof_plain_atomic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_defined_atomic_formula*/
					recog.base.set_state(1142);
					recog.fof_defined_atomic_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule fof_system_atomic_formula*/
					recog.base.set_state(1143);
					recog.fof_system_atomic_formula()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_plain_atomic_formula ----------------
pub type Fof_plain_atomic_formulaContextAll<'input> = Fof_plain_atomic_formulaContext<'input>;


pub type Fof_plain_atomic_formulaContext<'input> = BaseParserRuleContext<'input,Fof_plain_atomic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_plain_atomic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_plain_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_plain_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_plain_atomic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_plain_atomic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_plain_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_plain_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_plain_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_plain_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_plain_atomic_formula }
}
antlr4rust::tid!{Fof_plain_atomic_formulaContextExt<'a>}

impl<'input> Fof_plain_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_plain_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_plain_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_plain_atomic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_plain_atomic_formulaContextExt<'input>>{

fn fof_plain_term(&self) -> Option<Rc<Fof_plain_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_plain_atomic_formulaContextAttrs<'input> for Fof_plain_atomic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_plain_atomic_formula(&mut self,)
	-> Result<Rc<Fof_plain_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_plain_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 204, RULE_fof_plain_atomic_formula);
        let mut _localctx: Rc<Fof_plain_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_plain_term*/
			recog.base.set_state(1146);
			recog.fof_plain_term()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_defined_atomic_formula ----------------
pub type Fof_defined_atomic_formulaContextAll<'input> = Fof_defined_atomic_formulaContext<'input>;


pub type Fof_defined_atomic_formulaContext<'input> = BaseParserRuleContext<'input,Fof_defined_atomic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_defined_atomic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_defined_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_defined_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_defined_atomic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_defined_atomic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_defined_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_defined_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_atomic_formula }
}
antlr4rust::tid!{Fof_defined_atomic_formulaContextExt<'a>}

impl<'input> Fof_defined_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_atomic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_defined_atomic_formulaContextExt<'input>>{

fn fof_defined_plain_formula(&self) -> Option<Rc<Fof_defined_plain_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_defined_infix_formula(&self) -> Option<Rc<Fof_defined_infix_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_atomic_formulaContextAttrs<'input> for Fof_defined_atomic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_defined_atomic_formula(&mut self,)
	-> Result<Rc<Fof_defined_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 206, RULE_fof_defined_atomic_formula);
        let mut _localctx: Rc<Fof_defined_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1150);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(75,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_defined_plain_formula*/
					recog.base.set_state(1148);
					recog.fof_defined_plain_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_defined_infix_formula*/
					recog.base.set_state(1149);
					recog.fof_defined_infix_formula()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_defined_plain_formula ----------------
pub type Fof_defined_plain_formulaContextAll<'input> = Fof_defined_plain_formulaContext<'input>;


pub type Fof_defined_plain_formulaContext<'input> = BaseParserRuleContext<'input,Fof_defined_plain_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_defined_plain_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_defined_plain_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_defined_plain_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_defined_plain_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_defined_plain_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_defined_plain_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_defined_plain_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_plain_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_plain_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_plain_formula }
}
antlr4rust::tid!{Fof_defined_plain_formulaContextExt<'a>}

impl<'input> Fof_defined_plain_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_plain_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_plain_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_plain_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_defined_plain_formulaContextExt<'input>>{

fn fof_defined_term(&self) -> Option<Rc<Fof_defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_plain_formulaContextAttrs<'input> for Fof_defined_plain_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_defined_plain_formula(&mut self,)
	-> Result<Rc<Fof_defined_plain_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_plain_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 208, RULE_fof_defined_plain_formula);
        let mut _localctx: Rc<Fof_defined_plain_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_defined_term*/
			recog.base.set_state(1152);
			recog.fof_defined_term()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_defined_infix_formula ----------------
pub type Fof_defined_infix_formulaContextAll<'input> = Fof_defined_infix_formulaContext<'input>;


pub type Fof_defined_infix_formulaContext<'input> = BaseParserRuleContext<'input,Fof_defined_infix_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_defined_infix_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_defined_infix_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_defined_infix_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_defined_infix_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_defined_infix_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_defined_infix_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_defined_infix_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_infix_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_infix_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_infix_formula }
}
antlr4rust::tid!{Fof_defined_infix_formulaContextExt<'a>}

impl<'input> Fof_defined_infix_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_infix_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_infix_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_infix_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_defined_infix_formulaContextExt<'input>>{

fn fof_term_all(&self) ->  Vec<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_term(&self, i: usize) -> Option<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn defined_infix_pred(&self) -> Option<Rc<Defined_infix_predContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_infix_formulaContextAttrs<'input> for Fof_defined_infix_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_defined_infix_formula(&mut self,)
	-> Result<Rc<Fof_defined_infix_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_infix_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 210, RULE_fof_defined_infix_formula);
        let mut _localctx: Rc<Fof_defined_infix_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_term*/
			recog.base.set_state(1154);
			recog.fof_term()?;

			/*InvokeRule defined_infix_pred*/
			recog.base.set_state(1155);
			recog.defined_infix_pred()?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1156);
			recog.fof_term()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_system_atomic_formula ----------------
pub type Fof_system_atomic_formulaContextAll<'input> = Fof_system_atomic_formulaContext<'input>;


pub type Fof_system_atomic_formulaContext<'input> = BaseParserRuleContext<'input,Fof_system_atomic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_system_atomic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_system_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_system_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_system_atomic_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_system_atomic_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_system_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_system_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_system_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_system_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_system_atomic_formula }
}
antlr4rust::tid!{Fof_system_atomic_formulaContextExt<'a>}

impl<'input> Fof_system_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_system_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_system_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_system_atomic_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_system_atomic_formulaContextExt<'input>>{

fn fof_system_term(&self) -> Option<Rc<Fof_system_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_system_atomic_formulaContextAttrs<'input> for Fof_system_atomic_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_system_atomic_formula(&mut self,)
	-> Result<Rc<Fof_system_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_system_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 212, RULE_fof_system_atomic_formula);
        let mut _localctx: Rc<Fof_system_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_system_term*/
			recog.base.set_state(1158);
			recog.fof_system_term()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_plain_term ----------------
pub type Fof_plain_termContextAll<'input> = Fof_plain_termContext<'input>;


pub type Fof_plain_termContext<'input> = BaseParserRuleContext<'input,Fof_plain_termContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_plain_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_plain_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_plain_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_plain_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_plain_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_plain_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_plain_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_plain_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_plain_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_plain_term }
}
antlr4rust::tid!{Fof_plain_termContextExt<'a>}

impl<'input> Fof_plain_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_plain_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_plain_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_plain_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_plain_termContextExt<'input>>{

fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functor(&self) -> Option<Rc<FunctorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_arguments(&self) -> Option<Rc<Fof_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_plain_termContextAttrs<'input> for Fof_plain_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_plain_term(&mut self,)
	-> Result<Rc<Fof_plain_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_plain_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 214, RULE_fof_plain_term);
        let mut _localctx: Rc<Fof_plain_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1166);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(76,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule constant*/
					recog.base.set_state(1160);
					recog.constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule functor*/
					recog.base.set_state(1161);
					recog.functor()?;

					recog.base.set_state(1162);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1163);
					recog.fof_arguments()?;

					recog.base.set_state(1164);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_defined_term ----------------
pub type Fof_defined_termContextAll<'input> = Fof_defined_termContext<'input>;


pub type Fof_defined_termContext<'input> = BaseParserRuleContext<'input,Fof_defined_termContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_defined_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_defined_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_defined_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_defined_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_defined_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_defined_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_defined_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_term }
}
antlr4rust::tid!{Fof_defined_termContextExt<'a>}

impl<'input> Fof_defined_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_defined_termContextExt<'input>>{

fn defined_term(&self) -> Option<Rc<Defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_defined_atomic_term(&self) -> Option<Rc<Fof_defined_atomic_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_termContextAttrs<'input> for Fof_defined_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_defined_term(&mut self,)
	-> Result<Rc<Fof_defined_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 216, RULE_fof_defined_term);
        let mut _localctx: Rc<Fof_defined_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1170);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Real |tptp_v7_0_0_0_Rational |tptp_v7_0_0_0_Integer |tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(1168);
					recog.defined_term()?;

					}
				}

			tptp_v7_0_0_0_Dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_defined_atomic_term*/
					recog.base.set_state(1169);
					recog.fof_defined_atomic_term()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_defined_atomic_term ----------------
pub type Fof_defined_atomic_termContextAll<'input> = Fof_defined_atomic_termContext<'input>;


pub type Fof_defined_atomic_termContext<'input> = BaseParserRuleContext<'input,Fof_defined_atomic_termContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_defined_atomic_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_defined_atomic_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_defined_atomic_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_defined_atomic_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_defined_atomic_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_defined_atomic_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_defined_atomic_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_atomic_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_atomic_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_atomic_term }
}
antlr4rust::tid!{Fof_defined_atomic_termContextExt<'a>}

impl<'input> Fof_defined_atomic_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_atomic_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_atomic_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_atomic_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_defined_atomic_termContextExt<'input>>{

fn fof_defined_plain_term(&self) -> Option<Rc<Fof_defined_plain_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_atomic_termContextAttrs<'input> for Fof_defined_atomic_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_defined_atomic_term(&mut self,)
	-> Result<Rc<Fof_defined_atomic_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_atomic_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 218, RULE_fof_defined_atomic_term);
        let mut _localctx: Rc<Fof_defined_atomic_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_defined_plain_term*/
			recog.base.set_state(1172);
			recog.fof_defined_plain_term()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_defined_plain_term ----------------
pub type Fof_defined_plain_termContextAll<'input> = Fof_defined_plain_termContext<'input>;


pub type Fof_defined_plain_termContext<'input> = BaseParserRuleContext<'input,Fof_defined_plain_termContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_defined_plain_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_defined_plain_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_defined_plain_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_defined_plain_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_defined_plain_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_defined_plain_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_defined_plain_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_plain_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_plain_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_plain_term }
}
antlr4rust::tid!{Fof_defined_plain_termContextExt<'a>}

impl<'input> Fof_defined_plain_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_plain_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_plain_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_plain_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_defined_plain_termContextExt<'input>>{

fn defined_constant(&self) -> Option<Rc<Defined_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_functor(&self) -> Option<Rc<Defined_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_arguments(&self) -> Option<Rc<Fof_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_plain_termContextAttrs<'input> for Fof_defined_plain_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_defined_plain_term(&mut self,)
	-> Result<Rc<Fof_defined_plain_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_plain_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 220, RULE_fof_defined_plain_term);
        let mut _localctx: Rc<Fof_defined_plain_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1180);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(78,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(1174);
					recog.defined_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule defined_functor*/
					recog.base.set_state(1175);
					recog.defined_functor()?;

					recog.base.set_state(1176);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1177);
					recog.fof_arguments()?;

					recog.base.set_state(1178);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_system_term ----------------
pub type Fof_system_termContextAll<'input> = Fof_system_termContext<'input>;


pub type Fof_system_termContext<'input> = BaseParserRuleContext<'input,Fof_system_termContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_system_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_system_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_system_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_system_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_system_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_system_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_system_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_system_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_system_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_system_term }
}
antlr4rust::tid!{Fof_system_termContextExt<'a>}

impl<'input> Fof_system_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_system_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_system_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_system_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_system_termContextExt<'input>>{

fn system_constant(&self) -> Option<Rc<System_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn system_functor(&self) -> Option<Rc<System_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_arguments(&self) -> Option<Rc<Fof_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_system_termContextAttrs<'input> for Fof_system_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_system_term(&mut self,)
	-> Result<Rc<Fof_system_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_system_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 222, RULE_fof_system_term);
        let mut _localctx: Rc<Fof_system_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1188);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(79,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule system_constant*/
					recog.base.set_state(1182);
					recog.system_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule system_functor*/
					recog.base.set_state(1183);
					recog.system_functor()?;

					recog.base.set_state(1184);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1185);
					recog.fof_arguments()?;

					recog.base.set_state(1186);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_arguments ----------------
pub type Fof_argumentsContextAll<'input> = Fof_argumentsContext<'input>;


pub type Fof_argumentsContext<'input> = BaseParserRuleContext<'input,Fof_argumentsContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_argumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_argumentsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_argumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_arguments(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_arguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_argumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_arguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_argumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_arguments }
}
antlr4rust::tid!{Fof_argumentsContextExt<'a>}

impl<'input> Fof_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_argumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_argumentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_argumentsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_argumentsContextExt<'input>>{

fn fof_term_all(&self) ->  Vec<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_term(&self, i: usize) -> Option<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Fof_argumentsContextAttrs<'input> for Fof_argumentsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_arguments(&mut self,)
	-> Result<Rc<Fof_argumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_argumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 224, RULE_fof_arguments);
        let mut _localctx: Rc<Fof_argumentsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_term*/
			recog.base.set_state(1190);
			recog.fof_term()?;

			recog.base.set_state(1195);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1191);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule fof_term*/
				recog.base.set_state(1192);
				recog.fof_term()?;

				}
				}
				recog.base.set_state(1197);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_term ----------------
pub type Fof_termContextAll<'input> = Fof_termContext<'input>;


pub type Fof_termContext<'input> = BaseParserRuleContext<'input,Fof_termContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_term }
}
antlr4rust::tid!{Fof_termContextExt<'a>}

impl<'input> Fof_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_termContextExt<'input>>{

fn fof_function_term(&self) -> Option<Rc<Fof_function_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_conditional_term(&self) -> Option<Rc<Tff_conditional_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_term(&self) -> Option<Rc<Tff_let_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_tuple_term(&self) -> Option<Rc<Tff_tuple_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_termContextAttrs<'input> for Fof_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_term(&mut self,)
	-> Result<Rc<Fof_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 226, RULE_fof_term);
        let mut _localctx: Rc<Fof_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1203);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Real |tptp_v7_0_0_0_Rational |tptp_v7_0_0_0_Integer |tptp_v7_0_0_0_Dollar_word |
			tptp_v7_0_0_0_Dollar_dollar_word |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted |
			tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_function_term*/
					recog.base.set_state(1198);
					recog.fof_function_term()?;

					}
				}

			tptp_v7_0_0_0_Upper_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable*/
					recog.base.set_state(1199);
					recog.variable()?;

					}
				}

			tptp_v7_0_0_0_T__23 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule tff_conditional_term*/
					recog.base.set_state(1200);
					recog.tff_conditional_term()?;

					}
				}

			tptp_v7_0_0_0_T__24 |tptp_v7_0_0_0_T__25 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule tff_let_term*/
					recog.base.set_state(1201);
					recog.tff_let_term()?;

					}
				}

			tptp_v7_0_0_0_T__17 |tptp_v7_0_0_0_T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule tff_tuple_term*/
					recog.base.set_state(1202);
					recog.tff_tuple_term()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_function_term ----------------
pub type Fof_function_termContextAll<'input> = Fof_function_termContext<'input>;


pub type Fof_function_termContext<'input> = BaseParserRuleContext<'input,Fof_function_termContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_function_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_function_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_function_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_function_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_function_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_function_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_function_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_function_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_function_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_function_term }
}
antlr4rust::tid!{Fof_function_termContextExt<'a>}

impl<'input> Fof_function_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_function_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_function_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_function_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_function_termContextExt<'input>>{

fn fof_plain_term(&self) -> Option<Rc<Fof_plain_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_defined_term(&self) -> Option<Rc<Fof_defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_system_term(&self) -> Option<Rc<Fof_system_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_function_termContextAttrs<'input> for Fof_function_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_function_term(&mut self,)
	-> Result<Rc<Fof_function_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_function_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 228, RULE_fof_function_term);
        let mut _localctx: Rc<Fof_function_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1208);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_plain_term*/
					recog.base.set_state(1205);
					recog.fof_plain_term()?;

					}
				}

			tptp_v7_0_0_0_Real |tptp_v7_0_0_0_Rational |tptp_v7_0_0_0_Integer |tptp_v7_0_0_0_Dollar_word |
			tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_defined_term*/
					recog.base.set_state(1206);
					recog.fof_defined_term()?;

					}
				}

			tptp_v7_0_0_0_Dollar_dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule fof_system_term*/
					recog.base.set_state(1207);
					recog.fof_system_term()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_conditional_term ----------------
pub type Tff_conditional_termContextAll<'input> = Tff_conditional_termContext<'input>;


pub type Tff_conditional_termContext<'input> = BaseParserRuleContext<'input,Tff_conditional_termContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_conditional_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_conditional_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_conditional_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_conditional_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_conditional_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_conditional_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_conditional_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_conditional_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_conditional_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_conditional_term }
}
antlr4rust::tid!{Tff_conditional_termContextExt<'a>}

impl<'input> Tff_conditional_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_conditional_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_conditional_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_conditional_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_conditional_termContextExt<'input>>{

fn tff_logic_formula(&self) -> Option<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_term_all(&self) ->  Vec<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_term(&self, i: usize) -> Option<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tff_conditional_termContextAttrs<'input> for Tff_conditional_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_conditional_term(&mut self,)
	-> Result<Rc<Tff_conditional_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_conditional_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 230, RULE_tff_conditional_term);
        let mut _localctx: Rc<Tff_conditional_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1210);
			recog.base.match_token(tptp_v7_0_0_0_T__23,&mut recog.err_handler)?;

			/*InvokeRule tff_logic_formula*/
			recog.base.set_state(1211);
			recog.tff_logic_formula()?;

			recog.base.set_state(1212);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1213);
			recog.fof_term()?;

			recog.base.set_state(1214);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1215);
			recog.fof_term()?;

			recog.base.set_state(1216);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_let_term ----------------
pub type Tff_let_termContextAll<'input> = Tff_let_termContext<'input>;


pub type Tff_let_termContext<'input> = BaseParserRuleContext<'input,Tff_let_termContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_let_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_let_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_let_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_let_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_let_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_let_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_let_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_let_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_let_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_let_term }
}
antlr4rust::tid!{Tff_let_termContextExt<'a>}

impl<'input> Tff_let_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_let_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_let_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_let_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_let_termContextExt<'input>>{

fn tff_let_formula_defns(&self) -> Option<Rc<Tff_let_formula_defnsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_term(&self) -> Option<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_let_term_defns(&self) -> Option<Rc<Tff_let_term_defnsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_let_termContextAttrs<'input> for Tff_let_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_let_term(&mut self,)
	-> Result<Rc<Tff_let_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_let_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 232, RULE_tff_let_term);
        let mut _localctx: Rc<Tff_let_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1230);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__24 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1218);
					recog.base.match_token(tptp_v7_0_0_0_T__24,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_defns*/
					recog.base.set_state(1219);
					recog.tff_let_formula_defns()?;

					recog.base.set_state(1220);
					recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(1221);
					recog.fof_term()?;

					recog.base.set_state(1222);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__25 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1224);
					recog.base.match_token(tptp_v7_0_0_0_T__25,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_defns*/
					recog.base.set_state(1225);
					recog.tff_let_term_defns()?;

					recog.base.set_state(1226);
					recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(1227);
					recog.fof_term()?;

					recog.base.set_state(1228);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_tuple_term ----------------
pub type Tff_tuple_termContextAll<'input> = Tff_tuple_termContext<'input>;


pub type Tff_tuple_termContext<'input> = BaseParserRuleContext<'input,Tff_tuple_termContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_tuple_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_tuple_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_tuple_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_tuple_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_tuple_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_tuple_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_tuple_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_tuple_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_tuple_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_tuple_term }
}
antlr4rust::tid!{Tff_tuple_termContextExt<'a>}

impl<'input> Tff_tuple_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_tuple_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_tuple_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_tuple_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_tuple_termContextExt<'input>>{

fn fof_arguments(&self) -> Option<Rc<Fof_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_tuple_termContextAttrs<'input> for Tff_tuple_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_tuple_term(&mut self,)
	-> Result<Rc<Tff_tuple_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_tuple_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 234, RULE_tff_tuple_term);
        let mut _localctx: Rc<Tff_tuple_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1237);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1232);
					recog.base.match_token(tptp_v7_0_0_0_T__17,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1233);
					recog.base.match_token(tptp_v7_0_0_0_T__18,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1234);
					recog.fof_arguments()?;

					recog.base.set_state(1235);
					recog.base.match_token(tptp_v7_0_0_0_T__19,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_sequent ----------------
pub type Fof_sequentContextAll<'input> = Fof_sequentContext<'input>;


pub type Fof_sequentContext<'input> = BaseParserRuleContext<'input,Fof_sequentContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_sequentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_sequentContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_sequentContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_sequent(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_sequent(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_sequentContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_sequent(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_sequentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_sequent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_sequent }
}
antlr4rust::tid!{Fof_sequentContextExt<'a>}

impl<'input> Fof_sequentContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_sequentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_sequentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_sequentContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_sequentContextExt<'input>>{

fn fof_formula_tuple_all(&self) ->  Vec<Rc<Fof_formula_tupleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_formula_tuple(&self, i: usize) -> Option<Rc<Fof_formula_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Gentzen_arrow
/// Returns `None` if there is no child corresponding to token Gentzen_arrow
fn Gentzen_arrow(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Gentzen_arrow, 0)
}
fn fof_sequent(&self) -> Option<Rc<Fof_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_sequentContextAttrs<'input> for Fof_sequentContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_sequent(&mut self,)
	-> Result<Rc<Fof_sequentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_sequentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 236, RULE_fof_sequent);
        let mut _localctx: Rc<Fof_sequentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1247);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__11 |tptp_v7_0_0_0_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_formula_tuple*/
					recog.base.set_state(1239);
					recog.fof_formula_tuple()?;

					recog.base.set_state(1240);
					recog.base.match_token(tptp_v7_0_0_0_Gentzen_arrow,&mut recog.err_handler)?;

					/*InvokeRule fof_formula_tuple*/
					recog.base.set_state(1241);
					recog.fof_formula_tuple()?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1243);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_sequent*/
					recog.base.set_state(1244);
					recog.fof_sequent()?;

					recog.base.set_state(1245);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_formula_tuple ----------------
pub type Fof_formula_tupleContextAll<'input> = Fof_formula_tupleContext<'input>;


pub type Fof_formula_tupleContext<'input> = BaseParserRuleContext<'input,Fof_formula_tupleContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_formula_tupleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_formula_tupleContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_formula_tupleContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_formula_tuple(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_formula_tuple(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_formula_tupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_formula_tuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_formula_tupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_formula_tuple }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_formula_tuple }
}
antlr4rust::tid!{Fof_formula_tupleContextExt<'a>}

impl<'input> Fof_formula_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_formula_tupleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_formula_tupleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_formula_tupleContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_formula_tupleContextExt<'input>>{

fn fof_formula_tuple_list(&self) -> Option<Rc<Fof_formula_tuple_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_formula_tupleContextAttrs<'input> for Fof_formula_tupleContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_formula_tuple(&mut self,)
	-> Result<Rc<Fof_formula_tupleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_formula_tupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 238, RULE_fof_formula_tuple);
        let mut _localctx: Rc<Fof_formula_tupleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1254);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1249);
					recog.base.match_token(tptp_v7_0_0_0_T__16,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1250);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_formula_tuple_list*/
					recog.base.set_state(1251);
					recog.fof_formula_tuple_list()?;

					recog.base.set_state(1252);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_formula_tuple_list ----------------
pub type Fof_formula_tuple_listContextAll<'input> = Fof_formula_tuple_listContext<'input>;


pub type Fof_formula_tuple_listContext<'input> = BaseParserRuleContext<'input,Fof_formula_tuple_listContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_formula_tuple_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_formula_tuple_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_formula_tuple_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_formula_tuple_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_formula_tuple_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_formula_tuple_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_formula_tuple_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_formula_tuple_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_formula_tuple_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_formula_tuple_list }
}
antlr4rust::tid!{Fof_formula_tuple_listContextExt<'a>}

impl<'input> Fof_formula_tuple_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_formula_tuple_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_formula_tuple_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_formula_tuple_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_formula_tuple_listContextExt<'input>>{

fn fof_logic_formula_all(&self) ->  Vec<Rc<Fof_logic_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_logic_formula(&self, i: usize) -> Option<Rc<Fof_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Fof_formula_tuple_listContextAttrs<'input> for Fof_formula_tuple_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_formula_tuple_list(&mut self,)
	-> Result<Rc<Fof_formula_tuple_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_formula_tuple_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 240, RULE_fof_formula_tuple_list);
        let mut _localctx: Rc<Fof_formula_tuple_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule fof_logic_formula*/
			recog.base.set_state(1256);
			recog.fof_logic_formula()?;

			recog.base.set_state(1261);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1257);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule fof_logic_formula*/
				recog.base.set_state(1258);
				recog.fof_logic_formula()?;

				}
				}
				recog.base.set_state(1263);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- cnf_formula ----------------
pub type Cnf_formulaContextAll<'input> = Cnf_formulaContext<'input>;


pub type Cnf_formulaContext<'input> = BaseParserRuleContext<'input,Cnf_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Cnf_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Cnf_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Cnf_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cnf_formula(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_cnf_formula(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Cnf_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_cnf_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Cnf_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cnf_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cnf_formula }
}
antlr4rust::tid!{Cnf_formulaContextExt<'a>}

impl<'input> Cnf_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Cnf_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Cnf_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Cnf_formulaContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Cnf_formulaContextExt<'input>>{

fn cnf_disjunction(&self) -> Option<Rc<Cnf_disjunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Cnf_formulaContextAttrs<'input> for Cnf_formulaContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cnf_formula(&mut self,)
	-> Result<Rc<Cnf_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Cnf_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 242, RULE_cnf_formula);
        let mut _localctx: Rc<Cnf_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1269);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__17 |tptp_v7_0_0_0_T__18 |tptp_v7_0_0_0_T__23 |tptp_v7_0_0_0_T__24 |
			tptp_v7_0_0_0_T__25 |tptp_v7_0_0_0_Not |tptp_v7_0_0_0_Real |tptp_v7_0_0_0_Rational |
			tptp_v7_0_0_0_Integer |tptp_v7_0_0_0_Dollar_word |tptp_v7_0_0_0_Dollar_dollar_word |
			tptp_v7_0_0_0_Upper_word |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted |
			tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule cnf_disjunction*/
					recog.base.set_state(1264);
					recog.cnf_disjunction_rec(0)?;

					}
				}

			tptp_v7_0_0_0_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1265);
					recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

					/*InvokeRule cnf_disjunction*/
					recog.base.set_state(1266);
					recog.cnf_disjunction_rec(0)?;

					recog.base.set_state(1267);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- cnf_disjunction ----------------
pub type Cnf_disjunctionContextAll<'input> = Cnf_disjunctionContext<'input>;


pub type Cnf_disjunctionContext<'input> = BaseParserRuleContext<'input,Cnf_disjunctionContextExt<'input>>;

#[derive(Clone)]
pub struct Cnf_disjunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Cnf_disjunctionContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Cnf_disjunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cnf_disjunction(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_cnf_disjunction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Cnf_disjunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_cnf_disjunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for Cnf_disjunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cnf_disjunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cnf_disjunction }
}
antlr4rust::tid!{Cnf_disjunctionContextExt<'a>}

impl<'input> Cnf_disjunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Cnf_disjunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Cnf_disjunctionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Cnf_disjunctionContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Cnf_disjunctionContextExt<'input>>{

fn cnf_literal(&self) -> Option<Rc<Cnf_literalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cnf_disjunction(&self) -> Option<Rc<Cnf_disjunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Or
/// Returns `None` if there is no child corresponding to token Or
fn Or(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Or, 0)
}

}

impl<'input> Cnf_disjunctionContextAttrs<'input> for Cnf_disjunctionContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  cnf_disjunction(&mut self,)
	-> Result<Rc<Cnf_disjunctionContextAll<'input>>,ANTLRError> {
		self.cnf_disjunction_rec(0)
	}

	fn cnf_disjunction_rec(&mut self, _p: i32)
	-> Result<Rc<Cnf_disjunctionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Cnf_disjunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 244, RULE_cnf_disjunction, _p);
	    let mut _localctx: Rc<Cnf_disjunctionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 244;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule cnf_literal*/
			recog.base.set_state(1272);
			recog.cnf_literal()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1279);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(89,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Cnf_disjunctionContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_cnf_disjunction);
					_localctx = tmp;
					recog.base.set_state(1274);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1275);
					recog.base.match_token(tptp_v7_0_0_0_Or,&mut recog.err_handler)?;

					/*InvokeRule cnf_literal*/
					recog.base.set_state(1276);
					recog.cnf_literal()?;

					}
					} 
				}
				recog.base.set_state(1281);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(89,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- cnf_literal ----------------
pub type Cnf_literalContextAll<'input> = Cnf_literalContext<'input>;


pub type Cnf_literalContext<'input> = BaseParserRuleContext<'input,Cnf_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Cnf_literalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Cnf_literalContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Cnf_literalContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_cnf_literal(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_cnf_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Cnf_literalContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_cnf_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for Cnf_literalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cnf_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cnf_literal }
}
antlr4rust::tid!{Cnf_literalContextExt<'a>}

impl<'input> Cnf_literalContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Cnf_literalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Cnf_literalContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Cnf_literalContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Cnf_literalContextExt<'input>>{

fn fof_atomic_formula(&self) -> Option<Rc<Fof_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Not
/// Returns `None` if there is no child corresponding to token Not
fn Not(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Not, 0)
}
fn fof_infix_unary(&self) -> Option<Rc<Fof_infix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Cnf_literalContextAttrs<'input> for Cnf_literalContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cnf_literal(&mut self,)
	-> Result<Rc<Cnf_literalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Cnf_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 246, RULE_cnf_literal);
        let mut _localctx: Rc<Cnf_literalContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1286);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(90,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1282);
					recog.fof_atomic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1283);
					recog.base.match_token(tptp_v7_0_0_0_Not,&mut recog.err_handler)?;

					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1284);
					recog.fof_atomic_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule fof_infix_unary*/
					recog.base.set_state(1285);
					recog.fof_infix_unary()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_quantifier ----------------
pub type Thf_quantifierContextAll<'input> = Thf_quantifierContext<'input>;


pub type Thf_quantifierContext<'input> = BaseParserRuleContext<'input,Thf_quantifierContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_quantifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_quantifierContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_quantifierContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_quantifier(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_quantifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_quantifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_quantifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_quantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_quantifier }
}
antlr4rust::tid!{Thf_quantifierContextExt<'a>}

impl<'input> Thf_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_quantifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_quantifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_quantifierContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_quantifierContextExt<'input>>{

fn fof_quantifier(&self) -> Option<Rc<Fof_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn th0_quantifier(&self) -> Option<Rc<Th0_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn th1_quantifier(&self) -> Option<Rc<Th1_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_quantifierContextAttrs<'input> for Thf_quantifierContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_quantifier(&mut self,)
	-> Result<Rc<Thf_quantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_quantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 248, RULE_thf_quantifier);
        let mut _localctx: Rc<Thf_quantifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1291);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Forall |tptp_v7_0_0_0_Exists 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_quantifier*/
					recog.base.set_state(1288);
					recog.fof_quantifier()?;

					}
				}

			tptp_v7_0_0_0_Lambda |tptp_v7_0_0_0_Choice |tptp_v7_0_0_0_Description 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule th0_quantifier*/
					recog.base.set_state(1289);
					recog.th0_quantifier()?;

					}
				}

			tptp_v7_0_0_0_TyForall |tptp_v7_0_0_0_TyExists 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule th1_quantifier*/
					recog.base.set_state(1290);
					recog.th1_quantifier()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- th0_quantifier ----------------
pub type Th0_quantifierContextAll<'input> = Th0_quantifierContext<'input>;


pub type Th0_quantifierContext<'input> = BaseParserRuleContext<'input,Th0_quantifierContextExt<'input>>;

#[derive(Clone)]
pub struct Th0_quantifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Th0_quantifierContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Th0_quantifierContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_th0_quantifier(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_th0_quantifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Th0_quantifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_th0_quantifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Th0_quantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_th0_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_th0_quantifier }
}
antlr4rust::tid!{Th0_quantifierContextExt<'a>}

impl<'input> Th0_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Th0_quantifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Th0_quantifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Th0_quantifierContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Th0_quantifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Lambda
/// Returns `None` if there is no child corresponding to token Lambda
fn Lambda(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Lambda, 0)
}
/// Retrieves first TerminalNode corresponding to token Choice
/// Returns `None` if there is no child corresponding to token Choice
fn Choice(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Choice, 0)
}
/// Retrieves first TerminalNode corresponding to token Description
/// Returns `None` if there is no child corresponding to token Description
fn Description(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Description, 0)
}

}

impl<'input> Th0_quantifierContextAttrs<'input> for Th0_quantifierContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn th0_quantifier(&mut self,)
	-> Result<Rc<Th0_quantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Th0_quantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 250, RULE_th0_quantifier);
        let mut _localctx: Rc<Th0_quantifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1293);
			_la = recog.base.input.la(1);
			if { !(((((_la - 61)) & !0x3f) == 0 && ((1usize << (_la - 61)) & 21) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- th1_quantifier ----------------
pub type Th1_quantifierContextAll<'input> = Th1_quantifierContext<'input>;


pub type Th1_quantifierContext<'input> = BaseParserRuleContext<'input,Th1_quantifierContextExt<'input>>;

#[derive(Clone)]
pub struct Th1_quantifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Th1_quantifierContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Th1_quantifierContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_th1_quantifier(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_th1_quantifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Th1_quantifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_th1_quantifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Th1_quantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_th1_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_th1_quantifier }
}
antlr4rust::tid!{Th1_quantifierContextExt<'a>}

impl<'input> Th1_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Th1_quantifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Th1_quantifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Th1_quantifierContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Th1_quantifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TyForall
/// Returns `None` if there is no child corresponding to token TyForall
fn TyForall(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_TyForall, 0)
}
/// Retrieves first TerminalNode corresponding to token TyExists
/// Returns `None` if there is no child corresponding to token TyExists
fn TyExists(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_TyExists, 0)
}

}

impl<'input> Th1_quantifierContextAttrs<'input> for Th1_quantifierContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn th1_quantifier(&mut self,)
	-> Result<Rc<Th1_quantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Th1_quantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 252, RULE_th1_quantifier);
        let mut _localctx: Rc<Th1_quantifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1295);
			_la = recog.base.input.la(1);
			if { !(_la==tptp_v7_0_0_0_TyForall || _la==tptp_v7_0_0_0_TyExists) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_pair_connective ----------------
pub type Thf_pair_connectiveContextAll<'input> = Thf_pair_connectiveContext<'input>;


pub type Thf_pair_connectiveContext<'input> = BaseParserRuleContext<'input,Thf_pair_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_pair_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_pair_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_pair_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_pair_connective(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_pair_connective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_pair_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_pair_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_pair_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_pair_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_pair_connective }
}
antlr4rust::tid!{Thf_pair_connectiveContextExt<'a>}

impl<'input> Thf_pair_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_pair_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_pair_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_pair_connectiveContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_pair_connectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Infix_equality
/// Returns `None` if there is no child corresponding to token Infix_equality
fn Infix_equality(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Infix_equality, 0)
}
/// Retrieves first TerminalNode corresponding to token Infix_inequality
/// Returns `None` if there is no child corresponding to token Infix_inequality
fn Infix_inequality(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Infix_inequality, 0)
}
fn binary_connective(&self) -> Option<Rc<Binary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Assignment
/// Returns `None` if there is no child corresponding to token Assignment
fn Assignment(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Assignment, 0)
}

}

impl<'input> Thf_pair_connectiveContextAttrs<'input> for Thf_pair_connectiveContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_pair_connective(&mut self,)
	-> Result<Rc<Thf_pair_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_pair_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 254, RULE_thf_pair_connective);
        let mut _localctx: Rc<Thf_pair_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1301);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Infix_equality 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1297);
					recog.base.match_token(tptp_v7_0_0_0_Infix_equality,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_Infix_inequality 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1298);
					recog.base.match_token(tptp_v7_0_0_0_Infix_inequality,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_Iff |tptp_v7_0_0_0_Impl |tptp_v7_0_0_0_If |tptp_v7_0_0_0_Niff |
			tptp_v7_0_0_0_Nor |tptp_v7_0_0_0_Nand 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule binary_connective*/
					recog.base.set_state(1299);
					recog.binary_connective()?;

					}
				}

			tptp_v7_0_0_0_Assignment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1300);
					recog.base.match_token(tptp_v7_0_0_0_Assignment,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- thf_unary_connective ----------------
pub type Thf_unary_connectiveContextAll<'input> = Thf_unary_connectiveContext<'input>;


pub type Thf_unary_connectiveContext<'input> = BaseParserRuleContext<'input,Thf_unary_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_unary_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Thf_unary_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Thf_unary_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_thf_unary_connective(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_thf_unary_connective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Thf_unary_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_thf_unary_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unary_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unary_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unary_connective }
}
antlr4rust::tid!{Thf_unary_connectiveContextExt<'a>}

impl<'input> Thf_unary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unary_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unary_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unary_connectiveContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Thf_unary_connectiveContextExt<'input>>{

fn unary_connective(&self) -> Option<Rc<Unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn th1_unary_connective(&self) -> Option<Rc<Th1_unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unary_connectiveContextAttrs<'input> for Thf_unary_connectiveContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn thf_unary_connective(&mut self,)
	-> Result<Rc<Thf_unary_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unary_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 256, RULE_thf_unary_connective);
        let mut _localctx: Rc<Thf_unary_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1305);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Not 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(1303);
					recog.unary_connective()?;

					}
				}

			tptp_v7_0_0_0_ForallComb |tptp_v7_0_0_0_ExistsComb |tptp_v7_0_0_0_ChoiceComb |
			tptp_v7_0_0_0_DescriptionComb |tptp_v7_0_0_0_EqComb 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule th1_unary_connective*/
					recog.base.set_state(1304);
					recog.th1_unary_connective()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- th1_unary_connective ----------------
pub type Th1_unary_connectiveContextAll<'input> = Th1_unary_connectiveContext<'input>;


pub type Th1_unary_connectiveContext<'input> = BaseParserRuleContext<'input,Th1_unary_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Th1_unary_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Th1_unary_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Th1_unary_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_th1_unary_connective(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_th1_unary_connective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Th1_unary_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_th1_unary_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Th1_unary_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_th1_unary_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_th1_unary_connective }
}
antlr4rust::tid!{Th1_unary_connectiveContextExt<'a>}

impl<'input> Th1_unary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Th1_unary_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Th1_unary_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Th1_unary_connectiveContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Th1_unary_connectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ForallComb
/// Returns `None` if there is no child corresponding to token ForallComb
fn ForallComb(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_ForallComb, 0)
}
/// Retrieves first TerminalNode corresponding to token ExistsComb
/// Returns `None` if there is no child corresponding to token ExistsComb
fn ExistsComb(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_ExistsComb, 0)
}
/// Retrieves first TerminalNode corresponding to token ChoiceComb
/// Returns `None` if there is no child corresponding to token ChoiceComb
fn ChoiceComb(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_ChoiceComb, 0)
}
/// Retrieves first TerminalNode corresponding to token DescriptionComb
/// Returns `None` if there is no child corresponding to token DescriptionComb
fn DescriptionComb(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_DescriptionComb, 0)
}
/// Retrieves first TerminalNode corresponding to token EqComb
/// Returns `None` if there is no child corresponding to token EqComb
fn EqComb(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_EqComb, 0)
}

}

impl<'input> Th1_unary_connectiveContextAttrs<'input> for Th1_unary_connectiveContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn th1_unary_connective(&mut self,)
	-> Result<Rc<Th1_unary_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Th1_unary_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 258, RULE_th1_unary_connective);
        let mut _localctx: Rc<Th1_unary_connectiveContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1307);
			_la = recog.base.input.la(1);
			if { !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & 10785) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tff_pair_connective ----------------
pub type Tff_pair_connectiveContextAll<'input> = Tff_pair_connectiveContext<'input>;


pub type Tff_pair_connectiveContext<'input> = BaseParserRuleContext<'input,Tff_pair_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_pair_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Tff_pair_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Tff_pair_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_tff_pair_connective(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_tff_pair_connective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Tff_pair_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_tff_pair_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_pair_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_pair_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_pair_connective }
}
antlr4rust::tid!{Tff_pair_connectiveContextExt<'a>}

impl<'input> Tff_pair_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_pair_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_pair_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_pair_connectiveContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Tff_pair_connectiveContextExt<'input>>{

fn binary_connective(&self) -> Option<Rc<Binary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Assignment
/// Returns `None` if there is no child corresponding to token Assignment
fn Assignment(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Assignment, 0)
}

}

impl<'input> Tff_pair_connectiveContextAttrs<'input> for Tff_pair_connectiveContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tff_pair_connective(&mut self,)
	-> Result<Rc<Tff_pair_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_pair_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 260, RULE_tff_pair_connective);
        let mut _localctx: Rc<Tff_pair_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1311);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Iff |tptp_v7_0_0_0_Impl |tptp_v7_0_0_0_If |tptp_v7_0_0_0_Niff |
			tptp_v7_0_0_0_Nor |tptp_v7_0_0_0_Nand 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule binary_connective*/
					recog.base.set_state(1309);
					recog.binary_connective()?;

					}
				}

			tptp_v7_0_0_0_Assignment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1310);
					recog.base.match_token(tptp_v7_0_0_0_Assignment,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fof_quantifier ----------------
pub type Fof_quantifierContextAll<'input> = Fof_quantifierContext<'input>;


pub type Fof_quantifierContext<'input> = BaseParserRuleContext<'input,Fof_quantifierContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_quantifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Fof_quantifierContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Fof_quantifierContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fof_quantifier(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_fof_quantifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Fof_quantifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_fof_quantifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_quantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_quantifier }
}
antlr4rust::tid!{Fof_quantifierContextExt<'a>}

impl<'input> Fof_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_quantifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_quantifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_quantifierContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Fof_quantifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Forall
/// Returns `None` if there is no child corresponding to token Forall
fn Forall(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Forall, 0)
}
/// Retrieves first TerminalNode corresponding to token Exists
/// Returns `None` if there is no child corresponding to token Exists
fn Exists(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Exists, 0)
}

}

impl<'input> Fof_quantifierContextAttrs<'input> for Fof_quantifierContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fof_quantifier(&mut self,)
	-> Result<Rc<Fof_quantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_quantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 262, RULE_fof_quantifier);
        let mut _localctx: Rc<Fof_quantifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1313);
			_la = recog.base.input.la(1);
			if { !(_la==tptp_v7_0_0_0_Forall || _la==tptp_v7_0_0_0_Exists) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- binary_connective ----------------
pub type Binary_connectiveContextAll<'input> = Binary_connectiveContext<'input>;


pub type Binary_connectiveContext<'input> = BaseParserRuleContext<'input,Binary_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Binary_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Binary_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Binary_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_binary_connective(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_binary_connective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Binary_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_binary_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Binary_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binary_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binary_connective }
}
antlr4rust::tid!{Binary_connectiveContextExt<'a>}

impl<'input> Binary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Binary_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Binary_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Binary_connectiveContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Binary_connectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Iff
/// Returns `None` if there is no child corresponding to token Iff
fn Iff(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Iff, 0)
}
/// Retrieves first TerminalNode corresponding to token Impl
/// Returns `None` if there is no child corresponding to token Impl
fn Impl(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Impl, 0)
}
/// Retrieves first TerminalNode corresponding to token If
/// Returns `None` if there is no child corresponding to token If
fn If(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_If, 0)
}
/// Retrieves first TerminalNode corresponding to token Niff
/// Returns `None` if there is no child corresponding to token Niff
fn Niff(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Niff, 0)
}
/// Retrieves first TerminalNode corresponding to token Nor
/// Returns `None` if there is no child corresponding to token Nor
fn Nor(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Nor, 0)
}
/// Retrieves first TerminalNode corresponding to token Nand
/// Returns `None` if there is no child corresponding to token Nand
fn Nand(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Nand, 0)
}

}

impl<'input> Binary_connectiveContextAttrs<'input> for Binary_connectiveContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn binary_connective(&mut self,)
	-> Result<Rc<Binary_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Binary_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 264, RULE_binary_connective);
        let mut _localctx: Rc<Binary_connectiveContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1315);
			_la = recog.base.input.la(1);
			if { !(((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & 63) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assoc_connective ----------------
pub type Assoc_connectiveContextAll<'input> = Assoc_connectiveContext<'input>;


pub type Assoc_connectiveContext<'input> = BaseParserRuleContext<'input,Assoc_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Assoc_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Assoc_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Assoc_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assoc_connective(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_assoc_connective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Assoc_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_assoc_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Assoc_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assoc_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assoc_connective }
}
antlr4rust::tid!{Assoc_connectiveContextExt<'a>}

impl<'input> Assoc_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Assoc_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Assoc_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Assoc_connectiveContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Assoc_connectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Or
/// Returns `None` if there is no child corresponding to token Or
fn Or(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Or, 0)
}
/// Retrieves first TerminalNode corresponding to token And
/// Returns `None` if there is no child corresponding to token And
fn And(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_And, 0)
}

}

impl<'input> Assoc_connectiveContextAttrs<'input> for Assoc_connectiveContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assoc_connective(&mut self,)
	-> Result<Rc<Assoc_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Assoc_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 266, RULE_assoc_connective);
        let mut _localctx: Rc<Assoc_connectiveContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1317);
			_la = recog.base.input.la(1);
			if { !(_la==tptp_v7_0_0_0_Or || _la==tptp_v7_0_0_0_And) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unary_connective ----------------
pub type Unary_connectiveContextAll<'input> = Unary_connectiveContext<'input>;


pub type Unary_connectiveContext<'input> = BaseParserRuleContext<'input,Unary_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Unary_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Unary_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Unary_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unary_connective(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_unary_connective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Unary_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_unary_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Unary_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_connective }
}
antlr4rust::tid!{Unary_connectiveContextExt<'a>}

impl<'input> Unary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Unary_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Unary_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Unary_connectiveContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Unary_connectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Not
/// Returns `None` if there is no child corresponding to token Not
fn Not(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Not, 0)
}

}

impl<'input> Unary_connectiveContextAttrs<'input> for Unary_connectiveContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unary_connective(&mut self,)
	-> Result<Rc<Unary_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Unary_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 268, RULE_unary_connective);
        let mut _localctx: Rc<Unary_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1319);
			recog.base.match_token(tptp_v7_0_0_0_Not,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type_constant ----------------
pub type Type_constantContextAll<'input> = Type_constantContext<'input>;


pub type Type_constantContext<'input> = BaseParserRuleContext<'input,Type_constantContextExt<'input>>;

#[derive(Clone)]
pub struct Type_constantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Type_constantContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Type_constantContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type_constant(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_type_constant(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Type_constantContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_type_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_constantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_constant }
}
antlr4rust::tid!{Type_constantContextExt<'a>}

impl<'input> Type_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Type_constantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_constantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Type_constantContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Type_constantContextExt<'input>>{

fn type_functor(&self) -> Option<Rc<Type_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Type_constantContextAttrs<'input> for Type_constantContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_constant(&mut self,)
	-> Result<Rc<Type_constantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_constantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 270, RULE_type_constant);
        let mut _localctx: Rc<Type_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type_functor*/
			recog.base.set_state(1321);
			recog.type_functor()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type_functor ----------------
pub type Type_functorContextAll<'input> = Type_functorContext<'input>;


pub type Type_functorContext<'input> = BaseParserRuleContext<'input,Type_functorContextExt<'input>>;

#[derive(Clone)]
pub struct Type_functorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Type_functorContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Type_functorContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type_functor(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_type_functor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Type_functorContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_type_functor(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_functorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_functor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_functor }
}
antlr4rust::tid!{Type_functorContextExt<'a>}

impl<'input> Type_functorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Type_functorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_functorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Type_functorContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Type_functorContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Type_functorContextAttrs<'input> for Type_functorContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_functor(&mut self,)
	-> Result<Rc<Type_functorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_functorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 272, RULE_type_functor);
        let mut _localctx: Rc<Type_functorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1323);
			recog.atomic_word()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defined_type ----------------
pub type Defined_typeContextAll<'input> = Defined_typeContext<'input>;


pub type Defined_typeContext<'input> = BaseParserRuleContext<'input,Defined_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Defined_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Defined_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Defined_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defined_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_defined_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Defined_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_defined_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_type }
}
antlr4rust::tid!{Defined_typeContextExt<'a>}

impl<'input> Defined_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Defined_typeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Dollar_word
/// Returns `None` if there is no child corresponding to token Dollar_word
fn Dollar_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Dollar_word, 0)
}

}

impl<'input> Defined_typeContextAttrs<'input> for Defined_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defined_type(&mut self,)
	-> Result<Rc<Defined_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 274, RULE_defined_type);
        let mut _localctx: Rc<Defined_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1325);
			recog.base.match_token(tptp_v7_0_0_0_Dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- system_type ----------------
pub type System_typeContextAll<'input> = System_typeContext<'input>;


pub type System_typeContext<'input> = BaseParserRuleContext<'input,System_typeContextExt<'input>>;

#[derive(Clone)]
pub struct System_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for System_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for System_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_system_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_system_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for System_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_system_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for System_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_system_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_system_type }
}
antlr4rust::tid!{System_typeContextExt<'a>}

impl<'input> System_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<System_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,System_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait System_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<System_typeContextExt<'input>>{

fn atomic_system_word(&self) -> Option<Rc<Atomic_system_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> System_typeContextAttrs<'input> for System_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn system_type(&mut self,)
	-> Result<Rc<System_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = System_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 276, RULE_system_type);
        let mut _localctx: Rc<System_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atomic_system_word*/
			recog.base.set_state(1327);
			recog.atomic_system_word()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- atom ----------------
pub type AtomContextAll<'input> = AtomContext<'input>;


pub type AtomContext<'input> = BaseParserRuleContext<'input,AtomContextExt<'input>>;

#[derive(Clone)]
pub struct AtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for AtomContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for AtomContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_atom(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_atom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for AtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}
antlr4rust::tid!{AtomContextExt<'a>}

impl<'input> AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AtomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AtomContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<AtomContextExt<'input>>{

fn untyped_atom(&self) -> Option<Rc<Untyped_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_constant(&self) -> Option<Rc<Defined_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atom(&mut self,)
	-> Result<Rc<AtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 278, RULE_atom);
        let mut _localctx: Rc<AtomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1331);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Dollar_dollar_word |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule untyped_atom*/
					recog.base.set_state(1329);
					recog.untyped_atom()?;

					}
				}

			tptp_v7_0_0_0_Dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(1330);
					recog.defined_constant()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- untyped_atom ----------------
pub type Untyped_atomContextAll<'input> = Untyped_atomContext<'input>;


pub type Untyped_atomContext<'input> = BaseParserRuleContext<'input,Untyped_atomContextExt<'input>>;

#[derive(Clone)]
pub struct Untyped_atomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Untyped_atomContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Untyped_atomContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_untyped_atom(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_untyped_atom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Untyped_atomContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_untyped_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for Untyped_atomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_untyped_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_untyped_atom }
}
antlr4rust::tid!{Untyped_atomContextExt<'a>}

impl<'input> Untyped_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Untyped_atomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Untyped_atomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Untyped_atomContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Untyped_atomContextExt<'input>>{

fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn system_constant(&self) -> Option<Rc<System_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Untyped_atomContextAttrs<'input> for Untyped_atomContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn untyped_atom(&mut self,)
	-> Result<Rc<Untyped_atomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Untyped_atomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 280, RULE_untyped_atom);
        let mut _localctx: Rc<Untyped_atomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1335);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule constant*/
					recog.base.set_state(1333);
					recog.constant()?;

					}
				}

			tptp_v7_0_0_0_Dollar_dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule system_constant*/
					recog.base.set_state(1334);
					recog.system_constant()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defined_proposition ----------------
pub type Defined_propositionContextAll<'input> = Defined_propositionContext<'input>;


pub type Defined_propositionContext<'input> = BaseParserRuleContext<'input,Defined_propositionContextExt<'input>>;

#[derive(Clone)]
pub struct Defined_propositionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Defined_propositionContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Defined_propositionContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defined_proposition(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_defined_proposition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Defined_propositionContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_defined_proposition(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_propositionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_proposition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_proposition }
}
antlr4rust::tid!{Defined_propositionContextExt<'a>}

impl<'input> Defined_propositionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_propositionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_propositionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_propositionContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Defined_propositionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Dollar_word
/// Returns `None` if there is no child corresponding to token Dollar_word
fn Dollar_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Dollar_word, 0)
}

}

impl<'input> Defined_propositionContextAttrs<'input> for Defined_propositionContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defined_proposition(&mut self,)
	-> Result<Rc<Defined_propositionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_propositionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 282, RULE_defined_proposition);
        let mut _localctx: Rc<Defined_propositionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1337);
			recog.base.match_token(tptp_v7_0_0_0_Dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defined_predicate ----------------
pub type Defined_predicateContextAll<'input> = Defined_predicateContext<'input>;


pub type Defined_predicateContext<'input> = BaseParserRuleContext<'input,Defined_predicateContextExt<'input>>;

#[derive(Clone)]
pub struct Defined_predicateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Defined_predicateContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Defined_predicateContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defined_predicate(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_defined_predicate(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Defined_predicateContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_defined_predicate(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_predicateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_predicate }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_predicate }
}
antlr4rust::tid!{Defined_predicateContextExt<'a>}

impl<'input> Defined_predicateContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_predicateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_predicateContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_predicateContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Defined_predicateContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Dollar_word
/// Returns `None` if there is no child corresponding to token Dollar_word
fn Dollar_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Dollar_word, 0)
}

}

impl<'input> Defined_predicateContextAttrs<'input> for Defined_predicateContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defined_predicate(&mut self,)
	-> Result<Rc<Defined_predicateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_predicateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 284, RULE_defined_predicate);
        let mut _localctx: Rc<Defined_predicateContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1339);
			recog.base.match_token(tptp_v7_0_0_0_Dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defined_infix_pred ----------------
pub type Defined_infix_predContextAll<'input> = Defined_infix_predContext<'input>;


pub type Defined_infix_predContext<'input> = BaseParserRuleContext<'input,Defined_infix_predContextExt<'input>>;

#[derive(Clone)]
pub struct Defined_infix_predContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Defined_infix_predContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Defined_infix_predContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defined_infix_pred(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_defined_infix_pred(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Defined_infix_predContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_defined_infix_pred(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_infix_predContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_infix_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_infix_pred }
}
antlr4rust::tid!{Defined_infix_predContextExt<'a>}

impl<'input> Defined_infix_predContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_infix_predContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_infix_predContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_infix_predContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Defined_infix_predContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Infix_equality
/// Returns `None` if there is no child corresponding to token Infix_equality
fn Infix_equality(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Infix_equality, 0)
}
/// Retrieves first TerminalNode corresponding to token Assignment
/// Returns `None` if there is no child corresponding to token Assignment
fn Assignment(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Assignment, 0)
}

}

impl<'input> Defined_infix_predContextAttrs<'input> for Defined_infix_predContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defined_infix_pred(&mut self,)
	-> Result<Rc<Defined_infix_predContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_infix_predContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 286, RULE_defined_infix_pred);
        let mut _localctx: Rc<Defined_infix_predContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1341);
			_la = recog.base.input.la(1);
			if { !(_la==tptp_v7_0_0_0_Infix_equality || _la==tptp_v7_0_0_0_Assignment) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constant ----------------
pub type ConstantContextAll<'input> = ConstantContext<'input>;


pub type ConstantContext<'input> = BaseParserRuleContext<'input,ConstantContextExt<'input>>;

#[derive(Clone)]
pub struct ConstantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for ConstantContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for ConstantContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constant(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_constant(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for ConstantContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}
antlr4rust::tid!{ConstantContextExt<'a>}

impl<'input> ConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ConstantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ConstantContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<ConstantContextExt<'input>>{

fn functor(&self) -> Option<Rc<FunctorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConstantContextAttrs<'input> for ConstantContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constant(&mut self,)
	-> Result<Rc<ConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 288, RULE_constant);
        let mut _localctx: Rc<ConstantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule functor*/
			recog.base.set_state(1343);
			recog.functor()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functor ----------------
pub type FunctorContextAll<'input> = FunctorContext<'input>;


pub type FunctorContext<'input> = BaseParserRuleContext<'input,FunctorContextExt<'input>>;

#[derive(Clone)]
pub struct FunctorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for FunctorContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for FunctorContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functor(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_functor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for FunctorContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_functor(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functor }
}
antlr4rust::tid!{FunctorContextExt<'a>}

impl<'input> FunctorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FunctorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FunctorContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<FunctorContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctorContextAttrs<'input> for FunctorContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functor(&mut self,)
	-> Result<Rc<FunctorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 290, RULE_functor);
        let mut _localctx: Rc<FunctorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1345);
			recog.atomic_word()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- system_constant ----------------
pub type System_constantContextAll<'input> = System_constantContext<'input>;


pub type System_constantContext<'input> = BaseParserRuleContext<'input,System_constantContextExt<'input>>;

#[derive(Clone)]
pub struct System_constantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for System_constantContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for System_constantContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_system_constant(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_system_constant(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for System_constantContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_system_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for System_constantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_system_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_system_constant }
}
antlr4rust::tid!{System_constantContextExt<'a>}

impl<'input> System_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<System_constantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,System_constantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait System_constantContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<System_constantContextExt<'input>>{

fn system_functor(&self) -> Option<Rc<System_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> System_constantContextAttrs<'input> for System_constantContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn system_constant(&mut self,)
	-> Result<Rc<System_constantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = System_constantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 292, RULE_system_constant);
        let mut _localctx: Rc<System_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule system_functor*/
			recog.base.set_state(1347);
			recog.system_functor()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- system_functor ----------------
pub type System_functorContextAll<'input> = System_functorContext<'input>;


pub type System_functorContext<'input> = BaseParserRuleContext<'input,System_functorContextExt<'input>>;

#[derive(Clone)]
pub struct System_functorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for System_functorContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for System_functorContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_system_functor(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_system_functor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for System_functorContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_system_functor(self);
	}
}

impl<'input> CustomRuleContext<'input> for System_functorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_system_functor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_system_functor }
}
antlr4rust::tid!{System_functorContextExt<'a>}

impl<'input> System_functorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<System_functorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,System_functorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait System_functorContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<System_functorContextExt<'input>>{

fn atomic_system_word(&self) -> Option<Rc<Atomic_system_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> System_functorContextAttrs<'input> for System_functorContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn system_functor(&mut self,)
	-> Result<Rc<System_functorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = System_functorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 294, RULE_system_functor);
        let mut _localctx: Rc<System_functorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atomic_system_word*/
			recog.base.set_state(1349);
			recog.atomic_system_word()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defined_constant ----------------
pub type Defined_constantContextAll<'input> = Defined_constantContext<'input>;


pub type Defined_constantContext<'input> = BaseParserRuleContext<'input,Defined_constantContextExt<'input>>;

#[derive(Clone)]
pub struct Defined_constantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Defined_constantContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Defined_constantContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defined_constant(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_defined_constant(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Defined_constantContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_defined_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_constantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_constant }
}
antlr4rust::tid!{Defined_constantContextExt<'a>}

impl<'input> Defined_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_constantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_constantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_constantContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Defined_constantContextExt<'input>>{

fn defined_functor(&self) -> Option<Rc<Defined_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Defined_constantContextAttrs<'input> for Defined_constantContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defined_constant(&mut self,)
	-> Result<Rc<Defined_constantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_constantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 296, RULE_defined_constant);
        let mut _localctx: Rc<Defined_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule defined_functor*/
			recog.base.set_state(1351);
			recog.defined_functor()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defined_functor ----------------
pub type Defined_functorContextAll<'input> = Defined_functorContext<'input>;


pub type Defined_functorContext<'input> = BaseParserRuleContext<'input,Defined_functorContextExt<'input>>;

#[derive(Clone)]
pub struct Defined_functorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Defined_functorContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Defined_functorContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defined_functor(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_defined_functor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Defined_functorContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_defined_functor(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_functorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_functor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_functor }
}
antlr4rust::tid!{Defined_functorContextExt<'a>}

impl<'input> Defined_functorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_functorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_functorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_functorContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Defined_functorContextExt<'input>>{

fn atomic_defined_word(&self) -> Option<Rc<Atomic_defined_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Defined_functorContextAttrs<'input> for Defined_functorContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defined_functor(&mut self,)
	-> Result<Rc<Defined_functorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_functorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 298, RULE_defined_functor);
        let mut _localctx: Rc<Defined_functorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atomic_defined_word*/
			recog.base.set_state(1353);
			recog.atomic_defined_word()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defined_term ----------------
pub type Defined_termContextAll<'input> = Defined_termContext<'input>;


pub type Defined_termContext<'input> = BaseParserRuleContext<'input,Defined_termContextExt<'input>>;

#[derive(Clone)]
pub struct Defined_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Defined_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Defined_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defined_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_defined_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Defined_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_defined_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_term }
}
antlr4rust::tid!{Defined_termContextExt<'a>}

impl<'input> Defined_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Defined_termContextExt<'input>>{

fn number(&self) -> Option<Rc<NumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Distinct_object
/// Returns `None` if there is no child corresponding to token Distinct_object
fn Distinct_object(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Distinct_object, 0)
}

}

impl<'input> Defined_termContextAttrs<'input> for Defined_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defined_term(&mut self,)
	-> Result<Rc<Defined_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 300, RULE_defined_term);
        let mut _localctx: Rc<Defined_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1357);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Real |tptp_v7_0_0_0_Rational |tptp_v7_0_0_0_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule number*/
					recog.base.set_state(1355);
					recog.number()?;

					}
				}

			tptp_v7_0_0_0_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1356);
					recog.base.match_token(tptp_v7_0_0_0_Distinct_object,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;


pub type VariableContext<'input> = BaseParserRuleContext<'input,VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for VariableContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variable(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_variable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr4rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait VariableContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<VariableContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Upper_word
/// Returns `None` if there is no child corresponding to token Upper_word
fn Upper_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Upper_word, 0)
}

}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable(&mut self,)
	-> Result<Rc<VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 302, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1359);
			recog.base.match_token(tptp_v7_0_0_0_Upper_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- source ----------------
pub type SourceContextAll<'input> = SourceContext<'input>;


pub type SourceContext<'input> = BaseParserRuleContext<'input,SourceContextExt<'input>>;

#[derive(Clone)]
pub struct SourceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for SourceContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for SourceContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_source(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_source(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for SourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for SourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_source }
}
antlr4rust::tid!{SourceContextExt<'a>}

impl<'input> SourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SourceContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<SourceContextExt<'input>>{

fn dag_source(&self) -> Option<Rc<Dag_sourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn internal_source(&self) -> Option<Rc<Internal_sourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn external_source(&self) -> Option<Rc<External_sourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Lower_word
/// Returns `None` if there is no child corresponding to token Lower_word
fn Lower_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Lower_word, 0)
}
fn sources(&self) -> Option<Rc<SourcesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SourceContextAttrs<'input> for SourceContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn source(&mut self,)
	-> Result<Rc<SourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 304, RULE_source);
        let mut _localctx: Rc<SourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1369);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(98,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule dag_source*/
					recog.base.set_state(1361);
					recog.dag_source()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule internal_source*/
					recog.base.set_state(1362);
					recog.internal_source()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule external_source*/
					recog.base.set_state(1363);
					recog.external_source()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1364);
					recog.base.match_token(tptp_v7_0_0_0_Lower_word,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1365);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule sources*/
					recog.base.set_state(1366);
					recog.sources()?;

					recog.base.set_state(1367);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- sources ----------------
pub type SourcesContextAll<'input> = SourcesContext<'input>;


pub type SourcesContext<'input> = BaseParserRuleContext<'input,SourcesContextExt<'input>>;

#[derive(Clone)]
pub struct SourcesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for SourcesContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for SourcesContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sources(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_sources(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for SourcesContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_sources(self);
	}
}

impl<'input> CustomRuleContext<'input> for SourcesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sources }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sources }
}
antlr4rust::tid!{SourcesContextExt<'a>}

impl<'input> SourcesContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SourcesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SourcesContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SourcesContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<SourcesContextExt<'input>>{

fn source_all(&self) ->  Vec<Rc<SourceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn source(&self, i: usize) -> Option<Rc<SourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SourcesContextAttrs<'input> for SourcesContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sources(&mut self,)
	-> Result<Rc<SourcesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SourcesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 306, RULE_sources);
        let mut _localctx: Rc<SourcesContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule source*/
			recog.base.set_state(1371);
			recog.source()?;

			recog.base.set_state(1376);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1372);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule source*/
				recog.base.set_state(1373);
				recog.source()?;

				}
				}
				recog.base.set_state(1378);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- dag_source ----------------
pub type Dag_sourceContextAll<'input> = Dag_sourceContext<'input>;


pub type Dag_sourceContext<'input> = BaseParserRuleContext<'input,Dag_sourceContextExt<'input>>;

#[derive(Clone)]
pub struct Dag_sourceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Dag_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Dag_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_dag_source(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_dag_source(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Dag_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_dag_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for Dag_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dag_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dag_source }
}
antlr4rust::tid!{Dag_sourceContextExt<'a>}

impl<'input> Dag_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Dag_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Dag_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Dag_sourceContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Dag_sourceContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inference_record(&self) -> Option<Rc<Inference_recordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Dag_sourceContextAttrs<'input> for Dag_sourceContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dag_source(&mut self,)
	-> Result<Rc<Dag_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Dag_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 308, RULE_dag_source);
        let mut _localctx: Rc<Dag_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1381);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Integer |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule name*/
					recog.base.set_state(1379);
					recog.name()?;

					}
				}

			tptp_v7_0_0_0_T__26 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule inference_record*/
					recog.base.set_state(1380);
					recog.inference_record()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inference_record ----------------
pub type Inference_recordContextAll<'input> = Inference_recordContext<'input>;


pub type Inference_recordContext<'input> = BaseParserRuleContext<'input,Inference_recordContextExt<'input>>;

#[derive(Clone)]
pub struct Inference_recordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Inference_recordContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Inference_recordContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inference_record(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_inference_record(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Inference_recordContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_inference_record(self);
	}
}

impl<'input> CustomRuleContext<'input> for Inference_recordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inference_record }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inference_record }
}
antlr4rust::tid!{Inference_recordContextExt<'a>}

impl<'input> Inference_recordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Inference_recordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Inference_recordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Inference_recordContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Inference_recordContextExt<'input>>{

fn inference_rule(&self) -> Option<Rc<Inference_ruleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn useful_info(&self) -> Option<Rc<Useful_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inference_parents(&self) -> Option<Rc<Inference_parentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Inference_recordContextAttrs<'input> for Inference_recordContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inference_record(&mut self,)
	-> Result<Rc<Inference_recordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Inference_recordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 310, RULE_inference_record);
        let mut _localctx: Rc<Inference_recordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1383);
			recog.base.match_token(tptp_v7_0_0_0_T__26,&mut recog.err_handler)?;

			/*InvokeRule inference_rule*/
			recog.base.set_state(1384);
			recog.inference_rule()?;

			recog.base.set_state(1385);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule useful_info*/
			recog.base.set_state(1386);
			recog.useful_info()?;

			recog.base.set_state(1387);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule inference_parents*/
			recog.base.set_state(1388);
			recog.inference_parents()?;

			recog.base.set_state(1389);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inference_rule ----------------
pub type Inference_ruleContextAll<'input> = Inference_ruleContext<'input>;


pub type Inference_ruleContext<'input> = BaseParserRuleContext<'input,Inference_ruleContextExt<'input>>;

#[derive(Clone)]
pub struct Inference_ruleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Inference_ruleContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Inference_ruleContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inference_rule(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_inference_rule(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Inference_ruleContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_inference_rule(self);
	}
}

impl<'input> CustomRuleContext<'input> for Inference_ruleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inference_rule }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inference_rule }
}
antlr4rust::tid!{Inference_ruleContextExt<'a>}

impl<'input> Inference_ruleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Inference_ruleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Inference_ruleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Inference_ruleContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Inference_ruleContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Inference_ruleContextAttrs<'input> for Inference_ruleContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inference_rule(&mut self,)
	-> Result<Rc<Inference_ruleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Inference_ruleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 312, RULE_inference_rule);
        let mut _localctx: Rc<Inference_ruleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1391);
			recog.atomic_word()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inference_parents ----------------
pub type Inference_parentsContextAll<'input> = Inference_parentsContext<'input>;


pub type Inference_parentsContext<'input> = BaseParserRuleContext<'input,Inference_parentsContextExt<'input>>;

#[derive(Clone)]
pub struct Inference_parentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Inference_parentsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Inference_parentsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inference_parents(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_inference_parents(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Inference_parentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_inference_parents(self);
	}
}

impl<'input> CustomRuleContext<'input> for Inference_parentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inference_parents }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inference_parents }
}
antlr4rust::tid!{Inference_parentsContextExt<'a>}

impl<'input> Inference_parentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Inference_parentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Inference_parentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Inference_parentsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Inference_parentsContextExt<'input>>{

fn parent_list(&self) -> Option<Rc<Parent_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Inference_parentsContextAttrs<'input> for Inference_parentsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inference_parents(&mut self,)
	-> Result<Rc<Inference_parentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Inference_parentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 314, RULE_inference_parents);
        let mut _localctx: Rc<Inference_parentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1398);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1393);
					recog.base.match_token(tptp_v7_0_0_0_T__16,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1394);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule parent_list*/
					recog.base.set_state(1395);
					recog.parent_list()?;

					recog.base.set_state(1396);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parent_list ----------------
pub type Parent_listContextAll<'input> = Parent_listContext<'input>;


pub type Parent_listContext<'input> = BaseParserRuleContext<'input,Parent_listContextExt<'input>>;

#[derive(Clone)]
pub struct Parent_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Parent_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Parent_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parent_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_parent_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Parent_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_parent_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Parent_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parent_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parent_list }
}
antlr4rust::tid!{Parent_listContextExt<'a>}

impl<'input> Parent_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Parent_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Parent_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Parent_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Parent_listContextExt<'input>>{

fn parent_info_all(&self) ->  Vec<Rc<Parent_infoContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parent_info(&self, i: usize) -> Option<Rc<Parent_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Parent_listContextAttrs<'input> for Parent_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parent_list(&mut self,)
	-> Result<Rc<Parent_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Parent_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 316, RULE_parent_list);
        let mut _localctx: Rc<Parent_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule parent_info*/
			recog.base.set_state(1400);
			recog.parent_info()?;

			recog.base.set_state(1405);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1401);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule parent_info*/
				recog.base.set_state(1402);
				recog.parent_info()?;

				}
				}
				recog.base.set_state(1407);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parent_info ----------------
pub type Parent_infoContextAll<'input> = Parent_infoContext<'input>;


pub type Parent_infoContext<'input> = BaseParserRuleContext<'input,Parent_infoContextExt<'input>>;

#[derive(Clone)]
pub struct Parent_infoContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Parent_infoContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Parent_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parent_info(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_parent_info(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Parent_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_parent_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for Parent_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parent_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parent_info }
}
antlr4rust::tid!{Parent_infoContextExt<'a>}

impl<'input> Parent_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Parent_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Parent_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Parent_infoContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Parent_infoContextExt<'input>>{

fn source(&self) -> Option<Rc<SourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parent_details(&self) -> Option<Rc<Parent_detailsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Parent_infoContextAttrs<'input> for Parent_infoContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parent_info(&mut self,)
	-> Result<Rc<Parent_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Parent_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 318, RULE_parent_info);
        let mut _localctx: Rc<Parent_infoContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule source*/
			recog.base.set_state(1408);
			recog.source()?;

			recog.base.set_state(1410);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__13 {
				{
				/*InvokeRule parent_details*/
				recog.base.set_state(1409);
				recog.parent_details()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parent_details ----------------
pub type Parent_detailsContextAll<'input> = Parent_detailsContext<'input>;


pub type Parent_detailsContext<'input> = BaseParserRuleContext<'input,Parent_detailsContextExt<'input>>;

#[derive(Clone)]
pub struct Parent_detailsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Parent_detailsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Parent_detailsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parent_details(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_parent_details(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Parent_detailsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_parent_details(self);
	}
}

impl<'input> CustomRuleContext<'input> for Parent_detailsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parent_details }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parent_details }
}
antlr4rust::tid!{Parent_detailsContextExt<'a>}

impl<'input> Parent_detailsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Parent_detailsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Parent_detailsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Parent_detailsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Parent_detailsContextExt<'input>>{

fn general_list(&self) -> Option<Rc<General_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Parent_detailsContextAttrs<'input> for Parent_detailsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parent_details(&mut self,)
	-> Result<Rc<Parent_detailsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Parent_detailsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 320, RULE_parent_details);
        let mut _localctx: Rc<Parent_detailsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1412);
			recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

			/*InvokeRule general_list*/
			recog.base.set_state(1413);
			recog.general_list()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- internal_source ----------------
pub type Internal_sourceContextAll<'input> = Internal_sourceContext<'input>;


pub type Internal_sourceContext<'input> = BaseParserRuleContext<'input,Internal_sourceContextExt<'input>>;

#[derive(Clone)]
pub struct Internal_sourceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Internal_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Internal_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_internal_source(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_internal_source(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Internal_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_internal_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for Internal_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_internal_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_internal_source }
}
antlr4rust::tid!{Internal_sourceContextExt<'a>}

impl<'input> Internal_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Internal_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Internal_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Internal_sourceContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Internal_sourceContextExt<'input>>{

fn intro_type(&self) -> Option<Rc<Intro_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn optional_info(&self) -> Option<Rc<Optional_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Internal_sourceContextAttrs<'input> for Internal_sourceContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn internal_source(&mut self,)
	-> Result<Rc<Internal_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Internal_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 322, RULE_internal_source);
        let mut _localctx: Rc<Internal_sourceContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1415);
			recog.base.match_token(tptp_v7_0_0_0_T__27,&mut recog.err_handler)?;

			/*InvokeRule intro_type*/
			recog.base.set_state(1416);
			recog.intro_type()?;

			recog.base.set_state(1418);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule optional_info*/
				recog.base.set_state(1417);
				recog.optional_info()?;

				}
			}

			recog.base.set_state(1420);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- intro_type ----------------
pub type Intro_typeContextAll<'input> = Intro_typeContext<'input>;


pub type Intro_typeContext<'input> = BaseParserRuleContext<'input,Intro_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Intro_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Intro_typeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Intro_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_intro_type(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_intro_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Intro_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_intro_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Intro_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intro_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intro_type }
}
antlr4rust::tid!{Intro_typeContextExt<'a>}

impl<'input> Intro_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Intro_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Intro_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Intro_typeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Intro_typeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Lower_word
/// Returns `None` if there is no child corresponding to token Lower_word
fn Lower_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Lower_word, 0)
}

}

impl<'input> Intro_typeContextAttrs<'input> for Intro_typeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn intro_type(&mut self,)
	-> Result<Rc<Intro_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Intro_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 324, RULE_intro_type);
        let mut _localctx: Rc<Intro_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1422);
			recog.base.match_token(tptp_v7_0_0_0_Lower_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- external_source ----------------
pub type External_sourceContextAll<'input> = External_sourceContext<'input>;


pub type External_sourceContext<'input> = BaseParserRuleContext<'input,External_sourceContextExt<'input>>;

#[derive(Clone)]
pub struct External_sourceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for External_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for External_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_external_source(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_external_source(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for External_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_external_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for External_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_external_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_external_source }
}
antlr4rust::tid!{External_sourceContextExt<'a>}

impl<'input> External_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<External_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,External_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait External_sourceContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<External_sourceContextExt<'input>>{

fn file_source(&self) -> Option<Rc<File_sourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn theory(&self) -> Option<Rc<TheoryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn creator_source(&self) -> Option<Rc<Creator_sourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> External_sourceContextAttrs<'input> for External_sourceContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn external_source(&mut self,)
	-> Result<Rc<External_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = External_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 326, RULE_external_source);
        let mut _localctx: Rc<External_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1427);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__28 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule file_source*/
					recog.base.set_state(1424);
					recog.file_source()?;

					}
				}

			tptp_v7_0_0_0_T__29 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule theory*/
					recog.base.set_state(1425);
					recog.theory()?;

					}
				}

			tptp_v7_0_0_0_T__30 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule creator_source*/
					recog.base.set_state(1426);
					recog.creator_source()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- file_source ----------------
pub type File_sourceContextAll<'input> = File_sourceContext<'input>;


pub type File_sourceContext<'input> = BaseParserRuleContext<'input,File_sourceContextExt<'input>>;

#[derive(Clone)]
pub struct File_sourceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for File_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for File_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_file_source(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_file_source(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for File_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_file_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for File_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file_source }
}
antlr4rust::tid!{File_sourceContextExt<'a>}

impl<'input> File_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<File_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,File_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait File_sourceContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<File_sourceContextExt<'input>>{

fn file_name(&self) -> Option<Rc<File_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn file_info(&self) -> Option<Rc<File_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> File_sourceContextAttrs<'input> for File_sourceContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn file_source(&mut self,)
	-> Result<Rc<File_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = File_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 328, RULE_file_source);
        let mut _localctx: Rc<File_sourceContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1429);
			recog.base.match_token(tptp_v7_0_0_0_T__28,&mut recog.err_handler)?;

			/*InvokeRule file_name*/
			recog.base.set_state(1430);
			recog.file_name()?;

			recog.base.set_state(1432);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule file_info*/
				recog.base.set_state(1431);
				recog.file_info()?;

				}
			}

			recog.base.set_state(1434);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- file_info ----------------
pub type File_infoContextAll<'input> = File_infoContext<'input>;


pub type File_infoContext<'input> = BaseParserRuleContext<'input,File_infoContextExt<'input>>;

#[derive(Clone)]
pub struct File_infoContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for File_infoContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for File_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_file_info(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_file_info(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for File_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_file_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for File_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file_info }
}
antlr4rust::tid!{File_infoContextExt<'a>}

impl<'input> File_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<File_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,File_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait File_infoContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<File_infoContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> File_infoContextAttrs<'input> for File_infoContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn file_info(&mut self,)
	-> Result<Rc<File_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = File_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 330, RULE_file_info);
        let mut _localctx: Rc<File_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1436);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(1437);
			recog.name()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- theory ----------------
pub type TheoryContextAll<'input> = TheoryContext<'input>;


pub type TheoryContext<'input> = BaseParserRuleContext<'input,TheoryContextExt<'input>>;

#[derive(Clone)]
pub struct TheoryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for TheoryContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for TheoryContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_theory(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_theory(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for TheoryContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_theory(self);
	}
}

impl<'input> CustomRuleContext<'input> for TheoryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_theory }
	//fn type_rule_index() -> usize where Self: Sized { RULE_theory }
}
antlr4rust::tid!{TheoryContextExt<'a>}

impl<'input> TheoryContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TheoryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TheoryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TheoryContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<TheoryContextExt<'input>>{

fn theory_name(&self) -> Option<Rc<Theory_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn optional_info(&self) -> Option<Rc<Optional_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TheoryContextAttrs<'input> for TheoryContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn theory(&mut self,)
	-> Result<Rc<TheoryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TheoryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 332, RULE_theory);
        let mut _localctx: Rc<TheoryContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1439);
			recog.base.match_token(tptp_v7_0_0_0_T__29,&mut recog.err_handler)?;

			/*InvokeRule theory_name*/
			recog.base.set_state(1440);
			recog.theory_name()?;

			recog.base.set_state(1442);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule optional_info*/
				recog.base.set_state(1441);
				recog.optional_info()?;

				}
			}

			recog.base.set_state(1444);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- theory_name ----------------
pub type Theory_nameContextAll<'input> = Theory_nameContext<'input>;


pub type Theory_nameContext<'input> = BaseParserRuleContext<'input,Theory_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Theory_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Theory_nameContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Theory_nameContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_theory_name(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_theory_name(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Theory_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_theory_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Theory_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_theory_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_theory_name }
}
antlr4rust::tid!{Theory_nameContextExt<'a>}

impl<'input> Theory_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Theory_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Theory_nameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Theory_nameContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Theory_nameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Lower_word
/// Returns `None` if there is no child corresponding to token Lower_word
fn Lower_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Lower_word, 0)
}

}

impl<'input> Theory_nameContextAttrs<'input> for Theory_nameContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn theory_name(&mut self,)
	-> Result<Rc<Theory_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Theory_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 334, RULE_theory_name);
        let mut _localctx: Rc<Theory_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1446);
			recog.base.match_token(tptp_v7_0_0_0_Lower_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- creator_source ----------------
pub type Creator_sourceContextAll<'input> = Creator_sourceContext<'input>;


pub type Creator_sourceContext<'input> = BaseParserRuleContext<'input,Creator_sourceContextExt<'input>>;

#[derive(Clone)]
pub struct Creator_sourceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Creator_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Creator_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_creator_source(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_creator_source(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Creator_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_creator_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for Creator_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_creator_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_creator_source }
}
antlr4rust::tid!{Creator_sourceContextExt<'a>}

impl<'input> Creator_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Creator_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Creator_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Creator_sourceContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Creator_sourceContextExt<'input>>{

fn creator_name(&self) -> Option<Rc<Creator_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn optional_info(&self) -> Option<Rc<Optional_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Creator_sourceContextAttrs<'input> for Creator_sourceContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn creator_source(&mut self,)
	-> Result<Rc<Creator_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Creator_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 336, RULE_creator_source);
        let mut _localctx: Rc<Creator_sourceContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1448);
			recog.base.match_token(tptp_v7_0_0_0_T__30,&mut recog.err_handler)?;

			/*InvokeRule creator_name*/
			recog.base.set_state(1449);
			recog.creator_name()?;

			recog.base.set_state(1451);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule optional_info*/
				recog.base.set_state(1450);
				recog.optional_info()?;

				}
			}

			recog.base.set_state(1453);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- creator_name ----------------
pub type Creator_nameContextAll<'input> = Creator_nameContext<'input>;


pub type Creator_nameContext<'input> = BaseParserRuleContext<'input,Creator_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Creator_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Creator_nameContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Creator_nameContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_creator_name(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_creator_name(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Creator_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_creator_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Creator_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_creator_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_creator_name }
}
antlr4rust::tid!{Creator_nameContextExt<'a>}

impl<'input> Creator_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Creator_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Creator_nameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Creator_nameContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Creator_nameContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Creator_nameContextAttrs<'input> for Creator_nameContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn creator_name(&mut self,)
	-> Result<Rc<Creator_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Creator_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 338, RULE_creator_name);
        let mut _localctx: Rc<Creator_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1455);
			recog.atomic_word()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- optional_info ----------------
pub type Optional_infoContextAll<'input> = Optional_infoContext<'input>;


pub type Optional_infoContext<'input> = BaseParserRuleContext<'input,Optional_infoContextExt<'input>>;

#[derive(Clone)]
pub struct Optional_infoContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Optional_infoContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Optional_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_optional_info(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_optional_info(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Optional_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_optional_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for Optional_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_optional_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_optional_info }
}
antlr4rust::tid!{Optional_infoContextExt<'a>}

impl<'input> Optional_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Optional_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Optional_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Optional_infoContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Optional_infoContextExt<'input>>{

fn useful_info(&self) -> Option<Rc<Useful_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Optional_infoContextAttrs<'input> for Optional_infoContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn optional_info(&mut self,)
	-> Result<Rc<Optional_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Optional_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 340, RULE_optional_info);
        let mut _localctx: Rc<Optional_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1457);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule useful_info*/
			recog.base.set_state(1458);
			recog.useful_info()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- useful_info ----------------
pub type Useful_infoContextAll<'input> = Useful_infoContext<'input>;


pub type Useful_infoContext<'input> = BaseParserRuleContext<'input,Useful_infoContextExt<'input>>;

#[derive(Clone)]
pub struct Useful_infoContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Useful_infoContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Useful_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_useful_info(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_useful_info(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Useful_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_useful_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for Useful_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_useful_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_useful_info }
}
antlr4rust::tid!{Useful_infoContextExt<'a>}

impl<'input> Useful_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Useful_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Useful_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Useful_infoContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Useful_infoContextExt<'input>>{

fn info_items(&self) -> Option<Rc<Info_itemsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn general_list(&self) -> Option<Rc<General_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Useful_infoContextAttrs<'input> for Useful_infoContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn useful_info(&mut self,)
	-> Result<Rc<Useful_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Useful_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 342, RULE_useful_info);
        let mut _localctx: Rc<Useful_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1466);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(109,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1460);
					recog.base.match_token(tptp_v7_0_0_0_T__16,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1461);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule info_items*/
					recog.base.set_state(1462);
					recog.info_items()?;

					recog.base.set_state(1463);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule general_list*/
					recog.base.set_state(1465);
					recog.general_list()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- info_items ----------------
pub type Info_itemsContextAll<'input> = Info_itemsContext<'input>;


pub type Info_itemsContext<'input> = BaseParserRuleContext<'input,Info_itemsContextExt<'input>>;

#[derive(Clone)]
pub struct Info_itemsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Info_itemsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Info_itemsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_info_items(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_info_items(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Info_itemsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_info_items(self);
	}
}

impl<'input> CustomRuleContext<'input> for Info_itemsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_info_items }
	//fn type_rule_index() -> usize where Self: Sized { RULE_info_items }
}
antlr4rust::tid!{Info_itemsContextExt<'a>}

impl<'input> Info_itemsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Info_itemsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Info_itemsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Info_itemsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Info_itemsContextExt<'input>>{

fn info_item_all(&self) ->  Vec<Rc<Info_itemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn info_item(&self, i: usize) -> Option<Rc<Info_itemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Info_itemsContextAttrs<'input> for Info_itemsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn info_items(&mut self,)
	-> Result<Rc<Info_itemsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Info_itemsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 344, RULE_info_items);
        let mut _localctx: Rc<Info_itemsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule info_item*/
			recog.base.set_state(1468);
			recog.info_item()?;

			recog.base.set_state(1473);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1469);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule info_item*/
				recog.base.set_state(1470);
				recog.info_item()?;

				}
				}
				recog.base.set_state(1475);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- info_item ----------------
pub type Info_itemContextAll<'input> = Info_itemContext<'input>;


pub type Info_itemContext<'input> = BaseParserRuleContext<'input,Info_itemContextExt<'input>>;

#[derive(Clone)]
pub struct Info_itemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Info_itemContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Info_itemContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_info_item(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_info_item(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Info_itemContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_info_item(self);
	}
}

impl<'input> CustomRuleContext<'input> for Info_itemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_info_item }
	//fn type_rule_index() -> usize where Self: Sized { RULE_info_item }
}
antlr4rust::tid!{Info_itemContextExt<'a>}

impl<'input> Info_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Info_itemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Info_itemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Info_itemContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Info_itemContextExt<'input>>{

fn formula_item(&self) -> Option<Rc<Formula_itemContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inference_item(&self) -> Option<Rc<Inference_itemContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn general_function(&self) -> Option<Rc<General_functionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Info_itemContextAttrs<'input> for Info_itemContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn info_item(&mut self,)
	-> Result<Rc<Info_itemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Info_itemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 346, RULE_info_item);
        let mut _localctx: Rc<Info_itemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1479);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(111,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule formula_item*/
					recog.base.set_state(1476);
					recog.formula_item()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule inference_item*/
					recog.base.set_state(1477);
					recog.inference_item()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule general_function*/
					recog.base.set_state(1478);
					recog.general_function()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- formula_item ----------------
pub type Formula_itemContextAll<'input> = Formula_itemContext<'input>;


pub type Formula_itemContext<'input> = BaseParserRuleContext<'input,Formula_itemContextExt<'input>>;

#[derive(Clone)]
pub struct Formula_itemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Formula_itemContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Formula_itemContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_formula_item(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_formula_item(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Formula_itemContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_formula_item(self);
	}
}

impl<'input> CustomRuleContext<'input> for Formula_itemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formula_item }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formula_item }
}
antlr4rust::tid!{Formula_itemContextExt<'a>}

impl<'input> Formula_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Formula_itemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Formula_itemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Formula_itemContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Formula_itemContextExt<'input>>{

fn description_item(&self) -> Option<Rc<Description_itemContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn iquote_item(&self) -> Option<Rc<Iquote_itemContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Formula_itemContextAttrs<'input> for Formula_itemContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn formula_item(&mut self,)
	-> Result<Rc<Formula_itemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Formula_itemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 348, RULE_formula_item);
        let mut _localctx: Rc<Formula_itemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1483);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__31 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule description_item*/
					recog.base.set_state(1481);
					recog.description_item()?;

					}
				}

			tptp_v7_0_0_0_T__32 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule iquote_item*/
					recog.base.set_state(1482);
					recog.iquote_item()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- description_item ----------------
pub type Description_itemContextAll<'input> = Description_itemContext<'input>;


pub type Description_itemContext<'input> = BaseParserRuleContext<'input,Description_itemContextExt<'input>>;

#[derive(Clone)]
pub struct Description_itemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Description_itemContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Description_itemContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_description_item(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_description_item(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Description_itemContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_description_item(self);
	}
}

impl<'input> CustomRuleContext<'input> for Description_itemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_description_item }
	//fn type_rule_index() -> usize where Self: Sized { RULE_description_item }
}
antlr4rust::tid!{Description_itemContextExt<'a>}

impl<'input> Description_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Description_itemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Description_itemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Description_itemContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Description_itemContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Description_itemContextAttrs<'input> for Description_itemContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn description_item(&mut self,)
	-> Result<Rc<Description_itemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Description_itemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 350, RULE_description_item);
        let mut _localctx: Rc<Description_itemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1485);
			recog.base.match_token(tptp_v7_0_0_0_T__31,&mut recog.err_handler)?;

			/*InvokeRule atomic_word*/
			recog.base.set_state(1486);
			recog.atomic_word()?;

			recog.base.set_state(1487);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- iquote_item ----------------
pub type Iquote_itemContextAll<'input> = Iquote_itemContext<'input>;


pub type Iquote_itemContext<'input> = BaseParserRuleContext<'input,Iquote_itemContextExt<'input>>;

#[derive(Clone)]
pub struct Iquote_itemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Iquote_itemContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Iquote_itemContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_iquote_item(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_iquote_item(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Iquote_itemContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_iquote_item(self);
	}
}

impl<'input> CustomRuleContext<'input> for Iquote_itemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_iquote_item }
	//fn type_rule_index() -> usize where Self: Sized { RULE_iquote_item }
}
antlr4rust::tid!{Iquote_itemContextExt<'a>}

impl<'input> Iquote_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Iquote_itemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Iquote_itemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Iquote_itemContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Iquote_itemContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Iquote_itemContextAttrs<'input> for Iquote_itemContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn iquote_item(&mut self,)
	-> Result<Rc<Iquote_itemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Iquote_itemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 352, RULE_iquote_item);
        let mut _localctx: Rc<Iquote_itemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1489);
			recog.base.match_token(tptp_v7_0_0_0_T__32,&mut recog.err_handler)?;

			/*InvokeRule atomic_word*/
			recog.base.set_state(1490);
			recog.atomic_word()?;

			recog.base.set_state(1491);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inference_item ----------------
pub type Inference_itemContextAll<'input> = Inference_itemContext<'input>;


pub type Inference_itemContext<'input> = BaseParserRuleContext<'input,Inference_itemContextExt<'input>>;

#[derive(Clone)]
pub struct Inference_itemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Inference_itemContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Inference_itemContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inference_item(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_inference_item(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Inference_itemContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_inference_item(self);
	}
}

impl<'input> CustomRuleContext<'input> for Inference_itemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inference_item }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inference_item }
}
antlr4rust::tid!{Inference_itemContextExt<'a>}

impl<'input> Inference_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Inference_itemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Inference_itemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Inference_itemContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Inference_itemContextExt<'input>>{

fn inference_status(&self) -> Option<Rc<Inference_statusContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assumptions_record(&self) -> Option<Rc<Assumptions_recordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn new_symbol_record(&self) -> Option<Rc<New_symbol_recordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn refutation(&self) -> Option<Rc<RefutationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Inference_itemContextAttrs<'input> for Inference_itemContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inference_item(&mut self,)
	-> Result<Rc<Inference_itemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Inference_itemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 354, RULE_inference_item);
        let mut _localctx: Rc<Inference_itemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1497);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__33 |tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule inference_status*/
					recog.base.set_state(1493);
					recog.inference_status()?;

					}
				}

			tptp_v7_0_0_0_T__34 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assumptions_record*/
					recog.base.set_state(1494);
					recog.assumptions_record()?;

					}
				}

			tptp_v7_0_0_0_T__36 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule new_symbol_record*/
					recog.base.set_state(1495);
					recog.new_symbol_record()?;

					}
				}

			tptp_v7_0_0_0_T__35 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule refutation*/
					recog.base.set_state(1496);
					recog.refutation()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inference_status ----------------
pub type Inference_statusContextAll<'input> = Inference_statusContext<'input>;


pub type Inference_statusContext<'input> = BaseParserRuleContext<'input,Inference_statusContextExt<'input>>;

#[derive(Clone)]
pub struct Inference_statusContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Inference_statusContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Inference_statusContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inference_status(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_inference_status(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Inference_statusContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_inference_status(self);
	}
}

impl<'input> CustomRuleContext<'input> for Inference_statusContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inference_status }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inference_status }
}
antlr4rust::tid!{Inference_statusContextExt<'a>}

impl<'input> Inference_statusContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Inference_statusContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Inference_statusContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Inference_statusContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Inference_statusContextExt<'input>>{

fn status_value(&self) -> Option<Rc<Status_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inference_info(&self) -> Option<Rc<Inference_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Inference_statusContextAttrs<'input> for Inference_statusContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inference_status(&mut self,)
	-> Result<Rc<Inference_statusContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Inference_statusContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 356, RULE_inference_status);
        let mut _localctx: Rc<Inference_statusContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1504);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__33 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1499);
					recog.base.match_token(tptp_v7_0_0_0_T__33,&mut recog.err_handler)?;

					/*InvokeRule status_value*/
					recog.base.set_state(1500);
					recog.status_value()?;

					recog.base.set_state(1501);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule inference_info*/
					recog.base.set_state(1503);
					recog.inference_info()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- status_value ----------------
pub type Status_valueContextAll<'input> = Status_valueContext<'input>;


pub type Status_valueContext<'input> = BaseParserRuleContext<'input,Status_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Status_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Status_valueContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Status_valueContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_status_value(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_status_value(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Status_valueContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_status_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for Status_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_status_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_status_value }
}
antlr4rust::tid!{Status_valueContextExt<'a>}

impl<'input> Status_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Status_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Status_valueContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Status_valueContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Status_valueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Lower_word
/// Returns `None` if there is no child corresponding to token Lower_word
fn Lower_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Lower_word, 0)
}

}

impl<'input> Status_valueContextAttrs<'input> for Status_valueContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn status_value(&mut self,)
	-> Result<Rc<Status_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Status_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 358, RULE_status_value);
        let mut _localctx: Rc<Status_valueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1506);
			recog.base.match_token(tptp_v7_0_0_0_Lower_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inference_info ----------------
pub type Inference_infoContextAll<'input> = Inference_infoContext<'input>;


pub type Inference_infoContext<'input> = BaseParserRuleContext<'input,Inference_infoContextExt<'input>>;

#[derive(Clone)]
pub struct Inference_infoContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Inference_infoContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Inference_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_inference_info(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_inference_info(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Inference_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_inference_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for Inference_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inference_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inference_info }
}
antlr4rust::tid!{Inference_infoContextExt<'a>}

impl<'input> Inference_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Inference_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Inference_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Inference_infoContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Inference_infoContextExt<'input>>{

fn inference_rule(&self) -> Option<Rc<Inference_ruleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn general_list(&self) -> Option<Rc<General_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Inference_infoContextAttrs<'input> for Inference_infoContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inference_info(&mut self,)
	-> Result<Rc<Inference_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Inference_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 360, RULE_inference_info);
        let mut _localctx: Rc<Inference_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule inference_rule*/
			recog.base.set_state(1508);
			recog.inference_rule()?;

			recog.base.set_state(1509);
			recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

			/*InvokeRule atomic_word*/
			recog.base.set_state(1510);
			recog.atomic_word()?;

			recog.base.set_state(1511);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			/*InvokeRule general_list*/
			recog.base.set_state(1512);
			recog.general_list()?;

			recog.base.set_state(1513);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assumptions_record ----------------
pub type Assumptions_recordContextAll<'input> = Assumptions_recordContext<'input>;


pub type Assumptions_recordContext<'input> = BaseParserRuleContext<'input,Assumptions_recordContextExt<'input>>;

#[derive(Clone)]
pub struct Assumptions_recordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Assumptions_recordContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Assumptions_recordContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assumptions_record(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_assumptions_record(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Assumptions_recordContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_assumptions_record(self);
	}
}

impl<'input> CustomRuleContext<'input> for Assumptions_recordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assumptions_record }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assumptions_record }
}
antlr4rust::tid!{Assumptions_recordContextExt<'a>}

impl<'input> Assumptions_recordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Assumptions_recordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Assumptions_recordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Assumptions_recordContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Assumptions_recordContextExt<'input>>{

fn name_list(&self) -> Option<Rc<Name_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Assumptions_recordContextAttrs<'input> for Assumptions_recordContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assumptions_record(&mut self,)
	-> Result<Rc<Assumptions_recordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Assumptions_recordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 362, RULE_assumptions_record);
        let mut _localctx: Rc<Assumptions_recordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1515);
			recog.base.match_token(tptp_v7_0_0_0_T__34,&mut recog.err_handler)?;

			recog.base.set_state(1516);
			recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

			/*InvokeRule name_list*/
			recog.base.set_state(1517);
			recog.name_list()?;

			recog.base.set_state(1518);
			recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

			recog.base.set_state(1519);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- refutation ----------------
pub type RefutationContextAll<'input> = RefutationContext<'input>;


pub type RefutationContext<'input> = BaseParserRuleContext<'input,RefutationContextExt<'input>>;

#[derive(Clone)]
pub struct RefutationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for RefutationContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for RefutationContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_refutation(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_refutation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for RefutationContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_refutation(self);
	}
}

impl<'input> CustomRuleContext<'input> for RefutationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_refutation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_refutation }
}
antlr4rust::tid!{RefutationContextExt<'a>}

impl<'input> RefutationContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RefutationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RefutationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RefutationContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<RefutationContextExt<'input>>{

fn file_source(&self) -> Option<Rc<File_sourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RefutationContextAttrs<'input> for RefutationContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn refutation(&mut self,)
	-> Result<Rc<RefutationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RefutationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 364, RULE_refutation);
        let mut _localctx: Rc<RefutationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1521);
			recog.base.match_token(tptp_v7_0_0_0_T__35,&mut recog.err_handler)?;

			/*InvokeRule file_source*/
			recog.base.set_state(1522);
			recog.file_source()?;

			recog.base.set_state(1523);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- new_symbol_record ----------------
pub type New_symbol_recordContextAll<'input> = New_symbol_recordContext<'input>;


pub type New_symbol_recordContext<'input> = BaseParserRuleContext<'input,New_symbol_recordContextExt<'input>>;

#[derive(Clone)]
pub struct New_symbol_recordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for New_symbol_recordContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for New_symbol_recordContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_new_symbol_record(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_new_symbol_record(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for New_symbol_recordContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_new_symbol_record(self);
	}
}

impl<'input> CustomRuleContext<'input> for New_symbol_recordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_new_symbol_record }
	//fn type_rule_index() -> usize where Self: Sized { RULE_new_symbol_record }
}
antlr4rust::tid!{New_symbol_recordContextExt<'a>}

impl<'input> New_symbol_recordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<New_symbol_recordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,New_symbol_recordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait New_symbol_recordContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<New_symbol_recordContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn new_symbol_list(&self) -> Option<Rc<New_symbol_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> New_symbol_recordContextAttrs<'input> for New_symbol_recordContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn new_symbol_record(&mut self,)
	-> Result<Rc<New_symbol_recordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = New_symbol_recordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 366, RULE_new_symbol_record);
        let mut _localctx: Rc<New_symbol_recordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1525);
			recog.base.match_token(tptp_v7_0_0_0_T__36,&mut recog.err_handler)?;

			/*InvokeRule atomic_word*/
			recog.base.set_state(1526);
			recog.atomic_word()?;

			recog.base.set_state(1527);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			recog.base.set_state(1528);
			recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

			/*InvokeRule new_symbol_list*/
			recog.base.set_state(1529);
			recog.new_symbol_list()?;

			recog.base.set_state(1530);
			recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

			recog.base.set_state(1531);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- new_symbol_list ----------------
pub type New_symbol_listContextAll<'input> = New_symbol_listContext<'input>;


pub type New_symbol_listContext<'input> = BaseParserRuleContext<'input,New_symbol_listContextExt<'input>>;

#[derive(Clone)]
pub struct New_symbol_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for New_symbol_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for New_symbol_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_new_symbol_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_new_symbol_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for New_symbol_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_new_symbol_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for New_symbol_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_new_symbol_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_new_symbol_list }
}
antlr4rust::tid!{New_symbol_listContextExt<'a>}

impl<'input> New_symbol_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<New_symbol_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,New_symbol_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait New_symbol_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<New_symbol_listContextExt<'input>>{

fn principal_symbol_all(&self) ->  Vec<Rc<Principal_symbolContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn principal_symbol(&self, i: usize) -> Option<Rc<Principal_symbolContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> New_symbol_listContextAttrs<'input> for New_symbol_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn new_symbol_list(&mut self,)
	-> Result<Rc<New_symbol_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = New_symbol_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 368, RULE_new_symbol_list);
        let mut _localctx: Rc<New_symbol_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule principal_symbol*/
			recog.base.set_state(1533);
			recog.principal_symbol()?;

			recog.base.set_state(1538);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1534);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule principal_symbol*/
				recog.base.set_state(1535);
				recog.principal_symbol()?;

				}
				}
				recog.base.set_state(1540);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- principal_symbol ----------------
pub type Principal_symbolContextAll<'input> = Principal_symbolContext<'input>;


pub type Principal_symbolContext<'input> = BaseParserRuleContext<'input,Principal_symbolContextExt<'input>>;

#[derive(Clone)]
pub struct Principal_symbolContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Principal_symbolContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Principal_symbolContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_principal_symbol(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_principal_symbol(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Principal_symbolContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_principal_symbol(self);
	}
}

impl<'input> CustomRuleContext<'input> for Principal_symbolContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_principal_symbol }
	//fn type_rule_index() -> usize where Self: Sized { RULE_principal_symbol }
}
antlr4rust::tid!{Principal_symbolContextExt<'a>}

impl<'input> Principal_symbolContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Principal_symbolContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Principal_symbolContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Principal_symbolContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Principal_symbolContextExt<'input>>{

fn functor(&self) -> Option<Rc<FunctorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Principal_symbolContextAttrs<'input> for Principal_symbolContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn principal_symbol(&mut self,)
	-> Result<Rc<Principal_symbolContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Principal_symbolContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 370, RULE_principal_symbol);
        let mut _localctx: Rc<Principal_symbolContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1543);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule functor*/
					recog.base.set_state(1541);
					recog.functor()?;

					}
				}

			tptp_v7_0_0_0_Upper_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable*/
					recog.base.set_state(1542);
					recog.variable()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- include ----------------
pub type IncludeContextAll<'input> = IncludeContext<'input>;


pub type IncludeContext<'input> = BaseParserRuleContext<'input,IncludeContextExt<'input>>;

#[derive(Clone)]
pub struct IncludeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for IncludeContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for IncludeContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_include(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_include(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for IncludeContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_include(self);
	}
}

impl<'input> CustomRuleContext<'input> for IncludeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_include }
	//fn type_rule_index() -> usize where Self: Sized { RULE_include }
}
antlr4rust::tid!{IncludeContextExt<'a>}

impl<'input> IncludeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IncludeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IncludeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IncludeContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<IncludeContextExt<'input>>{

fn file_name(&self) -> Option<Rc<File_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formula_selection(&self) -> Option<Rc<Formula_selectionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IncludeContextAttrs<'input> for IncludeContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn include(&mut self,)
	-> Result<Rc<IncludeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IncludeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 372, RULE_include);
        let mut _localctx: Rc<IncludeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1545);
			recog.base.match_token(tptp_v7_0_0_0_T__37,&mut recog.err_handler)?;

			/*InvokeRule file_name*/
			recog.base.set_state(1546);
			recog.file_name()?;

			recog.base.set_state(1548);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==tptp_v7_0_0_0_T__1 {
				{
				/*InvokeRule formula_selection*/
				recog.base.set_state(1547);
				recog.formula_selection()?;

				}
			}

			recog.base.set_state(1550);
			recog.base.match_token(tptp_v7_0_0_0_T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- formula_selection ----------------
pub type Formula_selectionContextAll<'input> = Formula_selectionContext<'input>;


pub type Formula_selectionContext<'input> = BaseParserRuleContext<'input,Formula_selectionContextExt<'input>>;

#[derive(Clone)]
pub struct Formula_selectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Formula_selectionContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Formula_selectionContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_formula_selection(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_formula_selection(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Formula_selectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_formula_selection(self);
	}
}

impl<'input> CustomRuleContext<'input> for Formula_selectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formula_selection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formula_selection }
}
antlr4rust::tid!{Formula_selectionContextExt<'a>}

impl<'input> Formula_selectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Formula_selectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Formula_selectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Formula_selectionContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Formula_selectionContextExt<'input>>{

fn name_list(&self) -> Option<Rc<Name_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Formula_selectionContextAttrs<'input> for Formula_selectionContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn formula_selection(&mut self,)
	-> Result<Rc<Formula_selectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Formula_selectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 374, RULE_formula_selection);
        let mut _localctx: Rc<Formula_selectionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1552);
			recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

			recog.base.set_state(1553);
			recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

			/*InvokeRule name_list*/
			recog.base.set_state(1554);
			recog.name_list()?;

			recog.base.set_state(1555);
			recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- name_list ----------------
pub type Name_listContextAll<'input> = Name_listContext<'input>;


pub type Name_listContext<'input> = BaseParserRuleContext<'input,Name_listContextExt<'input>>;

#[derive(Clone)]
pub struct Name_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Name_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Name_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_name_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_name_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Name_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_name_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Name_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_name_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_name_list }
}
antlr4rust::tid!{Name_listContextExt<'a>}

impl<'input> Name_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Name_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Name_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Name_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Name_listContextExt<'input>>{

fn name_all(&self) ->  Vec<Rc<NameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn name(&self, i: usize) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Name_listContextAttrs<'input> for Name_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn name_list(&mut self,)
	-> Result<Rc<Name_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Name_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 376, RULE_name_list);
        let mut _localctx: Rc<Name_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule name*/
			recog.base.set_state(1557);
			recog.name()?;

			recog.base.set_state(1562);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1558);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule name*/
				recog.base.set_state(1559);
				recog.name()?;

				}
				}
				recog.base.set_state(1564);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- general_term ----------------
pub type General_termContextAll<'input> = General_termContext<'input>;


pub type General_termContext<'input> = BaseParserRuleContext<'input,General_termContextExt<'input>>;

#[derive(Clone)]
pub struct General_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for General_termContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for General_termContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_general_term(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_general_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for General_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_general_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_term }
}
antlr4rust::tid!{General_termContextExt<'a>}

impl<'input> General_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_termContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<General_termContextExt<'input>>{

fn general_data(&self) -> Option<Rc<General_dataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn general_term(&self) -> Option<Rc<General_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn general_list(&self) -> Option<Rc<General_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> General_termContextAttrs<'input> for General_termContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn general_term(&mut self,)
	-> Result<Rc<General_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 378, RULE_general_term);
        let mut _localctx: Rc<General_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1571);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(119,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule general_data*/
					recog.base.set_state(1565);
					recog.general_data()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule general_data*/
					recog.base.set_state(1566);
					recog.general_data()?;

					recog.base.set_state(1567);
					recog.base.match_token(tptp_v7_0_0_0_T__13,&mut recog.err_handler)?;

					/*InvokeRule general_term*/
					recog.base.set_state(1568);
					recog.general_term()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule general_list*/
					recog.base.set_state(1570);
					recog.general_list()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- general_data ----------------
pub type General_dataContextAll<'input> = General_dataContext<'input>;


pub type General_dataContext<'input> = BaseParserRuleContext<'input,General_dataContextExt<'input>>;

#[derive(Clone)]
pub struct General_dataContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for General_dataContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for General_dataContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_general_data(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_general_data(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for General_dataContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_general_data(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_dataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_data }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_data }
}
antlr4rust::tid!{General_dataContextExt<'a>}

impl<'input> General_dataContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_dataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_dataContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_dataContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<General_dataContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn general_function(&self) -> Option<Rc<General_functionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn number(&self) -> Option<Rc<NumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Distinct_object
/// Returns `None` if there is no child corresponding to token Distinct_object
fn Distinct_object(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Distinct_object, 0)
}
fn formula_data(&self) -> Option<Rc<Formula_dataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> General_dataContextAttrs<'input> for General_dataContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn general_data(&mut self,)
	-> Result<Rc<General_dataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_dataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 380, RULE_general_data);
        let mut _localctx: Rc<General_dataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1579);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(120,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule atomic_word*/
					recog.base.set_state(1573);
					recog.atomic_word()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule general_function*/
					recog.base.set_state(1574);
					recog.general_function()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule variable*/
					recog.base.set_state(1575);
					recog.variable()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule number*/
					recog.base.set_state(1576);
					recog.number()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1577);
					recog.base.match_token(tptp_v7_0_0_0_Distinct_object,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule formula_data*/
					recog.base.set_state(1578);
					recog.formula_data()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- general_function ----------------
pub type General_functionContextAll<'input> = General_functionContext<'input>;


pub type General_functionContext<'input> = BaseParserRuleContext<'input,General_functionContextExt<'input>>;

#[derive(Clone)]
pub struct General_functionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for General_functionContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for General_functionContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_general_function(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_general_function(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for General_functionContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_general_function(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_functionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_function }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_function }
}
antlr4rust::tid!{General_functionContextExt<'a>}

impl<'input> General_functionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_functionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_functionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_functionContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<General_functionContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn general_terms(&self) -> Option<Rc<General_termsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> General_functionContextAttrs<'input> for General_functionContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn general_function(&mut self,)
	-> Result<Rc<General_functionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_functionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 382, RULE_general_function);
        let mut _localctx: Rc<General_functionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1581);
			recog.atomic_word()?;

			recog.base.set_state(1582);
			recog.base.match_token(tptp_v7_0_0_0_T__9,&mut recog.err_handler)?;

			/*InvokeRule general_terms*/
			recog.base.set_state(1583);
			recog.general_terms()?;

			recog.base.set_state(1584);
			recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- formula_data ----------------
pub type Formula_dataContextAll<'input> = Formula_dataContext<'input>;


pub type Formula_dataContext<'input> = BaseParserRuleContext<'input,Formula_dataContextExt<'input>>;

#[derive(Clone)]
pub struct Formula_dataContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Formula_dataContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Formula_dataContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_formula_data(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_formula_data(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Formula_dataContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_formula_data(self);
	}
}

impl<'input> CustomRuleContext<'input> for Formula_dataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formula_data }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formula_data }
}
antlr4rust::tid!{Formula_dataContextExt<'a>}

impl<'input> Formula_dataContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Formula_dataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Formula_dataContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Formula_dataContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Formula_dataContextExt<'input>>{

fn thf_formula(&self) -> Option<Rc<Thf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_formula(&self) -> Option<Rc<Tff_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_formula(&self) -> Option<Rc<Fof_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cnf_formula(&self) -> Option<Rc<Cnf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_term(&self) -> Option<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Formula_dataContextAttrs<'input> for Formula_dataContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn formula_data(&mut self,)
	-> Result<Rc<Formula_dataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Formula_dataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 384, RULE_formula_data);
        let mut _localctx: Rc<Formula_dataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1606);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__38 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1586);
					recog.base.match_token(tptp_v7_0_0_0_T__38,&mut recog.err_handler)?;

					/*InvokeRule thf_formula*/
					recog.base.set_state(1587);
					recog.thf_formula()?;

					recog.base.set_state(1588);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__39 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1590);
					recog.base.match_token(tptp_v7_0_0_0_T__39,&mut recog.err_handler)?;

					/*InvokeRule tff_formula*/
					recog.base.set_state(1591);
					recog.tff_formula()?;

					recog.base.set_state(1592);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__40 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1594);
					recog.base.match_token(tptp_v7_0_0_0_T__40,&mut recog.err_handler)?;

					/*InvokeRule fof_formula*/
					recog.base.set_state(1595);
					recog.fof_formula()?;

					recog.base.set_state(1596);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__41 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1598);
					recog.base.match_token(tptp_v7_0_0_0_T__41,&mut recog.err_handler)?;

					/*InvokeRule cnf_formula*/
					recog.base.set_state(1599);
					recog.cnf_formula()?;

					recog.base.set_state(1600);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__42 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1602);
					recog.base.match_token(tptp_v7_0_0_0_T__42,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(1603);
					recog.fof_term()?;

					recog.base.set_state(1604);
					recog.base.match_token(tptp_v7_0_0_0_T__10,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- general_list ----------------
pub type General_listContextAll<'input> = General_listContext<'input>;


pub type General_listContext<'input> = BaseParserRuleContext<'input,General_listContextExt<'input>>;

#[derive(Clone)]
pub struct General_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for General_listContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for General_listContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_general_list(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_general_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for General_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_general_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_list }
}
antlr4rust::tid!{General_listContextExt<'a>}

impl<'input> General_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_listContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<General_listContextExt<'input>>{

fn general_terms(&self) -> Option<Rc<General_termsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> General_listContextAttrs<'input> for General_listContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn general_list(&mut self,)
	-> Result<Rc<General_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 386, RULE_general_list);
        let mut _localctx: Rc<General_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1613);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1608);
					recog.base.match_token(tptp_v7_0_0_0_T__16,&mut recog.err_handler)?;

					}
				}

			tptp_v7_0_0_0_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1609);
					recog.base.match_token(tptp_v7_0_0_0_T__11,&mut recog.err_handler)?;

					/*InvokeRule general_terms*/
					recog.base.set_state(1610);
					recog.general_terms()?;

					recog.base.set_state(1611);
					recog.base.match_token(tptp_v7_0_0_0_T__12,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- general_terms ----------------
pub type General_termsContextAll<'input> = General_termsContext<'input>;


pub type General_termsContext<'input> = BaseParserRuleContext<'input,General_termsContextExt<'input>>;

#[derive(Clone)]
pub struct General_termsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for General_termsContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for General_termsContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_general_terms(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_general_terms(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for General_termsContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_general_terms(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_termsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_terms }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_terms }
}
antlr4rust::tid!{General_termsContextExt<'a>}

impl<'input> General_termsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_termsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_termsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_termsContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<General_termsContextExt<'input>>{

fn general_term_all(&self) ->  Vec<Rc<General_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn general_term(&self, i: usize) -> Option<Rc<General_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> General_termsContextAttrs<'input> for General_termsContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn general_terms(&mut self,)
	-> Result<Rc<General_termsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_termsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 388, RULE_general_terms);
        let mut _localctx: Rc<General_termsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule general_term*/
			recog.base.set_state(1615);
			recog.general_term()?;

			recog.base.set_state(1620);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==tptp_v7_0_0_0_T__1 {
				{
				{
				recog.base.set_state(1616);
				recog.base.match_token(tptp_v7_0_0_0_T__1,&mut recog.err_handler)?;

				/*InvokeRule general_term*/
				recog.base.set_state(1617);
				recog.general_term()?;

				}
				}
				recog.base.set_state(1622);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- name ----------------
pub type NameContextAll<'input> = NameContext<'input>;


pub type NameContext<'input> = BaseParserRuleContext<'input,NameContextExt<'input>>;

#[derive(Clone)]
pub struct NameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for NameContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for NameContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_name(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_name(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for NameContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for NameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_name }
}
antlr4rust::tid!{NameContextExt<'a>}

impl<'input> NameContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NameContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<NameContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Integer
/// Returns `None` if there is no child corresponding to token Integer
fn Integer(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Integer, 0)
}

}

impl<'input> NameContextAttrs<'input> for NameContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn name(&mut self,)
	-> Result<Rc<NameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 390, RULE_name);
        let mut _localctx: Rc<NameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1625);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			tptp_v7_0_0_0_Lower_word |tptp_v7_0_0_0_Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule atomic_word*/
					recog.base.set_state(1623);
					recog.atomic_word()?;

					}
				}

			tptp_v7_0_0_0_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1624);
					recog.base.match_token(tptp_v7_0_0_0_Integer,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- atomic_word ----------------
pub type Atomic_wordContextAll<'input> = Atomic_wordContext<'input>;


pub type Atomic_wordContext<'input> = BaseParserRuleContext<'input,Atomic_wordContextExt<'input>>;

#[derive(Clone)]
pub struct Atomic_wordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Atomic_wordContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Atomic_wordContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_atomic_word(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_atomic_word(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Atomic_wordContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_atomic_word(self);
	}
}

impl<'input> CustomRuleContext<'input> for Atomic_wordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomic_word }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomic_word }
}
antlr4rust::tid!{Atomic_wordContextExt<'a>}

impl<'input> Atomic_wordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atomic_wordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atomic_wordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atomic_wordContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Atomic_wordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Lower_word
/// Returns `None` if there is no child corresponding to token Lower_word
fn Lower_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Lower_word, 0)
}
/// Retrieves first TerminalNode corresponding to token Single_quoted
/// Returns `None` if there is no child corresponding to token Single_quoted
fn Single_quoted(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Single_quoted, 0)
}

}

impl<'input> Atomic_wordContextAttrs<'input> for Atomic_wordContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atomic_word(&mut self,)
	-> Result<Rc<Atomic_wordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atomic_wordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 392, RULE_atomic_word);
        let mut _localctx: Rc<Atomic_wordContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1627);
			_la = recog.base.input.la(1);
			if { !(_la==tptp_v7_0_0_0_Lower_word || _la==tptp_v7_0_0_0_Single_quoted) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- atomic_defined_word ----------------
pub type Atomic_defined_wordContextAll<'input> = Atomic_defined_wordContext<'input>;


pub type Atomic_defined_wordContext<'input> = BaseParserRuleContext<'input,Atomic_defined_wordContextExt<'input>>;

#[derive(Clone)]
pub struct Atomic_defined_wordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Atomic_defined_wordContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Atomic_defined_wordContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_atomic_defined_word(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_atomic_defined_word(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Atomic_defined_wordContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_atomic_defined_word(self);
	}
}

impl<'input> CustomRuleContext<'input> for Atomic_defined_wordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomic_defined_word }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomic_defined_word }
}
antlr4rust::tid!{Atomic_defined_wordContextExt<'a>}

impl<'input> Atomic_defined_wordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atomic_defined_wordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atomic_defined_wordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atomic_defined_wordContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Atomic_defined_wordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Dollar_word
/// Returns `None` if there is no child corresponding to token Dollar_word
fn Dollar_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Dollar_word, 0)
}

}

impl<'input> Atomic_defined_wordContextAttrs<'input> for Atomic_defined_wordContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atomic_defined_word(&mut self,)
	-> Result<Rc<Atomic_defined_wordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atomic_defined_wordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 394, RULE_atomic_defined_word);
        let mut _localctx: Rc<Atomic_defined_wordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1629);
			recog.base.match_token(tptp_v7_0_0_0_Dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- atomic_system_word ----------------
pub type Atomic_system_wordContextAll<'input> = Atomic_system_wordContext<'input>;


pub type Atomic_system_wordContext<'input> = BaseParserRuleContext<'input,Atomic_system_wordContextExt<'input>>;

#[derive(Clone)]
pub struct Atomic_system_wordContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for Atomic_system_wordContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for Atomic_system_wordContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_atomic_system_word(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_atomic_system_word(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for Atomic_system_wordContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_atomic_system_word(self);
	}
}

impl<'input> CustomRuleContext<'input> for Atomic_system_wordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomic_system_word }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomic_system_word }
}
antlr4rust::tid!{Atomic_system_wordContextExt<'a>}

impl<'input> Atomic_system_wordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atomic_system_wordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atomic_system_wordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atomic_system_wordContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<Atomic_system_wordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Dollar_dollar_word
/// Returns `None` if there is no child corresponding to token Dollar_dollar_word
fn Dollar_dollar_word(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Dollar_dollar_word, 0)
}

}

impl<'input> Atomic_system_wordContextAttrs<'input> for Atomic_system_wordContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atomic_system_word(&mut self,)
	-> Result<Rc<Atomic_system_wordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atomic_system_wordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 396, RULE_atomic_system_word);
        let mut _localctx: Rc<Atomic_system_wordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1631);
			recog.base.match_token(tptp_v7_0_0_0_Dollar_dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- number ----------------
pub type NumberContextAll<'input> = NumberContext<'input>;


pub type NumberContext<'input> = BaseParserRuleContext<'input,NumberContextExt<'input>>;

#[derive(Clone)]
pub struct NumberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for NumberContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for NumberContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_number(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_number(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for NumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_number(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_number }
	//fn type_rule_index() -> usize where Self: Sized { RULE_number }
}
antlr4rust::tid!{NumberContextExt<'a>}

impl<'input> NumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NumberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NumberContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<NumberContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Integer
/// Returns `None` if there is no child corresponding to token Integer
fn Integer(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Integer, 0)
}
/// Retrieves first TerminalNode corresponding to token Rational
/// Returns `None` if there is no child corresponding to token Rational
fn Rational(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Rational, 0)
}
/// Retrieves first TerminalNode corresponding to token Real
/// Returns `None` if there is no child corresponding to token Real
fn Real(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Real, 0)
}

}

impl<'input> NumberContextAttrs<'input> for NumberContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn number(&mut self,)
	-> Result<Rc<NumberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 398, RULE_number);
        let mut _localctx: Rc<NumberContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1633);
			_la = recog.base.input.la(1);
			if { !(((((_la - 74)) & !0x3f) == 0 && ((1usize << (_la - 74)) & 73) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- file_name ----------------
pub type File_nameContextAll<'input> = File_nameContext<'input>;


pub type File_nameContext<'input> = BaseParserRuleContext<'input,File_nameContextExt<'input>>;

#[derive(Clone)]
pub struct File_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> tptp_v7_0_0_0ParserContext<'input> for File_nameContext<'input>{}

impl<'input,'a> Listenable<dyn tptp_v7_0_0_0Listener<'input> + 'a> for File_nameContext<'input>{
		fn enter(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_file_name(self);
		}
		fn exit(&self,listener: &mut (dyn tptp_v7_0_0_0Listener<'input> + 'a)) {
			listener.exit_file_name(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn tptp_v7_0_0_0Visitor<'input> + 'a> for File_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn tptp_v7_0_0_0Visitor<'input> + 'a)) {
		visitor.visit_file_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for File_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = tptp_v7_0_0_0ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file_name }
}
antlr4rust::tid!{File_nameContextExt<'a>}

impl<'input> File_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<File_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,File_nameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait File_nameContextAttrs<'input>: tptp_v7_0_0_0ParserContext<'input> + BorrowMut<File_nameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Single_quoted
/// Returns `None` if there is no child corresponding to token Single_quoted
fn Single_quoted(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(tptp_v7_0_0_0_Single_quoted, 0)
}

}

impl<'input> File_nameContextAttrs<'input> for File_nameContext<'input>{}

impl<'input, I, H> tptp_v7_0_0_0Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn file_name(&mut self,)
	-> Result<Rc<File_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = File_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 400, RULE_file_name);
        let mut _localctx: Rc<File_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1635);
			recog.base.match_token(tptp_v7_0_0_0_Single_quoted,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.into_iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
    }
const _serializedATN: [i32; 13561] = [
	4, 1, 99, 1638, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 7, 
	4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 7, 
	10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 7, 15, 
	2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 20, 7, 20, 2, 
	21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 7, 25, 2, 26, 
	7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 30, 7, 30, 2, 31, 7, 
	31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 35, 7, 35, 2, 36, 7, 36, 
	2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 39, 2, 40, 7, 40, 2, 41, 7, 41, 2, 
	42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 44, 2, 45, 7, 45, 2, 46, 7, 46, 2, 47, 
	7, 47, 2, 48, 7, 48, 2, 49, 7, 49, 2, 50, 7, 50, 2, 51, 7, 51, 2, 52, 7, 
	52, 2, 53, 7, 53, 2, 54, 7, 54, 2, 55, 7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 
	2, 58, 7, 58, 2, 59, 7, 59, 2, 60, 7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 
	63, 7, 63, 2, 64, 7, 64, 2, 65, 7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 2, 68, 
	7, 68, 2, 69, 7, 69, 2, 70, 7, 70, 2, 71, 7, 71, 2, 72, 7, 72, 2, 73, 7, 
	73, 2, 74, 7, 74, 2, 75, 7, 75, 2, 76, 7, 76, 2, 77, 7, 77, 2, 78, 7, 78, 
	2, 79, 7, 79, 2, 80, 7, 80, 2, 81, 7, 81, 2, 82, 7, 82, 2, 83, 7, 83, 2, 
	84, 7, 84, 2, 85, 7, 85, 2, 86, 7, 86, 2, 87, 7, 87, 2, 88, 7, 88, 2, 89, 
	7, 89, 2, 90, 7, 90, 2, 91, 7, 91, 2, 92, 7, 92, 2, 93, 7, 93, 2, 94, 7, 
	94, 2, 95, 7, 95, 2, 96, 7, 96, 2, 97, 7, 97, 2, 98, 7, 98, 2, 99, 7, 99, 
	2, 100, 7, 100, 2, 101, 7, 101, 2, 102, 7, 102, 2, 103, 7, 103, 2, 104, 
	7, 104, 2, 105, 7, 105, 2, 106, 7, 106, 2, 107, 7, 107, 2, 108, 7, 108, 
	2, 109, 7, 109, 2, 110, 7, 110, 2, 111, 7, 111, 2, 112, 7, 112, 2, 113, 
	7, 113, 2, 114, 7, 114, 2, 115, 7, 115, 2, 116, 7, 116, 2, 117, 7, 117, 
	2, 118, 7, 118, 2, 119, 7, 119, 2, 120, 7, 120, 2, 121, 7, 121, 2, 122, 
	7, 122, 2, 123, 7, 123, 2, 124, 7, 124, 2, 125, 7, 125, 2, 126, 7, 126, 
	2, 127, 7, 127, 2, 128, 7, 128, 2, 129, 7, 129, 2, 130, 7, 130, 2, 131, 
	7, 131, 2, 132, 7, 132, 2, 133, 7, 133, 2, 134, 7, 134, 2, 135, 7, 135, 
	2, 136, 7, 136, 2, 137, 7, 137, 2, 138, 7, 138, 2, 139, 7, 139, 2, 140, 
	7, 140, 2, 141, 7, 141, 2, 142, 7, 142, 2, 143, 7, 143, 2, 144, 7, 144, 
	2, 145, 7, 145, 2, 146, 7, 146, 2, 147, 7, 147, 2, 148, 7, 148, 2, 149, 
	7, 149, 2, 150, 7, 150, 2, 151, 7, 151, 2, 152, 7, 152, 2, 153, 7, 153, 
	2, 154, 7, 154, 2, 155, 7, 155, 2, 156, 7, 156, 2, 157, 7, 157, 2, 158, 
	7, 158, 2, 159, 7, 159, 2, 160, 7, 160, 2, 161, 7, 161, 2, 162, 7, 162, 
	2, 163, 7, 163, 2, 164, 7, 164, 2, 165, 7, 165, 2, 166, 7, 166, 2, 167, 
	7, 167, 2, 168, 7, 168, 2, 169, 7, 169, 2, 170, 7, 170, 2, 171, 7, 171, 
	2, 172, 7, 172, 2, 173, 7, 173, 2, 174, 7, 174, 2, 175, 7, 175, 2, 176, 
	7, 176, 2, 177, 7, 177, 2, 178, 7, 178, 2, 179, 7, 179, 2, 180, 7, 180, 
	2, 181, 7, 181, 2, 182, 7, 182, 2, 183, 7, 183, 2, 184, 7, 184, 2, 185, 
	7, 185, 2, 186, 7, 186, 2, 187, 7, 187, 2, 188, 7, 188, 2, 189, 7, 189, 
	2, 190, 7, 190, 2, 191, 7, 191, 2, 192, 7, 192, 2, 193, 7, 193, 2, 194, 
	7, 194, 2, 195, 7, 195, 2, 196, 7, 196, 2, 197, 7, 197, 2, 198, 7, 198, 
	2, 199, 7, 199, 2, 200, 7, 200, 1, 0, 5, 0, 404, 8, 0, 10, 0, 12, 0, 407, 
	9, 0, 1, 0, 1, 0, 1, 1, 1, 1, 3, 1, 413, 8, 1, 1, 2, 1, 2, 1, 2, 1, 2, 
	1, 2, 1, 2, 1, 2, 3, 2, 422, 8, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 
	1, 3, 3, 3, 431, 8, 3, 1, 3, 1, 3, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 1, 5, 
	1, 5, 1, 5, 1, 5, 3, 5, 444, 8, 5, 1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 
	1, 6, 1, 6, 1, 6, 3, 6, 455, 8, 6, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 7, 
	1, 7, 1, 7, 1, 7, 3, 7, 466, 8, 7, 1, 7, 1, 7, 1, 8, 1, 8, 1, 8, 1, 8, 
	1, 8, 1, 8, 1, 8, 3, 8, 477, 8, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 
	1, 9, 1, 9, 1, 9, 3, 9, 488, 8, 9, 1, 9, 1, 9, 1, 10, 1, 10, 1, 10, 1, 
	10, 1, 10, 1, 10, 1, 10, 3, 10, 499, 8, 10, 1, 10, 1, 10, 1, 11, 1, 11, 
	1, 11, 3, 11, 506, 8, 11, 1, 12, 1, 12, 1, 13, 1, 13, 3, 13, 512, 8, 13, 
	1, 14, 1, 14, 1, 14, 1, 14, 3, 14, 518, 8, 14, 1, 15, 1, 15, 1, 15, 3, 
	15, 523, 8, 15, 1, 16, 1, 16, 1, 16, 1, 16, 1, 17, 1, 17, 1, 17, 3, 17, 
	532, 8, 17, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 5, 
	18, 542, 8, 18, 10, 18, 12, 18, 545, 9, 18, 1, 19, 1, 19, 1, 19, 1, 19, 
	1, 19, 1, 19, 1, 19, 1, 19, 5, 19, 555, 8, 19, 10, 19, 12, 19, 558, 9, 
	19, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 5, 20, 568, 
	8, 20, 10, 20, 12, 20, 571, 9, 20, 1, 21, 1, 21, 1, 21, 1, 21, 1, 21, 1, 
	21, 1, 21, 1, 21, 1, 21, 1, 21, 3, 21, 583, 8, 21, 1, 22, 1, 22, 1, 22, 
	1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 24, 1, 24, 1, 24, 5, 24, 597, 
	8, 24, 10, 24, 12, 24, 600, 9, 24, 1, 25, 1, 25, 3, 25, 604, 8, 25, 1, 
	26, 1, 26, 1, 26, 1, 26, 1, 27, 1, 27, 1, 27, 1, 27, 1, 27, 1, 28, 1, 28, 
	1, 28, 1, 28, 3, 28, 619, 8, 28, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 
	29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 1, 29, 
	3, 29, 637, 8, 29, 1, 30, 1, 30, 1, 30, 3, 30, 642, 8, 30, 1, 31, 1, 31, 
	1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 32, 1, 32, 1, 32, 1, 32, 1, 
	32, 1, 32, 1, 33, 1, 33, 1, 34, 1, 34, 1, 34, 1, 34, 1, 35, 1, 35, 1, 35, 
	1, 35, 1, 35, 3, 35, 669, 8, 35, 1, 36, 1, 36, 1, 36, 1, 36, 1, 37, 1, 
	37, 1, 37, 3, 37, 678, 8, 37, 1, 38, 1, 38, 1, 39, 1, 39, 1, 40, 1, 40, 
	1, 40, 3, 40, 687, 8, 40, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 
	41, 1, 41, 3, 41, 697, 8, 41, 1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 
	1, 42, 1, 42, 5, 42, 707, 8, 42, 10, 42, 12, 42, 710, 9, 42, 1, 43, 1, 
	43, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 5, 43, 720, 8, 43, 10, 43, 
	12, 43, 723, 9, 43, 1, 44, 1, 44, 1, 44, 1, 44, 1, 44, 1, 44, 1, 44, 1, 
	44, 3, 44, 733, 8, 44, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 
	1, 45, 1, 45, 1, 45, 3, 45, 745, 8, 45, 1, 46, 1, 46, 1, 46, 5, 46, 750, 
	8, 46, 10, 46, 12, 46, 753, 9, 46, 1, 47, 1, 47, 3, 47, 757, 8, 47, 1, 
	48, 1, 48, 1, 49, 1, 49, 1, 49, 3, 49, 764, 8, 49, 1, 50, 1, 50, 1, 50, 
	3, 50, 769, 8, 50, 1, 51, 1, 51, 3, 51, 773, 8, 51, 1, 52, 1, 52, 1, 52, 
	1, 52, 1, 53, 1, 53, 3, 53, 781, 8, 53, 1, 54, 1, 54, 1, 54, 1, 54, 1, 
	54, 1, 54, 1, 54, 1, 54, 5, 54, 791, 8, 54, 10, 54, 12, 54, 794, 9, 54, 
	1, 55, 1, 55, 1, 55, 1, 55, 1, 55, 1, 55, 1, 55, 1, 55, 5, 55, 804, 8, 
	55, 10, 55, 12, 55, 807, 9, 55, 1, 56, 1, 56, 1, 56, 1, 56, 1, 56, 1, 56, 
	1, 56, 1, 56, 1, 56, 3, 56, 818, 8, 56, 1, 57, 1, 57, 1, 57, 1, 57, 1, 
	57, 1, 57, 1, 57, 1, 58, 1, 58, 1, 58, 5, 58, 830, 8, 58, 10, 58, 12, 58, 
	833, 9, 58, 1, 59, 1, 59, 3, 59, 837, 8, 59, 1, 60, 1, 60, 1, 60, 1, 60, 
	1, 61, 1, 61, 1, 61, 1, 61, 3, 61, 847, 8, 61, 1, 62, 1, 62, 1, 63, 1, 
	63, 1, 63, 1, 63, 1, 63, 1, 63, 1, 63, 1, 63, 1, 64, 1, 64, 1, 64, 1, 64, 
	1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 3, 64, 871, 8, 
	64, 1, 65, 1, 65, 1, 65, 1, 65, 1, 65, 3, 65, 878, 8, 65, 1, 66, 1, 66, 
	1, 66, 5, 66, 883, 8, 66, 10, 66, 12, 66, 886, 9, 66, 1, 67, 1, 67, 1, 
	67, 1, 67, 1, 67, 1, 67, 1, 67, 1, 67, 3, 67, 896, 8, 67, 1, 68, 1, 68, 
	1, 68, 1, 68, 1, 68, 1, 68, 1, 68, 1, 68, 3, 68, 906, 8, 68, 1, 69, 1, 
	69, 1, 69, 1, 69, 1, 69, 3, 69, 913, 8, 69, 1, 70, 1, 70, 1, 70, 5, 70, 
	918, 8, 70, 10, 70, 12, 70, 921, 9, 70, 1, 71, 1, 71, 1, 71, 1, 71, 1, 
	71, 1, 71, 1, 71, 1, 71, 3, 71, 931, 8, 71, 1, 72, 1, 72, 1, 72, 1, 72, 
	1, 72, 1, 72, 1, 72, 1, 72, 3, 72, 941, 8, 72, 1, 73, 1, 73, 1, 73, 1, 
	73, 1, 73, 1, 73, 1, 73, 1, 73, 3, 73, 951, 8, 73, 1, 74, 1, 74, 1, 74, 
	1, 74, 1, 74, 3, 74, 958, 8, 74, 1, 75, 1, 75, 1, 75, 5, 75, 963, 8, 75, 
	10, 75, 12, 75, 966, 9, 75, 1, 76, 1, 76, 1, 76, 1, 76, 1, 76, 1, 76, 1, 
	76, 1, 76, 3, 76, 976, 8, 76, 1, 77, 1, 77, 1, 77, 1, 77, 1, 78, 1, 78, 
	1, 78, 1, 78, 1, 78, 1, 78, 1, 78, 3, 78, 989, 8, 78, 1, 79, 1, 79, 1, 
	79, 1, 79, 1, 79, 1, 79, 1, 79, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 3, 80, 
	1003, 8, 80, 1, 81, 1, 81, 1, 81, 1, 81, 1, 81, 3, 81, 1010, 8, 81, 1, 
	82, 1, 82, 1, 82, 1, 82, 1, 82, 1, 82, 1, 82, 1, 82, 3, 82, 1020, 8, 82, 
	1, 83, 1, 83, 1, 83, 5, 83, 1025, 8, 83, 10, 83, 12, 83, 1028, 9, 83, 1, 
	84, 1, 84, 1, 84, 1, 84, 1, 85, 1, 85, 1, 85, 1, 85, 1, 85, 1, 85, 1, 85, 
	1, 85, 5, 85, 1042, 8, 85, 10, 85, 12, 85, 1045, 9, 85, 1, 86, 1, 86, 3, 
	86, 1049, 8, 86, 1, 87, 1, 87, 3, 87, 1053, 8, 87, 1, 88, 1, 88, 1, 88, 
	1, 88, 1, 88, 1, 88, 1, 88, 1, 89, 1, 89, 3, 89, 1064, 8, 89, 1, 90, 1, 
	90, 3, 90, 1068, 8, 90, 1, 91, 1, 91, 3, 91, 1072, 8, 91, 1, 92, 1, 92, 
	1, 92, 1, 92, 1, 93, 1, 93, 3, 93, 1080, 8, 93, 1, 94, 1, 94, 1, 94, 1, 
	94, 1, 94, 1, 94, 1, 94, 1, 94, 5, 94, 1090, 8, 94, 10, 94, 12, 94, 1093, 
	9, 94, 1, 95, 1, 95, 1, 95, 1, 95, 1, 95, 1, 95, 1, 95, 1, 95, 5, 95, 1103, 
	8, 95, 10, 95, 12, 95, 1106, 9, 95, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 
	1, 96, 1, 96, 3, 96, 1115, 8, 96, 1, 97, 1, 97, 1, 97, 1, 97, 1, 97, 1, 
	97, 1, 97, 1, 98, 1, 98, 1, 98, 5, 98, 1127, 8, 98, 10, 98, 12, 98, 1130, 
	9, 98, 1, 99, 1, 99, 1, 99, 1, 99, 3, 99, 1136, 8, 99, 1, 100, 1, 100, 
	1, 100, 1, 100, 1, 101, 1, 101, 1, 101, 3, 101, 1145, 8, 101, 1, 102, 1, 
	102, 1, 103, 1, 103, 3, 103, 1151, 8, 103, 1, 104, 1, 104, 1, 105, 1, 105, 
	1, 105, 1, 105, 1, 106, 1, 106, 1, 107, 1, 107, 1, 107, 1, 107, 1, 107, 
	1, 107, 3, 107, 1167, 8, 107, 1, 108, 1, 108, 3, 108, 1171, 8, 108, 1, 
	109, 1, 109, 1, 110, 1, 110, 1, 110, 1, 110, 1, 110, 1, 110, 3, 110, 1181, 
	8, 110, 1, 111, 1, 111, 1, 111, 1, 111, 1, 111, 1, 111, 3, 111, 1189, 8, 
	111, 1, 112, 1, 112, 1, 112, 5, 112, 1194, 8, 112, 10, 112, 12, 112, 1197, 
	9, 112, 1, 113, 1, 113, 1, 113, 1, 113, 1, 113, 3, 113, 1204, 8, 113, 1, 
	114, 1, 114, 1, 114, 3, 114, 1209, 8, 114, 1, 115, 1, 115, 1, 115, 1, 115, 
	1, 115, 1, 115, 1, 115, 1, 115, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 
	1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 3, 116, 1231, 8, 
	116, 1, 117, 1, 117, 1, 117, 1, 117, 1, 117, 3, 117, 1238, 8, 117, 1, 118, 
	1, 118, 1, 118, 1, 118, 1, 118, 1, 118, 1, 118, 1, 118, 3, 118, 1248, 8, 
	118, 1, 119, 1, 119, 1, 119, 1, 119, 1, 119, 3, 119, 1255, 8, 119, 1, 120, 
	1, 120, 1, 120, 5, 120, 1260, 8, 120, 10, 120, 12, 120, 1263, 9, 120, 1, 
	121, 1, 121, 1, 121, 1, 121, 1, 121, 3, 121, 1270, 8, 121, 1, 122, 1, 122, 
	1, 122, 1, 122, 1, 122, 1, 122, 5, 122, 1278, 8, 122, 10, 122, 12, 122, 
	1281, 9, 122, 1, 123, 1, 123, 1, 123, 1, 123, 3, 123, 1287, 8, 123, 1, 
	124, 1, 124, 1, 124, 3, 124, 1292, 8, 124, 1, 125, 1, 125, 1, 126, 1, 126, 
	1, 127, 1, 127, 1, 127, 1, 127, 3, 127, 1302, 8, 127, 1, 128, 1, 128, 3, 
	128, 1306, 8, 128, 1, 129, 1, 129, 1, 130, 1, 130, 3, 130, 1312, 8, 130, 
	1, 131, 1, 131, 1, 132, 1, 132, 1, 133, 1, 133, 1, 134, 1, 134, 1, 135, 
	1, 135, 1, 136, 1, 136, 1, 137, 1, 137, 1, 138, 1, 138, 1, 139, 1, 139, 
	3, 139, 1332, 8, 139, 1, 140, 1, 140, 3, 140, 1336, 8, 140, 1, 141, 1, 
	141, 1, 142, 1, 142, 1, 143, 1, 143, 1, 144, 1, 144, 1, 145, 1, 145, 1, 
	146, 1, 146, 1, 147, 1, 147, 1, 148, 1, 148, 1, 149, 1, 149, 1, 150, 1, 
	150, 3, 150, 1358, 8, 150, 1, 151, 1, 151, 1, 152, 1, 152, 1, 152, 1, 152, 
	1, 152, 1, 152, 1, 152, 1, 152, 3, 152, 1370, 8, 152, 1, 153, 1, 153, 1, 
	153, 5, 153, 1375, 8, 153, 10, 153, 12, 153, 1378, 9, 153, 1, 154, 1, 154, 
	3, 154, 1382, 8, 154, 1, 155, 1, 155, 1, 155, 1, 155, 1, 155, 1, 155, 1, 
	155, 1, 155, 1, 156, 1, 156, 1, 157, 1, 157, 1, 157, 1, 157, 1, 157, 3, 
	157, 1399, 8, 157, 1, 158, 1, 158, 1, 158, 5, 158, 1404, 8, 158, 10, 158, 
	12, 158, 1407, 9, 158, 1, 159, 1, 159, 3, 159, 1411, 8, 159, 1, 160, 1, 
	160, 1, 160, 1, 161, 1, 161, 1, 161, 3, 161, 1419, 8, 161, 1, 161, 1, 161, 
	1, 162, 1, 162, 1, 163, 1, 163, 1, 163, 3, 163, 1428, 8, 163, 1, 164, 1, 
	164, 1, 164, 3, 164, 1433, 8, 164, 1, 164, 1, 164, 1, 165, 1, 165, 1, 165, 
	1, 166, 1, 166, 1, 166, 3, 166, 1443, 8, 166, 1, 166, 1, 166, 1, 167, 1, 
	167, 1, 168, 1, 168, 1, 168, 3, 168, 1452, 8, 168, 1, 168, 1, 168, 1, 169, 
	1, 169, 1, 170, 1, 170, 1, 170, 1, 171, 1, 171, 1, 171, 1, 171, 1, 171, 
	1, 171, 3, 171, 1467, 8, 171, 1, 172, 1, 172, 1, 172, 5, 172, 1472, 8, 
	172, 10, 172, 12, 172, 1475, 9, 172, 1, 173, 1, 173, 1, 173, 3, 173, 1480, 
	8, 173, 1, 174, 1, 174, 3, 174, 1484, 8, 174, 1, 175, 1, 175, 1, 175, 1, 
	175, 1, 176, 1, 176, 1, 176, 1, 176, 1, 177, 1, 177, 1, 177, 1, 177, 3, 
	177, 1498, 8, 177, 1, 178, 1, 178, 1, 178, 1, 178, 1, 178, 3, 178, 1505, 
	8, 178, 1, 179, 1, 179, 1, 180, 1, 180, 1, 180, 1, 180, 1, 180, 1, 180, 
	1, 180, 1, 181, 1, 181, 1, 181, 1, 181, 1, 181, 1, 181, 1, 182, 1, 182, 
	1, 182, 1, 182, 1, 183, 1, 183, 1, 183, 1, 183, 1, 183, 1, 183, 1, 183, 
	1, 183, 1, 184, 1, 184, 1, 184, 5, 184, 1537, 8, 184, 10, 184, 12, 184, 
	1540, 9, 184, 1, 185, 1, 185, 3, 185, 1544, 8, 185, 1, 186, 1, 186, 1, 
	186, 3, 186, 1549, 8, 186, 1, 186, 1, 186, 1, 187, 1, 187, 1, 187, 1, 187, 
	1, 187, 1, 188, 1, 188, 1, 188, 5, 188, 1561, 8, 188, 10, 188, 12, 188, 
	1564, 9, 188, 1, 189, 1, 189, 1, 189, 1, 189, 1, 189, 1, 189, 3, 189, 1572, 
	8, 189, 1, 190, 1, 190, 1, 190, 1, 190, 1, 190, 1, 190, 3, 190, 1580, 8, 
	190, 1, 191, 1, 191, 1, 191, 1, 191, 1, 191, 1, 192, 1, 192, 1, 192, 1, 
	192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 
	192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 3, 192, 1607, 
	8, 192, 1, 193, 1, 193, 1, 193, 1, 193, 1, 193, 3, 193, 1614, 8, 193, 1, 
	194, 1, 194, 1, 194, 5, 194, 1619, 8, 194, 10, 194, 12, 194, 1622, 9, 194, 
	1, 195, 1, 195, 3, 195, 1626, 8, 195, 1, 196, 1, 196, 1, 197, 1, 197, 1, 
	198, 1, 198, 1, 199, 1, 199, 1, 200, 1, 200, 1, 200, 0, 11, 36, 38, 40, 
	84, 86, 108, 110, 170, 188, 190, 244, 201, 0, 2, 4, 6, 8, 10, 12, 14, 16, 
	18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 
	54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 
	90, 92, 94, 96, 98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 
	122, 124, 126, 128, 130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 
	152, 154, 156, 158, 160, 162, 164, 166, 168, 170, 172, 174, 176, 178, 180, 
	182, 184, 186, 188, 190, 192, 194, 196, 198, 200, 202, 204, 206, 208, 210, 
	212, 214, 216, 218, 220, 222, 224, 226, 228, 230, 232, 234, 236, 238, 240, 
	242, 244, 246, 248, 250, 252, 254, 256, 258, 260, 262, 264, 266, 268, 270, 
	272, 274, 276, 278, 280, 282, 284, 286, 288, 290, 292, 294, 296, 298, 300, 
	302, 304, 306, 308, 310, 312, 314, 316, 318, 320, 322, 324, 326, 328, 330, 
	332, 334, 336, 338, 340, 342, 344, 346, 348, 350, 352, 354, 356, 358, 360, 
	362, 364, 366, 368, 370, 372, 374, 376, 378, 380, 382, 384, 386, 388, 390, 
	392, 394, 396, 398, 400, 0, 9, 3, 0, 61, 61, 63, 63, 65, 65, 2, 0, 54, 
	54, 59, 59, 5, 0, 53, 53, 58, 58, 62, 62, 64, 64, 66, 66, 2, 0, 57, 57, 
	60, 60, 1, 0, 46, 51, 1, 0, 44, 45, 2, 0, 56, 56, 68, 68, 1, 0, 94, 95, 
	3, 0, 74, 74, 77, 77, 80, 80, 1621, 0, 405, 1, 0, 0, 0, 2, 412, 1, 0, 0, 
	0, 4, 421, 1, 0, 0, 0, 6, 423, 1, 0, 0, 0, 8, 434, 1, 0, 0, 0, 10, 436, 
	1, 0, 0, 0, 12, 447, 1, 0, 0, 0, 14, 458, 1, 0, 0, 0, 16, 469, 1, 0, 0, 
	0, 18, 480, 1, 0, 0, 0, 20, 491, 1, 0, 0, 0, 22, 502, 1, 0, 0, 0, 24, 507, 
	1, 0, 0, 0, 26, 511, 1, 0, 0, 0, 28, 517, 1, 0, 0, 0, 30, 522, 1, 0, 0, 
	0, 32, 524, 1, 0, 0, 0, 34, 531, 1, 0, 0, 0, 36, 533, 1, 0, 0, 0, 38, 546, 
	1, 0, 0, 0, 40, 559, 1, 0, 0, 0, 42, 582, 1, 0, 0, 0, 44, 584, 1, 0, 0, 
	0, 46, 587, 1, 0, 0, 0, 48, 593, 1, 0, 0, 0, 50, 603, 1, 0, 0, 0, 52, 605, 
	1, 0, 0, 0, 54, 609, 1, 0, 0, 0, 56, 618, 1, 0, 0, 0, 58, 636, 1, 0, 0, 
	0, 60, 641, 1, 0, 0, 0, 62, 643, 1, 0, 0, 0, 64, 651, 1, 0, 0, 0, 66, 657, 
	1, 0, 0, 0, 68, 659, 1, 0, 0, 0, 70, 668, 1, 0, 0, 0, 72, 670, 1, 0, 0, 
	0, 74, 677, 1, 0, 0, 0, 76, 679, 1, 0, 0, 0, 78, 681, 1, 0, 0, 0, 80, 686, 
	1, 0, 0, 0, 82, 696, 1, 0, 0, 0, 84, 698, 1, 0, 0, 0, 86, 711, 1, 0, 0, 
	0, 88, 732, 1, 0, 0, 0, 90, 744, 1, 0, 0, 0, 92, 746, 1, 0, 0, 0, 94, 756, 
	1, 0, 0, 0, 96, 758, 1, 0, 0, 0, 98, 763, 1, 0, 0, 0, 100, 768, 1, 0, 0, 
	0, 102, 772, 1, 0, 0, 0, 104, 774, 1, 0, 0, 0, 106, 780, 1, 0, 0, 0, 108, 
	782, 1, 0, 0, 0, 110, 795, 1, 0, 0, 0, 112, 817, 1, 0, 0, 0, 114, 819, 
	1, 0, 0, 0, 116, 826, 1, 0, 0, 0, 118, 836, 1, 0, 0, 0, 120, 838, 1, 0, 
	0, 0, 122, 846, 1, 0, 0, 0, 124, 848, 1, 0, 0, 0, 126, 850, 1, 0, 0, 0, 
	128, 870, 1, 0, 0, 0, 130, 877, 1, 0, 0, 0, 132, 879, 1, 0, 0, 0, 134, 
	895, 1, 0, 0, 0, 136, 905, 1, 0, 0, 0, 138, 912, 1, 0, 0, 0, 140, 914, 
	1, 0, 0, 0, 142, 930, 1, 0, 0, 0, 144, 940, 1, 0, 0, 0, 146, 950, 1, 0, 
	0, 0, 148, 957, 1, 0, 0, 0, 150, 959, 1, 0, 0, 0, 152, 975, 1, 0, 0, 0, 
	154, 977, 1, 0, 0, 0, 156, 988, 1, 0, 0, 0, 158, 990, 1, 0, 0, 0, 160, 
	1002, 1, 0, 0, 0, 162, 1009, 1, 0, 0, 0, 164, 1019, 1, 0, 0, 0, 166, 1021, 
	1, 0, 0, 0, 168, 1029, 1, 0, 0, 0, 170, 1033, 1, 0, 0, 0, 172, 1048, 1, 
	0, 0, 0, 174, 1052, 1, 0, 0, 0, 176, 1054, 1, 0, 0, 0, 178, 1063, 1, 0, 
	0, 0, 180, 1067, 1, 0, 0, 0, 182, 1071, 1, 0, 0, 0, 184, 1073, 1, 0, 0, 
	0, 186, 1079, 1, 0, 0, 0, 188, 1081, 1, 0, 0, 0, 190, 1094, 1, 0, 0, 0, 
	192, 1114, 1, 0, 0, 0, 194, 1116, 1, 0, 0, 0, 196, 1123, 1, 0, 0, 0, 198, 
	1135, 1, 0, 0, 0, 200, 1137, 1, 0, 0, 0, 202, 1144, 1, 0, 0, 0, 204, 1146, 
	1, 0, 0, 0, 206, 1150, 1, 0, 0, 0, 208, 1152, 1, 0, 0, 0, 210, 1154, 1, 
	0, 0, 0, 212, 1158, 1, 0, 0, 0, 214, 1166, 1, 0, 0, 0, 216, 1170, 1, 0, 
	0, 0, 218, 1172, 1, 0, 0, 0, 220, 1180, 1, 0, 0, 0, 222, 1188, 1, 0, 0, 
	0, 224, 1190, 1, 0, 0, 0, 226, 1203, 1, 0, 0, 0, 228, 1208, 1, 0, 0, 0, 
	230, 1210, 1, 0, 0, 0, 232, 1230, 1, 0, 0, 0, 234, 1237, 1, 0, 0, 0, 236, 
	1247, 1, 0, 0, 0, 238, 1254, 1, 0, 0, 0, 240, 1256, 1, 0, 0, 0, 242, 1269, 
	1, 0, 0, 0, 244, 1271, 1, 0, 0, 0, 246, 1286, 1, 0, 0, 0, 248, 1291, 1, 
	0, 0, 0, 250, 1293, 1, 0, 0, 0, 252, 1295, 1, 0, 0, 0, 254, 1301, 1, 0, 
	0, 0, 256, 1305, 1, 0, 0, 0, 258, 1307, 1, 0, 0, 0, 260, 1311, 1, 0, 0, 
	0, 262, 1313, 1, 0, 0, 0, 264, 1315, 1, 0, 0, 0, 266, 1317, 1, 0, 0, 0, 
	268, 1319, 1, 0, 0, 0, 270, 1321, 1, 0, 0, 0, 272, 1323, 1, 0, 0, 0, 274, 
	1325, 1, 0, 0, 0, 276, 1327, 1, 0, 0, 0, 278, 1331, 1, 0, 0, 0, 280, 1335, 
	1, 0, 0, 0, 282, 1337, 1, 0, 0, 0, 284, 1339, 1, 0, 0, 0, 286, 1341, 1, 
	0, 0, 0, 288, 1343, 1, 0, 0, 0, 290, 1345, 1, 0, 0, 0, 292, 1347, 1, 0, 
	0, 0, 294, 1349, 1, 0, 0, 0, 296, 1351, 1, 0, 0, 0, 298, 1353, 1, 0, 0, 
	0, 300, 1357, 1, 0, 0, 0, 302, 1359, 1, 0, 0, 0, 304, 1369, 1, 0, 0, 0, 
	306, 1371, 1, 0, 0, 0, 308, 1381, 1, 0, 0, 0, 310, 1383, 1, 0, 0, 0, 312, 
	1391, 1, 0, 0, 0, 314, 1398, 1, 0, 0, 0, 316, 1400, 1, 0, 0, 0, 318, 1408, 
	1, 0, 0, 0, 320, 1412, 1, 0, 0, 0, 322, 1415, 1, 0, 0, 0, 324, 1422, 1, 
	0, 0, 0, 326, 1427, 1, 0, 0, 0, 328, 1429, 1, 0, 0, 0, 330, 1436, 1, 0, 
	0, 0, 332, 1439, 1, 0, 0, 0, 334, 1446, 1, 0, 0, 0, 336, 1448, 1, 0, 0, 
	0, 338, 1455, 1, 0, 0, 0, 340, 1457, 1, 0, 0, 0, 342, 1466, 1, 0, 0, 0, 
	344, 1468, 1, 0, 0, 0, 346, 1479, 1, 0, 0, 0, 348, 1483, 1, 0, 0, 0, 350, 
	1485, 1, 0, 0, 0, 352, 1489, 1, 0, 0, 0, 354, 1497, 1, 0, 0, 0, 356, 1504, 
	1, 0, 0, 0, 358, 1506, 1, 0, 0, 0, 360, 1508, 1, 0, 0, 0, 362, 1515, 1, 
	0, 0, 0, 364, 1521, 1, 0, 0, 0, 366, 1525, 1, 0, 0, 0, 368, 1533, 1, 0, 
	0, 0, 370, 1543, 1, 0, 0, 0, 372, 1545, 1, 0, 0, 0, 374, 1552, 1, 0, 0, 
	0, 376, 1557, 1, 0, 0, 0, 378, 1571, 1, 0, 0, 0, 380, 1579, 1, 0, 0, 0, 
	382, 1581, 1, 0, 0, 0, 384, 1606, 1, 0, 0, 0, 386, 1613, 1, 0, 0, 0, 388, 
	1615, 1, 0, 0, 0, 390, 1625, 1, 0, 0, 0, 392, 1627, 1, 0, 0, 0, 394, 1629, 
	1, 0, 0, 0, 396, 1631, 1, 0, 0, 0, 398, 1633, 1, 0, 0, 0, 400, 1635, 1, 
	0, 0, 0, 402, 404, 3, 2, 1, 0, 403, 402, 1, 0, 0, 0, 404, 407, 1, 0, 0, 
	0, 405, 403, 1, 0, 0, 0, 405, 406, 1, 0, 0, 0, 406, 408, 1, 0, 0, 0, 407, 
	405, 1, 0, 0, 0, 408, 409, 5, 0, 0, 1, 409, 1, 1, 0, 0, 0, 410, 413, 3, 
	4, 2, 0, 411, 413, 3, 372, 186, 0, 412, 410, 1, 0, 0, 0, 412, 411, 1, 0, 
	0, 0, 413, 3, 1, 0, 0, 0, 414, 422, 3, 10, 5, 0, 415, 422, 3, 12, 6, 0, 
	416, 422, 3, 14, 7, 0, 417, 422, 3, 16, 8, 0, 418, 422, 3, 18, 9, 0, 419, 
	422, 3, 20, 10, 0, 420, 422, 3, 6, 3, 0, 421, 414, 1, 0, 0, 0, 421, 415, 
	1, 0, 0, 0, 421, 416, 1, 0, 0, 0, 421, 417, 1, 0, 0, 0, 421, 418, 1, 0, 
	0, 0, 421, 419, 1, 0, 0, 0, 421, 420, 1, 0, 0, 0, 422, 5, 1, 0, 0, 0, 423, 
	424, 5, 1, 0, 0, 424, 425, 3, 390, 195, 0, 425, 426, 5, 2, 0, 0, 426, 427, 
	3, 24, 12, 0, 427, 428, 5, 2, 0, 0, 428, 430, 3, 8, 4, 0, 429, 431, 3, 
	22, 11, 0, 430, 429, 1, 0, 0, 0, 430, 431, 1, 0, 0, 0, 431, 432, 1, 0, 
	0, 0, 432, 433, 5, 3, 0, 0, 433, 7, 1, 0, 0, 0, 434, 435, 3, 178, 89, 0, 
	435, 9, 1, 0, 0, 0, 436, 437, 5, 4, 0, 0, 437, 438, 3, 390, 195, 0, 438, 
	439, 5, 2, 0, 0, 439, 440, 3, 24, 12, 0, 440, 441, 5, 2, 0, 0, 441, 443, 
	3, 26, 13, 0, 442, 444, 3, 22, 11, 0, 443, 442, 1, 0, 0, 0, 443, 444, 1, 
	0, 0, 0, 444, 445, 1, 0, 0, 0, 445, 446, 5, 3, 0, 0, 446, 11, 1, 0, 0, 
	0, 447, 448, 5, 5, 0, 0, 448, 449, 3, 390, 195, 0, 449, 450, 5, 2, 0, 0, 
	450, 451, 3, 24, 12, 0, 451, 452, 5, 2, 0, 0, 452, 454, 3, 94, 47, 0, 453, 
	455, 3, 22, 11, 0, 454, 453, 1, 0, 0, 0, 454, 455, 1, 0, 0, 0, 455, 456, 
	1, 0, 0, 0, 456, 457, 5, 3, 0, 0, 457, 13, 1, 0, 0, 0, 458, 459, 5, 6, 
	0, 0, 459, 460, 3, 390, 195, 0, 460, 461, 5, 2, 0, 0, 461, 462, 3, 24, 
	12, 0, 462, 463, 5, 2, 0, 0, 463, 465, 3, 98, 49, 0, 464, 466, 3, 22, 11, 
	0, 465, 464, 1, 0, 0, 0, 465, 466, 1, 0, 0, 0, 466, 467, 1, 0, 0, 0, 467, 
	468, 5, 3, 0, 0, 468, 15, 1, 0, 0, 0, 469, 470, 5, 7, 0, 0, 470, 471, 3, 
	390, 195, 0, 471, 472, 5, 2, 0, 0, 472, 473, 3, 24, 12, 0, 473, 474, 5, 
	2, 0, 0, 474, 476, 3, 172, 86, 0, 475, 477, 3, 22, 11, 0, 476, 475, 1, 
	0, 0, 0, 476, 477, 1, 0, 0, 0, 477, 478, 1, 0, 0, 0, 478, 479, 5, 3, 0, 
	0, 479, 17, 1, 0, 0, 0, 480, 481, 5, 8, 0, 0, 481, 482, 3, 390, 195, 0, 
	482, 483, 5, 2, 0, 0, 483, 484, 3, 24, 12, 0, 484, 485, 5, 2, 0, 0, 485, 
	487, 3, 178, 89, 0, 486, 488, 3, 22, 11, 0, 487, 486, 1, 0, 0, 0, 487, 
	488, 1, 0, 0, 0, 488, 489, 1, 0, 0, 0, 489, 490, 5, 3, 0, 0, 490, 19, 1, 
	0, 0, 0, 491, 492, 5, 9, 0, 0, 492, 493, 3, 390, 195, 0, 493, 494, 5, 2, 
	0, 0, 494, 495, 3, 24, 12, 0, 495, 496, 5, 2, 0, 0, 496, 498, 3, 242, 121, 
	0, 497, 499, 3, 22, 11, 0, 498, 497, 1, 0, 0, 0, 498, 499, 1, 0, 0, 0, 
	499, 500, 1, 0, 0, 0, 500, 501, 5, 3, 0, 0, 501, 21, 1, 0, 0, 0, 502, 503, 
	5, 2, 0, 0, 503, 505, 3, 304, 152, 0, 504, 506, 3, 340, 170, 0, 505, 504, 
	1, 0, 0, 0, 505, 506, 1, 0, 0, 0, 506, 23, 1, 0, 0, 0, 507, 508, 5, 94, 
	0, 0, 508, 25, 1, 0, 0, 0, 509, 512, 3, 28, 14, 0, 510, 512, 3, 88, 44, 
	0, 511, 509, 1, 0, 0, 0, 511, 510, 1, 0, 0, 0, 512, 27, 1, 0, 0, 0, 513, 
	518, 3, 30, 15, 0, 514, 518, 3, 42, 21, 0, 515, 518, 3, 68, 34, 0, 516, 
	518, 3, 72, 36, 0, 517, 513, 1, 0, 0, 0, 517, 514, 1, 0, 0, 0, 517, 515, 
	1, 0, 0, 0, 517, 516, 1, 0, 0, 0, 518, 29, 1, 0, 0, 0, 519, 523, 3, 32, 
	16, 0, 520, 523, 3, 34, 17, 0, 521, 523, 3, 80, 40, 0, 522, 519, 1, 0, 
	0, 0, 522, 520, 1, 0, 0, 0, 522, 521, 1, 0, 0, 0, 523, 31, 1, 0, 0, 0, 
	524, 525, 3, 42, 21, 0, 525, 526, 3, 254, 127, 0, 526, 527, 3, 42, 21, 
	0, 527, 33, 1, 0, 0, 0, 528, 532, 3, 36, 18, 0, 529, 532, 3, 38, 19, 0, 
	530, 532, 3, 40, 20, 0, 531, 528, 1, 0, 0, 0, 531, 529, 1, 0, 0, 0, 531, 
	530, 1, 0, 0, 0, 532, 35, 1, 0, 0, 0, 533, 534, 6, 18, -1, 0, 534, 535, 
	3, 42, 21, 0, 535, 536, 5, 44, 0, 0, 536, 537, 3, 42, 21, 0, 537, 543, 
	1, 0, 0, 0, 538, 539, 10, 1, 0, 0, 539, 540, 5, 44, 0, 0, 540, 542, 3, 
	42, 21, 0, 541, 538, 1, 0, 0, 0, 542, 545, 1, 0, 0, 0, 543, 541, 1, 0, 
	0, 0, 543, 544, 1, 0, 0, 0, 544, 37, 1, 0, 0, 0, 545, 543, 1, 0, 0, 0, 
	546, 547, 6, 19, -1, 0, 547, 548, 3, 42, 21, 0, 548, 549, 5, 45, 0, 0, 
	549, 550, 3, 42, 21, 0, 550, 556, 1, 0, 0, 0, 551, 552, 10, 1, 0, 0, 552, 
	553, 5, 45, 0, 0, 553, 555, 3, 42, 21, 0, 554, 551, 1, 0, 0, 0, 555, 558, 
	1, 0, 0, 0, 556, 554, 1, 0, 0, 0, 556, 557, 1, 0, 0, 0, 557, 39, 1, 0, 
	0, 0, 558, 556, 1, 0, 0, 0, 559, 560, 6, 20, -1, 0, 560, 561, 3, 42, 21, 
	0, 561, 562, 5, 67, 0, 0, 562, 563, 3, 42, 21, 0, 563, 569, 1, 0, 0, 0, 
	564, 565, 10, 1, 0, 0, 565, 566, 5, 67, 0, 0, 566, 568, 3, 42, 21, 0, 567, 
	564, 1, 0, 0, 0, 568, 571, 1, 0, 0, 0, 569, 567, 1, 0, 0, 0, 569, 570, 
	1, 0, 0, 0, 570, 41, 1, 0, 0, 0, 571, 569, 1, 0, 0, 0, 572, 583, 3, 44, 
	22, 0, 573, 583, 3, 54, 27, 0, 574, 583, 3, 56, 28, 0, 575, 583, 3, 62, 
	31, 0, 576, 583, 3, 64, 32, 0, 577, 583, 3, 90, 45, 0, 578, 579, 5, 10, 
	0, 0, 579, 580, 3, 28, 14, 0, 580, 581, 5, 11, 0, 0, 581, 583, 1, 0, 0, 
	0, 582, 572, 1, 0, 0, 0, 582, 573, 1, 0, 0, 0, 582, 574, 1, 0, 0, 0, 582, 
	575, 1, 0, 0, 0, 582, 576, 1, 0, 0, 0, 582, 577, 1, 0, 0, 0, 582, 578, 
	1, 0, 0, 0, 583, 43, 1, 0, 0, 0, 584, 585, 3, 46, 23, 0, 585, 586, 3, 42, 
	21, 0, 586, 45, 1, 0, 0, 0, 587, 588, 3, 248, 124, 0, 588, 589, 5, 12, 
	0, 0, 589, 590, 3, 48, 24, 0, 590, 591, 5, 13, 0, 0, 591, 592, 5, 14, 0, 
	0, 592, 47, 1, 0, 0, 0, 593, 598, 3, 50, 25, 0, 594, 595, 5, 2, 0, 0, 595, 
	597, 3, 50, 25, 0, 596, 594, 1, 0, 0, 0, 597, 600, 1, 0, 0, 0, 598, 596, 
	1, 0, 0, 0, 598, 599, 1, 0, 0, 0, 599, 49, 1, 0, 0, 0, 600, 598, 1, 0, 
	0, 0, 601, 604, 3, 52, 26, 0, 602, 604, 3, 302, 151, 0, 603, 601, 1, 0, 
	0, 0, 603, 602, 1, 0, 0, 0, 604, 51, 1, 0, 0, 0, 605, 606, 3, 302, 151, 
	0, 606, 607, 5, 14, 0, 0, 607, 608, 3, 74, 37, 0, 608, 53, 1, 0, 0, 0, 
	609, 610, 3, 256, 128, 0, 610, 611, 5, 10, 0, 0, 611, 612, 3, 28, 14, 0, 
	612, 613, 5, 11, 0, 0, 613, 55, 1, 0, 0, 0, 614, 619, 3, 58, 29, 0, 615, 
	619, 3, 302, 151, 0, 616, 619, 3, 300, 150, 0, 617, 619, 3, 60, 30, 0, 
	618, 614, 1, 0, 0, 0, 618, 615, 1, 0, 0, 0, 618, 616, 1, 0, 0, 0, 618, 
	617, 1, 0, 0, 0, 619, 57, 1, 0, 0, 0, 620, 637, 3, 278, 139, 0, 621, 622, 
	3, 290, 145, 0, 622, 623, 5, 10, 0, 0, 623, 624, 3, 66, 33, 0, 624, 625, 
	5, 11, 0, 0, 625, 637, 1, 0, 0, 0, 626, 627, 3, 298, 149, 0, 627, 628, 
	5, 10, 0, 0, 628, 629, 3, 66, 33, 0, 629, 630, 5, 11, 0, 0, 630, 637, 1, 
	0, 0, 0, 631, 632, 3, 294, 147, 0, 632, 633, 5, 10, 0, 0, 633, 634, 3, 
	66, 33, 0, 634, 635, 5, 11, 0, 0, 635, 637, 1, 0, 0, 0, 636, 620, 1, 0, 
	0, 0, 636, 621, 1, 0, 0, 0, 636, 626, 1, 0, 0, 0, 636, 631, 1, 0, 0, 0, 
	637, 59, 1, 0, 0, 0, 638, 642, 3, 254, 127, 0, 639, 642, 3, 266, 133, 0, 
	640, 642, 3, 256, 128, 0, 641, 638, 1, 0, 0, 0, 641, 639, 1, 0, 0, 0, 641, 
	640, 1, 0, 0, 0, 642, 61, 1, 0, 0, 0, 643, 644, 5, 15, 0, 0, 644, 645, 
	3, 28, 14, 0, 645, 646, 5, 2, 0, 0, 646, 647, 3, 28, 14, 0, 647, 648, 5, 
	2, 0, 0, 648, 649, 3, 28, 14, 0, 649, 650, 5, 11, 0, 0, 650, 63, 1, 0, 
	0, 0, 651, 652, 5, 16, 0, 0, 652, 653, 3, 42, 21, 0, 653, 654, 5, 2, 0, 
	0, 654, 655, 3, 26, 13, 0, 655, 656, 5, 11, 0, 0, 656, 65, 1, 0, 0, 0, 
	657, 658, 3, 92, 46, 0, 658, 67, 1, 0, 0, 0, 659, 660, 3, 70, 35, 0, 660, 
	661, 5, 14, 0, 0, 661, 662, 3, 74, 37, 0, 662, 69, 1, 0, 0, 0, 663, 669, 
	3, 56, 28, 0, 664, 665, 5, 10, 0, 0, 665, 666, 3, 28, 14, 0, 666, 667, 
	5, 11, 0, 0, 667, 669, 1, 0, 0, 0, 668, 663, 1, 0, 0, 0, 668, 664, 1, 0, 
	0, 0, 669, 71, 1, 0, 0, 0, 670, 671, 3, 56, 28, 0, 671, 672, 5, 72, 0, 
	0, 672, 673, 3, 56, 28, 0, 673, 73, 1, 0, 0, 0, 674, 678, 3, 76, 38, 0, 
	675, 678, 3, 82, 41, 0, 676, 678, 3, 78, 39, 0, 677, 674, 1, 0, 0, 0, 677, 
	675, 1, 0, 0, 0, 677, 676, 1, 0, 0, 0, 678, 75, 1, 0, 0, 0, 679, 680, 3, 
	42, 21, 0, 680, 77, 1, 0, 0, 0, 681, 682, 3, 40, 20, 0, 682, 79, 1, 0, 
	0, 0, 683, 687, 3, 82, 41, 0, 684, 687, 3, 84, 42, 0, 685, 687, 3, 86, 
	43, 0, 686, 683, 1, 0, 0, 0, 686, 684, 1, 0, 0, 0, 686, 685, 1, 0, 0, 0, 
	687, 81, 1, 0, 0, 0, 688, 689, 3, 76, 38, 0, 689, 690, 5, 69, 0, 0, 690, 
	691, 3, 76, 38, 0, 691, 697, 1, 0, 0, 0, 692, 693, 3, 76, 38, 0, 693, 694, 
	5, 69, 0, 0, 694, 695, 3, 82, 41, 0, 695, 697, 1, 0, 0, 0, 696, 688, 1, 
	0, 0, 0, 696, 692, 1, 0, 0, 0, 697, 83, 1, 0, 0, 0, 698, 699, 6, 42, -1, 
	0, 699, 700, 3, 76, 38, 0, 700, 701, 5, 70, 0, 0, 701, 702, 3, 76, 38, 
	0, 702, 708, 1, 0, 0, 0, 703, 704, 10, 1, 0, 0, 704, 705, 5, 70, 0, 0, 
	705, 707, 3, 76, 38, 0, 706, 703, 1, 0, 0, 0, 707, 710, 1, 0, 0, 0, 708, 
	706, 1, 0, 0, 0, 708, 709, 1, 0, 0, 0, 709, 85, 1, 0, 0, 0, 710, 708, 1, 
	0, 0, 0, 711, 712, 6, 43, -1, 0, 712, 713, 3, 76, 38, 0, 713, 714, 5, 71, 
	0, 0, 714, 715, 3, 76, 38, 0, 715, 721, 1, 0, 0, 0, 716, 717, 10, 1, 0, 
	0, 717, 718, 5, 71, 0, 0, 718, 720, 3, 76, 38, 0, 719, 716, 1, 0, 0, 0, 
	720, 723, 1, 0, 0, 0, 721, 719, 1, 0, 0, 0, 721, 722, 1, 0, 0, 0, 722, 
	87, 1, 0, 0, 0, 723, 721, 1, 0, 0, 0, 724, 725, 3, 90, 45, 0, 725, 726, 
	5, 73, 0, 0, 726, 727, 3, 90, 45, 0, 727, 733, 1, 0, 0, 0, 728, 729, 5, 
	10, 0, 0, 729, 730, 3, 88, 44, 0, 730, 731, 5, 11, 0, 0, 731, 733, 1, 0, 
	0, 0, 732, 724, 1, 0, 0, 0, 732, 728, 1, 0, 0, 0, 733, 89, 1, 0, 0, 0, 
	734, 745, 5, 17, 0, 0, 735, 736, 5, 12, 0, 0, 736, 737, 3, 92, 46, 0, 737, 
	738, 5, 13, 0, 0, 738, 745, 1, 0, 0, 0, 739, 745, 5, 18, 0, 0, 740, 741, 
	5, 19, 0, 0, 741, 742, 3, 92, 46, 0, 742, 743, 5, 20, 0, 0, 743, 745, 1, 
	0, 0, 0, 744, 734, 1, 0, 0, 0, 744, 735, 1, 0, 0, 0, 744, 739, 1, 0, 0, 
	0, 744, 740, 1, 0, 0, 0, 745, 91, 1, 0, 0, 0, 746, 751, 3, 28, 14, 0, 747, 
	748, 5, 2, 0, 0, 748, 750, 3, 28, 14, 0, 749, 747, 1, 0, 0, 0, 750, 753, 
	1, 0, 0, 0, 751, 749, 1, 0, 0, 0, 751, 752, 1, 0, 0, 0, 752, 93, 1, 0, 
	0, 0, 753, 751, 1, 0, 0, 0, 754, 757, 3, 96, 48, 0, 755, 757, 3, 88, 44, 
	0, 756, 754, 1, 0, 0, 0, 756, 755, 1, 0, 0, 0, 757, 95, 1, 0, 0, 0, 758, 
	759, 3, 28, 14, 0, 759, 97, 1, 0, 0, 0, 760, 764, 3, 100, 50, 0, 761, 764, 
	3, 152, 76, 0, 762, 764, 3, 146, 73, 0, 763, 760, 1, 0, 0, 0, 763, 761, 
	1, 0, 0, 0, 763, 762, 1, 0, 0, 0, 764, 99, 1, 0, 0, 0, 765, 769, 3, 102, 
	51, 0, 766, 769, 3, 112, 56, 0, 767, 769, 3, 154, 77, 0, 768, 765, 1, 0, 
	0, 0, 768, 766, 1, 0, 0, 0, 768, 767, 1, 0, 0, 0, 769, 101, 1, 0, 0, 0, 
	770, 773, 3, 104, 52, 0, 771, 773, 3, 106, 53, 0, 772, 770, 1, 0, 0, 0, 
	772, 771, 1, 0, 0, 0, 773, 103, 1, 0, 0, 0, 774, 775, 3, 112, 56, 0, 775, 
	776, 3, 264, 132, 0, 776, 777, 3, 112, 56, 0, 777, 105, 1, 0, 0, 0, 778, 
	781, 3, 108, 54, 0, 779, 781, 3, 110, 55, 0, 780, 778, 1, 0, 0, 0, 780, 
	779, 1, 0, 0, 0, 781, 107, 1, 0, 0, 0, 782, 783, 6, 54, -1, 0, 783, 784, 
	3, 112, 56, 0, 784, 785, 5, 44, 0, 0, 785, 786, 3, 112, 56, 0, 786, 792, 
	1, 0, 0, 0, 787, 788, 10, 1, 0, 0, 788, 789, 5, 44, 0, 0, 789, 791, 3, 
	112, 56, 0, 790, 787, 1, 0, 0, 0, 791, 794, 1, 0, 0, 0, 792, 790, 1, 0, 
	0, 0, 792, 793, 1, 0, 0, 0, 793, 109, 1, 0, 0, 0, 794, 792, 1, 0, 0, 0, 
	795, 796, 6, 55, -1, 0, 796, 797, 3, 112, 56, 0, 797, 798, 5, 45, 0, 0, 
	798, 799, 3, 112, 56, 0, 799, 805, 1, 0, 0, 0, 800, 801, 10, 1, 0, 0, 801, 
	802, 5, 45, 0, 0, 802, 804, 3, 112, 56, 0, 803, 800, 1, 0, 0, 0, 804, 807, 
	1, 0, 0, 0, 805, 803, 1, 0, 0, 0, 805, 806, 1, 0, 0, 0, 806, 111, 1, 0, 
	0, 0, 807, 805, 1, 0, 0, 0, 808, 818, 3, 114, 57, 0, 809, 818, 3, 122, 
	61, 0, 810, 818, 3, 124, 62, 0, 811, 818, 3, 126, 63, 0, 812, 818, 3, 128, 
	64, 0, 813, 814, 5, 10, 0, 0, 814, 815, 3, 100, 50, 0, 815, 816, 5, 11, 
	0, 0, 816, 818, 1, 0, 0, 0, 817, 808, 1, 0, 0, 0, 817, 809, 1, 0, 0, 0, 
	817, 810, 1, 0, 0, 0, 817, 811, 1, 0, 0, 0, 817, 812, 1, 0, 0, 0, 817, 
	813, 1, 0, 0, 0, 818, 113, 1, 0, 0, 0, 819, 820, 3, 262, 131, 0, 820, 821, 
	5, 12, 0, 0, 821, 822, 3, 116, 58, 0, 822, 823, 5, 13, 0, 0, 823, 824, 
	5, 14, 0, 0, 824, 825, 3, 112, 56, 0, 825, 115, 1, 0, 0, 0, 826, 831, 3, 
	118, 59, 0, 827, 828, 5, 2, 0, 0, 828, 830, 3, 118, 59, 0, 829, 827, 1, 
	0, 0, 0, 830, 833, 1, 0, 0, 0, 831, 829, 1, 0, 0, 0, 831, 832, 1, 0, 0, 
	0, 832, 117, 1, 0, 0, 0, 833, 831, 1, 0, 0, 0, 834, 837, 3, 120, 60, 0, 
	835, 837, 3, 302, 151, 0, 836, 834, 1, 0, 0, 0, 836, 835, 1, 0, 0, 0, 837, 
	119, 1, 0, 0, 0, 838, 839, 3, 302, 151, 0, 839, 840, 5, 14, 0, 0, 840, 
	841, 3, 164, 82, 0, 841, 121, 1, 0, 0, 0, 842, 843, 3, 268, 134, 0, 843, 
	844, 3, 112, 56, 0, 844, 847, 1, 0, 0, 0, 845, 847, 3, 200, 100, 0, 846, 
	842, 1, 0, 0, 0, 846, 845, 1, 0, 0, 0, 847, 123, 1, 0, 0, 0, 848, 849, 
	3, 202, 101, 0, 849, 125, 1, 0, 0, 0, 850, 851, 5, 21, 0, 0, 851, 852, 
	3, 100, 50, 0, 852, 853, 5, 2, 0, 0, 853, 854, 3, 100, 50, 0, 854, 855, 
	5, 2, 0, 0, 855, 856, 3, 100, 50, 0, 856, 857, 5, 11, 0, 0, 857, 127, 1, 
	0, 0, 0, 858, 859, 5, 22, 0, 0, 859, 860, 3, 130, 65, 0, 860, 861, 5, 2, 
	0, 0, 861, 862, 3, 98, 49, 0, 862, 863, 5, 11, 0, 0, 863, 871, 1, 0, 0, 
	0, 864, 865, 5, 23, 0, 0, 865, 866, 3, 138, 69, 0, 866, 867, 5, 2, 0, 0, 
	867, 868, 3, 98, 49, 0, 868, 869, 5, 11, 0, 0, 869, 871, 1, 0, 0, 0, 870, 
	858, 1, 0, 0, 0, 870, 864, 1, 0, 0, 0, 871, 129, 1, 0, 0, 0, 872, 878, 
	3, 134, 67, 0, 873, 874, 5, 12, 0, 0, 874, 875, 3, 132, 66, 0, 875, 876, 
	5, 13, 0, 0, 876, 878, 1, 0, 0, 0, 877, 872, 1, 0, 0, 0, 877, 873, 1, 0, 
	0, 0, 878, 131, 1, 0, 0, 0, 879, 884, 3, 134, 67, 0, 880, 881, 5, 2, 0, 
	0, 881, 883, 3, 134, 67, 0, 882, 880, 1, 0, 0, 0, 883, 886, 1, 0, 0, 0, 
	884, 882, 1, 0, 0, 0, 884, 885, 1, 0, 0, 0, 885, 133, 1, 0, 0, 0, 886, 
	884, 1, 0, 0, 0, 887, 888, 5, 57, 0, 0, 888, 889, 5, 12, 0, 0, 889, 890, 
	3, 116, 58, 0, 890, 891, 5, 13, 0, 0, 891, 892, 5, 14, 0, 0, 892, 893, 
	3, 134, 67, 0, 893, 896, 1, 0, 0, 0, 894, 896, 3, 136, 68, 0, 895, 887, 
	1, 0, 0, 0, 895, 894, 1, 0, 0, 0, 896, 135, 1, 0, 0, 0, 897, 898, 3, 214, 
	107, 0, 898, 899, 5, 56, 0, 0, 899, 900, 3, 226, 113, 0, 900, 906, 1, 0, 
	0, 0, 901, 902, 5, 10, 0, 0, 902, 903, 3, 136, 68, 0, 903, 904, 5, 11, 
	0, 0, 904, 906, 1, 0, 0, 0, 905, 897, 1, 0, 0, 0, 905, 901, 1, 0, 0, 0, 
	906, 137, 1, 0, 0, 0, 907, 913, 3, 142, 71, 0, 908, 909, 5, 12, 0, 0, 909, 
	910, 3, 140, 70, 0, 910, 911, 5, 13, 0, 0, 911, 913, 1, 0, 0, 0, 912, 907, 
	1, 0, 0, 0, 912, 908, 1, 0, 0, 0, 913, 139, 1, 0, 0, 0, 914, 919, 3, 142, 
	71, 0, 915, 916, 5, 2, 0, 0, 916, 918, 3, 142, 71, 0, 917, 915, 1, 0, 0, 
	0, 918, 921, 1, 0, 0, 0, 919, 917, 1, 0, 0, 0, 919, 920, 1, 0, 0, 0, 920, 
	141, 1, 0, 0, 0, 921, 919, 1, 0, 0, 0, 922, 923, 5, 57, 0, 0, 923, 924, 
	5, 12, 0, 0, 924, 925, 3, 116, 58, 0, 925, 926, 5, 13, 0, 0, 926, 927, 
	5, 14, 0, 0, 927, 928, 3, 142, 71, 0, 928, 931, 1, 0, 0, 0, 929, 931, 3, 
	144, 72, 0, 930, 922, 1, 0, 0, 0, 930, 929, 1, 0, 0, 0, 931, 143, 1, 0, 
	0, 0, 932, 933, 3, 204, 102, 0, 933, 934, 5, 46, 0, 0, 934, 935, 3, 112, 
	56, 0, 935, 941, 1, 0, 0, 0, 936, 937, 5, 10, 0, 0, 937, 938, 3, 144, 72, 
	0, 938, 939, 5, 11, 0, 0, 939, 941, 1, 0, 0, 0, 940, 932, 1, 0, 0, 0, 940, 
	936, 1, 0, 0, 0, 941, 145, 1, 0, 0, 0, 942, 943, 3, 148, 74, 0, 943, 944, 
	5, 73, 0, 0, 944, 945, 3, 148, 74, 0, 945, 951, 1, 0, 0, 0, 946, 947, 5, 
	10, 0, 0, 947, 948, 3, 146, 73, 0, 948, 949, 5, 11, 0, 0, 949, 951, 1, 
	0, 0, 0, 950, 942, 1, 0, 0, 0, 950, 946, 1, 0, 0, 0, 951, 147, 1, 0, 0, 
	0, 952, 958, 5, 17, 0, 0, 953, 954, 5, 12, 0, 0, 954, 955, 3, 150, 75, 
	0, 955, 956, 5, 13, 0, 0, 956, 958, 1, 0, 0, 0, 957, 952, 1, 0, 0, 0, 957, 
	953, 1, 0, 0, 0, 958, 149, 1, 0, 0, 0, 959, 964, 3, 100, 50, 0, 960, 961, 
	5, 2, 0, 0, 961, 963, 3, 100, 50, 0, 962, 960, 1, 0, 0, 0, 963, 966, 1, 
	0, 0, 0, 964, 962, 1, 0, 0, 0, 964, 965, 1, 0, 0, 0, 965, 151, 1, 0, 0, 
	0, 966, 964, 1, 0, 0, 0, 967, 968, 3, 280, 140, 0, 968, 969, 5, 14, 0, 
	0, 969, 970, 3, 156, 78, 0, 970, 976, 1, 0, 0, 0, 971, 972, 5, 10, 0, 0, 
	972, 973, 3, 152, 76, 0, 973, 974, 5, 11, 0, 0, 974, 976, 1, 0, 0, 0, 975, 
	967, 1, 0, 0, 0, 975, 971, 1, 0, 0, 0, 976, 153, 1, 0, 0, 0, 977, 978, 
	3, 280, 140, 0, 978, 979, 5, 72, 0, 0, 979, 980, 3, 278, 139, 0, 980, 155, 
	1, 0, 0, 0, 981, 989, 3, 164, 82, 0, 982, 989, 3, 168, 84, 0, 983, 989, 
	3, 158, 79, 0, 984, 985, 5, 10, 0, 0, 985, 986, 3, 156, 78, 0, 986, 987, 
	5, 11, 0, 0, 987, 989, 1, 0, 0, 0, 988, 981, 1, 0, 0, 0, 988, 982, 1, 0, 
	0, 0, 988, 983, 1, 0, 0, 0, 988, 984, 1, 0, 0, 0, 989, 157, 1, 0, 0, 0, 
	990, 991, 5, 54, 0, 0, 991, 992, 5, 12, 0, 0, 992, 993, 3, 116, 58, 0, 
	993, 994, 5, 13, 0, 0, 994, 995, 5, 14, 0, 0, 995, 996, 3, 160, 80, 0, 
	996, 159, 1, 0, 0, 0, 997, 1003, 3, 164, 82, 0, 998, 999, 5, 10, 0, 0, 
	999, 1000, 3, 168, 84, 0, 1000, 1001, 5, 11, 0, 0, 1001, 1003, 1, 0, 0, 
	0, 1002, 997, 1, 0, 0, 0, 1002, 998, 1, 0, 0, 0, 1003, 161, 1, 0, 0, 0, 
	1004, 1010, 3, 164, 82, 0, 1005, 1006, 5, 10, 0, 0, 1006, 1007, 3, 170, 
	85, 0, 1007, 1008, 5, 11, 0, 0, 1008, 1010, 1, 0, 0, 0, 1009, 1004, 1, 
	0, 0, 0, 1009, 1005, 1, 0, 0, 0, 1010, 163, 1, 0, 0, 0, 1011, 1020, 3, 
	270, 135, 0, 1012, 1020, 3, 274, 137, 0, 1013, 1014, 3, 272, 136, 0, 1014, 
	1015, 5, 10, 0, 0, 1015, 1016, 3, 166, 83, 0, 1016, 1017, 5, 11, 0, 0, 
	1017, 1020, 1, 0, 0, 0, 1018, 1020, 3, 302, 151, 0, 1019, 1011, 1, 0, 0, 
	0, 1019, 1012, 1, 0, 0, 0, 1019, 1013, 1, 0, 0, 0, 1019, 1018, 1, 0, 0, 
	0, 1020, 165, 1, 0, 0, 0, 1021, 1026, 3, 164, 82, 0, 1022, 1023, 5, 2, 
	0, 0, 1023, 1025, 3, 164, 82, 0, 1024, 1022, 1, 0, 0, 0, 1025, 1028, 1, 
	0, 0, 0, 1026, 1024, 1, 0, 0, 0, 1026, 1027, 1, 0, 0, 0, 1027, 167, 1, 
	0, 0, 0, 1028, 1026, 1, 0, 0, 0, 1029, 1030, 3, 162, 81, 0, 1030, 1031, 
	5, 69, 0, 0, 1031, 1032, 3, 164, 82, 0, 1032, 169, 1, 0, 0, 0, 1033, 1034, 
	6, 85, -1, 0, 1034, 1035, 3, 162, 81, 0, 1035, 1036, 5, 70, 0, 0, 1036, 
	1037, 3, 164, 82, 0, 1037, 1043, 1, 0, 0, 0, 1038, 1039, 10, 1, 0, 0, 1039, 
	1040, 5, 70, 0, 0, 1040, 1042, 3, 164, 82, 0, 1041, 1038, 1, 0, 0, 0, 1042, 
	1045, 1, 0, 0, 0, 1043, 1041, 1, 0, 0, 0, 1043, 1044, 1, 0, 0, 0, 1044, 
	171, 1, 0, 0, 0, 1045, 1043, 1, 0, 0, 0, 1046, 1049, 3, 174, 87, 0, 1047, 
	1049, 3, 152, 76, 0, 1048, 1046, 1, 0, 0, 0, 1048, 1047, 1, 0, 0, 0, 1049, 
	173, 1, 0, 0, 0, 1050, 1053, 3, 176, 88, 0, 1051, 1053, 3, 242, 121, 0, 
	1052, 1050, 1, 0, 0, 0, 1052, 1051, 1, 0, 0, 0, 1053, 175, 1, 0, 0, 0, 
	1054, 1055, 5, 57, 0, 0, 1055, 1056, 5, 12, 0, 0, 1056, 1057, 3, 116, 58, 
	0, 1057, 1058, 5, 13, 0, 0, 1058, 1059, 5, 14, 0, 0, 1059, 1060, 3, 242, 
	121, 0, 1060, 177, 1, 0, 0, 0, 1061, 1064, 3, 180, 90, 0, 1062, 1064, 3, 
	236, 118, 0, 1063, 1061, 1, 0, 0, 0, 1063, 1062, 1, 0, 0, 0, 1064, 179, 
	1, 0, 0, 0, 1065, 1068, 3, 182, 91, 0, 1066, 1068, 3, 192, 96, 0, 1067, 
	1065, 1, 0, 0, 0, 1067, 1066, 1, 0, 0, 0, 1068, 181, 1, 0, 0, 0, 1069, 
	1072, 3, 184, 92, 0, 1070, 1072, 3, 186, 93, 0, 1071, 1069, 1, 0, 0, 0, 
	1071, 1070, 1, 0, 0, 0, 1072, 183, 1, 0, 0, 0, 1073, 1074, 3, 192, 96, 
	0, 1074, 1075, 3, 264, 132, 0, 1075, 1076, 3, 192, 96, 0, 1076, 185, 1, 
	0, 0, 0, 1077, 1080, 3, 188, 94, 0, 1078, 1080, 3, 190, 95, 0, 1079, 1077, 
	1, 0, 0, 0, 1079, 1078, 1, 0, 0, 0, 1080, 187, 1, 0, 0, 0, 1081, 1082, 
	6, 94, -1, 0, 1082, 1083, 3, 192, 96, 0, 1083, 1084, 5, 44, 0, 0, 1084, 
	1085, 3, 192, 96, 0, 1085, 1091, 1, 0, 0, 0, 1086, 1087, 10, 1, 0, 0, 1087, 
	1088, 5, 44, 0, 0, 1088, 1090, 3, 192, 96, 0, 1089, 1086, 1, 0, 0, 0, 1090, 
	1093, 1, 0, 0, 0, 1091, 1089, 1, 0, 0, 0, 1091, 1092, 1, 0, 0, 0, 1092, 
	189, 1, 0, 0, 0, 1093, 1091, 1, 0, 0, 0, 1094, 1095, 6, 95, -1, 0, 1095, 
	1096, 3, 192, 96, 0, 1096, 1097, 5, 45, 0, 0, 1097, 1098, 3, 192, 96, 0, 
	1098, 1104, 1, 0, 0, 0, 1099, 1100, 10, 1, 0, 0, 1100, 1101, 5, 45, 0, 
	0, 1101, 1103, 3, 192, 96, 0, 1102, 1099, 1, 0, 0, 0, 1103, 1106, 1, 0, 
	0, 0, 1104, 1102, 1, 0, 0, 0, 1104, 1105, 1, 0, 0, 0, 1105, 191, 1, 0, 
	0, 0, 1106, 1104, 1, 0, 0, 0, 1107, 1115, 3, 194, 97, 0, 1108, 1115, 3, 
	198, 99, 0, 1109, 1115, 3, 202, 101, 0, 1110, 1111, 5, 10, 0, 0, 1111, 
	1112, 3, 180, 90, 0, 1112, 1113, 5, 11, 0, 0, 1113, 1115, 1, 0, 0, 0, 1114, 
	1107, 1, 0, 0, 0, 1114, 1108, 1, 0, 0, 0, 1114, 1109, 1, 0, 0, 0, 1114, 
	1110, 1, 0, 0, 0, 1115, 193, 1, 0, 0, 0, 1116, 1117, 3, 262, 131, 0, 1117, 
	1118, 5, 12, 0, 0, 1118, 1119, 3, 196, 98, 0, 1119, 1120, 5, 13, 0, 0, 
	1120, 1121, 5, 14, 0, 0, 1121, 1122, 3, 192, 96, 0, 1122, 195, 1, 0, 0, 
	0, 1123, 1128, 3, 302, 151, 0, 1124, 1125, 5, 2, 0, 0, 1125, 1127, 3, 302, 
	151, 0, 1126, 1124, 1, 0, 0, 0, 1127, 1130, 1, 0, 0, 0, 1128, 1126, 1, 
	0, 0, 0, 1128, 1129, 1, 0, 0, 0, 1129, 197, 1, 0, 0, 0, 1130, 1128, 1, 
	0, 0, 0, 1131, 1132, 3, 268, 134, 0, 1132, 1133, 3, 192, 96, 0, 1133, 1136, 
	1, 0, 0, 0, 1134, 1136, 3, 200, 100, 0, 1135, 1131, 1, 0, 0, 0, 1135, 1134, 
	1, 0, 0, 0, 1136, 199, 1, 0, 0, 0, 1137, 1138, 3, 226, 113, 0, 1138, 1139, 
	5, 55, 0, 0, 1139, 1140, 3, 226, 113, 0, 1140, 201, 1, 0, 0, 0, 1141, 1145, 
	3, 204, 102, 0, 1142, 1145, 3, 206, 103, 0, 1143, 1145, 3, 212, 106, 0, 
	1144, 1141, 1, 0, 0, 0, 1144, 1142, 1, 0, 0, 0, 1144, 1143, 1, 0, 0, 0, 
	1145, 203, 1, 0, 0, 0, 1146, 1147, 3, 214, 107, 0, 1147, 205, 1, 0, 0, 
	0, 1148, 1151, 3, 208, 104, 0, 1149, 1151, 3, 210, 105, 0, 1150, 1148, 
	1, 0, 0, 0, 1150, 1149, 1, 0, 0, 0, 1151, 207, 1, 0, 0, 0, 1152, 1153, 
	3, 216, 108, 0, 1153, 209, 1, 0, 0, 0, 1154, 1155, 3, 226, 113, 0, 1155, 
	1156, 3, 286, 143, 0, 1156, 1157, 3, 226, 113, 0, 1157, 211, 1, 0, 0, 0, 
	1158, 1159, 3, 222, 111, 0, 1159, 213, 1, 0, 0, 0, 1160, 1167, 3, 288, 
	144, 0, 1161, 1162, 3, 290, 145, 0, 1162, 1163, 5, 10, 0, 0, 1163, 1164, 
	3, 224, 112, 0, 1164, 1165, 5, 11, 0, 0, 1165, 1167, 1, 0, 0, 0, 1166, 
	1160, 1, 0, 0, 0, 1166, 1161, 1, 0, 0, 0, 1167, 215, 1, 0, 0, 0, 1168, 
	1171, 3, 300, 150, 0, 1169, 1171, 3, 218, 109, 0, 1170, 1168, 1, 0, 0, 
	0, 1170, 1169, 1, 0, 0, 0, 1171, 217, 1, 0, 0, 0, 1172, 1173, 3, 220, 110, 
	0, 1173, 219, 1, 0, 0, 0, 1174, 1181, 3, 296, 148, 0, 1175, 1176, 3, 298, 
	149, 0, 1176, 1177, 5, 10, 0, 0, 1177, 1178, 3, 224, 112, 0, 1178, 1179, 
	5, 11, 0, 0, 1179, 1181, 1, 0, 0, 0, 1180, 1174, 1, 0, 0, 0, 1180, 1175, 
	1, 0, 0, 0, 1181, 221, 1, 0, 0, 0, 1182, 1189, 3, 292, 146, 0, 1183, 1184, 
	3, 294, 147, 0, 1184, 1185, 5, 10, 0, 0, 1185, 1186, 3, 224, 112, 0, 1186, 
	1187, 5, 11, 0, 0, 1187, 1189, 1, 0, 0, 0, 1188, 1182, 1, 0, 0, 0, 1188, 
	1183, 1, 0, 0, 0, 1189, 223, 1, 0, 0, 0, 1190, 1195, 3, 226, 113, 0, 1191, 
	1192, 5, 2, 0, 0, 1192, 1194, 3, 226, 113, 0, 1193, 1191, 1, 0, 0, 0, 1194, 
	1197, 1, 0, 0, 0, 1195, 1193, 1, 0, 0, 0, 1195, 1196, 1, 0, 0, 0, 1196, 
	225, 1, 0, 0, 0, 1197, 1195, 1, 0, 0, 0, 1198, 1204, 3, 228, 114, 0, 1199, 
	1204, 3, 302, 151, 0, 1200, 1204, 3, 230, 115, 0, 1201, 1204, 3, 232, 116, 
	0, 1202, 1204, 3, 234, 117, 0, 1203, 1198, 1, 0, 0, 0, 1203, 1199, 1, 0, 
	0, 0, 1203, 1200, 1, 0, 0, 0, 1203, 1201, 1, 0, 0, 0, 1203, 1202, 1, 0, 
	0, 0, 1204, 227, 1, 0, 0, 0, 1205, 1209, 3, 214, 107, 0, 1206, 1209, 3, 
	216, 108, 0, 1207, 1209, 3, 222, 111, 0, 1208, 1205, 1, 0, 0, 0, 1208, 
	1206, 1, 0, 0, 0, 1208, 1207, 1, 0, 0, 0, 1209, 229, 1, 0, 0, 0, 1210, 
	1211, 5, 24, 0, 0, 1211, 1212, 3, 100, 50, 0, 1212, 1213, 5, 2, 0, 0, 1213, 
	1214, 3, 226, 113, 0, 1214, 1215, 5, 2, 0, 0, 1215, 1216, 3, 226, 113, 
	0, 1216, 1217, 5, 11, 0, 0, 1217, 231, 1, 0, 0, 0, 1218, 1219, 5, 25, 0, 
	0, 1219, 1220, 3, 138, 69, 0, 1220, 1221, 5, 2, 0, 0, 1221, 1222, 3, 226, 
	113, 0, 1222, 1223, 5, 11, 0, 0, 1223, 1231, 1, 0, 0, 0, 1224, 1225, 5, 
	26, 0, 0, 1225, 1226, 3, 130, 65, 0, 1226, 1227, 5, 2, 0, 0, 1227, 1228, 
	3, 226, 113, 0, 1228, 1229, 5, 11, 0, 0, 1229, 1231, 1, 0, 0, 0, 1230, 
	1218, 1, 0, 0, 0, 1230, 1224, 1, 0, 0, 0, 1231, 233, 1, 0, 0, 0, 1232, 
	1238, 5, 18, 0, 0, 1233, 1234, 5, 19, 0, 0, 1234, 1235, 3, 224, 112, 0, 
	1235, 1236, 5, 20, 0, 0, 1236, 1238, 1, 0, 0, 0, 1237, 1232, 1, 0, 0, 0, 
	1237, 1233, 1, 0, 0, 0, 1238, 235, 1, 0, 0, 0, 1239, 1240, 3, 238, 119, 
	0, 1240, 1241, 5, 73, 0, 0, 1241, 1242, 3, 238, 119, 0, 1242, 1248, 1, 
	0, 0, 0, 1243, 1244, 5, 10, 0, 0, 1244, 1245, 3, 236, 118, 0, 1245, 1246, 
	5, 11, 0, 0, 1246, 1248, 1, 0, 0, 0, 1247, 1239, 1, 0, 0, 0, 1247, 1243, 
	1, 0, 0, 0, 1248, 237, 1, 0, 0, 0, 1249, 1255, 5, 17, 0, 0, 1250, 1251, 
	5, 12, 0, 0, 1251, 1252, 3, 240, 120, 0, 1252, 1253, 5, 13, 0, 0, 1253, 
	1255, 1, 0, 0, 0, 1254, 1249, 1, 0, 0, 0, 1254, 1250, 1, 0, 0, 0, 1255, 
	239, 1, 0, 0, 0, 1256, 1261, 3, 180, 90, 0, 1257, 1258, 5, 2, 0, 0, 1258, 
	1260, 3, 180, 90, 0, 1259, 1257, 1, 0, 0, 0, 1260, 1263, 1, 0, 0, 0, 1261, 
	1259, 1, 0, 0, 0, 1261, 1262, 1, 0, 0, 0, 1262, 241, 1, 0, 0, 0, 1263, 
	1261, 1, 0, 0, 0, 1264, 1270, 3, 244, 122, 0, 1265, 1266, 5, 10, 0, 0, 
	1266, 1267, 3, 244, 122, 0, 1267, 1268, 5, 11, 0, 0, 1268, 1270, 1, 0, 
	0, 0, 1269, 1264, 1, 0, 0, 0, 1269, 1265, 1, 0, 0, 0, 1270, 243, 1, 0, 
	0, 0, 1271, 1272, 6, 122, -1, 0, 1272, 1273, 3, 246, 123, 0, 1273, 1279, 
	1, 0, 0, 0, 1274, 1275, 10, 1, 0, 0, 1275, 1276, 5, 44, 0, 0, 1276, 1278, 
	3, 246, 123, 0, 1277, 1274, 1, 0, 0, 0, 1278, 1281, 1, 0, 0, 0, 1279, 1277, 
	1, 0, 0, 0, 1279, 1280, 1, 0, 0, 0, 1280, 245, 1, 0, 0, 0, 1281, 1279, 
	1, 0, 0, 0, 1282, 1287, 3, 202, 101, 0, 1283, 1284, 5, 52, 0, 0, 1284, 
	1287, 3, 202, 101, 0, 1285, 1287, 3, 200, 100, 0, 1286, 1282, 1, 0, 0, 
	0, 1286, 1283, 1, 0, 0, 0, 1286, 1285, 1, 0, 0, 0, 1287, 247, 1, 0, 0, 
	0, 1288, 1292, 3, 262, 131, 0, 1289, 1292, 3, 250, 125, 0, 1290, 1292, 
	3, 252, 126, 0, 1291, 1288, 1, 0, 0, 0, 1291, 1289, 1, 0, 0, 0, 1291, 1290, 
	1, 0, 0, 0, 1292, 249, 1, 0, 0, 0, 1293, 1294, 7, 0, 0, 0, 1294, 251, 1, 
	0, 0, 0, 1295, 1296, 7, 1, 0, 0, 1296, 253, 1, 0, 0, 0, 1297, 1302, 5, 
	56, 0, 0, 1298, 1302, 5, 55, 0, 0, 1299, 1302, 3, 264, 132, 0, 1300, 1302, 
	5, 68, 0, 0, 1301, 1297, 1, 0, 0, 0, 1301, 1298, 1, 0, 0, 0, 1301, 1299, 
	1, 0, 0, 0, 1301, 1300, 1, 0, 0, 0, 1302, 255, 1, 0, 0, 0, 1303, 1306, 
	3, 268, 134, 0, 1304, 1306, 3, 258, 129, 0, 1305, 1303, 1, 0, 0, 0, 1305, 
	1304, 1, 0, 0, 0, 1306, 257, 1, 0, 0, 0, 1307, 1308, 7, 2, 0, 0, 1308, 
	259, 1, 0, 0, 0, 1309, 1312, 3, 264, 132, 0, 1310, 1312, 5, 68, 0, 0, 1311, 
	1309, 1, 0, 0, 0, 1311, 1310, 1, 0, 0, 0, 1312, 261, 1, 0, 0, 0, 1313, 
	1314, 7, 3, 0, 0, 1314, 263, 1, 0, 0, 0, 1315, 1316, 7, 4, 0, 0, 1316, 
	265, 1, 0, 0, 0, 1317, 1318, 7, 5, 0, 0, 1318, 267, 1, 0, 0, 0, 1319, 1320, 
	5, 52, 0, 0, 1320, 269, 1, 0, 0, 0, 1321, 1322, 3, 272, 136, 0, 1322, 271, 
	1, 0, 0, 0, 1323, 1324, 3, 392, 196, 0, 1324, 273, 1, 0, 0, 0, 1325, 1326, 
	5, 91, 0, 0, 1326, 275, 1, 0, 0, 0, 1327, 1328, 3, 396, 198, 0, 1328, 277, 
	1, 0, 0, 0, 1329, 1332, 3, 280, 140, 0, 1330, 1332, 3, 296, 148, 0, 1331, 
	1329, 1, 0, 0, 0, 1331, 1330, 1, 0, 0, 0, 1332, 279, 1, 0, 0, 0, 1333, 
	1336, 3, 288, 144, 0, 1334, 1336, 3, 292, 146, 0, 1335, 1333, 1, 0, 0, 
	0, 1335, 1334, 1, 0, 0, 0, 1336, 281, 1, 0, 0, 0, 1337, 1338, 5, 91, 0, 
	0, 1338, 283, 1, 0, 0, 0, 1339, 1340, 5, 91, 0, 0, 1340, 285, 1, 0, 0, 
	0, 1341, 1342, 7, 6, 0, 0, 1342, 287, 1, 0, 0, 0, 1343, 1344, 3, 290, 145, 
	0, 1344, 289, 1, 0, 0, 0, 1345, 1346, 3, 392, 196, 0, 1346, 291, 1, 0, 
	0, 0, 1347, 1348, 3, 294, 147, 0, 1348, 293, 1, 0, 0, 0, 1349, 1350, 3, 
	396, 198, 0, 1350, 295, 1, 0, 0, 0, 1351, 1352, 3, 298, 149, 0, 1352, 297, 
	1, 0, 0, 0, 1353, 1354, 3, 394, 197, 0, 1354, 299, 1, 0, 0, 0, 1355, 1358, 
	3, 398, 199, 0, 1356, 1358, 5, 96, 0, 0, 1357, 1355, 1, 0, 0, 0, 1357, 
	1356, 1, 0, 0, 0, 1358, 301, 1, 0, 0, 0, 1359, 1360, 5, 93, 0, 0, 1360, 
	303, 1, 0, 0, 0, 1361, 1370, 3, 308, 154, 0, 1362, 1370, 3, 322, 161, 0, 
	1363, 1370, 3, 326, 163, 0, 1364, 1370, 5, 94, 0, 0, 1365, 1366, 5, 12, 
	0, 0, 1366, 1367, 3, 306, 153, 0, 1367, 1368, 5, 13, 0, 0, 1368, 1370, 
	1, 0, 0, 0, 1369, 1361, 1, 0, 0, 0, 1369, 1362, 1, 0, 0, 0, 1369, 1363, 
	1, 0, 0, 0, 1369, 1364, 1, 0, 0, 0, 1369, 1365, 1, 0, 0, 0, 1370, 305, 
	1, 0, 0, 0, 1371, 1376, 3, 304, 152, 0, 1372, 1373, 5, 2, 0, 0, 1373, 1375, 
	3, 304, 152, 0, 1374, 1372, 1, 0, 0, 0, 1375, 1378, 1, 0, 0, 0, 1376, 1374, 
	1, 0, 0, 0, 1376, 1377, 1, 0, 0, 0, 1377, 307, 1, 0, 0, 0, 1378, 1376, 
	1, 0, 0, 0, 1379, 1382, 3, 390, 195, 0, 1380, 1382, 3, 310, 155, 0, 1381, 
	1379, 1, 0, 0, 0, 1381, 1380, 1, 0, 0, 0, 1382, 309, 1, 0, 0, 0, 1383, 
	1384, 5, 27, 0, 0, 1384, 1385, 3, 312, 156, 0, 1385, 1386, 5, 2, 0, 0, 
	1386, 1387, 3, 342, 171, 0, 1387, 1388, 5, 2, 0, 0, 1388, 1389, 3, 314, 
	157, 0, 1389, 1390, 5, 11, 0, 0, 1390, 311, 1, 0, 0, 0, 1391, 1392, 3, 
	392, 196, 0, 1392, 313, 1, 0, 0, 0, 1393, 1399, 5, 17, 0, 0, 1394, 1395, 
	5, 12, 0, 0, 1395, 1396, 3, 316, 158, 0, 1396, 1397, 5, 13, 0, 0, 1397, 
	1399, 1, 0, 0, 0, 1398, 1393, 1, 0, 0, 0, 1398, 1394, 1, 0, 0, 0, 1399, 
	315, 1, 0, 0, 0, 1400, 1405, 3, 318, 159, 0, 1401, 1402, 5, 2, 0, 0, 1402, 
	1404, 3, 318, 159, 0, 1403, 1401, 1, 0, 0, 0, 1404, 1407, 1, 0, 0, 0, 1405, 
	1403, 1, 0, 0, 0, 1405, 1406, 1, 0, 0, 0, 1406, 317, 1, 0, 0, 0, 1407, 
	1405, 1, 0, 0, 0, 1408, 1410, 3, 304, 152, 0, 1409, 1411, 3, 320, 160, 
	0, 1410, 1409, 1, 0, 0, 0, 1410, 1411, 1, 0, 0, 0, 1411, 319, 1, 0, 0, 
	0, 1412, 1413, 5, 14, 0, 0, 1413, 1414, 3, 386, 193, 0, 1414, 321, 1, 0, 
	0, 0, 1415, 1416, 5, 28, 0, 0, 1416, 1418, 3, 324, 162, 0, 1417, 1419, 
	3, 340, 170, 0, 1418, 1417, 1, 0, 0, 0, 1418, 1419, 1, 0, 0, 0, 1419, 1420, 
	1, 0, 0, 0, 1420, 1421, 5, 11, 0, 0, 1421, 323, 1, 0, 0, 0, 1422, 1423, 
	5, 94, 0, 0, 1423, 325, 1, 0, 0, 0, 1424, 1428, 3, 328, 164, 0, 1425, 1428, 
	3, 332, 166, 0, 1426, 1428, 3, 336, 168, 0, 1427, 1424, 1, 0, 0, 0, 1427, 
	1425, 1, 0, 0, 0, 1427, 1426, 1, 0, 0, 0, 1428, 327, 1, 0, 0, 0, 1429, 
	1430, 5, 29, 0, 0, 1430, 1432, 3, 400, 200, 0, 1431, 1433, 3, 330, 165, 
	0, 1432, 1431, 1, 0, 0, 0, 1432, 1433, 1, 0, 0, 0, 1433, 1434, 1, 0, 0, 
	0, 1434, 1435, 5, 11, 0, 0, 1435, 329, 1, 0, 0, 0, 1436, 1437, 5, 2, 0, 
	0, 1437, 1438, 3, 390, 195, 0, 1438, 331, 1, 0, 0, 0, 1439, 1440, 5, 30, 
	0, 0, 1440, 1442, 3, 334, 167, 0, 1441, 1443, 3, 340, 170, 0, 1442, 1441, 
	1, 0, 0, 0, 1442, 1443, 1, 0, 0, 0, 1443, 1444, 1, 0, 0, 0, 1444, 1445, 
	5, 11, 0, 0, 1445, 333, 1, 0, 0, 0, 1446, 1447, 5, 94, 0, 0, 1447, 335, 
	1, 0, 0, 0, 1448, 1449, 5, 31, 0, 0, 1449, 1451, 3, 338, 169, 0, 1450, 
	1452, 3, 340, 170, 0, 1451, 1450, 1, 0, 0, 0, 1451, 1452, 1, 0, 0, 0, 1452, 
	1453, 1, 0, 0, 0, 1453, 1454, 5, 11, 0, 0, 1454, 337, 1, 0, 0, 0, 1455, 
	1456, 3, 392, 196, 0, 1456, 339, 1, 0, 0, 0, 1457, 1458, 5, 2, 0, 0, 1458, 
	1459, 3, 342, 171, 0, 1459, 341, 1, 0, 0, 0, 1460, 1467, 5, 17, 0, 0, 1461, 
	1462, 5, 12, 0, 0, 1462, 1463, 3, 344, 172, 0, 1463, 1464, 5, 13, 0, 0, 
	1464, 1467, 1, 0, 0, 0, 1465, 1467, 3, 386, 193, 0, 1466, 1460, 1, 0, 0, 
	0, 1466, 1461, 1, 0, 0, 0, 1466, 1465, 1, 0, 0, 0, 1467, 343, 1, 0, 0, 
	0, 1468, 1473, 3, 346, 173, 0, 1469, 1470, 5, 2, 0, 0, 1470, 1472, 3, 346, 
	173, 0, 1471, 1469, 1, 0, 0, 0, 1472, 1475, 1, 0, 0, 0, 1473, 1471, 1, 
	0, 0, 0, 1473, 1474, 1, 0, 0, 0, 1474, 345, 1, 0, 0, 0, 1475, 1473, 1, 
	0, 0, 0, 1476, 1480, 3, 348, 174, 0, 1477, 1480, 3, 354, 177, 0, 1478, 
	1480, 3, 382, 191, 0, 1479, 1476, 1, 0, 0, 0, 1479, 1477, 1, 0, 0, 0, 1479, 
	1478, 1, 0, 0, 0, 1480, 347, 1, 0, 0, 0, 1481, 1484, 3, 350, 175, 0, 1482, 
	1484, 3, 352, 176, 0, 1483, 1481, 1, 0, 0, 0, 1483, 1482, 1, 0, 0, 0, 1484, 
	349, 1, 0, 0, 0, 1485, 1486, 5, 32, 0, 0, 1486, 1487, 3, 392, 196, 0, 1487, 
	1488, 5, 11, 0, 0, 1488, 351, 1, 0, 0, 0, 1489, 1490, 5, 33, 0, 0, 1490, 
	1491, 3, 392, 196, 0, 1491, 1492, 5, 11, 0, 0, 1492, 353, 1, 0, 0, 0, 1493, 
	1498, 3, 356, 178, 0, 1494, 1498, 3, 362, 181, 0, 1495, 1498, 3, 366, 183, 
	0, 1496, 1498, 3, 364, 182, 0, 1497, 1493, 1, 0, 0, 0, 1497, 1494, 1, 0, 
	0, 0, 1497, 1495, 1, 0, 0, 0, 1497, 1496, 1, 0, 0, 0, 1498, 355, 1, 0, 
	0, 0, 1499, 1500, 5, 34, 0, 0, 1500, 1501, 3, 358, 179, 0, 1501, 1502, 
	5, 11, 0, 0, 1502, 1505, 1, 0, 0, 0, 1503, 1505, 3, 360, 180, 0, 1504, 
	1499, 1, 0, 0, 0, 1504, 1503, 1, 0, 0, 0, 1505, 357, 1, 0, 0, 0, 1506, 
	1507, 5, 94, 0, 0, 1507, 359, 1, 0, 0, 0, 1508, 1509, 3, 312, 156, 0, 1509, 
	1510, 5, 10, 0, 0, 1510, 1511, 3, 392, 196, 0, 1511, 1512, 5, 2, 0, 0, 
	1512, 1513, 3, 386, 193, 0, 1513, 1514, 5, 11, 0, 0, 1514, 361, 1, 0, 0, 
	0, 1515, 1516, 5, 35, 0, 0, 1516, 1517, 5, 12, 0, 0, 1517, 1518, 3, 376, 
	188, 0, 1518, 1519, 5, 13, 0, 0, 1519, 1520, 5, 11, 0, 0, 1520, 363, 1, 
	0, 0, 0, 1521, 1522, 5, 36, 0, 0, 1522, 1523, 3, 328, 164, 0, 1523, 1524, 
	5, 11, 0, 0, 1524, 365, 1, 0, 0, 0, 1525, 1526, 5, 37, 0, 0, 1526, 1527, 
	3, 392, 196, 0, 1527, 1528, 5, 2, 0, 0, 1528, 1529, 5, 12, 0, 0, 1529, 
	1530, 3, 368, 184, 0, 1530, 1531, 5, 13, 0, 0, 1531, 1532, 5, 11, 0, 0, 
	1532, 367, 1, 0, 0, 0, 1533, 1538, 3, 370, 185, 0, 1534, 1535, 5, 2, 0, 
	0, 1535, 1537, 3, 370, 185, 0, 1536, 1534, 1, 0, 0, 0, 1537, 1540, 1, 0, 
	0, 0, 1538, 1536, 1, 0, 0, 0, 1538, 1539, 1, 0, 0, 0, 1539, 369, 1, 0, 
	0, 0, 1540, 1538, 1, 0, 0, 0, 1541, 1544, 3, 290, 145, 0, 1542, 1544, 3, 
	302, 151, 0, 1543, 1541, 1, 0, 0, 0, 1543, 1542, 1, 0, 0, 0, 1544, 371, 
	1, 0, 0, 0, 1545, 1546, 5, 38, 0, 0, 1546, 1548, 3, 400, 200, 0, 1547, 
	1549, 3, 374, 187, 0, 1548, 1547, 1, 0, 0, 0, 1548, 1549, 1, 0, 0, 0, 1549, 
	1550, 1, 0, 0, 0, 1550, 1551, 5, 3, 0, 0, 1551, 373, 1, 0, 0, 0, 1552, 
	1553, 5, 2, 0, 0, 1553, 1554, 5, 12, 0, 0, 1554, 1555, 3, 376, 188, 0, 
	1555, 1556, 5, 13, 0, 0, 1556, 375, 1, 0, 0, 0, 1557, 1562, 3, 390, 195, 
	0, 1558, 1559, 5, 2, 0, 0, 1559, 1561, 3, 390, 195, 0, 1560, 1558, 1, 0, 
	0, 0, 1561, 1564, 1, 0, 0, 0, 1562, 1560, 1, 0, 0, 0, 1562, 1563, 1, 0, 
	0, 0, 1563, 377, 1, 0, 0, 0, 1564, 1562, 1, 0, 0, 0, 1565, 1572, 3, 380, 
	190, 0, 1566, 1567, 3, 380, 190, 0, 1567, 1568, 5, 14, 0, 0, 1568, 1569, 
	3, 378, 189, 0, 1569, 1572, 1, 0, 0, 0, 1570, 1572, 3, 386, 193, 0, 1571, 
	1565, 1, 0, 0, 0, 1571, 1566, 1, 0, 0, 0, 1571, 1570, 1, 0, 0, 0, 1572, 
	379, 1, 0, 0, 0, 1573, 1580, 3, 392, 196, 0, 1574, 1580, 3, 382, 191, 0, 
	1575, 1580, 3, 302, 151, 0, 1576, 1580, 3, 398, 199, 0, 1577, 1580, 5, 
	96, 0, 0, 1578, 1580, 3, 384, 192, 0, 1579, 1573, 1, 0, 0, 0, 1579, 1574, 
	1, 0, 0, 0, 1579, 1575, 1, 0, 0, 0, 1579, 1576, 1, 0, 0, 0, 1579, 1577, 
	1, 0, 0, 0, 1579, 1578, 1, 0, 0, 0, 1580, 381, 1, 0, 0, 0, 1581, 1582, 
	3, 392, 196, 0, 1582, 1583, 5, 10, 0, 0, 1583, 1584, 3, 388, 194, 0, 1584, 
	1585, 5, 11, 0, 0, 1585, 383, 1, 0, 0, 0, 1586, 1587, 5, 39, 0, 0, 1587, 
	1588, 3, 26, 13, 0, 1588, 1589, 5, 11, 0, 0, 1589, 1607, 1, 0, 0, 0, 1590, 
	1591, 5, 40, 0, 0, 1591, 1592, 3, 98, 49, 0, 1592, 1593, 5, 11, 0, 0, 1593, 
	1607, 1, 0, 0, 0, 1594, 1595, 5, 41, 0, 0, 1595, 1596, 3, 178, 89, 0, 1596, 
	1597, 5, 11, 0, 0, 1597, 1607, 1, 0, 0, 0, 1598, 1599, 5, 42, 0, 0, 1599, 
	1600, 3, 242, 121, 0, 1600, 1601, 5, 11, 0, 0, 1601, 1607, 1, 0, 0, 0, 
	1602, 1603, 5, 43, 0, 0, 1603, 1604, 3, 226, 113, 0, 1604, 1605, 5, 11, 
	0, 0, 1605, 1607, 1, 0, 0, 0, 1606, 1586, 1, 0, 0, 0, 1606, 1590, 1, 0, 
	0, 0, 1606, 1594, 1, 0, 0, 0, 1606, 1598, 1, 0, 0, 0, 1606, 1602, 1, 0, 
	0, 0, 1607, 385, 1, 0, 0, 0, 1608, 1614, 5, 17, 0, 0, 1609, 1610, 5, 12, 
	0, 0, 1610, 1611, 3, 388, 194, 0, 1611, 1612, 5, 13, 0, 0, 1612, 1614, 
	1, 0, 0, 0, 1613, 1608, 1, 0, 0, 0, 1613, 1609, 1, 0, 0, 0, 1614, 387, 
	1, 0, 0, 0, 1615, 1620, 3, 378, 189, 0, 1616, 1617, 5, 2, 0, 0, 1617, 1619, 
	3, 378, 189, 0, 1618, 1616, 1, 0, 0, 0, 1619, 1622, 1, 0, 0, 0, 1620, 1618, 
	1, 0, 0, 0, 1620, 1621, 1, 0, 0, 0, 1621, 389, 1, 0, 0, 0, 1622, 1620, 
	1, 0, 0, 0, 1623, 1626, 3, 392, 196, 0, 1624, 1626, 5, 80, 0, 0, 1625, 
	1623, 1, 0, 0, 0, 1625, 1624, 1, 0, 0, 0, 1626, 391, 1, 0, 0, 0, 1627, 
	1628, 7, 7, 0, 0, 1628, 393, 1, 0, 0, 0, 1629, 1630, 5, 91, 0, 0, 1630, 
	395, 1, 0, 0, 0, 1631, 1632, 5, 92, 0, 0, 1632, 397, 1, 0, 0, 0, 1633, 
	1634, 7, 8, 0, 0, 1634, 399, 1, 0, 0, 0, 1635, 1636, 5, 95, 0, 0, 1636, 
	401, 1, 0, 0, 0, 125, 405, 412, 421, 430, 443, 454, 465, 476, 487, 498, 
	505, 511, 517, 522, 531, 543, 556, 569, 582, 598, 603, 618, 636, 641, 668, 
	677, 686, 696, 708, 721, 732, 744, 751, 756, 763, 768, 772, 780, 792, 805, 
	817, 831, 836, 846, 870, 877, 884, 895, 905, 912, 919, 930, 940, 950, 957, 
	964, 975, 988, 1002, 1009, 1019, 1026, 1043, 1048, 1052, 1063, 1067, 1071, 
	1079, 1091, 1104, 1114, 1128, 1135, 1144, 1150, 1166, 1170, 1180, 1188, 
	1195, 1203, 1208, 1230, 1237, 1247, 1254, 1261, 1269, 1279, 1286, 1291, 
	1301, 1305, 1311, 1331, 1335, 1357, 1369, 1376, 1381, 1398, 1405, 1410, 
	1418, 1427, 1432, 1442, 1451, 1466, 1473, 1479, 1483, 1497, 1504, 1538, 
	1543, 1548, 1562, 1571, 1579, 1606, 1613, 1620, 1625
];
