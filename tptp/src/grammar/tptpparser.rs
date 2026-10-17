// Generated from TPTP.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(unused_parens)]
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

const _: () = {
    assert!(
        antlr4rust::const_check::version("0", "6"),
        "Incompatible version: either switch to v0.6 or regenerate with this version"
    );
};

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
		pub const TPTP_WS:i32=58;
		pub const TPTP_Comment_line:i32=59;
		pub const TPTP_Comment_block:i32=60;
		pub const TPTP_Single_quoted:i32=61;
		pub const TPTP_Back_quoted:i32=62;
		pub const TPTP_Distinct_object:i32=63;
		pub const TPTP_Dollar_word:i32=64;
		pub const TPTP_Dollar_dollar_word:i32=65;
		pub const TPTP_Upper_word:i32=66;
		pub const TPTP_Lower_word:i32=67;
		pub const TPTP_Real:i32=68;
		pub const TPTP_Signed_real:i32=69;
		pub const TPTP_Unsigned_real:i32=70;
		pub const TPTP_Decimal_exponent:i32=71;
		pub const TPTP_Decimal_fraction:i32=72;
		pub const TPTP_Exp_integer:i32=73;
		pub const TPTP_Signed_exp_integer:i32=74;
		pub const TPTP_Rational:i32=75;
		pub const TPTP_Signed_rational:i32=76;
		pub const TPTP_Unsigned_rational:i32=77;
		pub const TPTP_Integer:i32=78;
		pub const TPTP_Signed_integer:i32=79;
		pub const TPTP_Unsigned_integer:i32=80;
		pub const TPTP_Positive_integer:i32=81;
		pub const TPTP_Integer_digits:i32=82;
		pub const TPTP_Slash:i32=83;
		pub const TPTP_Slosh:i32=84;
		pub const TPTP_Vline:i32=85;
		pub const TPTP_Star:i32=86;
		pub const TPTP_Plus:i32=87;
		pub const TPTP_Arrow:i32=88;
		pub const TPTP_Less_sign:i32=89;
		pub const TPTP_Hash:i32=90;
		pub const TPTP_Not_star_slash:i32=91;
		pub const TPTP_Percentage_sign:i32=92;
		pub const TPTP_Double_quote:i32=93;
		pub const TPTP_Single_quote:i32=94;
		pub const TPTP_Back_quote:i32=95;
		pub const TPTP_Dot:i32=96;
		pub const TPTP_Slash_char:i32=97;
		pub const TPTP_Slosh_char:i32=98;
		pub const TPTP_Zero_numeric:i32=99;
		pub const TPTP_Underscore:i32=100;
		pub const TPTP_Alpha:i32=101;
		pub const TPTP_Dollar:i32=102;
		pub const TPTP_Comment_char:i32=103;
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
	pub const RULE_thf_binary_pair:usize = 15;
	pub const RULE_thf_binary_list:usize = 16;
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
	pub const RULE_tff_binary_pair:usize = 63;
	pub const RULE_tff_binary_list:usize = 64;
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
	pub const RULE_ntf_defined_connective:usize = 121;
	pub const RULE_ntf_index:usize = 122;
	pub const RULE_ntf_short_connective:usize = 123;
	pub const RULE_tcf_formula:usize = 124;
	pub const RULE_tcf_logic_formula:usize = 125;
	pub const RULE_tcf_quantified_formula:usize = 126;
	pub const RULE_fof_formula:usize = 127;
	pub const RULE_fof_logic_formula:usize = 128;
	pub const RULE_fof_binary_formula:usize = 129;
	pub const RULE_fof_binary_pair:usize = 130;
	pub const RULE_fof_binary_list:usize = 131;
	pub const RULE_fof_or_formula:usize = 132;
	pub const RULE_fof_and_formula:usize = 133;
	pub const RULE_fof_unary_formula:usize = 134;
	pub const RULE_fof_infix_unary:usize = 135;
	pub const RULE_fof_unit_formula:usize = 136;
	pub const RULE_fof_unitary_formula:usize = 137;
	pub const RULE_fof_quantified_formula:usize = 138;
	pub const RULE_fof_variable_list:usize = 139;
	pub const RULE_fof_atomic_formula:usize = 140;
	pub const RULE_fof_plain_atomic_formula:usize = 141;
	pub const RULE_fof_defined_atomic_formula:usize = 142;
	pub const RULE_fof_defined_plain_formula:usize = 143;
	pub const RULE_fof_defined_infix_formula:usize = 144;
	pub const RULE_fof_system_atomic_formula:usize = 145;
	pub const RULE_fof_plain_term:usize = 146;
	pub const RULE_fof_defined_term:usize = 147;
	pub const RULE_fof_defined_atomic_term:usize = 148;
	pub const RULE_fof_defined_plain_term:usize = 149;
	pub const RULE_fof_system_term:usize = 150;
	pub const RULE_fof_arguments:usize = 151;
	pub const RULE_fof_term:usize = 152;
	pub const RULE_fof_function_term:usize = 153;
	pub const RULE_fof_sequent:usize = 154;
	pub const RULE_fof_formula_tuple:usize = 155;
	pub const RULE_fof_formula_tuple_list:usize = 156;
	pub const RULE_comma_fof_logic_formula:usize = 157;
	pub const RULE_cnf_formula:usize = 158;
	pub const RULE_cnf_disjunction:usize = 159;
	pub const RULE_cnf_literal:usize = 160;
	pub const RULE_thf_quantifier:usize = 161;
	pub const RULE_thf_unary_connective:usize = 162;
	pub const RULE_th0_quantifier:usize = 163;
	pub const RULE_type_quantifier:usize = 164;
	pub const RULE_subtype_sign:usize = 165;
	pub const RULE_tff_unary_connective:usize = 166;
	pub const RULE_tff_quantifier:usize = 167;
	pub const RULE_fof_quantifier:usize = 168;
	pub const RULE_nonassoc_connective:usize = 169;
	pub const RULE_assoc_connective:usize = 170;
	pub const RULE_unary_connective:usize = 171;
	pub const RULE_gentzen_arrow:usize = 172;
	pub const RULE_assignment:usize = 173;
	pub const RULE_identical:usize = 174;
	pub const RULE_typeable_atom:usize = 175;
	pub const RULE_atomic_type:usize = 176;
	pub const RULE_type_constant:usize = 177;
	pub const RULE_type_functor:usize = 178;
	pub const RULE_defined_type:usize = 179;
	pub const RULE_system_type:usize = 180;
	pub const RULE_defined_infix_pred:usize = 181;
	pub const RULE_infix_equality:usize = 182;
	pub const RULE_infix_inequality:usize = 183;
	pub const RULE_constant:usize = 184;
	pub const RULE_functor:usize = 185;
	pub const RULE_defined_constant:usize = 186;
	pub const RULE_defined_functor:usize = 187;
	pub const RULE_system_constant:usize = 188;
	pub const RULE_system_functor:usize = 189;
	pub const RULE_th1_defined_term:usize = 190;
	pub const RULE_defined_term:usize = 191;
	pub const RULE_variable:usize = 192;
	pub const RULE_source:usize = 193;
	pub const RULE_sources:usize = 194;
	pub const RULE_dag_source:usize = 195;
	pub const RULE_inference_record:usize = 196;
	pub const RULE_inference_rule:usize = 197;
	pub const RULE_internal_source:usize = 198;
	pub const RULE_intro_type:usize = 199;
	pub const RULE_external_source:usize = 200;
	pub const RULE_file_source:usize = 201;
	pub const RULE_file_info:usize = 202;
	pub const RULE_parents:usize = 203;
	pub const RULE_parent_list:usize = 204;
	pub const RULE_comma_parent_info:usize = 205;
	pub const RULE_parent_info:usize = 206;
	pub const RULE_parent_details:usize = 207;
	pub const RULE_optional_info:usize = 208;
	pub const RULE_useful_info:usize = 209;
	pub const RULE_include:usize = 210;
	pub const RULE_include_optionals:usize = 211;
	pub const RULE_formula_selection:usize = 212;
	pub const RULE_name_list:usize = 213;
	pub const RULE_space_name:usize = 214;
	pub const RULE_general_term:usize = 215;
	pub const RULE_general_data:usize = 216;
	pub const RULE_general_function:usize = 217;
	pub const RULE_formula_data:usize = 218;
	pub const RULE_general_list:usize = 219;
	pub const RULE_general_terms:usize = 220;
	pub const RULE_comma_general_term:usize = 221;
	pub const RULE_name:usize = 222;
	pub const RULE_atomic_word:usize = 223;
	pub const RULE_atomic_defined_word:usize = 224;
	pub const RULE_atomic_system_word:usize = 225;
	pub const RULE_number:usize = 226;
	pub const RULE_file_name:usize = 227;
	pub const RULE_nothing:usize = 228;
	pub const ruleNames: [&'static str; 229] =  [
		"tptp_file", "tptp_input", "annotated_formula", "tpi_annotated", "tpi_formula",
		"thf_annotated", "tff_annotated", "tcf_annotated", "fof_annotated", "cnf_annotated",
		"annotations", "formula_role", "thf_formula", "thf_logic_formula", "thf_binary_formula",
		"thf_binary_pair", "thf_binary_list", "thf_or_formula", "thf_and_formula",
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
		"tff_logic_formula", "tff_binary_formula", "tff_binary_pair", "tff_binary_list",
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
		"ntf_connective_name", "ntf_defined_connective", "ntf_index", "ntf_short_connective",
		"tcf_formula", "tcf_logic_formula", "tcf_quantified_formula", "fof_formula",
		"fof_logic_formula", "fof_binary_formula", "fof_binary_pair", "fof_binary_list",
		"fof_or_formula", "fof_and_formula", "fof_unary_formula", "fof_infix_unary",
		"fof_unit_formula", "fof_unitary_formula", "fof_quantified_formula", "fof_variable_list",
		"fof_atomic_formula", "fof_plain_atomic_formula", "fof_defined_atomic_formula",
		"fof_defined_plain_formula", "fof_defined_infix_formula", "fof_system_atomic_formula",
		"fof_plain_term", "fof_defined_term", "fof_defined_atomic_term", "fof_defined_plain_term",
		"fof_system_term", "fof_arguments", "fof_term", "fof_function_term", "fof_sequent",
		"fof_formula_tuple", "fof_formula_tuple_list", "comma_fof_logic_formula",
		"cnf_formula", "cnf_disjunction", "cnf_literal", "thf_quantifier", "thf_unary_connective",
		"th0_quantifier", "type_quantifier", "subtype_sign", "tff_unary_connective",
		"tff_quantifier", "fof_quantifier", "nonassoc_connective", "assoc_connective",
		"unary_connective", "gentzen_arrow", "assignment", "identical", "typeable_atom",
		"atomic_type", "type_constant", "type_functor", "defined_type", "system_type",
		"defined_infix_pred", "infix_equality", "infix_inequality", "constant",
		"functor", "defined_constant", "defined_functor", "system_constant", "system_functor",
		"th1_defined_term", "defined_term", "variable", "source", "sources", "dag_source",
		"inference_record", "inference_rule", "internal_source", "intro_type",
		"external_source", "file_source", "file_info", "parents", "parent_list",
		"comma_parent_info", "parent_info", "parent_details", "optional_info",
		"useful_info", "include", "include_optionals", "formula_selection", "name_list",
		"space_name", "general_term", "general_data", "general_function", "formula_data",
		"general_list", "general_terms", "comma_general_term", "name", "atomic_word",
		"atomic_defined_word", "atomic_system_word", "number", "file_name", "nothing"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;99] = [
		None, Some("'tpi('"), Some("','"), Some("').'"), Some("'thf('"), Some("'tff('"),
		Some("'tcf('"), Some("'fof('"), Some("'cnf('"), Some("'-'"), Some("'&'"),
		Some("'@'"), Some("'('"), Some("')'"), Some("'['"), Some("']'"), Some("':'"),
		Some("'$let('"), Some("'[]'"), Some("'{'"), Some("'}'"), Some("')}'"),
		Some("'[.]'"), Some("'.'"), Some("'{.}'"), Some("'(.)'"), Some("'!'"),
		Some("'~'"), Some("'^'"), Some("'@+'"), Some("'@-'"), Some("'!>'"), Some("'?'"),
		Some("'<<'"), Some("'<=>'"), Some("'=>'"), Some("'<='"), Some("'<~>'"),
		Some("'~&'"), Some("'-->'"), Some("':='"), Some("'=='"), Some("'='"),
		Some("'!='"), Some("'!!'"), Some("'??'"), Some("'@@+'"), Some("'@@-'"),
		Some("'@='"), Some("'inference('"), Some("'introduced('"), Some("'file('"),
		Some("'include('"), Some("'$thf('"), Some("'$tff('"), Some("'$fof('"),
		Some("'$cnf('"), Some("'$fot('"), None, None, None, None, None, None,
		None, None, None, None, None, None, None, None, None, None, None, None,
		None, None, None, None, None, None, None, None, None, None, None, Some("'+'"),
		Some("'>'"), Some("'<'"), Some("'#'"), None, None, None, Some("'''"),
		None, None, None, Some("'\\\\'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;104]  = [
		None, None, None, None, None, None, None, None, None, None, None, None,
		None, None, None, None, None, None, None, None, None, None, None, None,
		None, None, None, None, None, None, None, None, None, None, None, None,
		None, None, None, None, None, None, None, None, None, None, None, None,
		None, None, None, None, None, None, None, None, None, None, Some("WS"),
		Some("Comment_line"), Some("Comment_block"), Some("Single_quoted"), Some("Back_quoted"),
		Some("Distinct_object"), Some("Dollar_word"), Some("Dollar_dollar_word"),
		Some("Upper_word"), Some("Lower_word"), Some("Real"), Some("Signed_real"),
		Some("Unsigned_real"), Some("Decimal_exponent"), Some("Decimal_fraction"),
		Some("Exp_integer"), Some("Signed_exp_integer"), Some("Rational"), Some("Signed_rational"),
		Some("Unsigned_rational"), Some("Integer"), Some("Signed_integer"), Some("Unsigned_integer"),
		Some("Positive_integer"), Some("Integer_digits"), Some("Slash"), Some("Slosh"),
		Some("Vline"), Some("Star"), Some("Plus"), Some("Arrow"), Some("Less_sign"),
		Some("Hash"), Some("Not_star_slash"), Some("Percentage_sign"), Some("Double_quote"),
		Some("Single_quote"), Some("Back_quote"), Some("Dot"), Some("Slash_char"),
		Some("Slosh_char"), Some("Zero_numeric"), Some("Underscore"), Some("Alpha"),
		Some("Dollar"), Some("Comment_char")
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
	interpreter:Rc<ParserATNSimulator>,
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
		let interpreter = Rc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Rc::clone(&interpreter),
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
					132 => TPTPParser::<'input,I>::fof_or_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					133 => TPTPParser::<'input,I>::fof_and_formula_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					159 => TPTPParser::<'input,I>::cnf_disjunction_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
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
			recog.base.set_state(461);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 498) != 0) || _la==TPTP_T__51 {
				{
				{
				/*InvokeRule tptp_input*/
				recog.base.set_state(458);
				recog.tptp_input()?;

				}
				}
				recog.base.set_state(463);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(464);
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

			recog.base.set_state(468);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__0 |TPTP_T__3 |TPTP_T__4 |TPTP_T__5 |TPTP_T__6 |TPTP_T__7
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule annotated_formula*/
					recog.base.set_state(466);
					recog.annotated_formula()?;

					}
				}

			TPTP_T__51
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule include*/
					recog.base.set_state(467);
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

			recog.base.set_state(476);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__3
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_annotated*/
					recog.base.set_state(470);
					recog.thf_annotated()?;

					}
				}

			TPTP_T__4
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_annotated*/
					recog.base.set_state(471);
					recog.tff_annotated()?;

					}
				}

			TPTP_T__5
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tcf_annotated*/
					recog.base.set_state(472);
					recog.tcf_annotated()?;

					}
				}

			TPTP_T__6
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule fof_annotated*/
					recog.base.set_state(473);
					recog.fof_annotated()?;

					}
				}

			TPTP_T__7
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule cnf_annotated*/
					recog.base.set_state(474);
					recog.cnf_annotated()?;

					}
				}

			TPTP_T__0
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule tpi_annotated*/
					recog.base.set_state(475);
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
			recog.base.set_state(478);
			recog.base.match_token(TPTP_T__0,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(479);
			recog.name()?;

			recog.base.set_state(480);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(481);
			recog.formula_role()?;

			recog.base.set_state(482);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tpi_formula*/
			recog.base.set_state(483);
			recog.tpi_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(484);
			recog.annotations()?;

			recog.base.set_state(485);
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
			recog.base.set_state(487);
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
			recog.base.set_state(489);
			recog.base.match_token(TPTP_T__3,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(490);
			recog.name()?;

			recog.base.set_state(491);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(492);
			recog.formula_role()?;

			recog.base.set_state(493);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_formula*/
			recog.base.set_state(494);
			recog.thf_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(495);
			recog.annotations()?;

			recog.base.set_state(496);
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
			recog.base.set_state(498);
			recog.base.match_token(TPTP_T__4,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(499);
			recog.name()?;

			recog.base.set_state(500);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(501);
			recog.formula_role()?;

			recog.base.set_state(502);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_formula*/
			recog.base.set_state(503);
			recog.tff_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(504);
			recog.annotations()?;

			recog.base.set_state(505);
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
			recog.base.set_state(507);
			recog.base.match_token(TPTP_T__5,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(508);
			recog.name()?;

			recog.base.set_state(509);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(510);
			recog.formula_role()?;

			recog.base.set_state(511);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tcf_formula*/
			recog.base.set_state(512);
			recog.tcf_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(513);
			recog.annotations()?;

			recog.base.set_state(514);
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
			recog.base.set_state(516);
			recog.base.match_token(TPTP_T__6,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(517);
			recog.name()?;

			recog.base.set_state(518);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(519);
			recog.formula_role()?;

			recog.base.set_state(520);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_formula*/
			recog.base.set_state(521);
			recog.fof_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(522);
			recog.annotations()?;

			recog.base.set_state(523);
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
			recog.base.set_state(525);
			recog.base.match_token(TPTP_T__7,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(526);
			recog.name()?;

			recog.base.set_state(527);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(528);
			recog.formula_role()?;

			recog.base.set_state(529);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule cnf_formula*/
			recog.base.set_state(530);
			recog.cnf_formula()?;

			/*InvokeRule annotations*/
			recog.base.set_state(531);
			recog.annotations()?;

			recog.base.set_state(532);
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

			recog.base.set_state(539);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__1
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(534);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule source*/
					recog.base.set_state(535);
					recog.source()?;

					/*InvokeRule optional_info*/
					recog.base.set_state(536);
					recog.optional_info()?;

					}
				}

			TPTP_T__2
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nothing*/
					recog.base.set_state(538);
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

			recog.base.set_state(545);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(541);
					recog.base.match_token(TPTP_Lower_word,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(542);
					recog.base.match_token(TPTP_Lower_word,&mut recog.err_handler)?;

					recog.base.set_state(543);
					recog.base.match_token(TPTP_T__8,&mut recog.err_handler)?;

					/*InvokeRule general_term*/
					recog.base.set_state(544);
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

			recog.base.set_state(550);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(547);
					recog.thf_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(548);
					recog.thf_atom_typing()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_subtype*/
					recog.base.set_state(549);
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

			recog.base.set_state(558);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(552);
					recog.thf_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_unary_formula*/
					recog.base.set_state(553);
					recog.thf_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_binary_formula*/
					recog.base.set_state(554);
					recog.thf_binary_formula()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule thf_defined_infix*/
					recog.base.set_state(555);
					recog.thf_defined_infix()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule thf_definition*/
					recog.base.set_state(556);
					recog.thf_definition()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule thf_sequent*/
					recog.base.set_state(557);
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

fn thf_binary_pair(&self) -> Option<Rc<Thf_binary_pairContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_binary_list(&self) -> Option<Rc<Thf_binary_listContextAll<'input>>> where Self:Sized{
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

			recog.base.set_state(563);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_binary_pair*/
					recog.base.set_state(560);
					recog.thf_binary_pair()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_binary_list*/
					recog.base.set_state(561);
					recog.thf_binary_list()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_binary_type*/
					recog.base.set_state(562);
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
//------------------- thf_binary_pair ----------------
pub type Thf_binary_pairContextAll<'input> = Thf_binary_pairContext<'input>;


pub type Thf_binary_pairContext<'input> = BaseParserRuleContext<'input,Thf_binary_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_binary_pairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_binary_pairContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_binary_pairContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_binary_pair(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_binary_pair(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_binary_pairContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_binary_pair(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_pairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_pair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_pair }
}
antlr4rust::tid!{Thf_binary_pairContextExt<'a>}

impl<'input> Thf_binary_pairContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_pairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_pairContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_pairContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_binary_pairContextExt<'input>>{

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

impl<'input> Thf_binary_pairContextAttrs<'input> for Thf_binary_pairContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_binary_pair(&mut self,)
	-> Result<Rc<Thf_binary_pairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_thf_binary_pair);
        let mut _localctx: Rc<Thf_binary_pairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(565);
			recog.thf_unit_formula()?;

			/*InvokeRule nonassoc_connective*/
			recog.base.set_state(566);
			recog.nonassoc_connective()?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(567);
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
//------------------- thf_binary_list ----------------
pub type Thf_binary_listContextAll<'input> = Thf_binary_listContext<'input>;


pub type Thf_binary_listContext<'input> = BaseParserRuleContext<'input,Thf_binary_listContextExt<'input>>;

#[derive(Clone)]
pub struct Thf_binary_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Thf_binary_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Thf_binary_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_thf_binary_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_thf_binary_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Thf_binary_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_thf_binary_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Thf_binary_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_thf_binary_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_thf_binary_list }
}
antlr4rust::tid!{Thf_binary_listContextExt<'a>}

impl<'input> Thf_binary_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Thf_binary_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Thf_binary_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Thf_binary_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Thf_binary_listContextExt<'input>>{

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

impl<'input> Thf_binary_listContextAttrs<'input> for Thf_binary_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn thf_binary_list(&mut self,)
	-> Result<Rc<Thf_binary_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Thf_binary_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_thf_binary_list);
        let mut _localctx: Rc<Thf_binary_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(572);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(8,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_or_formula*/
					recog.base.set_state(569);
					recog.thf_or_formula_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_and_formula*/
					recog.base.set_state(570);
					recog.thf_and_formula_rec(0)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_apply_formula*/
					recog.base.set_state(571);
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
			recog.base.set_state(575);
			recog.thf_unit_formula()?;

			recog.base.set_state(576);
			recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(577);
			recog.thf_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(584);
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
					recog.base.set_state(579);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(580);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					/*InvokeRule thf_unit_formula*/
					recog.base.set_state(581);
					recog.thf_unit_formula()?;

					}
					}
				}
				recog.base.set_state(586);
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
			recog.base.set_state(588);
			recog.thf_unit_formula()?;

			recog.base.set_state(589);
			recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(590);
			recog.thf_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(597);
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
					recog.base.set_state(592);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(593);
					recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_unit_formula*/
					recog.base.set_state(594);
					recog.thf_unit_formula()?;

					}
					}
				}
				recog.base.set_state(599);
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
			recog.base.set_state(601);
			recog.thf_unit_formula()?;

			recog.base.set_state(602);
			recog.base.match_token(TPTP_T__10,&mut recog.err_handler)?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(603);
			recog.thf_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(610);
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
					recog.base.set_state(605);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(606);
					recog.base.match_token(TPTP_T__10,&mut recog.err_handler)?;

					/*InvokeRule thf_unit_formula*/
					recog.base.set_state(607);
					recog.thf_unit_formula()?;

					}
					}
				}
				recog.base.set_state(612);
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

			recog.base.set_state(616);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(613);
					recog.thf_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_unary_formula*/
					recog.base.set_state(614);
					recog.thf_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_defined_infix*/
					recog.base.set_state(615);
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

			recog.base.set_state(620);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__11 |TPTP_T__13 |TPTP_T__16 |TPTP_T__17 |TPTP_T__18 |TPTP_T__25 |
			TPTP_T__27 |TPTP_T__28 |TPTP_T__29 |TPTP_T__30 |TPTP_T__31 |TPTP_T__43 |
			TPTP_T__44 |TPTP_T__45 |TPTP_T__46 |TPTP_T__47 |TPTP_Single_quoted |TPTP_Back_quoted |
			TPTP_Distinct_object |TPTP_Dollar_word |TPTP_Dollar_dollar_word |TPTP_Upper_word |
			TPTP_Lower_word |TPTP_Real |TPTP_Rational |TPTP_Integer |TPTP_Hash
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_formula*/
					recog.base.set_state(618);
					recog.thf_unitary_formula()?;

					}
				}

			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_T__26 |TPTP_Less_sign
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_prefix_unary*/
					recog.base.set_state(619);
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

			recog.base.set_state(629);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(14,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_quantified_formula*/
					recog.base.set_state(622);
					recog.thf_quantified_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_atomic_formula*/
					recog.base.set_state(623);
					recog.thf_atomic_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(624);
					recog.variable()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(625);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(626);
					recog.thf_logic_formula()?;

					recog.base.set_state(627);
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
			recog.base.set_state(631);
			recog.thf_quantification()?;

			/*InvokeRule thf_unit_formula*/
			recog.base.set_state(632);
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
			recog.base.set_state(634);
			recog.thf_quantifier()?;

			recog.base.set_state(635);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule thf_variable_list*/
			recog.base.set_state(636);
			recog.thf_variable_list()?;

			recog.base.set_state(637);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(638);
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

			recog.base.set_state(645);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(15,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_typed_variable*/
					recog.base.set_state(640);
					recog.thf_typed_variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_typed_variable*/
					recog.base.set_state(641);
					recog.thf_typed_variable()?;

					recog.base.set_state(642);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule thf_variable_list*/
					recog.base.set_state(643);
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
			recog.base.set_state(647);
			recog.variable()?;

			recog.base.set_state(648);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule thf_top_level_type*/
			recog.base.set_state(649);
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

			recog.base.set_state(653);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_T__26 |TPTP_Less_sign
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_prefix_unary*/
					recog.base.set_state(651);
					recog.thf_prefix_unary()?;

					}
				}

			TPTP_T__11 |TPTP_T__13 |TPTP_T__16 |TPTP_T__17 |TPTP_T__18 |TPTP_T__43 |
			TPTP_T__44 |TPTP_T__45 |TPTP_T__46 |TPTP_T__47 |TPTP_Single_quoted |TPTP_Back_quoted |
			TPTP_Distinct_object |TPTP_Dollar_word |TPTP_Dollar_dollar_word |TPTP_Upper_word |
			TPTP_Lower_word |TPTP_Real |TPTP_Rational |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_infix_unary*/
					recog.base.set_state(652);
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
			recog.base.set_state(655);
			recog.thf_unary_connective()?;

			/*InvokeRule thf_preunit_formula*/
			recog.base.set_state(656);
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
			recog.base.set_state(658);
			recog.thf_unitary_term()?;

			/*InvokeRule infix_inequality*/
			recog.base.set_state(659);
			recog.infix_inequality()?;

			/*InvokeRule thf_unitary_term*/
			recog.base.set_state(660);
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

			recog.base.set_state(666);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_plain_atomic*/
					recog.base.set_state(662);
					recog.thf_plain_atomic()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_defined_atomic*/
					recog.base.set_state(663);
					recog.thf_defined_atomic()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_system_atomic*/
					recog.base.set_state(664);
					recog.thf_system_atomic()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule thf_fof_function*/
					recog.base.set_state(665);
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

			recog.base.set_state(670);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule constant*/
					recog.base.set_state(668);
					recog.constant()?;

					}
				}

			TPTP_T__13 |TPTP_T__17
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_tuple*/
					recog.base.set_state(669);
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

			recog.base.set_state(680);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(672);
					recog.defined_constant()?;

					}
				}

			TPTP_T__43 |TPTP_T__44 |TPTP_T__45 |TPTP_T__46 |TPTP_T__47 |TPTP_Distinct_object |
			TPTP_Real |TPTP_Rational |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_defined_term*/
					recog.base.set_state(673);
					recog.thf_defined_term()?;

					}
				}

			TPTP_T__11
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(674);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_conn_term*/
					recog.base.set_state(675);
					recog.thf_conn_term()?;

					recog.base.set_state(676);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__18
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule nhf_long_connective*/
					recog.base.set_state(678);
					recog.nhf_long_connective()?;

					}
				}

			TPTP_T__16
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule thf_let*/
					recog.base.set_state(679);
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

			recog.base.set_state(684);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Distinct_object |TPTP_Real |TPTP_Rational |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(682);
					recog.defined_term()?;

					}
				}

			TPTP_T__43 |TPTP_T__44 |TPTP_T__45 |TPTP_T__46 |TPTP_T__47
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule th1_defined_term*/
					recog.base.set_state(683);
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
			recog.base.set_state(686);
			recog.thf_unitary_term()?;

			/*InvokeRule defined_infix_pred*/
			recog.base.set_state(687);
			recog.defined_infix_pred()?;

			/*InvokeRule thf_unitary_term*/
			recog.base.set_state(688);
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
			recog.base.set_state(690);
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
			recog.base.set_state(692);
			recog.base.match_token(TPTP_T__16,&mut recog.err_handler)?;

			/*InvokeRule thf_let_types*/
			recog.base.set_state(693);
			recog.thf_let_types()?;

			recog.base.set_state(694);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_let_defns*/
			recog.base.set_state(695);
			recog.thf_let_defns()?;

			recog.base.set_state(696);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(697);
			recog.thf_logic_formula()?;

			recog.base.set_state(698);
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

			recog.base.set_state(705);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__11 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |
			TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(700);
					recog.thf_atom_typing()?;

					}
				}

			TPTP_T__13
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(701);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule thf_atom_typing_list*/
					recog.base.set_state(702);
					recog.thf_atom_typing_list()?;

					recog.base.set_state(703);
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

			recog.base.set_state(712);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(707);
					recog.thf_atom_typing()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(708);
					recog.thf_atom_typing()?;

					recog.base.set_state(709);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule thf_atom_typing_list*/
					recog.base.set_state(710);
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

			recog.base.set_state(719);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_let_defn*/
					recog.base.set_state(714);
					recog.thf_let_defn()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(715);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule thf_let_defn_list*/
					recog.base.set_state(716);
					recog.thf_let_defn_list()?;

					recog.base.set_state(717);
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
			recog.base.set_state(721);
			recog.thf_logic_formula()?;

			/*InvokeRule assignment*/
			recog.base.set_state(722);
			recog.assignment()?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(723);
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

			recog.base.set_state(730);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(24,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_let_defn*/
					recog.base.set_state(725);
					recog.thf_let_defn()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_let_defn*/
					recog.base.set_state(726);
					recog.thf_let_defn()?;

					recog.base.set_state(727);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule thf_let_defn_list*/
					recog.base.set_state(728);
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

			recog.base.set_state(738);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(25,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_atomic_formula*/
					recog.base.set_state(732);
					recog.thf_atomic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(733);
					recog.variable()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(734);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(735);
					recog.thf_logic_formula()?;

					recog.base.set_state(736);
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

			recog.base.set_state(745);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nonassoc_connective*/
					recog.base.set_state(740);
					recog.nonassoc_connective()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule assoc_connective*/
					recog.base.set_state(741);
					recog.assoc_connective()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule infix_equality*/
					recog.base.set_state(742);
					recog.infix_equality()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule infix_inequality*/
					recog.base.set_state(743);
					recog.infix_inequality()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule thf_unary_connective*/
					recog.base.set_state(744);
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

			recog.base.set_state(752);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(747);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(748);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule thf_formula_list*/
					recog.base.set_state(749);
					recog.thf_formula_list()?;

					recog.base.set_state(750);
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

fn defined_functor(&self) -> Option<Rc<Defined_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn thf_arguments(&self) -> Option<Rc<Thf_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn system_functor(&self) -> Option<Rc<System_functorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functor(&self) -> Option<Rc<FunctorContextAll<'input>>> where Self:Sized{
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

			recog.base.set_state(769);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_functor*/
					recog.base.set_state(754);
					recog.defined_functor()?;

					recog.base.set_state(755);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(756);
					recog.thf_arguments()?;

					recog.base.set_state(757);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_Dollar_dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule system_functor*/
					recog.base.set_state(759);
					recog.system_functor()?;

					recog.base.set_state(760);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(761);
					recog.thf_arguments()?;

					recog.base.set_state(762);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule functor*/
					recog.base.set_state(764);
					recog.functor()?;

					recog.base.set_state(765);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(766);
					recog.thf_arguments()?;

					recog.base.set_state(767);
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
			recog.base.set_state(771);
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
			recog.base.set_state(773);
			recog.thf_logic_formula()?;

			recog.base.set_state(777);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_thf_logic_formula*/
				recog.base.set_state(774);
				recog.comma_thf_logic_formula()?;

				}
				}
				recog.base.set_state(779);
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
			recog.base.set_state(780);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(781);
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

fn typeable_atom(&self) -> Option<Rc<Typeable_atomContextAll<'input>>> where Self:Sized{
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

			recog.base.set_state(791);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule typeable_atom*/
					recog.base.set_state(783);
					recog.typeable_atom()?;

					recog.base.set_state(784);
					recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

					/*InvokeRule thf_top_level_type*/
					recog.base.set_state(785);
					recog.thf_top_level_type()?;

					}
				}

			TPTP_T__11
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(787);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_atom_typing*/
					recog.base.set_state(788);
					recog.thf_atom_typing()?;

					recog.base.set_state(789);
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

			recog.base.set_state(796);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(793);
					recog.thf_unitary_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_mapping_type*/
					recog.base.set_state(794);
					recog.thf_mapping_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_apply_type*/
					recog.base.set_state(795);
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
			recog.base.set_state(798);
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
			recog.base.set_state(800);
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

			recog.base.set_state(805);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_mapping_type*/
					recog.base.set_state(802);
					recog.thf_mapping_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_xprod_type*/
					recog.base.set_state(803);
					recog.thf_xprod_type_rec(0)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule thf_union_type*/
					recog.base.set_state(804);
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

			recog.base.set_state(815);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(807);
					recog.thf_unitary_type()?;

					recog.base.set_state(808);
					recog.base.match_token(TPTP_Arrow,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(809);
					recog.thf_unitary_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(811);
					recog.thf_unitary_type()?;

					recog.base.set_state(812);
					recog.base.match_token(TPTP_Arrow,&mut recog.err_handler)?;

					/*InvokeRule thf_mapping_type*/
					recog.base.set_state(813);
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
			recog.base.set_state(818);
			recog.thf_unitary_type()?;

			recog.base.set_state(819);
			recog.base.match_token(TPTP_Star,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(820);
			recog.thf_unitary_type()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(827);
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
					recog.base.set_state(822);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(823);
					recog.base.match_token(TPTP_Star,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(824);
					recog.thf_unitary_type()?;

					}
					}
				}
				recog.base.set_state(829);
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
			recog.base.set_state(831);
			recog.thf_unitary_type()?;

			recog.base.set_state(832);
			recog.base.match_token(TPTP_Plus,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(833);
			recog.thf_unitary_type()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(840);
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
					recog.base.set_state(835);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(836);
					recog.base.match_token(TPTP_Plus,&mut recog.err_handler)?;

					/*InvokeRule thf_unitary_type*/
					recog.base.set_state(837);
					recog.thf_unitary_type()?;

					}
					}
				}
				recog.base.set_state(842);
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

fn atomic_type_all(&self) ->  Vec<Rc<Atomic_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn atomic_type(&self, i: usize) -> Option<Rc<Atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn subtype_sign(&self) -> Option<Rc<Subtype_signContextAll<'input>>> where Self:Sized{
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
			/*InvokeRule atomic_type*/
			recog.base.set_state(843);
			recog.atomic_type()?;

			/*InvokeRule subtype_sign*/
			recog.base.set_state(844);
			recog.subtype_sign()?;

			/*InvokeRule atomic_type*/
			recog.base.set_state(845);
			recog.atomic_type()?;

			}
			Ok(())
		})();
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
			recog.base.set_state(847);
			recog.thf_atomic_formula()?;

			/*InvokeRule identical*/
			recog.base.set_state(848);
			recog.identical()?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(849);
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
			recog.base.set_state(851);
			recog.thf_tuple()?;

			/*InvokeRule gentzen_arrow*/
			recog.base.set_state(852);
			recog.gentzen_arrow()?;

			/*InvokeRule thf_tuple*/
			recog.base.set_state(853);
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

			recog.base.set_state(858);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(36,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(855);
					recog.tff_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(856);
					recog.tff_atom_typing()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tff_subtype*/
					recog.base.set_state(857);
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

			recog.base.set_state(866);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(860);
					recog.tff_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_unary_formula*/
					recog.base.set_state(861);
					recog.tff_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tff_binary_formula*/
					recog.base.set_state(862);
					recog.tff_binary_formula()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule tff_defined_infix*/
					recog.base.set_state(863);
					recog.tff_defined_infix()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule txf_definition*/
					recog.base.set_state(864);
					recog.txf_definition()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule txf_sequent*/
					recog.base.set_state(865);
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

fn tff_binary_pair(&self) -> Option<Rc<Tff_binary_pairContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_binary_list(&self) -> Option<Rc<Tff_binary_listContextAll<'input>>> where Self:Sized{
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

			recog.base.set_state(870);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(38,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_binary_pair*/
					recog.base.set_state(868);
					recog.tff_binary_pair()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_binary_list*/
					recog.base.set_state(869);
					recog.tff_binary_list()?;

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
//------------------- tff_binary_pair ----------------
pub type Tff_binary_pairContextAll<'input> = Tff_binary_pairContext<'input>;


pub type Tff_binary_pairContext<'input> = BaseParserRuleContext<'input,Tff_binary_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_binary_pairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_binary_pairContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_binary_pairContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_binary_pair(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_binary_pair(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_binary_pairContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_binary_pair(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_binary_pairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_binary_pair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_binary_pair }
}
antlr4rust::tid!{Tff_binary_pairContextExt<'a>}

impl<'input> Tff_binary_pairContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_binary_pairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_binary_pairContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_binary_pairContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_binary_pairContextExt<'input>>{

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

impl<'input> Tff_binary_pairContextAttrs<'input> for Tff_binary_pairContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_binary_pair(&mut self,)
	-> Result<Rc<Tff_binary_pairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_binary_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_tff_binary_pair);
        let mut _localctx: Rc<Tff_binary_pairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(872);
			recog.tff_unit_formula()?;

			/*InvokeRule nonassoc_connective*/
			recog.base.set_state(873);
			recog.nonassoc_connective()?;

			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(874);
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
//------------------- tff_binary_list ----------------
pub type Tff_binary_listContextAll<'input> = Tff_binary_listContext<'input>;


pub type Tff_binary_listContext<'input> = BaseParserRuleContext<'input,Tff_binary_listContextExt<'input>>;

#[derive(Clone)]
pub struct Tff_binary_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Tff_binary_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Tff_binary_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tff_binary_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tff_binary_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Tff_binary_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_tff_binary_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Tff_binary_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tff_binary_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tff_binary_list }
}
antlr4rust::tid!{Tff_binary_listContextExt<'a>}

impl<'input> Tff_binary_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Tff_binary_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Tff_binary_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Tff_binary_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Tff_binary_listContextExt<'input>>{

fn tff_or_formula(&self) -> Option<Rc<Tff_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tff_and_formula(&self) -> Option<Rc<Tff_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Tff_binary_listContextAttrs<'input> for Tff_binary_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tff_binary_list(&mut self,)
	-> Result<Rc<Tff_binary_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Tff_binary_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_tff_binary_list);
        let mut _localctx: Rc<Tff_binary_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(878);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(39,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_or_formula*/
					recog.base.set_state(876);
					recog.tff_or_formula_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_and_formula*/
					recog.base.set_state(877);
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
			recog.base.set_state(881);
			recog.tff_unit_formula()?;

			recog.base.set_state(882);
			recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(883);
			recog.tff_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(890);
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
					recog.base.set_state(885);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(886);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					/*InvokeRule tff_unit_formula*/
					recog.base.set_state(887);
					recog.tff_unit_formula()?;

					}
					}
				}
				recog.base.set_state(892);
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
			recog.base.set_state(894);
			recog.tff_unit_formula()?;

			recog.base.set_state(895);
			recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(896);
			recog.tff_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(903);
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
					recog.base.set_state(898);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(899);
					recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_unit_formula*/
					recog.base.set_state(900);
					recog.tff_unit_formula()?;

					}
					}
				}
				recog.base.set_state(905);
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

			recog.base.set_state(909);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(42,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(906);
					recog.tff_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_unary_formula*/
					recog.base.set_state(907);
					recog.tff_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tff_defined_infix*/
					recog.base.set_state(908);
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

			recog.base.set_state(913);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__11 |TPTP_T__16 |TPTP_T__18 |TPTP_T__25 |TPTP_T__31 |TPTP_Single_quoted |
			TPTP_Back_quoted |TPTP_Dollar_word |TPTP_Dollar_dollar_word |TPTP_Upper_word |
			TPTP_Lower_word |TPTP_Hash
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(911);
					recog.tff_unitary_formula()?;

					}
				}

			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_T__26 |TPTP_Less_sign
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_prefix_unary*/
					recog.base.set_state(912);
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

			recog.base.set_state(922);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__25 |TPTP_T__31 |TPTP_Hash
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_quantified_formula*/
					recog.base.set_state(915);
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
					recog.base.set_state(916);
					recog.tff_atomic_formula()?;

					}
				}

			TPTP_Upper_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule txf_unitary_formula*/
					recog.base.set_state(917);
					recog.txf_unitary_formula()?;

					}
				}

			TPTP_T__11
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(918);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(919);
					recog.tff_logic_formula()?;

					recog.base.set_state(920);
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
			recog.base.set_state(924);
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
			recog.base.set_state(926);
			recog.tff_quantifier()?;

			recog.base.set_state(927);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(928);
			recog.tff_variable_list()?;

			recog.base.set_state(929);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(930);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule tff_unit_formula*/
			recog.base.set_state(931);
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

			recog.base.set_state(938);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(45,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_variable*/
					recog.base.set_state(933);
					recog.tff_variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_variable*/
					recog.base.set_state(934);
					recog.tff_variable()?;

					recog.base.set_state(935);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_variable_list*/
					recog.base.set_state(936);
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

			recog.base.set_state(942);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(46,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_typed_variable*/
					recog.base.set_state(940);
					recog.tff_typed_variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(941);
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
			recog.base.set_state(944);
			recog.variable()?;

			recog.base.set_state(945);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(946);
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

			recog.base.set_state(950);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_T__26 |TPTP_Less_sign
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_prefix_unary*/
					recog.base.set_state(948);
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
					recog.base.set_state(949);
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
			recog.base.set_state(952);
			recog.tff_unary_connective()?;

			/*InvokeRule tff_preunit_formula*/
			recog.base.set_state(953);
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
			recog.base.set_state(955);
			recog.tff_unitary_term()?;

			/*InvokeRule infix_inequality*/
			recog.base.set_state(956);
			recog.infix_inequality()?;

			/*InvokeRule tff_unitary_term*/
			recog.base.set_state(957);
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

			recog.base.set_state(962);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_plain_atomic*/
					recog.base.set_state(959);
					recog.tff_plain_atomic()?;

					}
				}

			TPTP_T__16 |TPTP_T__18 |TPTP_Dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_defined_atomic*/
					recog.base.set_state(960);
					recog.tff_defined_atomic()?;

					}
				}

			TPTP_Dollar_dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tff_system_atomic*/
					recog.base.set_state(961);
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

			recog.base.set_state(970);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(49,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule constant*/
					recog.base.set_state(964);
					recog.constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule functor*/
					recog.base.set_state(965);
					recog.functor()?;

					recog.base.set_state(966);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_arguments*/
					recog.base.set_state(967);
					recog.tff_arguments()?;

					recog.base.set_state(968);
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
			recog.base.set_state(972);
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

			recog.base.set_state(982);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(50,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(974);
					recog.defined_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_functor*/
					recog.base.set_state(975);
					recog.defined_functor()?;

					recog.base.set_state(976);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_arguments*/
					recog.base.set_state(977);
					recog.tff_arguments()?;

					recog.base.set_state(978);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule nxf_atom*/
					recog.base.set_state(980);
					recog.nxf_atom()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule txf_let*/
					recog.base.set_state(981);
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
			recog.base.set_state(984);
			recog.tff_unitary_term()?;

			/*InvokeRule defined_infix_pred*/
			recog.base.set_state(985);
			recog.defined_infix_pred()?;

			/*InvokeRule tff_unitary_term*/
			recog.base.set_state(986);
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

			recog.base.set_state(994);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(51,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule system_constant*/
					recog.base.set_state(988);
					recog.system_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule system_functor*/
					recog.base.set_state(989);
					recog.system_functor()?;

					recog.base.set_state(990);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_arguments*/
					recog.base.set_state(991);
					recog.tff_arguments()?;

					recog.base.set_state(992);
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
			recog.base.set_state(996);
			recog.base.match_token(TPTP_T__16,&mut recog.err_handler)?;

			/*InvokeRule txf_let_types*/
			recog.base.set_state(997);
			recog.txf_let_types()?;

			recog.base.set_state(998);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule txf_let_defns*/
			recog.base.set_state(999);
			recog.txf_let_defns()?;

			recog.base.set_state(1000);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_term*/
			recog.base.set_state(1001);
			recog.tff_term()?;

			recog.base.set_state(1002);
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

			recog.base.set_state(1009);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__11 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |
			TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1004);
					recog.tff_atom_typing()?;

					}
				}

			TPTP_T__13
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1005);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_atom_typing_list*/
					recog.base.set_state(1006);
					recog.tff_atom_typing_list()?;

					recog.base.set_state(1007);
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

			recog.base.set_state(1016);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(53,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1011);
					recog.tff_atom_typing()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1012);
					recog.tff_atom_typing()?;

					recog.base.set_state(1013);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_atom_typing_list*/
					recog.base.set_state(1014);
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

			recog.base.set_state(1023);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(54,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule txf_let_defn*/
					recog.base.set_state(1018);
					recog.txf_let_defn()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1019);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule txf_let_defn_list*/
					recog.base.set_state(1020);
					recog.txf_let_defn_list()?;

					recog.base.set_state(1021);
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
			recog.base.set_state(1025);
			recog.txf_let_LHS()?;

			/*InvokeRule assignment*/
			recog.base.set_state(1026);
			recog.assignment()?;

			/*InvokeRule tff_term*/
			recog.base.set_state(1027);
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

			recog.base.set_state(1031);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_plain_atomic*/
					recog.base.set_state(1029);
					recog.tff_plain_atomic()?;

					}
				}

			TPTP_T__13 |TPTP_T__17
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule txf_tuple*/
					recog.base.set_state(1030);
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

			recog.base.set_state(1038);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(56,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule txf_let_defn*/
					recog.base.set_state(1033);
					recog.txf_let_defn()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule txf_let_defn*/
					recog.base.set_state(1034);
					recog.txf_let_defn()?;

					recog.base.set_state(1035);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule txf_let_defn_list*/
					recog.base.set_state(1036);
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
			recog.base.set_state(1040);
			recog.nxf_long_connective()?;

			recog.base.set_state(1041);
			recog.base.match_token(TPTP_T__10,&mut recog.err_handler)?;

			recog.base.set_state(1042);
			recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

			/*InvokeRule tff_arguments*/
			recog.base.set_state(1043);
			recog.tff_arguments()?;

			recog.base.set_state(1044);
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

			recog.base.set_state(1049);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(57,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(1046);
					recog.tff_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(1047);
					recog.defined_term()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule txf_tuple*/
					recog.base.set_state(1048);
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

			recog.base.set_state(1059);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__16 |TPTP_T__18 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Dollar_word |
			TPTP_Dollar_dollar_word |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_formula*/
					recog.base.set_state(1051);
					recog.tff_atomic_formula()?;

					}
				}

			TPTP_Distinct_object |TPTP_Real |TPTP_Rational |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(1052);
					recog.defined_term()?;

					}
				}

			TPTP_T__13 |TPTP_T__17
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule txf_tuple*/
					recog.base.set_state(1053);
					recog.txf_tuple()?;

					}
				}

			TPTP_Upper_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1054);
					recog.variable()?;

					}
				}

			TPTP_T__11
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1055);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(1056);
					recog.tff_logic_formula()?;

					recog.base.set_state(1057);
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

			recog.base.set_state(1066);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1061);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1062);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_arguments*/
					recog.base.set_state(1063);
					recog.tff_arguments()?;

					recog.base.set_state(1064);
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
			recog.base.set_state(1068);
			recog.tff_term()?;

			recog.base.set_state(1072);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_tff_term*/
				recog.base.set_state(1069);
				recog.comma_tff_term()?;

				}
				}
				recog.base.set_state(1074);
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
			recog.base.set_state(1075);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_term*/
			recog.base.set_state(1076);
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

fn typeable_atom(&self) -> Option<Rc<Typeable_atomContextAll<'input>>> where Self:Sized{
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

			recog.base.set_state(1086);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule typeable_atom*/
					recog.base.set_state(1078);
					recog.typeable_atom()?;

					recog.base.set_state(1079);
					recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(1080);
					recog.tff_top_level_type()?;

					}
				}

			TPTP_T__11
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1082);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1083);
					recog.tff_atom_typing()?;

					recog.base.set_state(1084);
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

			recog.base.set_state(1090);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(62,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1088);
					recog.tff_atomic_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_non_atomic_type*/
					recog.base.set_state(1089);
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

			recog.base.set_state(1098);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(63,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_mapping_type*/
					recog.base.set_state(1092);
					recog.tff_mapping_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tf1_quantified_type*/
					recog.base.set_state(1093);
					recog.tf1_quantified_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1094);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_non_atomic_type*/
					recog.base.set_state(1095);
					recog.tff_non_atomic_type()?;

					recog.base.set_state(1096);
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
			recog.base.set_state(1100);
			recog.type_quantifier()?;

			recog.base.set_state(1101);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(1102);
			recog.tff_variable_list()?;

			recog.base.set_state(1103);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(1104);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule tff_monotype*/
			recog.base.set_state(1105);
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

			recog.base.set_state(1113);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(64,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1107);
					recog.tff_atomic_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1108);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_mapping_type*/
					recog.base.set_state(1109);
					recog.tff_mapping_type()?;

					recog.base.set_state(1110);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule tf1_quantified_type*/
					recog.base.set_state(1112);
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

			recog.base.set_state(1120);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(65,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1115);
					recog.tff_atomic_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1116);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_xprod_type*/
					recog.base.set_state(1117);
					recog.tff_xprod_type_rec(0)?;

					recog.base.set_state(1118);
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

			recog.base.set_state(1135);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(66,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule type_constant*/
					recog.base.set_state(1122);
					recog.type_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_type*/
					recog.base.set_state(1123);
					recog.defined_type()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1124);
					recog.variable()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule type_functor*/
					recog.base.set_state(1125);
					recog.type_functor()?;

					recog.base.set_state(1126);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_type_arguments*/
					recog.base.set_state(1127);
					recog.tff_type_arguments()?;

					recog.base.set_state(1128);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1130);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1131);
					recog.tff_atomic_type()?;

					recog.base.set_state(1132);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule txf_tuple_type*/
					recog.base.set_state(1134);
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

			recog.base.set_state(1142);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(67,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1137);
					recog.tff_atomic_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1138);
					recog.tff_atomic_type()?;

					recog.base.set_state(1139);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_type_arguments*/
					recog.base.set_state(1140);
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
			recog.base.set_state(1144);
			recog.tff_unitary_type()?;

			recog.base.set_state(1145);
			recog.base.match_token(TPTP_Arrow,&mut recog.err_handler)?;

			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(1146);
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
			recog.base.set_state(1149);
			recog.tff_unitary_type()?;

			recog.base.set_state(1150);
			recog.base.match_token(TPTP_Star,&mut recog.err_handler)?;

			/*InvokeRule tff_atomic_type*/
			recog.base.set_state(1151);
			recog.tff_atomic_type()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1158);
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
					recog.base.set_state(1153);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1154);
					recog.base.match_token(TPTP_Star,&mut recog.err_handler)?;

					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1155);
					recog.tff_atomic_type()?;

					}
					}
				}
				recog.base.set_state(1160);
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
			recog.base.set_state(1161);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_type_list*/
			recog.base.set_state(1162);
			recog.tff_type_list()?;

			recog.base.set_state(1163);
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

			recog.base.set_state(1170);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(69,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(1165);
					recog.tff_top_level_type()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(1166);
					recog.tff_top_level_type()?;

					recog.base.set_state(1167);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_type_list*/
					recog.base.set_state(1168);
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

fn atomic_type_all(&self) ->  Vec<Rc<Atomic_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn atomic_type(&self, i: usize) -> Option<Rc<Atomic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn subtype_sign(&self) -> Option<Rc<Subtype_signContextAll<'input>>> where Self:Sized{
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
			/*InvokeRule atomic_type*/
			recog.base.set_state(1172);
			recog.atomic_type()?;

			/*InvokeRule subtype_sign*/
			recog.base.set_state(1173);
			recog.subtype_sign()?;

			/*InvokeRule atomic_type*/
			recog.base.set_state(1174);
			recog.atomic_type()?;

			}
			Ok(())
		})();
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
			recog.base.set_state(1176);
			recog.tff_atomic_formula()?;

			/*InvokeRule identical*/
			recog.base.set_state(1177);
			recog.identical()?;

			/*InvokeRule tff_term*/
			recog.base.set_state(1178);
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
			recog.base.set_state(1180);
			recog.txf_tuple()?;

			/*InvokeRule gentzen_arrow*/
			recog.base.set_state(1181);
			recog.gentzen_arrow()?;

			/*InvokeRule txf_tuple*/
			recog.base.set_state(1182);
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

			recog.base.set_state(1194);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(70,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1184);
					recog.base.match_token(TPTP_T__18,&mut recog.err_handler)?;

					/*InvokeRule ntf_connective_name*/
					recog.base.set_state(1185);
					recog.ntf_connective_name()?;

					recog.base.set_state(1186);
					recog.base.match_token(TPTP_T__19,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1188);
					recog.base.match_token(TPTP_T__18,&mut recog.err_handler)?;

					/*InvokeRule ntf_connective_name*/
					recog.base.set_state(1189);
					recog.ntf_connective_name()?;

					recog.base.set_state(1190);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule nhf_parameter_list*/
					recog.base.set_state(1191);
					recog.nhf_parameter_list()?;

					recog.base.set_state(1192);
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

			recog.base.set_state(1201);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(71,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nhf_parameter*/
					recog.base.set_state(1196);
					recog.nhf_parameter()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nhf_parameter*/
					recog.base.set_state(1197);
					recog.nhf_parameter()?;

					recog.base.set_state(1198);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule nhf_parameter_list*/
					recog.base.set_state(1199);
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

			recog.base.set_state(1205);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Hash
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule ntf_index*/
					recog.base.set_state(1203);
					recog.ntf_index()?;

					}
				}

			TPTP_T__11 |TPTP_T__13 |TPTP_T__16 |TPTP_T__17 |TPTP_T__18 |TPTP_T__43 |
			TPTP_T__44 |TPTP_T__45 |TPTP_T__46 |TPTP_T__47 |TPTP_Single_quoted |TPTP_Back_quoted |
			TPTP_Distinct_object |TPTP_Dollar_word |TPTP_Dollar_dollar_word |TPTP_Lower_word |
			TPTP_Real |TPTP_Rational |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nhf_key_pair*/
					recog.base.set_state(1204);
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
			recog.base.set_state(1207);
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

			recog.base.set_state(1219);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(73,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1209);
					recog.base.match_token(TPTP_T__18,&mut recog.err_handler)?;

					/*InvokeRule ntf_connective_name*/
					recog.base.set_state(1210);
					recog.ntf_connective_name()?;

					recog.base.set_state(1211);
					recog.base.match_token(TPTP_T__19,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1213);
					recog.base.match_token(TPTP_T__18,&mut recog.err_handler)?;

					/*InvokeRule ntf_connective_name*/
					recog.base.set_state(1214);
					recog.ntf_connective_name()?;

					recog.base.set_state(1215);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule nxf_parameter_list*/
					recog.base.set_state(1216);
					recog.nxf_parameter_list()?;

					recog.base.set_state(1217);
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

			recog.base.set_state(1226);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(74,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nxf_parameter*/
					recog.base.set_state(1221);
					recog.nxf_parameter()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nxf_parameter*/
					recog.base.set_state(1222);
					recog.nxf_parameter()?;

					recog.base.set_state(1223);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule nxf_parameter_list*/
					recog.base.set_state(1224);
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

			recog.base.set_state(1230);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Hash
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule ntf_index*/
					recog.base.set_state(1228);
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
					recog.base.set_state(1229);
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
			recog.base.set_state(1232);
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

fn ntf_defined_connective(&self) -> Option<Rc<Ntf_defined_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atomic_system_word(&self) -> Option<Rc<Atomic_system_wordContextAll<'input>>> where Self:Sized{
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

			recog.base.set_state(1236);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule ntf_defined_connective*/
					recog.base.set_state(1234);
					recog.ntf_defined_connective()?;

					}
				}

			TPTP_Dollar_dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule atomic_system_word*/
					recog.base.set_state(1235);
					recog.atomic_system_word()?;

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
//------------------- ntf_defined_connective ----------------
pub type Ntf_defined_connectiveContextAll<'input> = Ntf_defined_connectiveContext<'input>;


pub type Ntf_defined_connectiveContext<'input> = BaseParserRuleContext<'input,Ntf_defined_connectiveContextExt<'input>>;

#[derive(Clone)]
pub struct Ntf_defined_connectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Ntf_defined_connectiveContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Ntf_defined_connectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_ntf_defined_connective(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_ntf_defined_connective(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Ntf_defined_connectiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_ntf_defined_connective(self);
	}
}

impl<'input> CustomRuleContext<'input> for Ntf_defined_connectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ntf_defined_connective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ntf_defined_connective }
}
antlr4rust::tid!{Ntf_defined_connectiveContextExt<'a>}

impl<'input> Ntf_defined_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Ntf_defined_connectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Ntf_defined_connectiveContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Ntf_defined_connectiveContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Ntf_defined_connectiveContextExt<'input>>{

fn atomic_defined_word(&self) -> Option<Rc<Atomic_defined_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Ntf_defined_connectiveContextAttrs<'input> for Ntf_defined_connectiveContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn ntf_defined_connective(&mut self,)
	-> Result<Rc<Ntf_defined_connectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Ntf_defined_connectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 242, RULE_ntf_defined_connective);
        let mut _localctx: Rc<Ntf_defined_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_defined_word*/
			recog.base.set_state(1238);
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
        recog.base.enter_rule(_localctx.clone(), 244, RULE_ntf_index);
        let mut _localctx: Rc<Ntf_indexContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1240);
			recog.base.match_token(TPTP_Hash,&mut recog.err_handler)?;

			/*InvokeRule tff_unitary_term*/
			recog.base.set_state(1241);
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
        recog.base.enter_rule(_localctx.clone(), 246, RULE_ntf_short_connective);
        let mut _localctx: Rc<Ntf_short_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1249);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__21
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1243);
					recog.base.match_token(TPTP_T__21,&mut recog.err_handler)?;

					}
				}

			TPTP_Less_sign
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1244);
					recog.base.match_token(TPTP_Less_sign,&mut recog.err_handler)?;

					recog.base.set_state(1245);
					recog.base.match_token(TPTP_T__22,&mut recog.err_handler)?;

					recog.base.set_state(1246);
					recog.base.match_token(TPTP_Arrow,&mut recog.err_handler)?;

					}
				}

			TPTP_T__23
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1247);
					recog.base.match_token(TPTP_T__23,&mut recog.err_handler)?;

					}
				}

			TPTP_T__24
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(1248);
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
        recog.base.enter_rule(_localctx.clone(), 248, RULE_tcf_formula);
        let mut _localctx: Rc<Tcf_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1253);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(78,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tcf_logic_formula*/
					recog.base.set_state(1251);
					recog.tcf_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule tff_atom_typing*/
					recog.base.set_state(1252);
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
        recog.base.enter_rule(_localctx.clone(), 250, RULE_tcf_logic_formula);
        let mut _localctx: Rc<Tcf_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1257);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__25
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tcf_quantified_formula*/
					recog.base.set_state(1255);
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
					recog.base.set_state(1256);
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
        recog.base.enter_rule(_localctx.clone(), 252, RULE_tcf_quantified_formula);
        let mut _localctx: Rc<Tcf_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1259);
			recog.base.match_token(TPTP_T__25,&mut recog.err_handler)?;

			recog.base.set_state(1260);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(1261);
			recog.tff_variable_list()?;

			recog.base.set_state(1262);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(1263);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule tcf_logic_formula*/
			recog.base.set_state(1264);
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
        recog.base.enter_rule(_localctx.clone(), 254, RULE_fof_formula);
        let mut _localctx: Rc<Fof_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1268);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(80,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_logic_formula*/
					recog.base.set_state(1266);
					recog.fof_logic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_sequent*/
					recog.base.set_state(1267);
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
        recog.base.enter_rule(_localctx.clone(), 256, RULE_fof_logic_formula);
        let mut _localctx: Rc<Fof_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1273);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(81,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_binary_formula*/
					recog.base.set_state(1270);
					recog.fof_binary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_unary_formula*/
					recog.base.set_state(1271);
					recog.fof_unary_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule fof_unitary_formula*/
					recog.base.set_state(1272);
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

fn fof_binary_pair(&self) -> Option<Rc<Fof_binary_pairContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_binary_list(&self) -> Option<Rc<Fof_binary_listContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 258, RULE_fof_binary_formula);
        let mut _localctx: Rc<Fof_binary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1277);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(82,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_binary_pair*/
					recog.base.set_state(1275);
					recog.fof_binary_pair()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_binary_list*/
					recog.base.set_state(1276);
					recog.fof_binary_list()?;

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
//------------------- fof_binary_pair ----------------
pub type Fof_binary_pairContextAll<'input> = Fof_binary_pairContext<'input>;


pub type Fof_binary_pairContext<'input> = BaseParserRuleContext<'input,Fof_binary_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_binary_pairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Fof_binary_pairContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_binary_pairContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_binary_pair(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_binary_pair(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_binary_pairContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_binary_pair(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_binary_pairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_binary_pair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_binary_pair }
}
antlr4rust::tid!{Fof_binary_pairContextExt<'a>}

impl<'input> Fof_binary_pairContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_binary_pairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_binary_pairContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_binary_pairContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_binary_pairContextExt<'input>>{

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

impl<'input> Fof_binary_pairContextAttrs<'input> for Fof_binary_pairContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_binary_pair(&mut self,)
	-> Result<Rc<Fof_binary_pairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_binary_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 260, RULE_fof_binary_pair);
        let mut _localctx: Rc<Fof_binary_pairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1279);
			recog.fof_unit_formula()?;

			/*InvokeRule nonassoc_connective*/
			recog.base.set_state(1280);
			recog.nonassoc_connective()?;

			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1281);
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
//------------------- fof_binary_list ----------------
pub type Fof_binary_listContextAll<'input> = Fof_binary_listContext<'input>;


pub type Fof_binary_listContext<'input> = BaseParserRuleContext<'input,Fof_binary_listContextExt<'input>>;

#[derive(Clone)]
pub struct Fof_binary_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Fof_binary_listContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Fof_binary_listContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fof_binary_list(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fof_binary_list(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Fof_binary_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_fof_binary_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fof_binary_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fof_binary_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fof_binary_list }
}
antlr4rust::tid!{Fof_binary_listContextExt<'a>}

impl<'input> Fof_binary_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Fof_binary_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Fof_binary_listContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Fof_binary_listContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Fof_binary_listContextExt<'input>>{

fn fof_or_formula(&self) -> Option<Rc<Fof_or_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fof_and_formula(&self) -> Option<Rc<Fof_and_formulaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Fof_binary_listContextAttrs<'input> for Fof_binary_listContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fof_binary_list(&mut self,)
	-> Result<Rc<Fof_binary_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Fof_binary_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 262, RULE_fof_binary_list);
        let mut _localctx: Rc<Fof_binary_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1285);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(83,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_or_formula*/
					recog.base.set_state(1283);
					recog.fof_or_formula_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_and_formula*/
					recog.base.set_state(1284);
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
		recog.base.enter_recursion_rule(_localctx.clone(), 264, RULE_fof_or_formula, _p);
	    let mut _localctx: Rc<Fof_or_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 264;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1288);
			recog.fof_unit_formula()?;

			recog.base.set_state(1289);
			recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1290);
			recog.fof_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1297);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(84,&mut recog.base)?;
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
					recog.base.set_state(1292);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1293);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					/*InvokeRule fof_unit_formula*/
					recog.base.set_state(1294);
					recog.fof_unit_formula()?;

					}
					}
				}
				recog.base.set_state(1299);
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
		recog.base.enter_recursion_rule(_localctx.clone(), 266, RULE_fof_and_formula, _p);
	    let mut _localctx: Rc<Fof_and_formulaContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 266;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1301);
			recog.fof_unit_formula()?;

			recog.base.set_state(1302);
			recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1303);
			recog.fof_unit_formula()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1310);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(85,&mut recog.base)?;
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
					recog.base.set_state(1305);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1306);
					recog.base.match_token(TPTP_T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_unit_formula*/
					recog.base.set_state(1307);
					recog.fof_unit_formula()?;

					}
					}
				}
				recog.base.set_state(1312);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(85,&mut recog.base)?;
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
        recog.base.enter_rule(_localctx.clone(), 268, RULE_fof_unary_formula);
        let mut _localctx: Rc<Fof_unary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1317);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__26
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(1313);
					recog.unary_connective()?;

					/*InvokeRule fof_unit_formula*/
					recog.base.set_state(1314);
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
					recog.base.set_state(1316);
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
        recog.base.enter_rule(_localctx.clone(), 270, RULE_fof_infix_unary);
        let mut _localctx: Rc<Fof_infix_unaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_term*/
			recog.base.set_state(1319);
			recog.fof_term()?;

			/*InvokeRule infix_inequality*/
			recog.base.set_state(1320);
			recog.infix_inequality()?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1321);
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
        recog.base.enter_rule(_localctx.clone(), 272, RULE_fof_unit_formula);
        let mut _localctx: Rc<Fof_unit_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1325);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(87,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_unitary_formula*/
					recog.base.set_state(1323);
					recog.fof_unitary_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_unary_formula*/
					recog.base.set_state(1324);
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
        recog.base.enter_rule(_localctx.clone(), 274, RULE_fof_unitary_formula);
        let mut _localctx: Rc<Fof_unitary_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1333);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__25 |TPTP_T__31
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_quantified_formula*/
					recog.base.set_state(1327);
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
					recog.base.set_state(1328);
					recog.fof_atomic_formula()?;

					}
				}

			TPTP_T__11
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1329);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_logic_formula*/
					recog.base.set_state(1330);
					recog.fof_logic_formula()?;

					recog.base.set_state(1331);
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
        recog.base.enter_rule(_localctx.clone(), 276, RULE_fof_quantified_formula);
        let mut _localctx: Rc<Fof_quantified_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_quantifier*/
			recog.base.set_state(1335);
			recog.fof_quantifier()?;

			recog.base.set_state(1336);
			recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

			/*InvokeRule fof_variable_list*/
			recog.base.set_state(1337);
			recog.fof_variable_list()?;

			recog.base.set_state(1338);
			recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

			recog.base.set_state(1339);
			recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

			/*InvokeRule fof_unit_formula*/
			recog.base.set_state(1340);
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
        recog.base.enter_rule(_localctx.clone(), 278, RULE_fof_variable_list);
        let mut _localctx: Rc<Fof_variable_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1347);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(89,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1342);
					recog.variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1343);
					recog.variable()?;

					recog.base.set_state(1344);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule fof_variable_list*/
					recog.base.set_state(1345);
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
        recog.base.enter_rule(_localctx.clone(), 280, RULE_fof_atomic_formula);
        let mut _localctx: Rc<Fof_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1352);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(90,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_plain_atomic_formula*/
					recog.base.set_state(1349);
					recog.fof_plain_atomic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_defined_atomic_formula*/
					recog.base.set_state(1350);
					recog.fof_defined_atomic_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule fof_system_atomic_formula*/
					recog.base.set_state(1351);
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
        recog.base.enter_rule(_localctx.clone(), 282, RULE_fof_plain_atomic_formula);
        let mut _localctx: Rc<Fof_plain_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_plain_term*/
			recog.base.set_state(1354);
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
        recog.base.enter_rule(_localctx.clone(), 284, RULE_fof_defined_atomic_formula);
        let mut _localctx: Rc<Fof_defined_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1358);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(91,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_defined_plain_formula*/
					recog.base.set_state(1356);
					recog.fof_defined_plain_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_defined_infix_formula*/
					recog.base.set_state(1357);
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
        recog.base.enter_rule(_localctx.clone(), 286, RULE_fof_defined_plain_formula);
        let mut _localctx: Rc<Fof_defined_plain_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_defined_plain_term*/
			recog.base.set_state(1360);
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
        recog.base.enter_rule(_localctx.clone(), 288, RULE_fof_defined_infix_formula);
        let mut _localctx: Rc<Fof_defined_infix_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_term*/
			recog.base.set_state(1362);
			recog.fof_term()?;

			/*InvokeRule defined_infix_pred*/
			recog.base.set_state(1363);
			recog.defined_infix_pred()?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1364);
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
        recog.base.enter_rule(_localctx.clone(), 290, RULE_fof_system_atomic_formula);
        let mut _localctx: Rc<Fof_system_atomic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_system_term*/
			recog.base.set_state(1366);
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
        recog.base.enter_rule(_localctx.clone(), 292, RULE_fof_plain_term);
        let mut _localctx: Rc<Fof_plain_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1374);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(92,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule constant*/
					recog.base.set_state(1368);
					recog.constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule functor*/
					recog.base.set_state(1369);
					recog.functor()?;

					recog.base.set_state(1370);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1371);
					recog.fof_arguments()?;

					recog.base.set_state(1372);
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
        recog.base.enter_rule(_localctx.clone(), 294, RULE_fof_defined_term);
        let mut _localctx: Rc<Fof_defined_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1378);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Distinct_object |TPTP_Real |TPTP_Rational |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(1376);
					recog.defined_term()?;

					}
				}

			TPTP_Dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_defined_atomic_term*/
					recog.base.set_state(1377);
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
        recog.base.enter_rule(_localctx.clone(), 296, RULE_fof_defined_atomic_term);
        let mut _localctx: Rc<Fof_defined_atomic_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_defined_plain_term*/
			recog.base.set_state(1380);
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
        recog.base.enter_rule(_localctx.clone(), 298, RULE_fof_defined_plain_term);
        let mut _localctx: Rc<Fof_defined_plain_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1388);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(94,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(1382);
					recog.defined_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_functor*/
					recog.base.set_state(1383);
					recog.defined_functor()?;

					recog.base.set_state(1384);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1385);
					recog.fof_arguments()?;

					recog.base.set_state(1386);
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
        recog.base.enter_rule(_localctx.clone(), 300, RULE_fof_system_term);
        let mut _localctx: Rc<Fof_system_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1396);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(95,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule system_constant*/
					recog.base.set_state(1390);
					recog.system_constant()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule system_functor*/
					recog.base.set_state(1391);
					recog.system_functor()?;

					recog.base.set_state(1392);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1393);
					recog.fof_arguments()?;

					recog.base.set_state(1394);
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
        recog.base.enter_rule(_localctx.clone(), 302, RULE_fof_arguments);
        let mut _localctx: Rc<Fof_argumentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1403);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(96,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_term*/
					recog.base.set_state(1398);
					recog.fof_term()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_term*/
					recog.base.set_state(1399);
					recog.fof_term()?;

					recog.base.set_state(1400);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1401);
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
        recog.base.enter_rule(_localctx.clone(), 304, RULE_fof_term);
        let mut _localctx: Rc<Fof_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1407);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |TPTP_Dollar_word |
			TPTP_Dollar_dollar_word |TPTP_Lower_word |TPTP_Real |TPTP_Rational |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_function_term*/
					recog.base.set_state(1405);
					recog.fof_function_term()?;

					}
				}

			TPTP_Upper_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1406);
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
        recog.base.enter_rule(_localctx.clone(), 306, RULE_fof_function_term);
        let mut _localctx: Rc<Fof_function_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1412);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_plain_term*/
					recog.base.set_state(1409);
					recog.fof_plain_term()?;

					}
				}

			TPTP_Distinct_object |TPTP_Dollar_word |TPTP_Real |TPTP_Rational |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule fof_defined_term*/
					recog.base.set_state(1410);
					recog.fof_defined_term()?;

					}
				}

			TPTP_Dollar_dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule fof_system_term*/
					recog.base.set_state(1411);
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
        recog.base.enter_rule(_localctx.clone(), 308, RULE_fof_sequent);
        let mut _localctx: Rc<Fof_sequentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1422);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__13 |TPTP_T__17
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_formula_tuple*/
					recog.base.set_state(1414);
					recog.fof_formula_tuple()?;

					/*InvokeRule gentzen_arrow*/
					recog.base.set_state(1415);
					recog.gentzen_arrow()?;

					/*InvokeRule fof_formula_tuple*/
					recog.base.set_state(1416);
					recog.fof_formula_tuple()?;

					}
				}

			TPTP_T__11
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1418);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_sequent*/
					recog.base.set_state(1419);
					recog.fof_sequent()?;

					recog.base.set_state(1420);
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
        recog.base.enter_rule(_localctx.clone(), 310, RULE_fof_formula_tuple);
        let mut _localctx: Rc<Fof_formula_tupleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1429);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1424);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1425);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule fof_formula_tuple_list*/
					recog.base.set_state(1426);
					recog.fof_formula_tuple_list()?;

					recog.base.set_state(1427);
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
        recog.base.enter_rule(_localctx.clone(), 312, RULE_fof_formula_tuple_list);
        let mut _localctx: Rc<Fof_formula_tuple_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule fof_logic_formula*/
			recog.base.set_state(1431);
			recog.fof_logic_formula()?;

			recog.base.set_state(1435);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_fof_logic_formula*/
				recog.base.set_state(1432);
				recog.comma_fof_logic_formula()?;

				}
				}
				recog.base.set_state(1437);
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
        recog.base.enter_rule(_localctx.clone(), 314, RULE_comma_fof_logic_formula);
        let mut _localctx: Rc<Comma_fof_logic_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1438);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_logic_formula*/
			recog.base.set_state(1439);
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
        recog.base.enter_rule(_localctx.clone(), 316, RULE_cnf_formula);
        let mut _localctx: Rc<Cnf_formulaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1446);
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
					recog.base.set_state(1441);
					recog.cnf_disjunction_rec(0)?;

					}
				}

			TPTP_T__11
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1442);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule cnf_formula*/
					recog.base.set_state(1443);
					recog.cnf_formula()?;

					recog.base.set_state(1444);
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
		recog.base.enter_recursion_rule(_localctx.clone(), 318, RULE_cnf_disjunction, _p);
	    let mut _localctx: Rc<Cnf_disjunctionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 318;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			/*InvokeRule cnf_literal*/
			recog.base.set_state(1449);
			recog.cnf_literal()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1456);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(103,&mut recog.base)?;
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
					recog.base.set_state(1451);
					if !({let _localctx = Some(_localctx.clone());
					recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1452);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					/*InvokeRule cnf_literal*/
					recog.base.set_state(1453);
					recog.cnf_literal()?;

					}
					}
				}
				recog.base.set_state(1458);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(103,&mut recog.base)?;
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
        recog.base.enter_rule(_localctx.clone(), 320, RULE_cnf_literal);
        let mut _localctx: Rc<Cnf_literalContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1468);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(104,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1459);
					recog.fof_atomic_formula()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1460);
					recog.base.match_token(TPTP_T__26,&mut recog.err_handler)?;

					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1461);
					recog.fof_atomic_formula()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1462);
					recog.base.match_token(TPTP_T__26,&mut recog.err_handler)?;

					recog.base.set_state(1463);
					recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_atomic_formula*/
					recog.base.set_state(1464);
					recog.fof_atomic_formula()?;

					recog.base.set_state(1465);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule fof_infix_unary*/
					recog.base.set_state(1467);
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
        recog.base.enter_rule(_localctx.clone(), 322, RULE_thf_quantifier);
        let mut _localctx: Rc<Thf_quantifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1473);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(105,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule tff_quantifier*/
					recog.base.set_state(1470);
					recog.tff_quantifier()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule th0_quantifier*/
					recog.base.set_state(1471);
					recog.th0_quantifier()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule type_quantifier*/
					recog.base.set_state(1472);
					recog.type_quantifier()?;

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
        recog.base.enter_rule(_localctx.clone(), 324, RULE_thf_unary_connective);
        let mut _localctx: Rc<Thf_unary_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1477);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__26
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(1475);
					recog.unary_connective()?;

					}
				}

			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_Less_sign
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule ntf_short_connective*/
					recog.base.set_state(1476);
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
        recog.base.enter_rule(_localctx.clone(), 326, RULE_th0_quantifier);
        let mut _localctx: Rc<Th0_quantifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1479);
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
        recog.base.enter_rule(_localctx.clone(), 328, RULE_type_quantifier);
        let mut _localctx: Rc<Type_quantifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1488);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__30
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1481);
					recog.base.match_token(TPTP_T__30,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13 |TPTP_T__31
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1485);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==TPTP_T__31 {
						{
						{
						recog.base.set_state(1482);
						recog.base.match_token(TPTP_T__31,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(1487);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
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
        recog.base.enter_rule(_localctx.clone(), 330, RULE_subtype_sign);
        let mut _localctx: Rc<Subtype_signContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1490);
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
        recog.base.enter_rule(_localctx.clone(), 332, RULE_tff_unary_connective);
        let mut _localctx: Rc<Tff_unary_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1494);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__26
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(1492);
					recog.unary_connective()?;

					}
				}

			TPTP_T__21 |TPTP_T__23 |TPTP_T__24 |TPTP_Less_sign
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule ntf_short_connective*/
					recog.base.set_state(1493);
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
        recog.base.enter_rule(_localctx.clone(), 334, RULE_tff_quantifier);
        let mut _localctx: Rc<Tff_quantifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1498);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__25 |TPTP_T__31
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fof_quantifier*/
					recog.base.set_state(1496);
					recog.fof_quantifier()?;

					}
				}

			TPTP_Hash
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1497);
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
        recog.base.enter_rule(_localctx.clone(), 336, RULE_fof_quantifier);
        let mut _localctx: Rc<Fof_quantifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1500);
			_la = recog.base.input.la(1);
			if { !(_la==TPTP_T__25 || _la==TPTP_T__31) } {
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
        recog.base.enter_rule(_localctx.clone(), 338, RULE_nonassoc_connective);
        let mut _localctx: Rc<Nonassoc_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1509);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__33
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1502);
					recog.base.match_token(TPTP_T__33,&mut recog.err_handler)?;

					}
				}

			TPTP_T__34
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1503);
					recog.base.match_token(TPTP_T__34,&mut recog.err_handler)?;

					}
				}

			TPTP_T__35
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1504);
					recog.base.match_token(TPTP_T__35,&mut recog.err_handler)?;

					}
				}

			TPTP_T__36
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(1505);
					recog.base.match_token(TPTP_T__36,&mut recog.err_handler)?;

					}
				}

			TPTP_T__26
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1506);
					recog.base.match_token(TPTP_T__26,&mut recog.err_handler)?;

					recog.base.set_state(1507);
					recog.base.match_token(TPTP_Vline,&mut recog.err_handler)?;

					}
				}

			TPTP_T__37
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(1508);
					recog.base.match_token(TPTP_T__37,&mut recog.err_handler)?;

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
        recog.base.enter_rule(_localctx.clone(), 340, RULE_assoc_connective);
        let mut _localctx: Rc<Assoc_connectiveContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1511);
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
        recog.base.enter_rule(_localctx.clone(), 342, RULE_unary_connective);
        let mut _localctx: Rc<Unary_connectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1513);
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
        recog.base.enter_rule(_localctx.clone(), 344, RULE_gentzen_arrow);
        let mut _localctx: Rc<Gentzen_arrowContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1515);
			recog.base.match_token(TPTP_T__38,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
        recog.base.enter_rule(_localctx.clone(), 346, RULE_assignment);
        let mut _localctx: Rc<AssignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1517);
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
        recog.base.enter_rule(_localctx.clone(), 348, RULE_identical);
        let mut _localctx: Rc<IdenticalContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1519);
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
//------------------- typeable_atom ----------------
pub type Typeable_atomContextAll<'input> = Typeable_atomContext<'input>;


pub type Typeable_atomContext<'input> = BaseParserRuleContext<'input,Typeable_atomContextExt<'input>>;

#[derive(Clone)]
pub struct Typeable_atomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Typeable_atomContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Typeable_atomContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_typeable_atom(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_typeable_atom(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Typeable_atomContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_typeable_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for Typeable_atomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeable_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeable_atom }
}
antlr4rust::tid!{Typeable_atomContextExt<'a>}

impl<'input> Typeable_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Typeable_atomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Typeable_atomContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Typeable_atomContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Typeable_atomContextExt<'input>>{

fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Distinct_object
/// Returns `None` if there is no child corresponding to token Distinct_object
fn Distinct_object(&self) -> Option<Rc<TerminalNode<'input,TPTPParserContextType>>> where Self:Sized{
	self.get_token(TPTP_Distinct_object, 0)
}

}

impl<'input> Typeable_atomContextAttrs<'input> for Typeable_atomContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn typeable_atom(&mut self,)
	-> Result<Rc<Typeable_atomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Typeable_atomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 350, RULE_typeable_atom);
        let mut _localctx: Rc<Typeable_atomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1523);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule constant*/
					recog.base.set_state(1521);
					recog.constant()?;

					}
				}

			TPTP_Distinct_object
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1522);
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
//------------------- atomic_type ----------------
pub type Atomic_typeContextAll<'input> = Atomic_typeContext<'input>;


pub type Atomic_typeContext<'input> = BaseParserRuleContext<'input,Atomic_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Atomic_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for Atomic_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for Atomic_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atomic_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atomic_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for Atomic_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_atomic_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for Atomic_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomic_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomic_type }
}
antlr4rust::tid!{Atomic_typeContextExt<'a>}

impl<'input> Atomic_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Atomic_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Atomic_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Atomic_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<Atomic_typeContextExt<'input>>{

fn typeable_atom(&self) -> Option<Rc<Typeable_atomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn defined_constant(&self) -> Option<Rc<Defined_constantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn system_type(&self) -> Option<Rc<System_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Atomic_typeContextAttrs<'input> for Atomic_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atomic_type(&mut self,)
	-> Result<Rc<Atomic_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Atomic_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 352, RULE_atomic_type);
        let mut _localctx: Rc<Atomic_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1528);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Distinct_object |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule typeable_atom*/
					recog.base.set_state(1525);
					recog.typeable_atom()?;

					}
				}

			TPTP_Dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule defined_constant*/
					recog.base.set_state(1526);
					recog.defined_constant()?;

					}
				}

			TPTP_Dollar_dollar_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule system_type*/
					recog.base.set_state(1527);
					recog.system_type()?;

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
        recog.base.enter_rule(_localctx.clone(), 354, RULE_type_constant);
        let mut _localctx: Rc<Type_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule type_functor*/
			recog.base.set_state(1530);
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
        recog.base.enter_rule(_localctx.clone(), 356, RULE_type_functor);
        let mut _localctx: Rc<Type_functorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1532);
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
        recog.base.enter_rule(_localctx.clone(), 358, RULE_defined_type);
        let mut _localctx: Rc<Defined_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_defined_word*/
			recog.base.set_state(1534);
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
//------------------- system_type ----------------
pub type System_typeContextAll<'input> = System_typeContext<'input>;


pub type System_typeContext<'input> = BaseParserRuleContext<'input,System_typeContextExt<'input>>;

#[derive(Clone)]
pub struct System_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TPTPParserContext<'input> for System_typeContext<'input>{}

impl<'input,'a> Listenable<dyn TPTPListener<'input> + 'a> for System_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_system_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn TPTPListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_system_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn TPTPVisitor<'input> + 'a> for System_typeContext<'input>{
	fn accept(&self,visitor: &mut (dyn TPTPVisitor<'input> + 'a)) {
		visitor.visit_system_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for System_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TPTPParserContextType;
	fn get_rule_index(&self) -> usize { RULE_system_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_system_type }
}
antlr4rust::tid!{System_typeContextExt<'a>}

impl<'input> System_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TPTPParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<System_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,System_typeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait System_typeContextAttrs<'input>: TPTPParserContext<'input> + BorrowMut<System_typeContextExt<'input>>{

fn atomic_system_word(&self) -> Option<Rc<Atomic_system_wordContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> System_typeContextAttrs<'input> for System_typeContext<'input>{}

impl<'input, I> TPTPParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn system_type(&mut self,)
	-> Result<Rc<System_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = System_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 360, RULE_system_type);
        let mut _localctx: Rc<System_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_system_word*/
			recog.base.set_state(1536);
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
        recog.base.enter_rule(_localctx.clone(), 362, RULE_defined_infix_pred);
        let mut _localctx: Rc<Defined_infix_predContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule infix_equality*/
			recog.base.set_state(1538);
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
        recog.base.enter_rule(_localctx.clone(), 364, RULE_infix_equality);
        let mut _localctx: Rc<Infix_equalityContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1540);
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
        recog.base.enter_rule(_localctx.clone(), 366, RULE_infix_inequality);
        let mut _localctx: Rc<Infix_inequalityContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1542);
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
        recog.base.enter_rule(_localctx.clone(), 368, RULE_constant);
        let mut _localctx: Rc<ConstantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule functor*/
			recog.base.set_state(1544);
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
        recog.base.enter_rule(_localctx.clone(), 370, RULE_functor);
        let mut _localctx: Rc<FunctorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1546);
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
        recog.base.enter_rule(_localctx.clone(), 372, RULE_defined_constant);
        let mut _localctx: Rc<Defined_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule defined_functor*/
			recog.base.set_state(1548);
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
        recog.base.enter_rule(_localctx.clone(), 374, RULE_defined_functor);
        let mut _localctx: Rc<Defined_functorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_defined_word*/
			recog.base.set_state(1550);
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
        recog.base.enter_rule(_localctx.clone(), 376, RULE_system_constant);
        let mut _localctx: Rc<System_constantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule system_functor*/
			recog.base.set_state(1552);
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
        recog.base.enter_rule(_localctx.clone(), 378, RULE_system_functor);
        let mut _localctx: Rc<System_functorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_system_word*/
			recog.base.set_state(1554);
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
        recog.base.enter_rule(_localctx.clone(), 380, RULE_th1_defined_term);
        let mut _localctx: Rc<Th1_defined_termContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1556);
			_la = recog.base.input.la(1);
			if { !(((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 31) != 0)) } {
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
        recog.base.enter_rule(_localctx.clone(), 382, RULE_defined_term);
        let mut _localctx: Rc<Defined_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1560);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Real |TPTP_Rational |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule number*/
					recog.base.set_state(1558);
					recog.number()?;

					}
				}

			TPTP_Distinct_object
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1559);
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
        recog.base.enter_rule(_localctx.clone(), 384, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1562);
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
        recog.base.enter_rule(_localctx.clone(), 386, RULE_source);
        let mut _localctx: Rc<SourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1571);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__48 |TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule dag_source*/
					recog.base.set_state(1564);
					recog.dag_source()?;

					}
				}

			TPTP_T__49
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule internal_source*/
					recog.base.set_state(1565);
					recog.internal_source()?;

					}
				}

			TPTP_T__50
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule external_source*/
					recog.base.set_state(1566);
					recog.external_source()?;

					}
				}

			TPTP_T__13
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(1567);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule sources*/
					recog.base.set_state(1568);
					recog.sources()?;

					recog.base.set_state(1569);
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
        recog.base.enter_rule(_localctx.clone(), 388, RULE_sources);
        let mut _localctx: Rc<SourcesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1578);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(116,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule source*/
					recog.base.set_state(1573);
					recog.source()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule source*/
					recog.base.set_state(1574);
					recog.source()?;

					recog.base.set_state(1575);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule sources*/
					recog.base.set_state(1576);
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
        recog.base.enter_rule(_localctx.clone(), 390, RULE_dag_source);
        let mut _localctx: Rc<Dag_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1582);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word |TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule name*/
					recog.base.set_state(1580);
					recog.name()?;

					}
				}

			TPTP_T__48
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule inference_record*/
					recog.base.set_state(1581);
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
        recog.base.enter_rule(_localctx.clone(), 392, RULE_inference_record);
        let mut _localctx: Rc<Inference_recordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1584);
			recog.base.match_token(TPTP_T__48,&mut recog.err_handler)?;

			/*InvokeRule inference_rule*/
			recog.base.set_state(1585);
			recog.inference_rule()?;

			recog.base.set_state(1586);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule useful_info*/
			recog.base.set_state(1587);
			recog.useful_info()?;

			recog.base.set_state(1588);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule parents*/
			recog.base.set_state(1589);
			recog.parents()?;

			recog.base.set_state(1590);
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
        recog.base.enter_rule(_localctx.clone(), 394, RULE_inference_rule);
        let mut _localctx: Rc<Inference_ruleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1592);
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
        recog.base.enter_rule(_localctx.clone(), 396, RULE_internal_source);
        let mut _localctx: Rc<Internal_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1594);
			recog.base.match_token(TPTP_T__49,&mut recog.err_handler)?;

			/*InvokeRule intro_type*/
			recog.base.set_state(1595);
			recog.intro_type()?;

			recog.base.set_state(1596);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule useful_info*/
			recog.base.set_state(1597);
			recog.useful_info()?;

			recog.base.set_state(1598);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule parents*/
			recog.base.set_state(1599);
			recog.parents()?;

			recog.base.set_state(1600);
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
        recog.base.enter_rule(_localctx.clone(), 398, RULE_intro_type);
        let mut _localctx: Rc<Intro_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1602);
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
        recog.base.enter_rule(_localctx.clone(), 400, RULE_external_source);
        let mut _localctx: Rc<External_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule file_source*/
			recog.base.set_state(1604);
			recog.file_source()?;

			}
			Ok(())
		})();
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
        recog.base.enter_rule(_localctx.clone(), 402, RULE_file_source);
        let mut _localctx: Rc<File_sourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1606);
			recog.base.match_token(TPTP_T__50,&mut recog.err_handler)?;

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
        recog.base.enter_rule(_localctx.clone(), 404, RULE_file_info);
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
        recog.base.enter_rule(_localctx.clone(), 406, RULE_parents);
        let mut _localctx: Rc<ParentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1621);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1616);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1617);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule parent_list*/
					recog.base.set_state(1618);
					recog.parent_list()?;

					recog.base.set_state(1619);
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
        recog.base.enter_rule(_localctx.clone(), 408, RULE_parent_list);
        let mut _localctx: Rc<Parent_listContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule parent_info*/
			recog.base.set_state(1623);
			recog.parent_info()?;

			recog.base.set_state(1627);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_parent_info*/
				recog.base.set_state(1624);
				recog.comma_parent_info()?;

				}
				}
				recog.base.set_state(1629);
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
        recog.base.enter_rule(_localctx.clone(), 410, RULE_comma_parent_info);
        let mut _localctx: Rc<Comma_parent_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1630);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule parent_info*/
			recog.base.set_state(1631);
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
        recog.base.enter_rule(_localctx.clone(), 412, RULE_parent_info);
        let mut _localctx: Rc<Parent_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule source*/
			recog.base.set_state(1633);
			recog.source()?;

			/*InvokeRule parent_details*/
			recog.base.set_state(1634);
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
        recog.base.enter_rule(_localctx.clone(), 414, RULE_parent_details);
        let mut _localctx: Rc<Parent_detailsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1639);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__15
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1636);
					recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

					/*InvokeRule general_term*/
					recog.base.set_state(1637);
					recog.general_term()?;

					}
				}

			TPTP_T__1 |TPTP_T__14
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nothing*/
					recog.base.set_state(1638);
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
        recog.base.enter_rule(_localctx.clone(), 416, RULE_optional_info);
        let mut _localctx: Rc<Optional_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1644);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__1
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1641);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule useful_info*/
					recog.base.set_state(1642);
					recog.useful_info()?;

					}
				}

			TPTP_T__2
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule nothing*/
					recog.base.set_state(1643);
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
        recog.base.enter_rule(_localctx.clone(), 418, RULE_useful_info);
        let mut _localctx: Rc<Useful_infoContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule general_list*/
			recog.base.set_state(1646);
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
        recog.base.enter_rule(_localctx.clone(), 420, RULE_include);
        let mut _localctx: Rc<IncludeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1648);
			recog.base.match_token(TPTP_T__51,&mut recog.err_handler)?;

			/*InvokeRule file_name*/
			recog.base.set_state(1649);
			recog.file_name()?;

			/*InvokeRule include_optionals*/
			recog.base.set_state(1650);
			recog.include_optionals()?;

			recog.base.set_state(1651);
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
        recog.base.enter_rule(_localctx.clone(), 422, RULE_include_optionals);
        let mut _localctx: Rc<Include_optionalsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1661);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(123,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nothing*/
					recog.base.set_state(1653);
					recog.nothing()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1654);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule formula_selection*/
					recog.base.set_state(1655);
					recog.formula_selection()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1656);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule formula_selection*/
					recog.base.set_state(1657);
					recog.formula_selection()?;

					recog.base.set_state(1658);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule space_name*/
					recog.base.set_state(1659);
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
        recog.base.enter_rule(_localctx.clone(), 424, RULE_formula_selection);
        let mut _localctx: Rc<Formula_selectionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1668);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__13
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1663);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule name_list*/
					recog.base.set_state(1664);
					recog.name_list()?;

					recog.base.set_state(1665);
					recog.base.match_token(TPTP_T__14,&mut recog.err_handler)?;

					}
				}

			TPTP_Star
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1667);
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
        recog.base.enter_rule(_localctx.clone(), 426, RULE_name_list);
        let mut _localctx: Rc<Name_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1675);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(125,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule name*/
					recog.base.set_state(1670);
					recog.name()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule name*/
					recog.base.set_state(1671);
					recog.name()?;

					recog.base.set_state(1672);
					recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

					/*InvokeRule name_list*/
					recog.base.set_state(1673);
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
        recog.base.enter_rule(_localctx.clone(), 428, RULE_space_name);
        let mut _localctx: Rc<Space_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule name*/
			recog.base.set_state(1677);
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
        recog.base.enter_rule(_localctx.clone(), 430, RULE_general_term);
        let mut _localctx: Rc<General_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1685);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(126,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule general_data*/
					recog.base.set_state(1679);
					recog.general_data()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule general_data*/
					recog.base.set_state(1680);
					recog.general_data()?;

					recog.base.set_state(1681);
					recog.base.match_token(TPTP_T__15,&mut recog.err_handler)?;

					/*InvokeRule general_term*/
					recog.base.set_state(1682);
					recog.general_term()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule general_list*/
					recog.base.set_state(1684);
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
        recog.base.enter_rule(_localctx.clone(), 432, RULE_general_data);
        let mut _localctx: Rc<General_dataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1693);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(127,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule atomic_word*/
					recog.base.set_state(1687);
					recog.atomic_word()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule general_function*/
					recog.base.set_state(1688);
					recog.general_function()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1689);
					recog.variable()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule number*/
					recog.base.set_state(1690);
					recog.number()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1691);
					recog.base.match_token(TPTP_Distinct_object,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule formula_data*/
					recog.base.set_state(1692);
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
        recog.base.enter_rule(_localctx.clone(), 434, RULE_general_function);
        let mut _localctx: Rc<General_functionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1695);
			recog.atomic_word()?;

			recog.base.set_state(1696);
			recog.base.match_token(TPTP_T__11,&mut recog.err_handler)?;

			/*InvokeRule general_terms*/
			recog.base.set_state(1697);
			recog.general_terms()?;

			recog.base.set_state(1698);
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
        recog.base.enter_rule(_localctx.clone(), 436, RULE_formula_data);
        let mut _localctx: Rc<Formula_dataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1720);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__52
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1700);
					recog.base.match_token(TPTP_T__52,&mut recog.err_handler)?;

					/*InvokeRule thf_formula*/
					recog.base.set_state(1701);
					recog.thf_formula()?;

					recog.base.set_state(1702);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__53
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1704);
					recog.base.match_token(TPTP_T__53,&mut recog.err_handler)?;

					/*InvokeRule tff_formula*/
					recog.base.set_state(1705);
					recog.tff_formula()?;

					recog.base.set_state(1706);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__54
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1708);
					recog.base.match_token(TPTP_T__54,&mut recog.err_handler)?;

					/*InvokeRule fof_formula*/
					recog.base.set_state(1709);
					recog.fof_formula()?;

					recog.base.set_state(1710);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__55
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(1712);
					recog.base.match_token(TPTP_T__55,&mut recog.err_handler)?;

					/*InvokeRule cnf_formula*/
					recog.base.set_state(1713);
					recog.cnf_formula()?;

					recog.base.set_state(1714);
					recog.base.match_token(TPTP_T__12,&mut recog.err_handler)?;

					}
				}

			TPTP_T__56
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(1716);
					recog.base.match_token(TPTP_T__56,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(1717);
					recog.fof_term()?;

					recog.base.set_state(1718);
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
        recog.base.enter_rule(_localctx.clone(), 438, RULE_general_list);
        let mut _localctx: Rc<General_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1727);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_T__17
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1722);
					recog.base.match_token(TPTP_T__17,&mut recog.err_handler)?;

					}
				}

			TPTP_T__13
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1723);
					recog.base.match_token(TPTP_T__13,&mut recog.err_handler)?;

					/*InvokeRule general_terms*/
					recog.base.set_state(1724);
					recog.general_terms()?;

					recog.base.set_state(1725);
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
        recog.base.enter_rule(_localctx.clone(), 440, RULE_general_terms);
        let mut _localctx: Rc<General_termsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule general_term*/
			recog.base.set_state(1729);
			recog.general_term()?;

			recog.base.set_state(1733);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TPTP_T__1 {
				{
				{
				/*InvokeRule comma_general_term*/
				recog.base.set_state(1730);
				recog.comma_general_term()?;

				}
				}
				recog.base.set_state(1735);
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
        recog.base.enter_rule(_localctx.clone(), 442, RULE_comma_general_term);
        let mut _localctx: Rc<Comma_general_termContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1736);
			recog.base.match_token(TPTP_T__1,&mut recog.err_handler)?;

			/*InvokeRule general_term*/
			recog.base.set_state(1737);
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
        recog.base.enter_rule(_localctx.clone(), 444, RULE_name);
        let mut _localctx: Rc<NameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1741);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			TPTP_Single_quoted |TPTP_Back_quoted |TPTP_Lower_word
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule atomic_word*/
					recog.base.set_state(1739);
					recog.atomic_word()?;

					}
				}

			TPTP_Integer
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1740);
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
        recog.base.enter_rule(_localctx.clone(), 446, RULE_atomic_word);
        let mut _localctx: Rc<Atomic_wordContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1743);
			_la = recog.base.input.la(1);
			if { !(((((_la - 61)) & !0x3f) == 0 && ((1usize << (_la - 61)) & 67) != 0)) } {
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
        recog.base.enter_rule(_localctx.clone(), 448, RULE_atomic_defined_word);
        let mut _localctx: Rc<Atomic_defined_wordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1745);
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
        recog.base.enter_rule(_localctx.clone(), 450, RULE_atomic_system_word);
        let mut _localctx: Rc<Atomic_system_wordContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1747);
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
        recog.base.enter_rule(_localctx.clone(), 452, RULE_number);
        let mut _localctx: Rc<NumberContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1749);
			_la = recog.base.input.la(1);
			if { !(((((_la - 68)) & !0x3f) == 0 && ((1usize << (_la - 68)) & 1153) != 0)) } {
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
        recog.base.enter_rule(_localctx.clone(), 454, RULE_file_name);
        let mut _localctx: Rc<File_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule atomic_word*/
			recog.base.set_state(1751);
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
        recog.base.enter_rule(_localctx.clone(), 456, RULE_nothing);
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
		4, 1, 103, 1756, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4,
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
		7, 226, 2, 227, 7, 227, 2, 228, 7, 228, 1, 0, 5, 0, 460, 8, 0, 10, 0,
		12, 0, 463, 9, 0, 1, 0, 1, 0, 1, 1, 1, 1, 3, 1, 469, 8, 1, 1, 2, 1, 2,
		1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 477, 8, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3,
		1, 3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5,
		1, 5, 1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6,
		1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 8, 1, 8, 1, 8,
		1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9,
		1, 9, 1, 9, 1, 9, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 3, 10, 540, 8, 10,
		1, 11, 1, 11, 1, 11, 1, 11, 3, 11, 546, 8, 11, 1, 12, 1, 12, 1, 12, 3,
		12, 551, 8, 12, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 3, 13, 559,
		8, 13, 1, 14, 1, 14, 1, 14, 3, 14, 564, 8, 14, 1, 15, 1, 15, 1, 15, 1,
		15, 1, 16, 1, 16, 1, 16, 3, 16, 573, 8, 16, 1, 17, 1, 17, 1, 17, 1, 17,
		1, 17, 1, 17, 1, 17, 1, 17, 5, 17, 583, 8, 17, 10, 17, 12, 17, 586, 9,
		17, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 5, 18, 596,
		8, 18, 10, 18, 12, 18, 599, 9, 18, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19,
		1, 19, 1, 19, 1, 19, 5, 19, 609, 8, 19, 10, 19, 12, 19, 612, 9, 19, 1,
		20, 1, 20, 1, 20, 3, 20, 617, 8, 20, 1, 21, 1, 21, 3, 21, 621, 8, 21,
		1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 3, 22, 630, 8, 22, 1,
		23, 1, 23, 1, 23, 1, 24, 1, 24, 1, 24, 1, 24, 1, 24, 1, 24, 1, 25, 1,
		25, 1, 25, 1, 25, 1, 25, 3, 25, 646, 8, 25, 1, 26, 1, 26, 1, 26, 1, 26,
		1, 27, 1, 27, 3, 27, 654, 8, 27, 1, 28, 1, 28, 1, 28, 1, 29, 1, 29, 1,
		29, 1, 29, 1, 30, 1, 30, 1, 30, 1, 30, 3, 30, 667, 8, 30, 1, 31, 1, 31,
		3, 31, 671, 8, 31, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1,
		32, 3, 32, 681, 8, 32, 1, 33, 1, 33, 3, 33, 685, 8, 33, 1, 34, 1, 34,
		1, 34, 1, 34, 1, 35, 1, 35, 1, 36, 1, 36, 1, 36, 1, 36, 1, 36, 1, 36,
		1, 36, 1, 36, 1, 37, 1, 37, 1, 37, 1, 37, 1, 37, 3, 37, 706, 8, 37, 1,
		38, 1, 38, 1, 38, 1, 38, 1, 38, 3, 38, 713, 8, 38, 1, 39, 1, 39, 1, 39,
		1, 39, 1, 39, 3, 39, 720, 8, 39, 1, 40, 1, 40, 1, 40, 1, 40, 1, 41, 1,
		41, 1, 41, 1, 41, 1, 41, 3, 41, 731, 8, 41, 1, 42, 1, 42, 1, 42, 1, 42,
		1, 42, 1, 42, 3, 42, 739, 8, 42, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 3,
		43, 746, 8, 43, 1, 44, 1, 44, 1, 44, 1, 44, 1, 44, 3, 44, 753, 8, 44,
		1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45,
		1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 3, 45, 770, 8, 45, 1, 46, 1, 46, 1,
		47, 1, 47, 5, 47, 776, 8, 47, 10, 47, 12, 47, 779, 9, 47, 1, 48, 1, 48,
		1, 48, 1, 49, 1, 49, 1, 49, 1, 49, 1, 49, 1, 49, 1, 49, 1, 49, 3, 49,
		792, 8, 49, 1, 50, 1, 50, 1, 50, 3, 50, 797, 8, 50, 1, 51, 1, 51, 1, 52,
		1, 52, 1, 53, 1, 53, 1, 53, 3, 53, 806, 8, 53, 1, 54, 1, 54, 1, 54, 1,
		54, 1, 54, 1, 54, 1, 54, 1, 54, 3, 54, 816, 8, 54, 1, 55, 1, 55, 1, 55,
		1, 55, 1, 55, 1, 55, 1, 55, 1, 55, 5, 55, 826, 8, 55, 10, 55, 12, 55,
		829, 9, 55, 1, 56, 1, 56, 1, 56, 1, 56, 1, 56, 1, 56, 1, 56, 1, 56, 5,
		56, 839, 8, 56, 10, 56, 12, 56, 842, 9, 56, 1, 57, 1, 57, 1, 57, 1, 57,
		1, 58, 1, 58, 1, 58, 1, 58, 1, 59, 1, 59, 1, 59, 1, 59, 1, 60, 1, 60,
		1, 60, 3, 60, 859, 8, 60, 1, 61, 1, 61, 1, 61, 1, 61, 1, 61, 1, 61, 3,
		61, 867, 8, 61, 1, 62, 1, 62, 3, 62, 871, 8, 62, 1, 63, 1, 63, 1, 63,
		1, 63, 1, 64, 1, 64, 3, 64, 879, 8, 64, 1, 65, 1, 65, 1, 65, 1, 65, 1,
		65, 1, 65, 1, 65, 1, 65, 5, 65, 889, 8, 65, 10, 65, 12, 65, 892, 9, 65,
		1, 66, 1, 66, 1, 66, 1, 66, 1, 66, 1, 66, 1, 66, 1, 66, 5, 66, 902, 8,
		66, 10, 66, 12, 66, 905, 9, 66, 1, 67, 1, 67, 1, 67, 3, 67, 910, 8, 67,
		1, 68, 1, 68, 3, 68, 914, 8, 68, 1, 69, 1, 69, 1, 69, 1, 69, 1, 69, 1,
		69, 1, 69, 3, 69, 923, 8, 69, 1, 70, 1, 70, 1, 71, 1, 71, 1, 71, 1, 71,
		1, 71, 1, 71, 1, 71, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 3, 72, 939, 8,
		72, 1, 73, 1, 73, 3, 73, 943, 8, 73, 1, 74, 1, 74, 1, 74, 1, 74, 1, 75,
		1, 75, 3, 75, 951, 8, 75, 1, 76, 1, 76, 1, 76, 1, 77, 1, 77, 1, 77, 1,
		77, 1, 78, 1, 78, 1, 78, 3, 78, 963, 8, 78, 1, 79, 1, 79, 1, 79, 1, 79,
		1, 79, 1, 79, 3, 79, 971, 8, 79, 1, 80, 1, 80, 1, 81, 1, 81, 1, 81, 1,
		81, 1, 81, 1, 81, 1, 81, 1, 81, 3, 81, 983, 8, 81, 1, 82, 1, 82, 1, 82,
		1, 82, 1, 83, 1, 83, 1, 83, 1, 83, 1, 83, 1, 83, 3, 83, 995, 8, 83, 1,
		84, 1, 84, 1, 84, 1, 84, 1, 84, 1, 84, 1, 84, 1, 84, 1, 85, 1, 85, 1,
		85, 1, 85, 1, 85, 3, 85, 1010, 8, 85, 1, 86, 1, 86, 1, 86, 1, 86, 1, 86,
		3, 86, 1017, 8, 86, 1, 87, 1, 87, 1, 87, 1, 87, 1, 87, 3, 87, 1024, 8,
		87, 1, 88, 1, 88, 1, 88, 1, 88, 1, 89, 1, 89, 3, 89, 1032, 8, 89, 1, 90,
		1, 90, 1, 90, 1, 90, 1, 90, 3, 90, 1039, 8, 90, 1, 91, 1, 91, 1, 91, 1,
		91, 1, 91, 1, 91, 1, 92, 1, 92, 1, 92, 3, 92, 1050, 8, 92, 1, 93, 1, 93,
		1, 93, 1, 93, 1, 93, 1, 93, 1, 93, 1, 93, 3, 93, 1060, 8, 93, 1, 94, 1,
		94, 1, 94, 1, 94, 1, 94, 3, 94, 1067, 8, 94, 1, 95, 1, 95, 5, 95, 1071,
		8, 95, 10, 95, 12, 95, 1074, 9, 95, 1, 96, 1, 96, 1, 96, 1, 97, 1, 97,
		1, 97, 1, 97, 1, 97, 1, 97, 1, 97, 1, 97, 3, 97, 1087, 8, 97, 1, 98, 1,
		98, 3, 98, 1091, 8, 98, 1, 99, 1, 99, 1, 99, 1, 99, 1, 99, 1, 99, 3, 99,
		1099, 8, 99, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1, 100, 1,
		101, 1, 101, 1, 101, 1, 101, 1, 101, 1, 101, 3, 101, 1114, 8, 101, 1,
		102, 1, 102, 1, 102, 1, 102, 1, 102, 3, 102, 1121, 8, 102, 1, 103, 1,
		103, 1, 103, 1, 103, 1, 103, 1, 103, 1, 103, 1, 103, 1, 103, 1, 103, 1,
		103, 1, 103, 1, 103, 3, 103, 1136, 8, 103, 1, 104, 1, 104, 1, 104, 1,
		104, 1, 104, 3, 104, 1143, 8, 104, 1, 105, 1, 105, 1, 105, 1, 105, 1,
		106, 1, 106, 1, 106, 1, 106, 1, 106, 1, 106, 1, 106, 1, 106, 5, 106, 1157,
		8, 106, 10, 106, 12, 106, 1160, 9, 106, 1, 107, 1, 107, 1, 107, 1, 107,
		1, 108, 1, 108, 1, 108, 1, 108, 1, 108, 3, 108, 1171, 8, 108, 1, 109,
		1, 109, 1, 109, 1, 109, 1, 110, 1, 110, 1, 110, 1, 110, 1, 111, 1, 111,
		1, 111, 1, 111, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112,
		1, 112, 1, 112, 1, 112, 3, 112, 1195, 8, 112, 1, 113, 1, 113, 1, 113,
		1, 113, 1, 113, 3, 113, 1202, 8, 113, 1, 114, 1, 114, 3, 114, 1206, 8,
		114, 1, 115, 1, 115, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 1,
		116, 1, 116, 1, 116, 1, 116, 3, 116, 1220, 8, 116, 1, 117, 1, 117, 1,
		117, 1, 117, 1, 117, 3, 117, 1227, 8, 117, 1, 118, 1, 118, 3, 118, 1231,
		8, 118, 1, 119, 1, 119, 1, 120, 1, 120, 3, 120, 1237, 8, 120, 1, 121,
		1, 121, 1, 122, 1, 122, 1, 122, 1, 123, 1, 123, 1, 123, 1, 123, 1, 123,
		1, 123, 3, 123, 1250, 8, 123, 1, 124, 1, 124, 3, 124, 1254, 8, 124, 1,
		125, 1, 125, 3, 125, 1258, 8, 125, 1, 126, 1, 126, 1, 126, 1, 126, 1,
		126, 1, 126, 1, 126, 1, 127, 1, 127, 3, 127, 1269, 8, 127, 1, 128, 1,
		128, 1, 128, 3, 128, 1274, 8, 128, 1, 129, 1, 129, 3, 129, 1278, 8, 129,
		1, 130, 1, 130, 1, 130, 1, 130, 1, 131, 1, 131, 3, 131, 1286, 8, 131,
		1, 132, 1, 132, 1, 132, 1, 132, 1, 132, 1, 132, 1, 132, 1, 132, 5, 132,
		1296, 8, 132, 10, 132, 12, 132, 1299, 9, 132, 1, 133, 1, 133, 1, 133,
		1, 133, 1, 133, 1, 133, 1, 133, 1, 133, 5, 133, 1309, 8, 133, 10, 133,
		12, 133, 1312, 9, 133, 1, 134, 1, 134, 1, 134, 1, 134, 3, 134, 1318, 8,
		134, 1, 135, 1, 135, 1, 135, 1, 135, 1, 136, 1, 136, 3, 136, 1326, 8,
		136, 1, 137, 1, 137, 1, 137, 1, 137, 1, 137, 1, 137, 3, 137, 1334, 8,
		137, 1, 138, 1, 138, 1, 138, 1, 138, 1, 138, 1, 138, 1, 138, 1, 139, 1,
		139, 1, 139, 1, 139, 1, 139, 3, 139, 1348, 8, 139, 1, 140, 1, 140, 1,
		140, 3, 140, 1353, 8, 140, 1, 141, 1, 141, 1, 142, 1, 142, 3, 142, 1359,
		8, 142, 1, 143, 1, 143, 1, 144, 1, 144, 1, 144, 1, 144, 1, 145, 1, 145,
		1, 146, 1, 146, 1, 146, 1, 146, 1, 146, 1, 146, 3, 146, 1375, 8, 146,
		1, 147, 1, 147, 3, 147, 1379, 8, 147, 1, 148, 1, 148, 1, 149, 1, 149,
		1, 149, 1, 149, 1, 149, 1, 149, 3, 149, 1389, 8, 149, 1, 150, 1, 150,
		1, 150, 1, 150, 1, 150, 1, 150, 3, 150, 1397, 8, 150, 1, 151, 1, 151,
		1, 151, 1, 151, 1, 151, 3, 151, 1404, 8, 151, 1, 152, 1, 152, 3, 152,
		1408, 8, 152, 1, 153, 1, 153, 1, 153, 3, 153, 1413, 8, 153, 1, 154, 1,
		154, 1, 154, 1, 154, 1, 154, 1, 154, 1, 154, 1, 154, 3, 154, 1423, 8,
		154, 1, 155, 1, 155, 1, 155, 1, 155, 1, 155, 3, 155, 1430, 8, 155, 1,
		156, 1, 156, 5, 156, 1434, 8, 156, 10, 156, 12, 156, 1437, 9, 156, 1,
		157, 1, 157, 1, 157, 1, 158, 1, 158, 1, 158, 1, 158, 1, 158, 3, 158, 1447,
		8, 158, 1, 159, 1, 159, 1, 159, 1, 159, 1, 159, 1, 159, 5, 159, 1455,
		8, 159, 10, 159, 12, 159, 1458, 9, 159, 1, 160, 1, 160, 1, 160, 1, 160,
		1, 160, 1, 160, 1, 160, 1, 160, 1, 160, 3, 160, 1469, 8, 160, 1, 161,
		1, 161, 1, 161, 3, 161, 1474, 8, 161, 1, 162, 1, 162, 3, 162, 1478, 8,
		162, 1, 163, 1, 163, 1, 164, 1, 164, 5, 164, 1484, 8, 164, 10, 164, 12,
		164, 1487, 9, 164, 3, 164, 1489, 8, 164, 1, 165, 1, 165, 1, 166, 1, 166,
		3, 166, 1495, 8, 166, 1, 167, 1, 167, 3, 167, 1499, 8, 167, 1, 168, 1,
		168, 1, 169, 1, 169, 1, 169, 1, 169, 1, 169, 1, 169, 1, 169, 3, 169, 1510,
		8, 169, 1, 170, 1, 170, 1, 171, 1, 171, 1, 172, 1, 172, 1, 173, 1, 173,
		1, 174, 1, 174, 1, 175, 1, 175, 3, 175, 1524, 8, 175, 1, 176, 1, 176,
		1, 176, 3, 176, 1529, 8, 176, 1, 177, 1, 177, 1, 178, 1, 178, 1, 179,
		1, 179, 1, 180, 1, 180, 1, 181, 1, 181, 1, 182, 1, 182, 1, 183, 1, 183,
		1, 184, 1, 184, 1, 185, 1, 185, 1, 186, 1, 186, 1, 187, 1, 187, 1, 188,
		1, 188, 1, 189, 1, 189, 1, 190, 1, 190, 1, 191, 1, 191, 3, 191, 1561,
		8, 191, 1, 192, 1, 192, 1, 193, 1, 193, 1, 193, 1, 193, 1, 193, 1, 193,
		1, 193, 3, 193, 1572, 8, 193, 1, 194, 1, 194, 1, 194, 1, 194, 1, 194,
		3, 194, 1579, 8, 194, 1, 195, 1, 195, 3, 195, 1583, 8, 195, 1, 196, 1,
		196, 1, 196, 1, 196, 1, 196, 1, 196, 1, 196, 1, 196, 1, 197, 1, 197, 1,
		198, 1, 198, 1, 198, 1, 198, 1, 198, 1, 198, 1, 198, 1, 198, 1, 199, 1,
		199, 1, 200, 1, 200, 1, 201, 1, 201, 1, 201, 1, 201, 1, 201, 1, 202, 1,
		202, 1, 202, 3, 202, 1615, 8, 202, 1, 203, 1, 203, 1, 203, 1, 203, 1,
		203, 3, 203, 1622, 8, 203, 1, 204, 1, 204, 5, 204, 1626, 8, 204, 10, 204,
		12, 204, 1629, 9, 204, 1, 205, 1, 205, 1, 205, 1, 206, 1, 206, 1, 206,
		1, 207, 1, 207, 1, 207, 3, 207, 1640, 8, 207, 1, 208, 1, 208, 1, 208,
		3, 208, 1645, 8, 208, 1, 209, 1, 209, 1, 210, 1, 210, 1, 210, 1, 210,
		1, 210, 1, 211, 1, 211, 1, 211, 1, 211, 1, 211, 1, 211, 1, 211, 1, 211,
		3, 211, 1662, 8, 211, 1, 212, 1, 212, 1, 212, 1, 212, 1, 212, 3, 212,
		1669, 8, 212, 1, 213, 1, 213, 1, 213, 1, 213, 1, 213, 3, 213, 1676, 8,
		213, 1, 214, 1, 214, 1, 215, 1, 215, 1, 215, 1, 215, 1, 215, 1, 215, 3,
		215, 1686, 8, 215, 1, 216, 1, 216, 1, 216, 1, 216, 1, 216, 1, 216, 3,
		216, 1694, 8, 216, 1, 217, 1, 217, 1, 217, 1, 217, 1, 217, 1, 218, 1,
		218, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 1,
		218, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 1, 218, 1,
		218, 3, 218, 1721, 8, 218, 1, 219, 1, 219, 1, 219, 1, 219, 1, 219, 3,
		219, 1728, 8, 219, 1, 220, 1, 220, 5, 220, 1732, 8, 220, 10, 220, 12,
		220, 1735, 9, 220, 1, 221, 1, 221, 1, 221, 1, 222, 1, 222, 3, 222, 1742,
		8, 222, 1, 223, 1, 223, 1, 224, 1, 224, 1, 225, 1, 225, 1, 226, 1, 226,
		1, 227, 1, 227, 1, 228, 1, 228, 1, 228, 0, 11, 34, 36, 38, 110, 112, 130,
		132, 212, 264, 266, 318, 229, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22,
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
		434, 436, 438, 440, 442, 444, 446, 448, 450, 452, 454, 456, 0, 6, 1, 0,
		28, 30, 2, 0, 26, 26, 32, 32, 2, 0, 10, 10, 85, 85, 1, 0, 44, 48, 2, 0,
		61, 62, 67, 67, 3, 0, 68, 68, 75, 75, 78, 78, 1730, 0, 461, 1, 0, 0, 0,
		2, 468, 1, 0, 0, 0, 4, 476, 1, 0, 0, 0, 6, 478, 1, 0, 0, 0, 8, 487, 1,
		0, 0, 0, 10, 489, 1, 0, 0, 0, 12, 498, 1, 0, 0, 0, 14, 507, 1, 0, 0, 0,
		16, 516, 1, 0, 0, 0, 18, 525, 1, 0, 0, 0, 20, 539, 1, 0, 0, 0, 22, 545,
		1, 0, 0, 0, 24, 550, 1, 0, 0, 0, 26, 558, 1, 0, 0, 0, 28, 563, 1, 0, 0,
		0, 30, 565, 1, 0, 0, 0, 32, 572, 1, 0, 0, 0, 34, 574, 1, 0, 0, 0, 36,
		587, 1, 0, 0, 0, 38, 600, 1, 0, 0, 0, 40, 616, 1, 0, 0, 0, 42, 620, 1,
		0, 0, 0, 44, 629, 1, 0, 0, 0, 46, 631, 1, 0, 0, 0, 48, 634, 1, 0, 0, 0,
		50, 645, 1, 0, 0, 0, 52, 647, 1, 0, 0, 0, 54, 653, 1, 0, 0, 0, 56, 655,
		1, 0, 0, 0, 58, 658, 1, 0, 0, 0, 60, 666, 1, 0, 0, 0, 62, 670, 1, 0, 0,
		0, 64, 680, 1, 0, 0, 0, 66, 684, 1, 0, 0, 0, 68, 686, 1, 0, 0, 0, 70,
		690, 1, 0, 0, 0, 72, 692, 1, 0, 0, 0, 74, 705, 1, 0, 0, 0, 76, 712, 1,
		0, 0, 0, 78, 719, 1, 0, 0, 0, 80, 721, 1, 0, 0, 0, 82, 730, 1, 0, 0, 0,
		84, 738, 1, 0, 0, 0, 86, 745, 1, 0, 0, 0, 88, 752, 1, 0, 0, 0, 90, 769,
		1, 0, 0, 0, 92, 771, 1, 0, 0, 0, 94, 773, 1, 0, 0, 0, 96, 780, 1, 0, 0,
		0, 98, 791, 1, 0, 0, 0, 100, 796, 1, 0, 0, 0, 102, 798, 1, 0, 0, 0, 104,
		800, 1, 0, 0, 0, 106, 805, 1, 0, 0, 0, 108, 815, 1, 0, 0, 0, 110, 817,
		1, 0, 0, 0, 112, 830, 1, 0, 0, 0, 114, 843, 1, 0, 0, 0, 116, 847, 1, 0,
		0, 0, 118, 851, 1, 0, 0, 0, 120, 858, 1, 0, 0, 0, 122, 866, 1, 0, 0, 0,
		124, 870, 1, 0, 0, 0, 126, 872, 1, 0, 0, 0, 128, 878, 1, 0, 0, 0, 130,
		880, 1, 0, 0, 0, 132, 893, 1, 0, 0, 0, 134, 909, 1, 0, 0, 0, 136, 913,
		1, 0, 0, 0, 138, 922, 1, 0, 0, 0, 140, 924, 1, 0, 0, 0, 142, 926, 1, 0,
		0, 0, 144, 938, 1, 0, 0, 0, 146, 942, 1, 0, 0, 0, 148, 944, 1, 0, 0, 0,
		150, 950, 1, 0, 0, 0, 152, 952, 1, 0, 0, 0, 154, 955, 1, 0, 0, 0, 156,
		962, 1, 0, 0, 0, 158, 970, 1, 0, 0, 0, 160, 972, 1, 0, 0, 0, 162, 982,
		1, 0, 0, 0, 164, 984, 1, 0, 0, 0, 166, 994, 1, 0, 0, 0, 168, 996, 1, 0,
		0, 0, 170, 1009, 1, 0, 0, 0, 172, 1016, 1, 0, 0, 0, 174, 1023, 1, 0, 0,
		0, 176, 1025, 1, 0, 0, 0, 178, 1031, 1, 0, 0, 0, 180, 1038, 1, 0, 0, 0,
		182, 1040, 1, 0, 0, 0, 184, 1049, 1, 0, 0, 0, 186, 1059, 1, 0, 0, 0, 188,
		1066, 1, 0, 0, 0, 190, 1068, 1, 0, 0, 0, 192, 1075, 1, 0, 0, 0, 194, 1086,
		1, 0, 0, 0, 196, 1090, 1, 0, 0, 0, 198, 1098, 1, 0, 0, 0, 200, 1100, 1,
		0, 0, 0, 202, 1113, 1, 0, 0, 0, 204, 1120, 1, 0, 0, 0, 206, 1135, 1, 0,
		0, 0, 208, 1142, 1, 0, 0, 0, 210, 1144, 1, 0, 0, 0, 212, 1148, 1, 0, 0,
		0, 214, 1161, 1, 0, 0, 0, 216, 1170, 1, 0, 0, 0, 218, 1172, 1, 0, 0, 0,
		220, 1176, 1, 0, 0, 0, 222, 1180, 1, 0, 0, 0, 224, 1194, 1, 0, 0, 0, 226,
		1201, 1, 0, 0, 0, 228, 1205, 1, 0, 0, 0, 230, 1207, 1, 0, 0, 0, 232, 1219,
		1, 0, 0, 0, 234, 1226, 1, 0, 0, 0, 236, 1230, 1, 0, 0, 0, 238, 1232, 1,
		0, 0, 0, 240, 1236, 1, 0, 0, 0, 242, 1238, 1, 0, 0, 0, 244, 1240, 1, 0,
		0, 0, 246, 1249, 1, 0, 0, 0, 248, 1253, 1, 0, 0, 0, 250, 1257, 1, 0, 0,
		0, 252, 1259, 1, 0, 0, 0, 254, 1268, 1, 0, 0, 0, 256, 1273, 1, 0, 0, 0,
		258, 1277, 1, 0, 0, 0, 260, 1279, 1, 0, 0, 0, 262, 1285, 1, 0, 0, 0, 264,
		1287, 1, 0, 0, 0, 266, 1300, 1, 0, 0, 0, 268, 1317, 1, 0, 0, 0, 270, 1319,
		1, 0, 0, 0, 272, 1325, 1, 0, 0, 0, 274, 1333, 1, 0, 0, 0, 276, 1335, 1,
		0, 0, 0, 278, 1347, 1, 0, 0, 0, 280, 1352, 1, 0, 0, 0, 282, 1354, 1, 0,
		0, 0, 284, 1358, 1, 0, 0, 0, 286, 1360, 1, 0, 0, 0, 288, 1362, 1, 0, 0,
		0, 290, 1366, 1, 0, 0, 0, 292, 1374, 1, 0, 0, 0, 294, 1378, 1, 0, 0, 0,
		296, 1380, 1, 0, 0, 0, 298, 1388, 1, 0, 0, 0, 300, 1396, 1, 0, 0, 0, 302,
		1403, 1, 0, 0, 0, 304, 1407, 1, 0, 0, 0, 306, 1412, 1, 0, 0, 0, 308, 1422,
		1, 0, 0, 0, 310, 1429, 1, 0, 0, 0, 312, 1431, 1, 0, 0, 0, 314, 1438, 1,
		0, 0, 0, 316, 1446, 1, 0, 0, 0, 318, 1448, 1, 0, 0, 0, 320, 1468, 1, 0,
		0, 0, 322, 1473, 1, 0, 0, 0, 324, 1477, 1, 0, 0, 0, 326, 1479, 1, 0, 0,
		0, 328, 1488, 1, 0, 0, 0, 330, 1490, 1, 0, 0, 0, 332, 1494, 1, 0, 0, 0,
		334, 1498, 1, 0, 0, 0, 336, 1500, 1, 0, 0, 0, 338, 1509, 1, 0, 0, 0, 340,
		1511, 1, 0, 0, 0, 342, 1513, 1, 0, 0, 0, 344, 1515, 1, 0, 0, 0, 346, 1517,
		1, 0, 0, 0, 348, 1519, 1, 0, 0, 0, 350, 1523, 1, 0, 0, 0, 352, 1528, 1,
		0, 0, 0, 354, 1530, 1, 0, 0, 0, 356, 1532, 1, 0, 0, 0, 358, 1534, 1, 0,
		0, 0, 360, 1536, 1, 0, 0, 0, 362, 1538, 1, 0, 0, 0, 364, 1540, 1, 0, 0,
		0, 366, 1542, 1, 0, 0, 0, 368, 1544, 1, 0, 0, 0, 370, 1546, 1, 0, 0, 0,
		372, 1548, 1, 0, 0, 0, 374, 1550, 1, 0, 0, 0, 376, 1552, 1, 0, 0, 0, 378,
		1554, 1, 0, 0, 0, 380, 1556, 1, 0, 0, 0, 382, 1560, 1, 0, 0, 0, 384, 1562,
		1, 0, 0, 0, 386, 1571, 1, 0, 0, 0, 388, 1578, 1, 0, 0, 0, 390, 1582, 1,
		0, 0, 0, 392, 1584, 1, 0, 0, 0, 394, 1592, 1, 0, 0, 0, 396, 1594, 1, 0,
		0, 0, 398, 1602, 1, 0, 0, 0, 400, 1604, 1, 0, 0, 0, 402, 1606, 1, 0, 0,
		0, 404, 1614, 1, 0, 0, 0, 406, 1621, 1, 0, 0, 0, 408, 1623, 1, 0, 0, 0,
		410, 1630, 1, 0, 0, 0, 412, 1633, 1, 0, 0, 0, 414, 1639, 1, 0, 0, 0, 416,
		1644, 1, 0, 0, 0, 418, 1646, 1, 0, 0, 0, 420, 1648, 1, 0, 0, 0, 422, 1661,
		1, 0, 0, 0, 424, 1668, 1, 0, 0, 0, 426, 1675, 1, 0, 0, 0, 428, 1677, 1,
		0, 0, 0, 430, 1685, 1, 0, 0, 0, 432, 1693, 1, 0, 0, 0, 434, 1695, 1, 0,
		0, 0, 436, 1720, 1, 0, 0, 0, 438, 1727, 1, 0, 0, 0, 440, 1729, 1, 0, 0,
		0, 442, 1736, 1, 0, 0, 0, 444, 1741, 1, 0, 0, 0, 446, 1743, 1, 0, 0, 0,
		448, 1745, 1, 0, 0, 0, 450, 1747, 1, 0, 0, 0, 452, 1749, 1, 0, 0, 0, 454,
		1751, 1, 0, 0, 0, 456, 1753, 1, 0, 0, 0, 458, 460, 3, 2, 1, 0, 459, 458,
		1, 0, 0, 0, 460, 463, 1, 0, 0, 0, 461, 459, 1, 0, 0, 0, 461, 462, 1, 0,
		0, 0, 462, 464, 1, 0, 0, 0, 463, 461, 1, 0, 0, 0, 464, 465, 5, 0, 0, 1,
		465, 1, 1, 0, 0, 0, 466, 469, 3, 4, 2, 0, 467, 469, 3, 420, 210, 0, 468,
		466, 1, 0, 0, 0, 468, 467, 1, 0, 0, 0, 469, 3, 1, 0, 0, 0, 470, 477, 3,
		10, 5, 0, 471, 477, 3, 12, 6, 0, 472, 477, 3, 14, 7, 0, 473, 477, 3, 16,
		8, 0, 474, 477, 3, 18, 9, 0, 475, 477, 3, 6, 3, 0, 476, 470, 1, 0, 0,
		0, 476, 471, 1, 0, 0, 0, 476, 472, 1, 0, 0, 0, 476, 473, 1, 0, 0, 0, 476,
		474, 1, 0, 0, 0, 476, 475, 1, 0, 0, 0, 477, 5, 1, 0, 0, 0, 478, 479, 5,
		1, 0, 0, 479, 480, 3, 444, 222, 0, 480, 481, 5, 2, 0, 0, 481, 482, 3,
		22, 11, 0, 482, 483, 5, 2, 0, 0, 483, 484, 3, 8, 4, 0, 484, 485, 3, 20,
		10, 0, 485, 486, 5, 3, 0, 0, 486, 7, 1, 0, 0, 0, 487, 488, 3, 254, 127,
		0, 488, 9, 1, 0, 0, 0, 489, 490, 5, 4, 0, 0, 490, 491, 3, 444, 222, 0,
		491, 492, 5, 2, 0, 0, 492, 493, 3, 22, 11, 0, 493, 494, 5, 2, 0, 0, 494,
		495, 3, 24, 12, 0, 495, 496, 3, 20, 10, 0, 496, 497, 5, 3, 0, 0, 497,
		11, 1, 0, 0, 0, 498, 499, 5, 5, 0, 0, 499, 500, 3, 444, 222, 0, 500, 501,
		5, 2, 0, 0, 501, 502, 3, 22, 11, 0, 502, 503, 5, 2, 0, 0, 503, 504, 3,
		120, 60, 0, 504, 505, 3, 20, 10, 0, 505, 506, 5, 3, 0, 0, 506, 13, 1,
		0, 0, 0, 507, 508, 5, 6, 0, 0, 508, 509, 3, 444, 222, 0, 509, 510, 5,
		2, 0, 0, 510, 511, 3, 22, 11, 0, 511, 512, 5, 2, 0, 0, 512, 513, 3, 248,
		124, 0, 513, 514, 3, 20, 10, 0, 514, 515, 5, 3, 0, 0, 515, 15, 1, 0, 0,
		0, 516, 517, 5, 7, 0, 0, 517, 518, 3, 444, 222, 0, 518, 519, 5, 2, 0,
		0, 519, 520, 3, 22, 11, 0, 520, 521, 5, 2, 0, 0, 521, 522, 3, 254, 127,
		0, 522, 523, 3, 20, 10, 0, 523, 524, 5, 3, 0, 0, 524, 17, 1, 0, 0, 0,
		525, 526, 5, 8, 0, 0, 526, 527, 3, 444, 222, 0, 527, 528, 5, 2, 0, 0,
		528, 529, 3, 22, 11, 0, 529, 530, 5, 2, 0, 0, 530, 531, 3, 316, 158, 0,
		531, 532, 3, 20, 10, 0, 532, 533, 5, 3, 0, 0, 533, 19, 1, 0, 0, 0, 534,
		535, 5, 2, 0, 0, 535, 536, 3, 386, 193, 0, 536, 537, 3, 416, 208, 0, 537,
		540, 1, 0, 0, 0, 538, 540, 3, 456, 228, 0, 539, 534, 1, 0, 0, 0, 539,
		538, 1, 0, 0, 0, 540, 21, 1, 0, 0, 0, 541, 546, 5, 67, 0, 0, 542, 543,
		5, 67, 0, 0, 543, 544, 5, 9, 0, 0, 544, 546, 3, 430, 215, 0, 545, 541,
		1, 0, 0, 0, 545, 542, 1, 0, 0, 0, 546, 23, 1, 0, 0, 0, 547, 551, 3, 26,
		13, 0, 548, 551, 3, 98, 49, 0, 549, 551, 3, 114, 57, 0, 550, 547, 1, 0,
		0, 0, 550, 548, 1, 0, 0, 0, 550, 549, 1, 0, 0, 0, 551, 25, 1, 0, 0, 0,
		552, 559, 3, 44, 22, 0, 553, 559, 3, 54, 27, 0, 554, 559, 3, 28, 14, 0,
		555, 559, 3, 68, 34, 0, 556, 559, 3, 116, 58, 0, 557, 559, 3, 118, 59,
		0, 558, 552, 1, 0, 0, 0, 558, 553, 1, 0, 0, 0, 558, 554, 1, 0, 0, 0, 558,
		555, 1, 0, 0, 0, 558, 556, 1, 0, 0, 0, 558, 557, 1, 0, 0, 0, 559, 27,
		1, 0, 0, 0, 560, 564, 3, 30, 15, 0, 561, 564, 3, 32, 16, 0, 562, 564,
		3, 106, 53, 0, 563, 560, 1, 0, 0, 0, 563, 561, 1, 0, 0, 0, 563, 562, 1,
		0, 0, 0, 564, 29, 1, 0, 0, 0, 565, 566, 3, 40, 20, 0, 566, 567, 3, 338,
		169, 0, 567, 568, 3, 40, 20, 0, 568, 31, 1, 0, 0, 0, 569, 573, 3, 34,
		17, 0, 570, 573, 3, 36, 18, 0, 571, 573, 3, 38, 19, 0, 572, 569, 1, 0,
		0, 0, 572, 570, 1, 0, 0, 0, 572, 571, 1, 0, 0, 0, 573, 33, 1, 0, 0, 0,
		574, 575, 6, 17, -1, 0, 575, 576, 3, 40, 20, 0, 576, 577, 5, 85, 0, 0,
		577, 578, 3, 40, 20, 0, 578, 584, 1, 0, 0, 0, 579, 580, 10, 1, 0, 0, 580,
		581, 5, 85, 0, 0, 581, 583, 3, 40, 20, 0, 582, 579, 1, 0, 0, 0, 583, 586,
		1, 0, 0, 0, 584, 582, 1, 0, 0, 0, 584, 585, 1, 0, 0, 0, 585, 35, 1, 0,
		0, 0, 586, 584, 1, 0, 0, 0, 587, 588, 6, 18, -1, 0, 588, 589, 3, 40, 20,
		0, 589, 590, 5, 10, 0, 0, 590, 591, 3, 40, 20, 0, 591, 597, 1, 0, 0, 0,
		592, 593, 10, 1, 0, 0, 593, 594, 5, 10, 0, 0, 594, 596, 3, 40, 20, 0,
		595, 592, 1, 0, 0, 0, 596, 599, 1, 0, 0, 0, 597, 595, 1, 0, 0, 0, 597,
		598, 1, 0, 0, 0, 598, 37, 1, 0, 0, 0, 599, 597, 1, 0, 0, 0, 600, 601,
		6, 19, -1, 0, 601, 602, 3, 40, 20, 0, 602, 603, 5, 11, 0, 0, 603, 604,
		3, 40, 20, 0, 604, 610, 1, 0, 0, 0, 605, 606, 10, 1, 0, 0, 606, 607, 5,
		11, 0, 0, 607, 609, 3, 40, 20, 0, 608, 605, 1, 0, 0, 0, 609, 612, 1, 0,
		0, 0, 610, 608, 1, 0, 0, 0, 610, 611, 1, 0, 0, 0, 611, 39, 1, 0, 0, 0,
		612, 610, 1, 0, 0, 0, 613, 617, 3, 44, 22, 0, 614, 617, 3, 54, 27, 0,
		615, 617, 3, 68, 34, 0, 616, 613, 1, 0, 0, 0, 616, 614, 1, 0, 0, 0, 616,
		615, 1, 0, 0, 0, 617, 41, 1, 0, 0, 0, 618, 621, 3, 44, 22, 0, 619, 621,
		3, 56, 28, 0, 620, 618, 1, 0, 0, 0, 620, 619, 1, 0, 0, 0, 621, 43, 1,
		0, 0, 0, 622, 630, 3, 46, 23, 0, 623, 630, 3, 60, 30, 0, 624, 630, 3,
		384, 192, 0, 625, 626, 5, 12, 0, 0, 626, 627, 3, 26, 13, 0, 627, 628,
		5, 13, 0, 0, 628, 630, 1, 0, 0, 0, 629, 622, 1, 0, 0, 0, 629, 623, 1,
		0, 0, 0, 629, 624, 1, 0, 0, 0, 629, 625, 1, 0, 0, 0, 630, 45, 1, 0, 0,
		0, 631, 632, 3, 48, 24, 0, 632, 633, 3, 40, 20, 0, 633, 47, 1, 0, 0, 0,
		634, 635, 3, 322, 161, 0, 635, 636, 5, 14, 0, 0, 636, 637, 3, 50, 25,
		0, 637, 638, 5, 15, 0, 0, 638, 639, 5, 16, 0, 0, 639, 49, 1, 0, 0, 0,
		640, 646, 3, 52, 26, 0, 641, 642, 3, 52, 26, 0, 642, 643, 5, 2, 0, 0,
		643, 644, 3, 50, 25, 0, 644, 646, 1, 0, 0, 0, 645, 640, 1, 0, 0, 0, 645,
		641, 1, 0, 0, 0, 646, 51, 1, 0, 0, 0, 647, 648, 3, 384, 192, 0, 648, 649,
		5, 16, 0, 0, 649, 650, 3, 100, 50, 0, 650, 53, 1, 0, 0, 0, 651, 654, 3,
		56, 28, 0, 652, 654, 3, 58, 29, 0, 653, 651, 1, 0, 0, 0, 653, 652, 1,
		0, 0, 0, 654, 55, 1, 0, 0, 0, 655, 656, 3, 324, 162, 0, 656, 657, 3, 42,
		21, 0, 657, 57, 1, 0, 0, 0, 658, 659, 3, 84, 42, 0, 659, 660, 3, 366,
		183, 0, 660, 661, 3, 84, 42, 0, 661, 59, 1, 0, 0, 0, 662, 667, 3, 62,
		31, 0, 663, 667, 3, 64, 32, 0, 664, 667, 3, 70, 35, 0, 665, 667, 3, 90,
		45, 0, 666, 662, 1, 0, 0, 0, 666, 663, 1, 0, 0, 0, 666, 664, 1, 0, 0,
		0, 666, 665, 1, 0, 0, 0, 667, 61, 1, 0, 0, 0, 668, 671, 3, 368, 184, 0,
		669, 671, 3, 88, 44, 0, 670, 668, 1, 0, 0, 0, 670, 669, 1, 0, 0, 0, 671,
		63, 1, 0, 0, 0, 672, 681, 3, 372, 186, 0, 673, 681, 3, 66, 33, 0, 674,
		675, 5, 12, 0, 0, 675, 676, 3, 86, 43, 0, 676, 677, 5, 13, 0, 0, 677,
		681, 1, 0, 0, 0, 678, 681, 3, 224, 112, 0, 679, 681, 3, 72, 36, 0, 680,
		672, 1, 0, 0, 0, 680, 673, 1, 0, 0, 0, 680, 674, 1, 0, 0, 0, 680, 678,
		1, 0, 0, 0, 680, 679, 1, 0, 0, 0, 681, 65, 1, 0, 0, 0, 682, 685, 3, 382,
		191, 0, 683, 685, 3, 380, 190, 0, 684, 682, 1, 0, 0, 0, 684, 683, 1, 0,
		0, 0, 685, 67, 1, 0, 0, 0, 686, 687, 3, 84, 42, 0, 687, 688, 3, 362, 181,
		0, 688, 689, 3, 84, 42, 0, 689, 69, 1, 0, 0, 0, 690, 691, 3, 376, 188,
		0, 691, 71, 1, 0, 0, 0, 692, 693, 5, 17, 0, 0, 693, 694, 3, 74, 37, 0,
		694, 695, 5, 2, 0, 0, 695, 696, 3, 78, 39, 0, 696, 697, 5, 2, 0, 0, 697,
		698, 3, 26, 13, 0, 698, 699, 5, 13, 0, 0, 699, 73, 1, 0, 0, 0, 700, 706,
		3, 98, 49, 0, 701, 702, 5, 14, 0, 0, 702, 703, 3, 76, 38, 0, 703, 704,
		5, 15, 0, 0, 704, 706, 1, 0, 0, 0, 705, 700, 1, 0, 0, 0, 705, 701, 1,
		0, 0, 0, 706, 75, 1, 0, 0, 0, 707, 713, 3, 98, 49, 0, 708, 709, 3, 98,
		49, 0, 709, 710, 5, 2, 0, 0, 710, 711, 3, 76, 38, 0, 711, 713, 1, 0, 0,
		0, 712, 707, 1, 0, 0, 0, 712, 708, 1, 0, 0, 0, 713, 77, 1, 0, 0, 0, 714,
		720, 3, 80, 40, 0, 715, 716, 5, 14, 0, 0, 716, 717, 3, 82, 41, 0, 717,
		718, 5, 15, 0, 0, 718, 720, 1, 0, 0, 0, 719, 714, 1, 0, 0, 0, 719, 715,
		1, 0, 0, 0, 720, 79, 1, 0, 0, 0, 721, 722, 3, 26, 13, 0, 722, 723, 3,
		346, 173, 0, 723, 724, 3, 26, 13, 0, 724, 81, 1, 0, 0, 0, 725, 731, 3,
		80, 40, 0, 726, 727, 3, 80, 40, 0, 727, 728, 5, 2, 0, 0, 728, 729, 3,
		82, 41, 0, 729, 731, 1, 0, 0, 0, 730, 725, 1, 0, 0, 0, 730, 726, 1, 0,
		0, 0, 731, 83, 1, 0, 0, 0, 732, 739, 3, 60, 30, 0, 733, 739, 3, 384, 192,
		0, 734, 735, 5, 12, 0, 0, 735, 736, 3, 26, 13, 0, 736, 737, 5, 13, 0,
		0, 737, 739, 1, 0, 0, 0, 738, 732, 1, 0, 0, 0, 738, 733, 1, 0, 0, 0, 738,
		734, 1, 0, 0, 0, 739, 85, 1, 0, 0, 0, 740, 746, 3, 338, 169, 0, 741, 746,
		3, 340, 170, 0, 742, 746, 3, 364, 182, 0, 743, 746, 3, 366, 183, 0, 744,
		746, 3, 324, 162, 0, 745, 740, 1, 0, 0, 0, 745, 741, 1, 0, 0, 0, 745,
		742, 1, 0, 0, 0, 745, 743, 1, 0, 0, 0, 745, 744, 1, 0, 0, 0, 746, 87,
		1, 0, 0, 0, 747, 753, 5, 18, 0, 0, 748, 749, 5, 14, 0, 0, 749, 750, 3,
		94, 47, 0, 750, 751, 5, 15, 0, 0, 751, 753, 1, 0, 0, 0, 752, 747, 1, 0,
		0, 0, 752, 748, 1, 0, 0, 0, 753, 89, 1, 0, 0, 0, 754, 755, 3, 374, 187,
		0, 755, 756, 5, 12, 0, 0, 756, 757, 3, 92, 46, 0, 757, 758, 5, 13, 0,
		0, 758, 770, 1, 0, 0, 0, 759, 760, 3, 378, 189, 0, 760, 761, 5, 12, 0,
		0, 761, 762, 3, 92, 46, 0, 762, 763, 5, 13, 0, 0, 763, 770, 1, 0, 0, 0,
		764, 765, 3, 370, 185, 0, 765, 766, 5, 12, 0, 0, 766, 767, 3, 92, 46,
		0, 767, 768, 5, 13, 0, 0, 768, 770, 1, 0, 0, 0, 769, 754, 1, 0, 0, 0,
		769, 759, 1, 0, 0, 0, 769, 764, 1, 0, 0, 0, 770, 91, 1, 0, 0, 0, 771,
		772, 3, 94, 47, 0, 772, 93, 1, 0, 0, 0, 773, 777, 3, 26, 13, 0, 774, 776,
		3, 96, 48, 0, 775, 774, 1, 0, 0, 0, 776, 779, 1, 0, 0, 0, 777, 775, 1,
		0, 0, 0, 777, 778, 1, 0, 0, 0, 778, 95, 1, 0, 0, 0, 779, 777, 1, 0, 0,
		0, 780, 781, 5, 2, 0, 0, 781, 782, 3, 26, 13, 0, 782, 97, 1, 0, 0, 0,
		783, 784, 3, 350, 175, 0, 784, 785, 5, 16, 0, 0, 785, 786, 3, 100, 50,
		0, 786, 792, 1, 0, 0, 0, 787, 788, 5, 12, 0, 0, 788, 789, 3, 98, 49, 0,
		789, 790, 5, 13, 0, 0, 790, 792, 1, 0, 0, 0, 791, 783, 1, 0, 0, 0, 791,
		787, 1, 0, 0, 0, 792, 99, 1, 0, 0, 0, 793, 797, 3, 102, 51, 0, 794, 797,
		3, 108, 54, 0, 795, 797, 3, 104, 52, 0, 796, 793, 1, 0, 0, 0, 796, 794,
		1, 0, 0, 0, 796, 795, 1, 0, 0, 0, 797, 101, 1, 0, 0, 0, 798, 799, 3, 44,
		22, 0, 799, 103, 1, 0, 0, 0, 800, 801, 3, 38, 19, 0, 801, 105, 1, 0, 0,
		0, 802, 806, 3, 108, 54, 0, 803, 806, 3, 110, 55, 0, 804, 806, 3, 112,
		56, 0, 805, 802, 1, 0, 0, 0, 805, 803, 1, 0, 0, 0, 805, 804, 1, 0, 0,
		0, 806, 107, 1, 0, 0, 0, 807, 808, 3, 102, 51, 0, 808, 809, 5, 88, 0,
		0, 809, 810, 3, 102, 51, 0, 810, 816, 1, 0, 0, 0, 811, 812, 3, 102, 51,
		0, 812, 813, 5, 88, 0, 0, 813, 814, 3, 108, 54, 0, 814, 816, 1, 0, 0,
		0, 815, 807, 1, 0, 0, 0, 815, 811, 1, 0, 0, 0, 816, 109, 1, 0, 0, 0, 817,
		818, 6, 55, -1, 0, 818, 819, 3, 102, 51, 0, 819, 820, 5, 86, 0, 0, 820,
		821, 3, 102, 51, 0, 821, 827, 1, 0, 0, 0, 822, 823, 10, 1, 0, 0, 823,
		824, 5, 86, 0, 0, 824, 826, 3, 102, 51, 0, 825, 822, 1, 0, 0, 0, 826,
		829, 1, 0, 0, 0, 827, 825, 1, 0, 0, 0, 827, 828, 1, 0, 0, 0, 828, 111,
		1, 0, 0, 0, 829, 827, 1, 0, 0, 0, 830, 831, 6, 56, -1, 0, 831, 832, 3,
		102, 51, 0, 832, 833, 5, 87, 0, 0, 833, 834, 3, 102, 51, 0, 834, 840,
		1, 0, 0, 0, 835, 836, 10, 1, 0, 0, 836, 837, 5, 87, 0, 0, 837, 839, 3,
		102, 51, 0, 838, 835, 1, 0, 0, 0, 839, 842, 1, 0, 0, 0, 840, 838, 1, 0,
		0, 0, 840, 841, 1, 0, 0, 0, 841, 113, 1, 0, 0, 0, 842, 840, 1, 0, 0, 0,
		843, 844, 3, 352, 176, 0, 844, 845, 3, 330, 165, 0, 845, 846, 3, 352,
		176, 0, 846, 115, 1, 0, 0, 0, 847, 848, 3, 60, 30, 0, 848, 849, 3, 348,
		174, 0, 849, 850, 3, 26, 13, 0, 850, 117, 1, 0, 0, 0, 851, 852, 3, 88,
		44, 0, 852, 853, 3, 344, 172, 0, 853, 854, 3, 88, 44, 0, 854, 119, 1,
		0, 0, 0, 855, 859, 3, 122, 61, 0, 856, 859, 3, 194, 97, 0, 857, 859, 3,
		218, 109, 0, 858, 855, 1, 0, 0, 0, 858, 856, 1, 0, 0, 0, 858, 857, 1,
		0, 0, 0, 859, 121, 1, 0, 0, 0, 860, 867, 3, 138, 69, 0, 861, 867, 3, 150,
		75, 0, 862, 867, 3, 124, 62, 0, 863, 867, 3, 164, 82, 0, 864, 867, 3,
		220, 110, 0, 865, 867, 3, 222, 111, 0, 866, 860, 1, 0, 0, 0, 866, 861,
		1, 0, 0, 0, 866, 862, 1, 0, 0, 0, 866, 863, 1, 0, 0, 0, 866, 864, 1, 0,
		0, 0, 866, 865, 1, 0, 0, 0, 867, 123, 1, 0, 0, 0, 868, 871, 3, 126, 63,
		0, 869, 871, 3, 128, 64, 0, 870, 868, 1, 0, 0, 0, 870, 869, 1, 0, 0, 0,
		871, 125, 1, 0, 0, 0, 872, 873, 3, 134, 67, 0, 873, 874, 3, 338, 169,
		0, 874, 875, 3, 134, 67, 0, 875, 127, 1, 0, 0, 0, 876, 879, 3, 130, 65,
		0, 877, 879, 3, 132, 66, 0, 878, 876, 1, 0, 0, 0, 878, 877, 1, 0, 0, 0,
		879, 129, 1, 0, 0, 0, 880, 881, 6, 65, -1, 0, 881, 882, 3, 134, 67, 0,
		882, 883, 5, 85, 0, 0, 883, 884, 3, 134, 67, 0, 884, 890, 1, 0, 0, 0,
		885, 886, 10, 1, 0, 0, 886, 887, 5, 85, 0, 0, 887, 889, 3, 134, 67, 0,
		888, 885, 1, 0, 0, 0, 889, 892, 1, 0, 0, 0, 890, 888, 1, 0, 0, 0, 890,
		891, 1, 0, 0, 0, 891, 131, 1, 0, 0, 0, 892, 890, 1, 0, 0, 0, 893, 894,
		6, 66, -1, 0, 894, 895, 3, 134, 67, 0, 895, 896, 5, 10, 0, 0, 896, 897,
		3, 134, 67, 0, 897, 903, 1, 0, 0, 0, 898, 899, 10, 1, 0, 0, 899, 900,
		5, 10, 0, 0, 900, 902, 3, 134, 67, 0, 901, 898, 1, 0, 0, 0, 902, 905,
		1, 0, 0, 0, 903, 901, 1, 0, 0, 0, 903, 904, 1, 0, 0, 0, 904, 133, 1, 0,
		0, 0, 905, 903, 1, 0, 0, 0, 906, 910, 3, 138, 69, 0, 907, 910, 3, 150,
		75, 0, 908, 910, 3, 164, 82, 0, 909, 906, 1, 0, 0, 0, 909, 907, 1, 0,
		0, 0, 909, 908, 1, 0, 0, 0, 910, 135, 1, 0, 0, 0, 911, 914, 3, 138, 69,
		0, 912, 914, 3, 152, 76, 0, 913, 911, 1, 0, 0, 0, 913, 912, 1, 0, 0, 0,
		914, 137, 1, 0, 0, 0, 915, 923, 3, 142, 71, 0, 916, 923, 3, 156, 78, 0,
		917, 923, 3, 140, 70, 0, 918, 919, 5, 12, 0, 0, 919, 920, 3, 122, 61,
		0, 920, 921, 5, 13, 0, 0, 921, 923, 1, 0, 0, 0, 922, 915, 1, 0, 0, 0,
		922, 916, 1, 0, 0, 0, 922, 917, 1, 0, 0, 0, 922, 918, 1, 0, 0, 0, 923,
		139, 1, 0, 0, 0, 924, 925, 3, 384, 192, 0, 925, 141, 1, 0, 0, 0, 926,
		927, 3, 334, 167, 0, 927, 928, 5, 14, 0, 0, 928, 929, 3, 144, 72, 0, 929,
		930, 5, 15, 0, 0, 930, 931, 5, 16, 0, 0, 931, 932, 3, 134, 67, 0, 932,
		143, 1, 0, 0, 0, 933, 939, 3, 146, 73, 0, 934, 935, 3, 146, 73, 0, 935,
		936, 5, 2, 0, 0, 936, 937, 3, 144, 72, 0, 937, 939, 1, 0, 0, 0, 938, 933,
		1, 0, 0, 0, 938, 934, 1, 0, 0, 0, 939, 145, 1, 0, 0, 0, 940, 943, 3, 148,
		74, 0, 941, 943, 3, 384, 192, 0, 942, 940, 1, 0, 0, 0, 942, 941, 1, 0,
		0, 0, 943, 147, 1, 0, 0, 0, 944, 945, 3, 384, 192, 0, 945, 946, 5, 16,
		0, 0, 946, 947, 3, 206, 103, 0, 947, 149, 1, 0, 0, 0, 948, 951, 3, 152,
		76, 0, 949, 951, 3, 154, 77, 0, 950, 948, 1, 0, 0, 0, 950, 949, 1, 0,
		0, 0, 951, 151, 1, 0, 0, 0, 952, 953, 3, 332, 166, 0, 953, 954, 3, 136,
		68, 0, 954, 153, 1, 0, 0, 0, 955, 956, 3, 186, 93, 0, 956, 957, 3, 366,
		183, 0, 957, 958, 3, 186, 93, 0, 958, 155, 1, 0, 0, 0, 959, 963, 3, 158,
		79, 0, 960, 963, 3, 160, 80, 0, 961, 963, 3, 166, 83, 0, 962, 959, 1,
		0, 0, 0, 962, 960, 1, 0, 0, 0, 962, 961, 1, 0, 0, 0, 963, 157, 1, 0, 0,
		0, 964, 971, 3, 368, 184, 0, 965, 966, 3, 370, 185, 0, 966, 967, 5, 12,
		0, 0, 967, 968, 3, 190, 95, 0, 968, 969, 5, 13, 0, 0, 969, 971, 1, 0,
		0, 0, 970, 964, 1, 0, 0, 0, 970, 965, 1, 0, 0, 0, 971, 159, 1, 0, 0, 0,
		972, 973, 3, 162, 81, 0, 973, 161, 1, 0, 0, 0, 974, 983, 3, 372, 186,
		0, 975, 976, 3, 374, 187, 0, 976, 977, 5, 12, 0, 0, 977, 978, 3, 190,
		95, 0, 978, 979, 5, 13, 0, 0, 979, 983, 1, 0, 0, 0, 980, 983, 3, 182,
		91, 0, 981, 983, 3, 168, 84, 0, 982, 974, 1, 0, 0, 0, 982, 975, 1, 0,
		0, 0, 982, 980, 1, 0, 0, 0, 982, 981, 1, 0, 0, 0, 983, 163, 1, 0, 0, 0,
		984, 985, 3, 186, 93, 0, 985, 986, 3, 362, 181, 0, 986, 987, 3, 186, 93,
		0, 987, 165, 1, 0, 0, 0, 988, 995, 3, 376, 188, 0, 989, 990, 3, 378, 189,
		0, 990, 991, 5, 12, 0, 0, 991, 992, 3, 190, 95, 0, 992, 993, 5, 13, 0,
		0, 993, 995, 1, 0, 0, 0, 994, 988, 1, 0, 0, 0, 994, 989, 1, 0, 0, 0, 995,
		167, 1, 0, 0, 0, 996, 997, 5, 17, 0, 0, 997, 998, 3, 170, 85, 0, 998,
		999, 5, 2, 0, 0, 999, 1000, 3, 174, 87, 0, 1000, 1001, 5, 2, 0, 0, 1001,
		1002, 3, 184, 92, 0, 1002, 1003, 5, 13, 0, 0, 1003, 169, 1, 0, 0, 0, 1004,
		1010, 3, 194, 97, 0, 1005, 1006, 5, 14, 0, 0, 1006, 1007, 3, 172, 86,
		0, 1007, 1008, 5, 15, 0, 0, 1008, 1010, 1, 0, 0, 0, 1009, 1004, 1, 0,
		0, 0, 1009, 1005, 1, 0, 0, 0, 1010, 171, 1, 0, 0, 0, 1011, 1017, 3, 194,
		97, 0, 1012, 1013, 3, 194, 97, 0, 1013, 1014, 5, 2, 0, 0, 1014, 1015,
		3, 172, 86, 0, 1015, 1017, 1, 0, 0, 0, 1016, 1011, 1, 0, 0, 0, 1016, 1012,
		1, 0, 0, 0, 1017, 173, 1, 0, 0, 0, 1018, 1024, 3, 176, 88, 0, 1019, 1020,
		5, 14, 0, 0, 1020, 1021, 3, 180, 90, 0, 1021, 1022, 5, 15, 0, 0, 1022,
		1024, 1, 0, 0, 0, 1023, 1018, 1, 0, 0, 0, 1023, 1019, 1, 0, 0, 0, 1024,
		175, 1, 0, 0, 0, 1025, 1026, 3, 178, 89, 0, 1026, 1027, 3, 346, 173, 0,
		1027, 1028, 3, 184, 92, 0, 1028, 177, 1, 0, 0, 0, 1029, 1032, 3, 158,
		79, 0, 1030, 1032, 3, 188, 94, 0, 1031, 1029, 1, 0, 0, 0, 1031, 1030,
		1, 0, 0, 0, 1032, 179, 1, 0, 0, 0, 1033, 1039, 3, 176, 88, 0, 1034, 1035,
		3, 176, 88, 0, 1035, 1036, 5, 2, 0, 0, 1036, 1037, 3, 180, 90, 0, 1037,
		1039, 1, 0, 0, 0, 1038, 1033, 1, 0, 0, 0, 1038, 1034, 1, 0, 0, 0, 1039,
		181, 1, 0, 0, 0, 1040, 1041, 3, 232, 116, 0, 1041, 1042, 5, 11, 0, 0,
		1042, 1043, 5, 12, 0, 0, 1043, 1044, 3, 190, 95, 0, 1044, 1045, 5, 13,
		0, 0, 1045, 183, 1, 0, 0, 0, 1046, 1050, 3, 122, 61, 0, 1047, 1050, 3,
		382, 191, 0, 1048, 1050, 3, 188, 94, 0, 1049, 1046, 1, 0, 0, 0, 1049,
		1047, 1, 0, 0, 0, 1049, 1048, 1, 0, 0, 0, 1050, 185, 1, 0, 0, 0, 1051,
		1060, 3, 156, 78, 0, 1052, 1060, 3, 382, 191, 0, 1053, 1060, 3, 188, 94,
		0, 1054, 1060, 3, 384, 192, 0, 1055, 1056, 5, 12, 0, 0, 1056, 1057, 3,
		122, 61, 0, 1057, 1058, 5, 13, 0, 0, 1058, 1060, 1, 0, 0, 0, 1059, 1051,
		1, 0, 0, 0, 1059, 1052, 1, 0, 0, 0, 1059, 1053, 1, 0, 0, 0, 1059, 1054,
		1, 0, 0, 0, 1059, 1055, 1, 0, 0, 0, 1060, 187, 1, 0, 0, 0, 1061, 1067,
		5, 18, 0, 0, 1062, 1063, 5, 14, 0, 0, 1063, 1064, 3, 190, 95, 0, 1064,
		1065, 5, 15, 0, 0, 1065, 1067, 1, 0, 0, 0, 1066, 1061, 1, 0, 0, 0, 1066,
		1062, 1, 0, 0, 0, 1067, 189, 1, 0, 0, 0, 1068, 1072, 3, 184, 92, 0, 1069,
		1071, 3, 192, 96, 0, 1070, 1069, 1, 0, 0, 0, 1071, 1074, 1, 0, 0, 0, 1072,
		1070, 1, 0, 0, 0, 1072, 1073, 1, 0, 0, 0, 1073, 191, 1, 0, 0, 0, 1074,
		1072, 1, 0, 0, 0, 1075, 1076, 5, 2, 0, 0, 1076, 1077, 3, 184, 92, 0, 1077,
		193, 1, 0, 0, 0, 1078, 1079, 3, 350, 175, 0, 1079, 1080, 5, 16, 0, 0,
		1080, 1081, 3, 196, 98, 0, 1081, 1087, 1, 0, 0, 0, 1082, 1083, 5, 12,
		0, 0, 1083, 1084, 3, 194, 97, 0, 1084, 1085, 5, 13, 0, 0, 1085, 1087,
		1, 0, 0, 0, 1086, 1078, 1, 0, 0, 0, 1086, 1082, 1, 0, 0, 0, 1087, 195,
		1, 0, 0, 0, 1088, 1091, 3, 206, 103, 0, 1089, 1091, 3, 198, 99, 0, 1090,
		1088, 1, 0, 0, 0, 1090, 1089, 1, 0, 0, 0, 1091, 197, 1, 0, 0, 0, 1092,
		1099, 3, 210, 105, 0, 1093, 1099, 3, 200, 100, 0, 1094, 1095, 5, 12, 0,
		0, 1095, 1096, 3, 198, 99, 0, 1096, 1097, 5, 13, 0, 0, 1097, 1099, 1,
		0, 0, 0, 1098, 1092, 1, 0, 0, 0, 1098, 1093, 1, 0, 0, 0, 1098, 1094, 1,
		0, 0, 0, 1099, 199, 1, 0, 0, 0, 1100, 1101, 3, 328, 164, 0, 1101, 1102,
		5, 14, 0, 0, 1102, 1103, 3, 144, 72, 0, 1103, 1104, 5, 15, 0, 0, 1104,
		1105, 5, 16, 0, 0, 1105, 1106, 3, 202, 101, 0, 1106, 201, 1, 0, 0, 0,
		1107, 1114, 3, 206, 103, 0, 1108, 1109, 5, 12, 0, 0, 1109, 1110, 3, 210,
		105, 0, 1110, 1111, 5, 13, 0, 0, 1111, 1114, 1, 0, 0, 0, 1112, 1114, 3,
		200, 100, 0, 1113, 1107, 1, 0, 0, 0, 1113, 1108, 1, 0, 0, 0, 1113, 1112,
		1, 0, 0, 0, 1114, 203, 1, 0, 0, 0, 1115, 1121, 3, 206, 103, 0, 1116, 1117,
		5, 12, 0, 0, 1117, 1118, 3, 212, 106, 0, 1118, 1119, 5, 13, 0, 0, 1119,
		1121, 1, 0, 0, 0, 1120, 1115, 1, 0, 0, 0, 1120, 1116, 1, 0, 0, 0, 1121,
		205, 1, 0, 0, 0, 1122, 1136, 3, 354, 177, 0, 1123, 1136, 3, 358, 179,
		0, 1124, 1136, 3, 384, 192, 0, 1125, 1126, 3, 356, 178, 0, 1126, 1127,
		5, 12, 0, 0, 1127, 1128, 3, 208, 104, 0, 1128, 1129, 5, 13, 0, 0, 1129,
		1136, 1, 0, 0, 0, 1130, 1131, 5, 12, 0, 0, 1131, 1132, 3, 206, 103, 0,
		1132, 1133, 5, 13, 0, 0, 1133, 1136, 1, 0, 0, 0, 1134, 1136, 3, 214, 107,
		0, 1135, 1122, 1, 0, 0, 0, 1135, 1123, 1, 0, 0, 0, 1135, 1124, 1, 0, 0,
		0, 1135, 1125, 1, 0, 0, 0, 1135, 1130, 1, 0, 0, 0, 1135, 1134, 1, 0, 0,
		0, 1136, 207, 1, 0, 0, 0, 1137, 1143, 3, 206, 103, 0, 1138, 1139, 3, 206,
		103, 0, 1139, 1140, 5, 2, 0, 0, 1140, 1141, 3, 208, 104, 0, 1141, 1143,
		1, 0, 0, 0, 1142, 1137, 1, 0, 0, 0, 1142, 1138, 1, 0, 0, 0, 1143, 209,
		1, 0, 0, 0, 1144, 1145, 3, 204, 102, 0, 1145, 1146, 5, 88, 0, 0, 1146,
		1147, 3, 206, 103, 0, 1147, 211, 1, 0, 0, 0, 1148, 1149, 6, 106, -1, 0,
		1149, 1150, 3, 204, 102, 0, 1150, 1151, 5, 86, 0, 0, 1151, 1152, 3, 206,
		103, 0, 1152, 1158, 1, 0, 0, 0, 1153, 1154, 10, 1, 0, 0, 1154, 1155, 5,
		86, 0, 0, 1155, 1157, 3, 206, 103, 0, 1156, 1153, 1, 0, 0, 0, 1157, 1160,
		1, 0, 0, 0, 1158, 1156, 1, 0, 0, 0, 1158, 1159, 1, 0, 0, 0, 1159, 213,
		1, 0, 0, 0, 1160, 1158, 1, 0, 0, 0, 1161, 1162, 5, 14, 0, 0, 1162, 1163,
		3, 216, 108, 0, 1163, 1164, 5, 15, 0, 0, 1164, 215, 1, 0, 0, 0, 1165,
		1171, 3, 196, 98, 0, 1166, 1167, 3, 196, 98, 0, 1167, 1168, 5, 2, 0, 0,
		1168, 1169, 3, 216, 108, 0, 1169, 1171, 1, 0, 0, 0, 1170, 1165, 1, 0,
		0, 0, 1170, 1166, 1, 0, 0, 0, 1171, 217, 1, 0, 0, 0, 1172, 1173, 3, 352,
		176, 0, 1173, 1174, 3, 330, 165, 0, 1174, 1175, 3, 352, 176, 0, 1175,
		219, 1, 0, 0, 0, 1176, 1177, 3, 156, 78, 0, 1177, 1178, 3, 348, 174, 0,
		1178, 1179, 3, 184, 92, 0, 1179, 221, 1, 0, 0, 0, 1180, 1181, 3, 188,
		94, 0, 1181, 1182, 3, 344, 172, 0, 1182, 1183, 3, 188, 94, 0, 1183, 223,
		1, 0, 0, 0, 1184, 1185, 5, 19, 0, 0, 1185, 1186, 3, 240, 120, 0, 1186,
		1187, 5, 20, 0, 0, 1187, 1195, 1, 0, 0, 0, 1188, 1189, 5, 19, 0, 0, 1189,
		1190, 3, 240, 120, 0, 1190, 1191, 5, 12, 0, 0, 1191, 1192, 3, 226, 113,
		0, 1192, 1193, 5, 21, 0, 0, 1193, 1195, 1, 0, 0, 0, 1194, 1184, 1, 0,
		0, 0, 1194, 1188, 1, 0, 0, 0, 1195, 225, 1, 0, 0, 0, 1196, 1202, 3, 228,
		114, 0, 1197, 1198, 3, 228, 114, 0, 1198, 1199, 5, 2, 0, 0, 1199, 1200,
		3, 226, 113, 0, 1200, 1202, 1, 0, 0, 0, 1201, 1196, 1, 0, 0, 0, 1201,
		1197, 1, 0, 0, 0, 1202, 227, 1, 0, 0, 0, 1203, 1206, 3, 244, 122, 0, 1204,
		1206, 3, 230, 115, 0, 1205, 1203, 1, 0, 0, 0, 1205, 1204, 1, 0, 0, 0,
		1206, 229, 1, 0, 0, 0, 1207, 1208, 3, 116, 58, 0, 1208, 231, 1, 0, 0,
		0, 1209, 1210, 5, 19, 0, 0, 1210, 1211, 3, 240, 120, 0, 1211, 1212, 5,
		20, 0, 0, 1212, 1220, 1, 0, 0, 0, 1213, 1214, 5, 19, 0, 0, 1214, 1215,
		3, 240, 120, 0, 1215, 1216, 5, 12, 0, 0, 1216, 1217, 3, 234, 117, 0, 1217,
		1218, 5, 21, 0, 0, 1218, 1220, 1, 0, 0, 0, 1219, 1209, 1, 0, 0, 0, 1219,
		1213, 1, 0, 0, 0, 1220, 233, 1, 0, 0, 0, 1221, 1227, 3, 236, 118, 0, 1222,
		1223, 3, 236, 118, 0, 1223, 1224, 5, 2, 0, 0, 1224, 1225, 3, 234, 117,
		0, 1225, 1227, 1, 0, 0, 0, 1226, 1221, 1, 0, 0, 0, 1226, 1222, 1, 0, 0,
		0, 1227, 235, 1, 0, 0, 0, 1228, 1231, 3, 244, 122, 0, 1229, 1231, 3, 238,
		119, 0, 1230, 1228, 1, 0, 0, 0, 1230, 1229, 1, 0, 0, 0, 1231, 237, 1,
		0, 0, 0, 1232, 1233, 3, 220, 110, 0, 1233, 239, 1, 0, 0, 0, 1234, 1237,
		3, 242, 121, 0, 1235, 1237, 3, 450, 225, 0, 1236, 1234, 1, 0, 0, 0, 1236,
		1235, 1, 0, 0, 0, 1237, 241, 1, 0, 0, 0, 1238, 1239, 3, 448, 224, 0, 1239,
		243, 1, 0, 0, 0, 1240, 1241, 5, 90, 0, 0, 1241, 1242, 3, 186, 93, 0, 1242,
		245, 1, 0, 0, 0, 1243, 1250, 5, 22, 0, 0, 1244, 1245, 5, 89, 0, 0, 1245,
		1246, 5, 23, 0, 0, 1246, 1250, 5, 88, 0, 0, 1247, 1250, 5, 24, 0, 0, 1248,
		1250, 5, 25, 0, 0, 1249, 1243, 1, 0, 0, 0, 1249, 1244, 1, 0, 0, 0, 1249,
		1247, 1, 0, 0, 0, 1249, 1248, 1, 0, 0, 0, 1250, 247, 1, 0, 0, 0, 1251,
		1254, 3, 250, 125, 0, 1252, 1254, 3, 194, 97, 0, 1253, 1251, 1, 0, 0,
		0, 1253, 1252, 1, 0, 0, 0, 1254, 249, 1, 0, 0, 0, 1255, 1258, 3, 252,
		126, 0, 1256, 1258, 3, 316, 158, 0, 1257, 1255, 1, 0, 0, 0, 1257, 1256,
		1, 0, 0, 0, 1258, 251, 1, 0, 0, 0, 1259, 1260, 5, 26, 0, 0, 1260, 1261,
		5, 14, 0, 0, 1261, 1262, 3, 144, 72, 0, 1262, 1263, 5, 15, 0, 0, 1263,
		1264, 5, 16, 0, 0, 1264, 1265, 3, 250, 125, 0, 1265, 253, 1, 0, 0, 0,
		1266, 1269, 3, 256, 128, 0, 1267, 1269, 3, 308, 154, 0, 1268, 1266, 1,
		0, 0, 0, 1268, 1267, 1, 0, 0, 0, 1269, 255, 1, 0, 0, 0, 1270, 1274, 3,
		258, 129, 0, 1271, 1274, 3, 268, 134, 0, 1272, 1274, 3, 274, 137, 0, 1273,
		1270, 1, 0, 0, 0, 1273, 1271, 1, 0, 0, 0, 1273, 1272, 1, 0, 0, 0, 1274,
		257, 1, 0, 0, 0, 1275, 1278, 3, 260, 130, 0, 1276, 1278, 3, 262, 131,
		0, 1277, 1275, 1, 0, 0, 0, 1277, 1276, 1, 0, 0, 0, 1278, 259, 1, 0, 0,
		0, 1279, 1280, 3, 272, 136, 0, 1280, 1281, 3, 338, 169, 0, 1281, 1282,
		3, 272, 136, 0, 1282, 261, 1, 0, 0, 0, 1283, 1286, 3, 264, 132, 0, 1284,
		1286, 3, 266, 133, 0, 1285, 1283, 1, 0, 0, 0, 1285, 1284, 1, 0, 0, 0,
		1286, 263, 1, 0, 0, 0, 1287, 1288, 6, 132, -1, 0, 1288, 1289, 3, 272,
		136, 0, 1289, 1290, 5, 85, 0, 0, 1290, 1291, 3, 272, 136, 0, 1291, 1297,
		1, 0, 0, 0, 1292, 1293, 10, 1, 0, 0, 1293, 1294, 5, 85, 0, 0, 1294, 1296,
		3, 272, 136, 0, 1295, 1292, 1, 0, 0, 0, 1296, 1299, 1, 0, 0, 0, 1297,
		1295, 1, 0, 0, 0, 1297, 1298, 1, 0, 0, 0, 1298, 265, 1, 0, 0, 0, 1299,
		1297, 1, 0, 0, 0, 1300, 1301, 6, 133, -1, 0, 1301, 1302, 3, 272, 136,
		0, 1302, 1303, 5, 10, 0, 0, 1303, 1304, 3, 272, 136, 0, 1304, 1310, 1,
		0, 0, 0, 1305, 1306, 10, 1, 0, 0, 1306, 1307, 5, 10, 0, 0, 1307, 1309,
		3, 272, 136, 0, 1308, 1305, 1, 0, 0, 0, 1309, 1312, 1, 0, 0, 0, 1310,
		1308, 1, 0, 0, 0, 1310, 1311, 1, 0, 0, 0, 1311, 267, 1, 0, 0, 0, 1312,
		1310, 1, 0, 0, 0, 1313, 1314, 3, 342, 171, 0, 1314, 1315, 3, 272, 136,
		0, 1315, 1318, 1, 0, 0, 0, 1316, 1318, 3, 270, 135, 0, 1317, 1313, 1,
		0, 0, 0, 1317, 1316, 1, 0, 0, 0, 1318, 269, 1, 0, 0, 0, 1319, 1320, 3,
		304, 152, 0, 1320, 1321, 3, 366, 183, 0, 1321, 1322, 3, 304, 152, 0, 1322,
		271, 1, 0, 0, 0, 1323, 1326, 3, 274, 137, 0, 1324, 1326, 3, 268, 134,
		0, 1325, 1323, 1, 0, 0, 0, 1325, 1324, 1, 0, 0, 0, 1326, 273, 1, 0, 0,
		0, 1327, 1334, 3, 276, 138, 0, 1328, 1334, 3, 280, 140, 0, 1329, 1330,
		5, 12, 0, 0, 1330, 1331, 3, 256, 128, 0, 1331, 1332, 5, 13, 0, 0, 1332,
		1334, 1, 0, 0, 0, 1333, 1327, 1, 0, 0, 0, 1333, 1328, 1, 0, 0, 0, 1333,
		1329, 1, 0, 0, 0, 1334, 275, 1, 0, 0, 0, 1335, 1336, 3, 336, 168, 0, 1336,
		1337, 5, 14, 0, 0, 1337, 1338, 3, 278, 139, 0, 1338, 1339, 5, 15, 0, 0,
		1339, 1340, 5, 16, 0, 0, 1340, 1341, 3, 272, 136, 0, 1341, 277, 1, 0,
		0, 0, 1342, 1348, 3, 384, 192, 0, 1343, 1344, 3, 384, 192, 0, 1344, 1345,
		5, 2, 0, 0, 1345, 1346, 3, 278, 139, 0, 1346, 1348, 1, 0, 0, 0, 1347,
		1342, 1, 0, 0, 0, 1347, 1343, 1, 0, 0, 0, 1348, 279, 1, 0, 0, 0, 1349,
		1353, 3, 282, 141, 0, 1350, 1353, 3, 284, 142, 0, 1351, 1353, 3, 290,
		145, 0, 1352, 1349, 1, 0, 0, 0, 1352, 1350, 1, 0, 0, 0, 1352, 1351, 1,
		0, 0, 0, 1353, 281, 1, 0, 0, 0, 1354, 1355, 3, 292, 146, 0, 1355, 283,
		1, 0, 0, 0, 1356, 1359, 3, 286, 143, 0, 1357, 1359, 3, 288, 144, 0, 1358,
		1356, 1, 0, 0, 0, 1358, 1357, 1, 0, 0, 0, 1359, 285, 1, 0, 0, 0, 1360,
		1361, 3, 298, 149, 0, 1361, 287, 1, 0, 0, 0, 1362, 1363, 3, 304, 152,
		0, 1363, 1364, 3, 362, 181, 0, 1364, 1365, 3, 304, 152, 0, 1365, 289,
		1, 0, 0, 0, 1366, 1367, 3, 300, 150, 0, 1367, 291, 1, 0, 0, 0, 1368, 1375,
		3, 368, 184, 0, 1369, 1370, 3, 370, 185, 0, 1370, 1371, 5, 12, 0, 0, 1371,
		1372, 3, 302, 151, 0, 1372, 1373, 5, 13, 0, 0, 1373, 1375, 1, 0, 0, 0,
		1374, 1368, 1, 0, 0, 0, 1374, 1369, 1, 0, 0, 0, 1375, 293, 1, 0, 0, 0,
		1376, 1379, 3, 382, 191, 0, 1377, 1379, 3, 296, 148, 0, 1378, 1376, 1,
		0, 0, 0, 1378, 1377, 1, 0, 0, 0, 1379, 295, 1, 0, 0, 0, 1380, 1381, 3,
		298, 149, 0, 1381, 297, 1, 0, 0, 0, 1382, 1389, 3, 372, 186, 0, 1383,
		1384, 3, 374, 187, 0, 1384, 1385, 5, 12, 0, 0, 1385, 1386, 3, 302, 151,
		0, 1386, 1387, 5, 13, 0, 0, 1387, 1389, 1, 0, 0, 0, 1388, 1382, 1, 0,
		0, 0, 1388, 1383, 1, 0, 0, 0, 1389, 299, 1, 0, 0, 0, 1390, 1397, 3, 376,
		188, 0, 1391, 1392, 3, 378, 189, 0, 1392, 1393, 5, 12, 0, 0, 1393, 1394,
		3, 302, 151, 0, 1394, 1395, 5, 13, 0, 0, 1395, 1397, 1, 0, 0, 0, 1396,
		1390, 1, 0, 0, 0, 1396, 1391, 1, 0, 0, 0, 1397, 301, 1, 0, 0, 0, 1398,
		1404, 3, 304, 152, 0, 1399, 1400, 3, 304, 152, 0, 1400, 1401, 5, 2, 0,
		0, 1401, 1402, 3, 302, 151, 0, 1402, 1404, 1, 0, 0, 0, 1403, 1398, 1,
		0, 0, 0, 1403, 1399, 1, 0, 0, 0, 1404, 303, 1, 0, 0, 0, 1405, 1408, 3,
		306, 153, 0, 1406, 1408, 3, 384, 192, 0, 1407, 1405, 1, 0, 0, 0, 1407,
		1406, 1, 0, 0, 0, 1408, 305, 1, 0, 0, 0, 1409, 1413, 3, 292, 146, 0, 1410,
		1413, 3, 294, 147, 0, 1411, 1413, 3, 300, 150, 0, 1412, 1409, 1, 0, 0,
		0, 1412, 1410, 1, 0, 0, 0, 1412, 1411, 1, 0, 0, 0, 1413, 307, 1, 0, 0,
		0, 1414, 1415, 3, 310, 155, 0, 1415, 1416, 3, 344, 172, 0, 1416, 1417,
		3, 310, 155, 0, 1417, 1423, 1, 0, 0, 0, 1418, 1419, 5, 12, 0, 0, 1419,
		1420, 3, 308, 154, 0, 1420, 1421, 5, 13, 0, 0, 1421, 1423, 1, 0, 0, 0,
		1422, 1414, 1, 0, 0, 0, 1422, 1418, 1, 0, 0, 0, 1423, 309, 1, 0, 0, 0,
		1424, 1430, 5, 18, 0, 0, 1425, 1426, 5, 14, 0, 0, 1426, 1427, 3, 312,
		156, 0, 1427, 1428, 5, 15, 0, 0, 1428, 1430, 1, 0, 0, 0, 1429, 1424, 1,
		0, 0, 0, 1429, 1425, 1, 0, 0, 0, 1430, 311, 1, 0, 0, 0, 1431, 1435, 3,
		256, 128, 0, 1432, 1434, 3, 314, 157, 0, 1433, 1432, 1, 0, 0, 0, 1434,
		1437, 1, 0, 0, 0, 1435, 1433, 1, 0, 0, 0, 1435, 1436, 1, 0, 0, 0, 1436,
		313, 1, 0, 0, 0, 1437, 1435, 1, 0, 0, 0, 1438, 1439, 5, 2, 0, 0, 1439,
		1440, 3, 256, 128, 0, 1440, 315, 1, 0, 0, 0, 1441, 1447, 3, 318, 159,
		0, 1442, 1443, 5, 12, 0, 0, 1443, 1444, 3, 316, 158, 0, 1444, 1445, 5,
		13, 0, 0, 1445, 1447, 1, 0, 0, 0, 1446, 1441, 1, 0, 0, 0, 1446, 1442,
		1, 0, 0, 0, 1447, 317, 1, 0, 0, 0, 1448, 1449, 6, 159, -1, 0, 1449, 1450,
		3, 320, 160, 0, 1450, 1456, 1, 0, 0, 0, 1451, 1452, 10, 1, 0, 0, 1452,
		1453, 5, 85, 0, 0, 1453, 1455, 3, 320, 160, 0, 1454, 1451, 1, 0, 0, 0,
		1455, 1458, 1, 0, 0, 0, 1456, 1454, 1, 0, 0, 0, 1456, 1457, 1, 0, 0, 0,
		1457, 319, 1, 0, 0, 0, 1458, 1456, 1, 0, 0, 0, 1459, 1469, 3, 280, 140,
		0, 1460, 1461, 5, 27, 0, 0, 1461, 1469, 3, 280, 140, 0, 1462, 1463, 5,
		27, 0, 0, 1463, 1464, 5, 12, 0, 0, 1464, 1465, 3, 280, 140, 0, 1465, 1466,
		5, 13, 0, 0, 1466, 1469, 1, 0, 0, 0, 1467, 1469, 3, 270, 135, 0, 1468,
		1459, 1, 0, 0, 0, 1468, 1460, 1, 0, 0, 0, 1468, 1462, 1, 0, 0, 0, 1468,
		1467, 1, 0, 0, 0, 1469, 321, 1, 0, 0, 0, 1470, 1474, 3, 334, 167, 0, 1471,
		1474, 3, 326, 163, 0, 1472, 1474, 3, 328, 164, 0, 1473, 1470, 1, 0, 0,
		0, 1473, 1471, 1, 0, 0, 0, 1473, 1472, 1, 0, 0, 0, 1474, 323, 1, 0, 0,
		0, 1475, 1478, 3, 342, 171, 0, 1476, 1478, 3, 246, 123, 0, 1477, 1475,
		1, 0, 0, 0, 1477, 1476, 1, 0, 0, 0, 1478, 325, 1, 0, 0, 0, 1479, 1480,
		7, 0, 0, 0, 1480, 327, 1, 0, 0, 0, 1481, 1489, 5, 31, 0, 0, 1482, 1484,
		5, 32, 0, 0, 1483, 1482, 1, 0, 0, 0, 1484, 1487, 1, 0, 0, 0, 1485, 1483,
		1, 0, 0, 0, 1485, 1486, 1, 0, 0, 0, 1486, 1489, 1, 0, 0, 0, 1487, 1485,
		1, 0, 0, 0, 1488, 1481, 1, 0, 0, 0, 1488, 1485, 1, 0, 0, 0, 1489, 329,
		1, 0, 0, 0, 1490, 1491, 5, 33, 0, 0, 1491, 331, 1, 0, 0, 0, 1492, 1495,
		3, 342, 171, 0, 1493, 1495, 3, 246, 123, 0, 1494, 1492, 1, 0, 0, 0, 1494,
		1493, 1, 0, 0, 0, 1495, 333, 1, 0, 0, 0, 1496, 1499, 3, 336, 168, 0, 1497,
		1499, 5, 90, 0, 0, 1498, 1496, 1, 0, 0, 0, 1498, 1497, 1, 0, 0, 0, 1499,
		335, 1, 0, 0, 0, 1500, 1501, 7, 1, 0, 0, 1501, 337, 1, 0, 0, 0, 1502,
		1510, 5, 34, 0, 0, 1503, 1510, 5, 35, 0, 0, 1504, 1510, 5, 36, 0, 0, 1505,
		1510, 5, 37, 0, 0, 1506, 1507, 5, 27, 0, 0, 1507, 1510, 5, 85, 0, 0, 1508,
		1510, 5, 38, 0, 0, 1509, 1502, 1, 0, 0, 0, 1509, 1503, 1, 0, 0, 0, 1509,
		1504, 1, 0, 0, 0, 1509, 1505, 1, 0, 0, 0, 1509, 1506, 1, 0, 0, 0, 1509,
		1508, 1, 0, 0, 0, 1510, 339, 1, 0, 0, 0, 1511, 1512, 7, 2, 0, 0, 1512,
		341, 1, 0, 0, 0, 1513, 1514, 5, 27, 0, 0, 1514, 343, 1, 0, 0, 0, 1515,
		1516, 5, 39, 0, 0, 1516, 345, 1, 0, 0, 0, 1517, 1518, 5, 40, 0, 0, 1518,
		347, 1, 0, 0, 0, 1519, 1520, 5, 41, 0, 0, 1520, 349, 1, 0, 0, 0, 1521,
		1524, 3, 368, 184, 0, 1522, 1524, 5, 63, 0, 0, 1523, 1521, 1, 0, 0, 0,
		1523, 1522, 1, 0, 0, 0, 1524, 351, 1, 0, 0, 0, 1525, 1529, 3, 350, 175,
		0, 1526, 1529, 3, 372, 186, 0, 1527, 1529, 3, 360, 180, 0, 1528, 1525,
		1, 0, 0, 0, 1528, 1526, 1, 0, 0, 0, 1528, 1527, 1, 0, 0, 0, 1529, 353,
		1, 0, 0, 0, 1530, 1531, 3, 356, 178, 0, 1531, 355, 1, 0, 0, 0, 1532, 1533,
		3, 446, 223, 0, 1533, 357, 1, 0, 0, 0, 1534, 1535, 3, 448, 224, 0, 1535,
		359, 1, 0, 0, 0, 1536, 1537, 3, 450, 225, 0, 1537, 361, 1, 0, 0, 0, 1538,
		1539, 3, 364, 182, 0, 1539, 363, 1, 0, 0, 0, 1540, 1541, 5, 42, 0, 0,
		1541, 365, 1, 0, 0, 0, 1542, 1543, 5, 43, 0, 0, 1543, 367, 1, 0, 0, 0,
		1544, 1545, 3, 370, 185, 0, 1545, 369, 1, 0, 0, 0, 1546, 1547, 3, 446,
		223, 0, 1547, 371, 1, 0, 0, 0, 1548, 1549, 3, 374, 187, 0, 1549, 373,
		1, 0, 0, 0, 1550, 1551, 3, 448, 224, 0, 1551, 375, 1, 0, 0, 0, 1552, 1553,
		3, 378, 189, 0, 1553, 377, 1, 0, 0, 0, 1554, 1555, 3, 450, 225, 0, 1555,
		379, 1, 0, 0, 0, 1556, 1557, 7, 3, 0, 0, 1557, 381, 1, 0, 0, 0, 1558,
		1561, 3, 452, 226, 0, 1559, 1561, 5, 63, 0, 0, 1560, 1558, 1, 0, 0, 0,
		1560, 1559, 1, 0, 0, 0, 1561, 383, 1, 0, 0, 0, 1562, 1563, 5, 66, 0, 0,
		1563, 385, 1, 0, 0, 0, 1564, 1572, 3, 390, 195, 0, 1565, 1572, 3, 396,
		198, 0, 1566, 1572, 3, 400, 200, 0, 1567, 1568, 5, 14, 0, 0, 1568, 1569,
		3, 388, 194, 0, 1569, 1570, 5, 15, 0, 0, 1570, 1572, 1, 0, 0, 0, 1571,
		1564, 1, 0, 0, 0, 1571, 1565, 1, 0, 0, 0, 1571, 1566, 1, 0, 0, 0, 1571,
		1567, 1, 0, 0, 0, 1572, 387, 1, 0, 0, 0, 1573, 1579, 3, 386, 193, 0, 1574,
		1575, 3, 386, 193, 0, 1575, 1576, 5, 2, 0, 0, 1576, 1577, 3, 388, 194,
		0, 1577, 1579, 1, 0, 0, 0, 1578, 1573, 1, 0, 0, 0, 1578, 1574, 1, 0, 0,
		0, 1579, 389, 1, 0, 0, 0, 1580, 1583, 3, 444, 222, 0, 1581, 1583, 3, 392,
		196, 0, 1582, 1580, 1, 0, 0, 0, 1582, 1581, 1, 0, 0, 0, 1583, 391, 1,
		0, 0, 0, 1584, 1585, 5, 49, 0, 0, 1585, 1586, 3, 394, 197, 0, 1586, 1587,
		5, 2, 0, 0, 1587, 1588, 3, 418, 209, 0, 1588, 1589, 5, 2, 0, 0, 1589,
		1590, 3, 406, 203, 0, 1590, 1591, 5, 13, 0, 0, 1591, 393, 1, 0, 0, 0,
		1592, 1593, 3, 446, 223, 0, 1593, 395, 1, 0, 0, 0, 1594, 1595, 5, 50,
		0, 0, 1595, 1596, 3, 398, 199, 0, 1596, 1597, 5, 2, 0, 0, 1597, 1598,
		3, 418, 209, 0, 1598, 1599, 5, 2, 0, 0, 1599, 1600, 3, 406, 203, 0, 1600,
		1601, 5, 13, 0, 0, 1601, 397, 1, 0, 0, 0, 1602, 1603, 3, 446, 223, 0,
		1603, 399, 1, 0, 0, 0, 1604, 1605, 3, 402, 201, 0, 1605, 401, 1, 0, 0,
		0, 1606, 1607, 5, 51, 0, 0, 1607, 1608, 3, 454, 227, 0, 1608, 1609, 3,
		404, 202, 0, 1609, 1610, 5, 13, 0, 0, 1610, 403, 1, 0, 0, 0, 1611, 1612,
		5, 2, 0, 0, 1612, 1615, 3, 444, 222, 0, 1613, 1615, 3, 456, 228, 0, 1614,
		1611, 1, 0, 0, 0, 1614, 1613, 1, 0, 0, 0, 1615, 405, 1, 0, 0, 0, 1616,
		1622, 5, 18, 0, 0, 1617, 1618, 5, 14, 0, 0, 1618, 1619, 3, 408, 204, 0,
		1619, 1620, 5, 15, 0, 0, 1620, 1622, 1, 0, 0, 0, 1621, 1616, 1, 0, 0,
		0, 1621, 1617, 1, 0, 0, 0, 1622, 407, 1, 0, 0, 0, 1623, 1627, 3, 412,
		206, 0, 1624, 1626, 3, 410, 205, 0, 1625, 1624, 1, 0, 0, 0, 1626, 1629,
		1, 0, 0, 0, 1627, 1625, 1, 0, 0, 0, 1627, 1628, 1, 0, 0, 0, 1628, 409,
		1, 0, 0, 0, 1629, 1627, 1, 0, 0, 0, 1630, 1631, 5, 2, 0, 0, 1631, 1632,
		3, 412, 206, 0, 1632, 411, 1, 0, 0, 0, 1633, 1634, 3, 386, 193, 0, 1634,
		1635, 3, 414, 207, 0, 1635, 413, 1, 0, 0, 0, 1636, 1637, 5, 16, 0, 0,
		1637, 1640, 3, 430, 215, 0, 1638, 1640, 3, 456, 228, 0, 1639, 1636, 1,
		0, 0, 0, 1639, 1638, 1, 0, 0, 0, 1640, 415, 1, 0, 0, 0, 1641, 1642, 5,
		2, 0, 0, 1642, 1645, 3, 418, 209, 0, 1643, 1645, 3, 456, 228, 0, 1644,
		1641, 1, 0, 0, 0, 1644, 1643, 1, 0, 0, 0, 1645, 417, 1, 0, 0, 0, 1646,
		1647, 3, 438, 219, 0, 1647, 419, 1, 0, 0, 0, 1648, 1649, 5, 52, 0, 0,
		1649, 1650, 3, 454, 227, 0, 1650, 1651, 3, 422, 211, 0, 1651, 1652, 5,
		3, 0, 0, 1652, 421, 1, 0, 0, 0, 1653, 1662, 3, 456, 228, 0, 1654, 1655,
		5, 2, 0, 0, 1655, 1662, 3, 424, 212, 0, 1656, 1657, 5, 2, 0, 0, 1657,
		1658, 3, 424, 212, 0, 1658, 1659, 5, 2, 0, 0, 1659, 1660, 3, 428, 214,
		0, 1660, 1662, 1, 0, 0, 0, 1661, 1653, 1, 0, 0, 0, 1661, 1654, 1, 0, 0,
		0, 1661, 1656, 1, 0, 0, 0, 1662, 423, 1, 0, 0, 0, 1663, 1664, 5, 14, 0,
		0, 1664, 1665, 3, 426, 213, 0, 1665, 1666, 5, 15, 0, 0, 1666, 1669, 1,
		0, 0, 0, 1667, 1669, 5, 86, 0, 0, 1668, 1663, 1, 0, 0, 0, 1668, 1667,
		1, 0, 0, 0, 1669, 425, 1, 0, 0, 0, 1670, 1676, 3, 444, 222, 0, 1671, 1672,
		3, 444, 222, 0, 1672, 1673, 5, 2, 0, 0, 1673, 1674, 3, 426, 213, 0, 1674,
		1676, 1, 0, 0, 0, 1675, 1670, 1, 0, 0, 0, 1675, 1671, 1, 0, 0, 0, 1676,
		427, 1, 0, 0, 0, 1677, 1678, 3, 444, 222, 0, 1678, 429, 1, 0, 0, 0, 1679,
		1686, 3, 432, 216, 0, 1680, 1681, 3, 432, 216, 0, 1681, 1682, 5, 16, 0,
		0, 1682, 1683, 3, 430, 215, 0, 1683, 1686, 1, 0, 0, 0, 1684, 1686, 3,
		438, 219, 0, 1685, 1679, 1, 0, 0, 0, 1685, 1680, 1, 0, 0, 0, 1685, 1684,
		1, 0, 0, 0, 1686, 431, 1, 0, 0, 0, 1687, 1694, 3, 446, 223, 0, 1688, 1694,
		3, 434, 217, 0, 1689, 1694, 3, 384, 192, 0, 1690, 1694, 3, 452, 226, 0,
		1691, 1694, 5, 63, 0, 0, 1692, 1694, 3, 436, 218, 0, 1693, 1687, 1, 0,
		0, 0, 1693, 1688, 1, 0, 0, 0, 1693, 1689, 1, 0, 0, 0, 1693, 1690, 1, 0,
		0, 0, 1693, 1691, 1, 0, 0, 0, 1693, 1692, 1, 0, 0, 0, 1694, 433, 1, 0,
		0, 0, 1695, 1696, 3, 446, 223, 0, 1696, 1697, 5, 12, 0, 0, 1697, 1698,
		3, 440, 220, 0, 1698, 1699, 5, 13, 0, 0, 1699, 435, 1, 0, 0, 0, 1700,
		1701, 5, 53, 0, 0, 1701, 1702, 3, 24, 12, 0, 1702, 1703, 5, 13, 0, 0,
		1703, 1721, 1, 0, 0, 0, 1704, 1705, 5, 54, 0, 0, 1705, 1706, 3, 120, 60,
		0, 1706, 1707, 5, 13, 0, 0, 1707, 1721, 1, 0, 0, 0, 1708, 1709, 5, 55,
		0, 0, 1709, 1710, 3, 254, 127, 0, 1710, 1711, 5, 13, 0, 0, 1711, 1721,
		1, 0, 0, 0, 1712, 1713, 5, 56, 0, 0, 1713, 1714, 3, 316, 158, 0, 1714,
		1715, 5, 13, 0, 0, 1715, 1721, 1, 0, 0, 0, 1716, 1717, 5, 57, 0, 0, 1717,
		1718, 3, 304, 152, 0, 1718, 1719, 5, 13, 0, 0, 1719, 1721, 1, 0, 0, 0,
		1720, 1700, 1, 0, 0, 0, 1720, 1704, 1, 0, 0, 0, 1720, 1708, 1, 0, 0, 0,
		1720, 1712, 1, 0, 0, 0, 1720, 1716, 1, 0, 0, 0, 1721, 437, 1, 0, 0, 0,
		1722, 1728, 5, 18, 0, 0, 1723, 1724, 5, 14, 0, 0, 1724, 1725, 3, 440,
		220, 0, 1725, 1726, 5, 15, 0, 0, 1726, 1728, 1, 0, 0, 0, 1727, 1722, 1,
		0, 0, 0, 1727, 1723, 1, 0, 0, 0, 1728, 439, 1, 0, 0, 0, 1729, 1733, 3,
		430, 215, 0, 1730, 1732, 3, 442, 221, 0, 1731, 1730, 1, 0, 0, 0, 1732,
		1735, 1, 0, 0, 0, 1733, 1731, 1, 0, 0, 0, 1733, 1734, 1, 0, 0, 0, 1734,
		441, 1, 0, 0, 0, 1735, 1733, 1, 0, 0, 0, 1736, 1737, 5, 2, 0, 0, 1737,
		1738, 3, 430, 215, 0, 1738, 443, 1, 0, 0, 0, 1739, 1742, 3, 446, 223,
		0, 1740, 1742, 5, 78, 0, 0, 1741, 1739, 1, 0, 0, 0, 1741, 1740, 1, 0,
		0, 0, 1742, 445, 1, 0, 0, 0, 1743, 1744, 7, 4, 0, 0, 1744, 447, 1, 0,
		0, 0, 1745, 1746, 5, 64, 0, 0, 1746, 449, 1, 0, 0, 0, 1747, 1748, 5, 65,
		0, 0, 1748, 451, 1, 0, 0, 0, 1749, 1750, 7, 5, 0, 0, 1750, 453, 1, 0,
		0, 0, 1751, 1752, 3, 446, 223, 0, 1752, 455, 1, 0, 0, 0, 1753, 1754, 1,
		0, 0, 0, 1754, 457, 1, 0, 0, 0, 132, 461, 468, 476, 539, 545, 550, 558,
		563, 572, 584, 597, 610, 616, 620, 629, 645, 653, 666, 670, 680, 684,
		705, 712, 719, 730, 738, 745, 752, 769, 777, 791, 796, 805, 815, 827,
		840, 858, 866, 870, 878, 890, 903, 909, 913, 922, 938, 942, 950, 962,
		970, 982, 994, 1009, 1016, 1023, 1031, 1038, 1049, 1059, 1066, 1072, 1086,
		1090, 1098, 1113, 1120, 1135, 1142, 1158, 1170, 1194, 1201, 1205, 1219,
		1226, 1230, 1236, 1249, 1253, 1257, 1268, 1273, 1277, 1285, 1297, 1310,
		1317, 1325, 1333, 1347, 1352, 1358, 1374, 1378, 1388, 1396, 1403, 1407,
		1412, 1422, 1429, 1435, 1446, 1456, 1468, 1473, 1477, 1485, 1488, 1494,
		1498, 1509, 1523, 1528, 1560, 1571, 1578, 1582, 1614, 1621, 1627, 1639,
		1644, 1661, 1668, 1675, 1685, 1693, 1720, 1727, 1733, 1741
	];
}
