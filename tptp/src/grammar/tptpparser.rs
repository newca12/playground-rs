// Generated from TPTP.g4 by ANTLR 4.13.2
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
use super::tptplistener::*;
use super::tptpvisitor::*;

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

		pub const TPTP_T__0:i32=1; 
		pub const TPTP_T__1:i32=2; 
		pub const TPTP_T__2:i32=3; 
		pub const TPTP_T__3:i32=4; 
		pub const TPTP_T__4:i32=5; 
		pub const TPTP_T__5:i32=6; 
		pub const TPTP_T__6:i32=7; 
		pub const TPTP_T__7:i32=8; 
		pub const TPTP_T__8:i32=9; 
		pub const TPTP_T__9:i32=10; 
		pub const TPTP_T__10:i32=11; 
		pub const TPTP_T__11:i32=12; 
		pub const TPTP_T__12:i32=13; 
		pub const TPTP_T__13:i32=14; 
		pub const TPTP_T__14:i32=15; 
		pub const TPTP_T__15:i32=16; 
		pub const TPTP_T__16:i32=17; 
		pub const TPTP_T__17:i32=18; 
		pub const TPTP_T__18:i32=19; 
		pub const TPTP_T__19:i32=20; 
		pub const TPTP_T__20:i32=21; 
		pub const TPTP_T__21:i32=22; 
		pub const TPTP_T__22:i32=23; 
		pub const TPTP_T__23:i32=24; 
		pub const TPTP_T__24:i32=25; 
		pub const TPTP_T__25:i32=26; 
		pub const TPTP_T__26:i32=27; 
		pub const TPTP_T__27:i32=28; 
		pub const TPTP_T__28:i32=29; 
		pub const TPTP_T__29:i32=30; 
		pub const TPTP_T__30:i32=31; 
		pub const TPTP_T__31:i32=32; 
		pub const TPTP_T__32:i32=33; 
		pub const TPTP_T__33:i32=34; 
		pub const TPTP_T__34:i32=35; 
		pub const TPTP_T__35:i32=36; 
		pub const TPTP_T__36:i32=37; 
		pub const TPTP_T__37:i32=38; 
		pub const TPTP_T__38:i32=39; 
		pub const TPTP_T__39:i32=40; 
		pub const TPTP_T__40:i32=41; 
		pub const TPTP_T__41:i32=42; 
		pub const TPTP_T__42:i32=43; 
		pub const TPTP_T__43:i32=44; 
		pub const TPTP_T__44:i32=45; 
		pub const TPTP_T__45:i32=46; 
		pub const TPTP_T__46:i32=47; 
		pub const TPTP_T__47:i32=48; 
		pub const TPTP_T__48:i32=49; 
		pub const TPTP_T__49:i32=50; 
		pub const TPTP_T__50:i32=51; 
		pub const TPTP_T__51:i32=52; 
		pub const TPTP_T__52:i32=53; 
		pub const TPTP_T__53:i32=54; 
		pub const TPTP_T__54:i32=55; 
		pub const TPTP_T__55:i32=56; 
		pub const TPTP_T__56:i32=57; 
		pub const TPTP_T__57:i32=58; 
		pub const TPTP_T__58:i32=59; 
		pub const TPTP_T__59:i32=60; 
		pub const TPTP_T__60:i32=61; 
		pub const TPTP_WS:i32=62; 
		pub const TPTP_Comment_line:i32=63; 
		pub const TPTP_Comment_block:i32=64; 
		pub const TPTP_Single_quoted:i32=65; 
		pub const TPTP_Back_quoted:i32=66; 
		pub const TPTP_Distinct_object:i32=67; 
		pub const TPTP_Dollar_word:i32=68; 
		pub const TPTP_Dollar_dollar_word:i32=69; 
		pub const TPTP_Upper_word:i32=70; 
		pub const TPTP_Lower_word:i32=71; 
		pub const TPTP_Real:i32=72; 
		pub const TPTP_Signed_real:i32=73; 
		pub const TPTP_Unsigned_real:i32=74; 
		pub const TPTP_Decimal_exponent:i32=75; 
		pub const TPTP_Decimal_fraction:i32=76; 
		pub const TPTP_Exp_integer:i32=77; 
		pub const TPTP_Signed_exp_integer:i32=78; 
		pub const TPTP_Rational:i32=79; 
		pub const TPTP_Signed_rational:i32=80; 
		pub const TPTP_Unsigned_rational:i32=81; 
		pub const TPTP_Integer:i32=82; 
		pub const TPTP_Signed_integer:i32=83; 
		pub const TPTP_Unsigned_integer:i32=84; 
		pub const TPTP_Positive_integer:i32=85; 
		pub const TPTP_Integer_digits:i32=86; 
		pub const TPTP_Slash:i32=87; 
		pub const TPTP_Slosh:i32=88; 
		pub const TPTP_Vline:i32=89; 
		pub const TPTP_Star:i32=90; 
		pub const TPTP_Plus:i32=91; 
		pub const TPTP_Arrow:i32=92; 
		pub const TPTP_Less_sign:i32=93; 
		pub const TPTP_Hash:i32=94; 
		pub const TPTP_Not_star_slash:i32=95; 
		pub const TPTP_Percentage_sign:i32=96; 
		pub const TPTP_Double_quote:i32=97; 
		pub const TPTP_Single_quote:i32=98; 
		pub const TPTP_Back_quote:i32=99; 
		pub const TPTP_Dot:i32=100; 
		pub const TPTP_Slash_char:i32=101; 
		pub const TPTP_Slosh_char:i32=102; 
		pub const TPTP_Zero_numeric:i32=103; 
		pub const TPTP_Underscore:i32=104; 
		pub const TPTP_Alpha:i32=105; 
		pub const TPTP_Dollar:i32=106; 
		pub const TPTP_Printable_char:i32=107; 
		pub const TPTP_Viewable_char:i32=108;
	pub const TPTP_EOF:i32=EOF;
	pub const RULE_tptp_file:usize = 0; 
	pub const RULE_tptp_input:usize = 1; 
	pub const RULE_annotated_formula:usize = 2; 
	pub const RULE_tpi_annotated:usize = 3; 
	pub const RULE_tpi_formula:usize = 4; 
	pub const RULE_thf_annotated:usize = 5; 
	pub const RULE_tff_annotated:usize = 6; 
	pub const RULE_tcf_annotated:usize = 7; 
	pub const RULE_fof_annotated:usize = 8; 
	pub const RULE_cnf_annotated:usize = 9; 
	pub const RULE_annotations:usize = 10; 
	pub const RULE_formula_role:usize = 11; 
	pub const RULE_thf_formula:usize = 12; 
	pub const RULE_thf_logic_formula:usize = 13; 
	pub const RULE_thf_binary_formula:usize = 14; 
	pub const RULE_thf_binary_nonassoc:usize = 15; 
	pub const RULE_thf_binary_assoc:usize = 16; 
	pub const RULE_thf_or_formula:usize = 17; 
	pub const RULE_thf_and_formula:usize = 18; 
	pub const RULE_thf_apply_formula:usize = 19; 
	pub const RULE_thf_unit_formula:usize = 20; 
	pub const RULE_thf_preunit_formula:usize = 21; 
	pub const RULE_thf_unitary_formula:usize = 22; 
	pub const RULE_thf_quantified_formula:usize = 23; 
	pub const RULE_thf_quantification:usize = 24; 
	pub const RULE_thf_variable_list:usize = 25; 
	pub const RULE_thf_typed_variable:usize = 26; 
	pub const RULE_thf_unary_formula:usize = 27; 
	pub const RULE_thf_prefix_unary:usize = 28; 
	pub const RULE_thf_infix_unary:usize = 29; 
	pub const RULE_thf_atomic_formula:usize = 30; 
	pub const RULE_thf_plain_atomic:usize = 31; 
	pub const RULE_thf_defined_atomic:usize = 32; 
	pub const RULE_thf_defined_term:usize = 33; 
	pub const RULE_thf_defined_infix:usize = 34; 
	pub const RULE_thf_system_atomic:usize = 35; 
	pub const RULE_thf_let:usize = 36; 
	pub const RULE_thf_let_types:usize = 37; 
	pub const RULE_thf_atom_typing_list:usize = 38; 
	pub const RULE_thf_let_defns:usize = 39; 
	pub const RULE_thf_let_defn:usize = 40; 
	pub const RULE_thf_let_defn_list:usize = 41; 
	pub const RULE_thf_unitary_term:usize = 42; 
	pub const RULE_thf_conn_term:usize = 43; 
	pub const RULE_thf_tuple:usize = 44; 
	pub const RULE_thf_fof_function:usize = 45; 
	pub const RULE_thf_arguments:usize = 46; 
	pub const RULE_thf_formula_list:usize = 47; 
	pub const RULE_comma_thf_logic_formula:usize = 48; 
	pub const RULE_thf_atom_typing:usize = 49; 
	pub const RULE_thf_top_level_type:usize = 50; 
	pub const RULE_thf_unitary_type:usize = 51; 
	pub const RULE_thf_apply_type:usize = 52; 
	pub const RULE_thf_binary_type:usize = 53; 
	pub const RULE_thf_mapping_type:usize = 54; 
	pub const RULE_thf_xprod_type:usize = 55; 
	pub const RULE_thf_union_type:usize = 56; 
	pub const RULE_thf_subtype:usize = 57; 
	pub const RULE_thf_definition:usize = 58; 
	pub const RULE_thf_sequent:usize = 59; 
	pub const RULE_tff_formula:usize = 60; 
	pub const RULE_tff_logic_formula:usize = 61; 
	pub const RULE_tff_binary_formula:usize = 62; 
	pub const RULE_tff_binary_nonassoc:usize = 63; 
	pub const RULE_tff_binary_assoc:usize = 64; 
	pub const RULE_tff_or_formula:usize = 65; 
	pub const RULE_tff_and_formula:usize = 66; 
	pub const RULE_tff_unit_formula:usize = 67; 
	pub const RULE_tff_preunit_formula:usize = 68; 
	pub const RULE_tff_unitary_formula:usize = 69; 
	pub const RULE_txf_unitary_formula:usize = 70; 
	pub const RULE_tff_quantified_formula:usize = 71; 
	pub const RULE_tff_variable_list:usize = 72; 
	pub const RULE_tff_variable:usize = 73; 
	pub const RULE_tff_typed_variable:usize = 74; 
	pub const RULE_tff_unary_formula:usize = 75; 
	pub const RULE_tff_prefix_unary:usize = 76; 
	pub const RULE_tff_infix_unary:usize = 77; 
	pub const RULE_tff_atomic_formula:usize = 78; 
	pub const RULE_tff_plain_atomic:usize = 79; 
	pub const RULE_tff_defined_atomic:usize = 80; 
	pub const RULE_tff_defined_plain:usize = 81; 
	pub const RULE_tff_defined_infix:usize = 82; 
	pub const RULE_tff_system_atomic:usize = 83; 
	pub const RULE_txf_let:usize = 84; 
	pub const RULE_txf_let_types:usize = 85; 
	pub const RULE_tff_atom_typing_list:usize = 86; 
	pub const RULE_txf_let_defns:usize = 87; 
	pub const RULE_txf_let_defn:usize = 88; 
	pub const RULE_txf_let_LHS:usize = 89; 
	pub const RULE_txf_let_defn_list:usize = 90; 
	pub const RULE_nxf_atom:usize = 91; 
	pub const RULE_tff_term:usize = 92; 
	pub const RULE_tff_unitary_term:usize = 93; 
	pub const RULE_txf_tuple:usize = 94; 
	pub const RULE_tff_arguments:usize = 95; 
	pub const RULE_comma_tff_term:usize = 96; 
	pub const RULE_tff_atom_typing:usize = 97; 
	pub const RULE_tff_top_level_type:usize = 98; 
	pub const RULE_tff_non_atomic_type:usize = 99; 
	pub const RULE_tf1_quantified_type:usize = 100; 
	pub const RULE_tff_monotype:usize = 101; 
	pub const RULE_tff_unitary_type:usize = 102; 
	pub const RULE_tff_atomic_type:usize = 103; 
	pub const RULE_tff_type_arguments:usize = 104; 
	pub const RULE_tff_mapping_type:usize = 105; 
	pub const RULE_tff_xprod_type:usize = 106; 
	pub const RULE_txf_tuple_type:usize = 107; 
	pub const RULE_tff_type_list:usize = 108; 
	pub const RULE_tff_subtype:usize = 109; 
	pub const RULE_txf_definition:usize = 110; 
	pub const RULE_txf_sequent:usize = 111; 
	pub const RULE_nhf_long_connective:usize = 112; 
	pub const RULE_nhf_parameter_list:usize = 113; 
	pub const RULE_nhf_parameter:usize = 114; 
	pub const RULE_nhf_key_pair:usize = 115; 
	pub const RULE_nxf_long_connective:usize = 116; 
	pub const RULE_nxf_parameter_list:usize = 117; 
	pub const RULE_nxf_parameter:usize = 118; 
	pub const RULE_nxf_key_pair:usize = 119; 
	pub const RULE_ntf_connective_name:usize = 120; 
	pub const RULE_ntf_index:usize = 121; 
	pub const RULE_ntf_short_connective:usize = 122; 
	pub const RULE_tcf_formula:usize = 123; 
	pub const RULE_tcf_logic_formula:usize = 124; 
	pub const RULE_tcf_quantified_formula:usize = 125; 
	pub const RULE_fof_formula:usize = 126; 
	pub const RULE_fof_logic_formula:usize = 127; 
	pub const RULE_fof_binary_formula:usize = 128; 
	pub const RULE_fof_binary_nonassoc:usize = 129; 
	pub const RULE_fof_binary_assoc:usize = 130; 
	pub const RULE_fof_or_formula:usize = 131; 
	pub const RULE_fof_and_formula:usize = 132; 
	pub const RULE_fof_unary_formula:usize = 133; 
	pub const RULE_fof_infix_unary:usize = 134; 
	pub const RULE_fof_unit_formula:usize = 135; 
	pub const RULE_fof_unitary_formula:usize = 136; 
	pub const RULE_fof_quantified_formula:usize = 137; 
	pub const RULE_fof_variable_list:usize = 138; 
	pub const RULE_fof_atomic_formula:usize = 139; 
	pub const RULE_fof_plain_atomic_formula:usize = 140; 
	pub const RULE_fof_defined_atomic_formula:usize = 141; 
	pub const RULE_fof_defined_plain_formula:usize = 142; 
	pub const RULE_fof_defined_infix_formula:usize = 143; 
	pub const RULE_fof_system_atomic_formula:usize = 144; 
	pub const RULE_fof_plain_term:usize = 145; 
	pub const RULE_fof_defined_term:usize = 146; 
	pub const RULE_fof_defined_atomic_term:usize = 147; 
	pub const RULE_fof_defined_plain_term:usize = 148; 
	pub const RULE_fof_system_term:usize = 149; 
	pub const RULE_fof_arguments:usize = 150; 
	pub const RULE_fof_term:usize = 151; 
	pub const RULE_fof_function_term:usize = 152; 
	pub const RULE_fof_sequent:usize = 153; 
	pub const RULE_fof_formula_tuple:usize = 154; 
	pub const RULE_fof_formula_tuple_list:usize = 155; 
	pub const RULE_comma_fof_logic_formula:usize = 156; 
	pub const RULE_cnf_formula:usize = 157; 
	pub const RULE_cnf_disjunction:usize = 158; 
	pub const RULE_cnf_literal:usize = 159; 
	pub const RULE_thf_quantifier:usize = 160; 
	pub const RULE_thf_unary_connective:usize = 161; 
	pub const RULE_th0_quantifier:usize = 162; 
	pub const RULE_type_quantifier:usize = 163; 
	pub const RULE_subtype_sign:usize = 164; 
	pub const RULE_tff_unary_connective:usize = 165; 
	pub const RULE_tff_quantifier:usize = 166; 
	pub const RULE_fof_quantifier:usize = 167; 
	pub const RULE_nonassoc_connective:usize = 168; 
	pub const RULE_assoc_connective:usize = 169; 
	pub const RULE_unary_connective:usize = 170; 
	pub const RULE_gentzen_arrow:usize = 171; 
	pub const RULE_assignment:usize = 172; 
	pub const RULE_identical:usize = 173; 
	pub const RULE_type_constant:usize = 174; 
	pub const RULE_type_functor:usize = 175; 
	pub const RULE_defined_type:usize = 176; 
	pub const RULE_atom:usize = 177; 
	pub const RULE_untyped_atom:usize = 178; 
	pub const RULE_defined_infix_pred:usize = 179; 
	pub const RULE_infix_equality:usize = 180; 
	pub const RULE_infix_inequality:usize = 181; 
	pub const RULE_constant:usize = 182; 
	pub const RULE_functor:usize = 183; 
	pub const RULE_defined_constant:usize = 184; 
	pub const RULE_defined_functor:usize = 185; 
	pub const RULE_system_constant:usize = 186; 
	pub const RULE_system_functor:usize = 187; 
	pub const RULE_def_or_sys_constant:usize = 188; 
	pub const RULE_th1_defined_term:usize = 189; 
	pub const RULE_defined_term:usize = 190; 
	pub const RULE_variable:usize = 191; 
	pub const RULE_source:usize = 192; 
	pub const RULE_sources:usize = 193; 
	pub const RULE_dag_source:usize = 194; 
	pub const RULE_inference_record:usize = 195; 
	pub const RULE_inference_rule:usize = 196; 
	pub const RULE_internal_source:usize = 197; 
	pub const RULE_intro_type:usize = 198; 
	pub const RULE_external_source:usize = 199; 
	pub const RULE_file_source:usize = 200; 
	pub const RULE_file_info:usize = 201; 
	pub const RULE_theory:usize = 202; 
	pub const RULE_theory_name:usize = 203; 
	pub const RULE_creator_source:usize = 204; 
	pub const RULE_creator_name:usize = 205; 
	pub const RULE_parents:usize = 206; 
	pub const RULE_parent_list:usize = 207; 
	pub const RULE_comma_parent_info:usize = 208; 
	pub const RULE_parent_info:usize = 209; 
	pub const RULE_parent_details:usize = 210; 
	pub const RULE_optional_info:usize = 211; 
	pub const RULE_useful_info:usize = 212; 
	pub const RULE_include:usize = 213; 
	pub const RULE_include_optionals:usize = 214; 
	pub const RULE_formula_selection:usize = 215; 
	pub const RULE_name_list:usize = 216; 
	pub const RULE_space_name:usize = 217; 
	pub const RULE_general_term:usize = 218; 
	pub const RULE_general_data:usize = 219; 
	pub const RULE_general_function:usize = 220; 
	pub const RULE_formula_data:usize = 221; 
	pub const RULE_general_list:usize = 222; 
	pub const RULE_general_terms:usize = 223; 
	pub const RULE_comma_general_term:usize = 224; 
	pub const RULE_name:usize = 225; 
	pub const RULE_atomic_word:usize = 226; 
	pub const RULE_atomic_defined_word:usize = 227; 
	pub const RULE_atomic_system_word:usize = 228; 
	pub const RULE_number:usize = 229; 
	pub const RULE_file_name:usize = 230; 
	pub const RULE_nothing:usize = 231;
	pub const ruleNames: [&'static str; 232] =  [
		"tptp_file", "tptp_input", "annotated_formula", "tpi_annotated", "tpi_formula", 
		"thf_annotated", "tff_annotated", "tcf_annotated", "fof_annotated", "cnf_annotated", 
		"annotations", "formula_role", "thf_formula", "thf_logic_formula", "thf_binary_formula", 
		"thf_binary_nonassoc", "thf_binary_assoc", "thf_or_formula", "thf_and_formula", 
		"thf_apply_formula", "thf_unit_formula", "thf_preunit_formula", "thf_unitary_formula", 
		"thf_quantified_formula", "thf_quantification", "thf_variable_list", "thf_typed_variable", 
		"thf_unary_formula", "thf_prefix_unary", "thf_infix_unary", "thf_atomic_formula", 
		"thf_plain_atomic", "thf_defined_atomic", "thf_defined_term", "thf_defined_infix", 
		"thf_system_atomic", "thf_let", "thf_let_types", "thf_atom_typing_list", 
		"thf_let_defns", "thf_let_defn", "thf_let_defn_list", "thf_unitary_term", 
		"thf_conn_term", "thf_tuple", "thf_fof_function", "thf_arguments", "thf_formula_list", 
		"comma_thf_logic_formula", "thf_atom_typing", "thf_top_level_type", "thf_unitary_type", 
		"thf_apply_type", "thf_binary_type", "thf_mapping_type", "thf_xprod_type", 
		"thf_union_type", "thf_subtype", "thf_definition", "thf_sequent", "tff_formula", 
		"tff_logic_formula", "tff_binary_formula", "tff_binary_nonassoc", "tff_binary_assoc", 
		"tff_or_formula", "tff_and_formula", "tff_unit_formula", "tff_preunit_formula", 
		"tff_unitary_formula", "txf_unitary_formula", "tff_quantified_formula", 
		"tff_variable_list", "tff_variable", "tff_typed_variable", "tff_unary_formula", 
		"tff_prefix_unary", "tff_infix_unary", "tff_atomic_formula", "tff_plain_atomic", 
		"tff_defined_atomic", "tff_defined_plain", "tff_defined_infix", "tff_system_atomic", 
		"txf_let", "txf_let_types", "tff_atom_typing_list", "txf_let_defns", "txf_let_defn", 
		"txf_let_LHS", "txf_let_defn_list", "nxf_atom", "tff_term", "tff_unitary_term", 
		"txf_tuple", "tff_arguments", "comma_tff_term", "tff_atom_typing", "tff_top_level_type", 
		"tff_non_atomic_type", "tf1_quantified_type", "tff_monotype", "tff_unitary_type", 
		"tff_atomic_type", "tff_type_arguments", "tff_mapping_type", "tff_xprod_type", 
		"txf_tuple_type", "tff_type_list", "tff_subtype", "txf_definition", "txf_sequent", 
		"nhf_long_connective", "nhf_parameter_list", "nhf_parameter", "nhf_key_pair", 
		"nxf_long_connective", "nxf_parameter_list", "nxf_parameter", "nxf_key_pair", 
		"ntf_connective_name", "ntf_index", "ntf_short_connective", "tcf_formula", 
		"tcf_logic_formula", "tcf_quantified_formula", "fof_formula", "fof_logic_formula", 
		"fof_binary_formula", "fof_binary_nonassoc", "fof_binary_assoc", "fof_or_formula", 
		"fof_and_formula", "fof_unary_formula", "fof_infix_unary", "fof_unit_formula", 
		"fof_unitary_formula", "fof_quantified_formula", "fof_variable_list", 
		"fof_atomic_formula", "fof_plain_atomic_formula", "fof_defined_atomic_formula", 
		"fof_defined_plain_formula", "fof_defined_infix_formula", "fof_system_atomic_formula", 
		"fof_plain_term", "fof_defined_term", "fof_defined_atomic_term", "fof_defined_plain_term", 
		"fof_system_term", "fof_arguments", "fof_term", "fof_function_term", "fof_sequent", 
		"fof_formula_tuple", "fof_formula_tuple_list", "comma_fof_logic_formula", 
		"cnf_formula", "cnf_disjunction", "cnf_literal", "thf_quantifier", "thf_unary_connective", 
		"th0_quantifier", "type_quantifier", "subtype_sign", "tff_unary_connective", 
		"tff_quantifier", "fof_quantifier", "nonassoc_connective", "assoc_connective", 
		"unary_connective", "gentzen_arrow", "assignment", "identical", "type_constant", 
		"type_functor", "defined_type", "atom", "untyped_atom", "defined_infix_pred", 
		"infix_equality", "infix_inequality", "constant", "functor", "defined_constant", 
		"defined_functor", "system_constant", "system_functor", "def_or_sys_constant", 
		"th1_defined_term", "defined_term", "variable", "source", "sources", "dag_source", 
		"inference_record", "inference_rule", "internal_source", "intro_type", 
		"external_source", "file_source", "file_info", "theory", "theory_name", 
		"creator_source", "creator_name", "parents", "parent_list", "comma_parent_info", 
		"parent_info", "parent_details", "optional_info", "useful_info", "include", 
		"include_optionals", "formula_selection", "name_list", "space_name", "general_term", 
		"general_data", "general_function", "formula_data", "general_list", "general_terms", 
		"comma_general_term", "name", "atomic_word", "atomic_defined_word", "atomic_system_word", 
		"number", "file_name", "nothing"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;109] = [
		None, Some("'tpi('"), Some("','"), Some("').'"), Some("'thf('"), Some("'tff('"), 
		Some("'tcf('"), Some("'fof('"), Some("'cnf('"), Some("'-'"), Some("'&'"), 
		Some("'@'"), Some("'('"), Some("')'"), Some("'['"), Some("']'"), Some("':'"), 
		Some("'$let('"), Some("'[]'"), Some("'{'"), Some("'}'"), Some("')}'"), 
		Some("'[.]'"), Some("'.'"), Some("'{.}'"), Some("'(.)'"), Some("'!'"), 
		Some("'~'"), Some("'^'"), Some("'@+'"), Some("'@-'"), Some("'!>'"), Some("'?*'"), 
		Some("'<<'"), Some("'?'"), Some("'<=>'"), Some("'=>'"), Some("'<='"), 
		Some("'<~>'"), Some("'~&'"), Some("'-->'"), Some("':='"), Some("'=='"), 
		Some("'='"), Some("'!='"), Some("'!!'"), Some("'??'"), Some("'@@+'"), 
		Some("'@@-'"), Some("'@='"), Some("'unknown'"), Some("'inference('"), 
		Some("'introduced('"), Some("'file('"), Some("'theory('"), Some("'creator('"), 
		Some("'include('"), Some("'$thf('"), Some("'$tff('"), Some("'$fof('"), 
		Some("'$cnf('"), Some("'$fot('"), None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, Some("'+'"), 
		Some("'>'"), Some("'<'"), Some("'#'"), None, None, None, Some("'''"), 
		None, None, None, Some("'\\\\'"), None, None, None, None, None, Some("'.\\n'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;109]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, Some("WS"), Some("Comment_line"), Some("Comment_block"), Some("Single_quoted"), 
		Some("Back_quoted"), Some("Distinct_object"), Some("Dollar_word"), Some("Dollar_dollar_word"), 
		Some("Upper_word"), Some("Lower_word"), Some("Real"), Some("Signed_real"), 
		Some("Unsigned_real"), Some("Decimal_exponent"), Some("Decimal_fraction"), 
		Some("Exp_integer"), Some("Signed_exp_integer"), Some("Rational"), Some("Signed_rational"), 
		Some("Unsigned_rational"), Some("Integer"), Some("Signed_integer"), Some("Unsigned_integer"), 
		Some("Positive_integer"), Some("Integer_digits"), Some("Slash"), Some("Slosh"), 
		Some("Vline"), Some("Star"), Some("Plus"), Some("Arrow"), Some("Less_sign"), 
		Some("Hash"), Some("Not_star_slash"), Some("Percentage_sign"), Some("Double_quote"), 
		Some("Single_quote"), Some("Back_quote"), Some("Dot"), Some("Slash_char"), 
		Some("Slosh_char"), Some("Zero_numeric"), Some("Underscore"), Some("Alpha"), 
		Some("Dollar"), Some("Printable_char"), Some("Viewable_char")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,TPTPParserExt<'input>, I, TPTPParserContextType , dyn TPTPListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type TPTPTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, TPTPParserContextType , dyn TPTPListener<'input> + 'a>;

/// Parser for TPTP grammar
pub struct TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				TPTPParserExt{
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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for TPTPParser
pub trait TPTPParserContext<'input>:
	for<'x> Listenable<dyn TPTPListener<'input> + 'x > + 
	for<'x> Visitable<dyn TPTPVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=TPTPParserContextType>
{}

antlr4rust::coerce_from!{ 'input : TPTPParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn TPTPParserContext<'input> + 'input
where
    T: TPTPVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn TPTPVisitor<'input> + 'x))
    }
}

impl<'input> TPTPParserContext<'input> for TerminalNode<'input,TPTPParserContextType> {}
impl<'input> TPTPParserContext<'input> for ErrorNode<'input,TPTPParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn TPTPParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn TPTPListener<'input> + 'input }

pub struct TPTPParserContextType;
antlr4rust::tid!{TPTPParserContextType}

impl<'input> ParserNodeType<'input> for TPTPParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn TPTPParserContext<'input> + 'input;
}

impl<'input, I> Deref for TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct TPTPParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> TPTPParserExt<'input>{
}
antlr4rust::tid! { TPTPParserExt<'a> }

impl<'input> TokenAware<'input> for TPTPParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for TPTPParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for TPTPParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "TPTP.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn TPTPParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					17 => TPTPParser::<'input,I>::thf_or_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					18 => TPTPParser::<'input,I>::thf_and_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					19 => TPTPParser::<'input,I>::thf_apply_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					55 => TPTPParser::<'input,I>::thf_xprod_type_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					56 => TPTPParser::<'input,I>::thf_union_type_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					65 => TPTPParser::<'input,I>::tff_or_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					66 => TPTPParser::<'input,I>::tff_and_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					106 => TPTPParser::<'input,I>::tff_xprod_type_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					131 => TPTPParser::<'input,I>::fof_or_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					132 => TPTPParser::<'input,I>::fof_and_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					158 => TPTPParser::<'input,I>::cnf_disjunction_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> TPTPParser<'input, I>
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

impl<'input> TPTPParserContext<'input> for Tptp_fileContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tptp_fileContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tptp_file(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tptp_file(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tptp_fileContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tptp_file(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tptp_fileContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tptp_file }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tptp_file }
}
antlr4rust::tid!{Tptp_fileContextExt<'a>}

impl<'input> Tptp_fileContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tptp_fileContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tptp_fileContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tptp_fileContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tptp_fileContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_EOF, 0)
}
fn tptp_input_all(&self) ->  Vec<Rc<Tptp_inputContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tptp_input(&self, i: usize) -> Option<Rc<Tptp_inputContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tptp_fileContextAttrs<'input> for Tptp_fileContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(467);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 498) != 0) || _la==TPTP_T__55 {
				{
				{
				/*InvokeRule tptp_input*/
				recog.base.set_state(464);
				recog.tptp_input()?;

				}
				}
				recog.base.set_state(469);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(470);
			recog.base.match_token(TPTP_EOF,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tptp_inputContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tptp_inputContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tptp_input(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tptp_input(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tptp_inputContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tptp_input(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tptp_inputContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tptp_input }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tptp_input }
}
antlr4rust::tid!{Tptp_inputContextExt<'a>}

impl<'input> Tptp_inputContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tptp_inputContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tptp_inputContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tptp_inputContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tptp_inputContextExt<'input>>{

fn annotated_formula(&self) -> Option<Rc<Annotated_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn include(&self) -> Option<Rc<IncludeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tptp_inputContextAttrs<'input> for Tptp_inputContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tptp_input(&mut self,)
	-> Result<Rc<Tptp_inputContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tptp_inputContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_tptp_input);
        let mut _localctx: Rc<Tptp_inputContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(474);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__0 |TPTP_T__3 |TPTP_T__4 |TPTP_T__5 |TPTP_T__6 |TPTP_T__7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule annotated_formula*/
					recog.base.set_state(472);
					recog.annotated_formula()?;

					}
				}

			TPTP_T__55 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule include*/
					recog.base.set_state(473);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Annotated_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Annotated_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_annotated_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_annotated_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Annotated_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_annotated_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Annotated_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotated_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotated_formula }
}
antlr4rust::tid!{Annotated_formulaContextExt<'a>}

impl<'input> Annotated_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Annotated_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Annotated_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Annotated_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Annotated_formulaContextExt<'input>>{

fn thf_annotated(&self) -> Option<Rc<Thf_annotatedContextAll<'input>>> where Self:Sized{
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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn annotated_formula(&mut self,)
	-> Result<Rc<Annotated_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Annotated_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_annotated_formula);
        let mut _localctx: Rc<Annotated_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(482);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_annotated*/
					recog.base.set_state(476);
					recog.thf_annotated()?;

					}
				}

			TPTP_T__4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_annotated*/
					recog.base.set_state(477);
					recog.tff_annotated()?;

					}
				}

			TPTP_T__5 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tcf_annotated*/
					recog.base.set_state(478);
					recog.tcf_annotated()?;

					}
				}

			TPTP_T__6 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule fof_annotated*/
					recog.base.set_state(479);
					recog.fof_annotated()?;

					}
				}

			TPTP_T__7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule cnf_annotated*/
					recog.base.set_state(480);
					recog.cnf_annotated()?;

					}
				}

			TPTP_T__0 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule tpi_annotated*/
					recog.base.set_state(481);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tpi_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tpi_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tpi_annotated(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tpi_annotated(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tpi_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tpi_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tpi_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tpi_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tpi_annotated }
}
antlr4rust::tid!{Tpi_annotatedContextExt<'a>}

impl<'input> Tpi_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tpi_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tpi_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tpi_annotatedContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tpi_annotatedContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tpi_annotated(&mut self,)
	-> Result<Rc<Tpi_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tpi_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_tpi_annotated);
        let mut _localctx: Rc<Tpi_annotatedContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(484);
			recog.base.match_token(TPTP_T__0,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(485);
			recog.name()?;

			recog.base.set_state(486);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(487);
			recog.formula_role()?;

			recog.base.set_state(488);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tpi_formula*/
			recog.base.set_state(489);
			recog.tpi_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(490);
			recog.annotations()?;

			recog.base.set_state(491);
			recog.base.match_token(TPTP_T__2,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tpi_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tpi_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tpi_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tpi_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tpi_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tpi_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tpi_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tpi_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tpi_formula }
}
antlr4rust::tid!{Tpi_formulaContextExt<'a>}

impl<'input> Tpi_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tpi_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tpi_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tpi_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tpi_formulaContextExt<'input>>{

fn fof_formula(&self) -> Option<Rc<Fof_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tpi_formulaContextAttrs<'input> for Tpi_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tpi_formula(&mut self,)
	-> Result<Rc<Tpi_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tpi_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_tpi_formula);
        let mut _localctx: Rc<Tpi_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_formula*/
			recog.base.set_state(493);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_annotated(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_annotated(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_annotated }
}
antlr4rust::tid!{Thf_annotatedContextExt<'a>}

impl<'input> Thf_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_annotatedContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_annotatedContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_annotated(&mut self,)
	-> Result<Rc<Thf_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_thf_annotated);
        let mut _localctx: Rc<Thf_annotatedContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(495);
			recog.base.match_token(TPTP_T__3,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(496);
			recog.name()?;

			recog.base.set_state(497);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(498);
			recog.formula_role()?;

			recog.base.set_state(499);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_formula*/
			recog.base.set_state(500);
			recog.thf_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(501);
			recog.annotations()?;

			recog.base.set_state(502);
			recog.base.match_token(TPTP_T__2,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_annotated(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_annotated(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_annotated }
}
antlr4rust::tid!{Tff_annotatedContextExt<'a>}

impl<'input> Tff_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_annotatedContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_annotatedContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_annotated(&mut self,)
	-> Result<Rc<Tff_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_tff_annotated);
        let mut _localctx: Rc<Tff_annotatedContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(504);
			recog.base.match_token(TPTP_T__4,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(505);
			recog.name()?;

			recog.base.set_state(506);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(507);
			recog.formula_role()?;

			recog.base.set_state(508);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_formula*/
			recog.base.set_state(509);
			recog.tff_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(510);
			recog.annotations()?;

			recog.base.set_state(511);
			recog.base.match_token(TPTP_T__2,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tcf_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tcf_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tcf_annotated(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tcf_annotated(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tcf_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tcf_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tcf_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tcf_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tcf_annotated }
}
antlr4rust::tid!{Tcf_annotatedContextExt<'a>}

impl<'input> Tcf_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tcf_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tcf_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tcf_annotatedContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tcf_annotatedContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tcf_annotated(&mut self,)
	-> Result<Rc<Tcf_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tcf_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_tcf_annotated);
        let mut _localctx: Rc<Tcf_annotatedContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(513);
			recog.base.match_token(TPTP_T__5,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(514);
			recog.name()?;

			recog.base.set_state(515);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(516);
			recog.formula_role()?;

			recog.base.set_state(517);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tcf_formula*/
			recog.base.set_state(518);
			recog.tcf_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(519);
			recog.annotations()?;

			recog.base.set_state(520);
			recog.base.match_token(TPTP_T__2,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_annotated(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_annotated(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_annotated }
}
antlr4rust::tid!{Fof_annotatedContextExt<'a>}

impl<'input> Fof_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_annotatedContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_annotatedContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_annotated(&mut self,)
	-> Result<Rc<Fof_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_fof_annotated);
        let mut _localctx: Rc<Fof_annotatedContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(522);
			recog.base.match_token(TPTP_T__6,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(523);
			recog.name()?;

			recog.base.set_state(524);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(525);
			recog.formula_role()?;

			recog.base.set_state(526);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_formula*/
			recog.base.set_state(527);
			recog.fof_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(528);
			recog.annotations()?;

			recog.base.set_state(529);
			recog.base.match_token(TPTP_T__2,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Cnf_annotatedContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Cnf_annotatedContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_cnf_annotated(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_cnf_annotated(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Cnf_annotatedContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_cnf_annotated(self);
	}
}

impl<'input> CustomRuleContext<'input> for Cnf_annotatedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cnf_annotated }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cnf_annotated }
}
antlr4rust::tid!{Cnf_annotatedContextExt<'a>}

impl<'input> Cnf_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Cnf_annotatedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Cnf_annotatedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Cnf_annotatedContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Cnf_annotatedContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn cnf_annotated(&mut self,)
	-> Result<Rc<Cnf_annotatedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Cnf_annotatedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_cnf_annotated);
        let mut _localctx: Rc<Cnf_annotatedContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(531);
			recog.base.match_token(TPTP_T__7,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(532);
			recog.name()?;

			recog.base.set_state(533);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(534);
			recog.formula_role()?;

			recog.base.set_state(535);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule cnf_formula*/
			recog.base.set_state(536);
			recog.cnf_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(537);
			recog.annotations()?;

			recog.base.set_state(538);
			recog.base.match_token(TPTP_T__2,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for AnnotationsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for AnnotationsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_annotations(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_annotations(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for AnnotationsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_annotations(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnnotationsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotations }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotations }
}
antlr4rust::tid!{AnnotationsContextExt<'a>}

impl<'input> AnnotationsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AnnotationsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<AnnotationsContextExt<'input>>{

fn source(&self) -> Option<Rc<SourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn optional_info(&self) -> Option<Rc<Optional_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nothing(&self) -> Option<Rc<NothingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationsContextAttrs<'input> for AnnotationsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn annotations(&mut self,)
	-> Result<Rc<AnnotationsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_annotations);
        let mut _localctx: Rc<AnnotationsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(545);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__1 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(540);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule source*/
					recog.base.set_state(541);
					recog.source()?;

					/*InvokeRule optional_info*/
					recog.base.set_state(542);
					recog.optional_info()?;

					}
				}

			TPTP_T__2 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nothing*/
					recog.base.set_state(544);
					recog.nothing()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Formula_roleContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Formula_roleContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_formula_role(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_formula_role(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Formula_roleContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_formula_role(self);
	}
}

impl<'input> CustomRuleContext<'input> for Formula_roleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formula_role }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formula_role }
}
antlr4rust::tid!{Formula_roleContextExt<'a>}

impl<'input> Formula_roleContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Formula_roleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Formula_roleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Formula_roleContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Formula_roleContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Lower_word
/// Returns `None` if there is no child corresponding to token Lower_word
fn Lower_word(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Lower_word, 0)
}
fn general_term(&self) -> Option<Rc<General_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Formula_roleContextAttrs<'input> for Formula_roleContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn formula_role(&mut self,)
	-> Result<Rc<Formula_roleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Formula_roleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_formula_role);
        let mut _localctx: Rc<Formula_roleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(551);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(547);
					recog.base.match_token(TPTP_Lower_word,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(548);
					recog.base.match_token(TPTP_Lower_word,&mut recog.err_handler)?;

					recog.base.set_state(549);
					recog.base.match_token(TPTP_T__8,&mut recog.err_handler)?;

					/*InvokeRule general_term*/
					recog.base.set_state(550);
					recog.general_term()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_formula }
}
antlr4rust::tid!{Thf_formulaContextExt<'a>}

impl<'input> Thf_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_formulaContextExt<'input>>{

fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_atom_typing(&self) -> Option<Rc<Thf_atom_typingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_subtype(&self) -> Option<Rc<Thf_subtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_formulaContextAttrs<'input> for Thf_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_formula(&mut self,)
	-> Result<Rc<Thf_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_thf_formula);
        let mut _localctx: Rc<Thf_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(556);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(553);
					recog.thf_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(554);
					recog.thf_atom_typing()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_subtype*/
					recog.base.set_state(555);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_logic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_logic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_logic_formula }
}
antlr4rust::tid!{Thf_logic_formulaContextExt<'a>}

impl<'input> Thf_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_logic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_logic_formulaContextExt<'input>>{

fn thf_unitary_formula(&self) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_unary_formula(&self) -> Option<Rc<Thf_unary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_binary_formula(&self) -> Option<Rc<Thf_binary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_defined_infix(&self) -> Option<Rc<Thf_defined_infixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_definition(&self) -> Option<Rc<Thf_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_sequent(&self) -> Option<Rc<Thf_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_logic_formulaContextAttrs<'input> for Thf_logic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_logic_formula(&mut self,)
	-> Result<Rc<Thf_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_thf_logic_formula);
        let mut _localctx: Rc<Thf_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(564);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(558);
					recog.thf_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_unary_formula*/
					recog.base.set_state(559);
					recog.thf_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_binary_formula*/
					recog.base.set_state(560);
					recog.thf_binary_formula()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule thf_defined_infix*/
					recog.base.set_state(561);
					recog.thf_defined_infix()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule thf_definition*/
					recog.base.set_state(562);
					recog.thf_definition()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule thf_sequent*/
					recog.base.set_state(563);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_binary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_binary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_binary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_binary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_binary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_binary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_formula }
}
antlr4rust::tid!{Thf_binary_formulaContextExt<'a>}

impl<'input> Thf_binary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_binary_formulaContextExt<'input>>{

fn thf_binary_nonassoc(&self) -> Option<Rc<Thf_binary_nonassocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_binary_assoc(&self) -> Option<Rc<Thf_binary_assocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_binary_type(&self) -> Option<Rc<Thf_binary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_binary_formulaContextAttrs<'input> for Thf_binary_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_binary_formula(&mut self,)
	-> Result<Rc<Thf_binary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_thf_binary_formula);
        let mut _localctx: Rc<Thf_binary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(569);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_binary_nonassoc*/
					recog.base.set_state(566);
					recog.thf_binary_nonassoc()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_binary_assoc*/
					recog.base.set_state(567);
					recog.thf_binary_assoc()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_binary_type*/
					recog.base.set_state(568);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_binary_nonassoc ----------------
pub type Thf_binary_nonassocContextAll<'input> = Thf_binary_nonassocContext<'input>;


pub type Thf_binary_nonassocContext<'input> = BaseParserRuleContext<'input,Thf_binary_nonassocContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_binary_nonassocContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_binary_nonassocContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_binary_nonassocContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_binary_nonassoc(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_binary_nonassoc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_binary_nonassocContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_binary_nonassoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_nonassocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_nonassoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_nonassoc }
}
antlr4rust::tid!{Thf_binary_nonassocContextExt<'a>}

impl<'input> Thf_binary_nonassocContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_nonassocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_nonassocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_nonassocContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_binary_nonassocContextExt<'input>>{

fn thf_unit_formula_all(&self) ->  Vec<Rc<Thf_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unit_formula(&self, i: usize) -> Option<Rc<Thf_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn nonassoc_connective(&self) -> Option<Rc<Nonassoc_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_binary_nonassocContextAttrs<'input> for Thf_binary_nonassocContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_binary_nonassoc(&mut self,)
	-> Result<Rc<Thf_binary_nonassocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_nonassocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_thf_binary_nonassoc);
        let mut _localctx: Rc<Thf_binary_nonassocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(571);
			recog.thf_unit_formula()?;

			/*InvokeRule nonassoc_connective*/
			recog.base.set_state(572);
			recog.nonassoc_connective()?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(573);
			recog.thf_unit_formula()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_binary_assoc ----------------
pub type Thf_binary_assocContextAll<'input> = Thf_binary_assocContext<'input>;


pub type Thf_binary_assocContext<'input> = BaseParserRuleContext<'input,Thf_binary_assocContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_binary_assocContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_binary_assocContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_binary_assocContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_binary_assoc(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_binary_assoc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_binary_assocContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_binary_assoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_assocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_assoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_assoc }
}
antlr4rust::tid!{Thf_binary_assocContextExt<'a>}

impl<'input> Thf_binary_assocContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_assocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_assocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_assocContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_binary_assocContextExt<'input>>{

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

impl<'input> Thf_binary_assocContextAttrs<'input> for Thf_binary_assocContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_binary_assoc(&mut self,)
	-> Result<Rc<Thf_binary_assocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_assocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_thf_binary_assoc);
        let mut _localctx: Rc<Thf_binary_assocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(578);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(8,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_or_formula*/
					recog.base.set_state(575);
					recog.thf_or_formula_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_and_formula*/
					recog.base.set_state(576);
					recog.thf_and_formula_rec(0)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_apply_formula*/
					recog.base.set_state(577);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_or_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_or_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_or_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_or_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_or_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_or_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_or_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_or_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_or_formula }
}
antlr4rust::tid!{Thf_or_formulaContextExt<'a>}

impl<'input> Thf_or_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_or_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_or_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_or_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_or_formulaContextExt<'input>>{

fn thf_unit_formula_all(&self) ->  Vec<Rc<Thf_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unit_formula(&self, i: usize) -> Option<Rc<Thf_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Vline
/// Returns `None` if there is no child corresponding to token Vline
fn Vline(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Vline, 0)
}
fn thf_or_formula(&self) -> Option<Rc<Thf_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_or_formulaContextAttrs<'input> for Thf_or_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 34, RULE_thf_or_formula, _p);
	    let mut _localctx: Rc<Thf_or_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 34;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(581);
			recog.thf_unit_formula()?;

			recog.base.set_state(582);
			recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(583);
			recog.thf_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(590);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(9,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_or_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_or_formula)?;
					_localctx = tmp;
					recog.base.set_state(585);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(586);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					/*InvokeRule thf_unit_formula*/
					recog.base.set_state(587);
					recog.thf_unit_formula()?;

					}
					} 
				}
				recog.base.set_state(592);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(9,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

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

impl<'input> TPTPParserContext<'input> for Thf_and_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_and_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_and_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_and_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_and_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_and_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_and_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_and_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_and_formula }
}
antlr4rust::tid!{Thf_and_formulaContextExt<'a>}

impl<'input> Thf_and_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_and_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_and_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_and_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_and_formulaContextExt<'input>>{

fn thf_unit_formula_all(&self) ->  Vec<Rc<Thf_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unit_formula(&self, i: usize) -> Option<Rc<Thf_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn thf_and_formula(&self) -> Option<Rc<Thf_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_and_formulaContextAttrs<'input> for Thf_and_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 36, RULE_thf_and_formula, _p);
	    let mut _localctx: Rc<Thf_and_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 36;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(594);
			recog.thf_unit_formula()?;

			recog.base.set_state(595);
			recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(596);
			recog.thf_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(603);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_and_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_and_formula)?;
					_localctx = tmp;
					recog.base.set_state(598);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(599);
					recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_unit_formula*/
					recog.base.set_state(600);
					recog.thf_unit_formula()?;

					}
					} 
				}
				recog.base.set_state(605);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

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

impl<'input> TPTPParserContext<'input> for Thf_apply_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_apply_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_apply_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_apply_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_apply_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_apply_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_apply_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_apply_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_apply_formula }
}
antlr4rust::tid!{Thf_apply_formulaContextExt<'a>}

impl<'input> Thf_apply_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_apply_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_apply_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_apply_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_apply_formulaContextExt<'input>>{

fn thf_unit_formula_all(&self) ->  Vec<Rc<Thf_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unit_formula(&self, i: usize) -> Option<Rc<Thf_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn thf_apply_formula(&self) -> Option<Rc<Thf_apply_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_apply_formulaContextAttrs<'input> for Thf_apply_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 38, RULE_thf_apply_formula, _p);
	    let mut _localctx: Rc<Thf_apply_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 38;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(607);
			recog.thf_unit_formula()?;

			recog.base.set_state(608);
			recog.base.match_token(TPTP_T__10,&mut recog.err_handler)?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(609);
			recog.thf_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(616);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_apply_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_apply_formula)?;
					_localctx = tmp;
					recog.base.set_state(611);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(612);
					recog.base.match_token(TPTP_T__10,&mut recog.err_handler)?;

					/*InvokeRule thf_unit_formula*/
					recog.base.set_state(613);
					recog.thf_unit_formula()?;

					}
					} 
				}
				recog.base.set_state(618);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- thf_unit_formula ----------------
pub type Thf_unit_formulaContextAll<'input> = Thf_unit_formulaContext<'input>;


pub type Thf_unit_formulaContext<'input> = BaseParserRuleContext<'input,Thf_unit_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_unit_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_unit_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_unit_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_unit_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_unit_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_unit_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_unit_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unit_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unit_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unit_formula }
}
antlr4rust::tid!{Thf_unit_formulaContextExt<'a>}

impl<'input> Thf_unit_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unit_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unit_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unit_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_unit_formulaContextExt<'input>>{

fn thf_unitary_formula(&self) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_unary_formula(&self) -> Option<Rc<Thf_unary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_defined_infix(&self) -> Option<Rc<Thf_defined_infixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unit_formulaContextAttrs<'input> for Thf_unit_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_unit_formula(&mut self,)
	-> Result<Rc<Thf_unit_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unit_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_thf_unit_formula);
        let mut _localctx: Rc<Thf_unit_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(622);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(619);
					recog.thf_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_unary_formula*/
					recog.base.set_state(620);
					recog.thf_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_defined_infix*/
					recog.base.set_state(621);
					recog.thf_defined_infix()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_preunit_formula ----------------
pub type Thf_preunit_formulaContextAll<'input> = Thf_preunit_formulaContext<'input>;


pub type Thf_preunit_formulaContext<'input> = BaseParserRuleContext<'input,Thf_preunit_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_preunit_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_preunit_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_preunit_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_preunit_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_preunit_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_preunit_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_preunit_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_preunit_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_preunit_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_preunit_formula }
}
antlr4rust::tid!{Thf_preunit_formulaContextExt<'a>}

impl<'input> Thf_preunit_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_preunit_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_preunit_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_preunit_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_preunit_formulaContextExt<'input>>{

fn thf_unitary_formula(&self) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_prefix_unary(&self) -> Option<Rc<Thf_prefix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_preunit_formulaContextAttrs<'input> for Thf_preunit_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_preunit_formula(&mut self,)
	-> Result<Rc<Thf_preunit_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_preunit_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_thf_preunit_formula);
        let mut _localctx: Rc<Thf_preunit_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(626);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__11 |TPTP_T__13 |TPTP_T__16 |TPTP_T__17 |TPTP_T__18 |TPTP_T__25 |
			TPTP_T__27 |TPTP_T__28 |TPTP_T__29 |TPTP_T__30 |TPTP_T__31 |TPTP_T__33 |
			TPTP_T__44 |TPTP_T__45 |TPTP_T__46 |TPTP_T__47 |TPTP_T__48 |TPTP_Single_quoted |
			TPTP_Back_quoted |TPTP_Distinct_object |TPTP_Dollar_word |TPTP_Dollar_dollar_word |
			TPTP_Upper_word |TPTP_Lower_word |TPTP_Real |TPTP_Rational |TPTP_Integer |
			TPTP_Hash 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(624);
					recog.thf_unitary_formula()?;

					}
				}

			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_T__26 |TPTP_Less_sign 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_prefix_unary*/
					recog.base.set_state(625);
					recog.thf_prefix_unary()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_unitary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_unitary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_unitary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_unitary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_unitary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_unitary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unitary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unitary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unitary_formula }
}
antlr4rust::tid!{Thf_unitary_formulaContextExt<'a>}

impl<'input> Thf_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unitary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unitary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unitary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_unitary_formulaContextExt<'input>>{

fn thf_quantified_formula(&self) -> Option<Rc<Thf_quantified_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_atomic_formula(&self) -> Option<Rc<Thf_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unitary_formulaContextAttrs<'input> for Thf_unitary_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_unitary_formula(&mut self,)
	-> Result<Rc<Thf_unitary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unitary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_thf_unitary_formula);
        let mut _localctx: Rc<Thf_unitary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(635);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(14,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_quantified_formula*/
					recog.base.set_state(628);
					recog.thf_quantified_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_atomic_formula*/
					recog.base.set_state(629);
					recog.thf_atomic_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(630);
					recog.variable()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(631);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(632);
					recog.thf_logic_formula()?;

					recog.base.set_state(633);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_quantified_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_quantified_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_quantified_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_quantified_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_quantified_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_quantified_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_quantified_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_quantified_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_quantified_formula }
}
antlr4rust::tid!{Thf_quantified_formulaContextExt<'a>}

impl<'input> Thf_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_quantified_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_quantified_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_quantified_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_quantified_formulaContextExt<'input>>{

fn thf_quantification(&self) -> Option<Rc<Thf_quantificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_unit_formula(&self) -> Option<Rc<Thf_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_quantified_formulaContextAttrs<'input> for Thf_quantified_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_quantified_formula(&mut self,)
	-> Result<Rc<Thf_quantified_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_quantified_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_thf_quantified_formula);
        let mut _localctx: Rc<Thf_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_quantification*/
			recog.base.set_state(637);
			recog.thf_quantification()?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(638);
			recog.thf_unit_formula()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_quantificationContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_quantificationContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_quantification(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_quantification(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_quantificationContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_quantification(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_quantificationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_quantification }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_quantification }
}
antlr4rust::tid!{Thf_quantificationContextExt<'a>}

impl<'input> Thf_quantificationContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_quantificationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_quantificationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_quantificationContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_quantificationContextExt<'input>>{

fn thf_quantifier(&self) -> Option<Rc<Thf_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_variable_list(&self) -> Option<Rc<Thf_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_quantificationContextAttrs<'input> for Thf_quantificationContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_quantification(&mut self,)
	-> Result<Rc<Thf_quantificationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_quantificationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_thf_quantification);
        let mut _localctx: Rc<Thf_quantificationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_quantifier*/
			recog.base.set_state(640);
			recog.thf_quantifier()?;

			recog.base.set_state(641);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule thf_variable_list*/
			recog.base.set_state(642);
			recog.thf_variable_list()?;

			recog.base.set_state(643);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(644);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_variable_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_variable_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_variable_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_variable_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_variable_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_variable_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_variable_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_variable_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_variable_list }
}
antlr4rust::tid!{Thf_variable_listContextExt<'a>}

impl<'input> Thf_variable_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_variable_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_variable_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_variable_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_variable_listContextExt<'input>>{

fn thf_typed_variable(&self) -> Option<Rc<Thf_typed_variableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_variable_list(&self) -> Option<Rc<Thf_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_variable_listContextAttrs<'input> for Thf_variable_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_variable_list(&mut self,)
	-> Result<Rc<Thf_variable_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_variable_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_thf_variable_list);
        let mut _localctx: Rc<Thf_variable_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(651);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(15,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_typed_variable*/
					recog.base.set_state(646);
					recog.thf_typed_variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_typed_variable*/
					recog.base.set_state(647);
					recog.thf_typed_variable()?;

					recog.base.set_state(648);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule thf_variable_list*/
					recog.base.set_state(649);
					recog.thf_variable_list()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_typed_variableContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_typed_variableContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_typed_variable(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_typed_variable(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_typed_variableContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_typed_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_typed_variableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_typed_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_typed_variable }
}
antlr4rust::tid!{Thf_typed_variableContextExt<'a>}

impl<'input> Thf_typed_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_typed_variableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_typed_variableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_typed_variableContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_typed_variableContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_top_level_type(&self) -> Option<Rc<Thf_top_level_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_typed_variableContextAttrs<'input> for Thf_typed_variableContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_typed_variable(&mut self,)
	-> Result<Rc<Thf_typed_variableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_typed_variableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_thf_typed_variable);
        let mut _localctx: Rc<Thf_typed_variableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule variable*/
			recog.base.set_state(653);
			recog.variable()?;

			recog.base.set_state(654);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule thf_top_level_type*/
			recog.base.set_state(655);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_unary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_unary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_unary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_unary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_unary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_unary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unary_formula }
}
antlr4rust::tid!{Thf_unary_formulaContextExt<'a>}

impl<'input> Thf_unary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_unary_formulaContextExt<'input>>{

fn thf_prefix_unary(&self) -> Option<Rc<Thf_prefix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_infix_unary(&self) -> Option<Rc<Thf_infix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unary_formulaContextAttrs<'input> for Thf_unary_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_unary_formula(&mut self,)
	-> Result<Rc<Thf_unary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_thf_unary_formula);
        let mut _localctx: Rc<Thf_unary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(659);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_T__26 |TPTP_Less_sign 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_prefix_unary*/
					recog.base.set_state(657);
					recog.thf_prefix_unary()?;

					}
				}

			TPTP_T__11 |TPTP_T__13 |TPTP_T__16 |TPTP_T__17 |TPTP_T__18 |TPTP_T__44 |
			TPTP_T__45 |TPTP_T__46 |TPTP_T__47 |TPTP_T__48 |TPTP_Single_quoted |TPTP_Back_quoted |
			TPTP_Distinct_object |TPTP_Dollar_word |TPTP_Dollar_dollar_word |TPTP_Upper_word |
			TPTP_Lower_word |TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_infix_unary*/
					recog.base.set_state(658);
					recog.thf_infix_unary()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_prefix_unary ----------------
pub type Thf_prefix_unaryContextAll<'input> = Thf_prefix_unaryContext<'input>;


pub type Thf_prefix_unaryContext<'input> = BaseParserRuleContext<'input,Thf_prefix_unaryContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_prefix_unaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_prefix_unaryContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_prefix_unaryContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_prefix_unary(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_prefix_unary(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_prefix_unaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_prefix_unary(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_prefix_unaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_prefix_unary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_prefix_unary }
}
antlr4rust::tid!{Thf_prefix_unaryContextExt<'a>}

impl<'input> Thf_prefix_unaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_prefix_unaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_prefix_unaryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_prefix_unaryContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_prefix_unaryContextExt<'input>>{

fn thf_unary_connective(&self) -> Option<Rc<Thf_unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_preunit_formula(&self) -> Option<Rc<Thf_preunit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_prefix_unaryContextAttrs<'input> for Thf_prefix_unaryContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_prefix_unary(&mut self,)
	-> Result<Rc<Thf_prefix_unaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_prefix_unaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_thf_prefix_unary);
        let mut _localctx: Rc<Thf_prefix_unaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_unary_connective*/
			recog.base.set_state(661);
			recog.thf_unary_connective()?;

			/*InvokeRule thf_preunit_formula*/
			recog.base.set_state(662);
			recog.thf_preunit_formula()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_infix_unary ----------------
pub type Thf_infix_unaryContextAll<'input> = Thf_infix_unaryContext<'input>;


pub type Thf_infix_unaryContext<'input> = BaseParserRuleContext<'input,Thf_infix_unaryContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_infix_unaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_infix_unaryContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_infix_unaryContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_infix_unary(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_infix_unary(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_infix_unaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_infix_unary(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_infix_unaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_infix_unary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_infix_unary }
}
antlr4rust::tid!{Thf_infix_unaryContextExt<'a>}

impl<'input> Thf_infix_unaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_infix_unaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_infix_unaryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_infix_unaryContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_infix_unaryContextExt<'input>>{

fn thf_unitary_term_all(&self) ->  Vec<Rc<Thf_unitary_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_term(&self, i: usize) -> Option<Rc<Thf_unitary_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn infix_inequality(&self) -> Option<Rc<Infix_inequalityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_infix_unaryContextAttrs<'input> for Thf_infix_unaryContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_infix_unary(&mut self,)
	-> Result<Rc<Thf_infix_unaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_infix_unaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_thf_infix_unary);
        let mut _localctx: Rc<Thf_infix_unaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_unitary_term*/
			recog.base.set_state(664);
			recog.thf_unitary_term()?;

			/*InvokeRule infix_inequality*/
			recog.base.set_state(665);
			recog.infix_inequality()?;

			/*InvokeRule thf_unitary_term*/
			recog.base.set_state(666);
			recog.thf_unitary_term()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_atomic_formula ----------------
pub type Thf_atomic_formulaContextAll<'input> = Thf_atomic_formulaContext<'input>;


pub type Thf_atomic_formulaContext<'input> = BaseParserRuleContext<'input,Thf_atomic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_atomic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_atomic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_atomic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_atomic_formula }
}
antlr4rust::tid!{Thf_atomic_formulaContextExt<'a>}

impl<'input> Thf_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_atomic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_atomic_formulaContextExt<'input>>{

fn thf_plain_atomic(&self) -> Option<Rc<Thf_plain_atomicContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_defined_atomic(&self) -> Option<Rc<Thf_defined_atomicContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_system_atomic(&self) -> Option<Rc<Thf_system_atomicContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_fof_function(&self) -> Option<Rc<Thf_fof_functionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_atomic_formulaContextAttrs<'input> for Thf_atomic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_atomic_formula(&mut self,)
	-> Result<Rc<Thf_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_thf_atomic_formula);
        let mut _localctx: Rc<Thf_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(672);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_plain_atomic*/
					recog.base.set_state(668);
					recog.thf_plain_atomic()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_defined_atomic*/
					recog.base.set_state(669);
					recog.thf_defined_atomic()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_system_atomic*/
					recog.base.set_state(670);
					recog.thf_system_atomic()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule thf_fof_function*/
					recog.base.set_state(671);
					recog.thf_fof_function()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_plain_atomic ----------------
pub type Thf_plain_atomicContextAll<'input> = Thf_plain_atomicContext<'input>;


pub type Thf_plain_atomicContext<'input> = BaseParserRuleContext<'input,Thf_plain_atomicContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_plain_atomicContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_plain_atomicContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_plain_atomicContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_plain_atomic(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_plain_atomic(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_plain_atomicContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_plain_atomic(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_plain_atomicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_plain_atomic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_plain_atomic }
}
antlr4rust::tid!{Thf_plain_atomicContextExt<'a>}

impl<'input> Thf_plain_atomicContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_plain_atomicContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_plain_atomicContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_plain_atomicContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_plain_atomicContextExt<'input>>{

fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_tuple(&self) -> Option<Rc<Thf_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_plain_atomicContextAttrs<'input> for Thf_plain_atomicContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_plain_atomic(&mut self,)
	-> Result<Rc<Thf_plain_atomicContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_plain_atomicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_thf_plain_atomic);
        let mut _localctx: Rc<Thf_plain_atomicContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(676);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule constant*/
					recog.base.set_state(674);
					recog.constant()?;

					}
				}

			TPTP_T__13 |TPTP_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_tuple*/
					recog.base.set_state(675);
					recog.thf_tuple()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_defined_atomic ----------------
pub type Thf_defined_atomicContextAll<'input> = Thf_defined_atomicContext<'input>;


pub type Thf_defined_atomicContext<'input> = BaseParserRuleContext<'input,Thf_defined_atomicContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_defined_atomicContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_defined_atomicContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_defined_atomicContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_defined_atomic(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_defined_atomic(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_defined_atomicContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_defined_atomic(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_defined_atomicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_defined_atomic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_defined_atomic }
}
antlr4rust::tid!{Thf_defined_atomicContextExt<'a>}

impl<'input> Thf_defined_atomicContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_defined_atomicContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_defined_atomicContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_defined_atomicContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_defined_atomicContextExt<'input>>{

fn defined_constant(&self) -> Option<Rc<Defined_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_defined_term(&self) -> Option<Rc<Thf_defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_conn_term(&self) -> Option<Rc<Thf_conn_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nhf_long_connective(&self) -> Option<Rc<Nhf_long_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_let(&self) -> Option<Rc<Thf_letContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_defined_atomicContextAttrs<'input> for Thf_defined_atomicContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_defined_atomic(&mut self,)
	-> Result<Rc<Thf_defined_atomicContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_defined_atomicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_thf_defined_atomic);
        let mut _localctx: Rc<Thf_defined_atomicContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(686);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(678);
					recog.defined_constant()?;

					}
				}

			TPTP_T__44 |TPTP_T__45 |TPTP_T__46 |TPTP_T__47 |TPTP_T__48 |TPTP_Distinct_object |
			TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_defined_term*/
					recog.base.set_state(679);
					recog.thf_defined_term()?;

					}
				}

			TPTP_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(680);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_conn_term*/
					recog.base.set_state(681);
					recog.thf_conn_term()?;

					recog.base.set_state(682);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule nhf_long_connective*/
					recog.base.set_state(684);
					recog.nhf_long_connective()?;

					}
				}

			TPTP_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule thf_let*/
					recog.base.set_state(685);
					recog.thf_let()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_defined_term ----------------
pub type Thf_defined_termContextAll<'input> = Thf_defined_termContext<'input>;


pub type Thf_defined_termContext<'input> = BaseParserRuleContext<'input,Thf_defined_termContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_defined_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_defined_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_defined_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_defined_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_defined_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_defined_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_defined_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_defined_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_defined_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_defined_term }
}
antlr4rust::tid!{Thf_defined_termContextExt<'a>}

impl<'input> Thf_defined_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_defined_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_defined_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_defined_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_defined_termContextExt<'input>>{

fn defined_term(&self) -> Option<Rc<Defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn th1_defined_term(&self) -> Option<Rc<Th1_defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_defined_termContextAttrs<'input> for Thf_defined_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_defined_term(&mut self,)
	-> Result<Rc<Thf_defined_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_defined_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_thf_defined_term);
        let mut _localctx: Rc<Thf_defined_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(690);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Distinct_object |TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(688);
					recog.defined_term()?;

					}
				}

			TPTP_T__44 |TPTP_T__45 |TPTP_T__46 |TPTP_T__47 |TPTP_T__48 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule th1_defined_term*/
					recog.base.set_state(689);
					recog.th1_defined_term()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_defined_infix ----------------
pub type Thf_defined_infixContextAll<'input> = Thf_defined_infixContext<'input>;


pub type Thf_defined_infixContext<'input> = BaseParserRuleContext<'input,Thf_defined_infixContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_defined_infixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_defined_infixContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_defined_infixContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_defined_infix(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_defined_infix(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_defined_infixContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_defined_infix(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_defined_infixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_defined_infix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_defined_infix }
}
antlr4rust::tid!{Thf_defined_infixContextExt<'a>}

impl<'input> Thf_defined_infixContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_defined_infixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_defined_infixContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_defined_infixContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_defined_infixContextExt<'input>>{

fn thf_unitary_term_all(&self) ->  Vec<Rc<Thf_unitary_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_term(&self, i: usize) -> Option<Rc<Thf_unitary_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn defined_infix_pred(&self) -> Option<Rc<Defined_infix_predContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_defined_infixContextAttrs<'input> for Thf_defined_infixContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_defined_infix(&mut self,)
	-> Result<Rc<Thf_defined_infixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_defined_infixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_thf_defined_infix);
        let mut _localctx: Rc<Thf_defined_infixContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_unitary_term*/
			recog.base.set_state(692);
			recog.thf_unitary_term()?;

			/*InvokeRule defined_infix_pred*/
			recog.base.set_state(693);
			recog.defined_infix_pred()?;

			/*InvokeRule thf_unitary_term*/
			recog.base.set_state(694);
			recog.thf_unitary_term()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_system_atomic ----------------
pub type Thf_system_atomicContextAll<'input> = Thf_system_atomicContext<'input>;


pub type Thf_system_atomicContext<'input> = BaseParserRuleContext<'input,Thf_system_atomicContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_system_atomicContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_system_atomicContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_system_atomicContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_system_atomic(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_system_atomic(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_system_atomicContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_system_atomic(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_system_atomicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_system_atomic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_system_atomic }
}
antlr4rust::tid!{Thf_system_atomicContextExt<'a>}

impl<'input> Thf_system_atomicContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_system_atomicContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_system_atomicContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_system_atomicContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_system_atomicContextExt<'input>>{

fn system_constant(&self) -> Option<Rc<System_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_system_atomicContextAttrs<'input> for Thf_system_atomicContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_system_atomic(&mut self,)
	-> Result<Rc<Thf_system_atomicContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_system_atomicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_thf_system_atomic);
        let mut _localctx: Rc<Thf_system_atomicContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule system_constant*/
			recog.base.set_state(696);
			recog.system_constant()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_letContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_letContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_let(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_let(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_letContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_let(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_letContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_let }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_let }
}
antlr4rust::tid!{Thf_letContextExt<'a>}

impl<'input> Thf_letContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_letContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_letContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_letContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_letContextExt<'input>>{

fn thf_let_types(&self) -> Option<Rc<Thf_let_typesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_let_defns(&self) -> Option<Rc<Thf_let_defnsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_letContextAttrs<'input> for Thf_letContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_let(&mut self,)
	-> Result<Rc<Thf_letContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_letContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_thf_let);
        let mut _localctx: Rc<Thf_letContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(698);
			recog.base.match_token(TPTP_T__16,&mut recog.err_handler)?;

			/*InvokeRule thf_let_types*/
			recog.base.set_state(699);
			recog.thf_let_types()?;

			recog.base.set_state(700);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_let_defns*/
			recog.base.set_state(701);
			recog.thf_let_defns()?;

			recog.base.set_state(702);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(703);
			recog.thf_logic_formula()?;

			recog.base.set_state(704);
			recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_let_types ----------------
pub type Thf_let_typesContextAll<'input> = Thf_let_typesContext<'input>;


pub type Thf_let_typesContext<'input> = BaseParserRuleContext<'input,Thf_let_typesContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_let_typesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_let_typesContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_let_typesContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_let_types(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_let_types(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_let_typesContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_let_types(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_let_typesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_let_types }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_let_types }
}
antlr4rust::tid!{Thf_let_typesContextExt<'a>}

impl<'input> Thf_let_typesContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_let_typesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_let_typesContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_let_typesContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_let_typesContextExt<'input>>{

fn thf_atom_typing(&self) -> Option<Rc<Thf_atom_typingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_atom_typing_list(&self) -> Option<Rc<Thf_atom_typing_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_let_typesContextAttrs<'input> for Thf_let_typesContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_let_types(&mut self,)
	-> Result<Rc<Thf_let_typesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_let_typesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_thf_let_types);
        let mut _localctx: Rc<Thf_let_typesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(711);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__11 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Dollar_dollar_word |
			TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(706);
					recog.thf_atom_typing()?;

					}
				}

			TPTP_T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(707);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule thf_atom_typing_list*/
					recog.base.set_state(708);
					recog.thf_atom_typing_list()?;

					recog.base.set_state(709);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_atom_typing_list ----------------
pub type Thf_atom_typing_listContextAll<'input> = Thf_atom_typing_listContext<'input>;


pub type Thf_atom_typing_listContext<'input> = BaseParserRuleContext<'input,Thf_atom_typing_listContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_atom_typing_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_atom_typing_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_atom_typing_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_atom_typing_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_atom_typing_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_atom_typing_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_atom_typing_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_atom_typing_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_atom_typing_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_atom_typing_list }
}
antlr4rust::tid!{Thf_atom_typing_listContextExt<'a>}

impl<'input> Thf_atom_typing_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_atom_typing_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_atom_typing_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_atom_typing_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_atom_typing_listContextExt<'input>>{

fn thf_atom_typing(&self) -> Option<Rc<Thf_atom_typingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_atom_typing_list(&self) -> Option<Rc<Thf_atom_typing_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_atom_typing_listContextAttrs<'input> for Thf_atom_typing_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_atom_typing_list(&mut self,)
	-> Result<Rc<Thf_atom_typing_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_atom_typing_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_thf_atom_typing_list);
        let mut _localctx: Rc<Thf_atom_typing_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(718);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(713);
					recog.thf_atom_typing()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(714);
					recog.thf_atom_typing()?;

					recog.base.set_state(715);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule thf_atom_typing_list*/
					recog.base.set_state(716);
					recog.thf_atom_typing_list()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_let_defns ----------------
pub type Thf_let_defnsContextAll<'input> = Thf_let_defnsContext<'input>;


pub type Thf_let_defnsContext<'input> = BaseParserRuleContext<'input,Thf_let_defnsContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_let_defnsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_let_defnsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_let_defnsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_let_defns(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_let_defns(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_let_defnsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_let_defns(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_let_defnsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_let_defns }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_let_defns }
}
antlr4rust::tid!{Thf_let_defnsContextExt<'a>}

impl<'input> Thf_let_defnsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_let_defnsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_let_defnsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_let_defnsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_let_defnsContextExt<'input>>{

fn thf_let_defn(&self) -> Option<Rc<Thf_let_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_let_defn_list(&self) -> Option<Rc<Thf_let_defn_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_let_defnsContextAttrs<'input> for Thf_let_defnsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_let_defns(&mut self,)
	-> Result<Rc<Thf_let_defnsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_let_defnsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_thf_let_defns);
        let mut _localctx: Rc<Thf_let_defnsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(725);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_let_defn*/
					recog.base.set_state(720);
					recog.thf_let_defn()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(721);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule thf_let_defn_list*/
					recog.base.set_state(722);
					recog.thf_let_defn_list()?;

					recog.base.set_state(723);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_let_defn ----------------
pub type Thf_let_defnContextAll<'input> = Thf_let_defnContext<'input>;


pub type Thf_let_defnContext<'input> = BaseParserRuleContext<'input,Thf_let_defnContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_let_defnContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_let_defnContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_let_defnContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_let_defn(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_let_defn(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_let_defnContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_let_defn(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_let_defnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_let_defn }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_let_defn }
}
antlr4rust::tid!{Thf_let_defnContextExt<'a>}

impl<'input> Thf_let_defnContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_let_defnContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_let_defnContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_let_defnContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_let_defnContextExt<'input>>{

fn thf_logic_formula_all(&self) ->  Vec<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_logic_formula(&self, i: usize) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn assignment(&self) -> Option<Rc<AssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_let_defnContextAttrs<'input> for Thf_let_defnContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_let_defn(&mut self,)
	-> Result<Rc<Thf_let_defnContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_let_defnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_thf_let_defn);
        let mut _localctx: Rc<Thf_let_defnContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(727);
			recog.thf_logic_formula()?;

			/*InvokeRule assignment*/
			recog.base.set_state(728);
			recog.assignment()?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(729);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_let_defn_list ----------------
pub type Thf_let_defn_listContextAll<'input> = Thf_let_defn_listContext<'input>;


pub type Thf_let_defn_listContext<'input> = BaseParserRuleContext<'input,Thf_let_defn_listContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_let_defn_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_let_defn_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_let_defn_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_let_defn_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_let_defn_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_let_defn_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_let_defn_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_let_defn_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_let_defn_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_let_defn_list }
}
antlr4rust::tid!{Thf_let_defn_listContextExt<'a>}

impl<'input> Thf_let_defn_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_let_defn_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_let_defn_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_let_defn_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_let_defn_listContextExt<'input>>{

fn thf_let_defn(&self) -> Option<Rc<Thf_let_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_let_defn_list(&self) -> Option<Rc<Thf_let_defn_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_let_defn_listContextAttrs<'input> for Thf_let_defn_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_let_defn_list(&mut self,)
	-> Result<Rc<Thf_let_defn_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_let_defn_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_thf_let_defn_list);
        let mut _localctx: Rc<Thf_let_defn_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(736);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(24,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_let_defn*/
					recog.base.set_state(731);
					recog.thf_let_defn()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_let_defn*/
					recog.base.set_state(732);
					recog.thf_let_defn()?;

					recog.base.set_state(733);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule thf_let_defn_list*/
					recog.base.set_state(734);
					recog.thf_let_defn_list()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_unitary_term ----------------
pub type Thf_unitary_termContextAll<'input> = Thf_unitary_termContext<'input>;


pub type Thf_unitary_termContext<'input> = BaseParserRuleContext<'input,Thf_unitary_termContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_unitary_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_unitary_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_unitary_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_unitary_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_unitary_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_unitary_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_unitary_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unitary_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unitary_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unitary_term }
}
antlr4rust::tid!{Thf_unitary_termContextExt<'a>}

impl<'input> Thf_unitary_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unitary_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unitary_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unitary_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_unitary_termContextExt<'input>>{

fn thf_atomic_formula(&self) -> Option<Rc<Thf_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unitary_termContextAttrs<'input> for Thf_unitary_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_unitary_term(&mut self,)
	-> Result<Rc<Thf_unitary_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unitary_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_thf_unitary_term);
        let mut _localctx: Rc<Thf_unitary_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(744);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(25,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_atomic_formula*/
					recog.base.set_state(738);
					recog.thf_atomic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(739);
					recog.variable()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(740);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(741);
					recog.thf_logic_formula()?;

					recog.base.set_state(742);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_conn_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_conn_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_conn_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_conn_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_conn_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_conn_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_conn_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_conn_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_conn_term }
}
antlr4rust::tid!{Thf_conn_termContextExt<'a>}

impl<'input> Thf_conn_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_conn_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_conn_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_conn_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_conn_termContextExt<'input>>{

fn nonassoc_connective(&self) -> Option<Rc<Nonassoc_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assoc_connective(&self) -> Option<Rc<Assoc_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn infix_equality(&self) -> Option<Rc<Infix_equalityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn infix_inequality(&self) -> Option<Rc<Infix_inequalityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_unary_connective(&self) -> Option<Rc<Thf_unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_conn_termContextAttrs<'input> for Thf_conn_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_conn_term(&mut self,)
	-> Result<Rc<Thf_conn_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_conn_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_thf_conn_term);
        let mut _localctx: Rc<Thf_conn_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(751);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nonassoc_connective*/
					recog.base.set_state(746);
					recog.nonassoc_connective()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule assoc_connective*/
					recog.base.set_state(747);
					recog.assoc_connective()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule infix_equality*/
					recog.base.set_state(748);
					recog.infix_equality()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule infix_inequality*/
					recog.base.set_state(749);
					recog.infix_inequality()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule thf_unary_connective*/
					recog.base.set_state(750);
					recog.thf_unary_connective()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_tupleContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_tupleContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_tuple(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_tuple(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_tupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_tuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_tupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_tuple }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_tuple }
}
antlr4rust::tid!{Thf_tupleContextExt<'a>}

impl<'input> Thf_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_tupleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_tupleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_tupleContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_tupleContextExt<'input>>{

fn thf_formula_list(&self) -> Option<Rc<Thf_formula_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_tupleContextAttrs<'input> for Thf_tupleContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_tuple(&mut self,)
	-> Result<Rc<Thf_tupleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_tupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_thf_tuple);
        let mut _localctx: Rc<Thf_tupleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(758);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(753);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(754);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule thf_formula_list*/
					recog.base.set_state(755);
					recog.thf_formula_list()?;

					recog.base.set_state(756);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_fof_function ----------------
pub type Thf_fof_functionContextAll<'input> = Thf_fof_functionContext<'input>;


pub type Thf_fof_functionContext<'input> = BaseParserRuleContext<'input,Thf_fof_functionContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_fof_functionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_fof_functionContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_fof_functionContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_fof_function(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_fof_function(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_fof_functionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_fof_function(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_fof_functionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_fof_function }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_fof_function }
}
antlr4rust::tid!{Thf_fof_functionContextExt<'a>}

impl<'input> Thf_fof_functionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_fof_functionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_fof_functionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_fof_functionContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_fof_functionContextExt<'input>>{

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

impl<'input> Thf_fof_functionContextAttrs<'input> for Thf_fof_functionContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_fof_function(&mut self,)
	-> Result<Rc<Thf_fof_functionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_fof_functionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_thf_fof_function);
        let mut _localctx: Rc<Thf_fof_functionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(775);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule functor*/
					recog.base.set_state(760);
					recog.functor()?;

					recog.base.set_state(761);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(762);
					recog.thf_arguments()?;

					recog.base.set_state(763);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_Dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_functor*/
					recog.base.set_state(765);
					recog.defined_functor()?;

					recog.base.set_state(766);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(767);
					recog.thf_arguments()?;

					recog.base.set_state(768);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_Dollar_dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule system_functor*/
					recog.base.set_state(770);
					recog.system_functor()?;

					recog.base.set_state(771);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(772);
					recog.thf_arguments()?;

					recog.base.set_state(773);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_argumentsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_argumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_arguments(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_arguments(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_argumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_arguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_argumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_arguments }
}
antlr4rust::tid!{Thf_argumentsContextExt<'a>}

impl<'input> Thf_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_argumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_argumentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_argumentsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_argumentsContextExt<'input>>{

fn thf_formula_list(&self) -> Option<Rc<Thf_formula_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_argumentsContextAttrs<'input> for Thf_argumentsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_arguments(&mut self,)
	-> Result<Rc<Thf_argumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_argumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_thf_arguments);
        let mut _localctx: Rc<Thf_argumentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_formula_list*/
			recog.base.set_state(777);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_formula_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_formula_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_formula_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_formula_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_formula_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_formula_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_formula_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_formula_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_formula_list }
}
antlr4rust::tid!{Thf_formula_listContextExt<'a>}

impl<'input> Thf_formula_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_formula_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_formula_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_formula_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_formula_listContextExt<'input>>{

fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comma_thf_logic_formula_all(&self) ->  Vec<Rc<Comma_thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comma_thf_logic_formula(&self, i: usize) -> Option<Rc<Comma_thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Thf_formula_listContextAttrs<'input> for Thf_formula_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_formula_list(&mut self,)
	-> Result<Rc<Thf_formula_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_formula_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_thf_formula_list);
        let mut _localctx: Rc<Thf_formula_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(779);
			recog.thf_logic_formula()?;

			recog.base.set_state(783);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_thf_logic_formula*/
				recog.base.set_state(780);
				recog.comma_thf_logic_formula()?;

				}
				}
				recog.base.set_state(785);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- comma_thf_logic_formula ----------------
pub type Comma_thf_logic_formulaContextAll<'input> = Comma_thf_logic_formulaContext<'input>;


pub type Comma_thf_logic_formulaContext<'input> = BaseParserRuleContext<'input,Comma_thf_logic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Comma_thf_logic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Comma_thf_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Comma_thf_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_comma_thf_logic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_comma_thf_logic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Comma_thf_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_comma_thf_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Comma_thf_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comma_thf_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comma_thf_logic_formula }
}
antlr4rust::tid!{Comma_thf_logic_formulaContextExt<'a>}

impl<'input> Comma_thf_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Comma_thf_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Comma_thf_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Comma_thf_logic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Comma_thf_logic_formulaContextExt<'input>>{

fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Comma_thf_logic_formulaContextAttrs<'input> for Comma_thf_logic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comma_thf_logic_formula(&mut self,)
	-> Result<Rc<Comma_thf_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Comma_thf_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_comma_thf_logic_formula);
        let mut _localctx: Rc<Comma_thf_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(786);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(787);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_atom_typing ----------------
pub type Thf_atom_typingContextAll<'input> = Thf_atom_typingContext<'input>;


pub type Thf_atom_typingContext<'input> = BaseParserRuleContext<'input,Thf_atom_typingContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_atom_typingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_atom_typingContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_atom_typingContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_atom_typing(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_atom_typing(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_atom_typingContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_atom_typing(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_atom_typingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_atom_typing }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_atom_typing }
}
antlr4rust::tid!{Thf_atom_typingContextExt<'a>}

impl<'input> Thf_atom_typingContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_atom_typingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_atom_typingContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_atom_typingContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_atom_typingContextExt<'input>>{

fn untyped_atom(&self) -> Option<Rc<Untyped_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_top_level_type(&self) -> Option<Rc<Thf_top_level_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_atom_typing(&self) -> Option<Rc<Thf_atom_typingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_atom_typingContextAttrs<'input> for Thf_atom_typingContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_atom_typing(&mut self,)
	-> Result<Rc<Thf_atom_typingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_atom_typingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_thf_atom_typing);
        let mut _localctx: Rc<Thf_atom_typingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(797);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Dollar_dollar_word |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule untyped_atom*/
					recog.base.set_state(789);
					recog.untyped_atom()?;

					recog.base.set_state(790);
					recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

					/*InvokeRule thf_top_level_type*/
					recog.base.set_state(791);
					recog.thf_top_level_type()?;

					}
				}

			TPTP_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(793);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(794);
					recog.thf_atom_typing()?;

					recog.base.set_state(795);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_top_level_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_top_level_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_top_level_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_top_level_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_top_level_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_top_level_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_top_level_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_top_level_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_top_level_type }
}
antlr4rust::tid!{Thf_top_level_typeContextExt<'a>}

impl<'input> Thf_top_level_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_top_level_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_top_level_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_top_level_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_top_level_typeContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_top_level_type(&mut self,)
	-> Result<Rc<Thf_top_level_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_top_level_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_thf_top_level_type);
        let mut _localctx: Rc<Thf_top_level_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(802);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(799);
					recog.thf_unitary_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_mapping_type*/
					recog.base.set_state(800);
					recog.thf_mapping_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_apply_type*/
					recog.base.set_state(801);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_unitary_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_unitary_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_unitary_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_unitary_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_unitary_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_unitary_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unitary_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unitary_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unitary_type }
}
antlr4rust::tid!{Thf_unitary_typeContextExt<'a>}

impl<'input> Thf_unitary_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unitary_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unitary_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unitary_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_unitary_typeContextExt<'input>>{

fn thf_unitary_formula(&self) -> Option<Rc<Thf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unitary_typeContextAttrs<'input> for Thf_unitary_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_unitary_type(&mut self,)
	-> Result<Rc<Thf_unitary_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unitary_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_thf_unitary_type);
        let mut _localctx: Rc<Thf_unitary_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(804);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_apply_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_apply_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_apply_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_apply_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_apply_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_apply_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_apply_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_apply_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_apply_type }
}
antlr4rust::tid!{Thf_apply_typeContextExt<'a>}

impl<'input> Thf_apply_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_apply_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_apply_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_apply_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_apply_typeContextExt<'input>>{

fn thf_apply_formula(&self) -> Option<Rc<Thf_apply_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_apply_typeContextAttrs<'input> for Thf_apply_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_apply_type(&mut self,)
	-> Result<Rc<Thf_apply_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_apply_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_thf_apply_type);
        let mut _localctx: Rc<Thf_apply_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_apply_formula*/
			recog.base.set_state(806);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_binary_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_binary_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_binary_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_binary_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_binary_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_binary_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_type }
}
antlr4rust::tid!{Thf_binary_typeContextExt<'a>}

impl<'input> Thf_binary_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_binary_typeContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_binary_type(&mut self,)
	-> Result<Rc<Thf_binary_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_thf_binary_type);
        let mut _localctx: Rc<Thf_binary_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(811);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_mapping_type*/
					recog.base.set_state(808);
					recog.thf_mapping_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_xprod_type*/
					recog.base.set_state(809);
					recog.thf_xprod_type_rec(0)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_union_type*/
					recog.base.set_state(810);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_mapping_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_mapping_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_mapping_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_mapping_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_mapping_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_mapping_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_mapping_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_mapping_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_mapping_type }
}
antlr4rust::tid!{Thf_mapping_typeContextExt<'a>}

impl<'input> Thf_mapping_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_mapping_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_mapping_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_mapping_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_mapping_typeContextExt<'input>>{

fn thf_unitary_type_all(&self) ->  Vec<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_type(&self, i: usize) -> Option<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Arrow
/// Returns `None` if there is no child corresponding to token Arrow
fn Arrow(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Arrow, 0)
}
fn thf_mapping_type(&self) -> Option<Rc<Thf_mapping_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_mapping_typeContextAttrs<'input> for Thf_mapping_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_mapping_type(&mut self,)
	-> Result<Rc<Thf_mapping_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_mapping_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_thf_mapping_type);
        let mut _localctx: Rc<Thf_mapping_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(821);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(813);
					recog.thf_unitary_type()?;

					recog.base.set_state(814);
					recog.base.match_token(TPTP_Arrow,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(815);
					recog.thf_unitary_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(817);
					recog.thf_unitary_type()?;

					recog.base.set_state(818);
					recog.base.match_token(TPTP_Arrow,&mut recog.err_handler)?;

					/*InvokeRule thf_mapping_type*/
					recog.base.set_state(819);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_xprod_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_xprod_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_xprod_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_xprod_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_xprod_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_xprod_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_xprod_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_xprod_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_xprod_type }
}
antlr4rust::tid!{Thf_xprod_typeContextExt<'a>}

impl<'input> Thf_xprod_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_xprod_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_xprod_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_xprod_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_xprod_typeContextExt<'input>>{

fn thf_unitary_type_all(&self) ->  Vec<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_type(&self, i: usize) -> Option<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Star, 0)
}
fn thf_xprod_type(&self) -> Option<Rc<Thf_xprod_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_xprod_typeContextAttrs<'input> for Thf_xprod_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 110, RULE_thf_xprod_type, _p);
	    let mut _localctx: Rc<Thf_xprod_typeContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 110;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(824);
			recog.thf_unitary_type()?;

			recog.base.set_state(825);
			recog.base.match_token(TPTP_Star,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(826);
			recog.thf_unitary_type()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(833);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_xprod_typeContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_xprod_type)?;
					_localctx = tmp;
					recog.base.set_state(828);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(829);
					recog.base.match_token(TPTP_Star,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(830);
					recog.thf_unitary_type()?;

					}
					} 
				}
				recog.base.set_state(835);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

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

impl<'input> TPTPParserContext<'input> for Thf_union_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_union_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_union_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_union_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_union_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_union_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_union_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_union_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_union_type }
}
antlr4rust::tid!{Thf_union_typeContextExt<'a>}

impl<'input> Thf_union_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_union_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_union_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_union_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_union_typeContextExt<'input>>{

fn thf_unitary_type_all(&self) ->  Vec<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_unitary_type(&self, i: usize) -> Option<Rc<Thf_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Plus
/// Returns `None` if there is no child corresponding to token Plus
fn Plus(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Plus, 0)
}
fn thf_union_type(&self) -> Option<Rc<Thf_union_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_union_typeContextAttrs<'input> for Thf_union_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 112, RULE_thf_union_type, _p);
	    let mut _localctx: Rc<Thf_union_typeContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 112;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(837);
			recog.thf_unitary_type()?;

			recog.base.set_state(838);
			recog.base.match_token(TPTP_Plus,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(839);
			recog.thf_unitary_type()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(846);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(35,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Thf_union_typeContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_thf_union_type)?;
					_localctx = tmp;
					recog.base.set_state(841);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(842);
					recog.base.match_token(TPTP_Plus,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(843);
					recog.thf_unitary_type()?;

					}
					} 
				}
				recog.base.set_state(848);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(35,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

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

impl<'input> TPTPParserContext<'input> for Thf_subtypeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_subtypeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_subtype(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_subtype(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_subtypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_subtype(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_subtypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_subtype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_subtype }
}
antlr4rust::tid!{Thf_subtypeContextExt<'a>}

impl<'input> Thf_subtypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_subtypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_subtypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_subtypeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_subtypeContextExt<'input>>{

fn untyped_atom(&self) -> Option<Rc<Untyped_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subtype_sign(&self) -> Option<Rc<Subtype_signContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_subtypeContextAttrs<'input> for Thf_subtypeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_subtype(&mut self,)
	-> Result<Rc<Thf_subtypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_subtypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_thf_subtype);
        let mut _localctx: Rc<Thf_subtypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule untyped_atom*/
			recog.base.set_state(849);
			recog.untyped_atom()?;

			/*InvokeRule subtype_sign*/
			recog.base.set_state(850);
			recog.subtype_sign()?;

			/*InvokeRule atom*/
			recog.base.set_state(851);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- thf_definition ----------------
pub type Thf_definitionContextAll<'input> = Thf_definitionContext<'input>;


pub type Thf_definitionContext<'input> = BaseParserRuleContext<'input,Thf_definitionContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_definitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_definition(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_definition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_definitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_definition(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_definition }
}
antlr4rust::tid!{Thf_definitionContextExt<'a>}

impl<'input> Thf_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_definitionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_definitionContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_definitionContextExt<'input>>{

fn thf_atomic_formula(&self) -> Option<Rc<Thf_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identical(&self) -> Option<Rc<IdenticalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_logic_formula(&self) -> Option<Rc<Thf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_definitionContextAttrs<'input> for Thf_definitionContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_definition(&mut self,)
	-> Result<Rc<Thf_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_thf_definition);
        let mut _localctx: Rc<Thf_definitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_atomic_formula*/
			recog.base.set_state(853);
			recog.thf_atomic_formula()?;

			/*InvokeRule identical*/
			recog.base.set_state(854);
			recog.identical()?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(855);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_sequentContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_sequentContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_sequent(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_sequent(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_sequentContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_sequent(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_sequentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_sequent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_sequent }
}
antlr4rust::tid!{Thf_sequentContextExt<'a>}

impl<'input> Thf_sequentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_sequentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_sequentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_sequentContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_sequentContextExt<'input>>{

fn thf_tuple_all(&self) ->  Vec<Rc<Thf_tupleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn thf_tuple(&self, i: usize) -> Option<Rc<Thf_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn gentzen_arrow(&self) -> Option<Rc<Gentzen_arrowContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_sequentContextAttrs<'input> for Thf_sequentContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_sequent(&mut self,)
	-> Result<Rc<Thf_sequentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_sequentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_thf_sequent);
        let mut _localctx: Rc<Thf_sequentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_tuple*/
			recog.base.set_state(857);
			recog.thf_tuple()?;

			/*InvokeRule gentzen_arrow*/
			recog.base.set_state(858);
			recog.gentzen_arrow()?;

			/*InvokeRule thf_tuple*/
			recog.base.set_state(859);
			recog.thf_tuple()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_formula }
}
antlr4rust::tid!{Tff_formulaContextExt<'a>}

impl<'input> Tff_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_formulaContextExt<'input>>{

fn tff_logic_formula(&self) -> Option<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atom_typing(&self) -> Option<Rc<Tff_atom_typingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_subtype(&self) -> Option<Rc<Tff_subtypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_formulaContextAttrs<'input> for Tff_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_formula(&mut self,)
	-> Result<Rc<Tff_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_tff_formula);
        let mut _localctx: Rc<Tff_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(864);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(36,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(861);
					recog.tff_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(862);
					recog.tff_atom_typing()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tff_subtype*/
					recog.base.set_state(863);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_logic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_logic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_logic_formula }
}
antlr4rust::tid!{Tff_logic_formulaContextExt<'a>}

impl<'input> Tff_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_logic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_logic_formulaContextExt<'input>>{

fn tff_unitary_formula(&self) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_unary_formula(&self) -> Option<Rc<Tff_unary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_binary_formula(&self) -> Option<Rc<Tff_binary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_defined_infix(&self) -> Option<Rc<Tff_defined_infixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_definition(&self) -> Option<Rc<Txf_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_sequent(&self) -> Option<Rc<Txf_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_logic_formulaContextAttrs<'input> for Tff_logic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_logic_formula(&mut self,)
	-> Result<Rc<Tff_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_tff_logic_formula);
        let mut _localctx: Rc<Tff_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(872);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(866);
					recog.tff_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_unary_formula*/
					recog.base.set_state(867);
					recog.tff_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tff_binary_formula*/
					recog.base.set_state(868);
					recog.tff_binary_formula()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule tff_defined_infix*/
					recog.base.set_state(869);
					recog.tff_defined_infix()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule txf_definition*/
					recog.base.set_state(870);
					recog.txf_definition()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule txf_sequent*/
					recog.base.set_state(871);
					recog.txf_sequent()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_binary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_binary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_binary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_binary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_binary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_binary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_binary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_binary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_binary_formula }
}
antlr4rust::tid!{Tff_binary_formulaContextExt<'a>}

impl<'input> Tff_binary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_binary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_binary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_binary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_binary_formulaContextExt<'input>>{

fn tff_binary_nonassoc(&self) -> Option<Rc<Tff_binary_nonassocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_binary_assoc(&self) -> Option<Rc<Tff_binary_assocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_binary_formulaContextAttrs<'input> for Tff_binary_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_binary_formula(&mut self,)
	-> Result<Rc<Tff_binary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_binary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_tff_binary_formula);
        let mut _localctx: Rc<Tff_binary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(876);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(38,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_binary_nonassoc*/
					recog.base.set_state(874);
					recog.tff_binary_nonassoc()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_binary_assoc*/
					recog.base.set_state(875);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_binary_nonassocContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_binary_nonassocContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_binary_nonassoc(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_binary_nonassoc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_binary_nonassocContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_binary_nonassoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_binary_nonassocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_binary_nonassoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_binary_nonassoc }
}
antlr4rust::tid!{Tff_binary_nonassocContextExt<'a>}

impl<'input> Tff_binary_nonassocContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_binary_nonassocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_binary_nonassocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_binary_nonassocContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_binary_nonassocContextExt<'input>>{

fn tff_unit_formula_all(&self) ->  Vec<Rc<Tff_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_unit_formula(&self, i: usize) -> Option<Rc<Tff_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn nonassoc_connective(&self) -> Option<Rc<Nonassoc_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_binary_nonassocContextAttrs<'input> for Tff_binary_nonassocContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_binary_nonassoc(&mut self,)
	-> Result<Rc<Tff_binary_nonassocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_binary_nonassocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_tff_binary_nonassoc);
        let mut _localctx: Rc<Tff_binary_nonassocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(878);
			recog.tff_unit_formula()?;

			/*InvokeRule nonassoc_connective*/
			recog.base.set_state(879);
			recog.nonassoc_connective()?;

			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(880);
			recog.tff_unit_formula()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_binary_assocContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_binary_assocContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_binary_assoc(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_binary_assoc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_binary_assocContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_binary_assoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_binary_assocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_binary_assoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_binary_assoc }
}
antlr4rust::tid!{Tff_binary_assocContextExt<'a>}

impl<'input> Tff_binary_assocContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_binary_assocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_binary_assocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_binary_assocContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_binary_assocContextExt<'input>>{

fn tff_or_formula(&self) -> Option<Rc<Tff_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_and_formula(&self) -> Option<Rc<Tff_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_binary_assocContextAttrs<'input> for Tff_binary_assocContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_binary_assoc(&mut self,)
	-> Result<Rc<Tff_binary_assocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_binary_assocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_tff_binary_assoc);
        let mut _localctx: Rc<Tff_binary_assocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(884);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(39,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_or_formula*/
					recog.base.set_state(882);
					recog.tff_or_formula_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_and_formula*/
					recog.base.set_state(883);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_or_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_or_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_or_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_or_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_or_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_or_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_or_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_or_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_or_formula }
}
antlr4rust::tid!{Tff_or_formulaContextExt<'a>}

impl<'input> Tff_or_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_or_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_or_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_or_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_or_formulaContextExt<'input>>{

fn tff_unit_formula_all(&self) ->  Vec<Rc<Tff_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_unit_formula(&self, i: usize) -> Option<Rc<Tff_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Vline
/// Returns `None` if there is no child corresponding to token Vline
fn Vline(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Vline, 0)
}
fn tff_or_formula(&self) -> Option<Rc<Tff_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_or_formulaContextAttrs<'input> for Tff_or_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 130, RULE_tff_or_formula, _p);
	    let mut _localctx: Rc<Tff_or_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 130;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(887);
			recog.tff_unit_formula()?;

			recog.base.set_state(888);
			recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(889);
			recog.tff_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(896);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(40,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Tff_or_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_tff_or_formula)?;
					_localctx = tmp;
					recog.base.set_state(891);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(892);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					/*InvokeRule tff_unit_formula*/
					recog.base.set_state(893);
					recog.tff_unit_formula()?;

					}
					} 
				}
				recog.base.set_state(898);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(40,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

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

impl<'input> TPTPParserContext<'input> for Tff_and_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_and_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_and_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_and_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_and_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_and_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_and_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_and_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_and_formula }
}
antlr4rust::tid!{Tff_and_formulaContextExt<'a>}

impl<'input> Tff_and_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_and_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_and_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_and_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_and_formulaContextExt<'input>>{

fn tff_unit_formula_all(&self) ->  Vec<Rc<Tff_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_unit_formula(&self, i: usize) -> Option<Rc<Tff_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn tff_and_formula(&self) -> Option<Rc<Tff_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_and_formulaContextAttrs<'input> for Tff_and_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 132, RULE_tff_and_formula, _p);
	    let mut _localctx: Rc<Tff_and_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 132;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(900);
			recog.tff_unit_formula()?;

			recog.base.set_state(901);
			recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(902);
			recog.tff_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(909);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(41,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Tff_and_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_tff_and_formula)?;
					_localctx = tmp;
					recog.base.set_state(904);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(905);
					recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_unit_formula*/
					recog.base.set_state(906);
					recog.tff_unit_formula()?;

					}
					} 
				}
				recog.base.set_state(911);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(41,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- tff_unit_formula ----------------
pub type Tff_unit_formulaContextAll<'input> = Tff_unit_formulaContext<'input>;


pub type Tff_unit_formulaContext<'input> = BaseParserRuleContext<'input,Tff_unit_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_unit_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_unit_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_unit_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_unit_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_unit_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_unit_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_unit_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_unit_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_unit_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_unit_formula }
}
antlr4rust::tid!{Tff_unit_formulaContextExt<'a>}

impl<'input> Tff_unit_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_unit_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_unit_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_unit_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_unit_formulaContextExt<'input>>{

fn tff_unitary_formula(&self) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_unary_formula(&self) -> Option<Rc<Tff_unary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_defined_infix(&self) -> Option<Rc<Tff_defined_infixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_unit_formulaContextAttrs<'input> for Tff_unit_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_unit_formula(&mut self,)
	-> Result<Rc<Tff_unit_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_unit_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_tff_unit_formula);
        let mut _localctx: Rc<Tff_unit_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(915);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(42,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(912);
					recog.tff_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_unary_formula*/
					recog.base.set_state(913);
					recog.tff_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tff_defined_infix*/
					recog.base.set_state(914);
					recog.tff_defined_infix()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_preunit_formula ----------------
pub type Tff_preunit_formulaContextAll<'input> = Tff_preunit_formulaContext<'input>;


pub type Tff_preunit_formulaContext<'input> = BaseParserRuleContext<'input,Tff_preunit_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_preunit_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_preunit_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_preunit_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_preunit_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_preunit_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_preunit_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_preunit_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_preunit_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_preunit_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_preunit_formula }
}
antlr4rust::tid!{Tff_preunit_formulaContextExt<'a>}

impl<'input> Tff_preunit_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_preunit_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_preunit_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_preunit_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_preunit_formulaContextExt<'input>>{

fn tff_unitary_formula(&self) -> Option<Rc<Tff_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_prefix_unary(&self) -> Option<Rc<Tff_prefix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_preunit_formulaContextAttrs<'input> for Tff_preunit_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_preunit_formula(&mut self,)
	-> Result<Rc<Tff_preunit_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_preunit_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_tff_preunit_formula);
        let mut _localctx: Rc<Tff_preunit_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(919);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__11 |TPTP_T__16 |TPTP_T__18 |TPTP_T__25 |TPTP_T__33 |TPTP_Single_quoted |
			TPTP_Back_quoted |TPTP_Dollar_word |TPTP_Dollar_dollar_word |TPTP_Upper_word |
			TPTP_Lower_word |TPTP_Hash 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(917);
					recog.tff_unitary_formula()?;

					}
				}

			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_T__26 |TPTP_Less_sign 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_prefix_unary*/
					recog.base.set_state(918);
					recog.tff_prefix_unary()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_unitary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_unitary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_unitary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_unitary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_unitary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_unitary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_unitary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_unitary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_unitary_formula }
}
antlr4rust::tid!{Tff_unitary_formulaContextExt<'a>}

impl<'input> Tff_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_unitary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_unitary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_unitary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_unitary_formulaContextExt<'input>>{

fn tff_quantified_formula(&self) -> Option<Rc<Tff_quantified_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atomic_formula(&self) -> Option<Rc<Tff_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_unitary_formula(&self) -> Option<Rc<Txf_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_logic_formula(&self) -> Option<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_unitary_formulaContextAttrs<'input> for Tff_unitary_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_unitary_formula(&mut self,)
	-> Result<Rc<Tff_unitary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_unitary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_tff_unitary_formula);
        let mut _localctx: Rc<Tff_unitary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(928);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__25 |TPTP_T__33 |TPTP_Hash 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_quantified_formula*/
					recog.base.set_state(921);
					recog.tff_quantified_formula()?;

					}
				}

			TPTP_T__16 |TPTP_T__18 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Dollar_word |
			TPTP_Dollar_dollar_word |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_atomic_formula*/
					recog.base.set_state(922);
					recog.tff_atomic_formula()?;

					}
				}

			TPTP_Upper_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule txf_unitary_formula*/
					recog.base.set_state(923);
					recog.txf_unitary_formula()?;

					}
				}

			TPTP_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(924);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(925);
					recog.tff_logic_formula()?;

					recog.base.set_state(926);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_unitary_formula ----------------
pub type Txf_unitary_formulaContextAll<'input> = Txf_unitary_formulaContext<'input>;


pub type Txf_unitary_formulaContext<'input> = BaseParserRuleContext<'input,Txf_unitary_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_unitary_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_unitary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_unitary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_unitary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_unitary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_unitary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_unitary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_unitary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_unitary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_unitary_formula }
}
antlr4rust::tid!{Txf_unitary_formulaContextExt<'a>}

impl<'input> Txf_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_unitary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_unitary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_unitary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_unitary_formulaContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_unitary_formulaContextAttrs<'input> for Txf_unitary_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_unitary_formula(&mut self,)
	-> Result<Rc<Txf_unitary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_unitary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_txf_unitary_formula);
        let mut _localctx: Rc<Txf_unitary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule variable*/
			recog.base.set_state(930);
			recog.variable()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_quantified_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_quantified_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_quantified_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_quantified_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_quantified_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_quantified_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_quantified_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_quantified_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_quantified_formula }
}
antlr4rust::tid!{Tff_quantified_formulaContextExt<'a>}

impl<'input> Tff_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_quantified_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_quantified_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_quantified_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_quantified_formulaContextExt<'input>>{

fn tff_quantifier(&self) -> Option<Rc<Tff_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_variable_list(&self) -> Option<Rc<Tff_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_unit_formula(&self) -> Option<Rc<Tff_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_quantified_formulaContextAttrs<'input> for Tff_quantified_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_quantified_formula(&mut self,)
	-> Result<Rc<Tff_quantified_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_quantified_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_tff_quantified_formula);
        let mut _localctx: Rc<Tff_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_quantifier*/
			recog.base.set_state(932);
			recog.tff_quantifier()?;

			recog.base.set_state(933);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(934);
			recog.tff_variable_list()?;

			recog.base.set_state(935);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(936);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(937);
			recog.tff_unit_formula()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_variable_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_variable_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_variable_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_variable_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_variable_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_variable_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_variable_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_variable_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_variable_list }
}
antlr4rust::tid!{Tff_variable_listContextExt<'a>}

impl<'input> Tff_variable_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_variable_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_variable_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_variable_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_variable_listContextExt<'input>>{

fn tff_variable(&self) -> Option<Rc<Tff_variableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_variable_list(&self) -> Option<Rc<Tff_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_variable_listContextAttrs<'input> for Tff_variable_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_variable_list(&mut self,)
	-> Result<Rc<Tff_variable_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_variable_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_tff_variable_list);
        let mut _localctx: Rc<Tff_variable_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(944);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(45,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_variable*/
					recog.base.set_state(939);
					recog.tff_variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_variable*/
					recog.base.set_state(940);
					recog.tff_variable()?;

					recog.base.set_state(941);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_variable_list*/
					recog.base.set_state(942);
					recog.tff_variable_list()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_variableContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_variableContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_variable(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_variable(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_variableContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_variableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_variable }
}
antlr4rust::tid!{Tff_variableContextExt<'a>}

impl<'input> Tff_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_variableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_variableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_variableContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_variableContextExt<'input>>{

fn tff_typed_variable(&self) -> Option<Rc<Tff_typed_variableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_variableContextAttrs<'input> for Tff_variableContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_variable(&mut self,)
	-> Result<Rc<Tff_variableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_variableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_tff_variable);
        let mut _localctx: Rc<Tff_variableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(948);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(46,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_typed_variable*/
					recog.base.set_state(946);
					recog.tff_typed_variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(947);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_typed_variableContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_typed_variableContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_typed_variable(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_typed_variable(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_typed_variableContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_typed_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_typed_variableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_typed_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_typed_variable }
}
antlr4rust::tid!{Tff_typed_variableContextExt<'a>}

impl<'input> Tff_typed_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_typed_variableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_typed_variableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_typed_variableContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_typed_variableContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_typed_variableContextAttrs<'input> for Tff_typed_variableContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_typed_variable(&mut self,)
	-> Result<Rc<Tff_typed_variableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_typed_variableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_tff_typed_variable);
        let mut _localctx: Rc<Tff_typed_variableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule variable*/
			recog.base.set_state(950);
			recog.variable()?;

			recog.base.set_state(951);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(952);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_unary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_unary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_unary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_unary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_unary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_unary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_unary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_unary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_unary_formula }
}
antlr4rust::tid!{Tff_unary_formulaContextExt<'a>}

impl<'input> Tff_unary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_unary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_unary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_unary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_unary_formulaContextExt<'input>>{

fn tff_prefix_unary(&self) -> Option<Rc<Tff_prefix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_infix_unary(&self) -> Option<Rc<Tff_infix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_unary_formulaContextAttrs<'input> for Tff_unary_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_unary_formula(&mut self,)
	-> Result<Rc<Tff_unary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_unary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_tff_unary_formula);
        let mut _localctx: Rc<Tff_unary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(956);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_T__26 |TPTP_Less_sign 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_prefix_unary*/
					recog.base.set_state(954);
					recog.tff_prefix_unary()?;

					}
				}

			TPTP_T__11 |TPTP_T__13 |TPTP_T__16 |TPTP_T__17 |TPTP_T__18 |TPTP_Single_quoted |
			TPTP_Back_quoted |TPTP_Distinct_object |TPTP_Dollar_word |TPTP_Dollar_dollar_word |
			TPTP_Upper_word |TPTP_Lower_word |TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_infix_unary*/
					recog.base.set_state(955);
					recog.tff_infix_unary()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_prefix_unary ----------------
pub type Tff_prefix_unaryContextAll<'input> = Tff_prefix_unaryContext<'input>;


pub type Tff_prefix_unaryContext<'input> = BaseParserRuleContext<'input,Tff_prefix_unaryContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_prefix_unaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_prefix_unaryContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_prefix_unaryContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_prefix_unary(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_prefix_unary(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_prefix_unaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_prefix_unary(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_prefix_unaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_prefix_unary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_prefix_unary }
}
antlr4rust::tid!{Tff_prefix_unaryContextExt<'a>}

impl<'input> Tff_prefix_unaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_prefix_unaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_prefix_unaryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_prefix_unaryContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_prefix_unaryContextExt<'input>>{

fn tff_unary_connective(&self) -> Option<Rc<Tff_unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_preunit_formula(&self) -> Option<Rc<Tff_preunit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_prefix_unaryContextAttrs<'input> for Tff_prefix_unaryContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_prefix_unary(&mut self,)
	-> Result<Rc<Tff_prefix_unaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_prefix_unaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_tff_prefix_unary);
        let mut _localctx: Rc<Tff_prefix_unaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_unary_connective*/
			recog.base.set_state(958);
			recog.tff_unary_connective()?;

			/*InvokeRule tff_preunit_formula*/
			recog.base.set_state(959);
			recog.tff_preunit_formula()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_infix_unary ----------------
pub type Tff_infix_unaryContextAll<'input> = Tff_infix_unaryContext<'input>;


pub type Tff_infix_unaryContext<'input> = BaseParserRuleContext<'input,Tff_infix_unaryContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_infix_unaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_infix_unaryContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_infix_unaryContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_infix_unary(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_infix_unary(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_infix_unaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_infix_unary(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_infix_unaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_infix_unary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_infix_unary }
}
antlr4rust::tid!{Tff_infix_unaryContextExt<'a>}

impl<'input> Tff_infix_unaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_infix_unaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_infix_unaryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_infix_unaryContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_infix_unaryContextExt<'input>>{

fn tff_unitary_term_all(&self) ->  Vec<Rc<Tff_unitary_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_unitary_term(&self, i: usize) -> Option<Rc<Tff_unitary_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn infix_inequality(&self) -> Option<Rc<Infix_inequalityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_infix_unaryContextAttrs<'input> for Tff_infix_unaryContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_infix_unary(&mut self,)
	-> Result<Rc<Tff_infix_unaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_infix_unaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_tff_infix_unary);
        let mut _localctx: Rc<Tff_infix_unaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_unitary_term*/
			recog.base.set_state(961);
			recog.tff_unitary_term()?;

			/*InvokeRule infix_inequality*/
			recog.base.set_state(962);
			recog.infix_inequality()?;

			/*InvokeRule tff_unitary_term*/
			recog.base.set_state(963);
			recog.tff_unitary_term()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_atomic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_atomic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_atomic_formula }
}
antlr4rust::tid!{Tff_atomic_formulaContextExt<'a>}

impl<'input> Tff_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_atomic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_atomic_formulaContextExt<'input>>{

fn tff_plain_atomic(&self) -> Option<Rc<Tff_plain_atomicContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_defined_atomic(&self) -> Option<Rc<Tff_defined_atomicContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_system_atomic(&self) -> Option<Rc<Tff_system_atomicContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_atomic_formulaContextAttrs<'input> for Tff_atomic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_atomic_formula(&mut self,)
	-> Result<Rc<Tff_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_tff_atomic_formula);
        let mut _localctx: Rc<Tff_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(968);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_plain_atomic*/
					recog.base.set_state(965);
					recog.tff_plain_atomic()?;

					}
				}

			TPTP_T__16 |TPTP_T__18 |TPTP_Dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_defined_atomic*/
					recog.base.set_state(966);
					recog.tff_defined_atomic()?;

					}
				}

			TPTP_Dollar_dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tff_system_atomic*/
					recog.base.set_state(967);
					recog.tff_system_atomic()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_plain_atomic ----------------
pub type Tff_plain_atomicContextAll<'input> = Tff_plain_atomicContext<'input>;


pub type Tff_plain_atomicContext<'input> = BaseParserRuleContext<'input,Tff_plain_atomicContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_plain_atomicContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_plain_atomicContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_plain_atomicContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_plain_atomic(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_plain_atomic(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_plain_atomicContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_plain_atomic(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_plain_atomicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_plain_atomic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_plain_atomic }
}
antlr4rust::tid!{Tff_plain_atomicContextExt<'a>}

impl<'input> Tff_plain_atomicContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_plain_atomicContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_plain_atomicContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_plain_atomicContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_plain_atomicContextExt<'input>>{

fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functor(&self) -> Option<Rc<FunctorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_arguments(&self) -> Option<Rc<Tff_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_plain_atomicContextAttrs<'input> for Tff_plain_atomicContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_plain_atomic(&mut self,)
	-> Result<Rc<Tff_plain_atomicContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_plain_atomicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_tff_plain_atomic);
        let mut _localctx: Rc<Tff_plain_atomicContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(976);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(49,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule constant*/
					recog.base.set_state(970);
					recog.constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule functor*/
					recog.base.set_state(971);
					recog.functor()?;

					recog.base.set_state(972);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_arguments*/
					recog.base.set_state(973);
					recog.tff_arguments()?;

					recog.base.set_state(974);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_defined_atomic ----------------
pub type Tff_defined_atomicContextAll<'input> = Tff_defined_atomicContext<'input>;


pub type Tff_defined_atomicContext<'input> = BaseParserRuleContext<'input,Tff_defined_atomicContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_defined_atomicContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_defined_atomicContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_defined_atomicContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_defined_atomic(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_defined_atomic(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_defined_atomicContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_defined_atomic(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_defined_atomicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_defined_atomic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_defined_atomic }
}
antlr4rust::tid!{Tff_defined_atomicContextExt<'a>}

impl<'input> Tff_defined_atomicContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_defined_atomicContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_defined_atomicContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_defined_atomicContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_defined_atomicContextExt<'input>>{

fn tff_defined_plain(&self) -> Option<Rc<Tff_defined_plainContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_defined_atomicContextAttrs<'input> for Tff_defined_atomicContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_defined_atomic(&mut self,)
	-> Result<Rc<Tff_defined_atomicContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_defined_atomicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_tff_defined_atomic);
        let mut _localctx: Rc<Tff_defined_atomicContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_defined_plain*/
			recog.base.set_state(978);
			recog.tff_defined_plain()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_defined_plain ----------------
pub type Tff_defined_plainContextAll<'input> = Tff_defined_plainContext<'input>;


pub type Tff_defined_plainContext<'input> = BaseParserRuleContext<'input,Tff_defined_plainContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_defined_plainContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_defined_plainContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_defined_plainContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_defined_plain(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_defined_plain(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_defined_plainContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_defined_plain(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_defined_plainContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_defined_plain }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_defined_plain }
}
antlr4rust::tid!{Tff_defined_plainContextExt<'a>}

impl<'input> Tff_defined_plainContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_defined_plainContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_defined_plainContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_defined_plainContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_defined_plainContextExt<'input>>{

fn defined_constant(&self) -> Option<Rc<Defined_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_functor(&self) -> Option<Rc<Defined_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_arguments(&self) -> Option<Rc<Tff_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nxf_atom(&self) -> Option<Rc<Nxf_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_let(&self) -> Option<Rc<Txf_letContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_defined_plainContextAttrs<'input> for Tff_defined_plainContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_defined_plain(&mut self,)
	-> Result<Rc<Tff_defined_plainContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_defined_plainContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_tff_defined_plain);
        let mut _localctx: Rc<Tff_defined_plainContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(988);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(50,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(980);
					recog.defined_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_functor*/
					recog.base.set_state(981);
					recog.defined_functor()?;

					recog.base.set_state(982);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_arguments*/
					recog.base.set_state(983);
					recog.tff_arguments()?;

					recog.base.set_state(984);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule nxf_atom*/
					recog.base.set_state(986);
					recog.nxf_atom()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule txf_let*/
					recog.base.set_state(987);
					recog.txf_let()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_defined_infix ----------------
pub type Tff_defined_infixContextAll<'input> = Tff_defined_infixContext<'input>;


pub type Tff_defined_infixContext<'input> = BaseParserRuleContext<'input,Tff_defined_infixContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_defined_infixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_defined_infixContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_defined_infixContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_defined_infix(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_defined_infix(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_defined_infixContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_defined_infix(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_defined_infixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_defined_infix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_defined_infix }
}
antlr4rust::tid!{Tff_defined_infixContextExt<'a>}

impl<'input> Tff_defined_infixContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_defined_infixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_defined_infixContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_defined_infixContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_defined_infixContextExt<'input>>{

fn tff_unitary_term_all(&self) ->  Vec<Rc<Tff_unitary_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn tff_unitary_term(&self, i: usize) -> Option<Rc<Tff_unitary_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn defined_infix_pred(&self) -> Option<Rc<Defined_infix_predContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_defined_infixContextAttrs<'input> for Tff_defined_infixContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_defined_infix(&mut self,)
	-> Result<Rc<Tff_defined_infixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_defined_infixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_tff_defined_infix);
        let mut _localctx: Rc<Tff_defined_infixContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_unitary_term*/
			recog.base.set_state(990);
			recog.tff_unitary_term()?;

			/*InvokeRule defined_infix_pred*/
			recog.base.set_state(991);
			recog.defined_infix_pred()?;

			/*InvokeRule tff_unitary_term*/
			recog.base.set_state(992);
			recog.tff_unitary_term()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_system_atomic ----------------
pub type Tff_system_atomicContextAll<'input> = Tff_system_atomicContext<'input>;


pub type Tff_system_atomicContext<'input> = BaseParserRuleContext<'input,Tff_system_atomicContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_system_atomicContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_system_atomicContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_system_atomicContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_system_atomic(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_system_atomic(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_system_atomicContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_system_atomic(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_system_atomicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_system_atomic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_system_atomic }
}
antlr4rust::tid!{Tff_system_atomicContextExt<'a>}

impl<'input> Tff_system_atomicContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_system_atomicContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_system_atomicContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_system_atomicContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_system_atomicContextExt<'input>>{

fn system_constant(&self) -> Option<Rc<System_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn system_functor(&self) -> Option<Rc<System_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_arguments(&self) -> Option<Rc<Tff_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_system_atomicContextAttrs<'input> for Tff_system_atomicContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_system_atomic(&mut self,)
	-> Result<Rc<Tff_system_atomicContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_system_atomicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_tff_system_atomic);
        let mut _localctx: Rc<Tff_system_atomicContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1000);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(51,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule system_constant*/
					recog.base.set_state(994);
					recog.system_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule system_functor*/
					recog.base.set_state(995);
					recog.system_functor()?;

					recog.base.set_state(996);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_arguments*/
					recog.base.set_state(997);
					recog.tff_arguments()?;

					recog.base.set_state(998);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_let ----------------
pub type Txf_letContextAll<'input> = Txf_letContext<'input>;


pub type Txf_letContext<'input> = BaseParserRuleContext<'input,Txf_letContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_letContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_letContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_letContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_let(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_let(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_letContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_let(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_letContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_let }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_let }
}
antlr4rust::tid!{Txf_letContextExt<'a>}

impl<'input> Txf_letContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_letContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_letContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_letContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_letContextExt<'input>>{

fn txf_let_types(&self) -> Option<Rc<Txf_let_typesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_let_defns(&self) -> Option<Rc<Txf_let_defnsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_term(&self) -> Option<Rc<Tff_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_letContextAttrs<'input> for Txf_letContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_let(&mut self,)
	-> Result<Rc<Txf_letContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_letContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_txf_let);
        let mut _localctx: Rc<Txf_letContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1002);
			recog.base.match_token(TPTP_T__16,&mut recog.err_handler)?;

			/*InvokeRule txf_let_types*/
			recog.base.set_state(1003);
			recog.txf_let_types()?;

			recog.base.set_state(1004);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule txf_let_defns*/
			recog.base.set_state(1005);
			recog.txf_let_defns()?;

			recog.base.set_state(1006);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_term*/
			recog.base.set_state(1007);
			recog.tff_term()?;

			recog.base.set_state(1008);
			recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_let_types ----------------
pub type Txf_let_typesContextAll<'input> = Txf_let_typesContext<'input>;


pub type Txf_let_typesContext<'input> = BaseParserRuleContext<'input,Txf_let_typesContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_let_typesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_let_typesContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_let_typesContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_let_types(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_let_types(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_let_typesContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_let_types(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_let_typesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_let_types }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_let_types }
}
antlr4rust::tid!{Txf_let_typesContextExt<'a>}

impl<'input> Txf_let_typesContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_let_typesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_let_typesContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_let_typesContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_let_typesContextExt<'input>>{

fn tff_atom_typing(&self) -> Option<Rc<Tff_atom_typingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atom_typing_list(&self) -> Option<Rc<Tff_atom_typing_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_let_typesContextAttrs<'input> for Txf_let_typesContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_let_types(&mut self,)
	-> Result<Rc<Txf_let_typesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_let_typesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_txf_let_types);
        let mut _localctx: Rc<Txf_let_typesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1015);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__11 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Dollar_dollar_word |
			TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1010);
					recog.tff_atom_typing()?;

					}
				}

			TPTP_T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1011);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_atom_typing_list*/
					recog.base.set_state(1012);
					recog.tff_atom_typing_list()?;

					recog.base.set_state(1013);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_atom_typing_list ----------------
pub type Tff_atom_typing_listContextAll<'input> = Tff_atom_typing_listContext<'input>;


pub type Tff_atom_typing_listContext<'input> = BaseParserRuleContext<'input,Tff_atom_typing_listContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_atom_typing_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_atom_typing_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_atom_typing_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_atom_typing_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_atom_typing_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_atom_typing_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_atom_typing_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_atom_typing_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_atom_typing_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_atom_typing_list }
}
antlr4rust::tid!{Tff_atom_typing_listContextExt<'a>}

impl<'input> Tff_atom_typing_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_atom_typing_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_atom_typing_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_atom_typing_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_atom_typing_listContextExt<'input>>{

fn tff_atom_typing(&self) -> Option<Rc<Tff_atom_typingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atom_typing_list(&self) -> Option<Rc<Tff_atom_typing_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_atom_typing_listContextAttrs<'input> for Tff_atom_typing_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_atom_typing_list(&mut self,)
	-> Result<Rc<Tff_atom_typing_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_atom_typing_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_tff_atom_typing_list);
        let mut _localctx: Rc<Tff_atom_typing_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1022);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(53,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1017);
					recog.tff_atom_typing()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1018);
					recog.tff_atom_typing()?;

					recog.base.set_state(1019);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_atom_typing_list*/
					recog.base.set_state(1020);
					recog.tff_atom_typing_list()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_let_defns ----------------
pub type Txf_let_defnsContextAll<'input> = Txf_let_defnsContext<'input>;


pub type Txf_let_defnsContext<'input> = BaseParserRuleContext<'input,Txf_let_defnsContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_let_defnsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_let_defnsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_let_defnsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_let_defns(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_let_defns(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_let_defnsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_let_defns(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_let_defnsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_let_defns }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_let_defns }
}
antlr4rust::tid!{Txf_let_defnsContextExt<'a>}

impl<'input> Txf_let_defnsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_let_defnsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_let_defnsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_let_defnsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_let_defnsContextExt<'input>>{

fn txf_let_defn(&self) -> Option<Rc<Txf_let_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_let_defn_list(&self) -> Option<Rc<Txf_let_defn_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_let_defnsContextAttrs<'input> for Txf_let_defnsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_let_defns(&mut self,)
	-> Result<Rc<Txf_let_defnsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_let_defnsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_txf_let_defns);
        let mut _localctx: Rc<Txf_let_defnsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1029);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(54,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule txf_let_defn*/
					recog.base.set_state(1024);
					recog.txf_let_defn()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1025);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule txf_let_defn_list*/
					recog.base.set_state(1026);
					recog.txf_let_defn_list()?;

					recog.base.set_state(1027);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_let_defn ----------------
pub type Txf_let_defnContextAll<'input> = Txf_let_defnContext<'input>;


pub type Txf_let_defnContext<'input> = BaseParserRuleContext<'input,Txf_let_defnContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_let_defnContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_let_defnContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_let_defnContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_let_defn(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_let_defn(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_let_defnContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_let_defn(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_let_defnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_let_defn }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_let_defn }
}
antlr4rust::tid!{Txf_let_defnContextExt<'a>}

impl<'input> Txf_let_defnContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_let_defnContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_let_defnContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_let_defnContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_let_defnContextExt<'input>>{

fn txf_let_LHS(&self) -> Option<Rc<Txf_let_LHSContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignment(&self) -> Option<Rc<AssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_term(&self) -> Option<Rc<Tff_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_let_defnContextAttrs<'input> for Txf_let_defnContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_let_defn(&mut self,)
	-> Result<Rc<Txf_let_defnContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_let_defnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_txf_let_defn);
        let mut _localctx: Rc<Txf_let_defnContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule txf_let_LHS*/
			recog.base.set_state(1031);
			recog.txf_let_LHS()?;

			/*InvokeRule assignment*/
			recog.base.set_state(1032);
			recog.assignment()?;

			/*InvokeRule tff_term*/
			recog.base.set_state(1033);
			recog.tff_term()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_let_LHS ----------------
pub type Txf_let_LHSContextAll<'input> = Txf_let_LHSContext<'input>;


pub type Txf_let_LHSContext<'input> = BaseParserRuleContext<'input,Txf_let_LHSContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_let_LHSContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_let_LHSContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_let_LHSContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_let_LHS(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_let_LHS(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_let_LHSContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_let_LHS(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_let_LHSContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_let_LHS }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_let_LHS }
}
antlr4rust::tid!{Txf_let_LHSContextExt<'a>}

impl<'input> Txf_let_LHSContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_let_LHSContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_let_LHSContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_let_LHSContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_let_LHSContextExt<'input>>{

fn tff_plain_atomic(&self) -> Option<Rc<Tff_plain_atomicContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_tuple(&self) -> Option<Rc<Txf_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_let_LHSContextAttrs<'input> for Txf_let_LHSContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_let_LHS(&mut self,)
	-> Result<Rc<Txf_let_LHSContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_let_LHSContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_txf_let_LHS);
        let mut _localctx: Rc<Txf_let_LHSContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1037);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_plain_atomic*/
					recog.base.set_state(1035);
					recog.tff_plain_atomic()?;

					}
				}

			TPTP_T__13 |TPTP_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule txf_tuple*/
					recog.base.set_state(1036);
					recog.txf_tuple()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_let_defn_list ----------------
pub type Txf_let_defn_listContextAll<'input> = Txf_let_defn_listContext<'input>;


pub type Txf_let_defn_listContext<'input> = BaseParserRuleContext<'input,Txf_let_defn_listContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_let_defn_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_let_defn_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_let_defn_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_let_defn_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_let_defn_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_let_defn_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_let_defn_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_let_defn_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_let_defn_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_let_defn_list }
}
antlr4rust::tid!{Txf_let_defn_listContextExt<'a>}

impl<'input> Txf_let_defn_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_let_defn_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_let_defn_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_let_defn_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_let_defn_listContextExt<'input>>{

fn txf_let_defn(&self) -> Option<Rc<Txf_let_defnContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_let_defn_list(&self) -> Option<Rc<Txf_let_defn_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_let_defn_listContextAttrs<'input> for Txf_let_defn_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_let_defn_list(&mut self,)
	-> Result<Rc<Txf_let_defn_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_let_defn_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_txf_let_defn_list);
        let mut _localctx: Rc<Txf_let_defn_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1044);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(56,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule txf_let_defn*/
					recog.base.set_state(1039);
					recog.txf_let_defn()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule txf_let_defn*/
					recog.base.set_state(1040);
					recog.txf_let_defn()?;

					recog.base.set_state(1041);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule txf_let_defn_list*/
					recog.base.set_state(1042);
					recog.txf_let_defn_list()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nxf_atom ----------------
pub type Nxf_atomContextAll<'input> = Nxf_atomContext<'input>;


pub type Nxf_atomContext<'input> = BaseParserRuleContext<'input,Nxf_atomContextExt<'input>>;

#[derive(Clone)]
pub struct Nxf_atomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nxf_atomContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nxf_atomContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nxf_atom(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nxf_atom(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nxf_atomContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nxf_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nxf_atomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nxf_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nxf_atom }
}
antlr4rust::tid!{Nxf_atomContextExt<'a>}

impl<'input> Nxf_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nxf_atomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nxf_atomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nxf_atomContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nxf_atomContextExt<'input>>{

fn nxf_long_connective(&self) -> Option<Rc<Nxf_long_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_arguments(&self) -> Option<Rc<Tff_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Nxf_atomContextAttrs<'input> for Nxf_atomContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nxf_atom(&mut self,)
	-> Result<Rc<Nxf_atomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nxf_atomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_nxf_atom);
        let mut _localctx: Rc<Nxf_atomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule nxf_long_connective*/
			recog.base.set_state(1046);
			recog.nxf_long_connective()?;

			recog.base.set_state(1047);
			recog.base.match_token(TPTP_T__10,&mut recog.err_handler)?;

			recog.base.set_state(1048);
			recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

			/*InvokeRule tff_arguments*/
			recog.base.set_state(1049);
			recog.tff_arguments()?;

			recog.base.set_state(1050);
			recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_term ----------------
pub type Tff_termContextAll<'input> = Tff_termContext<'input>;


pub type Tff_termContext<'input> = BaseParserRuleContext<'input,Tff_termContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_term }
}
antlr4rust::tid!{Tff_termContextExt<'a>}

impl<'input> Tff_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_termContextExt<'input>>{

fn tff_logic_formula(&self) -> Option<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_term(&self) -> Option<Rc<Defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_tuple(&self) -> Option<Rc<Txf_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_termContextAttrs<'input> for Tff_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_term(&mut self,)
	-> Result<Rc<Tff_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_tff_term);
        let mut _localctx: Rc<Tff_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1055);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(57,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(1052);
					recog.tff_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(1053);
					recog.defined_term()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule txf_tuple*/
					recog.base.set_state(1054);
					recog.txf_tuple()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_unitary_term ----------------
pub type Tff_unitary_termContextAll<'input> = Tff_unitary_termContext<'input>;


pub type Tff_unitary_termContext<'input> = BaseParserRuleContext<'input,Tff_unitary_termContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_unitary_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_unitary_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_unitary_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_unitary_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_unitary_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_unitary_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_unitary_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_unitary_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_unitary_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_unitary_term }
}
antlr4rust::tid!{Tff_unitary_termContextExt<'a>}

impl<'input> Tff_unitary_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_unitary_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_unitary_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_unitary_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_unitary_termContextExt<'input>>{

fn tff_atomic_formula(&self) -> Option<Rc<Tff_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_term(&self) -> Option<Rc<Defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_tuple(&self) -> Option<Rc<Txf_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_logic_formula(&self) -> Option<Rc<Tff_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_unitary_termContextAttrs<'input> for Tff_unitary_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_unitary_term(&mut self,)
	-> Result<Rc<Tff_unitary_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_unitary_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_tff_unitary_term);
        let mut _localctx: Rc<Tff_unitary_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1065);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__16 |TPTP_T__18 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Dollar_word |
			TPTP_Dollar_dollar_word |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_formula*/
					recog.base.set_state(1057);
					recog.tff_atomic_formula()?;

					}
				}

			TPTP_Distinct_object |TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(1058);
					recog.defined_term()?;

					}
				}

			TPTP_T__13 |TPTP_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule txf_tuple*/
					recog.base.set_state(1059);
					recog.txf_tuple()?;

					}
				}

			TPTP_Upper_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1060);
					recog.variable()?;

					}
				}

			TPTP_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1061);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(1062);
					recog.tff_logic_formula()?;

					recog.base.set_state(1063);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_tuple ----------------
pub type Txf_tupleContextAll<'input> = Txf_tupleContext<'input>;


pub type Txf_tupleContext<'input> = BaseParserRuleContext<'input,Txf_tupleContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_tupleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_tupleContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_tupleContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_tuple(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_tuple(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_tupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_tuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_tupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_tuple }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_tuple }
}
antlr4rust::tid!{Txf_tupleContextExt<'a>}

impl<'input> Txf_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_tupleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_tupleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_tupleContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_tupleContextExt<'input>>{

fn tff_arguments(&self) -> Option<Rc<Tff_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_tupleContextAttrs<'input> for Txf_tupleContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_tuple(&mut self,)
	-> Result<Rc<Txf_tupleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_tupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_txf_tuple);
        let mut _localctx: Rc<Txf_tupleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1072);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1067);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1068);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_arguments*/
					recog.base.set_state(1069);
					recog.tff_arguments()?;

					recog.base.set_state(1070);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_arguments ----------------
pub type Tff_argumentsContextAll<'input> = Tff_argumentsContext<'input>;


pub type Tff_argumentsContext<'input> = BaseParserRuleContext<'input,Tff_argumentsContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_argumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_argumentsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_argumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_arguments(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_arguments(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_argumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_arguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_argumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_arguments }
}
antlr4rust::tid!{Tff_argumentsContextExt<'a>}

impl<'input> Tff_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_argumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_argumentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_argumentsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_argumentsContextExt<'input>>{

fn tff_term(&self) -> Option<Rc<Tff_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comma_tff_term_all(&self) ->  Vec<Rc<Comma_tff_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comma_tff_term(&self, i: usize) -> Option<Rc<Comma_tff_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Tff_argumentsContextAttrs<'input> for Tff_argumentsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_arguments(&mut self,)
	-> Result<Rc<Tff_argumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_argumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_tff_arguments);
        let mut _localctx: Rc<Tff_argumentsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_term*/
			recog.base.set_state(1074);
			recog.tff_term()?;

			recog.base.set_state(1078);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_tff_term*/
				recog.base.set_state(1075);
				recog.comma_tff_term()?;

				}
				}
				recog.base.set_state(1080);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- comma_tff_term ----------------
pub type Comma_tff_termContextAll<'input> = Comma_tff_termContext<'input>;


pub type Comma_tff_termContext<'input> = BaseParserRuleContext<'input,Comma_tff_termContextExt<'input>>;

#[derive(Clone)]
pub struct Comma_tff_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Comma_tff_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Comma_tff_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_comma_tff_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_comma_tff_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Comma_tff_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_comma_tff_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Comma_tff_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comma_tff_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comma_tff_term }
}
antlr4rust::tid!{Comma_tff_termContextExt<'a>}

impl<'input> Comma_tff_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Comma_tff_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Comma_tff_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Comma_tff_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Comma_tff_termContextExt<'input>>{

fn tff_term(&self) -> Option<Rc<Tff_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Comma_tff_termContextAttrs<'input> for Comma_tff_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comma_tff_term(&mut self,)
	-> Result<Rc<Comma_tff_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Comma_tff_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_comma_tff_term);
        let mut _localctx: Rc<Comma_tff_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1081);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_term*/
			recog.base.set_state(1082);
			recog.tff_term()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_atom_typing ----------------
pub type Tff_atom_typingContextAll<'input> = Tff_atom_typingContext<'input>;


pub type Tff_atom_typingContext<'input> = BaseParserRuleContext<'input,Tff_atom_typingContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_atom_typingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_atom_typingContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_atom_typingContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_atom_typing(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_atom_typing(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_atom_typingContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_atom_typing(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_atom_typingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_atom_typing }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_atom_typing }
}
antlr4rust::tid!{Tff_atom_typingContextExt<'a>}

impl<'input> Tff_atom_typingContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_atom_typingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_atom_typingContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_atom_typingContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_atom_typingContextExt<'input>>{

fn untyped_atom(&self) -> Option<Rc<Untyped_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_top_level_type(&self) -> Option<Rc<Tff_top_level_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atom_typing(&self) -> Option<Rc<Tff_atom_typingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_atom_typingContextAttrs<'input> for Tff_atom_typingContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_atom_typing(&mut self,)
	-> Result<Rc<Tff_atom_typingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_atom_typingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_tff_atom_typing);
        let mut _localctx: Rc<Tff_atom_typingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1092);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Dollar_dollar_word |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule untyped_atom*/
					recog.base.set_state(1084);
					recog.untyped_atom()?;

					recog.base.set_state(1085);
					recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(1086);
					recog.tff_top_level_type()?;

					}
				}

			TPTP_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1088);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1089);
					recog.tff_atom_typing()?;

					recog.base.set_state(1090);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_top_level_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_top_level_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_top_level_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_top_level_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_top_level_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_top_level_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_top_level_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_top_level_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_top_level_type }
}
antlr4rust::tid!{Tff_top_level_typeContextExt<'a>}

impl<'input> Tff_top_level_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_top_level_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_top_level_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_top_level_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_top_level_typeContextExt<'input>>{

fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_non_atomic_type(&self) -> Option<Rc<Tff_non_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_top_level_typeContextAttrs<'input> for Tff_top_level_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_top_level_type(&mut self,)
	-> Result<Rc<Tff_top_level_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_top_level_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_tff_top_level_type);
        let mut _localctx: Rc<Tff_top_level_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1096);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(62,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1094);
					recog.tff_atomic_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_non_atomic_type*/
					recog.base.set_state(1095);
					recog.tff_non_atomic_type()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_non_atomic_type ----------------
pub type Tff_non_atomic_typeContextAll<'input> = Tff_non_atomic_typeContext<'input>;


pub type Tff_non_atomic_typeContext<'input> = BaseParserRuleContext<'input,Tff_non_atomic_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_non_atomic_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_non_atomic_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_non_atomic_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_non_atomic_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_non_atomic_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_non_atomic_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_non_atomic_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_non_atomic_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_non_atomic_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_non_atomic_type }
}
antlr4rust::tid!{Tff_non_atomic_typeContextExt<'a>}

impl<'input> Tff_non_atomic_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_non_atomic_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_non_atomic_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_non_atomic_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_non_atomic_typeContextExt<'input>>{

fn tff_mapping_type(&self) -> Option<Rc<Tff_mapping_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tf1_quantified_type(&self) -> Option<Rc<Tf1_quantified_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_non_atomic_type(&self) -> Option<Rc<Tff_non_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_non_atomic_typeContextAttrs<'input> for Tff_non_atomic_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_non_atomic_type(&mut self,)
	-> Result<Rc<Tff_non_atomic_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_non_atomic_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_tff_non_atomic_type);
        let mut _localctx: Rc<Tff_non_atomic_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1104);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(63,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_mapping_type*/
					recog.base.set_state(1098);
					recog.tff_mapping_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tf1_quantified_type*/
					recog.base.set_state(1099);
					recog.tf1_quantified_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1100);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_non_atomic_type*/
					recog.base.set_state(1101);
					recog.tff_non_atomic_type()?;

					recog.base.set_state(1102);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tf1_quantified_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tf1_quantified_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tf1_quantified_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tf1_quantified_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tf1_quantified_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tf1_quantified_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tf1_quantified_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tf1_quantified_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tf1_quantified_type }
}
antlr4rust::tid!{Tf1_quantified_typeContextExt<'a>}

impl<'input> Tf1_quantified_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tf1_quantified_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tf1_quantified_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tf1_quantified_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tf1_quantified_typeContextExt<'input>>{

fn type_quantifier(&self) -> Option<Rc<Type_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_variable_list(&self) -> Option<Rc<Tff_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_monotype(&self) -> Option<Rc<Tff_monotypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tf1_quantified_typeContextAttrs<'input> for Tf1_quantified_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tf1_quantified_type(&mut self,)
	-> Result<Rc<Tf1_quantified_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tf1_quantified_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_tf1_quantified_type);
        let mut _localctx: Rc<Tf1_quantified_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule type_quantifier*/
			recog.base.set_state(1106);
			recog.type_quantifier()?;

			recog.base.set_state(1107);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(1108);
			recog.tff_variable_list()?;

			recog.base.set_state(1109);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(1110);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule tff_monotype*/
			recog.base.set_state(1111);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_monotypeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_monotypeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_monotype(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_monotype(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_monotypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_monotype(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_monotypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_monotype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_monotype }
}
antlr4rust::tid!{Tff_monotypeContextExt<'a>}

impl<'input> Tff_monotypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_monotypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_monotypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_monotypeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_monotypeContextExt<'input>>{

fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_mapping_type(&self) -> Option<Rc<Tff_mapping_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tf1_quantified_type(&self) -> Option<Rc<Tf1_quantified_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_monotypeContextAttrs<'input> for Tff_monotypeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_monotype(&mut self,)
	-> Result<Rc<Tff_monotypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_monotypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 202, RULE_tff_monotype);
        let mut _localctx: Rc<Tff_monotypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1119);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(64,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1113);
					recog.tff_atomic_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1114);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_mapping_type*/
					recog.base.set_state(1115);
					recog.tff_mapping_type()?;

					recog.base.set_state(1116);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tf1_quantified_type*/
					recog.base.set_state(1118);
					recog.tf1_quantified_type()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_unitary_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_unitary_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_unitary_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_unitary_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_unitary_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_unitary_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_unitary_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_unitary_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_unitary_type }
}
antlr4rust::tid!{Tff_unitary_typeContextExt<'a>}

impl<'input> Tff_unitary_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_unitary_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_unitary_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_unitary_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_unitary_typeContextExt<'input>>{

fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_xprod_type(&self) -> Option<Rc<Tff_xprod_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_unitary_typeContextAttrs<'input> for Tff_unitary_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_unitary_type(&mut self,)
	-> Result<Rc<Tff_unitary_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_unitary_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 204, RULE_tff_unitary_type);
        let mut _localctx: Rc<Tff_unitary_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1126);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(65,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1121);
					recog.tff_atomic_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1122);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_xprod_type*/
					recog.base.set_state(1123);
					recog.tff_xprod_type_rec(0)?;

					recog.base.set_state(1124);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_atomic_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_atomic_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_atomic_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_atomic_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_atomic_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_atomic_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_atomic_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_atomic_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_atomic_type }
}
antlr4rust::tid!{Tff_atomic_typeContextExt<'a>}

impl<'input> Tff_atomic_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_atomic_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_atomic_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_atomic_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_atomic_typeContextExt<'input>>{

fn type_constant(&self) -> Option<Rc<Type_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_type(&self) -> Option<Rc<Defined_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type_functor(&self) -> Option<Rc<Type_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_type_arguments(&self) -> Option<Rc<Tff_type_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn txf_tuple_type(&self) -> Option<Rc<Txf_tuple_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_atomic_typeContextAttrs<'input> for Tff_atomic_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_atomic_type(&mut self,)
	-> Result<Rc<Tff_atomic_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_atomic_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 206, RULE_tff_atomic_type);
        let mut _localctx: Rc<Tff_atomic_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1141);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(66,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule type_constant*/
					recog.base.set_state(1128);
					recog.type_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_type*/
					recog.base.set_state(1129);
					recog.defined_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1130);
					recog.variable()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule type_functor*/
					recog.base.set_state(1131);
					recog.type_functor()?;

					recog.base.set_state(1132);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_type_arguments*/
					recog.base.set_state(1133);
					recog.tff_type_arguments()?;

					recog.base.set_state(1134);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1136);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1137);
					recog.tff_atomic_type()?;

					recog.base.set_state(1138);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule txf_tuple_type*/
					recog.base.set_state(1140);
					recog.txf_tuple_type()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_type_argumentsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_type_argumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_type_arguments(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_type_arguments(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_type_argumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_type_arguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_type_argumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_type_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_type_arguments }
}
antlr4rust::tid!{Tff_type_argumentsContextExt<'a>}

impl<'input> Tff_type_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_type_argumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_type_argumentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_type_argumentsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_type_argumentsContextExt<'input>>{

fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_type_arguments(&self) -> Option<Rc<Tff_type_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_type_argumentsContextAttrs<'input> for Tff_type_argumentsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_type_arguments(&mut self,)
	-> Result<Rc<Tff_type_argumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_type_argumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 208, RULE_tff_type_arguments);
        let mut _localctx: Rc<Tff_type_argumentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1148);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(67,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1143);
					recog.tff_atomic_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1144);
					recog.tff_atomic_type()?;

					recog.base.set_state(1145);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_type_arguments*/
					recog.base.set_state(1146);
					recog.tff_type_arguments()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_mapping_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_mapping_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_mapping_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_mapping_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_mapping_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_mapping_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_mapping_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_mapping_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_mapping_type }
}
antlr4rust::tid!{Tff_mapping_typeContextExt<'a>}

impl<'input> Tff_mapping_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_mapping_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_mapping_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_mapping_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_mapping_typeContextExt<'input>>{

fn tff_unitary_type(&self) -> Option<Rc<Tff_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Arrow
/// Returns `None` if there is no child corresponding to token Arrow
fn Arrow(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Arrow, 0)
}
fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_mapping_typeContextAttrs<'input> for Tff_mapping_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_mapping_type(&mut self,)
	-> Result<Rc<Tff_mapping_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_mapping_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 210, RULE_tff_mapping_type);
        let mut _localctx: Rc<Tff_mapping_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_unitary_type*/
			recog.base.set_state(1150);
			recog.tff_unitary_type()?;

			recog.base.set_state(1151);
			recog.base.match_token(TPTP_Arrow,&mut recog.err_handler)?;

			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(1152);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_xprod_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_xprod_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_xprod_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_xprod_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_xprod_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_xprod_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_xprod_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_xprod_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_xprod_type }
}
antlr4rust::tid!{Tff_xprod_typeContextExt<'a>}

impl<'input> Tff_xprod_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_xprod_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_xprod_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_xprod_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_xprod_typeContextExt<'input>>{

fn tff_unitary_type(&self) -> Option<Rc<Tff_unitary_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Star, 0)
}
fn tff_atomic_type(&self) -> Option<Rc<Tff_atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_xprod_type(&self) -> Option<Rc<Tff_xprod_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_xprod_typeContextAttrs<'input> for Tff_xprod_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 212, RULE_tff_xprod_type, _p);
	    let mut _localctx: Rc<Tff_xprod_typeContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 212;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule tff_unitary_type*/
			recog.base.set_state(1155);
			recog.tff_unitary_type()?;

			recog.base.set_state(1156);
			recog.base.match_token(TPTP_Star,&mut recog.err_handler)?;

			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(1157);
			recog.tff_atomic_type()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1164);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(68,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Tff_xprod_typeContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_tff_xprod_type)?;
					_localctx = tmp;
					recog.base.set_state(1159);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1160);
					recog.base.match_token(TPTP_Star,&mut recog.err_handler)?;

					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1161);
					recog.tff_atomic_type()?;

					}
					} 
				}
				recog.base.set_state(1166);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(68,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- txf_tuple_type ----------------
pub type Txf_tuple_typeContextAll<'input> = Txf_tuple_typeContext<'input>;


pub type Txf_tuple_typeContext<'input> = BaseParserRuleContext<'input,Txf_tuple_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_tuple_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_tuple_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_tuple_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_tuple_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_tuple_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_tuple_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_tuple_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_tuple_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_tuple_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_tuple_type }
}
antlr4rust::tid!{Txf_tuple_typeContextExt<'a>}

impl<'input> Txf_tuple_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_tuple_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_tuple_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_tuple_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_tuple_typeContextExt<'input>>{

fn tff_type_list(&self) -> Option<Rc<Tff_type_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_tuple_typeContextAttrs<'input> for Txf_tuple_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_tuple_type(&mut self,)
	-> Result<Rc<Txf_tuple_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_tuple_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 214, RULE_txf_tuple_type);
        let mut _localctx: Rc<Txf_tuple_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1167);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_type_list*/
			recog.base.set_state(1168);
			recog.tff_type_list()?;

			recog.base.set_state(1169);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_type_list ----------------
pub type Tff_type_listContextAll<'input> = Tff_type_listContext<'input>;


pub type Tff_type_listContext<'input> = BaseParserRuleContext<'input,Tff_type_listContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_type_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_type_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_type_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_type_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_type_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_type_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_type_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_type_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_type_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_type_list }
}
antlr4rust::tid!{Tff_type_listContextExt<'a>}

impl<'input> Tff_type_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_type_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_type_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_type_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_type_listContextExt<'input>>{

fn tff_top_level_type(&self) -> Option<Rc<Tff_top_level_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_type_list(&self) -> Option<Rc<Tff_type_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_type_listContextAttrs<'input> for Tff_type_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_type_list(&mut self,)
	-> Result<Rc<Tff_type_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_type_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 216, RULE_tff_type_list);
        let mut _localctx: Rc<Tff_type_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1176);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(69,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(1171);
					recog.tff_top_level_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(1172);
					recog.tff_top_level_type()?;

					recog.base.set_state(1173);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_type_list*/
					recog.base.set_state(1174);
					recog.tff_type_list()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tff_subtypeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_subtypeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_subtype(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_subtype(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_subtypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_subtype(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_subtypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_subtype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_subtype }
}
antlr4rust::tid!{Tff_subtypeContextExt<'a>}

impl<'input> Tff_subtypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_subtypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_subtypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_subtypeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_subtypeContextExt<'input>>{

fn untyped_atom(&self) -> Option<Rc<Untyped_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subtype_sign(&self) -> Option<Rc<Subtype_signContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_subtypeContextAttrs<'input> for Tff_subtypeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_subtype(&mut self,)
	-> Result<Rc<Tff_subtypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_subtypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 218, RULE_tff_subtype);
        let mut _localctx: Rc<Tff_subtypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule untyped_atom*/
			recog.base.set_state(1178);
			recog.untyped_atom()?;

			/*InvokeRule subtype_sign*/
			recog.base.set_state(1179);
			recog.subtype_sign()?;

			/*InvokeRule atom*/
			recog.base.set_state(1180);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_definition ----------------
pub type Txf_definitionContextAll<'input> = Txf_definitionContext<'input>;


pub type Txf_definitionContext<'input> = BaseParserRuleContext<'input,Txf_definitionContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_definitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_definition(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_definition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_definitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_definition(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_definition }
}
antlr4rust::tid!{Txf_definitionContextExt<'a>}

impl<'input> Txf_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_definitionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_definitionContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_definitionContextExt<'input>>{

fn tff_atomic_formula(&self) -> Option<Rc<Tff_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identical(&self) -> Option<Rc<IdenticalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_term(&self) -> Option<Rc<Tff_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_definitionContextAttrs<'input> for Txf_definitionContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_definition(&mut self,)
	-> Result<Rc<Txf_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 220, RULE_txf_definition);
        let mut _localctx: Rc<Txf_definitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_atomic_formula*/
			recog.base.set_state(1182);
			recog.tff_atomic_formula()?;

			/*InvokeRule identical*/
			recog.base.set_state(1183);
			recog.identical()?;

			/*InvokeRule tff_term*/
			recog.base.set_state(1184);
			recog.tff_term()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- txf_sequent ----------------
pub type Txf_sequentContextAll<'input> = Txf_sequentContext<'input>;


pub type Txf_sequentContext<'input> = BaseParserRuleContext<'input,Txf_sequentContextExt<'input>>;

#[derive(Clone)]
pub struct Txf_sequentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Txf_sequentContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Txf_sequentContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_txf_sequent(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_txf_sequent(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Txf_sequentContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_txf_sequent(self);
	}
}

impl<'input> CustomRuleContext<'input> for Txf_sequentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_txf_sequent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_txf_sequent }
}
antlr4rust::tid!{Txf_sequentContextExt<'a>}

impl<'input> Txf_sequentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Txf_sequentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Txf_sequentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Txf_sequentContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Txf_sequentContextExt<'input>>{

fn txf_tuple_all(&self) ->  Vec<Rc<Txf_tupleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn txf_tuple(&self, i: usize) -> Option<Rc<Txf_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn gentzen_arrow(&self) -> Option<Rc<Gentzen_arrowContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Txf_sequentContextAttrs<'input> for Txf_sequentContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn txf_sequent(&mut self,)
	-> Result<Rc<Txf_sequentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Txf_sequentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 222, RULE_txf_sequent);
        let mut _localctx: Rc<Txf_sequentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule txf_tuple*/
			recog.base.set_state(1186);
			recog.txf_tuple()?;

			/*InvokeRule gentzen_arrow*/
			recog.base.set_state(1187);
			recog.gentzen_arrow()?;

			/*InvokeRule txf_tuple*/
			recog.base.set_state(1188);
			recog.txf_tuple()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nhf_long_connective ----------------
pub type Nhf_long_connectiveContextAll<'input> = Nhf_long_connectiveContext<'input>;


pub type Nhf_long_connectiveContext<'input> = BaseParserRuleContext<'input,Nhf_long_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Nhf_long_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nhf_long_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nhf_long_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nhf_long_connective(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nhf_long_connective(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nhf_long_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nhf_long_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nhf_long_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nhf_long_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nhf_long_connective }
}
antlr4rust::tid!{Nhf_long_connectiveContextExt<'a>}

impl<'input> Nhf_long_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nhf_long_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nhf_long_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nhf_long_connectiveContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nhf_long_connectiveContextExt<'input>>{

fn ntf_connective_name(&self) -> Option<Rc<Ntf_connective_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nhf_parameter_list(&self) -> Option<Rc<Nhf_parameter_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Nhf_long_connectiveContextAttrs<'input> for Nhf_long_connectiveContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nhf_long_connective(&mut self,)
	-> Result<Rc<Nhf_long_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nhf_long_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 224, RULE_nhf_long_connective);
        let mut _localctx: Rc<Nhf_long_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1200);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(70,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1190);
					recog.base.match_token(TPTP_T__18,&mut recog.err_handler)?;

					/*InvokeRule ntf_connective_name*/
					recog.base.set_state(1191);
					recog.ntf_connective_name()?;

					recog.base.set_state(1192);
					recog.base.match_token(TPTP_T__19,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1194);
					recog.base.match_token(TPTP_T__18,&mut recog.err_handler)?;

					/*InvokeRule ntf_connective_name*/
					recog.base.set_state(1195);
					recog.ntf_connective_name()?;

					recog.base.set_state(1196);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule nhf_parameter_list*/
					recog.base.set_state(1197);
					recog.nhf_parameter_list()?;

					recog.base.set_state(1198);
					recog.base.match_token(TPTP_T__20,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nhf_parameter_list ----------------
pub type Nhf_parameter_listContextAll<'input> = Nhf_parameter_listContext<'input>;


pub type Nhf_parameter_listContext<'input> = BaseParserRuleContext<'input,Nhf_parameter_listContextExt<'input>>;

#[derive(Clone)]
pub struct Nhf_parameter_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nhf_parameter_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nhf_parameter_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nhf_parameter_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nhf_parameter_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nhf_parameter_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nhf_parameter_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nhf_parameter_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nhf_parameter_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nhf_parameter_list }
}
antlr4rust::tid!{Nhf_parameter_listContextExt<'a>}

impl<'input> Nhf_parameter_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nhf_parameter_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nhf_parameter_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nhf_parameter_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nhf_parameter_listContextExt<'input>>{

fn nhf_parameter(&self) -> Option<Rc<Nhf_parameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nhf_parameter_list(&self) -> Option<Rc<Nhf_parameter_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Nhf_parameter_listContextAttrs<'input> for Nhf_parameter_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nhf_parameter_list(&mut self,)
	-> Result<Rc<Nhf_parameter_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nhf_parameter_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 226, RULE_nhf_parameter_list);
        let mut _localctx: Rc<Nhf_parameter_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1207);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(71,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nhf_parameter*/
					recog.base.set_state(1202);
					recog.nhf_parameter()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nhf_parameter*/
					recog.base.set_state(1203);
					recog.nhf_parameter()?;

					recog.base.set_state(1204);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule nhf_parameter_list*/
					recog.base.set_state(1205);
					recog.nhf_parameter_list()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nhf_parameter ----------------
pub type Nhf_parameterContextAll<'input> = Nhf_parameterContext<'input>;


pub type Nhf_parameterContext<'input> = BaseParserRuleContext<'input,Nhf_parameterContextExt<'input>>;

#[derive(Clone)]
pub struct Nhf_parameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nhf_parameterContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nhf_parameterContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nhf_parameter(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nhf_parameter(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nhf_parameterContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nhf_parameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nhf_parameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nhf_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nhf_parameter }
}
antlr4rust::tid!{Nhf_parameterContextExt<'a>}

impl<'input> Nhf_parameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nhf_parameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nhf_parameterContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nhf_parameterContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nhf_parameterContextExt<'input>>{

fn ntf_index(&self) -> Option<Rc<Ntf_indexContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nhf_key_pair(&self) -> Option<Rc<Nhf_key_pairContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Nhf_parameterContextAttrs<'input> for Nhf_parameterContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nhf_parameter(&mut self,)
	-> Result<Rc<Nhf_parameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nhf_parameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 228, RULE_nhf_parameter);
        let mut _localctx: Rc<Nhf_parameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1211);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Hash 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule ntf_index*/
					recog.base.set_state(1209);
					recog.ntf_index()?;

					}
				}

			TPTP_T__11 |TPTP_T__13 |TPTP_T__16 |TPTP_T__17 |TPTP_T__18 |TPTP_T__44 |
			TPTP_T__45 |TPTP_T__46 |TPTP_T__47 |TPTP_T__48 |TPTP_Single_quoted |TPTP_Back_quoted |
			TPTP_Distinct_object |TPTP_Dollar_word |TPTP_Dollar_dollar_word |TPTP_Lower_word |
			TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nhf_key_pair*/
					recog.base.set_state(1210);
					recog.nhf_key_pair()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nhf_key_pair ----------------
pub type Nhf_key_pairContextAll<'input> = Nhf_key_pairContext<'input>;


pub type Nhf_key_pairContext<'input> = BaseParserRuleContext<'input,Nhf_key_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Nhf_key_pairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nhf_key_pairContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nhf_key_pairContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nhf_key_pair(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nhf_key_pair(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nhf_key_pairContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nhf_key_pair(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nhf_key_pairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nhf_key_pair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nhf_key_pair }
}
antlr4rust::tid!{Nhf_key_pairContextExt<'a>}

impl<'input> Nhf_key_pairContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nhf_key_pairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nhf_key_pairContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nhf_key_pairContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nhf_key_pairContextExt<'input>>{

fn thf_definition(&self) -> Option<Rc<Thf_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Nhf_key_pairContextAttrs<'input> for Nhf_key_pairContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nhf_key_pair(&mut self,)
	-> Result<Rc<Nhf_key_pairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nhf_key_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 230, RULE_nhf_key_pair);
        let mut _localctx: Rc<Nhf_key_pairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_definition*/
			recog.base.set_state(1213);
			recog.thf_definition()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nxf_long_connective ----------------
pub type Nxf_long_connectiveContextAll<'input> = Nxf_long_connectiveContext<'input>;


pub type Nxf_long_connectiveContext<'input> = BaseParserRuleContext<'input,Nxf_long_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Nxf_long_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nxf_long_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nxf_long_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nxf_long_connective(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nxf_long_connective(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nxf_long_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nxf_long_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nxf_long_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nxf_long_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nxf_long_connective }
}
antlr4rust::tid!{Nxf_long_connectiveContextExt<'a>}

impl<'input> Nxf_long_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nxf_long_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nxf_long_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nxf_long_connectiveContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nxf_long_connectiveContextExt<'input>>{

fn ntf_connective_name(&self) -> Option<Rc<Ntf_connective_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nxf_parameter_list(&self) -> Option<Rc<Nxf_parameter_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Nxf_long_connectiveContextAttrs<'input> for Nxf_long_connectiveContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nxf_long_connective(&mut self,)
	-> Result<Rc<Nxf_long_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nxf_long_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 232, RULE_nxf_long_connective);
        let mut _localctx: Rc<Nxf_long_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1225);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(73,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1215);
					recog.base.match_token(TPTP_T__18,&mut recog.err_handler)?;

					/*InvokeRule ntf_connective_name*/
					recog.base.set_state(1216);
					recog.ntf_connective_name()?;

					recog.base.set_state(1217);
					recog.base.match_token(TPTP_T__19,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1219);
					recog.base.match_token(TPTP_T__18,&mut recog.err_handler)?;

					/*InvokeRule ntf_connective_name*/
					recog.base.set_state(1220);
					recog.ntf_connective_name()?;

					recog.base.set_state(1221);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule nxf_parameter_list*/
					recog.base.set_state(1222);
					recog.nxf_parameter_list()?;

					recog.base.set_state(1223);
					recog.base.match_token(TPTP_T__20,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nxf_parameter_list ----------------
pub type Nxf_parameter_listContextAll<'input> = Nxf_parameter_listContext<'input>;


pub type Nxf_parameter_listContext<'input> = BaseParserRuleContext<'input,Nxf_parameter_listContextExt<'input>>;

#[derive(Clone)]
pub struct Nxf_parameter_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nxf_parameter_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nxf_parameter_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nxf_parameter_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nxf_parameter_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nxf_parameter_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nxf_parameter_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nxf_parameter_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nxf_parameter_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nxf_parameter_list }
}
antlr4rust::tid!{Nxf_parameter_listContextExt<'a>}

impl<'input> Nxf_parameter_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nxf_parameter_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nxf_parameter_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nxf_parameter_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nxf_parameter_listContextExt<'input>>{

fn nxf_parameter(&self) -> Option<Rc<Nxf_parameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nxf_parameter_list(&self) -> Option<Rc<Nxf_parameter_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Nxf_parameter_listContextAttrs<'input> for Nxf_parameter_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nxf_parameter_list(&mut self,)
	-> Result<Rc<Nxf_parameter_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nxf_parameter_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 234, RULE_nxf_parameter_list);
        let mut _localctx: Rc<Nxf_parameter_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1232);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(74,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nxf_parameter*/
					recog.base.set_state(1227);
					recog.nxf_parameter()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nxf_parameter*/
					recog.base.set_state(1228);
					recog.nxf_parameter()?;

					recog.base.set_state(1229);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule nxf_parameter_list*/
					recog.base.set_state(1230);
					recog.nxf_parameter_list()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nxf_parameter ----------------
pub type Nxf_parameterContextAll<'input> = Nxf_parameterContext<'input>;


pub type Nxf_parameterContext<'input> = BaseParserRuleContext<'input,Nxf_parameterContextExt<'input>>;

#[derive(Clone)]
pub struct Nxf_parameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nxf_parameterContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nxf_parameterContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nxf_parameter(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nxf_parameter(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nxf_parameterContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nxf_parameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nxf_parameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nxf_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nxf_parameter }
}
antlr4rust::tid!{Nxf_parameterContextExt<'a>}

impl<'input> Nxf_parameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nxf_parameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nxf_parameterContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nxf_parameterContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nxf_parameterContextExt<'input>>{

fn ntf_index(&self) -> Option<Rc<Ntf_indexContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nxf_key_pair(&self) -> Option<Rc<Nxf_key_pairContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Nxf_parameterContextAttrs<'input> for Nxf_parameterContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nxf_parameter(&mut self,)
	-> Result<Rc<Nxf_parameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nxf_parameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 236, RULE_nxf_parameter);
        let mut _localctx: Rc<Nxf_parameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1236);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Hash 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule ntf_index*/
					recog.base.set_state(1234);
					recog.ntf_index()?;

					}
				}

			TPTP_T__16 |TPTP_T__18 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Dollar_word |
			TPTP_Dollar_dollar_word |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nxf_key_pair*/
					recog.base.set_state(1235);
					recog.nxf_key_pair()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nxf_key_pair ----------------
pub type Nxf_key_pairContextAll<'input> = Nxf_key_pairContext<'input>;


pub type Nxf_key_pairContext<'input> = BaseParserRuleContext<'input,Nxf_key_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Nxf_key_pairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nxf_key_pairContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nxf_key_pairContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nxf_key_pair(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nxf_key_pair(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nxf_key_pairContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nxf_key_pair(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nxf_key_pairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nxf_key_pair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nxf_key_pair }
}
antlr4rust::tid!{Nxf_key_pairContextExt<'a>}

impl<'input> Nxf_key_pairContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nxf_key_pairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nxf_key_pairContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nxf_key_pairContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nxf_key_pairContextExt<'input>>{

fn txf_definition(&self) -> Option<Rc<Txf_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Nxf_key_pairContextAttrs<'input> for Nxf_key_pairContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nxf_key_pair(&mut self,)
	-> Result<Rc<Nxf_key_pairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nxf_key_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 238, RULE_nxf_key_pair);
        let mut _localctx: Rc<Nxf_key_pairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule txf_definition*/
			recog.base.set_state(1238);
			recog.txf_definition()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- ntf_connective_name ----------------
pub type Ntf_connective_nameContextAll<'input> = Ntf_connective_nameContext<'input>;


pub type Ntf_connective_nameContext<'input> = BaseParserRuleContext<'input,Ntf_connective_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Ntf_connective_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Ntf_connective_nameContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Ntf_connective_nameContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_ntf_connective_name(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_ntf_connective_name(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Ntf_connective_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_ntf_connective_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Ntf_connective_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ntf_connective_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ntf_connective_name }
}
antlr4rust::tid!{Ntf_connective_nameContextExt<'a>}

impl<'input> Ntf_connective_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Ntf_connective_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Ntf_connective_nameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Ntf_connective_nameContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Ntf_connective_nameContextExt<'input>>{

fn def_or_sys_constant(&self) -> Option<Rc<Def_or_sys_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Ntf_connective_nameContextAttrs<'input> for Ntf_connective_nameContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn ntf_connective_name(&mut self,)
	-> Result<Rc<Ntf_connective_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Ntf_connective_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 240, RULE_ntf_connective_name);
        let mut _localctx: Rc<Ntf_connective_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule def_or_sys_constant*/
			recog.base.set_state(1240);
			recog.def_or_sys_constant()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- ntf_index ----------------
pub type Ntf_indexContextAll<'input> = Ntf_indexContext<'input>;


pub type Ntf_indexContext<'input> = BaseParserRuleContext<'input,Ntf_indexContextExt<'input>>;

#[derive(Clone)]
pub struct Ntf_indexContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Ntf_indexContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Ntf_indexContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_ntf_index(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_ntf_index(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Ntf_indexContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_ntf_index(self);
	}
}

impl<'input> CustomRuleContext<'input> for Ntf_indexContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ntf_index }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ntf_index }
}
antlr4rust::tid!{Ntf_indexContextExt<'a>}

impl<'input> Ntf_indexContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Ntf_indexContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Ntf_indexContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Ntf_indexContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Ntf_indexContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Hash
/// Returns `None` if there is no child corresponding to token Hash
fn Hash(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Hash, 0)
}
fn tff_unitary_term(&self) -> Option<Rc<Tff_unitary_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Ntf_indexContextAttrs<'input> for Ntf_indexContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn ntf_index(&mut self,)
	-> Result<Rc<Ntf_indexContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Ntf_indexContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 242, RULE_ntf_index);
        let mut _localctx: Rc<Ntf_indexContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1242);
			recog.base.match_token(TPTP_Hash,&mut recog.err_handler)?;

			/*InvokeRule tff_unitary_term*/
			recog.base.set_state(1243);
			recog.tff_unitary_term()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- ntf_short_connective ----------------
pub type Ntf_short_connectiveContextAll<'input> = Ntf_short_connectiveContext<'input>;


pub type Ntf_short_connectiveContext<'input> = BaseParserRuleContext<'input,Ntf_short_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Ntf_short_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Ntf_short_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Ntf_short_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_ntf_short_connective(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_ntf_short_connective(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Ntf_short_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_ntf_short_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Ntf_short_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ntf_short_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ntf_short_connective }
}
antlr4rust::tid!{Ntf_short_connectiveContextExt<'a>}

impl<'input> Ntf_short_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Ntf_short_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Ntf_short_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Ntf_short_connectiveContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Ntf_short_connectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Less_sign
/// Returns `None` if there is no child corresponding to token Less_sign
fn Less_sign(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Less_sign, 0)
}
/// Retrieves first TerminalNode corresponding to token Arrow
/// Returns `None` if there is no child corresponding to token Arrow
fn Arrow(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Arrow, 0)
}

}

impl<'input> Ntf_short_connectiveContextAttrs<'input> for Ntf_short_connectiveContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn ntf_short_connective(&mut self,)
	-> Result<Rc<Ntf_short_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Ntf_short_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 244, RULE_ntf_short_connective);
        let mut _localctx: Rc<Ntf_short_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1251);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__21 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1245);
					recog.base.match_token(TPTP_T__21,&mut recog.err_handler)?;

					}
				}

			TPTP_Less_sign 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1246);
					recog.base.match_token(TPTP_Less_sign,&mut recog.err_handler)?;

					recog.base.set_state(1247);
					recog.base.match_token(TPTP_T__22,&mut recog.err_handler)?;

					recog.base.set_state(1248);
					recog.base.match_token(TPTP_Arrow,&mut recog.err_handler)?;

					}
				}

			TPTP_T__23 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1249);
					recog.base.match_token(TPTP_T__23,&mut recog.err_handler)?;

					}
				}

			TPTP_T__24 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(1250);
					recog.base.match_token(TPTP_T__24,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tcf_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tcf_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tcf_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tcf_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tcf_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tcf_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tcf_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tcf_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tcf_formula }
}
antlr4rust::tid!{Tcf_formulaContextExt<'a>}

impl<'input> Tcf_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tcf_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tcf_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tcf_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tcf_formulaContextExt<'input>>{

fn tcf_logic_formula(&self) -> Option<Rc<Tcf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_atom_typing(&self) -> Option<Rc<Tff_atom_typingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tcf_formulaContextAttrs<'input> for Tcf_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tcf_formula(&mut self,)
	-> Result<Rc<Tcf_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tcf_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 246, RULE_tcf_formula);
        let mut _localctx: Rc<Tcf_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1255);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(77,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tcf_logic_formula*/
					recog.base.set_state(1253);
					recog.tcf_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1254);
					recog.tff_atom_typing()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tcf_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tcf_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tcf_logic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tcf_logic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tcf_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tcf_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tcf_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tcf_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tcf_logic_formula }
}
antlr4rust::tid!{Tcf_logic_formulaContextExt<'a>}

impl<'input> Tcf_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tcf_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tcf_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tcf_logic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tcf_logic_formulaContextExt<'input>>{

fn tcf_quantified_formula(&self) -> Option<Rc<Tcf_quantified_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cnf_formula(&self) -> Option<Rc<Cnf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tcf_logic_formulaContextAttrs<'input> for Tcf_logic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tcf_logic_formula(&mut self,)
	-> Result<Rc<Tcf_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tcf_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 248, RULE_tcf_logic_formula);
        let mut _localctx: Rc<Tcf_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1259);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__25 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tcf_quantified_formula*/
					recog.base.set_state(1257);
					recog.tcf_quantified_formula()?;

					}
				}

			TPTP_T__11 |TPTP_T__26 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |
			TPTP_Dollar_word |TPTP_Dollar_dollar_word |TPTP_Upper_word |TPTP_Lower_word |
			TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule cnf_formula*/
					recog.base.set_state(1258);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Tcf_quantified_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tcf_quantified_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tcf_quantified_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tcf_quantified_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tcf_quantified_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tcf_quantified_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tcf_quantified_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tcf_quantified_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tcf_quantified_formula }
}
antlr4rust::tid!{Tcf_quantified_formulaContextExt<'a>}

impl<'input> Tcf_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tcf_quantified_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tcf_quantified_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tcf_quantified_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tcf_quantified_formulaContextExt<'input>>{

fn tff_variable_list(&self) -> Option<Rc<Tff_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tcf_logic_formula(&self) -> Option<Rc<Tcf_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tcf_quantified_formulaContextAttrs<'input> for Tcf_quantified_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tcf_quantified_formula(&mut self,)
	-> Result<Rc<Tcf_quantified_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tcf_quantified_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 250, RULE_tcf_quantified_formula);
        let mut _localctx: Rc<Tcf_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1261);
			recog.base.match_token(TPTP_T__25,&mut recog.err_handler)?;

			recog.base.set_state(1262);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(1263);
			recog.tff_variable_list()?;

			recog.base.set_state(1264);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(1265);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule tcf_logic_formula*/
			recog.base.set_state(1266);
			recog.tcf_logic_formula()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_formula }
}
antlr4rust::tid!{Fof_formulaContextExt<'a>}

impl<'input> Fof_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_formulaContextExt<'input>>{

fn fof_logic_formula(&self) -> Option<Rc<Fof_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_sequent(&self) -> Option<Rc<Fof_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_formulaContextAttrs<'input> for Fof_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_formula(&mut self,)
	-> Result<Rc<Fof_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 252, RULE_fof_formula);
        let mut _localctx: Rc<Fof_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1270);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(79,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_logic_formula*/
					recog.base.set_state(1268);
					recog.fof_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_sequent*/
					recog.base.set_state(1269);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_logic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_logic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_logic_formula }
}
antlr4rust::tid!{Fof_logic_formulaContextExt<'a>}

impl<'input> Fof_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_logic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_logic_formulaContextExt<'input>>{

fn fof_binary_formula(&self) -> Option<Rc<Fof_binary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_unary_formula(&self) -> Option<Rc<Fof_unary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_unitary_formula(&self) -> Option<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_logic_formulaContextAttrs<'input> for Fof_logic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_logic_formula(&mut self,)
	-> Result<Rc<Fof_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 254, RULE_fof_logic_formula);
        let mut _localctx: Rc<Fof_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1275);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(80,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_binary_formula*/
					recog.base.set_state(1272);
					recog.fof_binary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_unary_formula*/
					recog.base.set_state(1273);
					recog.fof_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule fof_unitary_formula*/
					recog.base.set_state(1274);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_binary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_binary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_binary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_binary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_binary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_binary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_binary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_binary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_binary_formula }
}
antlr4rust::tid!{Fof_binary_formulaContextExt<'a>}

impl<'input> Fof_binary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_binary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_binary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_binary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_binary_formulaContextExt<'input>>{

fn fof_binary_nonassoc(&self) -> Option<Rc<Fof_binary_nonassocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_binary_assoc(&self) -> Option<Rc<Fof_binary_assocContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_binary_formulaContextAttrs<'input> for Fof_binary_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_binary_formula(&mut self,)
	-> Result<Rc<Fof_binary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_binary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 256, RULE_fof_binary_formula);
        let mut _localctx: Rc<Fof_binary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1279);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(81,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_binary_nonassoc*/
					recog.base.set_state(1277);
					recog.fof_binary_nonassoc()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_binary_assoc*/
					recog.base.set_state(1278);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_binary_nonassocContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_binary_nonassocContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_binary_nonassoc(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_binary_nonassoc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_binary_nonassocContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_binary_nonassoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_binary_nonassocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_binary_nonassoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_binary_nonassoc }
}
antlr4rust::tid!{Fof_binary_nonassocContextExt<'a>}

impl<'input> Fof_binary_nonassocContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_binary_nonassocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_binary_nonassocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_binary_nonassocContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_binary_nonassocContextExt<'input>>{

fn fof_unit_formula_all(&self) ->  Vec<Rc<Fof_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_unit_formula(&self, i: usize) -> Option<Rc<Fof_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn nonassoc_connective(&self) -> Option<Rc<Nonassoc_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_binary_nonassocContextAttrs<'input> for Fof_binary_nonassocContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_binary_nonassoc(&mut self,)
	-> Result<Rc<Fof_binary_nonassocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_binary_nonassocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 258, RULE_fof_binary_nonassoc);
        let mut _localctx: Rc<Fof_binary_nonassocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1281);
			recog.fof_unit_formula()?;

			/*InvokeRule nonassoc_connective*/
			recog.base.set_state(1282);
			recog.nonassoc_connective()?;

			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1283);
			recog.fof_unit_formula()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_binary_assocContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_binary_assocContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_binary_assoc(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_binary_assoc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_binary_assocContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_binary_assoc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_binary_assocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_binary_assoc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_binary_assoc }
}
antlr4rust::tid!{Fof_binary_assocContextExt<'a>}

impl<'input> Fof_binary_assocContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_binary_assocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_binary_assocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_binary_assocContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_binary_assocContextExt<'input>>{

fn fof_or_formula(&self) -> Option<Rc<Fof_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_and_formula(&self) -> Option<Rc<Fof_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_binary_assocContextAttrs<'input> for Fof_binary_assocContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_binary_assoc(&mut self,)
	-> Result<Rc<Fof_binary_assocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_binary_assocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 260, RULE_fof_binary_assoc);
        let mut _localctx: Rc<Fof_binary_assocContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1287);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(82,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_or_formula*/
					recog.base.set_state(1285);
					recog.fof_or_formula_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_and_formula*/
					recog.base.set_state(1286);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_or_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_or_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_or_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_or_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_or_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_or_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_or_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_or_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_or_formula }
}
antlr4rust::tid!{Fof_or_formulaContextExt<'a>}

impl<'input> Fof_or_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_or_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_or_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_or_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_or_formulaContextExt<'input>>{

fn fof_unit_formula_all(&self) ->  Vec<Rc<Fof_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_unit_formula(&self, i: usize) -> Option<Rc<Fof_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Vline
/// Returns `None` if there is no child corresponding to token Vline
fn Vline(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Vline, 0)
}
fn fof_or_formula(&self) -> Option<Rc<Fof_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_or_formulaContextAttrs<'input> for Fof_or_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 262, RULE_fof_or_formula, _p);
	    let mut _localctx: Rc<Fof_or_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 262;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1290);
			recog.fof_unit_formula()?;

			recog.base.set_state(1291);
			recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1292);
			recog.fof_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1299);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Fof_or_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_fof_or_formula)?;
					_localctx = tmp;
					recog.base.set_state(1294);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1295);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					/*InvokeRule fof_unit_formula*/
					recog.base.set_state(1296);
					recog.fof_unit_formula()?;

					}
					} 
				}
				recog.base.set_state(1301);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

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

impl<'input> TPTPParserContext<'input> for Fof_and_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_and_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_and_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_and_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_and_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_and_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_and_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_and_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_and_formula }
}
antlr4rust::tid!{Fof_and_formulaContextExt<'a>}

impl<'input> Fof_and_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_and_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_and_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_and_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_and_formulaContextExt<'input>>{

fn fof_unit_formula_all(&self) ->  Vec<Rc<Fof_unit_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_unit_formula(&self, i: usize) -> Option<Rc<Fof_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn fof_and_formula(&self) -> Option<Rc<Fof_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_and_formulaContextAttrs<'input> for Fof_and_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 264, RULE_fof_and_formula, _p);
	    let mut _localctx: Rc<Fof_and_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 264;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1303);
			recog.fof_unit_formula()?;

			recog.base.set_state(1304);
			recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1305);
			recog.fof_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1312);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(84,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Fof_and_formulaContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_fof_and_formula)?;
					_localctx = tmp;
					recog.base.set_state(1307);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1308);
					recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_unit_formula*/
					recog.base.set_state(1309);
					recog.fof_unit_formula()?;

					}
					} 
				}
				recog.base.set_state(1314);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(84,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

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

impl<'input> TPTPParserContext<'input> for Fof_unary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_unary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_unary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_unary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_unary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_unary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_unary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_unary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_unary_formula }
}
antlr4rust::tid!{Fof_unary_formulaContextExt<'a>}

impl<'input> Fof_unary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_unary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_unary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_unary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_unary_formulaContextExt<'input>>{

fn unary_connective(&self) -> Option<Rc<Unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_unit_formula(&self) -> Option<Rc<Fof_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_infix_unary(&self) -> Option<Rc<Fof_infix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_unary_formulaContextAttrs<'input> for Fof_unary_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_unary_formula(&mut self,)
	-> Result<Rc<Fof_unary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_unary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 266, RULE_fof_unary_formula);
        let mut _localctx: Rc<Fof_unary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1319);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__26 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(1315);
					recog.unary_connective()?;

					/*InvokeRule fof_unit_formula*/
					recog.base.set_state(1316);
					recog.fof_unit_formula()?;

					}
				}

			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |TPTP_Dollar_word |
			TPTP_Dollar_dollar_word |TPTP_Upper_word |TPTP_Lower_word |TPTP_Real |
			TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_infix_unary*/
					recog.base.set_state(1318);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_infix_unaryContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_infix_unaryContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_infix_unary(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_infix_unary(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_infix_unaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_infix_unary(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_infix_unaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_infix_unary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_infix_unary }
}
antlr4rust::tid!{Fof_infix_unaryContextExt<'a>}

impl<'input> Fof_infix_unaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_infix_unaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_infix_unaryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_infix_unaryContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_infix_unaryContextExt<'input>>{

fn fof_term_all(&self) ->  Vec<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_term(&self, i: usize) -> Option<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn infix_inequality(&self) -> Option<Rc<Infix_inequalityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_infix_unaryContextAttrs<'input> for Fof_infix_unaryContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_infix_unary(&mut self,)
	-> Result<Rc<Fof_infix_unaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_infix_unaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 268, RULE_fof_infix_unary);
        let mut _localctx: Rc<Fof_infix_unaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_term*/
			recog.base.set_state(1321);
			recog.fof_term()?;

			/*InvokeRule infix_inequality*/
			recog.base.set_state(1322);
			recog.infix_inequality()?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1323);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- fof_unit_formula ----------------
pub type Fof_unit_formulaContextAll<'input> = Fof_unit_formulaContext<'input>;


pub type Fof_unit_formulaContext<'input> = BaseParserRuleContext<'input,Fof_unit_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_unit_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Fof_unit_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_unit_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_unit_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_unit_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_unit_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_unit_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_unit_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_unit_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_unit_formula }
}
antlr4rust::tid!{Fof_unit_formulaContextExt<'a>}

impl<'input> Fof_unit_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_unit_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_unit_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_unit_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_unit_formulaContextExt<'input>>{

fn fof_unitary_formula(&self) -> Option<Rc<Fof_unitary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_unary_formula(&self) -> Option<Rc<Fof_unary_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_unit_formulaContextAttrs<'input> for Fof_unit_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_unit_formula(&mut self,)
	-> Result<Rc<Fof_unit_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_unit_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 270, RULE_fof_unit_formula);
        let mut _localctx: Rc<Fof_unit_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1327);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(86,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_unitary_formula*/
					recog.base.set_state(1325);
					recog.fof_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_unary_formula*/
					recog.base.set_state(1326);
					recog.fof_unary_formula()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_unitary_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_unitary_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_unitary_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_unitary_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_unitary_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_unitary_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_unitary_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_unitary_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_unitary_formula }
}
antlr4rust::tid!{Fof_unitary_formulaContextExt<'a>}

impl<'input> Fof_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_unitary_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_unitary_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_unitary_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_unitary_formulaContextExt<'input>>{

fn fof_quantified_formula(&self) -> Option<Rc<Fof_quantified_formulaContextAll<'input>>> where Self:Sized{
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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_unitary_formula(&mut self,)
	-> Result<Rc<Fof_unitary_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_unitary_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 272, RULE_fof_unitary_formula);
        let mut _localctx: Rc<Fof_unitary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1335);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__25 |TPTP_T__33 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_quantified_formula*/
					recog.base.set_state(1329);
					recog.fof_quantified_formula()?;

					}
				}

			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |TPTP_Dollar_word |
			TPTP_Dollar_dollar_word |TPTP_Upper_word |TPTP_Lower_word |TPTP_Real |
			TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1330);
					recog.fof_atomic_formula()?;

					}
				}

			TPTP_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1331);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_logic_formula*/
					recog.base.set_state(1332);
					recog.fof_logic_formula()?;

					recog.base.set_state(1333);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_quantified_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_quantified_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_quantified_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_quantified_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_quantified_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_quantified_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_quantified_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_quantified_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_quantified_formula }
}
antlr4rust::tid!{Fof_quantified_formulaContextExt<'a>}

impl<'input> Fof_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_quantified_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_quantified_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_quantified_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_quantified_formulaContextExt<'input>>{

fn fof_quantifier(&self) -> Option<Rc<Fof_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_variable_list(&self) -> Option<Rc<Fof_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_unit_formula(&self) -> Option<Rc<Fof_unit_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_quantified_formulaContextAttrs<'input> for Fof_quantified_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_quantified_formula(&mut self,)
	-> Result<Rc<Fof_quantified_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_quantified_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 274, RULE_fof_quantified_formula);
        let mut _localctx: Rc<Fof_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_quantifier*/
			recog.base.set_state(1337);
			recog.fof_quantifier()?;

			recog.base.set_state(1338);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule fof_variable_list*/
			recog.base.set_state(1339);
			recog.fof_variable_list()?;

			recog.base.set_state(1340);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(1341);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1342);
			recog.fof_unit_formula()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_variable_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_variable_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_variable_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_variable_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_variable_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_variable_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_variable_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_variable_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_variable_list }
}
antlr4rust::tid!{Fof_variable_listContextExt<'a>}

impl<'input> Fof_variable_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_variable_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_variable_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_variable_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_variable_listContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_variable_list(&self) -> Option<Rc<Fof_variable_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_variable_listContextAttrs<'input> for Fof_variable_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_variable_list(&mut self,)
	-> Result<Rc<Fof_variable_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_variable_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 276, RULE_fof_variable_list);
        let mut _localctx: Rc<Fof_variable_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1349);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(88,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1344);
					recog.variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1345);
					recog.variable()?;

					recog.base.set_state(1346);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule fof_variable_list*/
					recog.base.set_state(1347);
					recog.fof_variable_list()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_atomic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_atomic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_atomic_formula }
}
antlr4rust::tid!{Fof_atomic_formulaContextExt<'a>}

impl<'input> Fof_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_atomic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_atomic_formulaContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_atomic_formula(&mut self,)
	-> Result<Rc<Fof_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 278, RULE_fof_atomic_formula);
        let mut _localctx: Rc<Fof_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1354);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(89,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_plain_atomic_formula*/
					recog.base.set_state(1351);
					recog.fof_plain_atomic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_defined_atomic_formula*/
					recog.base.set_state(1352);
					recog.fof_defined_atomic_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule fof_system_atomic_formula*/
					recog.base.set_state(1353);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_plain_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_plain_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_plain_atomic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_plain_atomic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_plain_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_plain_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_plain_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_plain_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_plain_atomic_formula }
}
antlr4rust::tid!{Fof_plain_atomic_formulaContextExt<'a>}

impl<'input> Fof_plain_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_plain_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_plain_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_plain_atomic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_plain_atomic_formulaContextExt<'input>>{

fn fof_plain_term(&self) -> Option<Rc<Fof_plain_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_plain_atomic_formulaContextAttrs<'input> for Fof_plain_atomic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_plain_atomic_formula(&mut self,)
	-> Result<Rc<Fof_plain_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_plain_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 280, RULE_fof_plain_atomic_formula);
        let mut _localctx: Rc<Fof_plain_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_plain_term*/
			recog.base.set_state(1356);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_defined_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_defined_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_defined_atomic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_defined_atomic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_defined_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_defined_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_atomic_formula }
}
antlr4rust::tid!{Fof_defined_atomic_formulaContextExt<'a>}

impl<'input> Fof_defined_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_atomic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_defined_atomic_formulaContextExt<'input>>{

fn fof_defined_plain_formula(&self) -> Option<Rc<Fof_defined_plain_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_defined_infix_formula(&self) -> Option<Rc<Fof_defined_infix_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_atomic_formulaContextAttrs<'input> for Fof_defined_atomic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_defined_atomic_formula(&mut self,)
	-> Result<Rc<Fof_defined_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 282, RULE_fof_defined_atomic_formula);
        let mut _localctx: Rc<Fof_defined_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1360);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(90,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_defined_plain_formula*/
					recog.base.set_state(1358);
					recog.fof_defined_plain_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_defined_infix_formula*/
					recog.base.set_state(1359);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_defined_plain_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_defined_plain_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_defined_plain_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_defined_plain_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_defined_plain_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_defined_plain_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_plain_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_plain_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_plain_formula }
}
antlr4rust::tid!{Fof_defined_plain_formulaContextExt<'a>}

impl<'input> Fof_defined_plain_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_plain_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_plain_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_plain_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_defined_plain_formulaContextExt<'input>>{

fn fof_defined_plain_term(&self) -> Option<Rc<Fof_defined_plain_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_plain_formulaContextAttrs<'input> for Fof_defined_plain_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_defined_plain_formula(&mut self,)
	-> Result<Rc<Fof_defined_plain_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_plain_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 284, RULE_fof_defined_plain_formula);
        let mut _localctx: Rc<Fof_defined_plain_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_defined_plain_term*/
			recog.base.set_state(1362);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_defined_infix_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_defined_infix_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_defined_infix_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_defined_infix_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_defined_infix_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_defined_infix_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_infix_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_infix_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_infix_formula }
}
antlr4rust::tid!{Fof_defined_infix_formulaContextExt<'a>}

impl<'input> Fof_defined_infix_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_infix_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_infix_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_infix_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_defined_infix_formulaContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_defined_infix_formula(&mut self,)
	-> Result<Rc<Fof_defined_infix_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_infix_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 286, RULE_fof_defined_infix_formula);
        let mut _localctx: Rc<Fof_defined_infix_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_term*/
			recog.base.set_state(1364);
			recog.fof_term()?;

			/*InvokeRule defined_infix_pred*/
			recog.base.set_state(1365);
			recog.defined_infix_pred()?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1366);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_system_atomic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_system_atomic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_system_atomic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_system_atomic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_system_atomic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_system_atomic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_system_atomic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_system_atomic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_system_atomic_formula }
}
antlr4rust::tid!{Fof_system_atomic_formulaContextExt<'a>}

impl<'input> Fof_system_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_system_atomic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_system_atomic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_system_atomic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_system_atomic_formulaContextExt<'input>>{

fn fof_system_term(&self) -> Option<Rc<Fof_system_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_system_atomic_formulaContextAttrs<'input> for Fof_system_atomic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_system_atomic_formula(&mut self,)
	-> Result<Rc<Fof_system_atomic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_system_atomic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 288, RULE_fof_system_atomic_formula);
        let mut _localctx: Rc<Fof_system_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_system_term*/
			recog.base.set_state(1368);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_plain_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_plain_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_plain_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_plain_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_plain_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_plain_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_plain_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_plain_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_plain_term }
}
antlr4rust::tid!{Fof_plain_termContextExt<'a>}

impl<'input> Fof_plain_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_plain_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_plain_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_plain_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_plain_termContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_plain_term(&mut self,)
	-> Result<Rc<Fof_plain_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_plain_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 290, RULE_fof_plain_term);
        let mut _localctx: Rc<Fof_plain_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1376);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(91,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule constant*/
					recog.base.set_state(1370);
					recog.constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule functor*/
					recog.base.set_state(1371);
					recog.functor()?;

					recog.base.set_state(1372);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1373);
					recog.fof_arguments()?;

					recog.base.set_state(1374);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_defined_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_defined_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_defined_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_defined_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_defined_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_defined_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_term }
}
antlr4rust::tid!{Fof_defined_termContextExt<'a>}

impl<'input> Fof_defined_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_defined_termContextExt<'input>>{

fn defined_term(&self) -> Option<Rc<Defined_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_defined_atomic_term(&self) -> Option<Rc<Fof_defined_atomic_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_termContextAttrs<'input> for Fof_defined_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_defined_term(&mut self,)
	-> Result<Rc<Fof_defined_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 292, RULE_fof_defined_term);
        let mut _localctx: Rc<Fof_defined_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1380);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Distinct_object |TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(1378);
					recog.defined_term()?;

					}
				}

			TPTP_Dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_defined_atomic_term*/
					recog.base.set_state(1379);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_defined_atomic_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_defined_atomic_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_defined_atomic_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_defined_atomic_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_defined_atomic_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_defined_atomic_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_atomic_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_atomic_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_atomic_term }
}
antlr4rust::tid!{Fof_defined_atomic_termContextExt<'a>}

impl<'input> Fof_defined_atomic_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_atomic_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_atomic_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_atomic_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_defined_atomic_termContextExt<'input>>{

fn fof_defined_plain_term(&self) -> Option<Rc<Fof_defined_plain_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_defined_atomic_termContextAttrs<'input> for Fof_defined_atomic_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_defined_atomic_term(&mut self,)
	-> Result<Rc<Fof_defined_atomic_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_atomic_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 294, RULE_fof_defined_atomic_term);
        let mut _localctx: Rc<Fof_defined_atomic_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_defined_plain_term*/
			recog.base.set_state(1382);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_defined_plain_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_defined_plain_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_defined_plain_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_defined_plain_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_defined_plain_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_defined_plain_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_defined_plain_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_defined_plain_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_defined_plain_term }
}
antlr4rust::tid!{Fof_defined_plain_termContextExt<'a>}

impl<'input> Fof_defined_plain_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_defined_plain_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_defined_plain_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_defined_plain_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_defined_plain_termContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_defined_plain_term(&mut self,)
	-> Result<Rc<Fof_defined_plain_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_defined_plain_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 296, RULE_fof_defined_plain_term);
        let mut _localctx: Rc<Fof_defined_plain_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1390);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(93,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(1384);
					recog.defined_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_functor*/
					recog.base.set_state(1385);
					recog.defined_functor()?;

					recog.base.set_state(1386);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1387);
					recog.fof_arguments()?;

					recog.base.set_state(1388);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_system_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_system_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_system_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_system_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_system_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_system_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_system_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_system_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_system_term }
}
antlr4rust::tid!{Fof_system_termContextExt<'a>}

impl<'input> Fof_system_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_system_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_system_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_system_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_system_termContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_system_term(&mut self,)
	-> Result<Rc<Fof_system_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_system_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 298, RULE_fof_system_term);
        let mut _localctx: Rc<Fof_system_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1398);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(94,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule system_constant*/
					recog.base.set_state(1392);
					recog.system_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule system_functor*/
					recog.base.set_state(1393);
					recog.system_functor()?;

					recog.base.set_state(1394);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1395);
					recog.fof_arguments()?;

					recog.base.set_state(1396);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_argumentsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_argumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_arguments(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_arguments(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_argumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_arguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_argumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_arguments }
}
antlr4rust::tid!{Fof_argumentsContextExt<'a>}

impl<'input> Fof_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_argumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_argumentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_argumentsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_argumentsContextExt<'input>>{

fn fof_term(&self) -> Option<Rc<Fof_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_arguments(&self) -> Option<Rc<Fof_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_argumentsContextAttrs<'input> for Fof_argumentsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_arguments(&mut self,)
	-> Result<Rc<Fof_argumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_argumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 300, RULE_fof_arguments);
        let mut _localctx: Rc<Fof_argumentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1405);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(95,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_term*/
					recog.base.set_state(1400);
					recog.fof_term()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_term*/
					recog.base.set_state(1401);
					recog.fof_term()?;

					recog.base.set_state(1402);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1403);
					recog.fof_arguments()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_term }
}
antlr4rust::tid!{Fof_termContextExt<'a>}

impl<'input> Fof_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_termContextExt<'input>>{

fn fof_function_term(&self) -> Option<Rc<Fof_function_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_termContextAttrs<'input> for Fof_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_term(&mut self,)
	-> Result<Rc<Fof_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 302, RULE_fof_term);
        let mut _localctx: Rc<Fof_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1409);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |TPTP_Dollar_word |
			TPTP_Dollar_dollar_word |TPTP_Lower_word |TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_function_term*/
					recog.base.set_state(1407);
					recog.fof_function_term()?;

					}
				}

			TPTP_Upper_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1408);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_function_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_function_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_function_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_function_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_function_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_function_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_function_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_function_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_function_term }
}
antlr4rust::tid!{Fof_function_termContextExt<'a>}

impl<'input> Fof_function_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_function_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_function_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_function_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_function_termContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_function_term(&mut self,)
	-> Result<Rc<Fof_function_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_function_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 304, RULE_fof_function_term);
        let mut _localctx: Rc<Fof_function_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1414);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_plain_term*/
					recog.base.set_state(1411);
					recog.fof_plain_term()?;

					}
				}

			TPTP_Distinct_object |TPTP_Dollar_word |TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_defined_term*/
					recog.base.set_state(1412);
					recog.fof_defined_term()?;

					}
				}

			TPTP_Dollar_dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule fof_system_term*/
					recog.base.set_state(1413);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_sequentContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_sequentContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_sequent(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_sequent(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_sequentContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_sequent(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_sequentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_sequent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_sequent }
}
antlr4rust::tid!{Fof_sequentContextExt<'a>}

impl<'input> Fof_sequentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_sequentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_sequentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_sequentContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_sequentContextExt<'input>>{

fn fof_formula_tuple_all(&self) ->  Vec<Rc<Fof_formula_tupleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fof_formula_tuple(&self, i: usize) -> Option<Rc<Fof_formula_tupleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn gentzen_arrow(&self) -> Option<Rc<Gentzen_arrowContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_sequent(&self) -> Option<Rc<Fof_sequentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_sequentContextAttrs<'input> for Fof_sequentContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_sequent(&mut self,)
	-> Result<Rc<Fof_sequentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_sequentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 306, RULE_fof_sequent);
        let mut _localctx: Rc<Fof_sequentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1424);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__13 |TPTP_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_formula_tuple*/
					recog.base.set_state(1416);
					recog.fof_formula_tuple()?;

					/*InvokeRule gentzen_arrow*/
					recog.base.set_state(1417);
					recog.gentzen_arrow()?;

					/*InvokeRule fof_formula_tuple*/
					recog.base.set_state(1418);
					recog.fof_formula_tuple()?;

					}
				}

			TPTP_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1420);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_sequent*/
					recog.base.set_state(1421);
					recog.fof_sequent()?;

					recog.base.set_state(1422);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_formula_tupleContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_formula_tupleContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_formula_tuple(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_formula_tuple(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_formula_tupleContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_formula_tuple(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_formula_tupleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_formula_tuple }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_formula_tuple }
}
antlr4rust::tid!{Fof_formula_tupleContextExt<'a>}

impl<'input> Fof_formula_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_formula_tupleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_formula_tupleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_formula_tupleContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_formula_tupleContextExt<'input>>{

fn fof_formula_tuple_list(&self) -> Option<Rc<Fof_formula_tuple_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_formula_tupleContextAttrs<'input> for Fof_formula_tupleContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_formula_tuple(&mut self,)
	-> Result<Rc<Fof_formula_tupleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_formula_tupleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 308, RULE_fof_formula_tuple);
        let mut _localctx: Rc<Fof_formula_tupleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1431);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1426);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1427);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule fof_formula_tuple_list*/
					recog.base.set_state(1428);
					recog.fof_formula_tuple_list()?;

					recog.base.set_state(1429);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_formula_tuple_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_formula_tuple_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_formula_tuple_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_formula_tuple_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_formula_tuple_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_formula_tuple_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_formula_tuple_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_formula_tuple_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_formula_tuple_list }
}
antlr4rust::tid!{Fof_formula_tuple_listContextExt<'a>}

impl<'input> Fof_formula_tuple_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_formula_tuple_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_formula_tuple_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_formula_tuple_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_formula_tuple_listContextExt<'input>>{

fn fof_logic_formula(&self) -> Option<Rc<Fof_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comma_fof_logic_formula_all(&self) ->  Vec<Rc<Comma_fof_logic_formulaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comma_fof_logic_formula(&self, i: usize) -> Option<Rc<Comma_fof_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Fof_formula_tuple_listContextAttrs<'input> for Fof_formula_tuple_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_formula_tuple_list(&mut self,)
	-> Result<Rc<Fof_formula_tuple_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_formula_tuple_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 310, RULE_fof_formula_tuple_list);
        let mut _localctx: Rc<Fof_formula_tuple_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_logic_formula*/
			recog.base.set_state(1433);
			recog.fof_logic_formula()?;

			recog.base.set_state(1437);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_fof_logic_formula*/
				recog.base.set_state(1434);
				recog.comma_fof_logic_formula()?;

				}
				}
				recog.base.set_state(1439);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- comma_fof_logic_formula ----------------
pub type Comma_fof_logic_formulaContextAll<'input> = Comma_fof_logic_formulaContext<'input>;


pub type Comma_fof_logic_formulaContext<'input> = BaseParserRuleContext<'input,Comma_fof_logic_formulaContextExt<'input>>;

#[derive(Clone)]
pub struct Comma_fof_logic_formulaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Comma_fof_logic_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Comma_fof_logic_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_comma_fof_logic_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_comma_fof_logic_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Comma_fof_logic_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_comma_fof_logic_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Comma_fof_logic_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comma_fof_logic_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comma_fof_logic_formula }
}
antlr4rust::tid!{Comma_fof_logic_formulaContextExt<'a>}

impl<'input> Comma_fof_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Comma_fof_logic_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Comma_fof_logic_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Comma_fof_logic_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Comma_fof_logic_formulaContextExt<'input>>{

fn fof_logic_formula(&self) -> Option<Rc<Fof_logic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Comma_fof_logic_formulaContextAttrs<'input> for Comma_fof_logic_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comma_fof_logic_formula(&mut self,)
	-> Result<Rc<Comma_fof_logic_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Comma_fof_logic_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 312, RULE_comma_fof_logic_formula);
        let mut _localctx: Rc<Comma_fof_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1440);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_logic_formula*/
			recog.base.set_state(1441);
			recog.fof_logic_formula()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Cnf_formulaContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Cnf_formulaContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_cnf_formula(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_cnf_formula(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Cnf_formulaContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_cnf_formula(self);
	}
}

impl<'input> CustomRuleContext<'input> for Cnf_formulaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cnf_formula }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cnf_formula }
}
antlr4rust::tid!{Cnf_formulaContextExt<'a>}

impl<'input> Cnf_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Cnf_formulaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Cnf_formulaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Cnf_formulaContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Cnf_formulaContextExt<'input>>{

fn cnf_disjunction(&self) -> Option<Rc<Cnf_disjunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cnf_formula(&self) -> Option<Rc<Cnf_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Cnf_formulaContextAttrs<'input> for Cnf_formulaContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn cnf_formula(&mut self,)
	-> Result<Rc<Cnf_formulaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Cnf_formulaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 314, RULE_cnf_formula);
        let mut _localctx: Rc<Cnf_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1448);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__26 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |
			TPTP_Dollar_word |TPTP_Dollar_dollar_word |TPTP_Upper_word |TPTP_Lower_word |
			TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule cnf_disjunction*/
					recog.base.set_state(1443);
					recog.cnf_disjunction_rec(0)?;

					}
				}

			TPTP_T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1444);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule cnf_formula*/
					recog.base.set_state(1445);
					recog.cnf_formula()?;

					recog.base.set_state(1446);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Cnf_disjunctionContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Cnf_disjunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_cnf_disjunction(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_cnf_disjunction(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Cnf_disjunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_cnf_disjunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for Cnf_disjunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cnf_disjunction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cnf_disjunction }
}
antlr4rust::tid!{Cnf_disjunctionContextExt<'a>}

impl<'input> Cnf_disjunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Cnf_disjunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Cnf_disjunctionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Cnf_disjunctionContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Cnf_disjunctionContextExt<'input>>{

fn cnf_literal(&self) -> Option<Rc<Cnf_literalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn cnf_disjunction(&self) -> Option<Rc<Cnf_disjunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Vline
/// Returns `None` if there is no child corresponding to token Vline
fn Vline(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Vline, 0)
}

}

impl<'input> Cnf_disjunctionContextAttrs<'input> for Cnf_disjunctionContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
		recog.base.enter_recursion_rule(_localctx.clone(), 316, RULE_cnf_disjunction, _p);
	    let mut _localctx: Rc<Cnf_disjunctionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 316;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule cnf_literal*/
			recog.base.set_state(1451);
			recog.cnf_literal()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1458);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(102,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Cnf_disjunctionContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_cnf_disjunction)?;
					_localctx = tmp;
					recog.base.set_state(1453);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1454);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					/*InvokeRule cnf_literal*/
					recog.base.set_state(1455);
					recog.cnf_literal()?;

					}
					} 
				}
				recog.base.set_state(1460);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(102,&mut recog.base)?;
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
		recog.base.unroll_recursion_context(_parentctx)?;

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

impl<'input> TPTPParserContext<'input> for Cnf_literalContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Cnf_literalContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_cnf_literal(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_cnf_literal(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Cnf_literalContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_cnf_literal(self);
	}
}

impl<'input> CustomRuleContext<'input> for Cnf_literalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cnf_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cnf_literal }
}
antlr4rust::tid!{Cnf_literalContextExt<'a>}

impl<'input> Cnf_literalContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Cnf_literalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Cnf_literalContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Cnf_literalContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Cnf_literalContextExt<'input>>{

fn fof_atomic_formula(&self) -> Option<Rc<Fof_atomic_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_infix_unary(&self) -> Option<Rc<Fof_infix_unaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Cnf_literalContextAttrs<'input> for Cnf_literalContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn cnf_literal(&mut self,)
	-> Result<Rc<Cnf_literalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Cnf_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 318, RULE_cnf_literal);
        let mut _localctx: Rc<Cnf_literalContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1470);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(103,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1461);
					recog.fof_atomic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1462);
					recog.base.match_token(TPTP_T__26,&mut recog.err_handler)?;

					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1463);
					recog.fof_atomic_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1464);
					recog.base.match_token(TPTP_T__26,&mut recog.err_handler)?;

					recog.base.set_state(1465);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1466);
					recog.fof_atomic_formula()?;

					recog.base.set_state(1467);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule fof_infix_unary*/
					recog.base.set_state(1469);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_quantifierContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_quantifierContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_quantifier(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_quantifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_quantifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_quantifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_quantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_quantifier }
}
antlr4rust::tid!{Thf_quantifierContextExt<'a>}

impl<'input> Thf_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_quantifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_quantifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_quantifierContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_quantifierContextExt<'input>>{

fn tff_quantifier(&self) -> Option<Rc<Tff_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn th0_quantifier(&self) -> Option<Rc<Th0_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type_quantifier(&self) -> Option<Rc<Type_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_quantifierContextAttrs<'input> for Thf_quantifierContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_quantifier(&mut self,)
	-> Result<Rc<Thf_quantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_quantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 320, RULE_thf_quantifier);
        let mut _localctx: Rc<Thf_quantifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1475);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__25 |TPTP_T__33 |TPTP_Hash 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_quantifier*/
					recog.base.set_state(1472);
					recog.tff_quantifier()?;

					}
				}

			TPTP_T__27 |TPTP_T__28 |TPTP_T__29 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule th0_quantifier*/
					recog.base.set_state(1473);
					recog.th0_quantifier()?;

					}
				}

			TPTP_T__30 |TPTP_T__31 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule type_quantifier*/
					recog.base.set_state(1474);
					recog.type_quantifier()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Thf_unary_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_unary_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_unary_connective(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_unary_connective(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_unary_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_unary_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_unary_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_unary_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_unary_connective }
}
antlr4rust::tid!{Thf_unary_connectiveContextExt<'a>}

impl<'input> Thf_unary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_unary_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_unary_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_unary_connectiveContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_unary_connectiveContextExt<'input>>{

fn unary_connective(&self) -> Option<Rc<Unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ntf_short_connective(&self) -> Option<Rc<Ntf_short_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Thf_unary_connectiveContextAttrs<'input> for Thf_unary_connectiveContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_unary_connective(&mut self,)
	-> Result<Rc<Thf_unary_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_unary_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 322, RULE_thf_unary_connective);
        let mut _localctx: Rc<Thf_unary_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1479);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__26 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(1477);
					recog.unary_connective()?;

					}
				}

			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_Less_sign 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule ntf_short_connective*/
					recog.base.set_state(1478);
					recog.ntf_short_connective()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Th0_quantifierContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Th0_quantifierContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_th0_quantifier(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_th0_quantifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Th0_quantifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_th0_quantifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Th0_quantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_th0_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_th0_quantifier }
}
antlr4rust::tid!{Th0_quantifierContextExt<'a>}

impl<'input> Th0_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Th0_quantifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Th0_quantifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Th0_quantifierContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Th0_quantifierContextExt<'input>>{


}

impl<'input> Th0_quantifierContextAttrs<'input> for Th0_quantifierContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn th0_quantifier(&mut self,)
	-> Result<Rc<Th0_quantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Th0_quantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 324, RULE_th0_quantifier);
        let mut _localctx: Rc<Th0_quantifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1481);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 1879048192) != 0)) } {
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- type_quantifier ----------------
pub type Type_quantifierContextAll<'input> = Type_quantifierContext<'input>;


pub type Type_quantifierContext<'input> = BaseParserRuleContext<'input,Type_quantifierContextExt<'input>>;

#[derive(Clone)]
pub struct Type_quantifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Type_quantifierContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Type_quantifierContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_type_quantifier(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_type_quantifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Type_quantifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_type_quantifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_quantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_quantifier }
}
antlr4rust::tid!{Type_quantifierContextExt<'a>}

impl<'input> Type_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Type_quantifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_quantifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Type_quantifierContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Type_quantifierContextExt<'input>>{


}

impl<'input> Type_quantifierContextAttrs<'input> for Type_quantifierContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn type_quantifier(&mut self,)
	-> Result<Rc<Type_quantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_quantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 326, RULE_type_quantifier);
        let mut _localctx: Rc<Type_quantifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1483);
			_la = recog.base.input.la(1);
			if { !(_la==TPTP_T__30 || _la==TPTP_T__31) } {
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- subtype_sign ----------------
pub type Subtype_signContextAll<'input> = Subtype_signContext<'input>;


pub type Subtype_signContext<'input> = BaseParserRuleContext<'input,Subtype_signContextExt<'input>>;

#[derive(Clone)]
pub struct Subtype_signContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Subtype_signContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Subtype_signContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_subtype_sign(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_subtype_sign(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Subtype_signContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_subtype_sign(self);
	}
}

impl<'input> CustomRuleContext<'input> for Subtype_signContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subtype_sign }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subtype_sign }
}
antlr4rust::tid!{Subtype_signContextExt<'a>}

impl<'input> Subtype_signContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Subtype_signContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Subtype_signContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Subtype_signContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Subtype_signContextExt<'input>>{


}

impl<'input> Subtype_signContextAttrs<'input> for Subtype_signContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn subtype_sign(&mut self,)
	-> Result<Rc<Subtype_signContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Subtype_signContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 328, RULE_subtype_sign);
        let mut _localctx: Rc<Subtype_signContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1485);
			recog.base.match_token(TPTP_T__32,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_unary_connective ----------------
pub type Tff_unary_connectiveContextAll<'input> = Tff_unary_connectiveContext<'input>;


pub type Tff_unary_connectiveContext<'input> = BaseParserRuleContext<'input,Tff_unary_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_unary_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_unary_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_unary_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_unary_connective(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_unary_connective(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_unary_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_unary_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_unary_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_unary_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_unary_connective }
}
antlr4rust::tid!{Tff_unary_connectiveContextExt<'a>}

impl<'input> Tff_unary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_unary_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_unary_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_unary_connectiveContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_unary_connectiveContextExt<'input>>{

fn unary_connective(&self) -> Option<Rc<Unary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ntf_short_connective(&self) -> Option<Rc<Ntf_short_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_unary_connectiveContextAttrs<'input> for Tff_unary_connectiveContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_unary_connective(&mut self,)
	-> Result<Rc<Tff_unary_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_unary_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 330, RULE_tff_unary_connective);
        let mut _localctx: Rc<Tff_unary_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1489);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__26 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(1487);
					recog.unary_connective()?;

					}
				}

			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_Less_sign 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule ntf_short_connective*/
					recog.base.set_state(1488);
					recog.ntf_short_connective()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tff_quantifier ----------------
pub type Tff_quantifierContextAll<'input> = Tff_quantifierContext<'input>;


pub type Tff_quantifierContext<'input> = BaseParserRuleContext<'input,Tff_quantifierContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_quantifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_quantifierContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_quantifierContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_quantifier(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_quantifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_quantifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_quantifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_quantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_quantifier }
}
antlr4rust::tid!{Tff_quantifierContextExt<'a>}

impl<'input> Tff_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_quantifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_quantifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_quantifierContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_quantifierContextExt<'input>>{

fn fof_quantifier(&self) -> Option<Rc<Fof_quantifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Hash
/// Returns `None` if there is no child corresponding to token Hash
fn Hash(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Hash, 0)
}

}

impl<'input> Tff_quantifierContextAttrs<'input> for Tff_quantifierContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_quantifier(&mut self,)
	-> Result<Rc<Tff_quantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_quantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 332, RULE_tff_quantifier);
        let mut _localctx: Rc<Tff_quantifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1493);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__25 |TPTP_T__33 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_quantifier*/
					recog.base.set_state(1491);
					recog.fof_quantifier()?;

					}
				}

			TPTP_Hash 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1492);
					recog.base.match_token(TPTP_Hash,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Fof_quantifierContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_quantifierContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_quantifier(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_quantifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_quantifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_quantifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_quantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_quantifier }
}
antlr4rust::tid!{Fof_quantifierContextExt<'a>}

impl<'input> Fof_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_quantifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_quantifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_quantifierContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_quantifierContextExt<'input>>{


}

impl<'input> Fof_quantifierContextAttrs<'input> for Fof_quantifierContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_quantifier(&mut self,)
	-> Result<Rc<Fof_quantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_quantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 334, RULE_fof_quantifier);
        let mut _localctx: Rc<Fof_quantifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1495);
			_la = recog.base.input.la(1);
			if { !(_la==TPTP_T__25 || _la==TPTP_T__33) } {
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nonassoc_connective ----------------
pub type Nonassoc_connectiveContextAll<'input> = Nonassoc_connectiveContext<'input>;


pub type Nonassoc_connectiveContext<'input> = BaseParserRuleContext<'input,Nonassoc_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Nonassoc_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Nonassoc_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Nonassoc_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nonassoc_connective(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nonassoc_connective(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Nonassoc_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nonassoc_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Nonassoc_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nonassoc_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nonassoc_connective }
}
antlr4rust::tid!{Nonassoc_connectiveContextExt<'a>}

impl<'input> Nonassoc_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Nonassoc_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Nonassoc_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Nonassoc_connectiveContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Nonassoc_connectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Vline
/// Returns `None` if there is no child corresponding to token Vline
fn Vline(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Vline, 0)
}

}

impl<'input> Nonassoc_connectiveContextAttrs<'input> for Nonassoc_connectiveContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nonassoc_connective(&mut self,)
	-> Result<Rc<Nonassoc_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Nonassoc_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 336, RULE_nonassoc_connective);
        let mut _localctx: Rc<Nonassoc_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1504);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__34 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1497);
					recog.base.match_token(TPTP_T__34,&mut recog.err_handler)?;

					}
				}

			TPTP_T__35 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1498);
					recog.base.match_token(TPTP_T__35,&mut recog.err_handler)?;

					}
				}

			TPTP_T__36 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1499);
					recog.base.match_token(TPTP_T__36,&mut recog.err_handler)?;

					}
				}

			TPTP_T__37 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(1500);
					recog.base.match_token(TPTP_T__37,&mut recog.err_handler)?;

					}
				}

			TPTP_T__26 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1501);
					recog.base.match_token(TPTP_T__26,&mut recog.err_handler)?;

					recog.base.set_state(1502);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					}
				}

			TPTP_T__38 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(1503);
					recog.base.match_token(TPTP_T__38,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Assoc_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Assoc_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_assoc_connective(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_assoc_connective(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Assoc_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_assoc_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Assoc_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assoc_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assoc_connective }
}
antlr4rust::tid!{Assoc_connectiveContextExt<'a>}

impl<'input> Assoc_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Assoc_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Assoc_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Assoc_connectiveContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Assoc_connectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Vline
/// Returns `None` if there is no child corresponding to token Vline
fn Vline(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Vline, 0)
}

}

impl<'input> Assoc_connectiveContextAttrs<'input> for Assoc_connectiveContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn assoc_connective(&mut self,)
	-> Result<Rc<Assoc_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Assoc_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 338, RULE_assoc_connective);
        let mut _localctx: Rc<Assoc_connectiveContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1506);
			_la = recog.base.input.la(1);
			if { !(_la==TPTP_T__9 || _la==TPTP_Vline) } {
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Unary_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Unary_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_unary_connective(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_unary_connective(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Unary_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_unary_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Unary_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_connective }
}
antlr4rust::tid!{Unary_connectiveContextExt<'a>}

impl<'input> Unary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Unary_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Unary_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Unary_connectiveContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Unary_connectiveContextExt<'input>>{


}

impl<'input> Unary_connectiveContextAttrs<'input> for Unary_connectiveContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn unary_connective(&mut self,)
	-> Result<Rc<Unary_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Unary_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 340, RULE_unary_connective);
        let mut _localctx: Rc<Unary_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1508);
			recog.base.match_token(TPTP_T__26,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- gentzen_arrow ----------------
pub type Gentzen_arrowContextAll<'input> = Gentzen_arrowContext<'input>;


pub type Gentzen_arrowContext<'input> = BaseParserRuleContext<'input,Gentzen_arrowContextExt<'input>>;

#[derive(Clone)]
pub struct Gentzen_arrowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Gentzen_arrowContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Gentzen_arrowContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_gentzen_arrow(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_gentzen_arrow(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Gentzen_arrowContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_gentzen_arrow(self);
	}
}

impl<'input> CustomRuleContext<'input> for Gentzen_arrowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gentzen_arrow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gentzen_arrow }
}
antlr4rust::tid!{Gentzen_arrowContextExt<'a>}

impl<'input> Gentzen_arrowContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Gentzen_arrowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Gentzen_arrowContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Gentzen_arrowContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Gentzen_arrowContextExt<'input>>{


}

impl<'input> Gentzen_arrowContextAttrs<'input> for Gentzen_arrowContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn gentzen_arrow(&mut self,)
	-> Result<Rc<Gentzen_arrowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Gentzen_arrowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 342, RULE_gentzen_arrow);
        let mut _localctx: Rc<Gentzen_arrowContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1510);
			recog.base.match_token(TPTP_T__39,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- assignment ----------------
pub type AssignmentContextAll<'input> = AssignmentContext<'input>;


pub type AssignmentContext<'input> = BaseParserRuleContext<'input,AssignmentContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for AssignmentContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for AssignmentContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_assignment(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_assignment(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for AssignmentContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_assignment(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignment }
}
antlr4rust::tid!{AssignmentContextExt<'a>}

impl<'input> AssignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AssignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<AssignmentContextExt<'input>>{


}

impl<'input> AssignmentContextAttrs<'input> for AssignmentContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn assignment(&mut self,)
	-> Result<Rc<AssignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 344, RULE_assignment);
        let mut _localctx: Rc<AssignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1512);
			recog.base.match_token(TPTP_T__40,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- identical ----------------
pub type IdenticalContextAll<'input> = IdenticalContext<'input>;


pub type IdenticalContext<'input> = BaseParserRuleContext<'input,IdenticalContextExt<'input>>;

#[derive(Clone)]
pub struct IdenticalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for IdenticalContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for IdenticalContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_identical(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_identical(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for IdenticalContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_identical(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdenticalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identical }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identical }
}
antlr4rust::tid!{IdenticalContextExt<'a>}

impl<'input> IdenticalContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IdenticalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdenticalContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IdenticalContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<IdenticalContextExt<'input>>{


}

impl<'input> IdenticalContextAttrs<'input> for IdenticalContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn identical(&mut self,)
	-> Result<Rc<IdenticalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdenticalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 346, RULE_identical);
        let mut _localctx: Rc<IdenticalContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1514);
			recog.base.match_token(TPTP_T__41,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Type_constantContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Type_constantContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_type_constant(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_type_constant(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Type_constantContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_type_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_constantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_constant }
}
antlr4rust::tid!{Type_constantContextExt<'a>}

impl<'input> Type_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Type_constantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_constantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Type_constantContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Type_constantContextExt<'input>>{

fn type_functor(&self) -> Option<Rc<Type_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Type_constantContextAttrs<'input> for Type_constantContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn type_constant(&mut self,)
	-> Result<Rc<Type_constantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_constantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 348, RULE_type_constant);
        let mut _localctx: Rc<Type_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule type_functor*/
			recog.base.set_state(1516);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Type_functorContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Type_functorContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_type_functor(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_type_functor(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Type_functorContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_type_functor(self);
	}
}

impl<'input> CustomRuleContext<'input> for Type_functorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_functor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_functor }
}
antlr4rust::tid!{Type_functorContextExt<'a>}

impl<'input> Type_functorContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Type_functorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_functorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Type_functorContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Type_functorContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Type_functorContextAttrs<'input> for Type_functorContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn type_functor(&mut self,)
	-> Result<Rc<Type_functorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_functorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 350, RULE_type_functor);
        let mut _localctx: Rc<Type_functorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1518);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Defined_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Defined_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_defined_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_defined_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Defined_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_defined_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_type }
}
antlr4rust::tid!{Defined_typeContextExt<'a>}

impl<'input> Defined_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Defined_typeContextExt<'input>>{

fn atomic_defined_word(&self) -> Option<Rc<Atomic_defined_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Defined_typeContextAttrs<'input> for Defined_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn defined_type(&mut self,)
	-> Result<Rc<Defined_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 352, RULE_defined_type);
        let mut _localctx: Rc<Defined_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_defined_word*/
			recog.base.set_state(1520);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for AtomContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for AtomContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atom(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atom(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for AtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}
antlr4rust::tid!{AtomContextExt<'a>}

impl<'input> AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AtomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AtomContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<AtomContextExt<'input>>{

fn untyped_atom(&self) -> Option<Rc<Untyped_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_constant(&self) -> Option<Rc<Defined_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atom(&mut self,)
	-> Result<Rc<AtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 354, RULE_atom);
        let mut _localctx: Rc<AtomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1524);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Dollar_dollar_word |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule untyped_atom*/
					recog.base.set_state(1522);
					recog.untyped_atom()?;

					}
				}

			TPTP_Dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(1523);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Untyped_atomContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Untyped_atomContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_untyped_atom(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_untyped_atom(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Untyped_atomContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_untyped_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for Untyped_atomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_untyped_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_untyped_atom }
}
antlr4rust::tid!{Untyped_atomContextExt<'a>}

impl<'input> Untyped_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Untyped_atomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Untyped_atomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Untyped_atomContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Untyped_atomContextExt<'input>>{

fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn system_constant(&self) -> Option<Rc<System_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Untyped_atomContextAttrs<'input> for Untyped_atomContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn untyped_atom(&mut self,)
	-> Result<Rc<Untyped_atomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Untyped_atomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 356, RULE_untyped_atom);
        let mut _localctx: Rc<Untyped_atomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1528);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule constant*/
					recog.base.set_state(1526);
					recog.constant()?;

					}
				}

			TPTP_Dollar_dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule system_constant*/
					recog.base.set_state(1527);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Defined_infix_predContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Defined_infix_predContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_defined_infix_pred(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_defined_infix_pred(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Defined_infix_predContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_defined_infix_pred(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_infix_predContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_infix_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_infix_pred }
}
antlr4rust::tid!{Defined_infix_predContextExt<'a>}

impl<'input> Defined_infix_predContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_infix_predContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_infix_predContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_infix_predContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Defined_infix_predContextExt<'input>>{

fn infix_equality(&self) -> Option<Rc<Infix_equalityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Defined_infix_predContextAttrs<'input> for Defined_infix_predContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn defined_infix_pred(&mut self,)
	-> Result<Rc<Defined_infix_predContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_infix_predContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 358, RULE_defined_infix_pred);
        let mut _localctx: Rc<Defined_infix_predContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule infix_equality*/
			recog.base.set_state(1530);
			recog.infix_equality()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- infix_equality ----------------
pub type Infix_equalityContextAll<'input> = Infix_equalityContext<'input>;


pub type Infix_equalityContext<'input> = BaseParserRuleContext<'input,Infix_equalityContextExt<'input>>;

#[derive(Clone)]
pub struct Infix_equalityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Infix_equalityContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Infix_equalityContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_infix_equality(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_infix_equality(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Infix_equalityContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_infix_equality(self);
	}
}

impl<'input> CustomRuleContext<'input> for Infix_equalityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_infix_equality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_infix_equality }
}
antlr4rust::tid!{Infix_equalityContextExt<'a>}

impl<'input> Infix_equalityContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Infix_equalityContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Infix_equalityContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Infix_equalityContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Infix_equalityContextExt<'input>>{


}

impl<'input> Infix_equalityContextAttrs<'input> for Infix_equalityContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn infix_equality(&mut self,)
	-> Result<Rc<Infix_equalityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Infix_equalityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 360, RULE_infix_equality);
        let mut _localctx: Rc<Infix_equalityContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1532);
			recog.base.match_token(TPTP_T__42,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- infix_inequality ----------------
pub type Infix_inequalityContextAll<'input> = Infix_inequalityContext<'input>;


pub type Infix_inequalityContext<'input> = BaseParserRuleContext<'input,Infix_inequalityContextExt<'input>>;

#[derive(Clone)]
pub struct Infix_inequalityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Infix_inequalityContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Infix_inequalityContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_infix_inequality(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_infix_inequality(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Infix_inequalityContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_infix_inequality(self);
	}
}

impl<'input> CustomRuleContext<'input> for Infix_inequalityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_infix_inequality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_infix_inequality }
}
antlr4rust::tid!{Infix_inequalityContextExt<'a>}

impl<'input> Infix_inequalityContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Infix_inequalityContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Infix_inequalityContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Infix_inequalityContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Infix_inequalityContextExt<'input>>{


}

impl<'input> Infix_inequalityContextAttrs<'input> for Infix_inequalityContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn infix_inequality(&mut self,)
	-> Result<Rc<Infix_inequalityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Infix_inequalityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 362, RULE_infix_inequality);
        let mut _localctx: Rc<Infix_inequalityContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1534);
			recog.base.match_token(TPTP_T__43,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for ConstantContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for ConstantContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_constant(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_constant(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for ConstantContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}
antlr4rust::tid!{ConstantContextExt<'a>}

impl<'input> ConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ConstantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ConstantContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<ConstantContextExt<'input>>{

fn functor(&self) -> Option<Rc<FunctorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConstantContextAttrs<'input> for ConstantContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn constant(&mut self,)
	-> Result<Rc<ConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 364, RULE_constant);
        let mut _localctx: Rc<ConstantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule functor*/
			recog.base.set_state(1536);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for FunctorContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for FunctorContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_functor(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_functor(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for FunctorContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_functor(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functor }
}
antlr4rust::tid!{FunctorContextExt<'a>}

impl<'input> FunctorContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FunctorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FunctorContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<FunctorContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctorContextAttrs<'input> for FunctorContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn functor(&mut self,)
	-> Result<Rc<FunctorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 366, RULE_functor);
        let mut _localctx: Rc<FunctorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1538);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Defined_constantContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Defined_constantContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_defined_constant(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_defined_constant(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Defined_constantContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_defined_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_constantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_constant }
}
antlr4rust::tid!{Defined_constantContextExt<'a>}

impl<'input> Defined_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_constantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_constantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_constantContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Defined_constantContextExt<'input>>{

fn defined_functor(&self) -> Option<Rc<Defined_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Defined_constantContextAttrs<'input> for Defined_constantContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn defined_constant(&mut self,)
	-> Result<Rc<Defined_constantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_constantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 368, RULE_defined_constant);
        let mut _localctx: Rc<Defined_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule defined_functor*/
			recog.base.set_state(1540);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Defined_functorContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Defined_functorContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_defined_functor(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_defined_functor(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Defined_functorContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_defined_functor(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_functorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_functor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_functor }
}
antlr4rust::tid!{Defined_functorContextExt<'a>}

impl<'input> Defined_functorContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_functorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_functorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_functorContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Defined_functorContextExt<'input>>{

fn atomic_defined_word(&self) -> Option<Rc<Atomic_defined_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Defined_functorContextAttrs<'input> for Defined_functorContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn defined_functor(&mut self,)
	-> Result<Rc<Defined_functorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_functorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 370, RULE_defined_functor);
        let mut _localctx: Rc<Defined_functorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_defined_word*/
			recog.base.set_state(1542);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for System_constantContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for System_constantContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_system_constant(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_system_constant(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for System_constantContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_system_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for System_constantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_system_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_system_constant }
}
antlr4rust::tid!{System_constantContextExt<'a>}

impl<'input> System_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<System_constantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,System_constantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait System_constantContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<System_constantContextExt<'input>>{

fn system_functor(&self) -> Option<Rc<System_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> System_constantContextAttrs<'input> for System_constantContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn system_constant(&mut self,)
	-> Result<Rc<System_constantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = System_constantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 372, RULE_system_constant);
        let mut _localctx: Rc<System_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule system_functor*/
			recog.base.set_state(1544);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for System_functorContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for System_functorContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_system_functor(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_system_functor(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for System_functorContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_system_functor(self);
	}
}

impl<'input> CustomRuleContext<'input> for System_functorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_system_functor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_system_functor }
}
antlr4rust::tid!{System_functorContextExt<'a>}

impl<'input> System_functorContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<System_functorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,System_functorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait System_functorContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<System_functorContextExt<'input>>{

fn atomic_system_word(&self) -> Option<Rc<Atomic_system_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> System_functorContextAttrs<'input> for System_functorContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn system_functor(&mut self,)
	-> Result<Rc<System_functorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = System_functorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 374, RULE_system_functor);
        let mut _localctx: Rc<System_functorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_system_word*/
			recog.base.set_state(1546);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- def_or_sys_constant ----------------
pub type Def_or_sys_constantContextAll<'input> = Def_or_sys_constantContext<'input>;


pub type Def_or_sys_constantContext<'input> = BaseParserRuleContext<'input,Def_or_sys_constantContextExt<'input>>;

#[derive(Clone)]
pub struct Def_or_sys_constantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Def_or_sys_constantContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Def_or_sys_constantContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_def_or_sys_constant(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_def_or_sys_constant(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Def_or_sys_constantContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_def_or_sys_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for Def_or_sys_constantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_def_or_sys_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_def_or_sys_constant }
}
antlr4rust::tid!{Def_or_sys_constantContextExt<'a>}

impl<'input> Def_or_sys_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Def_or_sys_constantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Def_or_sys_constantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Def_or_sys_constantContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Def_or_sys_constantContextExt<'input>>{

fn defined_constant(&self) -> Option<Rc<Defined_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn system_constant(&self) -> Option<Rc<System_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Def_or_sys_constantContextAttrs<'input> for Def_or_sys_constantContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn def_or_sys_constant(&mut self,)
	-> Result<Rc<Def_or_sys_constantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Def_or_sys_constantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 376, RULE_def_or_sys_constant);
        let mut _localctx: Rc<Def_or_sys_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1550);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(1548);
					recog.defined_constant()?;

					}
				}

			TPTP_Dollar_dollar_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule system_constant*/
					recog.base.set_state(1549);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- th1_defined_term ----------------
pub type Th1_defined_termContextAll<'input> = Th1_defined_termContext<'input>;


pub type Th1_defined_termContext<'input> = BaseParserRuleContext<'input,Th1_defined_termContextExt<'input>>;

#[derive(Clone)]
pub struct Th1_defined_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Th1_defined_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Th1_defined_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_th1_defined_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_th1_defined_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Th1_defined_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_th1_defined_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Th1_defined_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_th1_defined_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_th1_defined_term }
}
antlr4rust::tid!{Th1_defined_termContextExt<'a>}

impl<'input> Th1_defined_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Th1_defined_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Th1_defined_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Th1_defined_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Th1_defined_termContextExt<'input>>{


}

impl<'input> Th1_defined_termContextAttrs<'input> for Th1_defined_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn th1_defined_term(&mut self,)
	-> Result<Rc<Th1_defined_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Th1_defined_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 378, RULE_th1_defined_term);
        let mut _localctx: Rc<Th1_defined_termContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1552);
			_la = recog.base.input.la(1);
			if { !(((((_la - 45)) & !0x3f) == 0 && ((1usize << (_la - 45)) & 31) != 0)) } {
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Defined_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Defined_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_defined_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_defined_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Defined_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_defined_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Defined_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defined_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defined_term }
}
antlr4rust::tid!{Defined_termContextExt<'a>}

impl<'input> Defined_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Defined_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Defined_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Defined_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Defined_termContextExt<'input>>{

fn number(&self) -> Option<Rc<NumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Distinct_object
/// Returns `None` if there is no child corresponding to token Distinct_object
fn Distinct_object(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Distinct_object, 0)
}

}

impl<'input> Defined_termContextAttrs<'input> for Defined_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn defined_term(&mut self,)
	-> Result<Rc<Defined_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Defined_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 380, RULE_defined_term);
        let mut _localctx: Rc<Defined_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1556);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Real |TPTP_Rational |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule number*/
					recog.base.set_state(1554);
					recog.number()?;

					}
				}

			TPTP_Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1555);
					recog.base.match_token(TPTP_Distinct_object,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for VariableContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_variable(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_variable(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr4rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait VariableContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<VariableContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Upper_word
/// Returns `None` if there is no child corresponding to token Upper_word
fn Upper_word(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Upper_word, 0)
}

}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn variable(&mut self,)
	-> Result<Rc<VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 382, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1558);
			recog.base.match_token(TPTP_Upper_word,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for SourceContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for SourceContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_source(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_source(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for SourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for SourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_source }
}
antlr4rust::tid!{SourceContextExt<'a>}

impl<'input> SourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SourceContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<SourceContextExt<'input>>{

fn dag_source(&self) -> Option<Rc<Dag_sourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn internal_source(&self) -> Option<Rc<Internal_sourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn external_source(&self) -> Option<Rc<External_sourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn sources(&self) -> Option<Rc<SourcesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SourceContextAttrs<'input> for SourceContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn source(&mut self,)
	-> Result<Rc<SourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 384, RULE_source);
        let mut _localctx: Rc<SourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1568);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__50 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule dag_source*/
					recog.base.set_state(1560);
					recog.dag_source()?;

					}
				}

			TPTP_T__51 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule internal_source*/
					recog.base.set_state(1561);
					recog.internal_source()?;

					}
				}

			TPTP_T__52 |TPTP_T__53 |TPTP_T__54 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule external_source*/
					recog.base.set_state(1562);
					recog.external_source()?;

					}
				}

			TPTP_T__49 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(1563);
					recog.base.match_token(TPTP_T__49,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1564);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule sources*/
					recog.base.set_state(1565);
					recog.sources()?;

					recog.base.set_state(1566);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for SourcesContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for SourcesContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_sources(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_sources(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for SourcesContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_sources(self);
	}
}

impl<'input> CustomRuleContext<'input> for SourcesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sources }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sources }
}
antlr4rust::tid!{SourcesContextExt<'a>}

impl<'input> SourcesContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SourcesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SourcesContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SourcesContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<SourcesContextExt<'input>>{

fn source(&self) -> Option<Rc<SourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn sources(&self) -> Option<Rc<SourcesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SourcesContextAttrs<'input> for SourcesContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn sources(&mut self,)
	-> Result<Rc<SourcesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SourcesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 386, RULE_sources);
        let mut _localctx: Rc<SourcesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1575);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(114,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule source*/
					recog.base.set_state(1570);
					recog.source()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule source*/
					recog.base.set_state(1571);
					recog.source()?;

					recog.base.set_state(1572);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule sources*/
					recog.base.set_state(1573);
					recog.sources()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Dag_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Dag_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_dag_source(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_dag_source(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Dag_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_dag_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for Dag_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dag_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dag_source }
}
antlr4rust::tid!{Dag_sourceContextExt<'a>}

impl<'input> Dag_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Dag_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Dag_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Dag_sourceContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Dag_sourceContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inference_record(&self) -> Option<Rc<Inference_recordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Dag_sourceContextAttrs<'input> for Dag_sourceContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn dag_source(&mut self,)
	-> Result<Rc<Dag_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Dag_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 388, RULE_dag_source);
        let mut _localctx: Rc<Dag_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1579);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word |TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule name*/
					recog.base.set_state(1577);
					recog.name()?;

					}
				}

			TPTP_T__50 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule inference_record*/
					recog.base.set_state(1578);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Inference_recordContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Inference_recordContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_inference_record(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_inference_record(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Inference_recordContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_inference_record(self);
	}
}

impl<'input> CustomRuleContext<'input> for Inference_recordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inference_record }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inference_record }
}
antlr4rust::tid!{Inference_recordContextExt<'a>}

impl<'input> Inference_recordContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Inference_recordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Inference_recordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Inference_recordContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Inference_recordContextExt<'input>>{

fn inference_rule(&self) -> Option<Rc<Inference_ruleContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn useful_info(&self) -> Option<Rc<Useful_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parents(&self) -> Option<Rc<ParentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Inference_recordContextAttrs<'input> for Inference_recordContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn inference_record(&mut self,)
	-> Result<Rc<Inference_recordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Inference_recordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 390, RULE_inference_record);
        let mut _localctx: Rc<Inference_recordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1581);
			recog.base.match_token(TPTP_T__50,&mut recog.err_handler)?;

			/*InvokeRule inference_rule*/
			recog.base.set_state(1582);
			recog.inference_rule()?;

			recog.base.set_state(1583);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule useful_info*/
			recog.base.set_state(1584);
			recog.useful_info()?;

			recog.base.set_state(1585);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule parents*/
			recog.base.set_state(1586);
			recog.parents()?;

			recog.base.set_state(1587);
			recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Inference_ruleContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Inference_ruleContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_inference_rule(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_inference_rule(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Inference_ruleContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_inference_rule(self);
	}
}

impl<'input> CustomRuleContext<'input> for Inference_ruleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inference_rule }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inference_rule }
}
antlr4rust::tid!{Inference_ruleContextExt<'a>}

impl<'input> Inference_ruleContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Inference_ruleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Inference_ruleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Inference_ruleContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Inference_ruleContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Inference_ruleContextAttrs<'input> for Inference_ruleContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn inference_rule(&mut self,)
	-> Result<Rc<Inference_ruleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Inference_ruleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 392, RULE_inference_rule);
        let mut _localctx: Rc<Inference_ruleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1589);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Internal_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Internal_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_internal_source(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_internal_source(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Internal_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_internal_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for Internal_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_internal_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_internal_source }
}
antlr4rust::tid!{Internal_sourceContextExt<'a>}

impl<'input> Internal_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Internal_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Internal_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Internal_sourceContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Internal_sourceContextExt<'input>>{

fn intro_type(&self) -> Option<Rc<Intro_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn useful_info(&self) -> Option<Rc<Useful_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parents(&self) -> Option<Rc<ParentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Internal_sourceContextAttrs<'input> for Internal_sourceContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn internal_source(&mut self,)
	-> Result<Rc<Internal_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Internal_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 394, RULE_internal_source);
        let mut _localctx: Rc<Internal_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1591);
			recog.base.match_token(TPTP_T__51,&mut recog.err_handler)?;

			/*InvokeRule intro_type*/
			recog.base.set_state(1592);
			recog.intro_type()?;

			recog.base.set_state(1593);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule useful_info*/
			recog.base.set_state(1594);
			recog.useful_info()?;

			recog.base.set_state(1595);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule parents*/
			recog.base.set_state(1596);
			recog.parents()?;

			recog.base.set_state(1597);
			recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Intro_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Intro_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intro_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intro_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Intro_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_intro_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Intro_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intro_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intro_type }
}
antlr4rust::tid!{Intro_typeContextExt<'a>}

impl<'input> Intro_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Intro_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Intro_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Intro_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Intro_typeContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Intro_typeContextAttrs<'input> for Intro_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn intro_type(&mut self,)
	-> Result<Rc<Intro_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Intro_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 396, RULE_intro_type);
        let mut _localctx: Rc<Intro_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1599);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for External_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for External_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_external_source(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_external_source(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for External_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_external_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for External_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_external_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_external_source }
}
antlr4rust::tid!{External_sourceContextExt<'a>}

impl<'input> External_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<External_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,External_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait External_sourceContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<External_sourceContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn external_source(&mut self,)
	-> Result<Rc<External_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = External_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 398, RULE_external_source);
        let mut _localctx: Rc<External_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1604);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__52 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule file_source*/
					recog.base.set_state(1601);
					recog.file_source()?;

					}
				}

			TPTP_T__53 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule theory*/
					recog.base.set_state(1602);
					recog.theory()?;

					}
				}

			TPTP_T__54 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule creator_source*/
					recog.base.set_state(1603);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for File_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for File_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_file_source(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_file_source(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for File_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_file_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for File_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file_source }
}
antlr4rust::tid!{File_sourceContextExt<'a>}

impl<'input> File_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<File_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,File_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait File_sourceContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<File_sourceContextExt<'input>>{

fn file_name(&self) -> Option<Rc<File_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn file_info(&self) -> Option<Rc<File_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> File_sourceContextAttrs<'input> for File_sourceContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn file_source(&mut self,)
	-> Result<Rc<File_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = File_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 400, RULE_file_source);
        let mut _localctx: Rc<File_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1606);
			recog.base.match_token(TPTP_T__52,&mut recog.err_handler)?;

			/*InvokeRule file_name*/
			recog.base.set_state(1607);
			recog.file_name()?;

			/*InvokeRule file_info*/
			recog.base.set_state(1608);
			recog.file_info()?;

			recog.base.set_state(1609);
			recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for File_infoContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for File_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_file_info(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_file_info(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for File_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_file_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for File_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file_info }
}
antlr4rust::tid!{File_infoContextExt<'a>}

impl<'input> File_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<File_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,File_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait File_infoContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<File_infoContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nothing(&self) -> Option<Rc<NothingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> File_infoContextAttrs<'input> for File_infoContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn file_info(&mut self,)
	-> Result<Rc<File_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = File_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 402, RULE_file_info);
        let mut _localctx: Rc<File_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1614);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__1 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1611);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule name*/
					recog.base.set_state(1612);
					recog.name()?;

					}
				}

			TPTP_T__12 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nothing*/
					recog.base.set_state(1613);
					recog.nothing()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for TheoryContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for TheoryContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_theory(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_theory(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for TheoryContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_theory(self);
	}
}

impl<'input> CustomRuleContext<'input> for TheoryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_theory }
	//fn type_rule_index() -> usize where Self: Sized { RULE_theory }
}
antlr4rust::tid!{TheoryContextExt<'a>}

impl<'input> TheoryContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TheoryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TheoryContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TheoryContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<TheoryContextExt<'input>>{

fn theory_name(&self) -> Option<Rc<Theory_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn optional_info(&self) -> Option<Rc<Optional_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TheoryContextAttrs<'input> for TheoryContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn theory(&mut self,)
	-> Result<Rc<TheoryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TheoryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 404, RULE_theory);
        let mut _localctx: Rc<TheoryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1616);
			recog.base.match_token(TPTP_T__53,&mut recog.err_handler)?;

			/*InvokeRule theory_name*/
			recog.base.set_state(1617);
			recog.theory_name()?;

			/*InvokeRule optional_info*/
			recog.base.set_state(1618);
			recog.optional_info()?;

			recog.base.set_state(1619);
			recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Theory_nameContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Theory_nameContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_theory_name(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_theory_name(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Theory_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_theory_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Theory_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_theory_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_theory_name }
}
antlr4rust::tid!{Theory_nameContextExt<'a>}

impl<'input> Theory_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Theory_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Theory_nameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Theory_nameContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Theory_nameContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Theory_nameContextAttrs<'input> for Theory_nameContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn theory_name(&mut self,)
	-> Result<Rc<Theory_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Theory_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 406, RULE_theory_name);
        let mut _localctx: Rc<Theory_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1621);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Creator_sourceContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Creator_sourceContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_creator_source(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_creator_source(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Creator_sourceContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_creator_source(self);
	}
}

impl<'input> CustomRuleContext<'input> for Creator_sourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_creator_source }
	//fn type_rule_index() -> usize where Self: Sized { RULE_creator_source }
}
antlr4rust::tid!{Creator_sourceContextExt<'a>}

impl<'input> Creator_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Creator_sourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Creator_sourceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Creator_sourceContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Creator_sourceContextExt<'input>>{

fn creator_name(&self) -> Option<Rc<Creator_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn useful_info(&self) -> Option<Rc<Useful_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parents(&self) -> Option<Rc<ParentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Creator_sourceContextAttrs<'input> for Creator_sourceContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn creator_source(&mut self,)
	-> Result<Rc<Creator_sourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Creator_sourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 408, RULE_creator_source);
        let mut _localctx: Rc<Creator_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1623);
			recog.base.match_token(TPTP_T__54,&mut recog.err_handler)?;

			/*InvokeRule creator_name*/
			recog.base.set_state(1624);
			recog.creator_name()?;

			recog.base.set_state(1625);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule useful_info*/
			recog.base.set_state(1626);
			recog.useful_info()?;

			recog.base.set_state(1627);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule parents*/
			recog.base.set_state(1628);
			recog.parents()?;

			recog.base.set_state(1629);
			recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Creator_nameContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Creator_nameContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_creator_name(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_creator_name(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Creator_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_creator_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Creator_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_creator_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_creator_name }
}
antlr4rust::tid!{Creator_nameContextExt<'a>}

impl<'input> Creator_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Creator_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Creator_nameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Creator_nameContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Creator_nameContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Creator_nameContextAttrs<'input> for Creator_nameContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn creator_name(&mut self,)
	-> Result<Rc<Creator_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Creator_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 410, RULE_creator_name);
        let mut _localctx: Rc<Creator_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1631);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- parents ----------------
pub type ParentsContextAll<'input> = ParentsContext<'input>;


pub type ParentsContext<'input> = BaseParserRuleContext<'input,ParentsContextExt<'input>>;

#[derive(Clone)]
pub struct ParentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for ParentsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for ParentsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parents(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parents(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for ParentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_parents(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parents }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parents }
}
antlr4rust::tid!{ParentsContextExt<'a>}

impl<'input> ParentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ParentsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<ParentsContextExt<'input>>{

fn parent_list(&self) -> Option<Rc<Parent_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParentsContextAttrs<'input> for ParentsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parents(&mut self,)
	-> Result<Rc<ParentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 412, RULE_parents);
        let mut _localctx: Rc<ParentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1638);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1633);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1634);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule parent_list*/
					recog.base.set_state(1635);
					recog.parent_list()?;

					recog.base.set_state(1636);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Parent_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Parent_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parent_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parent_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Parent_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_parent_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Parent_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parent_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parent_list }
}
antlr4rust::tid!{Parent_listContextExt<'a>}

impl<'input> Parent_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Parent_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Parent_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Parent_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Parent_listContextExt<'input>>{

fn parent_info(&self) -> Option<Rc<Parent_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comma_parent_info_all(&self) ->  Vec<Rc<Comma_parent_infoContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comma_parent_info(&self, i: usize) -> Option<Rc<Comma_parent_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Parent_listContextAttrs<'input> for Parent_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parent_list(&mut self,)
	-> Result<Rc<Parent_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Parent_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 414, RULE_parent_list);
        let mut _localctx: Rc<Parent_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule parent_info*/
			recog.base.set_state(1640);
			recog.parent_info()?;

			recog.base.set_state(1644);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_parent_info*/
				recog.base.set_state(1641);
				recog.comma_parent_info()?;

				}
				}
				recog.base.set_state(1646);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- comma_parent_info ----------------
pub type Comma_parent_infoContextAll<'input> = Comma_parent_infoContext<'input>;


pub type Comma_parent_infoContext<'input> = BaseParserRuleContext<'input,Comma_parent_infoContextExt<'input>>;

#[derive(Clone)]
pub struct Comma_parent_infoContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Comma_parent_infoContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Comma_parent_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_comma_parent_info(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_comma_parent_info(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Comma_parent_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_comma_parent_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for Comma_parent_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comma_parent_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comma_parent_info }
}
antlr4rust::tid!{Comma_parent_infoContextExt<'a>}

impl<'input> Comma_parent_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Comma_parent_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Comma_parent_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Comma_parent_infoContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Comma_parent_infoContextExt<'input>>{

fn parent_info(&self) -> Option<Rc<Parent_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Comma_parent_infoContextAttrs<'input> for Comma_parent_infoContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comma_parent_info(&mut self,)
	-> Result<Rc<Comma_parent_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Comma_parent_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 416, RULE_comma_parent_info);
        let mut _localctx: Rc<Comma_parent_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1647);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule parent_info*/
			recog.base.set_state(1648);
			recog.parent_info()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Parent_infoContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Parent_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parent_info(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parent_info(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Parent_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_parent_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for Parent_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parent_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parent_info }
}
antlr4rust::tid!{Parent_infoContextExt<'a>}

impl<'input> Parent_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Parent_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Parent_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Parent_infoContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Parent_infoContextExt<'input>>{

fn source(&self) -> Option<Rc<SourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parent_details(&self) -> Option<Rc<Parent_detailsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Parent_infoContextAttrs<'input> for Parent_infoContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parent_info(&mut self,)
	-> Result<Rc<Parent_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Parent_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 418, RULE_parent_info);
        let mut _localctx: Rc<Parent_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule source*/
			recog.base.set_state(1650);
			recog.source()?;

			/*InvokeRule parent_details*/
			recog.base.set_state(1651);
			recog.parent_details()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Parent_detailsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Parent_detailsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parent_details(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parent_details(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Parent_detailsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_parent_details(self);
	}
}

impl<'input> CustomRuleContext<'input> for Parent_detailsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parent_details }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parent_details }
}
antlr4rust::tid!{Parent_detailsContextExt<'a>}

impl<'input> Parent_detailsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Parent_detailsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Parent_detailsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Parent_detailsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Parent_detailsContextExt<'input>>{

fn general_term(&self) -> Option<Rc<General_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nothing(&self) -> Option<Rc<NothingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Parent_detailsContextAttrs<'input> for Parent_detailsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parent_details(&mut self,)
	-> Result<Rc<Parent_detailsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Parent_detailsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 420, RULE_parent_details);
        let mut _localctx: Rc<Parent_detailsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1656);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__15 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1653);
					recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

					/*InvokeRule general_term*/
					recog.base.set_state(1654);
					recog.general_term()?;

					}
				}

			TPTP_T__1 |TPTP_T__14 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nothing*/
					recog.base.set_state(1655);
					recog.nothing()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Optional_infoContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Optional_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_optional_info(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_optional_info(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Optional_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_optional_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for Optional_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_optional_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_optional_info }
}
antlr4rust::tid!{Optional_infoContextExt<'a>}

impl<'input> Optional_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Optional_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Optional_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Optional_infoContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Optional_infoContextExt<'input>>{

fn useful_info(&self) -> Option<Rc<Useful_infoContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nothing(&self) -> Option<Rc<NothingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Optional_infoContextAttrs<'input> for Optional_infoContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn optional_info(&mut self,)
	-> Result<Rc<Optional_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Optional_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 422, RULE_optional_info);
        let mut _localctx: Rc<Optional_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1661);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__1 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1658);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule useful_info*/
					recog.base.set_state(1659);
					recog.useful_info()?;

					}
				}

			TPTP_T__2 |TPTP_T__12 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nothing*/
					recog.base.set_state(1660);
					recog.nothing()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Useful_infoContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Useful_infoContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_useful_info(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_useful_info(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Useful_infoContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_useful_info(self);
	}
}

impl<'input> CustomRuleContext<'input> for Useful_infoContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_useful_info }
	//fn type_rule_index() -> usize where Self: Sized { RULE_useful_info }
}
antlr4rust::tid!{Useful_infoContextExt<'a>}

impl<'input> Useful_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Useful_infoContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Useful_infoContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Useful_infoContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Useful_infoContextExt<'input>>{

fn general_list(&self) -> Option<Rc<General_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Useful_infoContextAttrs<'input> for Useful_infoContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn useful_info(&mut self,)
	-> Result<Rc<Useful_infoContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Useful_infoContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 424, RULE_useful_info);
        let mut _localctx: Rc<Useful_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule general_list*/
			recog.base.set_state(1663);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for IncludeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for IncludeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_include(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_include(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for IncludeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_include(self);
	}
}

impl<'input> CustomRuleContext<'input> for IncludeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_include }
	//fn type_rule_index() -> usize where Self: Sized { RULE_include }
}
antlr4rust::tid!{IncludeContextExt<'a>}

impl<'input> IncludeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IncludeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IncludeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IncludeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<IncludeContextExt<'input>>{

fn file_name(&self) -> Option<Rc<File_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn include_optionals(&self) -> Option<Rc<Include_optionalsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IncludeContextAttrs<'input> for IncludeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn include(&mut self,)
	-> Result<Rc<IncludeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IncludeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 426, RULE_include);
        let mut _localctx: Rc<IncludeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1665);
			recog.base.match_token(TPTP_T__55,&mut recog.err_handler)?;

			/*InvokeRule file_name*/
			recog.base.set_state(1666);
			recog.file_name()?;

			/*InvokeRule include_optionals*/
			recog.base.set_state(1667);
			recog.include_optionals()?;

			recog.base.set_state(1668);
			recog.base.match_token(TPTP_T__2,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- include_optionals ----------------
pub type Include_optionalsContextAll<'input> = Include_optionalsContext<'input>;


pub type Include_optionalsContext<'input> = BaseParserRuleContext<'input,Include_optionalsContextExt<'input>>;

#[derive(Clone)]
pub struct Include_optionalsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Include_optionalsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Include_optionalsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_include_optionals(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_include_optionals(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Include_optionalsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_include_optionals(self);
	}
}

impl<'input> CustomRuleContext<'input> for Include_optionalsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_include_optionals }
	//fn type_rule_index() -> usize where Self: Sized { RULE_include_optionals }
}
antlr4rust::tid!{Include_optionalsContextExt<'a>}

impl<'input> Include_optionalsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Include_optionalsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Include_optionalsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Include_optionalsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Include_optionalsContextExt<'input>>{

fn nothing(&self) -> Option<Rc<NothingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formula_selection(&self) -> Option<Rc<Formula_selectionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn space_name(&self) -> Option<Rc<Space_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Include_optionalsContextAttrs<'input> for Include_optionalsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn include_optionals(&mut self,)
	-> Result<Rc<Include_optionalsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Include_optionalsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 428, RULE_include_optionals);
        let mut _localctx: Rc<Include_optionalsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1678);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(122,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nothing*/
					recog.base.set_state(1670);
					recog.nothing()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1671);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule formula_selection*/
					recog.base.set_state(1672);
					recog.formula_selection()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1673);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule formula_selection*/
					recog.base.set_state(1674);
					recog.formula_selection()?;

					recog.base.set_state(1675);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule space_name*/
					recog.base.set_state(1676);
					recog.space_name()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Formula_selectionContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Formula_selectionContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_formula_selection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_formula_selection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Formula_selectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_formula_selection(self);
	}
}

impl<'input> CustomRuleContext<'input> for Formula_selectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formula_selection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formula_selection }
}
antlr4rust::tid!{Formula_selectionContextExt<'a>}

impl<'input> Formula_selectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Formula_selectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Formula_selectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Formula_selectionContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Formula_selectionContextExt<'input>>{

fn name_list(&self) -> Option<Rc<Name_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Star, 0)
}

}

impl<'input> Formula_selectionContextAttrs<'input> for Formula_selectionContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn formula_selection(&mut self,)
	-> Result<Rc<Formula_selectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Formula_selectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 430, RULE_formula_selection);
        let mut _localctx: Rc<Formula_selectionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1685);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1680);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule name_list*/
					recog.base.set_state(1681);
					recog.name_list()?;

					recog.base.set_state(1682);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

					}
				}

			TPTP_Star 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1684);
					recog.base.match_token(TPTP_Star,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Name_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Name_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_name_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_name_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Name_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_name_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Name_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_name_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_name_list }
}
antlr4rust::tid!{Name_listContextExt<'a>}

impl<'input> Name_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Name_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Name_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Name_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Name_listContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn name_list(&self) -> Option<Rc<Name_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Name_listContextAttrs<'input> for Name_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn name_list(&mut self,)
	-> Result<Rc<Name_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Name_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 432, RULE_name_list);
        let mut _localctx: Rc<Name_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1692);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(124,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule name*/
					recog.base.set_state(1687);
					recog.name()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule name*/
					recog.base.set_state(1688);
					recog.name()?;

					recog.base.set_state(1689);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule name_list*/
					recog.base.set_state(1690);
					recog.name_list()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- space_name ----------------
pub type Space_nameContextAll<'input> = Space_nameContext<'input>;


pub type Space_nameContext<'input> = BaseParserRuleContext<'input,Space_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Space_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Space_nameContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Space_nameContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_space_name(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_space_name(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Space_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_space_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Space_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_space_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_space_name }
}
antlr4rust::tid!{Space_nameContextExt<'a>}

impl<'input> Space_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Space_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Space_nameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Space_nameContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Space_nameContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Space_nameContextAttrs<'input> for Space_nameContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn space_name(&mut self,)
	-> Result<Rc<Space_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Space_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 434, RULE_space_name);
        let mut _localctx: Rc<Space_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule name*/
			recog.base.set_state(1694);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for General_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for General_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_general_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_general_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for General_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_general_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_term }
}
antlr4rust::tid!{General_termContextExt<'a>}

impl<'input> General_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<General_termContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn general_term(&mut self,)
	-> Result<Rc<General_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 436, RULE_general_term);
        let mut _localctx: Rc<General_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1702);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(125,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule general_data*/
					recog.base.set_state(1696);
					recog.general_data()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule general_data*/
					recog.base.set_state(1697);
					recog.general_data()?;

					recog.base.set_state(1698);
					recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

					/*InvokeRule general_term*/
					recog.base.set_state(1699);
					recog.general_term()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule general_list*/
					recog.base.set_state(1701);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for General_dataContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for General_dataContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_general_data(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_general_data(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for General_dataContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_general_data(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_dataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_data }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_data }
}
antlr4rust::tid!{General_dataContextExt<'a>}

impl<'input> General_dataContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_dataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_dataContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_dataContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<General_dataContextExt<'input>>{

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
fn Distinct_object(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Distinct_object, 0)
}
fn formula_data(&self) -> Option<Rc<Formula_dataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> General_dataContextAttrs<'input> for General_dataContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn general_data(&mut self,)
	-> Result<Rc<General_dataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_dataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 438, RULE_general_data);
        let mut _localctx: Rc<General_dataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1710);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(126,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule atomic_word*/
					recog.base.set_state(1704);
					recog.atomic_word()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule general_function*/
					recog.base.set_state(1705);
					recog.general_function()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1706);
					recog.variable()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule number*/
					recog.base.set_state(1707);
					recog.number()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1708);
					recog.base.match_token(TPTP_Distinct_object,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule formula_data*/
					recog.base.set_state(1709);
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for General_functionContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for General_functionContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_general_function(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_general_function(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for General_functionContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_general_function(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_functionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_function }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_function }
}
antlr4rust::tid!{General_functionContextExt<'a>}

impl<'input> General_functionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_functionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_functionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_functionContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<General_functionContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn general_terms(&self) -> Option<Rc<General_termsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> General_functionContextAttrs<'input> for General_functionContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn general_function(&mut self,)
	-> Result<Rc<General_functionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_functionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 440, RULE_general_function);
        let mut _localctx: Rc<General_functionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1712);
			recog.atomic_word()?;

			recog.base.set_state(1713);
			recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

			/*InvokeRule general_terms*/
			recog.base.set_state(1714);
			recog.general_terms()?;

			recog.base.set_state(1715);
			recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Formula_dataContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Formula_dataContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_formula_data(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_formula_data(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Formula_dataContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_formula_data(self);
	}
}

impl<'input> CustomRuleContext<'input> for Formula_dataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formula_data }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formula_data }
}
antlr4rust::tid!{Formula_dataContextExt<'a>}

impl<'input> Formula_dataContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Formula_dataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Formula_dataContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Formula_dataContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Formula_dataContextExt<'input>>{

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

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn formula_data(&mut self,)
	-> Result<Rc<Formula_dataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Formula_dataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 442, RULE_formula_data);
        let mut _localctx: Rc<Formula_dataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1737);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__56 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1717);
					recog.base.match_token(TPTP_T__56,&mut recog.err_handler)?;

					/*InvokeRule thf_formula*/
					recog.base.set_state(1718);
					recog.thf_formula()?;

					recog.base.set_state(1719);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__57 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1721);
					recog.base.match_token(TPTP_T__57,&mut recog.err_handler)?;

					/*InvokeRule tff_formula*/
					recog.base.set_state(1722);
					recog.tff_formula()?;

					recog.base.set_state(1723);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__58 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1725);
					recog.base.match_token(TPTP_T__58,&mut recog.err_handler)?;

					/*InvokeRule fof_formula*/
					recog.base.set_state(1726);
					recog.fof_formula()?;

					recog.base.set_state(1727);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__59 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(1729);
					recog.base.match_token(TPTP_T__59,&mut recog.err_handler)?;

					/*InvokeRule cnf_formula*/
					recog.base.set_state(1730);
					recog.cnf_formula()?;

					recog.base.set_state(1731);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__60 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1733);
					recog.base.match_token(TPTP_T__60,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(1734);
					recog.fof_term()?;

					recog.base.set_state(1735);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for General_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for General_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_general_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_general_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for General_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_general_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_list }
}
antlr4rust::tid!{General_listContextExt<'a>}

impl<'input> General_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<General_listContextExt<'input>>{

fn general_terms(&self) -> Option<Rc<General_termsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> General_listContextAttrs<'input> for General_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn general_list(&mut self,)
	-> Result<Rc<General_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 444, RULE_general_list);
        let mut _localctx: Rc<General_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1744);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1739);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1740);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule general_terms*/
					recog.base.set_state(1741);
					recog.general_terms()?;

					recog.base.set_state(1742);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for General_termsContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for General_termsContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_general_terms(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_general_terms(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for General_termsContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_general_terms(self);
	}
}

impl<'input> CustomRuleContext<'input> for General_termsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_general_terms }
	//fn type_rule_index() -> usize where Self: Sized { RULE_general_terms }
}
antlr4rust::tid!{General_termsContextExt<'a>}

impl<'input> General_termsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<General_termsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,General_termsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait General_termsContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<General_termsContextExt<'input>>{

fn general_term(&self) -> Option<Rc<General_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comma_general_term_all(&self) ->  Vec<Rc<Comma_general_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comma_general_term(&self, i: usize) -> Option<Rc<Comma_general_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> General_termsContextAttrs<'input> for General_termsContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn general_terms(&mut self,)
	-> Result<Rc<General_termsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = General_termsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 446, RULE_general_terms);
        let mut _localctx: Rc<General_termsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule general_term*/
			recog.base.set_state(1746);
			recog.general_term()?;

			recog.base.set_state(1750);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_general_term*/
				recog.base.set_state(1747);
				recog.comma_general_term()?;

				}
				}
				recog.base.set_state(1752);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- comma_general_term ----------------
pub type Comma_general_termContextAll<'input> = Comma_general_termContext<'input>;


pub type Comma_general_termContext<'input> = BaseParserRuleContext<'input,Comma_general_termContextExt<'input>>;

#[derive(Clone)]
pub struct Comma_general_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Comma_general_termContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Comma_general_termContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_comma_general_term(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_comma_general_term(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Comma_general_termContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_comma_general_term(self);
	}
}

impl<'input> CustomRuleContext<'input> for Comma_general_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comma_general_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comma_general_term }
}
antlr4rust::tid!{Comma_general_termContextExt<'a>}

impl<'input> Comma_general_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Comma_general_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Comma_general_termContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Comma_general_termContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Comma_general_termContextExt<'input>>{

fn general_term(&self) -> Option<Rc<General_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Comma_general_termContextAttrs<'input> for Comma_general_termContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comma_general_term(&mut self,)
	-> Result<Rc<Comma_general_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Comma_general_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 448, RULE_comma_general_term);
        let mut _localctx: Rc<Comma_general_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1753);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule general_term*/
			recog.base.set_state(1754);
			recog.general_term()?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for NameContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for NameContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_name(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_name(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for NameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for NameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_name }
}
antlr4rust::tid!{NameContextExt<'a>}

impl<'input> NameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NameContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<NameContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Integer
/// Returns `None` if there is no child corresponding to token Integer
fn Integer(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Integer, 0)
}

}

impl<'input> NameContextAttrs<'input> for NameContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn name(&mut self,)
	-> Result<Rc<NameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 450, RULE_name);
        let mut _localctx: Rc<NameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1758);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule atomic_word*/
					recog.base.set_state(1756);
					recog.atomic_word()?;

					}
				}

			TPTP_Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1757);
					recog.base.match_token(TPTP_Integer,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Atomic_wordContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Atomic_wordContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atomic_word(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atomic_word(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Atomic_wordContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_atomic_word(self);
	}
}

impl<'input> CustomRuleContext<'input> for Atomic_wordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomic_word }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomic_word }
}
antlr4rust::tid!{Atomic_wordContextExt<'a>}

impl<'input> Atomic_wordContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atomic_wordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atomic_wordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atomic_wordContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Atomic_wordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Lower_word
/// Returns `None` if there is no child corresponding to token Lower_word
fn Lower_word(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Lower_word, 0)
}
/// Retrieves first TerminalNode corresponding to token Single_quoted
/// Returns `None` if there is no child corresponding to token Single_quoted
fn Single_quoted(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Single_quoted, 0)
}
/// Retrieves first TerminalNode corresponding to token Back_quoted
/// Returns `None` if there is no child corresponding to token Back_quoted
fn Back_quoted(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Back_quoted, 0)
}

}

impl<'input> Atomic_wordContextAttrs<'input> for Atomic_wordContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atomic_word(&mut self,)
	-> Result<Rc<Atomic_wordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atomic_wordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 452, RULE_atomic_word);
        let mut _localctx: Rc<Atomic_wordContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1760);
			_la = recog.base.input.la(1);
			if { !(((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & 67) != 0)) } {
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Atomic_defined_wordContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Atomic_defined_wordContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atomic_defined_word(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atomic_defined_word(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Atomic_defined_wordContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_atomic_defined_word(self);
	}
}

impl<'input> CustomRuleContext<'input> for Atomic_defined_wordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomic_defined_word }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomic_defined_word }
}
antlr4rust::tid!{Atomic_defined_wordContextExt<'a>}

impl<'input> Atomic_defined_wordContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atomic_defined_wordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atomic_defined_wordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atomic_defined_wordContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Atomic_defined_wordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Dollar_word
/// Returns `None` if there is no child corresponding to token Dollar_word
fn Dollar_word(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Dollar_word, 0)
}

}

impl<'input> Atomic_defined_wordContextAttrs<'input> for Atomic_defined_wordContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atomic_defined_word(&mut self,)
	-> Result<Rc<Atomic_defined_wordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atomic_defined_wordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 454, RULE_atomic_defined_word);
        let mut _localctx: Rc<Atomic_defined_wordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1762);
			recog.base.match_token(TPTP_Dollar_word,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for Atomic_system_wordContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Atomic_system_wordContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atomic_system_word(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atomic_system_word(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Atomic_system_wordContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_atomic_system_word(self);
	}
}

impl<'input> CustomRuleContext<'input> for Atomic_system_wordContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomic_system_word }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomic_system_word }
}
antlr4rust::tid!{Atomic_system_wordContextExt<'a>}

impl<'input> Atomic_system_wordContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atomic_system_wordContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atomic_system_wordContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atomic_system_wordContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Atomic_system_wordContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Dollar_dollar_word
/// Returns `None` if there is no child corresponding to token Dollar_dollar_word
fn Dollar_dollar_word(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Dollar_dollar_word, 0)
}

}

impl<'input> Atomic_system_wordContextAttrs<'input> for Atomic_system_wordContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atomic_system_word(&mut self,)
	-> Result<Rc<Atomic_system_wordContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atomic_system_wordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 456, RULE_atomic_system_word);
        let mut _localctx: Rc<Atomic_system_wordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1764);
			recog.base.match_token(TPTP_Dollar_dollar_word,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for NumberContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for NumberContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_number(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_number(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for NumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_number(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_number }
	//fn type_rule_index() -> usize where Self: Sized { RULE_number }
}
antlr4rust::tid!{NumberContextExt<'a>}

impl<'input> NumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NumberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NumberContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<NumberContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Integer
/// Returns `None` if there is no child corresponding to token Integer
fn Integer(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Integer, 0)
}
/// Retrieves first TerminalNode corresponding to token Rational
/// Returns `None` if there is no child corresponding to token Rational
fn Rational(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Rational, 0)
}
/// Retrieves first TerminalNode corresponding to token Real
/// Returns `None` if there is no child corresponding to token Real
fn Real(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Real, 0)
}

}

impl<'input> NumberContextAttrs<'input> for NumberContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn number(&mut self,)
	-> Result<Rc<NumberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 458, RULE_number);
        let mut _localctx: Rc<NumberContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1766);
			_la = recog.base.input.la(1);
			if { !(((((_la - 72)) & !0x3f) == 0 && ((1usize << (_la - 72)) & 1153) != 0)) } {
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
		recog.base.exit_rule()?;

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

impl<'input> TPTPParserContext<'input> for File_nameContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for File_nameContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_file_name(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_file_name(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for File_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_file_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for File_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file_name }
}
antlr4rust::tid!{File_nameContextExt<'a>}

impl<'input> File_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<File_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,File_nameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait File_nameContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<File_nameContextExt<'input>>{

fn atomic_word(&self) -> Option<Rc<Atomic_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> File_nameContextAttrs<'input> for File_nameContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn file_name(&mut self,)
	-> Result<Rc<File_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = File_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 460, RULE_file_name);
        let mut _localctx: Rc<File_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1768);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nothing ----------------
pub type NothingContextAll<'input> = NothingContext<'input>;


pub type NothingContext<'input> = BaseParserRuleContext<'input,NothingContextExt<'input>>;

#[derive(Clone)]
pub struct NothingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for NothingContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for NothingContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nothing(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nothing(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for NothingContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_nothing(self);
	}
}

impl<'input> CustomRuleContext<'input> for NothingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nothing }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nothing }
}
antlr4rust::tid!{NothingContextExt<'a>}

impl<'input> NothingContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NothingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NothingContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NothingContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<NothingContextExt<'input>>{


}

impl<'input> NothingContextAttrs<'input> for NothingContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nothing(&mut self,)
	-> Result<Rc<NothingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NothingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 462, RULE_nothing);
        let mut _localctx: Rc<NothingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
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
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 108, 1773, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 
		7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 
		7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 
		7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 20, 
		7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 
		7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 30, 
		7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 35, 
		7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 39, 2, 40, 
		7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 44, 2, 45, 
		7, 45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 48, 2, 49, 7, 49, 2, 50, 
		7, 50, 2, 51, 7, 51, 2, 52, 7, 52, 2, 53, 7, 53, 2, 54, 7, 54, 2, 55, 
		7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 2, 58, 7, 58, 2, 59, 7, 59, 2, 60, 
		7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 63, 7, 63, 2, 64, 7, 64, 2, 65, 
		7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 2, 68, 7, 68, 2, 69, 7, 69, 2, 70, 
		7, 70, 2, 71, 7, 71, 2, 72, 7, 72, 2, 73, 7, 73, 2, 74, 7, 74, 2, 75, 
		7, 75, 2, 76, 7, 76, 2, 77, 7, 77, 2, 78, 7, 78, 2, 79, 7, 79, 2, 80, 
		7, 80, 2, 81, 7, 81, 2, 82, 7, 82, 2, 83, 7, 83, 2, 84, 7, 84, 2, 85, 
		7, 85, 2, 86, 7, 86, 2, 87, 7, 87, 2, 88, 7, 88, 2, 89, 7, 89, 2, 90, 
		7, 90, 2, 91, 7, 91, 2, 92, 7, 92, 2, 93, 7, 93, 2, 94, 7, 94, 2, 95, 
		7, 95, 2, 96, 7, 96, 2, 97, 7, 97, 2, 98, 7, 98, 2, 99, 7, 99, 2, 100, 
		7, 100, 2, 101, 7, 101, 2, 102, 7, 102, 2, 103, 7, 103, 2, 104, 7, 104, 
		2, 105, 7, 105, 2, 106, 7, 106, 2, 107, 7, 107, 2, 108, 7, 108, 2, 109, 
		7, 109, 2, 110, 7, 110, 2, 111, 7, 111, 2, 112, 7, 112, 2, 113, 7, 113, 
		2, 114, 7, 114, 2, 115, 7, 115, 2, 116, 7, 116, 2, 117, 7, 117, 2, 118, 
		7, 118, 2, 119, 7, 119, 2, 120, 7, 120, 2, 121, 7, 121, 2, 122, 7, 122, 
		2, 123, 7, 123, 2, 124, 7, 124, 2, 125, 7, 125, 2, 126, 7, 126, 2, 127, 
		7, 127, 2, 128, 7, 128, 2, 129, 7, 129, 2, 130, 7, 130, 2, 131, 7, 131, 
		2, 132, 7, 132, 2, 133, 7, 133, 2, 134, 7, 134, 2, 135, 7, 135, 2, 136, 
		7, 136, 2, 137, 7, 137, 2, 138, 7, 138, 2, 139, 7, 139, 2, 140, 7, 140, 
		2, 141, 7, 141, 2, 142, 7, 142, 2, 143, 7, 143, 2, 144, 7, 144, 2, 145, 
		7, 145, 2, 146, 7, 146, 2, 147, 7, 147, 2, 148, 7, 148, 2, 149, 7, 149, 
		2, 150, 7, 150, 2, 151, 7, 151, 2, 152, 7, 152, 2, 153, 7, 153, 2, 154, 
		7, 154, 2, 155, 7, 155, 2, 156, 7, 156, 2, 157, 7, 157, 2, 158, 7, 158, 
		2, 159, 7, 159, 2, 160, 7, 160, 2, 161, 7, 161, 2, 162, 7, 162, 2, 163, 
		7, 163, 2, 164, 7, 164, 2, 165, 7, 165, 2, 166, 7, 166, 2, 167, 7, 167, 
		2, 168, 7, 168, 2, 169, 7, 169, 2, 170, 7, 170, 2, 171, 7, 171, 2, 172, 
		7, 172, 2, 173, 7, 173, 2, 174, 7, 174, 2, 175, 7, 175, 2, 176, 7, 176, 
		2, 177, 7, 177, 2, 178, 7, 178, 2, 179, 7, 179, 2, 180, 7, 180, 2, 181, 
		7, 181, 2, 182, 7, 182, 2, 183, 7, 183, 2, 184, 7, 184, 2, 185, 7, 185, 
		2, 186, 7, 186, 2, 187, 7, 187, 2, 188, 7, 188, 2, 189, 7, 189, 2, 190, 
		7, 190, 2, 191, 7, 191, 2, 192, 7, 192, 2, 193, 7, 193, 2, 194, 7, 194, 
		2, 195, 7, 195, 2, 196, 7, 196, 2, 197, 7, 197, 2, 198, 7, 198, 2, 199, 
		7, 199, 2, 200, 7, 200, 2, 201, 7, 201, 2, 202, 7, 202, 2, 203, 7, 203, 
		2, 204, 7, 204, 2, 205, 7, 205, 2, 206, 7, 206, 2, 207, 7, 207, 2, 208, 
		7, 208, 2, 209, 7, 209, 2, 210, 7, 210, 2, 211, 7, 211, 2, 212, 7, 212, 
		2, 213, 7, 213, 2, 214, 7, 214, 2, 215, 7, 215, 2, 216, 7, 216, 2, 217, 
		7, 217, 2, 218, 7, 218, 2, 219, 7, 219, 2, 220, 7, 220, 2, 221, 7, 221, 
		2, 222, 7, 222, 2, 223, 7, 223, 2, 224, 7, 224, 2, 225, 7, 225, 2, 226, 
		7, 226, 2, 227, 7, 227, 2, 228, 7, 228, 2, 229, 7, 229, 2, 230, 7, 230, 
		2, 231, 7, 231, 1, 0, 5, 0, 466, 8, 0, 10, 0, 12, 0, 469, 9, 0, 1, 0, 
		1, 0, 1, 1, 1, 1, 3, 1, 475, 8, 1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 
		3, 2, 483, 8, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 
		1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 6, 
		1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 7, 
		1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 
		1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 10, 
		1, 10, 1, 10, 1, 10, 1, 10, 3, 10, 546, 8, 10, 1, 11, 1, 11, 1, 11, 1, 
		11, 3, 11, 552, 8, 11, 1, 12, 1, 12, 1, 12, 3, 12, 557, 8, 12, 1, 13, 
		1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 3, 13, 565, 8, 13, 1, 14, 1, 14, 1, 
		14, 3, 14, 570, 8, 14, 1, 15, 1, 15, 1, 15, 1, 15, 1, 16, 1, 16, 1, 16, 
		3, 16, 579, 8, 16, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 
		17, 5, 17, 589, 8, 17, 10, 17, 12, 17, 592, 9, 17, 1, 18, 1, 18, 1, 18, 
		1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 5, 18, 602, 8, 18, 10, 18, 12, 18, 
		605, 9, 18, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 5, 
		19, 615, 8, 19, 10, 19, 12, 19, 618, 9, 19, 1, 20, 1, 20, 1, 20, 3, 20, 
		623, 8, 20, 1, 21, 1, 21, 3, 21, 627, 8, 21, 1, 22, 1, 22, 1, 22, 1, 22, 
		1, 22, 1, 22, 1, 22, 3, 22, 636, 8, 22, 1, 23, 1, 23, 1, 23, 1, 24, 1, 
		24, 1, 24, 1, 24, 1, 24, 1, 24, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 3, 
		25, 652, 8, 25, 1, 26, 1, 26, 1, 26, 1, 26, 1, 27, 1, 27, 3, 27, 660, 
		8, 27, 1, 28, 1, 28, 1, 28, 1, 29, 1, 29, 1, 29, 1, 29, 1, 30, 1, 30, 
		1, 30, 1, 30, 3, 30, 673, 8, 30, 1, 31, 1, 31, 3, 31, 677, 8, 31, 1, 32, 
		1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 3, 32, 687, 8, 32, 1, 
		33, 1, 33, 3, 33, 691, 8, 33, 1, 34, 1, 34, 1, 34, 1, 34, 1, 35, 1, 35, 
		1, 36, 1, 36, 1, 36, 1, 36, 1, 36, 1, 36, 1, 36, 1, 36, 1, 37, 1, 37, 
		1, 37, 1, 37, 1, 37, 3, 37, 712, 8, 37, 1, 38, 1, 38, 1, 38, 1, 38, 1, 
		38, 3, 38, 719, 8, 38, 1, 39, 1, 39, 1, 39, 1, 39, 1, 39, 3, 39, 726, 
		8, 39, 1, 40, 1, 40, 1, 40, 1, 40, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 
		3, 41, 737, 8, 41, 1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 3, 42, 745, 
		8, 42, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 3, 43, 752, 8, 43, 1, 44, 1, 
		44, 1, 44, 1, 44, 1, 44, 3, 44, 759, 8, 44, 1, 45, 1, 45, 1, 45, 1, 45, 
		1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 
		1, 45, 3, 45, 776, 8, 45, 1, 46, 1, 46, 1, 47, 1, 47, 5, 47, 782, 8, 47, 
		10, 47, 12, 47, 785, 9, 47, 1, 48, 1, 48, 1, 48, 1, 49, 1, 49, 1, 49, 
		1, 49, 1, 49, 1, 49, 1, 49, 1, 49, 3, 49, 798, 8, 49, 1, 50, 1, 50, 1, 
		50, 3, 50, 803, 8, 50, 1, 51, 1, 51, 1, 52, 1, 52, 1, 53, 1, 53, 1, 53, 
		3, 53, 812, 8, 53, 1, 54, 1, 54, 1, 54, 1, 54, 1, 54, 1, 54, 1, 54, 1, 
		54, 3, 54, 822, 8, 54, 1, 55, 1, 55, 1, 55, 1, 55, 1, 55, 1, 55, 1, 55, 
		1, 55, 5, 55, 832, 8, 55, 10, 55, 12, 55, 835, 9, 55, 1, 56, 1, 56, 1, 
		56, 1, 56, 1, 56, 1, 56, 1, 56, 1, 56, 5, 56, 845, 8, 56, 10, 56, 12, 
		56, 848, 9, 56, 1, 57, 1, 57, 1, 57, 1, 57, 1, 58, 1, 58, 1, 58, 1, 58, 
		1, 59, 1, 59, 1, 59, 1, 59, 1, 60, 1, 60, 1, 60, 3, 60, 865, 8, 60, 1, 
		61, 1, 61, 1, 61, 1, 61, 1, 61, 1, 61, 3, 61, 873, 8, 61, 1, 62, 1, 62, 
		3, 62, 877, 8, 62, 1, 63, 1, 63, 1, 63, 1, 63, 1, 64, 1, 64, 3, 64, 885, 
		8, 64, 1, 65, 1, 65, 1, 65, 1, 65, 1, 65, 1, 65, 1, 65, 1, 65, 5, 65, 
		895, 8, 65, 10, 65, 12, 65, 898, 9, 65, 1, 66, 1, 66, 1, 66, 1, 66, 1, 
		66, 1, 66, 1, 66, 1, 66, 5, 66, 908, 8, 66, 10, 66, 12, 66, 911, 9, 66, 
		1, 67, 1, 67, 1, 67, 3, 67, 916, 8, 67, 1, 68, 1, 68, 3, 68, 920, 8, 68, 
		1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 3, 69, 929, 8, 69, 1, 
		70, 1, 70, 1, 71, 1, 71, 1, 71, 1, 71, 1, 71, 1, 71, 1, 71, 1, 72, 1, 
		72, 1, 72, 1, 72, 1, 72, 3, 72, 945, 8, 72, 1, 73, 1, 73, 3, 73, 949, 
		8, 73, 1, 74, 1, 74, 1, 74, 1, 74, 1, 75, 1, 75, 3, 75, 957, 8, 75, 1, 
		76, 1, 76, 1, 76, 1, 77, 1, 77, 1, 77, 1, 77, 1, 78, 1, 78, 1, 78, 3, 
		78, 969, 8, 78, 1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 3, 79, 977, 
		8, 79, 1, 80, 1, 80, 1, 81, 1, 81, 1, 81, 1, 81, 1, 81, 1, 81, 1, 81, 
		1, 81, 3, 81, 989, 8, 81, 1, 82, 1, 82, 1, 82, 1, 82, 1, 83, 1, 83, 1, 
		83, 1, 83, 1, 83, 1, 83, 3, 83, 1001, 8, 83, 1, 84, 1, 84, 1, 84, 1, 84, 
		1, 84, 1, 84, 1, 84, 1, 84, 1, 85, 1, 85, 1, 85, 1, 85, 1, 85, 3, 85, 
		1016, 8, 85, 1, 86, 1, 86, 1, 86, 1, 86, 1, 86, 3, 86, 1023, 8, 86, 1, 
		87, 1, 87, 1, 87, 1, 87, 1, 87, 3, 87, 1030, 8, 87, 1, 88, 1, 88, 1, 88, 
		1, 88, 1, 89, 1, 89, 3, 89, 1038, 8, 89, 1, 90, 1, 90, 1, 90, 1, 90, 1, 
		90, 3, 90, 1045, 8, 90, 1, 91, 1, 91, 1, 91, 1, 91, 1, 91, 1, 91, 1, 92, 
		1, 92, 1, 92, 3, 92, 1056, 8, 92, 1, 93, 1, 93, 1, 93, 1, 93, 1, 93, 1, 
		93, 1, 93, 1, 93, 3, 93, 1066, 8, 93, 1, 94, 1, 94, 1, 94, 1, 94, 1, 94, 
		3, 94, 1073, 8, 94, 1, 95, 1, 95, 5, 95, 1077, 8, 95, 10, 95, 12, 95, 
		1080, 9, 95, 1, 96, 1, 96, 1, 96, 1, 97, 1, 97, 1, 97, 1, 97, 1, 97, 1, 
		97, 1, 97, 1, 97, 3, 97, 1093, 8, 97, 1, 98, 1, 98, 3, 98, 1097, 8, 98, 
		1, 99, 1, 99, 1, 99, 1, 99, 1, 99, 1, 99, 3, 99, 1105, 8, 99, 1, 100, 
		1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 101, 1, 101, 1, 101, 
		1, 101, 1, 101, 1, 101, 3, 101, 1120, 8, 101, 1, 102, 1, 102, 1, 102, 
		1, 102, 1, 102, 3, 102, 1127, 8, 102, 1, 103, 1, 103, 1, 103, 1, 103, 
		1, 103, 1, 103, 1, 103, 1, 103, 1, 103, 1, 103, 1, 103, 1, 103, 1, 103, 
		3, 103, 1142, 8, 103, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 3, 104, 
		1149, 8, 104, 1, 105, 1, 105, 1, 105, 1, 105, 1, 106, 1, 106, 1, 106, 
		1, 106, 1, 106, 1, 106, 1, 106, 1, 106, 5, 106, 1163, 8, 106, 10, 106, 
		12, 106, 1166, 9, 106, 1, 107, 1, 107, 1, 107, 1, 107, 1, 108, 1, 108, 
		1, 108, 1, 108, 1, 108, 3, 108, 1177, 8, 108, 1, 109, 1, 109, 1, 109, 
		1, 109, 1, 110, 1, 110, 1, 110, 1, 110, 1, 111, 1, 111, 1, 111, 1, 111, 
		1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 
		1, 112, 3, 112, 1201, 8, 112, 1, 113, 1, 113, 1, 113, 1, 113, 1, 113, 
		3, 113, 1208, 8, 113, 1, 114, 1, 114, 3, 114, 1212, 8, 114, 1, 115, 1, 
		115, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 1, 
		116, 1, 116, 3, 116, 1226, 8, 116, 1, 117, 1, 117, 1, 117, 1, 117, 1, 
		117, 3, 117, 1233, 8, 117, 1, 118, 1, 118, 3, 118, 1237, 8, 118, 1, 119, 
		1, 119, 1, 120, 1, 120, 1, 121, 1, 121, 1, 121, 1, 122, 1, 122, 1, 122, 
		1, 122, 1, 122, 1, 122, 3, 122, 1252, 8, 122, 1, 123, 1, 123, 3, 123, 
		1256, 8, 123, 1, 124, 1, 124, 3, 124, 1260, 8, 124, 1, 125, 1, 125, 1, 
		125, 1, 125, 1, 125, 1, 125, 1, 125, 1, 126, 1, 126, 3, 126, 1271, 8, 
		126, 1, 127, 1, 127, 1, 127, 3, 127, 1276, 8, 127, 1, 128, 1, 128, 3, 
		128, 1280, 8, 128, 1, 129, 1, 129, 1, 129, 1, 129, 1, 130, 1, 130, 3, 
		130, 1288, 8, 130, 1, 131, 1, 131, 1, 131, 1, 131, 1, 131, 1, 131, 1, 
		131, 1, 131, 5, 131, 1298, 8, 131, 10, 131, 12, 131, 1301, 9, 131, 1, 
		132, 1, 132, 1, 132, 1, 132, 1, 132, 1, 132, 1, 132, 1, 132, 5, 132, 1311, 
		8, 132, 10, 132, 12, 132, 1314, 9, 132, 1, 133, 1, 133, 1, 133, 1, 133, 
		3, 133, 1320, 8, 133, 1, 134, 1, 134, 1, 134, 1, 134, 1, 135, 1, 135, 
		3, 135, 1328, 8, 135, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 
		3, 136, 1336, 8, 136, 1, 137, 1, 137, 1, 137, 1, 137, 1, 137, 1, 137, 
		1, 137, 1, 138, 1, 138, 1, 138, 1, 138, 1, 138, 3, 138, 1350, 8, 138, 
		1, 139, 1, 139, 1, 139, 3, 139, 1355, 8, 139, 1, 140, 1, 140, 1, 141, 
		1, 141, 3, 141, 1361, 8, 141, 1, 142, 1, 142, 1, 143, 1, 143, 1, 143, 
		1, 143, 1, 144, 1, 144, 1, 145, 1, 145, 1, 145, 1, 145, 1, 145, 1, 145, 
		3, 145, 1377, 8, 145, 1, 146, 1, 146, 3, 146, 1381, 8, 146, 1, 147, 1, 
		147, 1, 148, 1, 148, 1, 148, 1, 148, 1, 148, 1, 148, 3, 148, 1391, 8, 
		148, 1, 149, 1, 149, 1, 149, 1, 149, 1, 149, 1, 149, 3, 149, 1399, 8, 
		149, 1, 150, 1, 150, 1, 150, 1, 150, 1, 150, 3, 150, 1406, 8, 150, 1, 
		151, 1, 151, 3, 151, 1410, 8, 151, 1, 152, 1, 152, 1, 152, 3, 152, 1415, 
		8, 152, 1, 153, 1, 153, 1, 153, 1, 153, 1, 153, 1, 153, 1, 153, 1, 153, 
		3, 153, 1425, 8, 153, 1, 154, 1, 154, 1, 154, 1, 154, 1, 154, 3, 154, 
		1432, 8, 154, 1, 155, 1, 155, 5, 155, 1436, 8, 155, 10, 155, 12, 155, 
		1439, 9, 155, 1, 156, 1, 156, 1, 156, 1, 157, 1, 157, 1, 157, 1, 157, 
		1, 157, 3, 157, 1449, 8, 157, 1, 158, 1, 158, 1, 158, 1, 158, 1, 158, 
		1, 158, 5, 158, 1457, 8, 158, 10, 158, 12, 158, 1460, 9, 158, 1, 159, 
		1, 159, 1, 159, 1, 159, 1, 159, 1, 159, 1, 159, 1, 159, 1, 159, 3, 159, 
		1471, 8, 159, 1, 160, 1, 160, 1, 160, 3, 160, 1476, 8, 160, 1, 161, 1, 
		161, 3, 161, 1480, 8, 161, 1, 162, 1, 162, 1, 163, 1, 163, 1, 164, 1, 
		164, 1, 165, 1, 165, 3, 165, 1490, 8, 165, 1, 166, 1, 166, 3, 166, 1494, 
		8, 166, 1, 167, 1, 167, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 
		1, 168, 3, 168, 1505, 8, 168, 1, 169, 1, 169, 1, 170, 1, 170, 1, 171, 
		1, 171, 1, 172, 1, 172, 1, 173, 1, 173, 1, 174, 1, 174, 1, 175, 1, 175, 
		1, 176, 1, 176, 1, 177, 1, 177, 3, 177, 1525, 8, 177, 1, 178, 1, 178, 
		3, 178, 1529, 8, 178, 1, 179, 1, 179, 1, 180, 1, 180, 1, 181, 1, 181, 
		1, 182, 1, 182, 1, 183, 1, 183, 1, 184, 1, 184, 1, 185, 1, 185, 1, 186, 
		1, 186, 1, 187, 1, 187, 1, 188, 1, 188, 3, 188, 1551, 8, 188, 1, 189, 
		1, 189, 1, 190, 1, 190, 3, 190, 1557, 8, 190, 1, 191, 1, 191, 1, 192, 
		1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 1, 192, 3, 192, 1569, 
		8, 192, 1, 193, 1, 193, 1, 193, 1, 193, 1, 193, 3, 193, 1576, 8, 193, 
		1, 194, 1, 194, 3, 194, 1580, 8, 194, 1, 195, 1, 195, 1, 195, 1, 195, 
		1, 195, 1, 195, 1, 195, 1, 195, 1, 196, 1, 196, 1, 197, 1, 197, 1, 197, 
		1, 197, 1, 197, 1, 197, 1, 197, 1, 197, 1, 198, 1, 198, 1, 199, 1, 199, 
		1, 199, 3, 199, 1605, 8, 199, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 
		1, 201, 1, 201, 1, 201, 3, 201, 1615, 8, 201, 1, 202, 1, 202, 1, 202, 
		1, 202, 1, 202, 1, 203, 1, 203, 1, 204, 1, 204, 1, 204, 1, 204, 1, 204, 
		1, 204, 1, 204, 1, 204, 1, 205, 1, 205, 1, 206, 1, 206, 1, 206, 1, 206, 
		1, 206, 3, 206, 1639, 8, 206, 1, 207, 1, 207, 5, 207, 1643, 8, 207, 10, 
		207, 12, 207, 1646, 9, 207, 1, 208, 1, 208, 1, 208, 1, 209, 1, 209, 1, 
		209, 1, 210, 1, 210, 1, 210, 3, 210, 1657, 8, 210, 1, 211, 1, 211, 1, 
		211, 3, 211, 1662, 8, 211, 1, 212, 1, 212, 1, 213, 1, 213, 1, 213, 1, 
		213, 1, 213, 1, 214, 1, 214, 1, 214, 1, 214, 1, 214, 1, 214, 1, 214, 1, 
		214, 3, 214, 1679, 8, 214, 1, 215, 1, 215, 1, 215, 1, 215, 1, 215, 3, 
		215, 1686, 8, 215, 1, 216, 1, 216, 1, 216, 1, 216, 1, 216, 3, 216, 1693, 
		8, 216, 1, 217, 1, 217, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 
		3, 218, 1703, 8, 218, 1, 219, 1, 219, 1, 219, 1, 219, 1, 219, 1, 219, 
		3, 219, 1711, 8, 219, 1, 220, 1, 220, 1, 220, 1, 220, 1, 220, 1, 221, 
		1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 
		1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 1, 221, 
		1, 221, 3, 221, 1738, 8, 221, 1, 222, 1, 222, 1, 222, 1, 222, 1, 222, 
		3, 222, 1745, 8, 222, 1, 223, 1, 223, 5, 223, 1749, 8, 223, 10, 223, 12, 
		223, 1752, 9, 223, 1, 224, 1, 224, 1, 224, 1, 225, 1, 225, 3, 225, 1759, 
		8, 225, 1, 226, 1, 226, 1, 227, 1, 227, 1, 228, 1, 228, 1, 229, 1, 229, 
		1, 230, 1, 230, 1, 231, 1, 231, 1, 231, 0, 11, 34, 36, 38, 110, 112, 130, 
		132, 212, 262, 264, 316, 232, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 
		24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 
		60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 
		96, 98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122, 124, 
		126, 128, 130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 152, 
		154, 156, 158, 160, 162, 164, 166, 168, 170, 172, 174, 176, 178, 180, 
		182, 184, 186, 188, 190, 192, 194, 196, 198, 200, 202, 204, 206, 208, 
		210, 212, 214, 216, 218, 220, 222, 224, 226, 228, 230, 232, 234, 236, 
		238, 240, 242, 244, 246, 248, 250, 252, 254, 256, 258, 260, 262, 264, 
		266, 268, 270, 272, 274, 276, 278, 280, 282, 284, 286, 288, 290, 292, 
		294, 296, 298, 300, 302, 304, 306, 308, 310, 312, 314, 316, 318, 320, 
		322, 324, 326, 328, 330, 332, 334, 336, 338, 340, 342, 344, 346, 348, 
		350, 352, 354, 356, 358, 360, 362, 364, 366, 368, 370, 372, 374, 376, 
		378, 380, 382, 384, 386, 388, 390, 392, 394, 396, 398, 400, 402, 404, 
		406, 408, 410, 412, 414, 416, 418, 420, 422, 424, 426, 428, 430, 432, 
		434, 436, 438, 440, 442, 444, 446, 448, 450, 452, 454, 456, 458, 460, 
		462, 0, 7, 1, 0, 28, 30, 1, 0, 31, 32, 2, 0, 26, 26, 34, 34, 2, 0, 10, 
		10, 89, 89, 1, 0, 45, 49, 2, 0, 65, 66, 71, 71, 3, 0, 72, 72, 79, 79, 
		82, 82, 1744, 0, 467, 1, 0, 0, 0, 2, 474, 1, 0, 0, 0, 4, 482, 1, 0, 0, 
		0, 6, 484, 1, 0, 0, 0, 8, 493, 1, 0, 0, 0, 10, 495, 1, 0, 0, 0, 12, 504, 
		1, 0, 0, 0, 14, 513, 1, 0, 0, 0, 16, 522, 1, 0, 0, 0, 18, 531, 1, 0, 0, 
		0, 20, 545, 1, 0, 0, 0, 22, 551, 1, 0, 0, 0, 24, 556, 1, 0, 0, 0, 26, 
		564, 1, 0, 0, 0, 28, 569, 1, 0, 0, 0, 30, 571, 1, 0, 0, 0, 32, 578, 1, 
		0, 0, 0, 34, 580, 1, 0, 0, 0, 36, 593, 1, 0, 0, 0, 38, 606, 1, 0, 0, 0, 
		40, 622, 1, 0, 0, 0, 42, 626, 1, 0, 0, 0, 44, 635, 1, 0, 0, 0, 46, 637, 
		1, 0, 0, 0, 48, 640, 1, 0, 0, 0, 50, 651, 1, 0, 0, 0, 52, 653, 1, 0, 0, 
		0, 54, 659, 1, 0, 0, 0, 56, 661, 1, 0, 0, 0, 58, 664, 1, 0, 0, 0, 60, 
		672, 1, 0, 0, 0, 62, 676, 1, 0, 0, 0, 64, 686, 1, 0, 0, 0, 66, 690, 1, 
		0, 0, 0, 68, 692, 1, 0, 0, 0, 70, 696, 1, 0, 0, 0, 72, 698, 1, 0, 0, 0, 
		74, 711, 1, 0, 0, 0, 76, 718, 1, 0, 0, 0, 78, 725, 1, 0, 0, 0, 80, 727, 
		1, 0, 0, 0, 82, 736, 1, 0, 0, 0, 84, 744, 1, 0, 0, 0, 86, 751, 1, 0, 0, 
		0, 88, 758, 1, 0, 0, 0, 90, 775, 1, 0, 0, 0, 92, 777, 1, 0, 0, 0, 94, 
		779, 1, 0, 0, 0, 96, 786, 1, 0, 0, 0, 98, 797, 1, 0, 0, 0, 100, 802, 1, 
		0, 0, 0, 102, 804, 1, 0, 0, 0, 104, 806, 1, 0, 0, 0, 106, 811, 1, 0, 0, 
		0, 108, 821, 1, 0, 0, 0, 110, 823, 1, 0, 0, 0, 112, 836, 1, 0, 0, 0, 114, 
		849, 1, 0, 0, 0, 116, 853, 1, 0, 0, 0, 118, 857, 1, 0, 0, 0, 120, 864, 
		1, 0, 0, 0, 122, 872, 1, 0, 0, 0, 124, 876, 1, 0, 0, 0, 126, 878, 1, 0, 
		0, 0, 128, 884, 1, 0, 0, 0, 130, 886, 1, 0, 0, 0, 132, 899, 1, 0, 0, 0, 
		134, 915, 1, 0, 0, 0, 136, 919, 1, 0, 0, 0, 138, 928, 1, 0, 0, 0, 140, 
		930, 1, 0, 0, 0, 142, 932, 1, 0, 0, 0, 144, 944, 1, 0, 0, 0, 146, 948, 
		1, 0, 0, 0, 148, 950, 1, 0, 0, 0, 150, 956, 1, 0, 0, 0, 152, 958, 1, 0, 
		0, 0, 154, 961, 1, 0, 0, 0, 156, 968, 1, 0, 0, 0, 158, 976, 1, 0, 0, 0, 
		160, 978, 1, 0, 0, 0, 162, 988, 1, 0, 0, 0, 164, 990, 1, 0, 0, 0, 166, 
		1000, 1, 0, 0, 0, 168, 1002, 1, 0, 0, 0, 170, 1015, 1, 0, 0, 0, 172, 1022, 
		1, 0, 0, 0, 174, 1029, 1, 0, 0, 0, 176, 1031, 1, 0, 0, 0, 178, 1037, 1, 
		0, 0, 0, 180, 1044, 1, 0, 0, 0, 182, 1046, 1, 0, 0, 0, 184, 1055, 1, 0, 
		0, 0, 186, 1065, 1, 0, 0, 0, 188, 1072, 1, 0, 0, 0, 190, 1074, 1, 0, 0, 
		0, 192, 1081, 1, 0, 0, 0, 194, 1092, 1, 0, 0, 0, 196, 1096, 1, 0, 0, 0, 
		198, 1104, 1, 0, 0, 0, 200, 1106, 1, 0, 0, 0, 202, 1119, 1, 0, 0, 0, 204, 
		1126, 1, 0, 0, 0, 206, 1141, 1, 0, 0, 0, 208, 1148, 1, 0, 0, 0, 210, 1150, 
		1, 0, 0, 0, 212, 1154, 1, 0, 0, 0, 214, 1167, 1, 0, 0, 0, 216, 1176, 1, 
		0, 0, 0, 218, 1178, 1, 0, 0, 0, 220, 1182, 1, 0, 0, 0, 222, 1186, 1, 0, 
		0, 0, 224, 1200, 1, 0, 0, 0, 226, 1207, 1, 0, 0, 0, 228, 1211, 1, 0, 0, 
		0, 230, 1213, 1, 0, 0, 0, 232, 1225, 1, 0, 0, 0, 234, 1232, 1, 0, 0, 0, 
		236, 1236, 1, 0, 0, 0, 238, 1238, 1, 0, 0, 0, 240, 1240, 1, 0, 0, 0, 242, 
		1242, 1, 0, 0, 0, 244, 1251, 1, 0, 0, 0, 246, 1255, 1, 0, 0, 0, 248, 1259, 
		1, 0, 0, 0, 250, 1261, 1, 0, 0, 0, 252, 1270, 1, 0, 0, 0, 254, 1275, 1, 
		0, 0, 0, 256, 1279, 1, 0, 0, 0, 258, 1281, 1, 0, 0, 0, 260, 1287, 1, 0, 
		0, 0, 262, 1289, 1, 0, 0, 0, 264, 1302, 1, 0, 0, 0, 266, 1319, 1, 0, 0, 
		0, 268, 1321, 1, 0, 0, 0, 270, 1327, 1, 0, 0, 0, 272, 1335, 1, 0, 0, 0, 
		274, 1337, 1, 0, 0, 0, 276, 1349, 1, 0, 0, 0, 278, 1354, 1, 0, 0, 0, 280, 
		1356, 1, 0, 0, 0, 282, 1360, 1, 0, 0, 0, 284, 1362, 1, 0, 0, 0, 286, 1364, 
		1, 0, 0, 0, 288, 1368, 1, 0, 0, 0, 290, 1376, 1, 0, 0, 0, 292, 1380, 1, 
		0, 0, 0, 294, 1382, 1, 0, 0, 0, 296, 1390, 1, 0, 0, 0, 298, 1398, 1, 0, 
		0, 0, 300, 1405, 1, 0, 0, 0, 302, 1409, 1, 0, 0, 0, 304, 1414, 1, 0, 0, 
		0, 306, 1424, 1, 0, 0, 0, 308, 1431, 1, 0, 0, 0, 310, 1433, 1, 0, 0, 0, 
		312, 1440, 1, 0, 0, 0, 314, 1448, 1, 0, 0, 0, 316, 1450, 1, 0, 0, 0, 318, 
		1470, 1, 0, 0, 0, 320, 1475, 1, 0, 0, 0, 322, 1479, 1, 0, 0, 0, 324, 1481, 
		1, 0, 0, 0, 326, 1483, 1, 0, 0, 0, 328, 1485, 1, 0, 0, 0, 330, 1489, 1, 
		0, 0, 0, 332, 1493, 1, 0, 0, 0, 334, 1495, 1, 0, 0, 0, 336, 1504, 1, 0, 
		0, 0, 338, 1506, 1, 0, 0, 0, 340, 1508, 1, 0, 0, 0, 342, 1510, 1, 0, 0, 
		0, 344, 1512, 1, 0, 0, 0, 346, 1514, 1, 0, 0, 0, 348, 1516, 1, 0, 0, 0, 
		350, 1518, 1, 0, 0, 0, 352, 1520, 1, 0, 0, 0, 354, 1524, 1, 0, 0, 0, 356, 
		1528, 1, 0, 0, 0, 358, 1530, 1, 0, 0, 0, 360, 1532, 1, 0, 0, 0, 362, 1534, 
		1, 0, 0, 0, 364, 1536, 1, 0, 0, 0, 366, 1538, 1, 0, 0, 0, 368, 1540, 1, 
		0, 0, 0, 370, 1542, 1, 0, 0, 0, 372, 1544, 1, 0, 0, 0, 374, 1546, 1, 0, 
		0, 0, 376, 1550, 1, 0, 0, 0, 378, 1552, 1, 0, 0, 0, 380, 1556, 1, 0, 0, 
		0, 382, 1558, 1, 0, 0, 0, 384, 1568, 1, 0, 0, 0, 386, 1575, 1, 0, 0, 0, 
		388, 1579, 1, 0, 0, 0, 390, 1581, 1, 0, 0, 0, 392, 1589, 1, 0, 0, 0, 394, 
		1591, 1, 0, 0, 0, 396, 1599, 1, 0, 0, 0, 398, 1604, 1, 0, 0, 0, 400, 1606, 
		1, 0, 0, 0, 402, 1614, 1, 0, 0, 0, 404, 1616, 1, 0, 0, 0, 406, 1621, 1, 
		0, 0, 0, 408, 1623, 1, 0, 0, 0, 410, 1631, 1, 0, 0, 0, 412, 1638, 1, 0, 
		0, 0, 414, 1640, 1, 0, 0, 0, 416, 1647, 1, 0, 0, 0, 418, 1650, 1, 0, 0, 
		0, 420, 1656, 1, 0, 0, 0, 422, 1661, 1, 0, 0, 0, 424, 1663, 1, 0, 0, 0, 
		426, 1665, 1, 0, 0, 0, 428, 1678, 1, 0, 0, 0, 430, 1685, 1, 0, 0, 0, 432, 
		1692, 1, 0, 0, 0, 434, 1694, 1, 0, 0, 0, 436, 1702, 1, 0, 0, 0, 438, 1710, 
		1, 0, 0, 0, 440, 1712, 1, 0, 0, 0, 442, 1737, 1, 0, 0, 0, 444, 1744, 1, 
		0, 0, 0, 446, 1746, 1, 0, 0, 0, 448, 1753, 1, 0, 0, 0, 450, 1758, 1, 0, 
		0, 0, 452, 1760, 1, 0, 0, 0, 454, 1762, 1, 0, 0, 0, 456, 1764, 1, 0, 0, 
		0, 458, 1766, 1, 0, 0, 0, 460, 1768, 1, 0, 0, 0, 462, 1770, 1, 0, 0, 0, 
		464, 466, 3, 2, 1, 0, 465, 464, 1, 0, 0, 0, 466, 469, 1, 0, 0, 0, 467, 
		465, 1, 0, 0, 0, 467, 468, 1, 0, 0, 0, 468, 470, 1, 0, 0, 0, 469, 467, 
		1, 0, 0, 0, 470, 471, 5, 0, 0, 1, 471, 1, 1, 0, 0, 0, 472, 475, 3, 4, 
		2, 0, 473, 475, 3, 426, 213, 0, 474, 472, 1, 0, 0, 0, 474, 473, 1, 0, 
		0, 0, 475, 3, 1, 0, 0, 0, 476, 483, 3, 10, 5, 0, 477, 483, 3, 12, 6, 0, 
		478, 483, 3, 14, 7, 0, 479, 483, 3, 16, 8, 0, 480, 483, 3, 18, 9, 0, 481, 
		483, 3, 6, 3, 0, 482, 476, 1, 0, 0, 0, 482, 477, 1, 0, 0, 0, 482, 478, 
		1, 0, 0, 0, 482, 479, 1, 0, 0, 0, 482, 480, 1, 0, 0, 0, 482, 481, 1, 0, 
		0, 0, 483, 5, 1, 0, 0, 0, 484, 485, 5, 1, 0, 0, 485, 486, 3, 450, 225, 
		0, 486, 487, 5, 2, 0, 0, 487, 488, 3, 22, 11, 0, 488, 489, 5, 2, 0, 0, 
		489, 490, 3, 8, 4, 0, 490, 491, 3, 20, 10, 0, 491, 492, 5, 3, 0, 0, 492, 
		7, 1, 0, 0, 0, 493, 494, 3, 252, 126, 0, 494, 9, 1, 0, 0, 0, 495, 496, 
		5, 4, 0, 0, 496, 497, 3, 450, 225, 0, 497, 498, 5, 2, 0, 0, 498, 499, 
		3, 22, 11, 0, 499, 500, 5, 2, 0, 0, 500, 501, 3, 24, 12, 0, 501, 502, 
		3, 20, 10, 0, 502, 503, 5, 3, 0, 0, 503, 11, 1, 0, 0, 0, 504, 505, 5, 
		5, 0, 0, 505, 506, 3, 450, 225, 0, 506, 507, 5, 2, 0, 0, 507, 508, 3, 
		22, 11, 0, 508, 509, 5, 2, 0, 0, 509, 510, 3, 120, 60, 0, 510, 511, 3, 
		20, 10, 0, 511, 512, 5, 3, 0, 0, 512, 13, 1, 0, 0, 0, 513, 514, 5, 6, 
		0, 0, 514, 515, 3, 450, 225, 0, 515, 516, 5, 2, 0, 0, 516, 517, 3, 22, 
		11, 0, 517, 518, 5, 2, 0, 0, 518, 519, 3, 246, 123, 0, 519, 520, 3, 20, 
		10, 0, 520, 521, 5, 3, 0, 0, 521, 15, 1, 0, 0, 0, 522, 523, 5, 7, 0, 0, 
		523, 524, 3, 450, 225, 0, 524, 525, 5, 2, 0, 0, 525, 526, 3, 22, 11, 0, 
		526, 527, 5, 2, 0, 0, 527, 528, 3, 252, 126, 0, 528, 529, 3, 20, 10, 0, 
		529, 530, 5, 3, 0, 0, 530, 17, 1, 0, 0, 0, 531, 532, 5, 8, 0, 0, 532, 
		533, 3, 450, 225, 0, 533, 534, 5, 2, 0, 0, 534, 535, 3, 22, 11, 0, 535, 
		536, 5, 2, 0, 0, 536, 537, 3, 314, 157, 0, 537, 538, 3, 20, 10, 0, 538, 
		539, 5, 3, 0, 0, 539, 19, 1, 0, 0, 0, 540, 541, 5, 2, 0, 0, 541, 542, 
		3, 384, 192, 0, 542, 543, 3, 422, 211, 0, 543, 546, 1, 0, 0, 0, 544, 546, 
		3, 462, 231, 0, 545, 540, 1, 0, 0, 0, 545, 544, 1, 0, 0, 0, 546, 21, 1, 
		0, 0, 0, 547, 552, 5, 71, 0, 0, 548, 549, 5, 71, 0, 0, 549, 550, 5, 9, 
		0, 0, 550, 552, 3, 436, 218, 0, 551, 547, 1, 0, 0, 0, 551, 548, 1, 0, 
		0, 0, 552, 23, 1, 0, 0, 0, 553, 557, 3, 26, 13, 0, 554, 557, 3, 98, 49, 
		0, 555, 557, 3, 114, 57, 0, 556, 553, 1, 0, 0, 0, 556, 554, 1, 0, 0, 0, 
		556, 555, 1, 0, 0, 0, 557, 25, 1, 0, 0, 0, 558, 565, 3, 44, 22, 0, 559, 
		565, 3, 54, 27, 0, 560, 565, 3, 28, 14, 0, 561, 565, 3, 68, 34, 0, 562, 
		565, 3, 116, 58, 0, 563, 565, 3, 118, 59, 0, 564, 558, 1, 0, 0, 0, 564, 
		559, 1, 0, 0, 0, 564, 560, 1, 0, 0, 0, 564, 561, 1, 0, 0, 0, 564, 562, 
		1, 0, 0, 0, 564, 563, 1, 0, 0, 0, 565, 27, 1, 0, 0, 0, 566, 570, 3, 30, 
		15, 0, 567, 570, 3, 32, 16, 0, 568, 570, 3, 106, 53, 0, 569, 566, 1, 0, 
		0, 0, 569, 567, 1, 0, 0, 0, 569, 568, 1, 0, 0, 0, 570, 29, 1, 0, 0, 0, 
		571, 572, 3, 40, 20, 0, 572, 573, 3, 336, 168, 0, 573, 574, 3, 40, 20, 
		0, 574, 31, 1, 0, 0, 0, 575, 579, 3, 34, 17, 0, 576, 579, 3, 36, 18, 0, 
		577, 579, 3, 38, 19, 0, 578, 575, 1, 0, 0, 0, 578, 576, 1, 0, 0, 0, 578, 
		577, 1, 0, 0, 0, 579, 33, 1, 0, 0, 0, 580, 581, 6, 17, -1, 0, 581, 582, 
		3, 40, 20, 0, 582, 583, 5, 89, 0, 0, 583, 584, 3, 40, 20, 0, 584, 590, 
		1, 0, 0, 0, 585, 586, 10, 1, 0, 0, 586, 587, 5, 89, 0, 0, 587, 589, 3, 
		40, 20, 0, 588, 585, 1, 0, 0, 0, 589, 592, 1, 0, 0, 0, 590, 588, 1, 0, 
		0, 0, 590, 591, 1, 0, 0, 0, 591, 35, 1, 0, 0, 0, 592, 590, 1, 0, 0, 0, 
		593, 594, 6, 18, -1, 0, 594, 595, 3, 40, 20, 0, 595, 596, 5, 10, 0, 0, 
		596, 597, 3, 40, 20, 0, 597, 603, 1, 0, 0, 0, 598, 599, 10, 1, 0, 0, 599, 
		600, 5, 10, 0, 0, 600, 602, 3, 40, 20, 0, 601, 598, 1, 0, 0, 0, 602, 605, 
		1, 0, 0, 0, 603, 601, 1, 0, 0, 0, 603, 604, 1, 0, 0, 0, 604, 37, 1, 0, 
		0, 0, 605, 603, 1, 0, 0, 0, 606, 607, 6, 19, -1, 0, 607, 608, 3, 40, 20, 
		0, 608, 609, 5, 11, 0, 0, 609, 610, 3, 40, 20, 0, 610, 616, 1, 0, 0, 0, 
		611, 612, 10, 1, 0, 0, 612, 613, 5, 11, 0, 0, 613, 615, 3, 40, 20, 0, 
		614, 611, 1, 0, 0, 0, 615, 618, 1, 0, 0, 0, 616, 614, 1, 0, 0, 0, 616, 
		617, 1, 0, 0, 0, 617, 39, 1, 0, 0, 0, 618, 616, 1, 0, 0, 0, 619, 623, 
		3, 44, 22, 0, 620, 623, 3, 54, 27, 0, 621, 623, 3, 68, 34, 0, 622, 619, 
		1, 0, 0, 0, 622, 620, 1, 0, 0, 0, 622, 621, 1, 0, 0, 0, 623, 41, 1, 0, 
		0, 0, 624, 627, 3, 44, 22, 0, 625, 627, 3, 56, 28, 0, 626, 624, 1, 0, 
		0, 0, 626, 625, 1, 0, 0, 0, 627, 43, 1, 0, 0, 0, 628, 636, 3, 46, 23, 
		0, 629, 636, 3, 60, 30, 0, 630, 636, 3, 382, 191, 0, 631, 632, 5, 12, 
		0, 0, 632, 633, 3, 26, 13, 0, 633, 634, 5, 13, 0, 0, 634, 636, 1, 0, 0, 
		0, 635, 628, 1, 0, 0, 0, 635, 629, 1, 0, 0, 0, 635, 630, 1, 0, 0, 0, 635, 
		631, 1, 0, 0, 0, 636, 45, 1, 0, 0, 0, 637, 638, 3, 48, 24, 0, 638, 639, 
		3, 40, 20, 0, 639, 47, 1, 0, 0, 0, 640, 641, 3, 320, 160, 0, 641, 642, 
		5, 14, 0, 0, 642, 643, 3, 50, 25, 0, 643, 644, 5, 15, 0, 0, 644, 645, 
		5, 16, 0, 0, 645, 49, 1, 0, 0, 0, 646, 652, 3, 52, 26, 0, 647, 648, 3, 
		52, 26, 0, 648, 649, 5, 2, 0, 0, 649, 650, 3, 50, 25, 0, 650, 652, 1, 
		0, 0, 0, 651, 646, 1, 0, 0, 0, 651, 647, 1, 0, 0, 0, 652, 51, 1, 0, 0, 
		0, 653, 654, 3, 382, 191, 0, 654, 655, 5, 16, 0, 0, 655, 656, 3, 100, 
		50, 0, 656, 53, 1, 0, 0, 0, 657, 660, 3, 56, 28, 0, 658, 660, 3, 58, 29, 
		0, 659, 657, 1, 0, 0, 0, 659, 658, 1, 0, 0, 0, 660, 55, 1, 0, 0, 0, 661, 
		662, 3, 322, 161, 0, 662, 663, 3, 42, 21, 0, 663, 57, 1, 0, 0, 0, 664, 
		665, 3, 84, 42, 0, 665, 666, 3, 362, 181, 0, 666, 667, 3, 84, 42, 0, 667, 
		59, 1, 0, 0, 0, 668, 673, 3, 62, 31, 0, 669, 673, 3, 64, 32, 0, 670, 673, 
		3, 70, 35, 0, 671, 673, 3, 90, 45, 0, 672, 668, 1, 0, 0, 0, 672, 669, 
		1, 0, 0, 0, 672, 670, 1, 0, 0, 0, 672, 671, 1, 0, 0, 0, 673, 61, 1, 0, 
		0, 0, 674, 677, 3, 364, 182, 0, 675, 677, 3, 88, 44, 0, 676, 674, 1, 0, 
		0, 0, 676, 675, 1, 0, 0, 0, 677, 63, 1, 0, 0, 0, 678, 687, 3, 368, 184, 
		0, 679, 687, 3, 66, 33, 0, 680, 681, 5, 12, 0, 0, 681, 682, 3, 86, 43, 
		0, 682, 683, 5, 13, 0, 0, 683, 687, 1, 0, 0, 0, 684, 687, 3, 224, 112, 
		0, 685, 687, 3, 72, 36, 0, 686, 678, 1, 0, 0, 0, 686, 679, 1, 0, 0, 0, 
		686, 680, 1, 0, 0, 0, 686, 684, 1, 0, 0, 0, 686, 685, 1, 0, 0, 0, 687, 
		65, 1, 0, 0, 0, 688, 691, 3, 380, 190, 0, 689, 691, 3, 378, 189, 0, 690, 
		688, 1, 0, 0, 0, 690, 689, 1, 0, 0, 0, 691, 67, 1, 0, 0, 0, 692, 693, 
		3, 84, 42, 0, 693, 694, 3, 358, 179, 0, 694, 695, 3, 84, 42, 0, 695, 69, 
		1, 0, 0, 0, 696, 697, 3, 372, 186, 0, 697, 71, 1, 0, 0, 0, 698, 699, 5, 
		17, 0, 0, 699, 700, 3, 74, 37, 0, 700, 701, 5, 2, 0, 0, 701, 702, 3, 78, 
		39, 0, 702, 703, 5, 2, 0, 0, 703, 704, 3, 26, 13, 0, 704, 705, 5, 13, 
		0, 0, 705, 73, 1, 0, 0, 0, 706, 712, 3, 98, 49, 0, 707, 708, 5, 14, 0, 
		0, 708, 709, 3, 76, 38, 0, 709, 710, 5, 15, 0, 0, 710, 712, 1, 0, 0, 0, 
		711, 706, 1, 0, 0, 0, 711, 707, 1, 0, 0, 0, 712, 75, 1, 0, 0, 0, 713, 
		719, 3, 98, 49, 0, 714, 715, 3, 98, 49, 0, 715, 716, 5, 2, 0, 0, 716, 
		717, 3, 76, 38, 0, 717, 719, 1, 0, 0, 0, 718, 713, 1, 0, 0, 0, 718, 714, 
		1, 0, 0, 0, 719, 77, 1, 0, 0, 0, 720, 726, 3, 80, 40, 0, 721, 722, 5, 
		14, 0, 0, 722, 723, 3, 82, 41, 0, 723, 724, 5, 15, 0, 0, 724, 726, 1, 
		0, 0, 0, 725, 720, 1, 0, 0, 0, 725, 721, 1, 0, 0, 0, 726, 79, 1, 0, 0, 
		0, 727, 728, 3, 26, 13, 0, 728, 729, 3, 344, 172, 0, 729, 730, 3, 26, 
		13, 0, 730, 81, 1, 0, 0, 0, 731, 737, 3, 80, 40, 0, 732, 733, 3, 80, 40, 
		0, 733, 734, 5, 2, 0, 0, 734, 735, 3, 82, 41, 0, 735, 737, 1, 0, 0, 0, 
		736, 731, 1, 0, 0, 0, 736, 732, 1, 0, 0, 0, 737, 83, 1, 0, 0, 0, 738, 
		745, 3, 60, 30, 0, 739, 745, 3, 382, 191, 0, 740, 741, 5, 12, 0, 0, 741, 
		742, 3, 26, 13, 0, 742, 743, 5, 13, 0, 0, 743, 745, 1, 0, 0, 0, 744, 738, 
		1, 0, 0, 0, 744, 739, 1, 0, 0, 0, 744, 740, 1, 0, 0, 0, 745, 85, 1, 0, 
		0, 0, 746, 752, 3, 336, 168, 0, 747, 752, 3, 338, 169, 0, 748, 752, 3, 
		360, 180, 0, 749, 752, 3, 362, 181, 0, 750, 752, 3, 322, 161, 0, 751, 
		746, 1, 0, 0, 0, 751, 747, 1, 0, 0, 0, 751, 748, 1, 0, 0, 0, 751, 749, 
		1, 0, 0, 0, 751, 750, 1, 0, 0, 0, 752, 87, 1, 0, 0, 0, 753, 759, 5, 18, 
		0, 0, 754, 755, 5, 14, 0, 0, 755, 756, 3, 94, 47, 0, 756, 757, 5, 15, 
		0, 0, 757, 759, 1, 0, 0, 0, 758, 753, 1, 0, 0, 0, 758, 754, 1, 0, 0, 0, 
		759, 89, 1, 0, 0, 0, 760, 761, 3, 366, 183, 0, 761, 762, 5, 12, 0, 0, 
		762, 763, 3, 92, 46, 0, 763, 764, 5, 13, 0, 0, 764, 776, 1, 0, 0, 0, 765, 
		766, 3, 370, 185, 0, 766, 767, 5, 12, 0, 0, 767, 768, 3, 92, 46, 0, 768, 
		769, 5, 13, 0, 0, 769, 776, 1, 0, 0, 0, 770, 771, 3, 374, 187, 0, 771, 
		772, 5, 12, 0, 0, 772, 773, 3, 92, 46, 0, 773, 774, 5, 13, 0, 0, 774, 
		776, 1, 0, 0, 0, 775, 760, 1, 0, 0, 0, 775, 765, 1, 0, 0, 0, 775, 770, 
		1, 0, 0, 0, 776, 91, 1, 0, 0, 0, 777, 778, 3, 94, 47, 0, 778, 93, 1, 0, 
		0, 0, 779, 783, 3, 26, 13, 0, 780, 782, 3, 96, 48, 0, 781, 780, 1, 0, 
		0, 0, 782, 785, 1, 0, 0, 0, 783, 781, 1, 0, 0, 0, 783, 784, 1, 0, 0, 0, 
		784, 95, 1, 0, 0, 0, 785, 783, 1, 0, 0, 0, 786, 787, 5, 2, 0, 0, 787, 
		788, 3, 26, 13, 0, 788, 97, 1, 0, 0, 0, 789, 790, 3, 356, 178, 0, 790, 
		791, 5, 16, 0, 0, 791, 792, 3, 100, 50, 0, 792, 798, 1, 0, 0, 0, 793, 
		794, 5, 12, 0, 0, 794, 795, 3, 98, 49, 0, 795, 796, 5, 13, 0, 0, 796, 
		798, 1, 0, 0, 0, 797, 789, 1, 0, 0, 0, 797, 793, 1, 0, 0, 0, 798, 99, 
		1, 0, 0, 0, 799, 803, 3, 102, 51, 0, 800, 803, 3, 108, 54, 0, 801, 803, 
		3, 104, 52, 0, 802, 799, 1, 0, 0, 0, 802, 800, 1, 0, 0, 0, 802, 801, 1, 
		0, 0, 0, 803, 101, 1, 0, 0, 0, 804, 805, 3, 44, 22, 0, 805, 103, 1, 0, 
		0, 0, 806, 807, 3, 38, 19, 0, 807, 105, 1, 0, 0, 0, 808, 812, 3, 108, 
		54, 0, 809, 812, 3, 110, 55, 0, 810, 812, 3, 112, 56, 0, 811, 808, 1, 
		0, 0, 0, 811, 809, 1, 0, 0, 0, 811, 810, 1, 0, 0, 0, 812, 107, 1, 0, 0, 
		0, 813, 814, 3, 102, 51, 0, 814, 815, 5, 92, 0, 0, 815, 816, 3, 102, 51, 
		0, 816, 822, 1, 0, 0, 0, 817, 818, 3, 102, 51, 0, 818, 819, 5, 92, 0, 
		0, 819, 820, 3, 108, 54, 0, 820, 822, 1, 0, 0, 0, 821, 813, 1, 0, 0, 0, 
		821, 817, 1, 0, 0, 0, 822, 109, 1, 0, 0, 0, 823, 824, 6, 55, -1, 0, 824, 
		825, 3, 102, 51, 0, 825, 826, 5, 90, 0, 0, 826, 827, 3, 102, 51, 0, 827, 
		833, 1, 0, 0, 0, 828, 829, 10, 1, 0, 0, 829, 830, 5, 90, 0, 0, 830, 832, 
		3, 102, 51, 0, 831, 828, 1, 0, 0, 0, 832, 835, 1, 0, 0, 0, 833, 831, 1, 
		0, 0, 0, 833, 834, 1, 0, 0, 0, 834, 111, 1, 0, 0, 0, 835, 833, 1, 0, 0, 
		0, 836, 837, 6, 56, -1, 0, 837, 838, 3, 102, 51, 0, 838, 839, 5, 91, 0, 
		0, 839, 840, 3, 102, 51, 0, 840, 846, 1, 0, 0, 0, 841, 842, 10, 1, 0, 
		0, 842, 843, 5, 91, 0, 0, 843, 845, 3, 102, 51, 0, 844, 841, 1, 0, 0, 
		0, 845, 848, 1, 0, 0, 0, 846, 844, 1, 0, 0, 0, 846, 847, 1, 0, 0, 0, 847, 
		113, 1, 0, 0, 0, 848, 846, 1, 0, 0, 0, 849, 850, 3, 356, 178, 0, 850, 
		851, 3, 328, 164, 0, 851, 852, 3, 354, 177, 0, 852, 115, 1, 0, 0, 0, 853, 
		854, 3, 60, 30, 0, 854, 855, 3, 346, 173, 0, 855, 856, 3, 26, 13, 0, 856, 
		117, 1, 0, 0, 0, 857, 858, 3, 88, 44, 0, 858, 859, 3, 342, 171, 0, 859, 
		860, 3, 88, 44, 0, 860, 119, 1, 0, 0, 0, 861, 865, 3, 122, 61, 0, 862, 
		865, 3, 194, 97, 0, 863, 865, 3, 218, 109, 0, 864, 861, 1, 0, 0, 0, 864, 
		862, 1, 0, 0, 0, 864, 863, 1, 0, 0, 0, 865, 121, 1, 0, 0, 0, 866, 873, 
		3, 138, 69, 0, 867, 873, 3, 150, 75, 0, 868, 873, 3, 124, 62, 0, 869, 
		873, 3, 164, 82, 0, 870, 873, 3, 220, 110, 0, 871, 873, 3, 222, 111, 0, 
		872, 866, 1, 0, 0, 0, 872, 867, 1, 0, 0, 0, 872, 868, 1, 0, 0, 0, 872, 
		869, 1, 0, 0, 0, 872, 870, 1, 0, 0, 0, 872, 871, 1, 0, 0, 0, 873, 123, 
		1, 0, 0, 0, 874, 877, 3, 126, 63, 0, 875, 877, 3, 128, 64, 0, 876, 874, 
		1, 0, 0, 0, 876, 875, 1, 0, 0, 0, 877, 125, 1, 0, 0, 0, 878, 879, 3, 134, 
		67, 0, 879, 880, 3, 336, 168, 0, 880, 881, 3, 134, 67, 0, 881, 127, 1, 
		0, 0, 0, 882, 885, 3, 130, 65, 0, 883, 885, 3, 132, 66, 0, 884, 882, 1, 
		0, 0, 0, 884, 883, 1, 0, 0, 0, 885, 129, 1, 0, 0, 0, 886, 887, 6, 65, 
		-1, 0, 887, 888, 3, 134, 67, 0, 888, 889, 5, 89, 0, 0, 889, 890, 3, 134, 
		67, 0, 890, 896, 1, 0, 0, 0, 891, 892, 10, 1, 0, 0, 892, 893, 5, 89, 0, 
		0, 893, 895, 3, 134, 67, 0, 894, 891, 1, 0, 0, 0, 895, 898, 1, 0, 0, 0, 
		896, 894, 1, 0, 0, 0, 896, 897, 1, 0, 0, 0, 897, 131, 1, 0, 0, 0, 898, 
		896, 1, 0, 0, 0, 899, 900, 6, 66, -1, 0, 900, 901, 3, 134, 67, 0, 901, 
		902, 5, 10, 0, 0, 902, 903, 3, 134, 67, 0, 903, 909, 1, 0, 0, 0, 904, 
		905, 10, 1, 0, 0, 905, 906, 5, 10, 0, 0, 906, 908, 3, 134, 67, 0, 907, 
		904, 1, 0, 0, 0, 908, 911, 1, 0, 0, 0, 909, 907, 1, 0, 0, 0, 909, 910, 
		1, 0, 0, 0, 910, 133, 1, 0, 0, 0, 911, 909, 1, 0, 0, 0, 912, 916, 3, 138, 
		69, 0, 913, 916, 3, 150, 75, 0, 914, 916, 3, 164, 82, 0, 915, 912, 1, 
		0, 0, 0, 915, 913, 1, 0, 0, 0, 915, 914, 1, 0, 0, 0, 916, 135, 1, 0, 0, 
		0, 917, 920, 3, 138, 69, 0, 918, 920, 3, 152, 76, 0, 919, 917, 1, 0, 0, 
		0, 919, 918, 1, 0, 0, 0, 920, 137, 1, 0, 0, 0, 921, 929, 3, 142, 71, 0, 
		922, 929, 3, 156, 78, 0, 923, 929, 3, 140, 70, 0, 924, 925, 5, 12, 0, 
		0, 925, 926, 3, 122, 61, 0, 926, 927, 5, 13, 0, 0, 927, 929, 1, 0, 0, 
		0, 928, 921, 1, 0, 0, 0, 928, 922, 1, 0, 0, 0, 928, 923, 1, 0, 0, 0, 928, 
		924, 1, 0, 0, 0, 929, 139, 1, 0, 0, 0, 930, 931, 3, 382, 191, 0, 931, 
		141, 1, 0, 0, 0, 932, 933, 3, 332, 166, 0, 933, 934, 5, 14, 0, 0, 934, 
		935, 3, 144, 72, 0, 935, 936, 5, 15, 0, 0, 936, 937, 5, 16, 0, 0, 937, 
		938, 3, 134, 67, 0, 938, 143, 1, 0, 0, 0, 939, 945, 3, 146, 73, 0, 940, 
		941, 3, 146, 73, 0, 941, 942, 5, 2, 0, 0, 942, 943, 3, 144, 72, 0, 943, 
		945, 1, 0, 0, 0, 944, 939, 1, 0, 0, 0, 944, 940, 1, 0, 0, 0, 945, 145, 
		1, 0, 0, 0, 946, 949, 3, 148, 74, 0, 947, 949, 3, 382, 191, 0, 948, 946, 
		1, 0, 0, 0, 948, 947, 1, 0, 0, 0, 949, 147, 1, 0, 0, 0, 950, 951, 3, 382, 
		191, 0, 951, 952, 5, 16, 0, 0, 952, 953, 3, 206, 103, 0, 953, 149, 1, 
		0, 0, 0, 954, 957, 3, 152, 76, 0, 955, 957, 3, 154, 77, 0, 956, 954, 1, 
		0, 0, 0, 956, 955, 1, 0, 0, 0, 957, 151, 1, 0, 0, 0, 958, 959, 3, 330, 
		165, 0, 959, 960, 3, 136, 68, 0, 960, 153, 1, 0, 0, 0, 961, 962, 3, 186, 
		93, 0, 962, 963, 3, 362, 181, 0, 963, 964, 3, 186, 93, 0, 964, 155, 1, 
		0, 0, 0, 965, 969, 3, 158, 79, 0, 966, 969, 3, 160, 80, 0, 967, 969, 3, 
		166, 83, 0, 968, 965, 1, 0, 0, 0, 968, 966, 1, 0, 0, 0, 968, 967, 1, 0, 
		0, 0, 969, 157, 1, 0, 0, 0, 970, 977, 3, 364, 182, 0, 971, 972, 3, 366, 
		183, 0, 972, 973, 5, 12, 0, 0, 973, 974, 3, 190, 95, 0, 974, 975, 5, 13, 
		0, 0, 975, 977, 1, 0, 0, 0, 976, 970, 1, 0, 0, 0, 976, 971, 1, 0, 0, 0, 
		977, 159, 1, 0, 0, 0, 978, 979, 3, 162, 81, 0, 979, 161, 1, 0, 0, 0, 980, 
		989, 3, 368, 184, 0, 981, 982, 3, 370, 185, 0, 982, 983, 5, 12, 0, 0, 
		983, 984, 3, 190, 95, 0, 984, 985, 5, 13, 0, 0, 985, 989, 1, 0, 0, 0, 
		986, 989, 3, 182, 91, 0, 987, 989, 3, 168, 84, 0, 988, 980, 1, 0, 0, 0, 
		988, 981, 1, 0, 0, 0, 988, 986, 1, 0, 0, 0, 988, 987, 1, 0, 0, 0, 989, 
		163, 1, 0, 0, 0, 990, 991, 3, 186, 93, 0, 991, 992, 3, 358, 179, 0, 992, 
		993, 3, 186, 93, 0, 993, 165, 1, 0, 0, 0, 994, 1001, 3, 372, 186, 0, 995, 
		996, 3, 374, 187, 0, 996, 997, 5, 12, 0, 0, 997, 998, 3, 190, 95, 0, 998, 
		999, 5, 13, 0, 0, 999, 1001, 1, 0, 0, 0, 1000, 994, 1, 0, 0, 0, 1000, 
		995, 1, 0, 0, 0, 1001, 167, 1, 0, 0, 0, 1002, 1003, 5, 17, 0, 0, 1003, 
		1004, 3, 170, 85, 0, 1004, 1005, 5, 2, 0, 0, 1005, 1006, 3, 174, 87, 0, 
		1006, 1007, 5, 2, 0, 0, 1007, 1008, 3, 184, 92, 0, 1008, 1009, 5, 13, 
		0, 0, 1009, 169, 1, 0, 0, 0, 1010, 1016, 3, 194, 97, 0, 1011, 1012, 5, 
		14, 0, 0, 1012, 1013, 3, 172, 86, 0, 1013, 1014, 5, 15, 0, 0, 1014, 1016, 
		1, 0, 0, 0, 1015, 1010, 1, 0, 0, 0, 1015, 1011, 1, 0, 0, 0, 1016, 171, 
		1, 0, 0, 0, 1017, 1023, 3, 194, 97, 0, 1018, 1019, 3, 194, 97, 0, 1019, 
		1020, 5, 2, 0, 0, 1020, 1021, 3, 172, 86, 0, 1021, 1023, 1, 0, 0, 0, 1022, 
		1017, 1, 0, 0, 0, 1022, 1018, 1, 0, 0, 0, 1023, 173, 1, 0, 0, 0, 1024, 
		1030, 3, 176, 88, 0, 1025, 1026, 5, 14, 0, 0, 1026, 1027, 3, 180, 90, 
		0, 1027, 1028, 5, 15, 0, 0, 1028, 1030, 1, 0, 0, 0, 1029, 1024, 1, 0, 
		0, 0, 1029, 1025, 1, 0, 0, 0, 1030, 175, 1, 0, 0, 0, 1031, 1032, 3, 178, 
		89, 0, 1032, 1033, 3, 344, 172, 0, 1033, 1034, 3, 184, 92, 0, 1034, 177, 
		1, 0, 0, 0, 1035, 1038, 3, 158, 79, 0, 1036, 1038, 3, 188, 94, 0, 1037, 
		1035, 1, 0, 0, 0, 1037, 1036, 1, 0, 0, 0, 1038, 179, 1, 0, 0, 0, 1039, 
		1045, 3, 176, 88, 0, 1040, 1041, 3, 176, 88, 0, 1041, 1042, 5, 2, 0, 0, 
		1042, 1043, 3, 180, 90, 0, 1043, 1045, 1, 0, 0, 0, 1044, 1039, 1, 0, 0, 
		0, 1044, 1040, 1, 0, 0, 0, 1045, 181, 1, 0, 0, 0, 1046, 1047, 3, 232, 
		116, 0, 1047, 1048, 5, 11, 0, 0, 1048, 1049, 5, 12, 0, 0, 1049, 1050, 
		3, 190, 95, 0, 1050, 1051, 5, 13, 0, 0, 1051, 183, 1, 0, 0, 0, 1052, 1056, 
		3, 122, 61, 0, 1053, 1056, 3, 380, 190, 0, 1054, 1056, 3, 188, 94, 0, 
		1055, 1052, 1, 0, 0, 0, 1055, 1053, 1, 0, 0, 0, 1055, 1054, 1, 0, 0, 0, 
		1056, 185, 1, 0, 0, 0, 1057, 1066, 3, 156, 78, 0, 1058, 1066, 3, 380, 
		190, 0, 1059, 1066, 3, 188, 94, 0, 1060, 1066, 3, 382, 191, 0, 1061, 1062, 
		5, 12, 0, 0, 1062, 1063, 3, 122, 61, 0, 1063, 1064, 5, 13, 0, 0, 1064, 
		1066, 1, 0, 0, 0, 1065, 1057, 1, 0, 0, 0, 1065, 1058, 1, 0, 0, 0, 1065, 
		1059, 1, 0, 0, 0, 1065, 1060, 1, 0, 0, 0, 1065, 1061, 1, 0, 0, 0, 1066, 
		187, 1, 0, 0, 0, 1067, 1073, 5, 18, 0, 0, 1068, 1069, 5, 14, 0, 0, 1069, 
		1070, 3, 190, 95, 0, 1070, 1071, 5, 15, 0, 0, 1071, 1073, 1, 0, 0, 0, 
		1072, 1067, 1, 0, 0, 0, 1072, 1068, 1, 0, 0, 0, 1073, 189, 1, 0, 0, 0, 
		1074, 1078, 3, 184, 92, 0, 1075, 1077, 3, 192, 96, 0, 1076, 1075, 1, 0, 
		0, 0, 1077, 1080, 1, 0, 0, 0, 1078, 1076, 1, 0, 0, 0, 1078, 1079, 1, 0, 
		0, 0, 1079, 191, 1, 0, 0, 0, 1080, 1078, 1, 0, 0, 0, 1081, 1082, 5, 2, 
		0, 0, 1082, 1083, 3, 184, 92, 0, 1083, 193, 1, 0, 0, 0, 1084, 1085, 3, 
		356, 178, 0, 1085, 1086, 5, 16, 0, 0, 1086, 1087, 3, 196, 98, 0, 1087, 
		1093, 1, 0, 0, 0, 1088, 1089, 5, 12, 0, 0, 1089, 1090, 3, 194, 97, 0, 
		1090, 1091, 5, 13, 0, 0, 1091, 1093, 1, 0, 0, 0, 1092, 1084, 1, 0, 0, 
		0, 1092, 1088, 1, 0, 0, 0, 1093, 195, 1, 0, 0, 0, 1094, 1097, 3, 206, 
		103, 0, 1095, 1097, 3, 198, 99, 0, 1096, 1094, 1, 0, 0, 0, 1096, 1095, 
		1, 0, 0, 0, 1097, 197, 1, 0, 0, 0, 1098, 1105, 3, 210, 105, 0, 1099, 1105, 
		3, 200, 100, 0, 1100, 1101, 5, 12, 0, 0, 1101, 1102, 3, 198, 99, 0, 1102, 
		1103, 5, 13, 0, 0, 1103, 1105, 1, 0, 0, 0, 1104, 1098, 1, 0, 0, 0, 1104, 
		1099, 1, 0, 0, 0, 1104, 1100, 1, 0, 0, 0, 1105, 199, 1, 0, 0, 0, 1106, 
		1107, 3, 326, 163, 0, 1107, 1108, 5, 14, 0, 0, 1108, 1109, 3, 144, 72, 
		0, 1109, 1110, 5, 15, 0, 0, 1110, 1111, 5, 16, 0, 0, 1111, 1112, 3, 202, 
		101, 0, 1112, 201, 1, 0, 0, 0, 1113, 1120, 3, 206, 103, 0, 1114, 1115, 
		5, 12, 0, 0, 1115, 1116, 3, 210, 105, 0, 1116, 1117, 5, 13, 0, 0, 1117, 
		1120, 1, 0, 0, 0, 1118, 1120, 3, 200, 100, 0, 1119, 1113, 1, 0, 0, 0, 
		1119, 1114, 1, 0, 0, 0, 1119, 1118, 1, 0, 0, 0, 1120, 203, 1, 0, 0, 0, 
		1121, 1127, 3, 206, 103, 0, 1122, 1123, 5, 12, 0, 0, 1123, 1124, 3, 212, 
		106, 0, 1124, 1125, 5, 13, 0, 0, 1125, 1127, 1, 0, 0, 0, 1126, 1121, 1, 
		0, 0, 0, 1126, 1122, 1, 0, 0, 0, 1127, 205, 1, 0, 0, 0, 1128, 1142, 3, 
		348, 174, 0, 1129, 1142, 3, 352, 176, 0, 1130, 1142, 3, 382, 191, 0, 1131, 
		1132, 3, 350, 175, 0, 1132, 1133, 5, 12, 0, 0, 1133, 1134, 3, 208, 104, 
		0, 1134, 1135, 5, 13, 0, 0, 1135, 1142, 1, 0, 0, 0, 1136, 1137, 5, 12, 
		0, 0, 1137, 1138, 3, 206, 103, 0, 1138, 1139, 5, 13, 0, 0, 1139, 1142, 
		1, 0, 0, 0, 1140, 1142, 3, 214, 107, 0, 1141, 1128, 1, 0, 0, 0, 1141, 
		1129, 1, 0, 0, 0, 1141, 1130, 1, 0, 0, 0, 1141, 1131, 1, 0, 0, 0, 1141, 
		1136, 1, 0, 0, 0, 1141, 1140, 1, 0, 0, 0, 1142, 207, 1, 0, 0, 0, 1143, 
		1149, 3, 206, 103, 0, 1144, 1145, 3, 206, 103, 0, 1145, 1146, 5, 2, 0, 
		0, 1146, 1147, 3, 208, 104, 0, 1147, 1149, 1, 0, 0, 0, 1148, 1143, 1, 
		0, 0, 0, 1148, 1144, 1, 0, 0, 0, 1149, 209, 1, 0, 0, 0, 1150, 1151, 3, 
		204, 102, 0, 1151, 1152, 5, 92, 0, 0, 1152, 1153, 3, 206, 103, 0, 1153, 
		211, 1, 0, 0, 0, 1154, 1155, 6, 106, -1, 0, 1155, 1156, 3, 204, 102, 0, 
		1156, 1157, 5, 90, 0, 0, 1157, 1158, 3, 206, 103, 0, 1158, 1164, 1, 0, 
		0, 0, 1159, 1160, 10, 1, 0, 0, 1160, 1161, 5, 90, 0, 0, 1161, 1163, 3, 
		206, 103, 0, 1162, 1159, 1, 0, 0, 0, 1163, 1166, 1, 0, 0, 0, 1164, 1162, 
		1, 0, 0, 0, 1164, 1165, 1, 0, 0, 0, 1165, 213, 1, 0, 0, 0, 1166, 1164, 
		1, 0, 0, 0, 1167, 1168, 5, 14, 0, 0, 1168, 1169, 3, 216, 108, 0, 1169, 
		1170, 5, 15, 0, 0, 1170, 215, 1, 0, 0, 0, 1171, 1177, 3, 196, 98, 0, 1172, 
		1173, 3, 196, 98, 0, 1173, 1174, 5, 2, 0, 0, 1174, 1175, 3, 216, 108, 
		0, 1175, 1177, 1, 0, 0, 0, 1176, 1171, 1, 0, 0, 0, 1176, 1172, 1, 0, 0, 
		0, 1177, 217, 1, 0, 0, 0, 1178, 1179, 3, 356, 178, 0, 1179, 1180, 3, 328, 
		164, 0, 1180, 1181, 3, 354, 177, 0, 1181, 219, 1, 0, 0, 0, 1182, 1183, 
		3, 156, 78, 0, 1183, 1184, 3, 346, 173, 0, 1184, 1185, 3, 184, 92, 0, 
		1185, 221, 1, 0, 0, 0, 1186, 1187, 3, 188, 94, 0, 1187, 1188, 3, 342, 
		171, 0, 1188, 1189, 3, 188, 94, 0, 1189, 223, 1, 0, 0, 0, 1190, 1191, 
		5, 19, 0, 0, 1191, 1192, 3, 240, 120, 0, 1192, 1193, 5, 20, 0, 0, 1193, 
		1201, 1, 0, 0, 0, 1194, 1195, 5, 19, 0, 0, 1195, 1196, 3, 240, 120, 0, 
		1196, 1197, 5, 12, 0, 0, 1197, 1198, 3, 226, 113, 0, 1198, 1199, 5, 21, 
		0, 0, 1199, 1201, 1, 0, 0, 0, 1200, 1190, 1, 0, 0, 0, 1200, 1194, 1, 0, 
		0, 0, 1201, 225, 1, 0, 0, 0, 1202, 1208, 3, 228, 114, 0, 1203, 1204, 3, 
		228, 114, 0, 1204, 1205, 5, 2, 0, 0, 1205, 1206, 3, 226, 113, 0, 1206, 
		1208, 1, 0, 0, 0, 1207, 1202, 1, 0, 0, 0, 1207, 1203, 1, 0, 0, 0, 1208, 
		227, 1, 0, 0, 0, 1209, 1212, 3, 242, 121, 0, 1210, 1212, 3, 230, 115, 
		0, 1211, 1209, 1, 0, 0, 0, 1211, 1210, 1, 0, 0, 0, 1212, 229, 1, 0, 0, 
		0, 1213, 1214, 3, 116, 58, 0, 1214, 231, 1, 0, 0, 0, 1215, 1216, 5, 19, 
		0, 0, 1216, 1217, 3, 240, 120, 0, 1217, 1218, 5, 20, 0, 0, 1218, 1226, 
		1, 0, 0, 0, 1219, 1220, 5, 19, 0, 0, 1220, 1221, 3, 240, 120, 0, 1221, 
		1222, 5, 12, 0, 0, 1222, 1223, 3, 234, 117, 0, 1223, 1224, 5, 21, 0, 0, 
		1224, 1226, 1, 0, 0, 0, 1225, 1215, 1, 0, 0, 0, 1225, 1219, 1, 0, 0, 0, 
		1226, 233, 1, 0, 0, 0, 1227, 1233, 3, 236, 118, 0, 1228, 1229, 3, 236, 
		118, 0, 1229, 1230, 5, 2, 0, 0, 1230, 1231, 3, 234, 117, 0, 1231, 1233, 
		1, 0, 0, 0, 1232, 1227, 1, 0, 0, 0, 1232, 1228, 1, 0, 0, 0, 1233, 235, 
		1, 0, 0, 0, 1234, 1237, 3, 242, 121, 0, 1235, 1237, 3, 238, 119, 0, 1236, 
		1234, 1, 0, 0, 0, 1236, 1235, 1, 0, 0, 0, 1237, 237, 1, 0, 0, 0, 1238, 
		1239, 3, 220, 110, 0, 1239, 239, 1, 0, 0, 0, 1240, 1241, 3, 376, 188, 
		0, 1241, 241, 1, 0, 0, 0, 1242, 1243, 5, 94, 0, 0, 1243, 1244, 3, 186, 
		93, 0, 1244, 243, 1, 0, 0, 0, 1245, 1252, 5, 22, 0, 0, 1246, 1247, 5, 
		93, 0, 0, 1247, 1248, 5, 23, 0, 0, 1248, 1252, 5, 92, 0, 0, 1249, 1252, 
		5, 24, 0, 0, 1250, 1252, 5, 25, 0, 0, 1251, 1245, 1, 0, 0, 0, 1251, 1246, 
		1, 0, 0, 0, 1251, 1249, 1, 0, 0, 0, 1251, 1250, 1, 0, 0, 0, 1252, 245, 
		1, 0, 0, 0, 1253, 1256, 3, 248, 124, 0, 1254, 1256, 3, 194, 97, 0, 1255, 
		1253, 1, 0, 0, 0, 1255, 1254, 1, 0, 0, 0, 1256, 247, 1, 0, 0, 0, 1257, 
		1260, 3, 250, 125, 0, 1258, 1260, 3, 314, 157, 0, 1259, 1257, 1, 0, 0, 
		0, 1259, 1258, 1, 0, 0, 0, 1260, 249, 1, 0, 0, 0, 1261, 1262, 5, 26, 0, 
		0, 1262, 1263, 5, 14, 0, 0, 1263, 1264, 3, 144, 72, 0, 1264, 1265, 5, 
		15, 0, 0, 1265, 1266, 5, 16, 0, 0, 1266, 1267, 3, 248, 124, 0, 1267, 251, 
		1, 0, 0, 0, 1268, 1271, 3, 254, 127, 0, 1269, 1271, 3, 306, 153, 0, 1270, 
		1268, 1, 0, 0, 0, 1270, 1269, 1, 0, 0, 0, 1271, 253, 1, 0, 0, 0, 1272, 
		1276, 3, 256, 128, 0, 1273, 1276, 3, 266, 133, 0, 1274, 1276, 3, 272, 
		136, 0, 1275, 1272, 1, 0, 0, 0, 1275, 1273, 1, 0, 0, 0, 1275, 1274, 1, 
		0, 0, 0, 1276, 255, 1, 0, 0, 0, 1277, 1280, 3, 258, 129, 0, 1278, 1280, 
		3, 260, 130, 0, 1279, 1277, 1, 0, 0, 0, 1279, 1278, 1, 0, 0, 0, 1280, 
		257, 1, 0, 0, 0, 1281, 1282, 3, 270, 135, 0, 1282, 1283, 3, 336, 168, 
		0, 1283, 1284, 3, 270, 135, 0, 1284, 259, 1, 0, 0, 0, 1285, 1288, 3, 262, 
		131, 0, 1286, 1288, 3, 264, 132, 0, 1287, 1285, 1, 0, 0, 0, 1287, 1286, 
		1, 0, 0, 0, 1288, 261, 1, 0, 0, 0, 1289, 1290, 6, 131, -1, 0, 1290, 1291, 
		3, 270, 135, 0, 1291, 1292, 5, 89, 0, 0, 1292, 1293, 3, 270, 135, 0, 1293, 
		1299, 1, 0, 0, 0, 1294, 1295, 10, 1, 0, 0, 1295, 1296, 5, 89, 0, 0, 1296, 
		1298, 3, 270, 135, 0, 1297, 1294, 1, 0, 0, 0, 1298, 1301, 1, 0, 0, 0, 
		1299, 1297, 1, 0, 0, 0, 1299, 1300, 1, 0, 0, 0, 1300, 263, 1, 0, 0, 0, 
		1301, 1299, 1, 0, 0, 0, 1302, 1303, 6, 132, -1, 0, 1303, 1304, 3, 270, 
		135, 0, 1304, 1305, 5, 10, 0, 0, 1305, 1306, 3, 270, 135, 0, 1306, 1312, 
		1, 0, 0, 0, 1307, 1308, 10, 1, 0, 0, 1308, 1309, 5, 10, 0, 0, 1309, 1311, 
		3, 270, 135, 0, 1310, 1307, 1, 0, 0, 0, 1311, 1314, 1, 0, 0, 0, 1312, 
		1310, 1, 0, 0, 0, 1312, 1313, 1, 0, 0, 0, 1313, 265, 1, 0, 0, 0, 1314, 
		1312, 1, 0, 0, 0, 1315, 1316, 3, 340, 170, 0, 1316, 1317, 3, 270, 135, 
		0, 1317, 1320, 1, 0, 0, 0, 1318, 1320, 3, 268, 134, 0, 1319, 1315, 1, 
		0, 0, 0, 1319, 1318, 1, 0, 0, 0, 1320, 267, 1, 0, 0, 0, 1321, 1322, 3, 
		302, 151, 0, 1322, 1323, 3, 362, 181, 0, 1323, 1324, 3, 302, 151, 0, 1324, 
		269, 1, 0, 0, 0, 1325, 1328, 3, 272, 136, 0, 1326, 1328, 3, 266, 133, 
		0, 1327, 1325, 1, 0, 0, 0, 1327, 1326, 1, 0, 0, 0, 1328, 271, 1, 0, 0, 
		0, 1329, 1336, 3, 274, 137, 0, 1330, 1336, 3, 278, 139, 0, 1331, 1332, 
		5, 12, 0, 0, 1332, 1333, 3, 254, 127, 0, 1333, 1334, 5, 13, 0, 0, 1334, 
		1336, 1, 0, 0, 0, 1335, 1329, 1, 0, 0, 0, 1335, 1330, 1, 0, 0, 0, 1335, 
		1331, 1, 0, 0, 0, 1336, 273, 1, 0, 0, 0, 1337, 1338, 3, 334, 167, 0, 1338, 
		1339, 5, 14, 0, 0, 1339, 1340, 3, 276, 138, 0, 1340, 1341, 5, 15, 0, 0, 
		1341, 1342, 5, 16, 0, 0, 1342, 1343, 3, 270, 135, 0, 1343, 275, 1, 0, 
		0, 0, 1344, 1350, 3, 382, 191, 0, 1345, 1346, 3, 382, 191, 0, 1346, 1347, 
		5, 2, 0, 0, 1347, 1348, 3, 276, 138, 0, 1348, 1350, 1, 0, 0, 0, 1349, 
		1344, 1, 0, 0, 0, 1349, 1345, 1, 0, 0, 0, 1350, 277, 1, 0, 0, 0, 1351, 
		1355, 3, 280, 140, 0, 1352, 1355, 3, 282, 141, 0, 1353, 1355, 3, 288, 
		144, 0, 1354, 1351, 1, 0, 0, 0, 1354, 1352, 1, 0, 0, 0, 1354, 1353, 1, 
		0, 0, 0, 1355, 279, 1, 0, 0, 0, 1356, 1357, 3, 290, 145, 0, 1357, 281, 
		1, 0, 0, 0, 1358, 1361, 3, 284, 142, 0, 1359, 1361, 3, 286, 143, 0, 1360, 
		1358, 1, 0, 0, 0, 1360, 1359, 1, 0, 0, 0, 1361, 283, 1, 0, 0, 0, 1362, 
		1363, 3, 296, 148, 0, 1363, 285, 1, 0, 0, 0, 1364, 1365, 3, 302, 151, 
		0, 1365, 1366, 3, 358, 179, 0, 1366, 1367, 3, 302, 151, 0, 1367, 287, 
		1, 0, 0, 0, 1368, 1369, 3, 298, 149, 0, 1369, 289, 1, 0, 0, 0, 1370, 1377, 
		3, 364, 182, 0, 1371, 1372, 3, 366, 183, 0, 1372, 1373, 5, 12, 0, 0, 1373, 
		1374, 3, 300, 150, 0, 1374, 1375, 5, 13, 0, 0, 1375, 1377, 1, 0, 0, 0, 
		1376, 1370, 1, 0, 0, 0, 1376, 1371, 1, 0, 0, 0, 1377, 291, 1, 0, 0, 0, 
		1378, 1381, 3, 380, 190, 0, 1379, 1381, 3, 294, 147, 0, 1380, 1378, 1, 
		0, 0, 0, 1380, 1379, 1, 0, 0, 0, 1381, 293, 1, 0, 0, 0, 1382, 1383, 3, 
		296, 148, 0, 1383, 295, 1, 0, 0, 0, 1384, 1391, 3, 368, 184, 0, 1385, 
		1386, 3, 370, 185, 0, 1386, 1387, 5, 12, 0, 0, 1387, 1388, 3, 300, 150, 
		0, 1388, 1389, 5, 13, 0, 0, 1389, 1391, 1, 0, 0, 0, 1390, 1384, 1, 0, 
		0, 0, 1390, 1385, 1, 0, 0, 0, 1391, 297, 1, 0, 0, 0, 1392, 1399, 3, 372, 
		186, 0, 1393, 1394, 3, 374, 187, 0, 1394, 1395, 5, 12, 0, 0, 1395, 1396, 
		3, 300, 150, 0, 1396, 1397, 5, 13, 0, 0, 1397, 1399, 1, 0, 0, 0, 1398, 
		1392, 1, 0, 0, 0, 1398, 1393, 1, 0, 0, 0, 1399, 299, 1, 0, 0, 0, 1400, 
		1406, 3, 302, 151, 0, 1401, 1402, 3, 302, 151, 0, 1402, 1403, 5, 2, 0, 
		0, 1403, 1404, 3, 300, 150, 0, 1404, 1406, 1, 0, 0, 0, 1405, 1400, 1, 
		0, 0, 0, 1405, 1401, 1, 0, 0, 0, 1406, 301, 1, 0, 0, 0, 1407, 1410, 3, 
		304, 152, 0, 1408, 1410, 3, 382, 191, 0, 1409, 1407, 1, 0, 0, 0, 1409, 
		1408, 1, 0, 0, 0, 1410, 303, 1, 0, 0, 0, 1411, 1415, 3, 290, 145, 0, 1412, 
		1415, 3, 292, 146, 0, 1413, 1415, 3, 298, 149, 0, 1414, 1411, 1, 0, 0, 
		0, 1414, 1412, 1, 0, 0, 0, 1414, 1413, 1, 0, 0, 0, 1415, 305, 1, 0, 0, 
		0, 1416, 1417, 3, 308, 154, 0, 1417, 1418, 3, 342, 171, 0, 1418, 1419, 
		3, 308, 154, 0, 1419, 1425, 1, 0, 0, 0, 1420, 1421, 5, 12, 0, 0, 1421, 
		1422, 3, 306, 153, 0, 1422, 1423, 5, 13, 0, 0, 1423, 1425, 1, 0, 0, 0, 
		1424, 1416, 1, 0, 0, 0, 1424, 1420, 1, 0, 0, 0, 1425, 307, 1, 0, 0, 0, 
		1426, 1432, 5, 18, 0, 0, 1427, 1428, 5, 14, 0, 0, 1428, 1429, 3, 310, 
		155, 0, 1429, 1430, 5, 15, 0, 0, 1430, 1432, 1, 0, 0, 0, 1431, 1426, 1, 
		0, 0, 0, 1431, 1427, 1, 0, 0, 0, 1432, 309, 1, 0, 0, 0, 1433, 1437, 3, 
		254, 127, 0, 1434, 1436, 3, 312, 156, 0, 1435, 1434, 1, 0, 0, 0, 1436, 
		1439, 1, 0, 0, 0, 1437, 1435, 1, 0, 0, 0, 1437, 1438, 1, 0, 0, 0, 1438, 
		311, 1, 0, 0, 0, 1439, 1437, 1, 0, 0, 0, 1440, 1441, 5, 2, 0, 0, 1441, 
		1442, 3, 254, 127, 0, 1442, 313, 1, 0, 0, 0, 1443, 1449, 3, 316, 158, 
		0, 1444, 1445, 5, 12, 0, 0, 1445, 1446, 3, 314, 157, 0, 1446, 1447, 5, 
		13, 0, 0, 1447, 1449, 1, 0, 0, 0, 1448, 1443, 1, 0, 0, 0, 1448, 1444, 
		1, 0, 0, 0, 1449, 315, 1, 0, 0, 0, 1450, 1451, 6, 158, -1, 0, 1451, 1452, 
		3, 318, 159, 0, 1452, 1458, 1, 0, 0, 0, 1453, 1454, 10, 1, 0, 0, 1454, 
		1455, 5, 89, 0, 0, 1455, 1457, 3, 318, 159, 0, 1456, 1453, 1, 0, 0, 0, 
		1457, 1460, 1, 0, 0, 0, 1458, 1456, 1, 0, 0, 0, 1458, 1459, 1, 0, 0, 0, 
		1459, 317, 1, 0, 0, 0, 1460, 1458, 1, 0, 0, 0, 1461, 1471, 3, 278, 139, 
		0, 1462, 1463, 5, 27, 0, 0, 1463, 1471, 3, 278, 139, 0, 1464, 1465, 5, 
		27, 0, 0, 1465, 1466, 5, 12, 0, 0, 1466, 1467, 3, 278, 139, 0, 1467, 1468, 
		5, 13, 0, 0, 1468, 1471, 1, 0, 0, 0, 1469, 1471, 3, 268, 134, 0, 1470, 
		1461, 1, 0, 0, 0, 1470, 1462, 1, 0, 0, 0, 1470, 1464, 1, 0, 0, 0, 1470, 
		1469, 1, 0, 0, 0, 1471, 319, 1, 0, 0, 0, 1472, 1476, 3, 332, 166, 0, 1473, 
		1476, 3, 324, 162, 0, 1474, 1476, 3, 326, 163, 0, 1475, 1472, 1, 0, 0, 
		0, 1475, 1473, 1, 0, 0, 0, 1475, 1474, 1, 0, 0, 0, 1476, 321, 1, 0, 0, 
		0, 1477, 1480, 3, 340, 170, 0, 1478, 1480, 3, 244, 122, 0, 1479, 1477, 
		1, 0, 0, 0, 1479, 1478, 1, 0, 0, 0, 1480, 323, 1, 0, 0, 0, 1481, 1482, 
		7, 0, 0, 0, 1482, 325, 1, 0, 0, 0, 1483, 1484, 7, 1, 0, 0, 1484, 327, 
		1, 0, 0, 0, 1485, 1486, 5, 33, 0, 0, 1486, 329, 1, 0, 0, 0, 1487, 1490, 
		3, 340, 170, 0, 1488, 1490, 3, 244, 122, 0, 1489, 1487, 1, 0, 0, 0, 1489, 
		1488, 1, 0, 0, 0, 1490, 331, 1, 0, 0, 0, 1491, 1494, 3, 334, 167, 0, 1492, 
		1494, 5, 94, 0, 0, 1493, 1491, 1, 0, 0, 0, 1493, 1492, 1, 0, 0, 0, 1494, 
		333, 1, 0, 0, 0, 1495, 1496, 7, 2, 0, 0, 1496, 335, 1, 0, 0, 0, 1497, 
		1505, 5, 35, 0, 0, 1498, 1505, 5, 36, 0, 0, 1499, 1505, 5, 37, 0, 0, 1500, 
		1505, 5, 38, 0, 0, 1501, 1502, 5, 27, 0, 0, 1502, 1505, 5, 89, 0, 0, 1503, 
		1505, 5, 39, 0, 0, 1504, 1497, 1, 0, 0, 0, 1504, 1498, 1, 0, 0, 0, 1504, 
		1499, 1, 0, 0, 0, 1504, 1500, 1, 0, 0, 0, 1504, 1501, 1, 0, 0, 0, 1504, 
		1503, 1, 0, 0, 0, 1505, 337, 1, 0, 0, 0, 1506, 1507, 7, 3, 0, 0, 1507, 
		339, 1, 0, 0, 0, 1508, 1509, 5, 27, 0, 0, 1509, 341, 1, 0, 0, 0, 1510, 
		1511, 5, 40, 0, 0, 1511, 343, 1, 0, 0, 0, 1512, 1513, 5, 41, 0, 0, 1513, 
		345, 1, 0, 0, 0, 1514, 1515, 5, 42, 0, 0, 1515, 347, 1, 0, 0, 0, 1516, 
		1517, 3, 350, 175, 0, 1517, 349, 1, 0, 0, 0, 1518, 1519, 3, 452, 226, 
		0, 1519, 351, 1, 0, 0, 0, 1520, 1521, 3, 454, 227, 0, 1521, 353, 1, 0, 
		0, 0, 1522, 1525, 3, 356, 178, 0, 1523, 1525, 3, 368, 184, 0, 1524, 1522, 
		1, 0, 0, 0, 1524, 1523, 1, 0, 0, 0, 1525, 355, 1, 0, 0, 0, 1526, 1529, 
		3, 364, 182, 0, 1527, 1529, 3, 372, 186, 0, 1528, 1526, 1, 0, 0, 0, 1528, 
		1527, 1, 0, 0, 0, 1529, 357, 1, 0, 0, 0, 1530, 1531, 3, 360, 180, 0, 1531, 
		359, 1, 0, 0, 0, 1532, 1533, 5, 43, 0, 0, 1533, 361, 1, 0, 0, 0, 1534, 
		1535, 5, 44, 0, 0, 1535, 363, 1, 0, 0, 0, 1536, 1537, 3, 366, 183, 0, 
		1537, 365, 1, 0, 0, 0, 1538, 1539, 3, 452, 226, 0, 1539, 367, 1, 0, 0, 
		0, 1540, 1541, 3, 370, 185, 0, 1541, 369, 1, 0, 0, 0, 1542, 1543, 3, 454, 
		227, 0, 1543, 371, 1, 0, 0, 0, 1544, 1545, 3, 374, 187, 0, 1545, 373, 
		1, 0, 0, 0, 1546, 1547, 3, 456, 228, 0, 1547, 375, 1, 0, 0, 0, 1548, 1551, 
		3, 368, 184, 0, 1549, 1551, 3, 372, 186, 0, 1550, 1548, 1, 0, 0, 0, 1550, 
		1549, 1, 0, 0, 0, 1551, 377, 1, 0, 0, 0, 1552, 1553, 7, 4, 0, 0, 1553, 
		379, 1, 0, 0, 0, 1554, 1557, 3, 458, 229, 0, 1555, 1557, 5, 67, 0, 0, 
		1556, 1554, 1, 0, 0, 0, 1556, 1555, 1, 0, 0, 0, 1557, 381, 1, 0, 0, 0, 
		1558, 1559, 5, 70, 0, 0, 1559, 383, 1, 0, 0, 0, 1560, 1569, 3, 388, 194, 
		0, 1561, 1569, 3, 394, 197, 0, 1562, 1569, 3, 398, 199, 0, 1563, 1569, 
		5, 50, 0, 0, 1564, 1565, 5, 14, 0, 0, 1565, 1566, 3, 386, 193, 0, 1566, 
		1567, 5, 15, 0, 0, 1567, 1569, 1, 0, 0, 0, 1568, 1560, 1, 0, 0, 0, 1568, 
		1561, 1, 0, 0, 0, 1568, 1562, 1, 0, 0, 0, 1568, 1563, 1, 0, 0, 0, 1568, 
		1564, 1, 0, 0, 0, 1569, 385, 1, 0, 0, 0, 1570, 1576, 3, 384, 192, 0, 1571, 
		1572, 3, 384, 192, 0, 1572, 1573, 5, 2, 0, 0, 1573, 1574, 3, 386, 193, 
		0, 1574, 1576, 1, 0, 0, 0, 1575, 1570, 1, 0, 0, 0, 1575, 1571, 1, 0, 0, 
		0, 1576, 387, 1, 0, 0, 0, 1577, 1580, 3, 450, 225, 0, 1578, 1580, 3, 390, 
		195, 0, 1579, 1577, 1, 0, 0, 0, 1579, 1578, 1, 0, 0, 0, 1580, 389, 1, 
		0, 0, 0, 1581, 1582, 5, 51, 0, 0, 1582, 1583, 3, 392, 196, 0, 1583, 1584, 
		5, 2, 0, 0, 1584, 1585, 3, 424, 212, 0, 1585, 1586, 5, 2, 0, 0, 1586, 
		1587, 3, 412, 206, 0, 1587, 1588, 5, 13, 0, 0, 1588, 391, 1, 0, 0, 0, 
		1589, 1590, 3, 452, 226, 0, 1590, 393, 1, 0, 0, 0, 1591, 1592, 5, 52, 
		0, 0, 1592, 1593, 3, 396, 198, 0, 1593, 1594, 5, 2, 0, 0, 1594, 1595, 
		3, 424, 212, 0, 1595, 1596, 5, 2, 0, 0, 1596, 1597, 3, 412, 206, 0, 1597, 
		1598, 5, 13, 0, 0, 1598, 395, 1, 0, 0, 0, 1599, 1600, 3, 452, 226, 0, 
		1600, 397, 1, 0, 0, 0, 1601, 1605, 3, 400, 200, 0, 1602, 1605, 3, 404, 
		202, 0, 1603, 1605, 3, 408, 204, 0, 1604, 1601, 1, 0, 0, 0, 1604, 1602, 
		1, 0, 0, 0, 1604, 1603, 1, 0, 0, 0, 1605, 399, 1, 0, 0, 0, 1606, 1607, 
		5, 53, 0, 0, 1607, 1608, 3, 460, 230, 0, 1608, 1609, 3, 402, 201, 0, 1609, 
		1610, 5, 13, 0, 0, 1610, 401, 1, 0, 0, 0, 1611, 1612, 5, 2, 0, 0, 1612, 
		1615, 3, 450, 225, 0, 1613, 1615, 3, 462, 231, 0, 1614, 1611, 1, 0, 0, 
		0, 1614, 1613, 1, 0, 0, 0, 1615, 403, 1, 0, 0, 0, 1616, 1617, 5, 54, 0, 
		0, 1617, 1618, 3, 406, 203, 0, 1618, 1619, 3, 422, 211, 0, 1619, 1620, 
		5, 13, 0, 0, 1620, 405, 1, 0, 0, 0, 1621, 1622, 3, 452, 226, 0, 1622, 
		407, 1, 0, 0, 0, 1623, 1624, 5, 55, 0, 0, 1624, 1625, 3, 410, 205, 0, 
		1625, 1626, 5, 2, 0, 0, 1626, 1627, 3, 424, 212, 0, 1627, 1628, 5, 2, 
		0, 0, 1628, 1629, 3, 412, 206, 0, 1629, 1630, 5, 13, 0, 0, 1630, 409, 
		1, 0, 0, 0, 1631, 1632, 3, 452, 226, 0, 1632, 411, 1, 0, 0, 0, 1633, 1639, 
		5, 18, 0, 0, 1634, 1635, 5, 14, 0, 0, 1635, 1636, 3, 414, 207, 0, 1636, 
		1637, 5, 15, 0, 0, 1637, 1639, 1, 0, 0, 0, 1638, 1633, 1, 0, 0, 0, 1638, 
		1634, 1, 0, 0, 0, 1639, 413, 1, 0, 0, 0, 1640, 1644, 3, 418, 209, 0, 1641, 
		1643, 3, 416, 208, 0, 1642, 1641, 1, 0, 0, 0, 1643, 1646, 1, 0, 0, 0, 
		1644, 1642, 1, 0, 0, 0, 1644, 1645, 1, 0, 0, 0, 1645, 415, 1, 0, 0, 0, 
		1646, 1644, 1, 0, 0, 0, 1647, 1648, 5, 2, 0, 0, 1648, 1649, 3, 418, 209, 
		0, 1649, 417, 1, 0, 0, 0, 1650, 1651, 3, 384, 192, 0, 1651, 1652, 3, 420, 
		210, 0, 1652, 419, 1, 0, 0, 0, 1653, 1654, 5, 16, 0, 0, 1654, 1657, 3, 
		436, 218, 0, 1655, 1657, 3, 462, 231, 0, 1656, 1653, 1, 0, 0, 0, 1656, 
		1655, 1, 0, 0, 0, 1657, 421, 1, 0, 0, 0, 1658, 1659, 5, 2, 0, 0, 1659, 
		1662, 3, 424, 212, 0, 1660, 1662, 3, 462, 231, 0, 1661, 1658, 1, 0, 0, 
		0, 1661, 1660, 1, 0, 0, 0, 1662, 423, 1, 0, 0, 0, 1663, 1664, 3, 444, 
		222, 0, 1664, 425, 1, 0, 0, 0, 1665, 1666, 5, 56, 0, 0, 1666, 1667, 3, 
		460, 230, 0, 1667, 1668, 3, 428, 214, 0, 1668, 1669, 5, 3, 0, 0, 1669, 
		427, 1, 0, 0, 0, 1670, 1679, 3, 462, 231, 0, 1671, 1672, 5, 2, 0, 0, 1672, 
		1679, 3, 430, 215, 0, 1673, 1674, 5, 2, 0, 0, 1674, 1675, 3, 430, 215, 
		0, 1675, 1676, 5, 2, 0, 0, 1676, 1677, 3, 434, 217, 0, 1677, 1679, 1, 
		0, 0, 0, 1678, 1670, 1, 0, 0, 0, 1678, 1671, 1, 0, 0, 0, 1678, 1673, 1, 
		0, 0, 0, 1679, 429, 1, 0, 0, 0, 1680, 1681, 5, 14, 0, 0, 1681, 1682, 3, 
		432, 216, 0, 1682, 1683, 5, 15, 0, 0, 1683, 1686, 1, 0, 0, 0, 1684, 1686, 
		5, 90, 0, 0, 1685, 1680, 1, 0, 0, 0, 1685, 1684, 1, 0, 0, 0, 1686, 431, 
		1, 0, 0, 0, 1687, 1693, 3, 450, 225, 0, 1688, 1689, 3, 450, 225, 0, 1689, 
		1690, 5, 2, 0, 0, 1690, 1691, 3, 432, 216, 0, 1691, 1693, 1, 0, 0, 0, 
		1692, 1687, 1, 0, 0, 0, 1692, 1688, 1, 0, 0, 0, 1693, 433, 1, 0, 0, 0, 
		1694, 1695, 3, 450, 225, 0, 1695, 435, 1, 0, 0, 0, 1696, 1703, 3, 438, 
		219, 0, 1697, 1698, 3, 438, 219, 0, 1698, 1699, 5, 16, 0, 0, 1699, 1700, 
		3, 436, 218, 0, 1700, 1703, 1, 0, 0, 0, 1701, 1703, 3, 444, 222, 0, 1702, 
		1696, 1, 0, 0, 0, 1702, 1697, 1, 0, 0, 0, 1702, 1701, 1, 0, 0, 0, 1703, 
		437, 1, 0, 0, 0, 1704, 1711, 3, 452, 226, 0, 1705, 1711, 3, 440, 220, 
		0, 1706, 1711, 3, 382, 191, 0, 1707, 1711, 3, 458, 229, 0, 1708, 1711, 
		5, 67, 0, 0, 1709, 1711, 3, 442, 221, 0, 1710, 1704, 1, 0, 0, 0, 1710, 
		1705, 1, 0, 0, 0, 1710, 1706, 1, 0, 0, 0, 1710, 1707, 1, 0, 0, 0, 1710, 
		1708, 1, 0, 0, 0, 1710, 1709, 1, 0, 0, 0, 1711, 439, 1, 0, 0, 0, 1712, 
		1713, 3, 452, 226, 0, 1713, 1714, 5, 12, 0, 0, 1714, 1715, 3, 446, 223, 
		0, 1715, 1716, 5, 13, 0, 0, 1716, 441, 1, 0, 0, 0, 1717, 1718, 5, 57, 
		0, 0, 1718, 1719, 3, 24, 12, 0, 1719, 1720, 5, 13, 0, 0, 1720, 1738, 1, 
		0, 0, 0, 1721, 1722, 5, 58, 0, 0, 1722, 1723, 3, 120, 60, 0, 1723, 1724, 
		5, 13, 0, 0, 1724, 1738, 1, 0, 0, 0, 1725, 1726, 5, 59, 0, 0, 1726, 1727, 
		3, 252, 126, 0, 1727, 1728, 5, 13, 0, 0, 1728, 1738, 1, 0, 0, 0, 1729, 
		1730, 5, 60, 0, 0, 1730, 1731, 3, 314, 157, 0, 1731, 1732, 5, 13, 0, 0, 
		1732, 1738, 1, 0, 0, 0, 1733, 1734, 5, 61, 0, 0, 1734, 1735, 3, 302, 151, 
		0, 1735, 1736, 5, 13, 0, 0, 1736, 1738, 1, 0, 0, 0, 1737, 1717, 1, 0, 
		0, 0, 1737, 1721, 1, 0, 0, 0, 1737, 1725, 1, 0, 0, 0, 1737, 1729, 1, 0, 
		0, 0, 1737, 1733, 1, 0, 0, 0, 1738, 443, 1, 0, 0, 0, 1739, 1745, 5, 18, 
		0, 0, 1740, 1741, 5, 14, 0, 0, 1741, 1742, 3, 446, 223, 0, 1742, 1743, 
		5, 15, 0, 0, 1743, 1745, 1, 0, 0, 0, 1744, 1739, 1, 0, 0, 0, 1744, 1740, 
		1, 0, 0, 0, 1745, 445, 1, 0, 0, 0, 1746, 1750, 3, 436, 218, 0, 1747, 1749, 
		3, 448, 224, 0, 1748, 1747, 1, 0, 0, 0, 1749, 1752, 1, 0, 0, 0, 1750, 
		1748, 1, 0, 0, 0, 1750, 1751, 1, 0, 0, 0, 1751, 447, 1, 0, 0, 0, 1752, 
		1750, 1, 0, 0, 0, 1753, 1754, 5, 2, 0, 0, 1754, 1755, 3, 436, 218, 0, 
		1755, 449, 1, 0, 0, 0, 1756, 1759, 3, 452, 226, 0, 1757, 1759, 5, 82, 
		0, 0, 1758, 1756, 1, 0, 0, 0, 1758, 1757, 1, 0, 0, 0, 1759, 451, 1, 0, 
		0, 0, 1760, 1761, 7, 5, 0, 0, 1761, 453, 1, 0, 0, 0, 1762, 1763, 5, 68, 
		0, 0, 1763, 455, 1, 0, 0, 0, 1764, 1765, 5, 69, 0, 0, 1765, 457, 1, 0, 
		0, 0, 1766, 1767, 7, 6, 0, 0, 1767, 459, 1, 0, 0, 0, 1768, 1769, 3, 452, 
		226, 0, 1769, 461, 1, 0, 0, 0, 1770, 1771, 1, 0, 0, 0, 1771, 463, 1, 0, 
		0, 0, 131, 467, 474, 482, 545, 551, 556, 564, 569, 578, 590, 603, 616, 
		622, 626, 635, 651, 659, 672, 676, 686, 690, 711, 718, 725, 736, 744, 
		751, 758, 775, 783, 797, 802, 811, 821, 833, 846, 864, 872, 876, 884, 
		896, 909, 915, 919, 928, 944, 948, 956, 968, 976, 988, 1000, 1015, 1022, 
		1029, 1037, 1044, 1055, 1065, 1072, 1078, 1092, 1096, 1104, 1119, 1126, 
		1141, 1148, 1164, 1176, 1200, 1207, 1211, 1225, 1232, 1236, 1251, 1255, 
		1259, 1270, 1275, 1279, 1287, 1299, 1312, 1319, 1327, 1335, 1349, 1354, 
		1360, 1376, 1380, 1390, 1398, 1405, 1409, 1414, 1424, 1431, 1437, 1448, 
		1458, 1470, 1475, 1479, 1489, 1493, 1504, 1524, 1528, 1550, 1556, 1568, 
		1575, 1579, 1604, 1614, 1638, 1644, 1656, 1661, 1678, 1685, 1692, 1702, 
		1710, 1737, 1744, 1750, 1758
	];
}
