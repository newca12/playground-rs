// Generated from tptp_v7_0_0_0.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::tptp_v7_0_0_0listener::*;
use super::tptp_v7_0_0_0visitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const T__21:isize=22; 
		pub const T__22:isize=23; 
		pub const T__23:isize=24; 
		pub const T__24:isize=25; 
		pub const T__25:isize=26; 
		pub const T__26:isize=27; 
		pub const T__27:isize=28; 
		pub const T__28:isize=29; 
		pub const T__29:isize=30; 
		pub const T__30:isize=31; 
		pub const T__31:isize=32; 
		pub const T__32:isize=33; 
		pub const T__33:isize=34; 
		pub const T__34:isize=35; 
		pub const T__35:isize=36; 
		pub const T__36:isize=37; 
		pub const T__37:isize=38; 
		pub const T__38:isize=39; 
		pub const T__39:isize=40; 
		pub const T__40:isize=41; 
		pub const T__41:isize=42; 
		pub const T__42:isize=43; 
		pub const Or:isize=44; 
		pub const And:isize=45; 
		pub const Iff:isize=46; 
		pub const Impl:isize=47; 
		pub const If:isize=48; 
		pub const Niff:isize=49; 
		pub const Nor:isize=50; 
		pub const Nand:isize=51; 
		pub const Not:isize=52; 
		pub const ForallComb:isize=53; 
		pub const TyForall:isize=54; 
		pub const Infix_inequality:isize=55; 
		pub const Infix_equality:isize=56; 
		pub const Forall:isize=57; 
		pub const ExistsComb:isize=58; 
		pub const TyExists:isize=59; 
		pub const Exists:isize=60; 
		pub const Lambda:isize=61; 
		pub const ChoiceComb:isize=62; 
		pub const Choice:isize=63; 
		pub const DescriptionComb:isize=64; 
		pub const Description:isize=65; 
		pub const EqComb:isize=66; 
		pub const App:isize=67; 
		pub const Assignment:isize=68; 
		pub const Arrow:isize=69; 
		pub const Star:isize=70; 
		pub const Plus:isize=71; 
		pub const Subtype_sign:isize=72; 
		pub const Gentzen_arrow:isize=73; 
		pub const Real:isize=74; 
		pub const Signed_real:isize=75; 
		pub const Unsigned_real:isize=76; 
		pub const Rational:isize=77; 
		pub const Signed_rational:isize=78; 
		pub const Unsigned_rational:isize=79; 
		pub const Integer:isize=80; 
		pub const Signed_integer:isize=81; 
		pub const Unsigned_integer:isize=82; 
		pub const Decimal:isize=83; 
		pub const Positive_decimal:isize=84; 
		pub const Decimal_exponent:isize=85; 
		pub const Decimal_fraction:isize=86; 
		pub const Dot_decimal:isize=87; 
		pub const Exp_integer:isize=88; 
		pub const Signed_exp_integer:isize=89; 
		pub const Unsigned_exp_integer:isize=90; 
		pub const Dollar_word:isize=91; 
		pub const Dollar_dollar_word:isize=92; 
		pub const Upper_word:isize=93; 
		pub const Lower_word:isize=94; 
		pub const Single_quoted:isize=95; 
		pub const Distinct_object:isize=96; 
		pub const WS:isize=97; 
		pub const Line_comment:isize=98; 
		pub const Block_comment:isize=99;
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
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
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

antlr_rust::coerce_from!{ 'input : tptp_v7_0_0_0ParserContext<'input> }

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

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn tptp_v7_0_0_0ParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn tptp_v7_0_0_0Listener<'input> + 'input }

pub struct tptp_v7_0_0_0ParserContextType;
antlr_rust::tid!{tptp_v7_0_0_0ParserContextType}

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
antlr_rust::tid! { tptp_v7_0_0_0ParserExt<'a> }

impl<'input> TokenAware<'input> for tptp_v7_0_0_0ParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for tptp_v7_0_0_0ParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for tptp_v7_0_0_0ParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "tptp_v7_0_0_0.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn tptp_v7_0_0_0ParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
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
	fn thf_or_formula_sempred(_localctx: Option<&Thf_or_formulaContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn thf_and_formula_sempred(_localctx: Option<&Thf_and_formulaContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				1=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn thf_apply_formula_sempred(_localctx: Option<&Thf_apply_formulaContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				2=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn thf_xprod_type_sempred(_localctx: Option<&Thf_xprod_typeContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				3=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn thf_union_type_sempred(_localctx: Option<&Thf_union_typeContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				4=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn tff_or_formula_sempred(_localctx: Option<&Tff_or_formulaContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				5=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn tff_and_formula_sempred(_localctx: Option<&Tff_and_formulaContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				6=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn tff_xprod_type_sempred(_localctx: Option<&Tff_xprod_typeContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				7=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn fof_or_formula_sempred(_localctx: Option<&Fof_or_formulaContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				8=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn fof_and_formula_sempred(_localctx: Option<&Fof_and_formulaContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				9=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn cnf_disjunction_sempred(_localctx: Option<&Cnf_disjunctionContext<'input>>, pred_index:isize,
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
antlr_rust::tid!{Tptp_fileContextExt<'a>}

impl<'input> Tptp_fileContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tptp_fileContextAll<'input>> {
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
	self.get_token(EOF, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(405);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__3) | (1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8))) != 0) || _la==T__37 {
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
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Tptp_inputContextExt<'a>}

impl<'input> Tptp_inputContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tptp_inputContextAll<'input>> {
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
			 T__0 | T__3 | T__4 | T__5 | T__6 | T__7 | T__8 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule annotated_formula*/
					recog.base.set_state(410);
					recog.annotated_formula()?;

					}
				}

			 T__37 
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
antlr_rust::tid!{Annotated_formulaContextExt<'a>}

impl<'input> Annotated_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Annotated_formulaContextAll<'input>> {
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
			 T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_annotated*/
					recog.base.set_state(414);
					recog.thf_annotated()?;

					}
				}

			 T__4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tfx_annotated*/
					recog.base.set_state(415);
					recog.tfx_annotated()?;

					}
				}

			 T__5 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule tff_annotated*/
					recog.base.set_state(416);
					recog.tff_annotated()?;

					}
				}

			 T__6 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule tcf_annotated*/
					recog.base.set_state(417);
					recog.tcf_annotated()?;

					}
				}

			 T__7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule fof_annotated*/
					recog.base.set_state(418);
					recog.fof_annotated()?;

					}
				}

			 T__8 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule cnf_annotated*/
					recog.base.set_state(419);
					recog.cnf_annotated()?;

					}
				}

			 T__0 
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
antlr_rust::tid!{Tpi_annotatedContextExt<'a>}

impl<'input> Tpi_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tpi_annotatedContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(423);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(424);
			recog.name()?;

			recog.base.set_state(425);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(426);
			recog.formula_role()?;

			recog.base.set_state(427);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule tpi_formula*/
			recog.base.set_state(428);
			recog.tpi_formula()?;

			recog.base.set_state(430);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(429);
				recog.annotations()?;

				}
			}

			recog.base.set_state(432);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Tpi_formulaContextExt<'a>}

impl<'input> Tpi_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tpi_formulaContextAll<'input>> {
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
antlr_rust::tid!{Thf_annotatedContextExt<'a>}

impl<'input> Thf_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_annotatedContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(436);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(437);
			recog.name()?;

			recog.base.set_state(438);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(439);
			recog.formula_role()?;

			recog.base.set_state(440);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_formula*/
			recog.base.set_state(441);
			recog.thf_formula()?;

			recog.base.set_state(443);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(442);
				recog.annotations()?;

				}
			}

			recog.base.set_state(445);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Tfx_annotatedContextExt<'a>}

impl<'input> Tfx_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tfx_annotatedContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(447);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(448);
			recog.name()?;

			recog.base.set_state(449);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(450);
			recog.formula_role()?;

			recog.base.set_state(451);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule tfx_formula*/
			recog.base.set_state(452);
			recog.tfx_formula()?;

			recog.base.set_state(454);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(453);
				recog.annotations()?;

				}
			}

			recog.base.set_state(456);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Tff_annotatedContextExt<'a>}

impl<'input> Tff_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_annotatedContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(458);
			recog.base.match_token(T__5,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(459);
			recog.name()?;

			recog.base.set_state(460);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(461);
			recog.formula_role()?;

			recog.base.set_state(462);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_formula*/
			recog.base.set_state(463);
			recog.tff_formula()?;

			recog.base.set_state(465);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(464);
				recog.annotations()?;

				}
			}

			recog.base.set_state(467);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Tcf_annotatedContextExt<'a>}

impl<'input> Tcf_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tcf_annotatedContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(469);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(470);
			recog.name()?;

			recog.base.set_state(471);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(472);
			recog.formula_role()?;

			recog.base.set_state(473);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule tcf_formula*/
			recog.base.set_state(474);
			recog.tcf_formula()?;

			recog.base.set_state(476);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(475);
				recog.annotations()?;

				}
			}

			recog.base.set_state(478);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Fof_annotatedContextExt<'a>}

impl<'input> Fof_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_annotatedContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(480);
			recog.base.match_token(T__7,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(481);
			recog.name()?;

			recog.base.set_state(482);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(483);
			recog.formula_role()?;

			recog.base.set_state(484);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_formula*/
			recog.base.set_state(485);
			recog.fof_formula()?;

			recog.base.set_state(487);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(486);
				recog.annotations()?;

				}
			}

			recog.base.set_state(489);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Cnf_annotatedContextExt<'a>}

impl<'input> Cnf_annotatedContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Cnf_annotatedContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(491);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(492);
			recog.name()?;

			recog.base.set_state(493);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule formula_role*/
			recog.base.set_state(494);
			recog.formula_role()?;

			recog.base.set_state(495);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule cnf_formula*/
			recog.base.set_state(496);
			recog.cnf_formula()?;

			recog.base.set_state(498);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule annotations*/
				recog.base.set_state(497);
				recog.annotations()?;

				}
			}

			recog.base.set_state(500);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{AnnotationsContextExt<'a>}

impl<'input> AnnotationsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationsContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(502);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule source*/
			recog.base.set_state(503);
			recog.source()?;

			recog.base.set_state(505);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
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
antlr_rust::tid!{Formula_roleContextExt<'a>}

impl<'input> Formula_roleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Formula_roleContextAll<'input>> {
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
	self.get_token(Lower_word, 0)
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
			recog.base.match_token(Lower_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Thf_formulaContextExt<'a>}

impl<'input> Thf_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_formulaContextAll<'input>> {
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
antlr_rust::tid!{Thf_logic_formulaContextExt<'a>}

impl<'input> Thf_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_logic_formulaContextAll<'input>> {
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
antlr_rust::tid!{Thf_binary_formulaContextExt<'a>}

impl<'input> Thf_binary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_binary_formulaContextAll<'input>> {
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
antlr_rust::tid!{Thf_binary_pairContextExt<'a>}

impl<'input> Thf_binary_pairContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_binary_pairContextAll<'input>> {
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
antlr_rust::tid!{Thf_binary_tupleContextExt<'a>}

impl<'input> Thf_binary_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_binary_tupleContextAll<'input>> {
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
antlr_rust::tid!{Thf_or_formulaContextExt<'a>}

impl<'input> Thf_or_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_or_formulaContextAll<'input>> {
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
	self.get_token(Or, 0)
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

	fn thf_or_formula_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(534);
			recog.thf_unitary_formula()?;

			recog.base.set_state(535);
			recog.base.match_token(Or,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(539);
					recog.base.match_token(Or,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_and_formulaContextExt<'a>}

impl<'input> Thf_and_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_and_formulaContextAll<'input>> {
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
	self.get_token(And, 0)
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

	fn thf_and_formula_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(547);
			recog.thf_unitary_formula()?;

			recog.base.set_state(548);
			recog.base.match_token(And,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(552);
					recog.base.match_token(And,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_apply_formulaContextExt<'a>}

impl<'input> Thf_apply_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_apply_formulaContextAll<'input>> {
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
	self.get_token(App, 0)
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

	fn thf_apply_formula_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(560);
			recog.thf_unitary_formula()?;

			recog.base.set_state(561);
			recog.base.match_token(App,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(565);
					recog.base.match_token(App,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_unitary_formulaContextExt<'a>}

impl<'input> Thf_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_unitary_formulaContextAll<'input>> {
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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(579);
					recog.thf_logic_formula()?;

					recog.base.set_state(580);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_quantified_formulaContextExt<'a>}

impl<'input> Thf_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_quantified_formulaContextAll<'input>> {
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
antlr_rust::tid!{Thf_quantificationContextExt<'a>}

impl<'input> Thf_quantificationContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_quantificationContextAll<'input>> {
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
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			/*InvokeRule thf_variable_list*/
			recog.base.set_state(589);
			recog.thf_variable_list()?;

			recog.base.set_state(590);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(591);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Thf_variable_listContextExt<'a>}

impl<'input> Thf_variable_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_variable_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(594);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_variableContextExt<'a>}

impl<'input> Thf_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_variableContextAll<'input>> {
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
antlr_rust::tid!{Thf_typed_variableContextExt<'a>}

impl<'input> Thf_typed_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_typed_variableContextAll<'input>> {
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
			recog.base.match_token(T__13,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_unary_formulaContextExt<'a>}

impl<'input> Thf_unary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_unary_formulaContextAll<'input>> {
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
			recog.base.match_token(T__9,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(611);
			recog.thf_logic_formula()?;

			recog.base.set_state(612);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Thf_atomContextExt<'a>}

impl<'input> Thf_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_atomContextAll<'input>> {
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
			 Dollar_word | Dollar_dollar_word | Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_function*/
					recog.base.set_state(614);
					recog.thf_function()?;

					}
				}

			 Upper_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable*/
					recog.base.set_state(615);
					recog.variable()?;

					}
				}

			 Real | Rational | Integer | Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(616);
					recog.defined_term()?;

					}
				}

			 Or | And | Iff | Impl | If | Niff | Nor | Nand | Not | ForallComb | Infix_inequality |
			 Infix_equality | ExistsComb | ChoiceComb | DescriptionComb | EqComb |
			 Assignment 
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
antlr_rust::tid!{Thf_functionContextExt<'a>}

impl<'input> Thf_functionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_functionContextAll<'input>> {
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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(623);
					recog.thf_arguments()?;

					recog.base.set_state(624);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(628);
					recog.thf_arguments()?;

					recog.base.set_state(629);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_arguments*/
					recog.base.set_state(633);
					recog.thf_arguments()?;

					recog.base.set_state(634);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_conn_termContextExt<'a>}

impl<'input> Thf_conn_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_conn_termContextAll<'input>> {
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
			 Iff | Impl | If | Niff | Nor | Nand | Infix_inequality | Infix_equality |
			 Assignment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_pair_connective*/
					recog.base.set_state(638);
					recog.thf_pair_connective()?;

					}
				}

			 Or | And 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assoc_connective*/
					recog.base.set_state(639);
					recog.assoc_connective()?;

					}
				}

			 Not | ForallComb | ExistsComb | ChoiceComb | DescriptionComb | EqComb 
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
antlr_rust::tid!{Thf_conditionalContextExt<'a>}

impl<'input> Thf_conditionalContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_conditionalContextAll<'input>> {
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
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(644);
			recog.thf_logic_formula()?;

			recog.base.set_state(645);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(646);
			recog.thf_logic_formula()?;

			recog.base.set_state(647);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_logic_formula*/
			recog.base.set_state(648);
			recog.thf_logic_formula()?;

			recog.base.set_state(649);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Thf_letContextExt<'a>}

impl<'input> Thf_letContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_letContextAll<'input>> {
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
			recog.base.match_token(T__15,&mut recog.err_handler)?;

			/*InvokeRule thf_unitary_formula*/
			recog.base.set_state(652);
			recog.thf_unitary_formula()?;

			recog.base.set_state(653);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule thf_formula*/
			recog.base.set_state(654);
			recog.thf_formula()?;

			recog.base.set_state(655);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Thf_argumentsContextExt<'a>}

impl<'input> Thf_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_argumentsContextAll<'input>> {
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
antlr_rust::tid!{Thf_type_formulaContextExt<'a>}

impl<'input> Thf_type_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_type_formulaContextAll<'input>> {
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
			recog.base.match_token(T__13,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_typeable_formulaContextExt<'a>}

impl<'input> Thf_typeable_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_typeable_formulaContextAll<'input>> {
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
			 Or | And | Iff | Impl | If | Niff | Nor | Nand | Not | ForallComb | Infix_inequality |
			 Infix_equality | ExistsComb | ChoiceComb | DescriptionComb | EqComb |
			 Assignment | Real | Rational | Integer | Dollar_word | Dollar_dollar_word |
			 Upper_word | Lower_word | Single_quoted | Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_atom*/
					recog.base.set_state(663);
					recog.thf_atom()?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(664);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_logic_formula*/
					recog.base.set_state(665);
					recog.thf_logic_formula()?;

					recog.base.set_state(666);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_subtypeContextExt<'a>}

impl<'input> Thf_subtypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_subtypeContextAll<'input>> {
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
	self.get_token(Subtype_sign, 0)
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
			recog.base.match_token(Subtype_sign,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_top_level_typeContextExt<'a>}

impl<'input> Thf_top_level_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_top_level_typeContextAll<'input>> {
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
antlr_rust::tid!{Thf_unitary_typeContextExt<'a>}

impl<'input> Thf_unitary_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_unitary_typeContextAll<'input>> {
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
antlr_rust::tid!{Thf_apply_typeContextExt<'a>}

impl<'input> Thf_apply_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_apply_typeContextAll<'input>> {
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
antlr_rust::tid!{Thf_binary_typeContextExt<'a>}

impl<'input> Thf_binary_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_binary_typeContextAll<'input>> {
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
antlr_rust::tid!{Thf_mapping_typeContextExt<'a>}

impl<'input> Thf_mapping_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_mapping_typeContextAll<'input>> {
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
	self.get_token(Arrow, 0)
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
					recog.base.match_token(Arrow,&mut recog.err_handler)?;

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
					recog.base.match_token(Arrow,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_xprod_typeContextExt<'a>}

impl<'input> Thf_xprod_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_xprod_typeContextAll<'input>> {
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
	self.get_token(Star, 0)
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

	fn thf_xprod_type_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(699);
			recog.thf_unitary_type()?;

			recog.base.set_state(700);
			recog.base.match_token(Star,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(704);
					recog.base.match_token(Star,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_union_typeContextExt<'a>}

impl<'input> Thf_union_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_union_typeContextAll<'input>> {
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
	self.get_token(Plus, 0)
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

	fn thf_union_type_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule thf_unitary_type*/
			recog.base.set_state(712);
			recog.thf_unitary_type()?;

			recog.base.set_state(713);
			recog.base.match_token(Plus,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(717);
					recog.base.match_token(Plus,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_sequentContextExt<'a>}

impl<'input> Thf_sequentContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_sequentContextAll<'input>> {
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
	self.get_token(Gentzen_arrow, 0)
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
			 T__11 | T__16 | T__17 | T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule thf_tuple*/
					recog.base.set_state(724);
					recog.thf_tuple()?;

					recog.base.set_state(725);
					recog.base.match_token(Gentzen_arrow,&mut recog.err_handler)?;

					/*InvokeRule thf_tuple*/
					recog.base.set_state(726);
					recog.thf_tuple()?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(728);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule thf_sequent*/
					recog.base.set_state(729);
					recog.thf_sequent()?;

					recog.base.set_state(730);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_tupleContextExt<'a>}

impl<'input> Thf_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_tupleContextAll<'input>> {
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
			 T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(734);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(735);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule thf_formula_list*/
					recog.base.set_state(736);
					recog.thf_formula_list()?;

					recog.base.set_state(737);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					}
				}

			 T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(739);
					recog.base.match_token(T__17,&mut recog.err_handler)?;

					}
				}

			 T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(740);
					recog.base.match_token(T__18,&mut recog.err_handler)?;

					/*InvokeRule thf_formula_list*/
					recog.base.set_state(741);
					recog.thf_formula_list()?;

					recog.base.set_state(742);
					recog.base.match_token(T__19,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_formula_listContextExt<'a>}

impl<'input> Thf_formula_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_formula_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(747);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tfx_formulaContextExt<'a>}

impl<'input> Tfx_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tfx_formulaContextAll<'input>> {
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
antlr_rust::tid!{Tfx_logic_formulaContextExt<'a>}

impl<'input> Tfx_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tfx_logic_formulaContextAll<'input>> {
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
antlr_rust::tid!{Tff_formulaContextExt<'a>}

impl<'input> Tff_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_formulaContextAll<'input>> {
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
antlr_rust::tid!{Tff_logic_formulaContextExt<'a>}

impl<'input> Tff_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_logic_formulaContextAll<'input>> {
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
antlr_rust::tid!{Tff_binary_formulaContextExt<'a>}

impl<'input> Tff_binary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_binary_formulaContextAll<'input>> {
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
antlr_rust::tid!{Tff_binary_nonassocContextExt<'a>}

impl<'input> Tff_binary_nonassocContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_binary_nonassocContextAll<'input>> {
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
antlr_rust::tid!{Tff_binary_assocContextExt<'a>}

impl<'input> Tff_binary_assocContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_binary_assocContextAll<'input>> {
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
antlr_rust::tid!{Tff_or_formulaContextExt<'a>}

impl<'input> Tff_or_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_or_formulaContextAll<'input>> {
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
	self.get_token(Or, 0)
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

	fn tff_or_formula_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule tff_unitary_formula*/
			recog.base.set_state(783);
			recog.tff_unitary_formula()?;

			recog.base.set_state(784);
			recog.base.match_token(Or,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(788);
					recog.base.match_token(Or,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_and_formulaContextExt<'a>}

impl<'input> Tff_and_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_and_formulaContextAll<'input>> {
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
	self.get_token(And, 0)
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

	fn tff_and_formula_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule tff_unitary_formula*/
			recog.base.set_state(796);
			recog.tff_unitary_formula()?;

			recog.base.set_state(797);
			recog.base.match_token(And,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(801);
					recog.base.match_token(And,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_unitary_formulaContextExt<'a>}

impl<'input> Tff_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_unitary_formulaContextAll<'input>> {
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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_logic_formula*/
					recog.base.set_state(814);
					recog.tff_logic_formula()?;

					recog.base.set_state(815);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_quantified_formulaContextExt<'a>}

impl<'input> Tff_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_quantified_formulaContextAll<'input>> {
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
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(821);
			recog.tff_variable_list()?;

			recog.base.set_state(822);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(823);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_variable_listContextExt<'a>}

impl<'input> Tff_variable_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_variable_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(827);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_variableContextExt<'a>}

impl<'input> Tff_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_variableContextAll<'input>> {
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
antlr_rust::tid!{Tff_typed_variableContextExt<'a>}

impl<'input> Tff_typed_variableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_typed_variableContextAll<'input>> {
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
			recog.base.match_token(T__13,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_unary_formulaContextExt<'a>}

impl<'input> Tff_unary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_unary_formulaContextAll<'input>> {
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
			 Not 
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

			 T__17 | T__18 | T__23 | T__24 | T__25 | Real | Rational | Integer | Dollar_word |
			 Dollar_dollar_word | Upper_word | Lower_word | Single_quoted | Distinct_object 
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
antlr_rust::tid!{Tff_atomic_formulaContextExt<'a>}

impl<'input> Tff_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_atomic_formulaContextAll<'input>> {
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
antlr_rust::tid!{Tff_conditionalContextExt<'a>}

impl<'input> Tff_conditionalContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_conditionalContextAll<'input>> {
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
			recog.base.match_token(T__20,&mut recog.err_handler)?;

			/*InvokeRule tff_logic_formula*/
			recog.base.set_state(851);
			recog.tff_logic_formula()?;

			recog.base.set_state(852);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_logic_formula*/
			recog.base.set_state(853);
			recog.tff_logic_formula()?;

			recog.base.set_state(854);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule tff_logic_formula*/
			recog.base.set_state(855);
			recog.tff_logic_formula()?;

			recog.base.set_state(856);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Tff_letContextExt<'a>}

impl<'input> Tff_letContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_letContextAll<'input>> {
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
			 T__21 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(858);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_defns*/
					recog.base.set_state(859);
					recog.tff_let_term_defns()?;

					recog.base.set_state(860);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_formula*/
					recog.base.set_state(861);
					recog.tff_formula()?;

					recog.base.set_state(862);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

			 T__22 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(864);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_defns*/
					recog.base.set_state(865);
					recog.tff_let_formula_defns()?;

					recog.base.set_state(866);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule tff_formula*/
					recog.base.set_state(867);
					recog.tff_formula()?;

					recog.base.set_state(868);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_let_term_defnsContextExt<'a>}

impl<'input> Tff_let_term_defnsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_let_term_defnsContextAll<'input>> {
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
			 T__9 | Forall | Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_let_term_defn*/
					recog.base.set_state(872);
					recog.tff_let_term_defn()?;

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(873);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_list*/
					recog.base.set_state(874);
					recog.tff_let_term_list()?;

					recog.base.set_state(875);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_let_term_listContextExt<'a>}

impl<'input> Tff_let_term_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_let_term_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(880);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_let_term_defnContextExt<'a>}

impl<'input> Tff_let_term_defnContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_let_term_defnContextAll<'input>> {
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
	self.get_token(Forall, 0)
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
			 Forall 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(887);
					recog.base.match_token(Forall,&mut recog.err_handler)?;

					recog.base.set_state(888);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_variable_list*/
					recog.base.set_state(889);
					recog.tff_variable_list()?;

					recog.base.set_state(890);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					recog.base.set_state(891);
					recog.base.match_token(T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_defn*/
					recog.base.set_state(892);
					recog.tff_let_term_defn()?;

					}
				}

			 T__9 | Lower_word | Single_quoted 
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
antlr_rust::tid!{Tff_let_term_bindingContextExt<'a>}

impl<'input> Tff_let_term_bindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_let_term_bindingContextAll<'input>> {
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
	self.get_token(Infix_equality, 0)
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
			 Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_plain_term*/
					recog.base.set_state(897);
					recog.fof_plain_term()?;

					recog.base.set_state(898);
					recog.base.match_token(Infix_equality,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(899);
					recog.fof_term()?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(901);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_binding*/
					recog.base.set_state(902);
					recog.tff_let_term_binding()?;

					recog.base.set_state(903);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_let_formula_defnsContextExt<'a>}

impl<'input> Tff_let_formula_defnsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_let_formula_defnsContextAll<'input>> {
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
			 T__9 | Forall | Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_let_formula_defn*/
					recog.base.set_state(907);
					recog.tff_let_formula_defn()?;

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(908);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_list*/
					recog.base.set_state(909);
					recog.tff_let_formula_list()?;

					recog.base.set_state(910);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_let_formula_listContextExt<'a>}

impl<'input> Tff_let_formula_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_let_formula_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(915);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_let_formula_defnContextExt<'a>}

impl<'input> Tff_let_formula_defnContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_let_formula_defnContextAll<'input>> {
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
	self.get_token(Forall, 0)
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
			 Forall 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(922);
					recog.base.match_token(Forall,&mut recog.err_handler)?;

					recog.base.set_state(923);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_variable_list*/
					recog.base.set_state(924);
					recog.tff_variable_list()?;

					recog.base.set_state(925);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					recog.base.set_state(926);
					recog.base.match_token(T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_defn*/
					recog.base.set_state(927);
					recog.tff_let_formula_defn()?;

					}
				}

			 T__9 | Lower_word | Single_quoted 
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
antlr_rust::tid!{Tff_let_formula_bindingContextExt<'a>}

impl<'input> Tff_let_formula_bindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_let_formula_bindingContextAll<'input>> {
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
	self.get_token(Iff, 0)
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
			 Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_plain_atomic_formula*/
					recog.base.set_state(932);
					recog.fof_plain_atomic_formula()?;

					recog.base.set_state(933);
					recog.base.match_token(Iff,&mut recog.err_handler)?;

					/*InvokeRule tff_unitary_formula*/
					recog.base.set_state(934);
					recog.tff_unitary_formula()?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(936);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_binding*/
					recog.base.set_state(937);
					recog.tff_let_formula_binding()?;

					recog.base.set_state(938);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_sequentContextExt<'a>}

impl<'input> Tff_sequentContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_sequentContextAll<'input>> {
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
	self.get_token(Gentzen_arrow, 0)
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
			 T__11 | T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_formula_tuple*/
					recog.base.set_state(942);
					recog.tff_formula_tuple()?;

					recog.base.set_state(943);
					recog.base.match_token(Gentzen_arrow,&mut recog.err_handler)?;

					/*InvokeRule tff_formula_tuple*/
					recog.base.set_state(944);
					recog.tff_formula_tuple()?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(946);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_sequent*/
					recog.base.set_state(947);
					recog.tff_sequent()?;

					recog.base.set_state(948);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_formula_tupleContextExt<'a>}

impl<'input> Tff_formula_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_formula_tupleContextAll<'input>> {
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
			 T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(952);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(953);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule tff_formula_tuple_list*/
					recog.base.set_state(954);
					recog.tff_formula_tuple_list()?;

					recog.base.set_state(955);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_formula_tuple_listContextExt<'a>}

impl<'input> Tff_formula_tuple_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_formula_tuple_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(960);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_typed_atomContextExt<'a>}

impl<'input> Tff_typed_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_typed_atomContextAll<'input>> {
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
			 Dollar_dollar_word | Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule untyped_atom*/
					recog.base.set_state(967);
					recog.untyped_atom()?;

					recog.base.set_state(968);
					recog.base.match_token(T__13,&mut recog.err_handler)?;

					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(969);
					recog.tff_top_level_type()?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(971);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_typed_atom*/
					recog.base.set_state(972);
					recog.tff_typed_atom()?;

					recog.base.set_state(973);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_subtypeContextExt<'a>}

impl<'input> Tff_subtypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_subtypeContextAll<'input>> {
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
	self.get_token(Subtype_sign, 0)
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
			recog.base.match_token(Subtype_sign,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_top_level_typeContextExt<'a>}

impl<'input> Tff_top_level_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_top_level_typeContextAll<'input>> {
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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_top_level_type*/
					recog.base.set_state(985);
					recog.tff_top_level_type()?;

					recog.base.set_state(986);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tf1_quantified_typeContextExt<'a>}

impl<'input> Tf1_quantified_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tf1_quantified_typeContextAll<'input>> {
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
	self.get_token(TyForall, 0)
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
			recog.base.match_token(TyForall,&mut recog.err_handler)?;

			recog.base.set_state(991);
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(992);
			recog.tff_variable_list()?;

			recog.base.set_state(993);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(994);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_monotypeContextExt<'a>}

impl<'input> Tff_monotypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_monotypeContextAll<'input>> {
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
			 Dollar_word | Upper_word | Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(997);
					recog.tff_atomic_type()?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(998);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_mapping_type*/
					recog.base.set_state(999);
					recog.tff_mapping_type()?;

					recog.base.set_state(1000);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_unitary_typeContextExt<'a>}

impl<'input> Tff_unitary_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_unitary_typeContextAll<'input>> {
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
			 Dollar_word | Upper_word | Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tff_atomic_type*/
					recog.base.set_state(1004);
					recog.tff_atomic_type()?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1005);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_xprod_type*/
					recog.base.set_state(1006);
					recog.tff_xprod_type_rec(0)?;

					recog.base.set_state(1007);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_atomic_typeContextExt<'a>}

impl<'input> Tff_atomic_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_atomic_typeContextAll<'input>> {
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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule tff_type_arguments*/
					recog.base.set_state(1015);
					recog.tff_type_arguments()?;

					recog.base.set_state(1016);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_type_argumentsContextExt<'a>}

impl<'input> Tff_type_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_type_argumentsContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1022);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_mapping_typeContextExt<'a>}

impl<'input> Tff_mapping_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_mapping_typeContextAll<'input>> {
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
	self.get_token(Arrow, 0)
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
			recog.base.match_token(Arrow,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_xprod_typeContextExt<'a>}

impl<'input> Tff_xprod_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_xprod_typeContextAll<'input>> {
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
	self.get_token(Star, 0)
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

	fn tff_xprod_type_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule tff_unitary_type*/
			recog.base.set_state(1034);
			recog.tff_unitary_type()?;

			recog.base.set_state(1035);
			recog.base.match_token(Star,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1039);
					recog.base.match_token(Star,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tcf_formulaContextExt<'a>}

impl<'input> Tcf_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tcf_formulaContextAll<'input>> {
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
antlr_rust::tid!{Tcf_logic_formulaContextExt<'a>}

impl<'input> Tcf_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tcf_logic_formulaContextAll<'input>> {
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
			 Forall 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule tcf_quantified_formula*/
					recog.base.set_state(1050);
					recog.tcf_quantified_formula()?;

					}
				}

			 T__9 | T__17 | T__18 | T__23 | T__24 | T__25 | Not | Real | Rational |
			 Integer | Dollar_word | Dollar_dollar_word | Upper_word | Lower_word |
			 Single_quoted | Distinct_object 
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
antlr_rust::tid!{Tcf_quantified_formulaContextExt<'a>}

impl<'input> Tcf_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tcf_quantified_formulaContextAll<'input>> {
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
	self.get_token(Forall, 0)
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
			recog.base.match_token(Forall,&mut recog.err_handler)?;

			recog.base.set_state(1055);
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			/*InvokeRule tff_variable_list*/
			recog.base.set_state(1056);
			recog.tff_variable_list()?;

			recog.base.set_state(1057);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(1058);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_formulaContextExt<'a>}

impl<'input> Fof_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_formulaContextAll<'input>> {
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
antlr_rust::tid!{Fof_logic_formulaContextExt<'a>}

impl<'input> Fof_logic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_logic_formulaContextAll<'input>> {
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
antlr_rust::tid!{Fof_binary_formulaContextExt<'a>}

impl<'input> Fof_binary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_binary_formulaContextAll<'input>> {
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
antlr_rust::tid!{Fof_binary_nonassocContextExt<'a>}

impl<'input> Fof_binary_nonassocContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_binary_nonassocContextAll<'input>> {
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
antlr_rust::tid!{Fof_binary_assocContextExt<'a>}

impl<'input> Fof_binary_assocContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_binary_assocContextAll<'input>> {
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
antlr_rust::tid!{Fof_or_formulaContextExt<'a>}

impl<'input> Fof_or_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_or_formulaContextAll<'input>> {
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
	self.get_token(Or, 0)
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

	fn fof_or_formula_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule fof_unitary_formula*/
			recog.base.set_state(1082);
			recog.fof_unitary_formula()?;

			recog.base.set_state(1083);
			recog.base.match_token(Or,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1087);
					recog.base.match_token(Or,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_and_formulaContextExt<'a>}

impl<'input> Fof_and_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_and_formulaContextAll<'input>> {
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
	self.get_token(And, 0)
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

	fn fof_and_formula_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule fof_unitary_formula*/
			recog.base.set_state(1095);
			recog.fof_unitary_formula()?;

			recog.base.set_state(1096);
			recog.base.match_token(And,&mut recog.err_handler)?;

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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1100);
					recog.base.match_token(And,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_unitary_formulaContextExt<'a>}

impl<'input> Fof_unitary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_unitary_formulaContextAll<'input>> {
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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_logic_formula*/
					recog.base.set_state(1111);
					recog.fof_logic_formula()?;

					recog.base.set_state(1112);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_quantified_formulaContextExt<'a>}

impl<'input> Fof_quantified_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_quantified_formulaContextAll<'input>> {
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
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			/*InvokeRule fof_variable_list*/
			recog.base.set_state(1118);
			recog.fof_variable_list()?;

			recog.base.set_state(1119);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(1120);
			recog.base.match_token(T__13,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_variable_listContextExt<'a>}

impl<'input> Fof_variable_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_variable_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1124);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_unary_formulaContextExt<'a>}

impl<'input> Fof_unary_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_unary_formulaContextAll<'input>> {
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
			 Not 
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

			 T__17 | T__18 | T__23 | T__24 | T__25 | Real | Rational | Integer | Dollar_word |
			 Dollar_dollar_word | Upper_word | Lower_word | Single_quoted | Distinct_object 
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
antlr_rust::tid!{Fof_infix_unaryContextExt<'a>}

impl<'input> Fof_infix_unaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_infix_unaryContextAll<'input>> {
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
	self.get_token(Infix_inequality, 0)
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
			recog.base.match_token(Infix_inequality,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_atomic_formulaContextExt<'a>}

impl<'input> Fof_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_atomic_formulaContextAll<'input>> {
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
antlr_rust::tid!{Fof_plain_atomic_formulaContextExt<'a>}

impl<'input> Fof_plain_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_plain_atomic_formulaContextAll<'input>> {
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
antlr_rust::tid!{Fof_defined_atomic_formulaContextExt<'a>}

impl<'input> Fof_defined_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_defined_atomic_formulaContextAll<'input>> {
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
antlr_rust::tid!{Fof_defined_plain_formulaContextExt<'a>}

impl<'input> Fof_defined_plain_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_defined_plain_formulaContextAll<'input>> {
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
antlr_rust::tid!{Fof_defined_infix_formulaContextExt<'a>}

impl<'input> Fof_defined_infix_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_defined_infix_formulaContextAll<'input>> {
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
antlr_rust::tid!{Fof_system_atomic_formulaContextExt<'a>}

impl<'input> Fof_system_atomic_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_system_atomic_formulaContextAll<'input>> {
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
antlr_rust::tid!{Fof_plain_termContextExt<'a>}

impl<'input> Fof_plain_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_plain_termContextAll<'input>> {
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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1163);
					recog.fof_arguments()?;

					recog.base.set_state(1164);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_defined_termContextExt<'a>}

impl<'input> Fof_defined_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_defined_termContextAll<'input>> {
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
			 Real | Rational | Integer | Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule defined_term*/
					recog.base.set_state(1168);
					recog.defined_term()?;

					}
				}

			 Dollar_word 
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
antlr_rust::tid!{Fof_defined_atomic_termContextExt<'a>}

impl<'input> Fof_defined_atomic_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_defined_atomic_termContextAll<'input>> {
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
antlr_rust::tid!{Fof_defined_plain_termContextExt<'a>}

impl<'input> Fof_defined_plain_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_defined_plain_termContextAll<'input>> {
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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1177);
					recog.fof_arguments()?;

					recog.base.set_state(1178);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_system_termContextExt<'a>}

impl<'input> Fof_system_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_system_termContextAll<'input>> {
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
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1185);
					recog.fof_arguments()?;

					recog.base.set_state(1186);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_argumentsContextExt<'a>}

impl<'input> Fof_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_argumentsContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1191);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_termContextExt<'a>}

impl<'input> Fof_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_termContextAll<'input>> {
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
			 Real | Rational | Integer | Dollar_word | Dollar_dollar_word | Lower_word |
			 Single_quoted | Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_function_term*/
					recog.base.set_state(1198);
					recog.fof_function_term()?;

					}
				}

			 Upper_word 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variable*/
					recog.base.set_state(1199);
					recog.variable()?;

					}
				}

			 T__23 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule tff_conditional_term*/
					recog.base.set_state(1200);
					recog.tff_conditional_term()?;

					}
				}

			 T__24 | T__25 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule tff_let_term*/
					recog.base.set_state(1201);
					recog.tff_let_term()?;

					}
				}

			 T__17 | T__18 
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
antlr_rust::tid!{Fof_function_termContextExt<'a>}

impl<'input> Fof_function_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_function_termContextAll<'input>> {
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
			 Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_plain_term*/
					recog.base.set_state(1205);
					recog.fof_plain_term()?;

					}
				}

			 Real | Rational | Integer | Dollar_word | Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule fof_defined_term*/
					recog.base.set_state(1206);
					recog.fof_defined_term()?;

					}
				}

			 Dollar_dollar_word 
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
antlr_rust::tid!{Tff_conditional_termContextExt<'a>}

impl<'input> Tff_conditional_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_conditional_termContextAll<'input>> {
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
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			/*InvokeRule tff_logic_formula*/
			recog.base.set_state(1211);
			recog.tff_logic_formula()?;

			recog.base.set_state(1212);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1213);
			recog.fof_term()?;

			recog.base.set_state(1214);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule fof_term*/
			recog.base.set_state(1215);
			recog.fof_term()?;

			recog.base.set_state(1216);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Tff_let_termContextExt<'a>}

impl<'input> Tff_let_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_let_termContextAll<'input>> {
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
			 T__24 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1218);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule tff_let_formula_defns*/
					recog.base.set_state(1219);
					recog.tff_let_formula_defns()?;

					recog.base.set_state(1220);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(1221);
					recog.fof_term()?;

					recog.base.set_state(1222);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

			 T__25 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1224);
					recog.base.match_token(T__25,&mut recog.err_handler)?;

					/*InvokeRule tff_let_term_defns*/
					recog.base.set_state(1225);
					recog.tff_let_term_defns()?;

					recog.base.set_state(1226);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(1227);
					recog.fof_term()?;

					recog.base.set_state(1228);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Tff_tuple_termContextExt<'a>}

impl<'input> Tff_tuple_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_tuple_termContextAll<'input>> {
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
			 T__17 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1232);
					recog.base.match_token(T__17,&mut recog.err_handler)?;

					}
				}

			 T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1233);
					recog.base.match_token(T__18,&mut recog.err_handler)?;

					/*InvokeRule fof_arguments*/
					recog.base.set_state(1234);
					recog.fof_arguments()?;

					recog.base.set_state(1235);
					recog.base.match_token(T__19,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_sequentContextExt<'a>}

impl<'input> Fof_sequentContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_sequentContextAll<'input>> {
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
	self.get_token(Gentzen_arrow, 0)
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
			 T__11 | T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_formula_tuple*/
					recog.base.set_state(1239);
					recog.fof_formula_tuple()?;

					recog.base.set_state(1240);
					recog.base.match_token(Gentzen_arrow,&mut recog.err_handler)?;

					/*InvokeRule fof_formula_tuple*/
					recog.base.set_state(1241);
					recog.fof_formula_tuple()?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1243);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule fof_sequent*/
					recog.base.set_state(1244);
					recog.fof_sequent()?;

					recog.base.set_state(1245);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_formula_tupleContextExt<'a>}

impl<'input> Fof_formula_tupleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_formula_tupleContextAll<'input>> {
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
			 T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1249);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1250);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule fof_formula_tuple_list*/
					recog.base.set_state(1251);
					recog.fof_formula_tuple_list()?;

					recog.base.set_state(1252);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_formula_tuple_listContextExt<'a>}

impl<'input> Fof_formula_tuple_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_formula_tuple_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1257);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Cnf_formulaContextExt<'a>}

impl<'input> Cnf_formulaContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Cnf_formulaContextAll<'input>> {
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
			 T__17 | T__18 | T__23 | T__24 | T__25 | Not | Real | Rational | Integer |
			 Dollar_word | Dollar_dollar_word | Upper_word | Lower_word | Single_quoted |
			 Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule cnf_disjunction*/
					recog.base.set_state(1264);
					recog.cnf_disjunction_rec(0)?;

					}
				}

			 T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1265);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					/*InvokeRule cnf_disjunction*/
					recog.base.set_state(1266);
					recog.cnf_disjunction_rec(0)?;

					recog.base.set_state(1267);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{Cnf_disjunctionContextExt<'a>}

impl<'input> Cnf_disjunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Cnf_disjunctionContextAll<'input>> {
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
	self.get_token(Or, 0)
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

	fn cnf_disjunction_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
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
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1275);
					recog.base.match_token(Or,&mut recog.err_handler)?;

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
antlr_rust::tid!{Cnf_literalContextExt<'a>}

impl<'input> Cnf_literalContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Cnf_literalContextAll<'input>> {
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
	self.get_token(Not, 0)
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
					recog.base.match_token(Not,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_quantifierContextExt<'a>}

impl<'input> Thf_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_quantifierContextAll<'input>> {
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
			 Forall | Exists 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule fof_quantifier*/
					recog.base.set_state(1288);
					recog.fof_quantifier()?;

					}
				}

			 Lambda | Choice | Description 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule th0_quantifier*/
					recog.base.set_state(1289);
					recog.th0_quantifier()?;

					}
				}

			 TyForall | TyExists 
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
antlr_rust::tid!{Th0_quantifierContextExt<'a>}

impl<'input> Th0_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Th0_quantifierContextAll<'input>> {
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
	self.get_token(Lambda, 0)
}
/// Retrieves first TerminalNode corresponding to token Choice
/// Returns `None` if there is no child corresponding to token Choice
fn Choice(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Choice, 0)
}
/// Retrieves first TerminalNode corresponding to token Description
/// Returns `None` if there is no child corresponding to token Description
fn Description(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Description, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1293);
			_la = recog.base.input.la(1);
			if { !(((((_la - 61)) & !0x3f) == 0 && ((1usize << (_la - 61)) & ((1usize << (Lambda - 61)) | (1usize << (Choice - 61)) | (1usize << (Description - 61)))) != 0)) } {
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
antlr_rust::tid!{Th1_quantifierContextExt<'a>}

impl<'input> Th1_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Th1_quantifierContextAll<'input>> {
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
	self.get_token(TyForall, 0)
}
/// Retrieves first TerminalNode corresponding to token TyExists
/// Returns `None` if there is no child corresponding to token TyExists
fn TyExists(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(TyExists, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1295);
			_la = recog.base.input.la(1);
			if { !(_la==TyForall || _la==TyExists) } {
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
antlr_rust::tid!{Thf_pair_connectiveContextExt<'a>}

impl<'input> Thf_pair_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_pair_connectiveContextAll<'input>> {
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
	self.get_token(Infix_equality, 0)
}
/// Retrieves first TerminalNode corresponding to token Infix_inequality
/// Returns `None` if there is no child corresponding to token Infix_inequality
fn Infix_inequality(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Infix_inequality, 0)
}
fn binary_connective(&self) -> Option<Rc<Binary_connectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Assignment
/// Returns `None` if there is no child corresponding to token Assignment
fn Assignment(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Assignment, 0)
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
			 Infix_equality 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1297);
					recog.base.match_token(Infix_equality,&mut recog.err_handler)?;

					}
				}

			 Infix_inequality 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1298);
					recog.base.match_token(Infix_inequality,&mut recog.err_handler)?;

					}
				}

			 Iff | Impl | If | Niff | Nor | Nand 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule binary_connective*/
					recog.base.set_state(1299);
					recog.binary_connective()?;

					}
				}

			 Assignment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1300);
					recog.base.match_token(Assignment,&mut recog.err_handler)?;

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
antlr_rust::tid!{Thf_unary_connectiveContextExt<'a>}

impl<'input> Thf_unary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Thf_unary_connectiveContextAll<'input>> {
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
			 Not 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule unary_connective*/
					recog.base.set_state(1303);
					recog.unary_connective()?;

					}
				}

			 ForallComb | ExistsComb | ChoiceComb | DescriptionComb | EqComb 
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
antlr_rust::tid!{Th1_unary_connectiveContextExt<'a>}

impl<'input> Th1_unary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Th1_unary_connectiveContextAll<'input>> {
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
	self.get_token(ForallComb, 0)
}
/// Retrieves first TerminalNode corresponding to token ExistsComb
/// Returns `None` if there is no child corresponding to token ExistsComb
fn ExistsComb(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(ExistsComb, 0)
}
/// Retrieves first TerminalNode corresponding to token ChoiceComb
/// Returns `None` if there is no child corresponding to token ChoiceComb
fn ChoiceComb(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(ChoiceComb, 0)
}
/// Retrieves first TerminalNode corresponding to token DescriptionComb
/// Returns `None` if there is no child corresponding to token DescriptionComb
fn DescriptionComb(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(DescriptionComb, 0)
}
/// Retrieves first TerminalNode corresponding to token EqComb
/// Returns `None` if there is no child corresponding to token EqComb
fn EqComb(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(EqComb, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1307);
			_la = recog.base.input.la(1);
			if { !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (ForallComb - 53)) | (1usize << (ExistsComb - 53)) | (1usize << (ChoiceComb - 53)) | (1usize << (DescriptionComb - 53)) | (1usize << (EqComb - 53)))) != 0)) } {
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
antlr_rust::tid!{Tff_pair_connectiveContextExt<'a>}

impl<'input> Tff_pair_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Tff_pair_connectiveContextAll<'input>> {
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
	self.get_token(Assignment, 0)
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
			 Iff | Impl | If | Niff | Nor | Nand 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule binary_connective*/
					recog.base.set_state(1309);
					recog.binary_connective()?;

					}
				}

			 Assignment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1310);
					recog.base.match_token(Assignment,&mut recog.err_handler)?;

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
antlr_rust::tid!{Fof_quantifierContextExt<'a>}

impl<'input> Fof_quantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Fof_quantifierContextAll<'input>> {
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
	self.get_token(Forall, 0)
}
/// Retrieves first TerminalNode corresponding to token Exists
/// Returns `None` if there is no child corresponding to token Exists
fn Exists(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Exists, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1313);
			_la = recog.base.input.la(1);
			if { !(_la==Forall || _la==Exists) } {
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
antlr_rust::tid!{Binary_connectiveContextExt<'a>}

impl<'input> Binary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Binary_connectiveContextAll<'input>> {
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
	self.get_token(Iff, 0)
}
/// Retrieves first TerminalNode corresponding to token Impl
/// Returns `None` if there is no child corresponding to token Impl
fn Impl(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Impl, 0)
}
/// Retrieves first TerminalNode corresponding to token If
/// Returns `None` if there is no child corresponding to token If
fn If(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(If, 0)
}
/// Retrieves first TerminalNode corresponding to token Niff
/// Returns `None` if there is no child corresponding to token Niff
fn Niff(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Niff, 0)
}
/// Retrieves first TerminalNode corresponding to token Nor
/// Returns `None` if there is no child corresponding to token Nor
fn Nor(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Nor, 0)
}
/// Retrieves first TerminalNode corresponding to token Nand
/// Returns `None` if there is no child corresponding to token Nand
fn Nand(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Nand, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1315);
			_la = recog.base.input.la(1);
			if { !(((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (Iff - 46)) | (1usize << (Impl - 46)) | (1usize << (If - 46)) | (1usize << (Niff - 46)) | (1usize << (Nor - 46)) | (1usize << (Nand - 46)))) != 0)) } {
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
antlr_rust::tid!{Assoc_connectiveContextExt<'a>}

impl<'input> Assoc_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Assoc_connectiveContextAll<'input>> {
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
	self.get_token(Or, 0)
}
/// Retrieves first TerminalNode corresponding to token And
/// Returns `None` if there is no child corresponding to token And
fn And(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(And, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1317);
			_la = recog.base.input.la(1);
			if { !(_la==Or || _la==And) } {
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
antlr_rust::tid!{Unary_connectiveContextExt<'a>}

impl<'input> Unary_connectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Unary_connectiveContextAll<'input>> {
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
	self.get_token(Not, 0)
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
			recog.base.match_token(Not,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Type_constantContextExt<'a>}

impl<'input> Type_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_constantContextAll<'input>> {
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
antlr_rust::tid!{Type_functorContextExt<'a>}

impl<'input> Type_functorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_functorContextAll<'input>> {
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
antlr_rust::tid!{Defined_typeContextExt<'a>}

impl<'input> Defined_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Defined_typeContextAll<'input>> {
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
	self.get_token(Dollar_word, 0)
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
			recog.base.match_token(Dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{System_typeContextExt<'a>}

impl<'input> System_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<System_typeContextAll<'input>> {
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
antlr_rust::tid!{AtomContextExt<'a>}

impl<'input> AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AtomContextAll<'input>> {
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
			 Dollar_dollar_word | Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule untyped_atom*/
					recog.base.set_state(1329);
					recog.untyped_atom()?;

					}
				}

			 Dollar_word 
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
antlr_rust::tid!{Untyped_atomContextExt<'a>}

impl<'input> Untyped_atomContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Untyped_atomContextAll<'input>> {
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
			 Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule constant*/
					recog.base.set_state(1333);
					recog.constant()?;

					}
				}

			 Dollar_dollar_word 
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
antlr_rust::tid!{Defined_propositionContextExt<'a>}

impl<'input> Defined_propositionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Defined_propositionContextAll<'input>> {
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
	self.get_token(Dollar_word, 0)
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
			recog.base.match_token(Dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Defined_predicateContextExt<'a>}

impl<'input> Defined_predicateContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Defined_predicateContextAll<'input>> {
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
	self.get_token(Dollar_word, 0)
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
			recog.base.match_token(Dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Defined_infix_predContextExt<'a>}

impl<'input> Defined_infix_predContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Defined_infix_predContextAll<'input>> {
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
	self.get_token(Infix_equality, 0)
}
/// Retrieves first TerminalNode corresponding to token Assignment
/// Returns `None` if there is no child corresponding to token Assignment
fn Assignment(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Assignment, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1341);
			_la = recog.base.input.la(1);
			if { !(_la==Infix_equality || _la==Assignment) } {
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
antlr_rust::tid!{ConstantContextExt<'a>}

impl<'input> ConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstantContextAll<'input>> {
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
antlr_rust::tid!{FunctorContextExt<'a>}

impl<'input> FunctorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctorContextAll<'input>> {
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
antlr_rust::tid!{System_constantContextExt<'a>}

impl<'input> System_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<System_constantContextAll<'input>> {
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
antlr_rust::tid!{System_functorContextExt<'a>}

impl<'input> System_functorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<System_functorContextAll<'input>> {
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
antlr_rust::tid!{Defined_constantContextExt<'a>}

impl<'input> Defined_constantContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Defined_constantContextAll<'input>> {
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
antlr_rust::tid!{Defined_functorContextExt<'a>}

impl<'input> Defined_functorContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Defined_functorContextAll<'input>> {
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
antlr_rust::tid!{Defined_termContextExt<'a>}

impl<'input> Defined_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Defined_termContextAll<'input>> {
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
	self.get_token(Distinct_object, 0)
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
			 Real | Rational | Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule number*/
					recog.base.set_state(1355);
					recog.number()?;

					}
				}

			 Distinct_object 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1356);
					recog.base.match_token(Distinct_object,&mut recog.err_handler)?;

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
antlr_rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableContextAll<'input>> {
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
	self.get_token(Upper_word, 0)
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
			recog.base.match_token(Upper_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{SourceContextExt<'a>}

impl<'input> SourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SourceContextAll<'input>> {
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
	self.get_token(Lower_word, 0)
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
					recog.base.match_token(Lower_word,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1365);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule sources*/
					recog.base.set_state(1366);
					recog.sources()?;

					recog.base.set_state(1367);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

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
antlr_rust::tid!{SourcesContextExt<'a>}

impl<'input> SourcesContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SourcesContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1372);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Dag_sourceContextExt<'a>}

impl<'input> Dag_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Dag_sourceContextAll<'input>> {
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
			 Integer | Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule name*/
					recog.base.set_state(1379);
					recog.name()?;

					}
				}

			 T__26 
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
antlr_rust::tid!{Inference_recordContextExt<'a>}

impl<'input> Inference_recordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Inference_recordContextAll<'input>> {
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
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			/*InvokeRule inference_rule*/
			recog.base.set_state(1384);
			recog.inference_rule()?;

			recog.base.set_state(1385);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule useful_info*/
			recog.base.set_state(1386);
			recog.useful_info()?;

			recog.base.set_state(1387);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule inference_parents*/
			recog.base.set_state(1388);
			recog.inference_parents()?;

			recog.base.set_state(1389);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Inference_ruleContextExt<'a>}

impl<'input> Inference_ruleContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Inference_ruleContextAll<'input>> {
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
antlr_rust::tid!{Inference_parentsContextExt<'a>}

impl<'input> Inference_parentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Inference_parentsContextAll<'input>> {
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
			 T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1393);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1394);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule parent_list*/
					recog.base.set_state(1395);
					recog.parent_list()?;

					recog.base.set_state(1396);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

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
antlr_rust::tid!{Parent_listContextExt<'a>}

impl<'input> Parent_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Parent_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1401);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Parent_infoContextExt<'a>}

impl<'input> Parent_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Parent_infoContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			if _la==T__13 {
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
antlr_rust::tid!{Parent_detailsContextExt<'a>}

impl<'input> Parent_detailsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Parent_detailsContextAll<'input>> {
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
			recog.base.match_token(T__13,&mut recog.err_handler)?;

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
antlr_rust::tid!{Internal_sourceContextExt<'a>}

impl<'input> Internal_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Internal_sourceContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1415);
			recog.base.match_token(T__27,&mut recog.err_handler)?;

			/*InvokeRule intro_type*/
			recog.base.set_state(1416);
			recog.intro_type()?;

			recog.base.set_state(1418);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule optional_info*/
				recog.base.set_state(1417);
				recog.optional_info()?;

				}
			}

			recog.base.set_state(1420);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Intro_typeContextExt<'a>}

impl<'input> Intro_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Intro_typeContextAll<'input>> {
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
	self.get_token(Lower_word, 0)
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
			recog.base.match_token(Lower_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{External_sourceContextExt<'a>}

impl<'input> External_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<External_sourceContextAll<'input>> {
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
			 T__28 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule file_source*/
					recog.base.set_state(1424);
					recog.file_source()?;

					}
				}

			 T__29 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule theory*/
					recog.base.set_state(1425);
					recog.theory()?;

					}
				}

			 T__30 
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
antlr_rust::tid!{File_sourceContextExt<'a>}

impl<'input> File_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<File_sourceContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1429);
			recog.base.match_token(T__28,&mut recog.err_handler)?;

			/*InvokeRule file_name*/
			recog.base.set_state(1430);
			recog.file_name()?;

			recog.base.set_state(1432);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule file_info*/
				recog.base.set_state(1431);
				recog.file_info()?;

				}
			}

			recog.base.set_state(1434);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{File_infoContextExt<'a>}

impl<'input> File_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<File_infoContextAll<'input>> {
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
			recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{TheoryContextExt<'a>}

impl<'input> TheoryContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TheoryContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1439);
			recog.base.match_token(T__29,&mut recog.err_handler)?;

			/*InvokeRule theory_name*/
			recog.base.set_state(1440);
			recog.theory_name()?;

			recog.base.set_state(1442);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule optional_info*/
				recog.base.set_state(1441);
				recog.optional_info()?;

				}
			}

			recog.base.set_state(1444);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Theory_nameContextExt<'a>}

impl<'input> Theory_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Theory_nameContextAll<'input>> {
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
	self.get_token(Lower_word, 0)
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
			recog.base.match_token(Lower_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Creator_sourceContextExt<'a>}

impl<'input> Creator_sourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Creator_sourceContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1448);
			recog.base.match_token(T__30,&mut recog.err_handler)?;

			/*InvokeRule creator_name*/
			recog.base.set_state(1449);
			recog.creator_name()?;

			recog.base.set_state(1451);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule optional_info*/
				recog.base.set_state(1450);
				recog.optional_info()?;

				}
			}

			recog.base.set_state(1453);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Creator_nameContextExt<'a>}

impl<'input> Creator_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Creator_nameContextAll<'input>> {
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
antlr_rust::tid!{Optional_infoContextExt<'a>}

impl<'input> Optional_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Optional_infoContextAll<'input>> {
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
			recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Useful_infoContextExt<'a>}

impl<'input> Useful_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Useful_infoContextAll<'input>> {
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
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1461);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule info_items*/
					recog.base.set_state(1462);
					recog.info_items()?;

					recog.base.set_state(1463);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

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
antlr_rust::tid!{Info_itemsContextExt<'a>}

impl<'input> Info_itemsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Info_itemsContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1469);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Info_itemContextExt<'a>}

impl<'input> Info_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Info_itemContextAll<'input>> {
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
antlr_rust::tid!{Formula_itemContextExt<'a>}

impl<'input> Formula_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Formula_itemContextAll<'input>> {
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
			 T__31 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule description_item*/
					recog.base.set_state(1481);
					recog.description_item()?;

					}
				}

			 T__32 
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
antlr_rust::tid!{Description_itemContextExt<'a>}

impl<'input> Description_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Description_itemContextAll<'input>> {
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
			recog.base.match_token(T__31,&mut recog.err_handler)?;

			/*InvokeRule atomic_word*/
			recog.base.set_state(1486);
			recog.atomic_word()?;

			recog.base.set_state(1487);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Iquote_itemContextExt<'a>}

impl<'input> Iquote_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Iquote_itemContextAll<'input>> {
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
			recog.base.match_token(T__32,&mut recog.err_handler)?;

			/*InvokeRule atomic_word*/
			recog.base.set_state(1490);
			recog.atomic_word()?;

			recog.base.set_state(1491);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Inference_itemContextExt<'a>}

impl<'input> Inference_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Inference_itemContextAll<'input>> {
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
			 T__33 | Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule inference_status*/
					recog.base.set_state(1493);
					recog.inference_status()?;

					}
				}

			 T__34 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assumptions_record*/
					recog.base.set_state(1494);
					recog.assumptions_record()?;

					}
				}

			 T__36 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule new_symbol_record*/
					recog.base.set_state(1495);
					recog.new_symbol_record()?;

					}
				}

			 T__35 
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
antlr_rust::tid!{Inference_statusContextExt<'a>}

impl<'input> Inference_statusContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Inference_statusContextAll<'input>> {
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
			 T__33 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1499);
					recog.base.match_token(T__33,&mut recog.err_handler)?;

					/*InvokeRule status_value*/
					recog.base.set_state(1500);
					recog.status_value()?;

					recog.base.set_state(1501);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

			 Lower_word | Single_quoted 
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
antlr_rust::tid!{Status_valueContextExt<'a>}

impl<'input> Status_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Status_valueContextAll<'input>> {
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
	self.get_token(Lower_word, 0)
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
			recog.base.match_token(Lower_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Inference_infoContextExt<'a>}

impl<'input> Inference_infoContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Inference_infoContextAll<'input>> {
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
			recog.base.match_token(T__9,&mut recog.err_handler)?;

			/*InvokeRule atomic_word*/
			recog.base.set_state(1510);
			recog.atomic_word()?;

			recog.base.set_state(1511);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule general_list*/
			recog.base.set_state(1512);
			recog.general_list()?;

			recog.base.set_state(1513);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Assumptions_recordContextExt<'a>}

impl<'input> Assumptions_recordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Assumptions_recordContextAll<'input>> {
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
			recog.base.match_token(T__34,&mut recog.err_handler)?;

			recog.base.set_state(1516);
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			/*InvokeRule name_list*/
			recog.base.set_state(1517);
			recog.name_list()?;

			recog.base.set_state(1518);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(1519);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{RefutationContextExt<'a>}

impl<'input> RefutationContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RefutationContextAll<'input>> {
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
			recog.base.match_token(T__35,&mut recog.err_handler)?;

			/*InvokeRule file_source*/
			recog.base.set_state(1522);
			recog.file_source()?;

			recog.base.set_state(1523);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{New_symbol_recordContextExt<'a>}

impl<'input> New_symbol_recordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<New_symbol_recordContextAll<'input>> {
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
			recog.base.match_token(T__36,&mut recog.err_handler)?;

			/*InvokeRule atomic_word*/
			recog.base.set_state(1526);
			recog.atomic_word()?;

			recog.base.set_state(1527);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			recog.base.set_state(1528);
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			/*InvokeRule new_symbol_list*/
			recog.base.set_state(1529);
			recog.new_symbol_list()?;

			recog.base.set_state(1530);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			recog.base.set_state(1531);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{New_symbol_listContextExt<'a>}

impl<'input> New_symbol_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<New_symbol_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1534);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{Principal_symbolContextExt<'a>}

impl<'input> Principal_symbolContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Principal_symbolContextAll<'input>> {
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
			 Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule functor*/
					recog.base.set_state(1541);
					recog.functor()?;

					}
				}

			 Upper_word 
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
antlr_rust::tid!{IncludeContextExt<'a>}

impl<'input> IncludeContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IncludeContextAll<'input>> {
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1545);
			recog.base.match_token(T__37,&mut recog.err_handler)?;

			/*InvokeRule file_name*/
			recog.base.set_state(1546);
			recog.file_name()?;

			recog.base.set_state(1548);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__1 {
				{
				/*InvokeRule formula_selection*/
				recog.base.set_state(1547);
				recog.formula_selection()?;

				}
			}

			recog.base.set_state(1550);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Formula_selectionContextExt<'a>}

impl<'input> Formula_selectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Formula_selectionContextAll<'input>> {
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
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			recog.base.set_state(1553);
			recog.base.match_token(T__11,&mut recog.err_handler)?;

			/*InvokeRule name_list*/
			recog.base.set_state(1554);
			recog.name_list()?;

			recog.base.set_state(1555);
			recog.base.match_token(T__12,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Name_listContextExt<'a>}

impl<'input> Name_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Name_listContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1558);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{General_termContextExt<'a>}

impl<'input> General_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<General_termContextAll<'input>> {
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
					recog.base.match_token(T__13,&mut recog.err_handler)?;

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
antlr_rust::tid!{General_dataContextExt<'a>}

impl<'input> General_dataContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<General_dataContextAll<'input>> {
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
	self.get_token(Distinct_object, 0)
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
					recog.base.match_token(Distinct_object,&mut recog.err_handler)?;

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
antlr_rust::tid!{General_functionContextExt<'a>}

impl<'input> General_functionContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<General_functionContextAll<'input>> {
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
			recog.base.match_token(T__9,&mut recog.err_handler)?;

			/*InvokeRule general_terms*/
			recog.base.set_state(1583);
			recog.general_terms()?;

			recog.base.set_state(1584);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Formula_dataContextExt<'a>}

impl<'input> Formula_dataContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Formula_dataContextAll<'input>> {
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
			 T__38 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1586);
					recog.base.match_token(T__38,&mut recog.err_handler)?;

					/*InvokeRule thf_formula*/
					recog.base.set_state(1587);
					recog.thf_formula()?;

					recog.base.set_state(1588);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

			 T__39 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1590);
					recog.base.match_token(T__39,&mut recog.err_handler)?;

					/*InvokeRule tff_formula*/
					recog.base.set_state(1591);
					recog.tff_formula()?;

					recog.base.set_state(1592);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

			 T__40 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1594);
					recog.base.match_token(T__40,&mut recog.err_handler)?;

					/*InvokeRule fof_formula*/
					recog.base.set_state(1595);
					recog.fof_formula()?;

					recog.base.set_state(1596);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

			 T__41 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1598);
					recog.base.match_token(T__41,&mut recog.err_handler)?;

					/*InvokeRule cnf_formula*/
					recog.base.set_state(1599);
					recog.cnf_formula()?;

					recog.base.set_state(1600);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

					}
				}

			 T__42 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1602);
					recog.base.match_token(T__42,&mut recog.err_handler)?;

					/*InvokeRule fof_term*/
					recog.base.set_state(1603);
					recog.fof_term()?;

					recog.base.set_state(1604);
					recog.base.match_token(T__10,&mut recog.err_handler)?;

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
antlr_rust::tid!{General_listContextExt<'a>}

impl<'input> General_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<General_listContextAll<'input>> {
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
			 T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1608);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}

			 T__11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1609);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					/*InvokeRule general_terms*/
					recog.base.set_state(1610);
					recog.general_terms()?;

					recog.base.set_state(1611);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

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
antlr_rust::tid!{General_termsContextExt<'a>}

impl<'input> General_termsContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<General_termsContextAll<'input>> {
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
		let mut _la: isize = -1;
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
			while _la==T__1 {
				{
				{
				recog.base.set_state(1616);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

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
antlr_rust::tid!{NameContextExt<'a>}

impl<'input> NameContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NameContextAll<'input>> {
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
	self.get_token(Integer, 0)
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
			 Lower_word | Single_quoted 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule atomic_word*/
					recog.base.set_state(1623);
					recog.atomic_word()?;

					}
				}

			 Integer 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1624);
					recog.base.match_token(Integer,&mut recog.err_handler)?;

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
antlr_rust::tid!{Atomic_wordContextExt<'a>}

impl<'input> Atomic_wordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Atomic_wordContextAll<'input>> {
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
	self.get_token(Lower_word, 0)
}
/// Retrieves first TerminalNode corresponding to token Single_quoted
/// Returns `None` if there is no child corresponding to token Single_quoted
fn Single_quoted(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Single_quoted, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1627);
			_la = recog.base.input.la(1);
			if { !(_la==Lower_word || _la==Single_quoted) } {
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
antlr_rust::tid!{Atomic_defined_wordContextExt<'a>}

impl<'input> Atomic_defined_wordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Atomic_defined_wordContextAll<'input>> {
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
	self.get_token(Dollar_word, 0)
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
			recog.base.match_token(Dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{Atomic_system_wordContextExt<'a>}

impl<'input> Atomic_system_wordContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Atomic_system_wordContextAll<'input>> {
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
	self.get_token(Dollar_dollar_word, 0)
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
			recog.base.match_token(Dollar_dollar_word,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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
antlr_rust::tid!{NumberContextExt<'a>}

impl<'input> NumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberContextAll<'input>> {
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
	self.get_token(Integer, 0)
}
/// Retrieves first TerminalNode corresponding to token Rational
/// Returns `None` if there is no child corresponding to token Rational
fn Rational(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Rational, 0)
}
/// Retrieves first TerminalNode corresponding to token Real
/// Returns `None` if there is no child corresponding to token Real
fn Real(&self) -> Option<Rc<TerminalNode<'input,tptp_v7_0_0_0ParserContextType>>> where Self:Sized{
	self.get_token(Real, 0)
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1633);
			_la = recog.base.input.la(1);
			if { !(((((_la - 74)) & !0x3f) == 0 && ((1usize << (_la - 74)) & ((1usize << (Real - 74)) | (1usize << (Rational - 74)) | (1usize << (Integer - 74)))) != 0)) } {
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
antlr_rust::tid!{File_nameContextExt<'a>}

impl<'input> File_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn tptp_v7_0_0_0ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<File_nameContextAll<'input>> {
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
	self.get_token(Single_quoted, 0)
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
			recog.base.match_token(Single_quoted,&mut recog.err_handler)?;

			}
			Ok(())
		})();
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

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x65\u{668}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x04\x5c\x09\x5c\x04\x5d\x09\x5d\x04\x5e\x09\x5e\x04\x5f\x09\
	\x5f\x04\x60\x09\x60\x04\x61\x09\x61\x04\x62\x09\x62\x04\x63\x09\x63\x04\
	\x64\x09\x64\x04\x65\x09\x65\x04\x66\x09\x66\x04\x67\x09\x67\x04\x68\x09\
	\x68\x04\x69\x09\x69\x04\x6a\x09\x6a\x04\x6b\x09\x6b\x04\x6c\x09\x6c\x04\
	\x6d\x09\x6d\x04\x6e\x09\x6e\x04\x6f\x09\x6f\x04\x70\x09\x70\x04\x71\x09\
	\x71\x04\x72\x09\x72\x04\x73\x09\x73\x04\x74\x09\x74\x04\x75\x09\x75\x04\
	\x76\x09\x76\x04\x77\x09\x77\x04\x78\x09\x78\x04\x79\x09\x79\x04\x7a\x09\
	\x7a\x04\x7b\x09\x7b\x04\x7c\x09\x7c\x04\x7d\x09\x7d\x04\x7e\x09\x7e\x04\
	\x7f\x09\x7f\x04\u{80}\x09\u{80}\x04\u{81}\x09\u{81}\x04\u{82}\x09\u{82}\
	\x04\u{83}\x09\u{83}\x04\u{84}\x09\u{84}\x04\u{85}\x09\u{85}\x04\u{86}\x09\
	\u{86}\x04\u{87}\x09\u{87}\x04\u{88}\x09\u{88}\x04\u{89}\x09\u{89}\x04\u{8a}\
	\x09\u{8a}\x04\u{8b}\x09\u{8b}\x04\u{8c}\x09\u{8c}\x04\u{8d}\x09\u{8d}\x04\
	\u{8e}\x09\u{8e}\x04\u{8f}\x09\u{8f}\x04\u{90}\x09\u{90}\x04\u{91}\x09\u{91}\
	\x04\u{92}\x09\u{92}\x04\u{93}\x09\u{93}\x04\u{94}\x09\u{94}\x04\u{95}\x09\
	\u{95}\x04\u{96}\x09\u{96}\x04\u{97}\x09\u{97}\x04\u{98}\x09\u{98}\x04\u{99}\
	\x09\u{99}\x04\u{9a}\x09\u{9a}\x04\u{9b}\x09\u{9b}\x04\u{9c}\x09\u{9c}\x04\
	\u{9d}\x09\u{9d}\x04\u{9e}\x09\u{9e}\x04\u{9f}\x09\u{9f}\x04\u{a0}\x09\u{a0}\
	\x04\u{a1}\x09\u{a1}\x04\u{a2}\x09\u{a2}\x04\u{a3}\x09\u{a3}\x04\u{a4}\x09\
	\u{a4}\x04\u{a5}\x09\u{a5}\x04\u{a6}\x09\u{a6}\x04\u{a7}\x09\u{a7}\x04\u{a8}\
	\x09\u{a8}\x04\u{a9}\x09\u{a9}\x04\u{aa}\x09\u{aa}\x04\u{ab}\x09\u{ab}\x04\
	\u{ac}\x09\u{ac}\x04\u{ad}\x09\u{ad}\x04\u{ae}\x09\u{ae}\x04\u{af}\x09\u{af}\
	\x04\u{b0}\x09\u{b0}\x04\u{b1}\x09\u{b1}\x04\u{b2}\x09\u{b2}\x04\u{b3}\x09\
	\u{b3}\x04\u{b4}\x09\u{b4}\x04\u{b5}\x09\u{b5}\x04\u{b6}\x09\u{b6}\x04\u{b7}\
	\x09\u{b7}\x04\u{b8}\x09\u{b8}\x04\u{b9}\x09\u{b9}\x04\u{ba}\x09\u{ba}\x04\
	\u{bb}\x09\u{bb}\x04\u{bc}\x09\u{bc}\x04\u{bd}\x09\u{bd}\x04\u{be}\x09\u{be}\
	\x04\u{bf}\x09\u{bf}\x04\u{c0}\x09\u{c0}\x04\u{c1}\x09\u{c1}\x04\u{c2}\x09\
	\u{c2}\x04\u{c3}\x09\u{c3}\x04\u{c4}\x09\u{c4}\x04\u{c5}\x09\u{c5}\x04\u{c6}\
	\x09\u{c6}\x04\u{c7}\x09\u{c7}\x04\u{c8}\x09\u{c8}\x04\u{c9}\x09\u{c9}\x04\
	\u{ca}\x09\u{ca}\x03\x02\x07\x02\u{196}\x0a\x02\x0c\x02\x0e\x02\u{199}\x0b\
	\x02\x03\x02\x03\x02\x03\x03\x03\x03\x05\x03\u{19f}\x0a\x03\x03\x04\x03\
	\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x05\x04\u{1a8}\x0a\x04\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{1b1}\x0a\
	\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\
	\x07\x03\x07\x03\x07\x05\x07\u{1be}\x0a\x07\x03\x07\x03\x07\x03\x08\x03\
	\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x05\x08\u{1c9}\x0a\x08\x03\
	\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x05\
	\x09\u{1d4}\x0a\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x03\x0a\x05\x0a\u{1df}\x0a\x0a\x03\x0a\x03\x0a\x03\x0b\x03\
	\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{1ea}\x0a\x0b\x03\
	\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\
	\x0c\u{1f5}\x0a\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{1fc}\
	\x0a\x0d\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x05\x0f\u{202}\x0a\x0f\x03\x10\
	\x03\x10\x03\x10\x03\x10\x05\x10\u{208}\x0a\x10\x03\x11\x03\x11\x03\x11\
	\x05\x11\u{20d}\x0a\x11\x03\x12\x03\x12\x03\x12\x03\x12\x03\x13\x03\x13\
	\x03\x13\x05\x13\u{216}\x0a\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\
	\x03\x14\x03\x14\x03\x14\x07\x14\u{220}\x0a\x14\x0c\x14\x0e\x14\u{223}\x0b\
	\x14\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x07\
	\x15\u{22d}\x0a\x15\x0c\x15\x0e\x15\u{230}\x0b\x15\x03\x16\x03\x16\x03\x16\
	\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x07\x16\u{23a}\x0a\x16\x0c\x16\
	\x0e\x16\u{23d}\x0b\x16\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\
	\x03\x17\x03\x17\x03\x17\x03\x17\x05\x17\u{249}\x0a\x17\x03\x18\x03\x18\
	\x03\x18\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\
	\x03\x1a\x07\x1a\u{257}\x0a\x1a\x0c\x1a\x0e\x1a\u{25a}\x0b\x1a\x03\x1b\x03\
	\x1b\x05\x1b\u{25e}\x0a\x1b\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1d\x03\
	\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{26d}\
	\x0a\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\
	\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x05\x1f\
	\u{27f}\x0a\x1f\x03\x20\x03\x20\x03\x20\x05\x20\u{284}\x0a\x20\x03\x21\x03\
	\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x21\x03\x22\x03\x22\x03\
	\x22\x03\x22\x03\x22\x03\x22\x03\x23\x03\x23\x03\x24\x03\x24\x03\x24\x03\
	\x24\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x05\x25\u{29f}\x0a\x25\x03\
	\x26\x03\x26\x03\x26\x03\x26\x03\x27\x03\x27\x03\x27\x05\x27\u{2a8}\x0a\
	\x27\x03\x28\x03\x28\x03\x29\x03\x29\x03\x2a\x03\x2a\x03\x2a\x05\x2a\u{2b1}\
	\x0a\x2a\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\
	\x05\x2b\u{2bb}\x0a\x2b\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\
	\x03\x2c\x03\x2c\x07\x2c\u{2c5}\x0a\x2c\x0c\x2c\x0e\x2c\u{2c8}\x0b\x2c\x03\
	\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x07\x2d\u{2d2}\
	\x0a\x2d\x0c\x2d\x0e\x2d\u{2d5}\x0b\x2d\x03\x2e\x03\x2e\x03\x2e\x03\x2e\
	\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x05\x2e\u{2df}\x0a\x2e\x03\x2f\x03\x2f\
	\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x05\x2f\
	\u{2eb}\x0a\x2f\x03\x30\x03\x30\x03\x30\x07\x30\u{2f0}\x0a\x30\x0c\x30\x0e\
	\x30\u{2f3}\x0b\x30\x03\x31\x03\x31\x05\x31\u{2f7}\x0a\x31\x03\x32\x03\x32\
	\x03\x33\x03\x33\x03\x33\x05\x33\u{2fe}\x0a\x33\x03\x34\x03\x34\x03\x34\
	\x05\x34\u{303}\x0a\x34\x03\x35\x03\x35\x05\x35\u{307}\x0a\x35\x03\x36\x03\
	\x36\x03\x36\x03\x36\x03\x37\x03\x37\x05\x37\u{30f}\x0a\x37\x03\x38\x03\
	\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x07\x38\u{319}\x0a\
	\x38\x0c\x38\x0e\x38\u{31c}\x0b\x38\x03\x39\x03\x39\x03\x39\x03\x39\x03\
	\x39\x03\x39\x03\x39\x03\x39\x07\x39\u{326}\x0a\x39\x0c\x39\x0e\x39\u{329}\
	\x0b\x39\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3a\
	\x03\x3a\x05\x3a\u{334}\x0a\x3a\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x03\x3b\
	\x03\x3b\x03\x3b\x03\x3c\x03\x3c\x03\x3c\x07\x3c\u{340}\x0a\x3c\x0c\x3c\
	\x0e\x3c\u{343}\x0b\x3c\x03\x3d\x03\x3d\x05\x3d\u{347}\x0a\x3d\x03\x3e\x03\
	\x3e\x03\x3e\x03\x3e\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x05\x3f\u{351}\x0a\
	\x3f\x03\x40\x03\x40\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\
	\x41\x03\x41\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x03\
	\x42\x03\x42\x03\x42\x03\x42\x03\x42\x05\x42\u{369}\x0a\x42\x03\x43\x03\
	\x43\x03\x43\x03\x43\x03\x43\x05\x43\u{370}\x0a\x43\x03\x44\x03\x44\x03\
	\x44\x07\x44\u{375}\x0a\x44\x0c\x44\x0e\x44\u{378}\x0b\x44\x03\x45\x03\x45\
	\x03\x45\x03\x45\x03\x45\x03\x45\x03\x45\x03\x45\x05\x45\u{382}\x0a\x45\
	\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x05\x46\
	\u{38c}\x0a\x46\x03\x47\x03\x47\x03\x47\x03\x47\x03\x47\x05\x47\u{393}\x0a\
	\x47\x03\x48\x03\x48\x03\x48\x07\x48\u{398}\x0a\x48\x0c\x48\x0e\x48\u{39b}\
	\x0b\x48\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\
	\x05\x49\u{3a5}\x0a\x49\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\
	\x03\x4a\x03\x4a\x05\x4a\u{3af}\x0a\x4a\x03\x4b\x03\x4b\x03\x4b\x03\x4b\
	\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x05\x4b\u{3b9}\x0a\x4b\x03\x4c\x03\x4c\
	\x03\x4c\x03\x4c\x03\x4c\x05\x4c\u{3c0}\x0a\x4c\x03\x4d\x03\x4d\x03\x4d\
	\x07\x4d\u{3c5}\x0a\x4d\x0c\x4d\x0e\x4d\u{3c8}\x0b\x4d\x03\x4e\x03\x4e\x03\
	\x4e\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x05\x4e\u{3d2}\x0a\x4e\x03\
	\x4f\x03\x4f\x03\x4f\x03\x4f\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\
	\x50\x03\x50\x05\x50\u{3df}\x0a\x50\x03\x51\x03\x51\x03\x51\x03\x51\x03\
	\x51\x03\x51\x03\x51\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x05\x52\u{3ed}\
	\x0a\x52\x03\x53\x03\x53\x03\x53\x03\x53\x03\x53\x05\x53\u{3f4}\x0a\x53\
	\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x05\x54\
	\u{3fe}\x0a\x54\x03\x55\x03\x55\x03\x55\x07\x55\u{403}\x0a\x55\x0c\x55\x0e\
	\x55\u{406}\x0b\x55\x03\x56\x03\x56\x03\x56\x03\x56\x03\x57\x03\x57\x03\
	\x57\x03\x57\x03\x57\x03\x57\x03\x57\x03\x57\x07\x57\u{414}\x0a\x57\x0c\
	\x57\x0e\x57\u{417}\x0b\x57\x03\x58\x03\x58\x05\x58\u{41b}\x0a\x58\x03\x59\
	\x03\x59\x05\x59\u{41f}\x0a\x59\x03\x5a\x03\x5a\x03\x5a\x03\x5a\x03\x5a\
	\x03\x5a\x03\x5a\x03\x5b\x03\x5b\x05\x5b\u{42a}\x0a\x5b\x03\x5c\x03\x5c\
	\x05\x5c\u{42e}\x0a\x5c\x03\x5d\x03\x5d\x05\x5d\u{432}\x0a\x5d\x03\x5e\x03\
	\x5e\x03\x5e\x03\x5e\x03\x5f\x03\x5f\x05\x5f\u{43a}\x0a\x5f\x03\x60\x03\
	\x60\x03\x60\x03\x60\x03\x60\x03\x60\x03\x60\x03\x60\x07\x60\u{444}\x0a\
	\x60\x0c\x60\x0e\x60\u{447}\x0b\x60\x03\x61\x03\x61\x03\x61\x03\x61\x03\
	\x61\x03\x61\x03\x61\x03\x61\x07\x61\u{451}\x0a\x61\x0c\x61\x0e\x61\u{454}\
	\x0b\x61\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x03\x62\x05\x62\
	\u{45d}\x0a\x62\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\
	\x03\x64\x03\x64\x03\x64\x07\x64\u{469}\x0a\x64\x0c\x64\x0e\x64\u{46c}\x0b\
	\x64\x03\x65\x03\x65\x03\x65\x03\x65\x05\x65\u{472}\x0a\x65\x03\x66\x03\
	\x66\x03\x66\x03\x66\x03\x67\x03\x67\x03\x67\x05\x67\u{47b}\x0a\x67\x03\
	\x68\x03\x68\x03\x69\x03\x69\x05\x69\u{481}\x0a\x69\x03\x6a\x03\x6a\x03\
	\x6b\x03\x6b\x03\x6b\x03\x6b\x03\x6c\x03\x6c\x03\x6d\x03\x6d\x03\x6d\x03\
	\x6d\x03\x6d\x03\x6d\x05\x6d\u{491}\x0a\x6d\x03\x6e\x03\x6e\x05\x6e\u{495}\
	\x0a\x6e\x03\x6f\x03\x6f\x03\x70\x03\x70\x03\x70\x03\x70\x03\x70\x03\x70\
	\x05\x70\u{49f}\x0a\x70\x03\x71\x03\x71\x03\x71\x03\x71\x03\x71\x03\x71\
	\x05\x71\u{4a7}\x0a\x71\x03\x72\x03\x72\x03\x72\x07\x72\u{4ac}\x0a\x72\x0c\
	\x72\x0e\x72\u{4af}\x0b\x72\x03\x73\x03\x73\x03\x73\x03\x73\x03\x73\x05\
	\x73\u{4b6}\x0a\x73\x03\x74\x03\x74\x03\x74\x05\x74\u{4bb}\x0a\x74\x03\x75\
	\x03\x75\x03\x75\x03\x75\x03\x75\x03\x75\x03\x75\x03\x75\x03\x76\x03\x76\
	\x03\x76\x03\x76\x03\x76\x03\x76\x03\x76\x03\x76\x03\x76\x03\x76\x03\x76\
	\x03\x76\x05\x76\u{4d1}\x0a\x76\x03\x77\x03\x77\x03\x77\x03\x77\x03\x77\
	\x05\x77\u{4d8}\x0a\x77\x03\x78\x03\x78\x03\x78\x03\x78\x03\x78\x03\x78\
	\x03\x78\x03\x78\x05\x78\u{4e2}\x0a\x78\x03\x79\x03\x79\x03\x79\x03\x79\
	\x03\x79\x05\x79\u{4e9}\x0a\x79\x03\x7a\x03\x7a\x03\x7a\x07\x7a\u{4ee}\x0a\
	\x7a\x0c\x7a\x0e\x7a\u{4f1}\x0b\x7a\x03\x7b\x03\x7b\x03\x7b\x03\x7b\x03\
	\x7b\x05\x7b\u{4f8}\x0a\x7b\x03\x7c\x03\x7c\x03\x7c\x03\x7c\x03\x7c\x03\
	\x7c\x07\x7c\u{500}\x0a\x7c\x0c\x7c\x0e\x7c\u{503}\x0b\x7c\x03\x7d\x03\x7d\
	\x03\x7d\x03\x7d\x05\x7d\u{509}\x0a\x7d\x03\x7e\x03\x7e\x03\x7e\x05\x7e\
	\u{50e}\x0a\x7e\x03\x7f\x03\x7f\x03\u{80}\x03\u{80}\x03\u{81}\x03\u{81}\
	\x03\u{81}\x03\u{81}\x05\u{81}\u{518}\x0a\u{81}\x03\u{82}\x03\u{82}\x05\
	\u{82}\u{51c}\x0a\u{82}\x03\u{83}\x03\u{83}\x03\u{84}\x03\u{84}\x05\u{84}\
	\u{522}\x0a\u{84}\x03\u{85}\x03\u{85}\x03\u{86}\x03\u{86}\x03\u{87}\x03\
	\u{87}\x03\u{88}\x03\u{88}\x03\u{89}\x03\u{89}\x03\u{8a}\x03\u{8a}\x03\u{8b}\
	\x03\u{8b}\x03\u{8c}\x03\u{8c}\x03\u{8d}\x03\u{8d}\x05\u{8d}\u{536}\x0a\
	\u{8d}\x03\u{8e}\x03\u{8e}\x05\u{8e}\u{53a}\x0a\u{8e}\x03\u{8f}\x03\u{8f}\
	\x03\u{90}\x03\u{90}\x03\u{91}\x03\u{91}\x03\u{92}\x03\u{92}\x03\u{93}\x03\
	\u{93}\x03\u{94}\x03\u{94}\x03\u{95}\x03\u{95}\x03\u{96}\x03\u{96}\x03\u{97}\
	\x03\u{97}\x03\u{98}\x03\u{98}\x05\u{98}\u{550}\x0a\u{98}\x03\u{99}\x03\
	\u{99}\x03\u{9a}\x03\u{9a}\x03\u{9a}\x03\u{9a}\x03\u{9a}\x03\u{9a}\x03\u{9a}\
	\x03\u{9a}\x05\u{9a}\u{55c}\x0a\u{9a}\x03\u{9b}\x03\u{9b}\x03\u{9b}\x07\
	\u{9b}\u{561}\x0a\u{9b}\x0c\u{9b}\x0e\u{9b}\u{564}\x0b\u{9b}\x03\u{9c}\x03\
	\u{9c}\x05\u{9c}\u{568}\x0a\u{9c}\x03\u{9d}\x03\u{9d}\x03\u{9d}\x03\u{9d}\
	\x03\u{9d}\x03\u{9d}\x03\u{9d}\x03\u{9d}\x03\u{9e}\x03\u{9e}\x03\u{9f}\x03\
	\u{9f}\x03\u{9f}\x03\u{9f}\x03\u{9f}\x05\u{9f}\u{579}\x0a\u{9f}\x03\u{a0}\
	\x03\u{a0}\x03\u{a0}\x07\u{a0}\u{57e}\x0a\u{a0}\x0c\u{a0}\x0e\u{a0}\u{581}\
	\x0b\u{a0}\x03\u{a1}\x03\u{a1}\x05\u{a1}\u{585}\x0a\u{a1}\x03\u{a2}\x03\
	\u{a2}\x03\u{a2}\x03\u{a3}\x03\u{a3}\x03\u{a3}\x05\u{a3}\u{58d}\x0a\u{a3}\
	\x03\u{a3}\x03\u{a3}\x03\u{a4}\x03\u{a4}\x03\u{a5}\x03\u{a5}\x03\u{a5}\x05\
	\u{a5}\u{596}\x0a\u{a5}\x03\u{a6}\x03\u{a6}\x03\u{a6}\x05\u{a6}\u{59b}\x0a\
	\u{a6}\x03\u{a6}\x03\u{a6}\x03\u{a7}\x03\u{a7}\x03\u{a7}\x03\u{a8}\x03\u{a8}\
	\x03\u{a8}\x05\u{a8}\u{5a5}\x0a\u{a8}\x03\u{a8}\x03\u{a8}\x03\u{a9}\x03\
	\u{a9}\x03\u{aa}\x03\u{aa}\x03\u{aa}\x05\u{aa}\u{5ae}\x0a\u{aa}\x03\u{aa}\
	\x03\u{aa}\x03\u{ab}\x03\u{ab}\x03\u{ac}\x03\u{ac}\x03\u{ac}\x03\u{ad}\x03\
	\u{ad}\x03\u{ad}\x03\u{ad}\x03\u{ad}\x03\u{ad}\x05\u{ad}\u{5bd}\x0a\u{ad}\
	\x03\u{ae}\x03\u{ae}\x03\u{ae}\x07\u{ae}\u{5c2}\x0a\u{ae}\x0c\u{ae}\x0e\
	\u{ae}\u{5c5}\x0b\u{ae}\x03\u{af}\x03\u{af}\x03\u{af}\x05\u{af}\u{5ca}\x0a\
	\u{af}\x03\u{b0}\x03\u{b0}\x05\u{b0}\u{5ce}\x0a\u{b0}\x03\u{b1}\x03\u{b1}\
	\x03\u{b1}\x03\u{b1}\x03\u{b2}\x03\u{b2}\x03\u{b2}\x03\u{b2}\x03\u{b3}\x03\
	\u{b3}\x03\u{b3}\x03\u{b3}\x05\u{b3}\u{5dc}\x0a\u{b3}\x03\u{b4}\x03\u{b4}\
	\x03\u{b4}\x03\u{b4}\x03\u{b4}\x05\u{b4}\u{5e3}\x0a\u{b4}\x03\u{b5}\x03\
	\u{b5}\x03\u{b6}\x03\u{b6}\x03\u{b6}\x03\u{b6}\x03\u{b6}\x03\u{b6}\x03\u{b6}\
	\x03\u{b7}\x03\u{b7}\x03\u{b7}\x03\u{b7}\x03\u{b7}\x03\u{b7}\x03\u{b8}\x03\
	\u{b8}\x03\u{b8}\x03\u{b8}\x03\u{b9}\x03\u{b9}\x03\u{b9}\x03\u{b9}\x03\u{b9}\
	\x03\u{b9}\x03\u{b9}\x03\u{b9}\x03\u{ba}\x03\u{ba}\x03\u{ba}\x07\u{ba}\u{603}\
	\x0a\u{ba}\x0c\u{ba}\x0e\u{ba}\u{606}\x0b\u{ba}\x03\u{bb}\x03\u{bb}\x05\
	\u{bb}\u{60a}\x0a\u{bb}\x03\u{bc}\x03\u{bc}\x03\u{bc}\x05\u{bc}\u{60f}\x0a\
	\u{bc}\x03\u{bc}\x03\u{bc}\x03\u{bd}\x03\u{bd}\x03\u{bd}\x03\u{bd}\x03\u{bd}\
	\x03\u{be}\x03\u{be}\x03\u{be}\x07\u{be}\u{61b}\x0a\u{be}\x0c\u{be}\x0e\
	\u{be}\u{61e}\x0b\u{be}\x03\u{bf}\x03\u{bf}\x03\u{bf}\x03\u{bf}\x03\u{bf}\
	\x03\u{bf}\x05\u{bf}\u{626}\x0a\u{bf}\x03\u{c0}\x03\u{c0}\x03\u{c0}\x03\
	\u{c0}\x03\u{c0}\x03\u{c0}\x05\u{c0}\u{62e}\x0a\u{c0}\x03\u{c1}\x03\u{c1}\
	\x03\u{c1}\x03\u{c1}\x03\u{c1}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\
	\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\
	\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\u{c2}\x03\
	\u{c2}\x05\u{c2}\u{649}\x0a\u{c2}\x03\u{c3}\x03\u{c3}\x03\u{c3}\x03\u{c3}\
	\x03\u{c3}\x05\u{c3}\u{650}\x0a\u{c3}\x03\u{c4}\x03\u{c4}\x03\u{c4}\x07\
	\u{c4}\u{655}\x0a\u{c4}\x0c\u{c4}\x0e\u{c4}\u{658}\x0b\u{c4}\x03\u{c5}\x03\
	\u{c5}\x05\u{c5}\u{65c}\x0a\u{c5}\x03\u{c6}\x03\u{c6}\x03\u{c7}\x03\u{c7}\
	\x03\u{c8}\x03\u{c8}\x03\u{c9}\x03\u{c9}\x03\u{ca}\x03\u{ca}\x03\u{ca}\x02\
	\x0d\x26\x28\x2a\x56\x58\x6e\x70\u{ac}\u{be}\u{c0}\u{f6}\u{cb}\x02\x04\x06\
	\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\
	\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\
	\x50\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\
	\x74\x76\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\
	\u{90}\u{92}\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\
	\u{a8}\u{aa}\u{ac}\u{ae}\u{b0}\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\u{bc}\u{be}\
	\u{c0}\u{c2}\u{c4}\u{c6}\u{c8}\u{ca}\u{cc}\u{ce}\u{d0}\u{d2}\u{d4}\u{d6}\
	\u{d8}\u{da}\u{dc}\u{de}\u{e0}\u{e2}\u{e4}\u{e6}\u{e8}\u{ea}\u{ec}\u{ee}\
	\u{f0}\u{f2}\u{f4}\u{f6}\u{f8}\u{fa}\u{fc}\u{fe}\u{100}\u{102}\u{104}\u{106}\
	\u{108}\u{10a}\u{10c}\u{10e}\u{110}\u{112}\u{114}\u{116}\u{118}\u{11a}\u{11c}\
	\u{11e}\u{120}\u{122}\u{124}\u{126}\u{128}\u{12a}\u{12c}\u{12e}\u{130}\u{132}\
	\u{134}\u{136}\u{138}\u{13a}\u{13c}\u{13e}\u{140}\u{142}\u{144}\u{146}\u{148}\
	\u{14a}\u{14c}\u{14e}\u{150}\u{152}\u{154}\u{156}\u{158}\u{15a}\u{15c}\u{15e}\
	\u{160}\u{162}\u{164}\u{166}\u{168}\u{16a}\u{16c}\u{16e}\u{170}\u{172}\u{174}\
	\u{176}\u{178}\u{17a}\u{17c}\u{17e}\u{180}\u{182}\u{184}\u{186}\u{188}\u{18a}\
	\u{18c}\u{18e}\u{190}\u{192}\x02\x0b\x05\x02\x3f\x3f\x41\x41\x43\x43\x04\
	\x02\x38\x38\x3d\x3d\x07\x02\x37\x37\x3c\x3c\x40\x40\x42\x42\x44\x44\x04\
	\x02\x3b\x3b\x3e\x3e\x03\x02\x30\x35\x03\x02\x2e\x2f\x04\x02\x3a\x3a\x46\
	\x46\x03\x02\x60\x61\x05\x02\x4c\x4c\x4f\x4f\x52\x52\x02\u{657}\x02\u{197}\
	\x03\x02\x02\x02\x04\u{19e}\x03\x02\x02\x02\x06\u{1a7}\x03\x02\x02\x02\x08\
	\u{1a9}\x03\x02\x02\x02\x0a\u{1b4}\x03\x02\x02\x02\x0c\u{1b6}\x03\x02\x02\
	\x02\x0e\u{1c1}\x03\x02\x02\x02\x10\u{1cc}\x03\x02\x02\x02\x12\u{1d7}\x03\
	\x02\x02\x02\x14\u{1e2}\x03\x02\x02\x02\x16\u{1ed}\x03\x02\x02\x02\x18\u{1f8}\
	\x03\x02\x02\x02\x1a\u{1fd}\x03\x02\x02\x02\x1c\u{201}\x03\x02\x02\x02\x1e\
	\u{207}\x03\x02\x02\x02\x20\u{20c}\x03\x02\x02\x02\x22\u{20e}\x03\x02\x02\
	\x02\x24\u{215}\x03\x02\x02\x02\x26\u{217}\x03\x02\x02\x02\x28\u{224}\x03\
	\x02\x02\x02\x2a\u{231}\x03\x02\x02\x02\x2c\u{248}\x03\x02\x02\x02\x2e\u{24a}\
	\x03\x02\x02\x02\x30\u{24d}\x03\x02\x02\x02\x32\u{253}\x03\x02\x02\x02\x34\
	\u{25d}\x03\x02\x02\x02\x36\u{25f}\x03\x02\x02\x02\x38\u{263}\x03\x02\x02\
	\x02\x3a\u{26c}\x03\x02\x02\x02\x3c\u{27e}\x03\x02\x02\x02\x3e\u{283}\x03\
	\x02\x02\x02\x40\u{285}\x03\x02\x02\x02\x42\u{28d}\x03\x02\x02\x02\x44\u{293}\
	\x03\x02\x02\x02\x46\u{295}\x03\x02\x02\x02\x48\u{29e}\x03\x02\x02\x02\x4a\
	\u{2a0}\x03\x02\x02\x02\x4c\u{2a7}\x03\x02\x02\x02\x4e\u{2a9}\x03\x02\x02\
	\x02\x50\u{2ab}\x03\x02\x02\x02\x52\u{2b0}\x03\x02\x02\x02\x54\u{2ba}\x03\
	\x02\x02\x02\x56\u{2bc}\x03\x02\x02\x02\x58\u{2c9}\x03\x02\x02\x02\x5a\u{2de}\
	\x03\x02\x02\x02\x5c\u{2ea}\x03\x02\x02\x02\x5e\u{2ec}\x03\x02\x02\x02\x60\
	\u{2f6}\x03\x02\x02\x02\x62\u{2f8}\x03\x02\x02\x02\x64\u{2fd}\x03\x02\x02\
	\x02\x66\u{302}\x03\x02\x02\x02\x68\u{306}\x03\x02\x02\x02\x6a\u{308}\x03\
	\x02\x02\x02\x6c\u{30e}\x03\x02\x02\x02\x6e\u{310}\x03\x02\x02\x02\x70\u{31d}\
	\x03\x02\x02\x02\x72\u{333}\x03\x02\x02\x02\x74\u{335}\x03\x02\x02\x02\x76\
	\u{33c}\x03\x02\x02\x02\x78\u{346}\x03\x02\x02\x02\x7a\u{348}\x03\x02\x02\
	\x02\x7c\u{350}\x03\x02\x02\x02\x7e\u{352}\x03\x02\x02\x02\u{80}\u{354}\
	\x03\x02\x02\x02\u{82}\u{368}\x03\x02\x02\x02\u{84}\u{36f}\x03\x02\x02\x02\
	\u{86}\u{371}\x03\x02\x02\x02\u{88}\u{381}\x03\x02\x02\x02\u{8a}\u{38b}\
	\x03\x02\x02\x02\u{8c}\u{392}\x03\x02\x02\x02\u{8e}\u{394}\x03\x02\x02\x02\
	\u{90}\u{3a4}\x03\x02\x02\x02\u{92}\u{3ae}\x03\x02\x02\x02\u{94}\u{3b8}\
	\x03\x02\x02\x02\u{96}\u{3bf}\x03\x02\x02\x02\u{98}\u{3c1}\x03\x02\x02\x02\
	\u{9a}\u{3d1}\x03\x02\x02\x02\u{9c}\u{3d3}\x03\x02\x02\x02\u{9e}\u{3de}\
	\x03\x02\x02\x02\u{a0}\u{3e0}\x03\x02\x02\x02\u{a2}\u{3ec}\x03\x02\x02\x02\
	\u{a4}\u{3f3}\x03\x02\x02\x02\u{a6}\u{3fd}\x03\x02\x02\x02\u{a8}\u{3ff}\
	\x03\x02\x02\x02\u{aa}\u{407}\x03\x02\x02\x02\u{ac}\u{40b}\x03\x02\x02\x02\
	\u{ae}\u{41a}\x03\x02\x02\x02\u{b0}\u{41e}\x03\x02\x02\x02\u{b2}\u{420}\
	\x03\x02\x02\x02\u{b4}\u{429}\x03\x02\x02\x02\u{b6}\u{42d}\x03\x02\x02\x02\
	\u{b8}\u{431}\x03\x02\x02\x02\u{ba}\u{433}\x03\x02\x02\x02\u{bc}\u{439}\
	\x03\x02\x02\x02\u{be}\u{43b}\x03\x02\x02\x02\u{c0}\u{448}\x03\x02\x02\x02\
	\u{c2}\u{45c}\x03\x02\x02\x02\u{c4}\u{45e}\x03\x02\x02\x02\u{c6}\u{465}\
	\x03\x02\x02\x02\u{c8}\u{471}\x03\x02\x02\x02\u{ca}\u{473}\x03\x02\x02\x02\
	\u{cc}\u{47a}\x03\x02\x02\x02\u{ce}\u{47c}\x03\x02\x02\x02\u{d0}\u{480}\
	\x03\x02\x02\x02\u{d2}\u{482}\x03\x02\x02\x02\u{d4}\u{484}\x03\x02\x02\x02\
	\u{d6}\u{488}\x03\x02\x02\x02\u{d8}\u{490}\x03\x02\x02\x02\u{da}\u{494}\
	\x03\x02\x02\x02\u{dc}\u{496}\x03\x02\x02\x02\u{de}\u{49e}\x03\x02\x02\x02\
	\u{e0}\u{4a6}\x03\x02\x02\x02\u{e2}\u{4a8}\x03\x02\x02\x02\u{e4}\u{4b5}\
	\x03\x02\x02\x02\u{e6}\u{4ba}\x03\x02\x02\x02\u{e8}\u{4bc}\x03\x02\x02\x02\
	\u{ea}\u{4d0}\x03\x02\x02\x02\u{ec}\u{4d7}\x03\x02\x02\x02\u{ee}\u{4e1}\
	\x03\x02\x02\x02\u{f0}\u{4e8}\x03\x02\x02\x02\u{f2}\u{4ea}\x03\x02\x02\x02\
	\u{f4}\u{4f7}\x03\x02\x02\x02\u{f6}\u{4f9}\x03\x02\x02\x02\u{f8}\u{508}\
	\x03\x02\x02\x02\u{fa}\u{50d}\x03\x02\x02\x02\u{fc}\u{50f}\x03\x02\x02\x02\
	\u{fe}\u{511}\x03\x02\x02\x02\u{100}\u{517}\x03\x02\x02\x02\u{102}\u{51b}\
	\x03\x02\x02\x02\u{104}\u{51d}\x03\x02\x02\x02\u{106}\u{521}\x03\x02\x02\
	\x02\u{108}\u{523}\x03\x02\x02\x02\u{10a}\u{525}\x03\x02\x02\x02\u{10c}\
	\u{527}\x03\x02\x02\x02\u{10e}\u{529}\x03\x02\x02\x02\u{110}\u{52b}\x03\
	\x02\x02\x02\u{112}\u{52d}\x03\x02\x02\x02\u{114}\u{52f}\x03\x02\x02\x02\
	\u{116}\u{531}\x03\x02\x02\x02\u{118}\u{535}\x03\x02\x02\x02\u{11a}\u{539}\
	\x03\x02\x02\x02\u{11c}\u{53b}\x03\x02\x02\x02\u{11e}\u{53d}\x03\x02\x02\
	\x02\u{120}\u{53f}\x03\x02\x02\x02\u{122}\u{541}\x03\x02\x02\x02\u{124}\
	\u{543}\x03\x02\x02\x02\u{126}\u{545}\x03\x02\x02\x02\u{128}\u{547}\x03\
	\x02\x02\x02\u{12a}\u{549}\x03\x02\x02\x02\u{12c}\u{54b}\x03\x02\x02\x02\
	\u{12e}\u{54f}\x03\x02\x02\x02\u{130}\u{551}\x03\x02\x02\x02\u{132}\u{55b}\
	\x03\x02\x02\x02\u{134}\u{55d}\x03\x02\x02\x02\u{136}\u{567}\x03\x02\x02\
	\x02\u{138}\u{569}\x03\x02\x02\x02\u{13a}\u{571}\x03\x02\x02\x02\u{13c}\
	\u{578}\x03\x02\x02\x02\u{13e}\u{57a}\x03\x02\x02\x02\u{140}\u{582}\x03\
	\x02\x02\x02\u{142}\u{586}\x03\x02\x02\x02\u{144}\u{589}\x03\x02\x02\x02\
	\u{146}\u{590}\x03\x02\x02\x02\u{148}\u{595}\x03\x02\x02\x02\u{14a}\u{597}\
	\x03\x02\x02\x02\u{14c}\u{59e}\x03\x02\x02\x02\u{14e}\u{5a1}\x03\x02\x02\
	\x02\u{150}\u{5a8}\x03\x02\x02\x02\u{152}\u{5aa}\x03\x02\x02\x02\u{154}\
	\u{5b1}\x03\x02\x02\x02\u{156}\u{5b3}\x03\x02\x02\x02\u{158}\u{5bc}\x03\
	\x02\x02\x02\u{15a}\u{5be}\x03\x02\x02\x02\u{15c}\u{5c9}\x03\x02\x02\x02\
	\u{15e}\u{5cd}\x03\x02\x02\x02\u{160}\u{5cf}\x03\x02\x02\x02\u{162}\u{5d3}\
	\x03\x02\x02\x02\u{164}\u{5db}\x03\x02\x02\x02\u{166}\u{5e2}\x03\x02\x02\
	\x02\u{168}\u{5e4}\x03\x02\x02\x02\u{16a}\u{5e6}\x03\x02\x02\x02\u{16c}\
	\u{5ed}\x03\x02\x02\x02\u{16e}\u{5f3}\x03\x02\x02\x02\u{170}\u{5f7}\x03\
	\x02\x02\x02\u{172}\u{5ff}\x03\x02\x02\x02\u{174}\u{609}\x03\x02\x02\x02\
	\u{176}\u{60b}\x03\x02\x02\x02\u{178}\u{612}\x03\x02\x02\x02\u{17a}\u{617}\
	\x03\x02\x02\x02\u{17c}\u{625}\x03\x02\x02\x02\u{17e}\u{62d}\x03\x02\x02\
	\x02\u{180}\u{62f}\x03\x02\x02\x02\u{182}\u{648}\x03\x02\x02\x02\u{184}\
	\u{64f}\x03\x02\x02\x02\u{186}\u{651}\x03\x02\x02\x02\u{188}\u{65b}\x03\
	\x02\x02\x02\u{18a}\u{65d}\x03\x02\x02\x02\u{18c}\u{65f}\x03\x02\x02\x02\
	\u{18e}\u{661}\x03\x02\x02\x02\u{190}\u{663}\x03\x02\x02\x02\u{192}\u{665}\
	\x03\x02\x02\x02\u{194}\u{196}\x05\x04\x03\x02\u{195}\u{194}\x03\x02\x02\
	\x02\u{196}\u{199}\x03\x02\x02\x02\u{197}\u{195}\x03\x02\x02\x02\u{197}\
	\u{198}\x03\x02\x02\x02\u{198}\u{19a}\x03\x02\x02\x02\u{199}\u{197}\x03\
	\x02\x02\x02\u{19a}\u{19b}\x07\x02\x02\x03\u{19b}\x03\x03\x02\x02\x02\u{19c}\
	\u{19f}\x05\x06\x04\x02\u{19d}\u{19f}\x05\u{176}\u{bc}\x02\u{19e}\u{19c}\
	\x03\x02\x02\x02\u{19e}\u{19d}\x03\x02\x02\x02\u{19f}\x05\x03\x02\x02\x02\
	\u{1a0}\u{1a8}\x05\x0c\x07\x02\u{1a1}\u{1a8}\x05\x0e\x08\x02\u{1a2}\u{1a8}\
	\x05\x10\x09\x02\u{1a3}\u{1a8}\x05\x12\x0a\x02\u{1a4}\u{1a8}\x05\x14\x0b\
	\x02\u{1a5}\u{1a8}\x05\x16\x0c\x02\u{1a6}\u{1a8}\x05\x08\x05\x02\u{1a7}\
	\u{1a0}\x03\x02\x02\x02\u{1a7}\u{1a1}\x03\x02\x02\x02\u{1a7}\u{1a2}\x03\
	\x02\x02\x02\u{1a7}\u{1a3}\x03\x02\x02\x02\u{1a7}\u{1a4}\x03\x02\x02\x02\
	\u{1a7}\u{1a5}\x03\x02\x02\x02\u{1a7}\u{1a6}\x03\x02\x02\x02\u{1a8}\x07\
	\x03\x02\x02\x02\u{1a9}\u{1aa}\x07\x03\x02\x02\u{1aa}\u{1ab}\x05\u{188}\
	\u{c5}\x02\u{1ab}\u{1ac}\x07\x04\x02\x02\u{1ac}\u{1ad}\x05\x1a\x0e\x02\u{1ad}\
	\u{1ae}\x07\x04\x02\x02\u{1ae}\u{1b0}\x05\x0a\x06\x02\u{1af}\u{1b1}\x05\
	\x18\x0d\x02\u{1b0}\u{1af}\x03\x02\x02\x02\u{1b0}\u{1b1}\x03\x02\x02\x02\
	\u{1b1}\u{1b2}\x03\x02\x02\x02\u{1b2}\u{1b3}\x07\x05\x02\x02\u{1b3}\x09\
	\x03\x02\x02\x02\u{1b4}\u{1b5}\x05\u{b4}\x5b\x02\u{1b5}\x0b\x03\x02\x02\
	\x02\u{1b6}\u{1b7}\x07\x06\x02\x02\u{1b7}\u{1b8}\x05\u{188}\u{c5}\x02\u{1b8}\
	\u{1b9}\x07\x04\x02\x02\u{1b9}\u{1ba}\x05\x1a\x0e\x02\u{1ba}\u{1bb}\x07\
	\x04\x02\x02\u{1bb}\u{1bd}\x05\x1c\x0f\x02\u{1bc}\u{1be}\x05\x18\x0d\x02\
	\u{1bd}\u{1bc}\x03\x02\x02\x02\u{1bd}\u{1be}\x03\x02\x02\x02\u{1be}\u{1bf}\
	\x03\x02\x02\x02\u{1bf}\u{1c0}\x07\x05\x02\x02\u{1c0}\x0d\x03\x02\x02\x02\
	\u{1c1}\u{1c2}\x07\x07\x02\x02\u{1c2}\u{1c3}\x05\u{188}\u{c5}\x02\u{1c3}\
	\u{1c4}\x07\x04\x02\x02\u{1c4}\u{1c5}\x05\x1a\x0e\x02\u{1c5}\u{1c6}\x07\
	\x04\x02\x02\u{1c6}\u{1c8}\x05\x60\x31\x02\u{1c7}\u{1c9}\x05\x18\x0d\x02\
	\u{1c8}\u{1c7}\x03\x02\x02\x02\u{1c8}\u{1c9}\x03\x02\x02\x02\u{1c9}\u{1ca}\
	\x03\x02\x02\x02\u{1ca}\u{1cb}\x07\x05\x02\x02\u{1cb}\x0f\x03\x02\x02\x02\
	\u{1cc}\u{1cd}\x07\x08\x02\x02\u{1cd}\u{1ce}\x05\u{188}\u{c5}\x02\u{1ce}\
	\u{1cf}\x07\x04\x02\x02\u{1cf}\u{1d0}\x05\x1a\x0e\x02\u{1d0}\u{1d1}\x07\
	\x04\x02\x02\u{1d1}\u{1d3}\x05\x64\x33\x02\u{1d2}\u{1d4}\x05\x18\x0d\x02\
	\u{1d3}\u{1d2}\x03\x02\x02\x02\u{1d3}\u{1d4}\x03\x02\x02\x02\u{1d4}\u{1d5}\
	\x03\x02\x02\x02\u{1d5}\u{1d6}\x07\x05\x02\x02\u{1d6}\x11\x03\x02\x02\x02\
	\u{1d7}\u{1d8}\x07\x09\x02\x02\u{1d8}\u{1d9}\x05\u{188}\u{c5}\x02\u{1d9}\
	\u{1da}\x07\x04\x02\x02\u{1da}\u{1db}\x05\x1a\x0e\x02\u{1db}\u{1dc}\x07\
	\x04\x02\x02\u{1dc}\u{1de}\x05\u{ae}\x58\x02\u{1dd}\u{1df}\x05\x18\x0d\x02\
	\u{1de}\u{1dd}\x03\x02\x02\x02\u{1de}\u{1df}\x03\x02\x02\x02\u{1df}\u{1e0}\
	\x03\x02\x02\x02\u{1e0}\u{1e1}\x07\x05\x02\x02\u{1e1}\x13\x03\x02\x02\x02\
	\u{1e2}\u{1e3}\x07\x0a\x02\x02\u{1e3}\u{1e4}\x05\u{188}\u{c5}\x02\u{1e4}\
	\u{1e5}\x07\x04\x02\x02\u{1e5}\u{1e6}\x05\x1a\x0e\x02\u{1e6}\u{1e7}\x07\
	\x04\x02\x02\u{1e7}\u{1e9}\x05\u{b4}\x5b\x02\u{1e8}\u{1ea}\x05\x18\x0d\x02\
	\u{1e9}\u{1e8}\x03\x02\x02\x02\u{1e9}\u{1ea}\x03\x02\x02\x02\u{1ea}\u{1eb}\
	\x03\x02\x02\x02\u{1eb}\u{1ec}\x07\x05\x02\x02\u{1ec}\x15\x03\x02\x02\x02\
	\u{1ed}\u{1ee}\x07\x0b\x02\x02\u{1ee}\u{1ef}\x05\u{188}\u{c5}\x02\u{1ef}\
	\u{1f0}\x07\x04\x02\x02\u{1f0}\u{1f1}\x05\x1a\x0e\x02\u{1f1}\u{1f2}\x07\
	\x04\x02\x02\u{1f2}\u{1f4}\x05\u{f4}\x7b\x02\u{1f3}\u{1f5}\x05\x18\x0d\x02\
	\u{1f4}\u{1f3}\x03\x02\x02\x02\u{1f4}\u{1f5}\x03\x02\x02\x02\u{1f5}\u{1f6}\
	\x03\x02\x02\x02\u{1f6}\u{1f7}\x07\x05\x02\x02\u{1f7}\x17\x03\x02\x02\x02\
	\u{1f8}\u{1f9}\x07\x04\x02\x02\u{1f9}\u{1fb}\x05\u{132}\u{9a}\x02\u{1fa}\
	\u{1fc}\x05\u{156}\u{ac}\x02\u{1fb}\u{1fa}\x03\x02\x02\x02\u{1fb}\u{1fc}\
	\x03\x02\x02\x02\u{1fc}\x19\x03\x02\x02\x02\u{1fd}\u{1fe}\x07\x60\x02\x02\
	\u{1fe}\x1b\x03\x02\x02\x02\u{1ff}\u{202}\x05\x1e\x10\x02\u{200}\u{202}\
	\x05\x5a\x2e\x02\u{201}\u{1ff}\x03\x02\x02\x02\u{201}\u{200}\x03\x02\x02\
	\x02\u{202}\x1d\x03\x02\x02\x02\u{203}\u{208}\x05\x20\x11\x02\u{204}\u{208}\
	\x05\x2c\x17\x02\u{205}\u{208}\x05\x46\x24\x02\u{206}\u{208}\x05\x4a\x26\
	\x02\u{207}\u{203}\x03\x02\x02\x02\u{207}\u{204}\x03\x02\x02\x02\u{207}\
	\u{205}\x03\x02\x02\x02\u{207}\u{206}\x03\x02\x02\x02\u{208}\x1f\x03\x02\
	\x02\x02\u{209}\u{20d}\x05\x22\x12\x02\u{20a}\u{20d}\x05\x24\x13\x02\u{20b}\
	\u{20d}\x05\x52\x2a\x02\u{20c}\u{209}\x03\x02\x02\x02\u{20c}\u{20a}\x03\
	\x02\x02\x02\u{20c}\u{20b}\x03\x02\x02\x02\u{20d}\x21\x03\x02\x02\x02\u{20e}\
	\u{20f}\x05\x2c\x17\x02\u{20f}\u{210}\x05\u{100}\u{81}\x02\u{210}\u{211}\
	\x05\x2c\x17\x02\u{211}\x23\x03\x02\x02\x02\u{212}\u{216}\x05\x26\x14\x02\
	\u{213}\u{216}\x05\x28\x15\x02\u{214}\u{216}\x05\x2a\x16\x02\u{215}\u{212}\
	\x03\x02\x02\x02\u{215}\u{213}\x03\x02\x02\x02\u{215}\u{214}\x03\x02\x02\
	\x02\u{216}\x25\x03\x02\x02\x02\u{217}\u{218}\x08\x14\x01\x02\u{218}\u{219}\
	\x05\x2c\x17\x02\u{219}\u{21a}\x07\x2e\x02\x02\u{21a}\u{21b}\x05\x2c\x17\
	\x02\u{21b}\u{221}\x03\x02\x02\x02\u{21c}\u{21d}\x0c\x03\x02\x02\u{21d}\
	\u{21e}\x07\x2e\x02\x02\u{21e}\u{220}\x05\x2c\x17\x02\u{21f}\u{21c}\x03\
	\x02\x02\x02\u{220}\u{223}\x03\x02\x02\x02\u{221}\u{21f}\x03\x02\x02\x02\
	\u{221}\u{222}\x03\x02\x02\x02\u{222}\x27\x03\x02\x02\x02\u{223}\u{221}\
	\x03\x02\x02\x02\u{224}\u{225}\x08\x15\x01\x02\u{225}\u{226}\x05\x2c\x17\
	\x02\u{226}\u{227}\x07\x2f\x02\x02\u{227}\u{228}\x05\x2c\x17\x02\u{228}\
	\u{22e}\x03\x02\x02\x02\u{229}\u{22a}\x0c\x03\x02\x02\u{22a}\u{22b}\x07\
	\x2f\x02\x02\u{22b}\u{22d}\x05\x2c\x17\x02\u{22c}\u{229}\x03\x02\x02\x02\
	\u{22d}\u{230}\x03\x02\x02\x02\u{22e}\u{22c}\x03\x02\x02\x02\u{22e}\u{22f}\
	\x03\x02\x02\x02\u{22f}\x29\x03\x02\x02\x02\u{230}\u{22e}\x03\x02\x02\x02\
	\u{231}\u{232}\x08\x16\x01\x02\u{232}\u{233}\x05\x2c\x17\x02\u{233}\u{234}\
	\x07\x45\x02\x02\u{234}\u{235}\x05\x2c\x17\x02\u{235}\u{23b}\x03\x02\x02\
	\x02\u{236}\u{237}\x0c\x03\x02\x02\u{237}\u{238}\x07\x45\x02\x02\u{238}\
	\u{23a}\x05\x2c\x17\x02\u{239}\u{236}\x03\x02\x02\x02\u{23a}\u{23d}\x03\
	\x02\x02\x02\u{23b}\u{239}\x03\x02\x02\x02\u{23b}\u{23c}\x03\x02\x02\x02\
	\u{23c}\x2b\x03\x02\x02\x02\u{23d}\u{23b}\x03\x02\x02\x02\u{23e}\u{249}\
	\x05\x2e\x18\x02\u{23f}\u{249}\x05\x38\x1d\x02\u{240}\u{249}\x05\x3a\x1e\
	\x02\u{241}\u{249}\x05\x40\x21\x02\u{242}\u{249}\x05\x42\x22\x02\u{243}\
	\u{249}\x05\x5c\x2f\x02\u{244}\u{245}\x07\x0c\x02\x02\u{245}\u{246}\x05\
	\x1e\x10\x02\u{246}\u{247}\x07\x0d\x02\x02\u{247}\u{249}\x03\x02\x02\x02\
	\u{248}\u{23e}\x03\x02\x02\x02\u{248}\u{23f}\x03\x02\x02\x02\u{248}\u{240}\
	\x03\x02\x02\x02\u{248}\u{241}\x03\x02\x02\x02\u{248}\u{242}\x03\x02\x02\
	\x02\u{248}\u{243}\x03\x02\x02\x02\u{248}\u{244}\x03\x02\x02\x02\u{249}\
	\x2d\x03\x02\x02\x02\u{24a}\u{24b}\x05\x30\x19\x02\u{24b}\u{24c}\x05\x2c\
	\x17\x02\u{24c}\x2f\x03\x02\x02\x02\u{24d}\u{24e}\x05\u{fa}\x7e\x02\u{24e}\
	\u{24f}\x07\x0e\x02\x02\u{24f}\u{250}\x05\x32\x1a\x02\u{250}\u{251}\x07\
	\x0f\x02\x02\u{251}\u{252}\x07\x10\x02\x02\u{252}\x31\x03\x02\x02\x02\u{253}\
	\u{258}\x05\x34\x1b\x02\u{254}\u{255}\x07\x04\x02\x02\u{255}\u{257}\x05\
	\x34\x1b\x02\u{256}\u{254}\x03\x02\x02\x02\u{257}\u{25a}\x03\x02\x02\x02\
	\u{258}\u{256}\x03\x02\x02\x02\u{258}\u{259}\x03\x02\x02\x02\u{259}\x33\
	\x03\x02\x02\x02\u{25a}\u{258}\x03\x02\x02\x02\u{25b}\u{25e}\x05\x36\x1c\
	\x02\u{25c}\u{25e}\x05\u{130}\u{99}\x02\u{25d}\u{25b}\x03\x02\x02\x02\u{25d}\
	\u{25c}\x03\x02\x02\x02\u{25e}\x35\x03\x02\x02\x02\u{25f}\u{260}\x05\u{130}\
	\u{99}\x02\u{260}\u{261}\x07\x10\x02\x02\u{261}\u{262}\x05\x4c\x27\x02\u{262}\
	\x37\x03\x02\x02\x02\u{263}\u{264}\x05\u{102}\u{82}\x02\u{264}\u{265}\x07\
	\x0c\x02\x02\u{265}\u{266}\x05\x1e\x10\x02\u{266}\u{267}\x07\x0d\x02\x02\
	\u{267}\x39\x03\x02\x02\x02\u{268}\u{26d}\x05\x3c\x1f\x02\u{269}\u{26d}\
	\x05\u{130}\u{99}\x02\u{26a}\u{26d}\x05\u{12e}\u{98}\x02\u{26b}\u{26d}\x05\
	\x3e\x20\x02\u{26c}\u{268}\x03\x02\x02\x02\u{26c}\u{269}\x03\x02\x02\x02\
	\u{26c}\u{26a}\x03\x02\x02\x02\u{26c}\u{26b}\x03\x02\x02\x02\u{26d}\x3b\
	\x03\x02\x02\x02\u{26e}\u{27f}\x05\u{118}\u{8d}\x02\u{26f}\u{270}\x05\u{124}\
	\u{93}\x02\u{270}\u{271}\x07\x0c\x02\x02\u{271}\u{272}\x05\x44\x23\x02\u{272}\
	\u{273}\x07\x0d\x02\x02\u{273}\u{27f}\x03\x02\x02\x02\u{274}\u{275}\x05\
	\u{12c}\u{97}\x02\u{275}\u{276}\x07\x0c\x02\x02\u{276}\u{277}\x05\x44\x23\
	\x02\u{277}\u{278}\x07\x0d\x02\x02\u{278}\u{27f}\x03\x02\x02\x02\u{279}\
	\u{27a}\x05\u{128}\u{95}\x02\u{27a}\u{27b}\x07\x0c\x02\x02\u{27b}\u{27c}\
	\x05\x44\x23\x02\u{27c}\u{27d}\x07\x0d\x02\x02\u{27d}\u{27f}\x03\x02\x02\
	\x02\u{27e}\u{26e}\x03\x02\x02\x02\u{27e}\u{26f}\x03\x02\x02\x02\u{27e}\
	\u{274}\x03\x02\x02\x02\u{27e}\u{279}\x03\x02\x02\x02\u{27f}\x3d\x03\x02\
	\x02\x02\u{280}\u{284}\x05\u{100}\u{81}\x02\u{281}\u{284}\x05\u{10c}\u{87}\
	\x02\u{282}\u{284}\x05\u{102}\u{82}\x02\u{283}\u{280}\x03\x02\x02\x02\u{283}\
	\u{281}\x03\x02\x02\x02\u{283}\u{282}\x03\x02\x02\x02\u{284}\x3f\x03\x02\
	\x02\x02\u{285}\u{286}\x07\x11\x02\x02\u{286}\u{287}\x05\x1e\x10\x02\u{287}\
	\u{288}\x07\x04\x02\x02\u{288}\u{289}\x05\x1e\x10\x02\u{289}\u{28a}\x07\
	\x04\x02\x02\u{28a}\u{28b}\x05\x1e\x10\x02\u{28b}\u{28c}\x07\x0d\x02\x02\
	\u{28c}\x41\x03\x02\x02\x02\u{28d}\u{28e}\x07\x12\x02\x02\u{28e}\u{28f}\
	\x05\x2c\x17\x02\u{28f}\u{290}\x07\x04\x02\x02\u{290}\u{291}\x05\x1c\x0f\
	\x02\u{291}\u{292}\x07\x0d\x02\x02\u{292}\x43\x03\x02\x02\x02\u{293}\u{294}\
	\x05\x5e\x30\x02\u{294}\x45\x03\x02\x02\x02\u{295}\u{296}\x05\x48\x25\x02\
	\u{296}\u{297}\x07\x10\x02\x02\u{297}\u{298}\x05\x4c\x27\x02\u{298}\x47\
	\x03\x02\x02\x02\u{299}\u{29f}\x05\x3a\x1e\x02\u{29a}\u{29b}\x07\x0c\x02\
	\x02\u{29b}\u{29c}\x05\x1e\x10\x02\u{29c}\u{29d}\x07\x0d\x02\x02\u{29d}\
	\u{29f}\x03\x02\x02\x02\u{29e}\u{299}\x03\x02\x02\x02\u{29e}\u{29a}\x03\
	\x02\x02\x02\u{29f}\x49\x03\x02\x02\x02\u{2a0}\u{2a1}\x05\x3a\x1e\x02\u{2a1}\
	\u{2a2}\x07\x4a\x02\x02\u{2a2}\u{2a3}\x05\x3a\x1e\x02\u{2a3}\x4b\x03\x02\
	\x02\x02\u{2a4}\u{2a8}\x05\x4e\x28\x02\u{2a5}\u{2a8}\x05\x54\x2b\x02\u{2a6}\
	\u{2a8}\x05\x50\x29\x02\u{2a7}\u{2a4}\x03\x02\x02\x02\u{2a7}\u{2a5}\x03\
	\x02\x02\x02\u{2a7}\u{2a6}\x03\x02\x02\x02\u{2a8}\x4d\x03\x02\x02\x02\u{2a9}\
	\u{2aa}\x05\x2c\x17\x02\u{2aa}\x4f\x03\x02\x02\x02\u{2ab}\u{2ac}\x05\x2a\
	\x16\x02\u{2ac}\x51\x03\x02\x02\x02\u{2ad}\u{2b1}\x05\x54\x2b\x02\u{2ae}\
	\u{2b1}\x05\x56\x2c\x02\u{2af}\u{2b1}\x05\x58\x2d\x02\u{2b0}\u{2ad}\x03\
	\x02\x02\x02\u{2b0}\u{2ae}\x03\x02\x02\x02\u{2b0}\u{2af}\x03\x02\x02\x02\
	\u{2b1}\x53\x03\x02\x02\x02\u{2b2}\u{2b3}\x05\x4e\x28\x02\u{2b3}\u{2b4}\
	\x07\x47\x02\x02\u{2b4}\u{2b5}\x05\x4e\x28\x02\u{2b5}\u{2bb}\x03\x02\x02\
	\x02\u{2b6}\u{2b7}\x05\x4e\x28\x02\u{2b7}\u{2b8}\x07\x47\x02\x02\u{2b8}\
	\u{2b9}\x05\x54\x2b\x02\u{2b9}\u{2bb}\x03\x02\x02\x02\u{2ba}\u{2b2}\x03\
	\x02\x02\x02\u{2ba}\u{2b6}\x03\x02\x02\x02\u{2bb}\x55\x03\x02\x02\x02\u{2bc}\
	\u{2bd}\x08\x2c\x01\x02\u{2bd}\u{2be}\x05\x4e\x28\x02\u{2be}\u{2bf}\x07\
	\x48\x02\x02\u{2bf}\u{2c0}\x05\x4e\x28\x02\u{2c0}\u{2c6}\x03\x02\x02\x02\
	\u{2c1}\u{2c2}\x0c\x03\x02\x02\u{2c2}\u{2c3}\x07\x48\x02\x02\u{2c3}\u{2c5}\
	\x05\x4e\x28\x02\u{2c4}\u{2c1}\x03\x02\x02\x02\u{2c5}\u{2c8}\x03\x02\x02\
	\x02\u{2c6}\u{2c4}\x03\x02\x02\x02\u{2c6}\u{2c7}\x03\x02\x02\x02\u{2c7}\
	\x57\x03\x02\x02\x02\u{2c8}\u{2c6}\x03\x02\x02\x02\u{2c9}\u{2ca}\x08\x2d\
	\x01\x02\u{2ca}\u{2cb}\x05\x4e\x28\x02\u{2cb}\u{2cc}\x07\x49\x02\x02\u{2cc}\
	\u{2cd}\x05\x4e\x28\x02\u{2cd}\u{2d3}\x03\x02\x02\x02\u{2ce}\u{2cf}\x0c\
	\x03\x02\x02\u{2cf}\u{2d0}\x07\x49\x02\x02\u{2d0}\u{2d2}\x05\x4e\x28\x02\
	\u{2d1}\u{2ce}\x03\x02\x02\x02\u{2d2}\u{2d5}\x03\x02\x02\x02\u{2d3}\u{2d1}\
	\x03\x02\x02\x02\u{2d3}\u{2d4}\x03\x02\x02\x02\u{2d4}\x59\x03\x02\x02\x02\
	\u{2d5}\u{2d3}\x03\x02\x02\x02\u{2d6}\u{2d7}\x05\x5c\x2f\x02\u{2d7}\u{2d8}\
	\x07\x4b\x02\x02\u{2d8}\u{2d9}\x05\x5c\x2f\x02\u{2d9}\u{2df}\x03\x02\x02\
	\x02\u{2da}\u{2db}\x07\x0c\x02\x02\u{2db}\u{2dc}\x05\x5a\x2e\x02\u{2dc}\
	\u{2dd}\x07\x0d\x02\x02\u{2dd}\u{2df}\x03\x02\x02\x02\u{2de}\u{2d6}\x03\
	\x02\x02\x02\u{2de}\u{2da}\x03\x02\x02\x02\u{2df}\x5b\x03\x02\x02\x02\u{2e0}\
	\u{2eb}\x07\x13\x02\x02\u{2e1}\u{2e2}\x07\x0e\x02\x02\u{2e2}\u{2e3}\x05\
	\x5e\x30\x02\u{2e3}\u{2e4}\x07\x0f\x02\x02\u{2e4}\u{2eb}\x03\x02\x02\x02\
	\u{2e5}\u{2eb}\x07\x14\x02\x02\u{2e6}\u{2e7}\x07\x15\x02\x02\u{2e7}\u{2e8}\
	\x05\x5e\x30\x02\u{2e8}\u{2e9}\x07\x16\x02\x02\u{2e9}\u{2eb}\x03\x02\x02\
	\x02\u{2ea}\u{2e0}\x03\x02\x02\x02\u{2ea}\u{2e1}\x03\x02\x02\x02\u{2ea}\
	\u{2e5}\x03\x02\x02\x02\u{2ea}\u{2e6}\x03\x02\x02\x02\u{2eb}\x5d\x03\x02\
	\x02\x02\u{2ec}\u{2f1}\x05\x1e\x10\x02\u{2ed}\u{2ee}\x07\x04\x02\x02\u{2ee}\
	\u{2f0}\x05\x1e\x10\x02\u{2ef}\u{2ed}\x03\x02\x02\x02\u{2f0}\u{2f3}\x03\
	\x02\x02\x02\u{2f1}\u{2ef}\x03\x02\x02\x02\u{2f1}\u{2f2}\x03\x02\x02\x02\
	\u{2f2}\x5f\x03\x02\x02\x02\u{2f3}\u{2f1}\x03\x02\x02\x02\u{2f4}\u{2f7}\
	\x05\x62\x32\x02\u{2f5}\u{2f7}\x05\x5a\x2e\x02\u{2f6}\u{2f4}\x03\x02\x02\
	\x02\u{2f6}\u{2f5}\x03\x02\x02\x02\u{2f7}\x61\x03\x02\x02\x02\u{2f8}\u{2f9}\
	\x05\x1e\x10\x02\u{2f9}\x63\x03\x02\x02\x02\u{2fa}\u{2fe}\x05\x66\x34\x02\
	\u{2fb}\u{2fe}\x05\u{9a}\x4e\x02\u{2fc}\u{2fe}\x05\u{94}\x4b\x02\u{2fd}\
	\u{2fa}\x03\x02\x02\x02\u{2fd}\u{2fb}\x03\x02\x02\x02\u{2fd}\u{2fc}\x03\
	\x02\x02\x02\u{2fe}\x65\x03\x02\x02\x02\u{2ff}\u{303}\x05\x68\x35\x02\u{300}\
	\u{303}\x05\x72\x3a\x02\u{301}\u{303}\x05\u{9c}\x4f\x02\u{302}\u{2ff}\x03\
	\x02\x02\x02\u{302}\u{300}\x03\x02\x02\x02\u{302}\u{301}\x03\x02\x02\x02\
	\u{303}\x67\x03\x02\x02\x02\u{304}\u{307}\x05\x6a\x36\x02\u{305}\u{307}\
	\x05\x6c\x37\x02\u{306}\u{304}\x03\x02\x02\x02\u{306}\u{305}\x03\x02\x02\
	\x02\u{307}\x69\x03\x02\x02\x02\u{308}\u{309}\x05\x72\x3a\x02\u{309}\u{30a}\
	\x05\u{10a}\u{86}\x02\u{30a}\u{30b}\x05\x72\x3a\x02\u{30b}\x6b\x03\x02\x02\
	\x02\u{30c}\u{30f}\x05\x6e\x38\x02\u{30d}\u{30f}\x05\x70\x39\x02\u{30e}\
	\u{30c}\x03\x02\x02\x02\u{30e}\u{30d}\x03\x02\x02\x02\u{30f}\x6d\x03\x02\
	\x02\x02\u{310}\u{311}\x08\x38\x01\x02\u{311}\u{312}\x05\x72\x3a\x02\u{312}\
	\u{313}\x07\x2e\x02\x02\u{313}\u{314}\x05\x72\x3a\x02\u{314}\u{31a}\x03\
	\x02\x02\x02\u{315}\u{316}\x0c\x03\x02\x02\u{316}\u{317}\x07\x2e\x02\x02\
	\u{317}\u{319}\x05\x72\x3a\x02\u{318}\u{315}\x03\x02\x02\x02\u{319}\u{31c}\
	\x03\x02\x02\x02\u{31a}\u{318}\x03\x02\x02\x02\u{31a}\u{31b}\x03\x02\x02\
	\x02\u{31b}\x6f\x03\x02\x02\x02\u{31c}\u{31a}\x03\x02\x02\x02\u{31d}\u{31e}\
	\x08\x39\x01\x02\u{31e}\u{31f}\x05\x72\x3a\x02\u{31f}\u{320}\x07\x2f\x02\
	\x02\u{320}\u{321}\x05\x72\x3a\x02\u{321}\u{327}\x03\x02\x02\x02\u{322}\
	\u{323}\x0c\x03\x02\x02\u{323}\u{324}\x07\x2f\x02\x02\u{324}\u{326}\x05\
	\x72\x3a\x02\u{325}\u{322}\x03\x02\x02\x02\u{326}\u{329}\x03\x02\x02\x02\
	\u{327}\u{325}\x03\x02\x02\x02\u{327}\u{328}\x03\x02\x02\x02\u{328}\x71\
	\x03\x02\x02\x02\u{329}\u{327}\x03\x02\x02\x02\u{32a}\u{334}\x05\x74\x3b\
	\x02\u{32b}\u{334}\x05\x7c\x3f\x02\u{32c}\u{334}\x05\x7e\x40\x02\u{32d}\
	\u{334}\x05\u{80}\x41\x02\u{32e}\u{334}\x05\u{82}\x42\x02\u{32f}\u{330}\
	\x07\x0c\x02\x02\u{330}\u{331}\x05\x66\x34\x02\u{331}\u{332}\x07\x0d\x02\
	\x02\u{332}\u{334}\x03\x02\x02\x02\u{333}\u{32a}\x03\x02\x02\x02\u{333}\
	\u{32b}\x03\x02\x02\x02\u{333}\u{32c}\x03\x02\x02\x02\u{333}\u{32d}\x03\
	\x02\x02\x02\u{333}\u{32e}\x03\x02\x02\x02\u{333}\u{32f}\x03\x02\x02\x02\
	\u{334}\x73\x03\x02\x02\x02\u{335}\u{336}\x05\u{108}\u{85}\x02\u{336}\u{337}\
	\x07\x0e\x02\x02\u{337}\u{338}\x05\x76\x3c\x02\u{338}\u{339}\x07\x0f\x02\
	\x02\u{339}\u{33a}\x07\x10\x02\x02\u{33a}\u{33b}\x05\x72\x3a\x02\u{33b}\
	\x75\x03\x02\x02\x02\u{33c}\u{341}\x05\x78\x3d\x02\u{33d}\u{33e}\x07\x04\
	\x02\x02\u{33e}\u{340}\x05\x78\x3d\x02\u{33f}\u{33d}\x03\x02\x02\x02\u{340}\
	\u{343}\x03\x02\x02\x02\u{341}\u{33f}\x03\x02\x02\x02\u{341}\u{342}\x03\
	\x02\x02\x02\u{342}\x77\x03\x02\x02\x02\u{343}\u{341}\x03\x02\x02\x02\u{344}\
	\u{347}\x05\x7a\x3e\x02\u{345}\u{347}\x05\u{130}\u{99}\x02\u{346}\u{344}\
	\x03\x02\x02\x02\u{346}\u{345}\x03\x02\x02\x02\u{347}\x79\x03\x02\x02\x02\
	\u{348}\u{349}\x05\u{130}\u{99}\x02\u{349}\u{34a}\x07\x10\x02\x02\u{34a}\
	\u{34b}\x05\u{a6}\x54\x02\u{34b}\x7b\x03\x02\x02\x02\u{34c}\u{34d}\x05\u{10e}\
	\u{88}\x02\u{34d}\u{34e}\x05\x72\x3a\x02\u{34e}\u{351}\x03\x02\x02\x02\u{34f}\
	\u{351}\x05\u{ca}\x66\x02\u{350}\u{34c}\x03\x02\x02\x02\u{350}\u{34f}\x03\
	\x02\x02\x02\u{351}\x7d\x03\x02\x02\x02\u{352}\u{353}\x05\u{cc}\x67\x02\
	\u{353}\x7f\x03\x02\x02\x02\u{354}\u{355}\x07\x17\x02\x02\u{355}\u{356}\
	\x05\x66\x34\x02\u{356}\u{357}\x07\x04\x02\x02\u{357}\u{358}\x05\x66\x34\
	\x02\u{358}\u{359}\x07\x04\x02\x02\u{359}\u{35a}\x05\x66\x34\x02\u{35a}\
	\u{35b}\x07\x0d\x02\x02\u{35b}\u{81}\x03\x02\x02\x02\u{35c}\u{35d}\x07\x18\
	\x02\x02\u{35d}\u{35e}\x05\u{84}\x43\x02\u{35e}\u{35f}\x07\x04\x02\x02\u{35f}\
	\u{360}\x05\x64\x33\x02\u{360}\u{361}\x07\x0d\x02\x02\u{361}\u{369}\x03\
	\x02\x02\x02\u{362}\u{363}\x07\x19\x02\x02\u{363}\u{364}\x05\u{8c}\x47\x02\
	\u{364}\u{365}\x07\x04\x02\x02\u{365}\u{366}\x05\x64\x33\x02\u{366}\u{367}\
	\x07\x0d\x02\x02\u{367}\u{369}\x03\x02\x02\x02\u{368}\u{35c}\x03\x02\x02\
	\x02\u{368}\u{362}\x03\x02\x02\x02\u{369}\u{83}\x03\x02\x02\x02\u{36a}\u{370}\
	\x05\u{88}\x45\x02\u{36b}\u{36c}\x07\x0e\x02\x02\u{36c}\u{36d}\x05\u{86}\
	\x44\x02\u{36d}\u{36e}\x07\x0f\x02\x02\u{36e}\u{370}\x03\x02\x02\x02\u{36f}\
	\u{36a}\x03\x02\x02\x02\u{36f}\u{36b}\x03\x02\x02\x02\u{370}\u{85}\x03\x02\
	\x02\x02\u{371}\u{376}\x05\u{88}\x45\x02\u{372}\u{373}\x07\x04\x02\x02\u{373}\
	\u{375}\x05\u{88}\x45\x02\u{374}\u{372}\x03\x02\x02\x02\u{375}\u{378}\x03\
	\x02\x02\x02\u{376}\u{374}\x03\x02\x02\x02\u{376}\u{377}\x03\x02\x02\x02\
	\u{377}\u{87}\x03\x02\x02\x02\u{378}\u{376}\x03\x02\x02\x02\u{379}\u{37a}\
	\x07\x3b\x02\x02\u{37a}\u{37b}\x07\x0e\x02\x02\u{37b}\u{37c}\x05\x76\x3c\
	\x02\u{37c}\u{37d}\x07\x0f\x02\x02\u{37d}\u{37e}\x07\x10\x02\x02\u{37e}\
	\u{37f}\x05\u{88}\x45\x02\u{37f}\u{382}\x03\x02\x02\x02\u{380}\u{382}\x05\
	\u{8a}\x46\x02\u{381}\u{379}\x03\x02\x02\x02\u{381}\u{380}\x03\x02\x02\x02\
	\u{382}\u{89}\x03\x02\x02\x02\u{383}\u{384}\x05\u{d8}\x6d\x02\u{384}\u{385}\
	\x07\x3a\x02\x02\u{385}\u{386}\x05\u{e4}\x73\x02\u{386}\u{38c}\x03\x02\x02\
	\x02\u{387}\u{388}\x07\x0c\x02\x02\u{388}\u{389}\x05\u{8a}\x46\x02\u{389}\
	\u{38a}\x07\x0d\x02\x02\u{38a}\u{38c}\x03\x02\x02\x02\u{38b}\u{383}\x03\
	\x02\x02\x02\u{38b}\u{387}\x03\x02\x02\x02\u{38c}\u{8b}\x03\x02\x02\x02\
	\u{38d}\u{393}\x05\u{90}\x49\x02\u{38e}\u{38f}\x07\x0e\x02\x02\u{38f}\u{390}\
	\x05\u{8e}\x48\x02\u{390}\u{391}\x07\x0f\x02\x02\u{391}\u{393}\x03\x02\x02\
	\x02\u{392}\u{38d}\x03\x02\x02\x02\u{392}\u{38e}\x03\x02\x02\x02\u{393}\
	\u{8d}\x03\x02\x02\x02\u{394}\u{399}\x05\u{90}\x49\x02\u{395}\u{396}\x07\
	\x04\x02\x02\u{396}\u{398}\x05\u{90}\x49\x02\u{397}\u{395}\x03\x02\x02\x02\
	\u{398}\u{39b}\x03\x02\x02\x02\u{399}\u{397}\x03\x02\x02\x02\u{399}\u{39a}\
	\x03\x02\x02\x02\u{39a}\u{8f}\x03\x02\x02\x02\u{39b}\u{399}\x03\x02\x02\
	\x02\u{39c}\u{39d}\x07\x3b\x02\x02\u{39d}\u{39e}\x07\x0e\x02\x02\u{39e}\
	\u{39f}\x05\x76\x3c\x02\u{39f}\u{3a0}\x07\x0f\x02\x02\u{3a0}\u{3a1}\x07\
	\x10\x02\x02\u{3a1}\u{3a2}\x05\u{90}\x49\x02\u{3a2}\u{3a5}\x03\x02\x02\x02\
	\u{3a3}\u{3a5}\x05\u{92}\x4a\x02\u{3a4}\u{39c}\x03\x02\x02\x02\u{3a4}\u{3a3}\
	\x03\x02\x02\x02\u{3a5}\u{91}\x03\x02\x02\x02\u{3a6}\u{3a7}\x05\u{ce}\x68\
	\x02\u{3a7}\u{3a8}\x07\x30\x02\x02\u{3a8}\u{3a9}\x05\x72\x3a\x02\u{3a9}\
	\u{3af}\x03\x02\x02\x02\u{3aa}\u{3ab}\x07\x0c\x02\x02\u{3ab}\u{3ac}\x05\
	\u{92}\x4a\x02\u{3ac}\u{3ad}\x07\x0d\x02\x02\u{3ad}\u{3af}\x03\x02\x02\x02\
	\u{3ae}\u{3a6}\x03\x02\x02\x02\u{3ae}\u{3aa}\x03\x02\x02\x02\u{3af}\u{93}\
	\x03\x02\x02\x02\u{3b0}\u{3b1}\x05\u{96}\x4c\x02\u{3b1}\u{3b2}\x07\x4b\x02\
	\x02\u{3b2}\u{3b3}\x05\u{96}\x4c\x02\u{3b3}\u{3b9}\x03\x02\x02\x02\u{3b4}\
	\u{3b5}\x07\x0c\x02\x02\u{3b5}\u{3b6}\x05\u{94}\x4b\x02\u{3b6}\u{3b7}\x07\
	\x0d\x02\x02\u{3b7}\u{3b9}\x03\x02\x02\x02\u{3b8}\u{3b0}\x03\x02\x02\x02\
	\u{3b8}\u{3b4}\x03\x02\x02\x02\u{3b9}\u{95}\x03\x02\x02\x02\u{3ba}\u{3c0}\
	\x07\x13\x02\x02\u{3bb}\u{3bc}\x07\x0e\x02\x02\u{3bc}\u{3bd}\x05\u{98}\x4d\
	\x02\u{3bd}\u{3be}\x07\x0f\x02\x02\u{3be}\u{3c0}\x03\x02\x02\x02\u{3bf}\
	\u{3ba}\x03\x02\x02\x02\u{3bf}\u{3bb}\x03\x02\x02\x02\u{3c0}\u{97}\x03\x02\
	\x02\x02\u{3c1}\u{3c6}\x05\x66\x34\x02\u{3c2}\u{3c3}\x07\x04\x02\x02\u{3c3}\
	\u{3c5}\x05\x66\x34\x02\u{3c4}\u{3c2}\x03\x02\x02\x02\u{3c5}\u{3c8}\x03\
	\x02\x02\x02\u{3c6}\u{3c4}\x03\x02\x02\x02\u{3c6}\u{3c7}\x03\x02\x02\x02\
	\u{3c7}\u{99}\x03\x02\x02\x02\u{3c8}\u{3c6}\x03\x02\x02\x02\u{3c9}\u{3ca}\
	\x05\u{11a}\u{8e}\x02\u{3ca}\u{3cb}\x07\x10\x02\x02\u{3cb}\u{3cc}\x05\u{9e}\
	\x50\x02\u{3cc}\u{3d2}\x03\x02\x02\x02\u{3cd}\u{3ce}\x07\x0c\x02\x02\u{3ce}\
	\u{3cf}\x05\u{9a}\x4e\x02\u{3cf}\u{3d0}\x07\x0d\x02\x02\u{3d0}\u{3d2}\x03\
	\x02\x02\x02\u{3d1}\u{3c9}\x03\x02\x02\x02\u{3d1}\u{3cd}\x03\x02\x02\x02\
	\u{3d2}\u{9b}\x03\x02\x02\x02\u{3d3}\u{3d4}\x05\u{11a}\u{8e}\x02\u{3d4}\
	\u{3d5}\x07\x4a\x02\x02\u{3d5}\u{3d6}\x05\u{118}\u{8d}\x02\u{3d6}\u{9d}\
	\x03\x02\x02\x02\u{3d7}\u{3df}\x05\u{a6}\x54\x02\u{3d8}\u{3df}\x05\u{aa}\
	\x56\x02\u{3d9}\u{3df}\x05\u{a0}\x51\x02\u{3da}\u{3db}\x07\x0c\x02\x02\u{3db}\
	\u{3dc}\x05\u{9e}\x50\x02\u{3dc}\u{3dd}\x07\x0d\x02\x02\u{3dd}\u{3df}\x03\
	\x02\x02\x02\u{3de}\u{3d7}\x03\x02\x02\x02\u{3de}\u{3d8}\x03\x02\x02\x02\
	\u{3de}\u{3d9}\x03\x02\x02\x02\u{3de}\u{3da}\x03\x02\x02\x02\u{3df}\u{9f}\
	\x03\x02\x02\x02\u{3e0}\u{3e1}\x07\x38\x02\x02\u{3e1}\u{3e2}\x07\x0e\x02\
	\x02\u{3e2}\u{3e3}\x05\x76\x3c\x02\u{3e3}\u{3e4}\x07\x0f\x02\x02\u{3e4}\
	\u{3e5}\x07\x10\x02\x02\u{3e5}\u{3e6}\x05\u{a2}\x52\x02\u{3e6}\u{a1}\x03\
	\x02\x02\x02\u{3e7}\u{3ed}\x05\u{a6}\x54\x02\u{3e8}\u{3e9}\x07\x0c\x02\x02\
	\u{3e9}\u{3ea}\x05\u{aa}\x56\x02\u{3ea}\u{3eb}\x07\x0d\x02\x02\u{3eb}\u{3ed}\
	\x03\x02\x02\x02\u{3ec}\u{3e7}\x03\x02\x02\x02\u{3ec}\u{3e8}\x03\x02\x02\
	\x02\u{3ed}\u{a3}\x03\x02\x02\x02\u{3ee}\u{3f4}\x05\u{a6}\x54\x02\u{3ef}\
	\u{3f0}\x07\x0c\x02\x02\u{3f0}\u{3f1}\x05\u{ac}\x57\x02\u{3f1}\u{3f2}\x07\
	\x0d\x02\x02\u{3f2}\u{3f4}\x03\x02\x02\x02\u{3f3}\u{3ee}\x03\x02\x02\x02\
	\u{3f3}\u{3ef}\x03\x02\x02\x02\u{3f4}\u{a5}\x03\x02\x02\x02\u{3f5}\u{3fe}\
	\x05\u{110}\u{89}\x02\u{3f6}\u{3fe}\x05\u{114}\u{8b}\x02\u{3f7}\u{3f8}\x05\
	\u{112}\u{8a}\x02\u{3f8}\u{3f9}\x07\x0c\x02\x02\u{3f9}\u{3fa}\x05\u{a8}\
	\x55\x02\u{3fa}\u{3fb}\x07\x0d\x02\x02\u{3fb}\u{3fe}\x03\x02\x02\x02\u{3fc}\
	\u{3fe}\x05\u{130}\u{99}\x02\u{3fd}\u{3f5}\x03\x02\x02\x02\u{3fd}\u{3f6}\
	\x03\x02\x02\x02\u{3fd}\u{3f7}\x03\x02\x02\x02\u{3fd}\u{3fc}\x03\x02\x02\
	\x02\u{3fe}\u{a7}\x03\x02\x02\x02\u{3ff}\u{404}\x05\u{a6}\x54\x02\u{400}\
	\u{401}\x07\x04\x02\x02\u{401}\u{403}\x05\u{a6}\x54\x02\u{402}\u{400}\x03\
	\x02\x02\x02\u{403}\u{406}\x03\x02\x02\x02\u{404}\u{402}\x03\x02\x02\x02\
	\u{404}\u{405}\x03\x02\x02\x02\u{405}\u{a9}\x03\x02\x02\x02\u{406}\u{404}\
	\x03\x02\x02\x02\u{407}\u{408}\x05\u{a4}\x53\x02\u{408}\u{409}\x07\x47\x02\
	\x02\u{409}\u{40a}\x05\u{a6}\x54\x02\u{40a}\u{ab}\x03\x02\x02\x02\u{40b}\
	\u{40c}\x08\x57\x01\x02\u{40c}\u{40d}\x05\u{a4}\x53\x02\u{40d}\u{40e}\x07\
	\x48\x02\x02\u{40e}\u{40f}\x05\u{a6}\x54\x02\u{40f}\u{415}\x03\x02\x02\x02\
	\u{410}\u{411}\x0c\x03\x02\x02\u{411}\u{412}\x07\x48\x02\x02\u{412}\u{414}\
	\x05\u{a6}\x54\x02\u{413}\u{410}\x03\x02\x02\x02\u{414}\u{417}\x03\x02\x02\
	\x02\u{415}\u{413}\x03\x02\x02\x02\u{415}\u{416}\x03\x02\x02\x02\u{416}\
	\u{ad}\x03\x02\x02\x02\u{417}\u{415}\x03\x02\x02\x02\u{418}\u{41b}\x05\u{b0}\
	\x59\x02\u{419}\u{41b}\x05\u{9a}\x4e\x02\u{41a}\u{418}\x03\x02\x02\x02\u{41a}\
	\u{419}\x03\x02\x02\x02\u{41b}\u{af}\x03\x02\x02\x02\u{41c}\u{41f}\x05\u{b2}\
	\x5a\x02\u{41d}\u{41f}\x05\u{f4}\x7b\x02\u{41e}\u{41c}\x03\x02\x02\x02\u{41e}\
	\u{41d}\x03\x02\x02\x02\u{41f}\u{b1}\x03\x02\x02\x02\u{420}\u{421}\x07\x3b\
	\x02\x02\u{421}\u{422}\x07\x0e\x02\x02\u{422}\u{423}\x05\x76\x3c\x02\u{423}\
	\u{424}\x07\x0f\x02\x02\u{424}\u{425}\x07\x10\x02\x02\u{425}\u{426}\x05\
	\u{f4}\x7b\x02\u{426}\u{b3}\x03\x02\x02\x02\u{427}\u{42a}\x05\u{b6}\x5c\
	\x02\u{428}\u{42a}\x05\u{ee}\x78\x02\u{429}\u{427}\x03\x02\x02\x02\u{429}\
	\u{428}\x03\x02\x02\x02\u{42a}\u{b5}\x03\x02\x02\x02\u{42b}\u{42e}\x05\u{b8}\
	\x5d\x02\u{42c}\u{42e}\x05\u{c2}\x62\x02\u{42d}\u{42b}\x03\x02\x02\x02\u{42d}\
	\u{42c}\x03\x02\x02\x02\u{42e}\u{b7}\x03\x02\x02\x02\u{42f}\u{432}\x05\u{ba}\
	\x5e\x02\u{430}\u{432}\x05\u{bc}\x5f\x02\u{431}\u{42f}\x03\x02\x02\x02\u{431}\
	\u{430}\x03\x02\x02\x02\u{432}\u{b9}\x03\x02\x02\x02\u{433}\u{434}\x05\u{c2}\
	\x62\x02\u{434}\u{435}\x05\u{10a}\u{86}\x02\u{435}\u{436}\x05\u{c2}\x62\
	\x02\u{436}\u{bb}\x03\x02\x02\x02\u{437}\u{43a}\x05\u{be}\x60\x02\u{438}\
	\u{43a}\x05\u{c0}\x61\x02\u{439}\u{437}\x03\x02\x02\x02\u{439}\u{438}\x03\
	\x02\x02\x02\u{43a}\u{bd}\x03\x02\x02\x02\u{43b}\u{43c}\x08\x60\x01\x02\
	\u{43c}\u{43d}\x05\u{c2}\x62\x02\u{43d}\u{43e}\x07\x2e\x02\x02\u{43e}\u{43f}\
	\x05\u{c2}\x62\x02\u{43f}\u{445}\x03\x02\x02\x02\u{440}\u{441}\x0c\x03\x02\
	\x02\u{441}\u{442}\x07\x2e\x02\x02\u{442}\u{444}\x05\u{c2}\x62\x02\u{443}\
	\u{440}\x03\x02\x02\x02\u{444}\u{447}\x03\x02\x02\x02\u{445}\u{443}\x03\
	\x02\x02\x02\u{445}\u{446}\x03\x02\x02\x02\u{446}\u{bf}\x03\x02\x02\x02\
	\u{447}\u{445}\x03\x02\x02\x02\u{448}\u{449}\x08\x61\x01\x02\u{449}\u{44a}\
	\x05\u{c2}\x62\x02\u{44a}\u{44b}\x07\x2f\x02\x02\u{44b}\u{44c}\x05\u{c2}\
	\x62\x02\u{44c}\u{452}\x03\x02\x02\x02\u{44d}\u{44e}\x0c\x03\x02\x02\u{44e}\
	\u{44f}\x07\x2f\x02\x02\u{44f}\u{451}\x05\u{c2}\x62\x02\u{450}\u{44d}\x03\
	\x02\x02\x02\u{451}\u{454}\x03\x02\x02\x02\u{452}\u{450}\x03\x02\x02\x02\
	\u{452}\u{453}\x03\x02\x02\x02\u{453}\u{c1}\x03\x02\x02\x02\u{454}\u{452}\
	\x03\x02\x02\x02\u{455}\u{45d}\x05\u{c4}\x63\x02\u{456}\u{45d}\x05\u{c8}\
	\x65\x02\u{457}\u{45d}\x05\u{cc}\x67\x02\u{458}\u{459}\x07\x0c\x02\x02\u{459}\
	\u{45a}\x05\u{b6}\x5c\x02\u{45a}\u{45b}\x07\x0d\x02\x02\u{45b}\u{45d}\x03\
	\x02\x02\x02\u{45c}\u{455}\x03\x02\x02\x02\u{45c}\u{456}\x03\x02\x02\x02\
	\u{45c}\u{457}\x03\x02\x02\x02\u{45c}\u{458}\x03\x02\x02\x02\u{45d}\u{c3}\
	\x03\x02\x02\x02\u{45e}\u{45f}\x05\u{108}\u{85}\x02\u{45f}\u{460}\x07\x0e\
	\x02\x02\u{460}\u{461}\x05\u{c6}\x64\x02\u{461}\u{462}\x07\x0f\x02\x02\u{462}\
	\u{463}\x07\x10\x02\x02\u{463}\u{464}\x05\u{c2}\x62\x02\u{464}\u{c5}\x03\
	\x02\x02\x02\u{465}\u{46a}\x05\u{130}\u{99}\x02\u{466}\u{467}\x07\x04\x02\
	\x02\u{467}\u{469}\x05\u{130}\u{99}\x02\u{468}\u{466}\x03\x02\x02\x02\u{469}\
	\u{46c}\x03\x02\x02\x02\u{46a}\u{468}\x03\x02\x02\x02\u{46a}\u{46b}\x03\
	\x02\x02\x02\u{46b}\u{c7}\x03\x02\x02\x02\u{46c}\u{46a}\x03\x02\x02\x02\
	\u{46d}\u{46e}\x05\u{10e}\u{88}\x02\u{46e}\u{46f}\x05\u{c2}\x62\x02\u{46f}\
	\u{472}\x03\x02\x02\x02\u{470}\u{472}\x05\u{ca}\x66\x02\u{471}\u{46d}\x03\
	\x02\x02\x02\u{471}\u{470}\x03\x02\x02\x02\u{472}\u{c9}\x03\x02\x02\x02\
	\u{473}\u{474}\x05\u{e4}\x73\x02\u{474}\u{475}\x07\x39\x02\x02\u{475}\u{476}\
	\x05\u{e4}\x73\x02\u{476}\u{cb}\x03\x02\x02\x02\u{477}\u{47b}\x05\u{ce}\
	\x68\x02\u{478}\u{47b}\x05\u{d0}\x69\x02\u{479}\u{47b}\x05\u{d6}\x6c\x02\
	\u{47a}\u{477}\x03\x02\x02\x02\u{47a}\u{478}\x03\x02\x02\x02\u{47a}\u{479}\
	\x03\x02\x02\x02\u{47b}\u{cd}\x03\x02\x02\x02\u{47c}\u{47d}\x05\u{d8}\x6d\
	\x02\u{47d}\u{cf}\x03\x02\x02\x02\u{47e}\u{481}\x05\u{d2}\x6a\x02\u{47f}\
	\u{481}\x05\u{d4}\x6b\x02\u{480}\u{47e}\x03\x02\x02\x02\u{480}\u{47f}\x03\
	\x02\x02\x02\u{481}\u{d1}\x03\x02\x02\x02\u{482}\u{483}\x05\u{da}\x6e\x02\
	\u{483}\u{d3}\x03\x02\x02\x02\u{484}\u{485}\x05\u{e4}\x73\x02\u{485}\u{486}\
	\x05\u{120}\u{91}\x02\u{486}\u{487}\x05\u{e4}\x73\x02\u{487}\u{d5}\x03\x02\
	\x02\x02\u{488}\u{489}\x05\u{e0}\x71\x02\u{489}\u{d7}\x03\x02\x02\x02\u{48a}\
	\u{491}\x05\u{122}\u{92}\x02\u{48b}\u{48c}\x05\u{124}\u{93}\x02\u{48c}\u{48d}\
	\x07\x0c\x02\x02\u{48d}\u{48e}\x05\u{e2}\x72\x02\u{48e}\u{48f}\x07\x0d\x02\
	\x02\u{48f}\u{491}\x03\x02\x02\x02\u{490}\u{48a}\x03\x02\x02\x02\u{490}\
	\u{48b}\x03\x02\x02\x02\u{491}\u{d9}\x03\x02\x02\x02\u{492}\u{495}\x05\u{12e}\
	\u{98}\x02\u{493}\u{495}\x05\u{dc}\x6f\x02\u{494}\u{492}\x03\x02\x02\x02\
	\u{494}\u{493}\x03\x02\x02\x02\u{495}\u{db}\x03\x02\x02\x02\u{496}\u{497}\
	\x05\u{de}\x70\x02\u{497}\u{dd}\x03\x02\x02\x02\u{498}\u{49f}\x05\u{12a}\
	\u{96}\x02\u{499}\u{49a}\x05\u{12c}\u{97}\x02\u{49a}\u{49b}\x07\x0c\x02\
	\x02\u{49b}\u{49c}\x05\u{e2}\x72\x02\u{49c}\u{49d}\x07\x0d\x02\x02\u{49d}\
	\u{49f}\x03\x02\x02\x02\u{49e}\u{498}\x03\x02\x02\x02\u{49e}\u{499}\x03\
	\x02\x02\x02\u{49f}\u{df}\x03\x02\x02\x02\u{4a0}\u{4a7}\x05\u{126}\u{94}\
	\x02\u{4a1}\u{4a2}\x05\u{128}\u{95}\x02\u{4a2}\u{4a3}\x07\x0c\x02\x02\u{4a3}\
	\u{4a4}\x05\u{e2}\x72\x02\u{4a4}\u{4a5}\x07\x0d\x02\x02\u{4a5}\u{4a7}\x03\
	\x02\x02\x02\u{4a6}\u{4a0}\x03\x02\x02\x02\u{4a6}\u{4a1}\x03\x02\x02\x02\
	\u{4a7}\u{e1}\x03\x02\x02\x02\u{4a8}\u{4ad}\x05\u{e4}\x73\x02\u{4a9}\u{4aa}\
	\x07\x04\x02\x02\u{4aa}\u{4ac}\x05\u{e4}\x73\x02\u{4ab}\u{4a9}\x03\x02\x02\
	\x02\u{4ac}\u{4af}\x03\x02\x02\x02\u{4ad}\u{4ab}\x03\x02\x02\x02\u{4ad}\
	\u{4ae}\x03\x02\x02\x02\u{4ae}\u{e3}\x03\x02\x02\x02\u{4af}\u{4ad}\x03\x02\
	\x02\x02\u{4b0}\u{4b6}\x05\u{e6}\x74\x02\u{4b1}\u{4b6}\x05\u{130}\u{99}\
	\x02\u{4b2}\u{4b6}\x05\u{e8}\x75\x02\u{4b3}\u{4b6}\x05\u{ea}\x76\x02\u{4b4}\
	\u{4b6}\x05\u{ec}\x77\x02\u{4b5}\u{4b0}\x03\x02\x02\x02\u{4b5}\u{4b1}\x03\
	\x02\x02\x02\u{4b5}\u{4b2}\x03\x02\x02\x02\u{4b5}\u{4b3}\x03\x02\x02\x02\
	\u{4b5}\u{4b4}\x03\x02\x02\x02\u{4b6}\u{e5}\x03\x02\x02\x02\u{4b7}\u{4bb}\
	\x05\u{d8}\x6d\x02\u{4b8}\u{4bb}\x05\u{da}\x6e\x02\u{4b9}\u{4bb}\x05\u{e0}\
	\x71\x02\u{4ba}\u{4b7}\x03\x02\x02\x02\u{4ba}\u{4b8}\x03\x02\x02\x02\u{4ba}\
	\u{4b9}\x03\x02\x02\x02\u{4bb}\u{e7}\x03\x02\x02\x02\u{4bc}\u{4bd}\x07\x1a\
	\x02\x02\u{4bd}\u{4be}\x05\x66\x34\x02\u{4be}\u{4bf}\x07\x04\x02\x02\u{4bf}\
	\u{4c0}\x05\u{e4}\x73\x02\u{4c0}\u{4c1}\x07\x04\x02\x02\u{4c1}\u{4c2}\x05\
	\u{e4}\x73\x02\u{4c2}\u{4c3}\x07\x0d\x02\x02\u{4c3}\u{e9}\x03\x02\x02\x02\
	\u{4c4}\u{4c5}\x07\x1b\x02\x02\u{4c5}\u{4c6}\x05\u{8c}\x47\x02\u{4c6}\u{4c7}\
	\x07\x04\x02\x02\u{4c7}\u{4c8}\x05\u{e4}\x73\x02\u{4c8}\u{4c9}\x07\x0d\x02\
	\x02\u{4c9}\u{4d1}\x03\x02\x02\x02\u{4ca}\u{4cb}\x07\x1c\x02\x02\u{4cb}\
	\u{4cc}\x05\u{84}\x43\x02\u{4cc}\u{4cd}\x07\x04\x02\x02\u{4cd}\u{4ce}\x05\
	\u{e4}\x73\x02\u{4ce}\u{4cf}\x07\x0d\x02\x02\u{4cf}\u{4d1}\x03\x02\x02\x02\
	\u{4d0}\u{4c4}\x03\x02\x02\x02\u{4d0}\u{4ca}\x03\x02\x02\x02\u{4d1}\u{eb}\
	\x03\x02\x02\x02\u{4d2}\u{4d8}\x07\x14\x02\x02\u{4d3}\u{4d4}\x07\x15\x02\
	\x02\u{4d4}\u{4d5}\x05\u{e2}\x72\x02\u{4d5}\u{4d6}\x07\x16\x02\x02\u{4d6}\
	\u{4d8}\x03\x02\x02\x02\u{4d7}\u{4d2}\x03\x02\x02\x02\u{4d7}\u{4d3}\x03\
	\x02\x02\x02\u{4d8}\u{ed}\x03\x02\x02\x02\u{4d9}\u{4da}\x05\u{f0}\x79\x02\
	\u{4da}\u{4db}\x07\x4b\x02\x02\u{4db}\u{4dc}\x05\u{f0}\x79\x02\u{4dc}\u{4e2}\
	\x03\x02\x02\x02\u{4dd}\u{4de}\x07\x0c\x02\x02\u{4de}\u{4df}\x05\u{ee}\x78\
	\x02\u{4df}\u{4e0}\x07\x0d\x02\x02\u{4e0}\u{4e2}\x03\x02\x02\x02\u{4e1}\
	\u{4d9}\x03\x02\x02\x02\u{4e1}\u{4dd}\x03\x02\x02\x02\u{4e2}\u{ef}\x03\x02\
	\x02\x02\u{4e3}\u{4e9}\x07\x13\x02\x02\u{4e4}\u{4e5}\x07\x0e\x02\x02\u{4e5}\
	\u{4e6}\x05\u{f2}\x7a\x02\u{4e6}\u{4e7}\x07\x0f\x02\x02\u{4e7}\u{4e9}\x03\
	\x02\x02\x02\u{4e8}\u{4e3}\x03\x02\x02\x02\u{4e8}\u{4e4}\x03\x02\x02\x02\
	\u{4e9}\u{f1}\x03\x02\x02\x02\u{4ea}\u{4ef}\x05\u{b6}\x5c\x02\u{4eb}\u{4ec}\
	\x07\x04\x02\x02\u{4ec}\u{4ee}\x05\u{b6}\x5c\x02\u{4ed}\u{4eb}\x03\x02\x02\
	\x02\u{4ee}\u{4f1}\x03\x02\x02\x02\u{4ef}\u{4ed}\x03\x02\x02\x02\u{4ef}\
	\u{4f0}\x03\x02\x02\x02\u{4f0}\u{f3}\x03\x02\x02\x02\u{4f1}\u{4ef}\x03\x02\
	\x02\x02\u{4f2}\u{4f8}\x05\u{f6}\x7c\x02\u{4f3}\u{4f4}\x07\x0c\x02\x02\u{4f4}\
	\u{4f5}\x05\u{f6}\x7c\x02\u{4f5}\u{4f6}\x07\x0d\x02\x02\u{4f6}\u{4f8}\x03\
	\x02\x02\x02\u{4f7}\u{4f2}\x03\x02\x02\x02\u{4f7}\u{4f3}\x03\x02\x02\x02\
	\u{4f8}\u{f5}\x03\x02\x02\x02\u{4f9}\u{4fa}\x08\x7c\x01\x02\u{4fa}\u{4fb}\
	\x05\u{f8}\x7d\x02\u{4fb}\u{501}\x03\x02\x02\x02\u{4fc}\u{4fd}\x0c\x03\x02\
	\x02\u{4fd}\u{4fe}\x07\x2e\x02\x02\u{4fe}\u{500}\x05\u{f8}\x7d\x02\u{4ff}\
	\u{4fc}\x03\x02\x02\x02\u{500}\u{503}\x03\x02\x02\x02\u{501}\u{4ff}\x03\
	\x02\x02\x02\u{501}\u{502}\x03\x02\x02\x02\u{502}\u{f7}\x03\x02\x02\x02\
	\u{503}\u{501}\x03\x02\x02\x02\u{504}\u{509}\x05\u{cc}\x67\x02\u{505}\u{506}\
	\x07\x36\x02\x02\u{506}\u{509}\x05\u{cc}\x67\x02\u{507}\u{509}\x05\u{ca}\
	\x66\x02\u{508}\u{504}\x03\x02\x02\x02\u{508}\u{505}\x03\x02\x02\x02\u{508}\
	\u{507}\x03\x02\x02\x02\u{509}\u{f9}\x03\x02\x02\x02\u{50a}\u{50e}\x05\u{108}\
	\u{85}\x02\u{50b}\u{50e}\x05\u{fc}\x7f\x02\u{50c}\u{50e}\x05\u{fe}\u{80}\
	\x02\u{50d}\u{50a}\x03\x02\x02\x02\u{50d}\u{50b}\x03\x02\x02\x02\u{50d}\
	\u{50c}\x03\x02\x02\x02\u{50e}\u{fb}\x03\x02\x02\x02\u{50f}\u{510}\x09\x02\
	\x02\x02\u{510}\u{fd}\x03\x02\x02\x02\u{511}\u{512}\x09\x03\x02\x02\u{512}\
	\u{ff}\x03\x02\x02\x02\u{513}\u{518}\x07\x3a\x02\x02\u{514}\u{518}\x07\x39\
	\x02\x02\u{515}\u{518}\x05\u{10a}\u{86}\x02\u{516}\u{518}\x07\x46\x02\x02\
	\u{517}\u{513}\x03\x02\x02\x02\u{517}\u{514}\x03\x02\x02\x02\u{517}\u{515}\
	\x03\x02\x02\x02\u{517}\u{516}\x03\x02\x02\x02\u{518}\u{101}\x03\x02\x02\
	\x02\u{519}\u{51c}\x05\u{10e}\u{88}\x02\u{51a}\u{51c}\x05\u{104}\u{83}\x02\
	\u{51b}\u{519}\x03\x02\x02\x02\u{51b}\u{51a}\x03\x02\x02\x02\u{51c}\u{103}\
	\x03\x02\x02\x02\u{51d}\u{51e}\x09\x04\x02\x02\u{51e}\u{105}\x03\x02\x02\
	\x02\u{51f}\u{522}\x05\u{10a}\u{86}\x02\u{520}\u{522}\x07\x46\x02\x02\u{521}\
	\u{51f}\x03\x02\x02\x02\u{521}\u{520}\x03\x02\x02\x02\u{522}\u{107}\x03\
	\x02\x02\x02\u{523}\u{524}\x09\x05\x02\x02\u{524}\u{109}\x03\x02\x02\x02\
	\u{525}\u{526}\x09\x06\x02\x02\u{526}\u{10b}\x03\x02\x02\x02\u{527}\u{528}\
	\x09\x07\x02\x02\u{528}\u{10d}\x03\x02\x02\x02\u{529}\u{52a}\x07\x36\x02\
	\x02\u{52a}\u{10f}\x03\x02\x02\x02\u{52b}\u{52c}\x05\u{112}\u{8a}\x02\u{52c}\
	\u{111}\x03\x02\x02\x02\u{52d}\u{52e}\x05\u{18a}\u{c6}\x02\u{52e}\u{113}\
	\x03\x02\x02\x02\u{52f}\u{530}\x07\x5d\x02\x02\u{530}\u{115}\x03\x02\x02\
	\x02\u{531}\u{532}\x05\u{18e}\u{c8}\x02\u{532}\u{117}\x03\x02\x02\x02\u{533}\
	\u{536}\x05\u{11a}\u{8e}\x02\u{534}\u{536}\x05\u{12a}\u{96}\x02\u{535}\u{533}\
	\x03\x02\x02\x02\u{535}\u{534}\x03\x02\x02\x02\u{536}\u{119}\x03\x02\x02\
	\x02\u{537}\u{53a}\x05\u{122}\u{92}\x02\u{538}\u{53a}\x05\u{126}\u{94}\x02\
	\u{539}\u{537}\x03\x02\x02\x02\u{539}\u{538}\x03\x02\x02\x02\u{53a}\u{11b}\
	\x03\x02\x02\x02\u{53b}\u{53c}\x07\x5d\x02\x02\u{53c}\u{11d}\x03\x02\x02\
	\x02\u{53d}\u{53e}\x07\x5d\x02\x02\u{53e}\u{11f}\x03\x02\x02\x02\u{53f}\
	\u{540}\x09\x08\x02\x02\u{540}\u{121}\x03\x02\x02\x02\u{541}\u{542}\x05\
	\u{124}\u{93}\x02\u{542}\u{123}\x03\x02\x02\x02\u{543}\u{544}\x05\u{18a}\
	\u{c6}\x02\u{544}\u{125}\x03\x02\x02\x02\u{545}\u{546}\x05\u{128}\u{95}\
	\x02\u{546}\u{127}\x03\x02\x02\x02\u{547}\u{548}\x05\u{18e}\u{c8}\x02\u{548}\
	\u{129}\x03\x02\x02\x02\u{549}\u{54a}\x05\u{12c}\u{97}\x02\u{54a}\u{12b}\
	\x03\x02\x02\x02\u{54b}\u{54c}\x05\u{18c}\u{c7}\x02\u{54c}\u{12d}\x03\x02\
	\x02\x02\u{54d}\u{550}\x05\u{190}\u{c9}\x02\u{54e}\u{550}\x07\x62\x02\x02\
	\u{54f}\u{54d}\x03\x02\x02\x02\u{54f}\u{54e}\x03\x02\x02\x02\u{550}\u{12f}\
	\x03\x02\x02\x02\u{551}\u{552}\x07\x5f\x02\x02\u{552}\u{131}\x03\x02\x02\
	\x02\u{553}\u{55c}\x05\u{136}\u{9c}\x02\u{554}\u{55c}\x05\u{144}\u{a3}\x02\
	\u{555}\u{55c}\x05\u{148}\u{a5}\x02\u{556}\u{55c}\x07\x60\x02\x02\u{557}\
	\u{558}\x07\x0e\x02\x02\u{558}\u{559}\x05\u{134}\u{9b}\x02\u{559}\u{55a}\
	\x07\x0f\x02\x02\u{55a}\u{55c}\x03\x02\x02\x02\u{55b}\u{553}\x03\x02\x02\
	\x02\u{55b}\u{554}\x03\x02\x02\x02\u{55b}\u{555}\x03\x02\x02\x02\u{55b}\
	\u{556}\x03\x02\x02\x02\u{55b}\u{557}\x03\x02\x02\x02\u{55c}\u{133}\x03\
	\x02\x02\x02\u{55d}\u{562}\x05\u{132}\u{9a}\x02\u{55e}\u{55f}\x07\x04\x02\
	\x02\u{55f}\u{561}\x05\u{132}\u{9a}\x02\u{560}\u{55e}\x03\x02\x02\x02\u{561}\
	\u{564}\x03\x02\x02\x02\u{562}\u{560}\x03\x02\x02\x02\u{562}\u{563}\x03\
	\x02\x02\x02\u{563}\u{135}\x03\x02\x02\x02\u{564}\u{562}\x03\x02\x02\x02\
	\u{565}\u{568}\x05\u{188}\u{c5}\x02\u{566}\u{568}\x05\u{138}\u{9d}\x02\u{567}\
	\u{565}\x03\x02\x02\x02\u{567}\u{566}\x03\x02\x02\x02\u{568}\u{137}\x03\
	\x02\x02\x02\u{569}\u{56a}\x07\x1d\x02\x02\u{56a}\u{56b}\x05\u{13a}\u{9e}\
	\x02\u{56b}\u{56c}\x07\x04\x02\x02\u{56c}\u{56d}\x05\u{158}\u{ad}\x02\u{56d}\
	\u{56e}\x07\x04\x02\x02\u{56e}\u{56f}\x05\u{13c}\u{9f}\x02\u{56f}\u{570}\
	\x07\x0d\x02\x02\u{570}\u{139}\x03\x02\x02\x02\u{571}\u{572}\x05\u{18a}\
	\u{c6}\x02\u{572}\u{13b}\x03\x02\x02\x02\u{573}\u{579}\x07\x13\x02\x02\u{574}\
	\u{575}\x07\x0e\x02\x02\u{575}\u{576}\x05\u{13e}\u{a0}\x02\u{576}\u{577}\
	\x07\x0f\x02\x02\u{577}\u{579}\x03\x02\x02\x02\u{578}\u{573}\x03\x02\x02\
	\x02\u{578}\u{574}\x03\x02\x02\x02\u{579}\u{13d}\x03\x02\x02\x02\u{57a}\
	\u{57f}\x05\u{140}\u{a1}\x02\u{57b}\u{57c}\x07\x04\x02\x02\u{57c}\u{57e}\
	\x05\u{140}\u{a1}\x02\u{57d}\u{57b}\x03\x02\x02\x02\u{57e}\u{581}\x03\x02\
	\x02\x02\u{57f}\u{57d}\x03\x02\x02\x02\u{57f}\u{580}\x03\x02\x02\x02\u{580}\
	\u{13f}\x03\x02\x02\x02\u{581}\u{57f}\x03\x02\x02\x02\u{582}\u{584}\x05\
	\u{132}\u{9a}\x02\u{583}\u{585}\x05\u{142}\u{a2}\x02\u{584}\u{583}\x03\x02\
	\x02\x02\u{584}\u{585}\x03\x02\x02\x02\u{585}\u{141}\x03\x02\x02\x02\u{586}\
	\u{587}\x07\x10\x02\x02\u{587}\u{588}\x05\u{184}\u{c3}\x02\u{588}\u{143}\
	\x03\x02\x02\x02\u{589}\u{58a}\x07\x1e\x02\x02\u{58a}\u{58c}\x05\u{146}\
	\u{a4}\x02\u{58b}\u{58d}\x05\u{156}\u{ac}\x02\u{58c}\u{58b}\x03\x02\x02\
	\x02\u{58c}\u{58d}\x03\x02\x02\x02\u{58d}\u{58e}\x03\x02\x02\x02\u{58e}\
	\u{58f}\x07\x0d\x02\x02\u{58f}\u{145}\x03\x02\x02\x02\u{590}\u{591}\x07\
	\x60\x02\x02\u{591}\u{147}\x03\x02\x02\x02\u{592}\u{596}\x05\u{14a}\u{a6}\
	\x02\u{593}\u{596}\x05\u{14e}\u{a8}\x02\u{594}\u{596}\x05\u{152}\u{aa}\x02\
	\u{595}\u{592}\x03\x02\x02\x02\u{595}\u{593}\x03\x02\x02\x02\u{595}\u{594}\
	\x03\x02\x02\x02\u{596}\u{149}\x03\x02\x02\x02\u{597}\u{598}\x07\x1f\x02\
	\x02\u{598}\u{59a}\x05\u{192}\u{ca}\x02\u{599}\u{59b}\x05\u{14c}\u{a7}\x02\
	\u{59a}\u{599}\x03\x02\x02\x02\u{59a}\u{59b}\x03\x02\x02\x02\u{59b}\u{59c}\
	\x03\x02\x02\x02\u{59c}\u{59d}\x07\x0d\x02\x02\u{59d}\u{14b}\x03\x02\x02\
	\x02\u{59e}\u{59f}\x07\x04\x02\x02\u{59f}\u{5a0}\x05\u{188}\u{c5}\x02\u{5a0}\
	\u{14d}\x03\x02\x02\x02\u{5a1}\u{5a2}\x07\x20\x02\x02\u{5a2}\u{5a4}\x05\
	\u{150}\u{a9}\x02\u{5a3}\u{5a5}\x05\u{156}\u{ac}\x02\u{5a4}\u{5a3}\x03\x02\
	\x02\x02\u{5a4}\u{5a5}\x03\x02\x02\x02\u{5a5}\u{5a6}\x03\x02\x02\x02\u{5a6}\
	\u{5a7}\x07\x0d\x02\x02\u{5a7}\u{14f}\x03\x02\x02\x02\u{5a8}\u{5a9}\x07\
	\x60\x02\x02\u{5a9}\u{151}\x03\x02\x02\x02\u{5aa}\u{5ab}\x07\x21\x02\x02\
	\u{5ab}\u{5ad}\x05\u{154}\u{ab}\x02\u{5ac}\u{5ae}\x05\u{156}\u{ac}\x02\u{5ad}\
	\u{5ac}\x03\x02\x02\x02\u{5ad}\u{5ae}\x03\x02\x02\x02\u{5ae}\u{5af}\x03\
	\x02\x02\x02\u{5af}\u{5b0}\x07\x0d\x02\x02\u{5b0}\u{153}\x03\x02\x02\x02\
	\u{5b1}\u{5b2}\x05\u{18a}\u{c6}\x02\u{5b2}\u{155}\x03\x02\x02\x02\u{5b3}\
	\u{5b4}\x07\x04\x02\x02\u{5b4}\u{5b5}\x05\u{158}\u{ad}\x02\u{5b5}\u{157}\
	\x03\x02\x02\x02\u{5b6}\u{5bd}\x07\x13\x02\x02\u{5b7}\u{5b8}\x07\x0e\x02\
	\x02\u{5b8}\u{5b9}\x05\u{15a}\u{ae}\x02\u{5b9}\u{5ba}\x07\x0f\x02\x02\u{5ba}\
	\u{5bd}\x03\x02\x02\x02\u{5bb}\u{5bd}\x05\u{184}\u{c3}\x02\u{5bc}\u{5b6}\
	\x03\x02\x02\x02\u{5bc}\u{5b7}\x03\x02\x02\x02\u{5bc}\u{5bb}\x03\x02\x02\
	\x02\u{5bd}\u{159}\x03\x02\x02\x02\u{5be}\u{5c3}\x05\u{15c}\u{af}\x02\u{5bf}\
	\u{5c0}\x07\x04\x02\x02\u{5c0}\u{5c2}\x05\u{15c}\u{af}\x02\u{5c1}\u{5bf}\
	\x03\x02\x02\x02\u{5c2}\u{5c5}\x03\x02\x02\x02\u{5c3}\u{5c1}\x03\x02\x02\
	\x02\u{5c3}\u{5c4}\x03\x02\x02\x02\u{5c4}\u{15b}\x03\x02\x02\x02\u{5c5}\
	\u{5c3}\x03\x02\x02\x02\u{5c6}\u{5ca}\x05\u{15e}\u{b0}\x02\u{5c7}\u{5ca}\
	\x05\u{164}\u{b3}\x02\u{5c8}\u{5ca}\x05\u{180}\u{c1}\x02\u{5c9}\u{5c6}\x03\
	\x02\x02\x02\u{5c9}\u{5c7}\x03\x02\x02\x02\u{5c9}\u{5c8}\x03\x02\x02\x02\
	\u{5ca}\u{15d}\x03\x02\x02\x02\u{5cb}\u{5ce}\x05\u{160}\u{b1}\x02\u{5cc}\
	\u{5ce}\x05\u{162}\u{b2}\x02\u{5cd}\u{5cb}\x03\x02\x02\x02\u{5cd}\u{5cc}\
	\x03\x02\x02\x02\u{5ce}\u{15f}\x03\x02\x02\x02\u{5cf}\u{5d0}\x07\x22\x02\
	\x02\u{5d0}\u{5d1}\x05\u{18a}\u{c6}\x02\u{5d1}\u{5d2}\x07\x0d\x02\x02\u{5d2}\
	\u{161}\x03\x02\x02\x02\u{5d3}\u{5d4}\x07\x23\x02\x02\u{5d4}\u{5d5}\x05\
	\u{18a}\u{c6}\x02\u{5d5}\u{5d6}\x07\x0d\x02\x02\u{5d6}\u{163}\x03\x02\x02\
	\x02\u{5d7}\u{5dc}\x05\u{166}\u{b4}\x02\u{5d8}\u{5dc}\x05\u{16c}\u{b7}\x02\
	\u{5d9}\u{5dc}\x05\u{170}\u{b9}\x02\u{5da}\u{5dc}\x05\u{16e}\u{b8}\x02\u{5db}\
	\u{5d7}\x03\x02\x02\x02\u{5db}\u{5d8}\x03\x02\x02\x02\u{5db}\u{5d9}\x03\
	\x02\x02\x02\u{5db}\u{5da}\x03\x02\x02\x02\u{5dc}\u{165}\x03\x02\x02\x02\
	\u{5dd}\u{5de}\x07\x24\x02\x02\u{5de}\u{5df}\x05\u{168}\u{b5}\x02\u{5df}\
	\u{5e0}\x07\x0d\x02\x02\u{5e0}\u{5e3}\x03\x02\x02\x02\u{5e1}\u{5e3}\x05\
	\u{16a}\u{b6}\x02\u{5e2}\u{5dd}\x03\x02\x02\x02\u{5e2}\u{5e1}\x03\x02\x02\
	\x02\u{5e3}\u{167}\x03\x02\x02\x02\u{5e4}\u{5e5}\x07\x60\x02\x02\u{5e5}\
	\u{169}\x03\x02\x02\x02\u{5e6}\u{5e7}\x05\u{13a}\u{9e}\x02\u{5e7}\u{5e8}\
	\x07\x0c\x02\x02\u{5e8}\u{5e9}\x05\u{18a}\u{c6}\x02\u{5e9}\u{5ea}\x07\x04\
	\x02\x02\u{5ea}\u{5eb}\x05\u{184}\u{c3}\x02\u{5eb}\u{5ec}\x07\x0d\x02\x02\
	\u{5ec}\u{16b}\x03\x02\x02\x02\u{5ed}\u{5ee}\x07\x25\x02\x02\u{5ee}\u{5ef}\
	\x07\x0e\x02\x02\u{5ef}\u{5f0}\x05\u{17a}\u{be}\x02\u{5f0}\u{5f1}\x07\x0f\
	\x02\x02\u{5f1}\u{5f2}\x07\x0d\x02\x02\u{5f2}\u{16d}\x03\x02\x02\x02\u{5f3}\
	\u{5f4}\x07\x26\x02\x02\u{5f4}\u{5f5}\x05\u{14a}\u{a6}\x02\u{5f5}\u{5f6}\
	\x07\x0d\x02\x02\u{5f6}\u{16f}\x03\x02\x02\x02\u{5f7}\u{5f8}\x07\x27\x02\
	\x02\u{5f8}\u{5f9}\x05\u{18a}\u{c6}\x02\u{5f9}\u{5fa}\x07\x04\x02\x02\u{5fa}\
	\u{5fb}\x07\x0e\x02\x02\u{5fb}\u{5fc}\x05\u{172}\u{ba}\x02\u{5fc}\u{5fd}\
	\x07\x0f\x02\x02\u{5fd}\u{5fe}\x07\x0d\x02\x02\u{5fe}\u{171}\x03\x02\x02\
	\x02\u{5ff}\u{604}\x05\u{174}\u{bb}\x02\u{600}\u{601}\x07\x04\x02\x02\u{601}\
	\u{603}\x05\u{174}\u{bb}\x02\u{602}\u{600}\x03\x02\x02\x02\u{603}\u{606}\
	\x03\x02\x02\x02\u{604}\u{602}\x03\x02\x02\x02\u{604}\u{605}\x03\x02\x02\
	\x02\u{605}\u{173}\x03\x02\x02\x02\u{606}\u{604}\x03\x02\x02\x02\u{607}\
	\u{60a}\x05\u{124}\u{93}\x02\u{608}\u{60a}\x05\u{130}\u{99}\x02\u{609}\u{607}\
	\x03\x02\x02\x02\u{609}\u{608}\x03\x02\x02\x02\u{60a}\u{175}\x03\x02\x02\
	\x02\u{60b}\u{60c}\x07\x28\x02\x02\u{60c}\u{60e}\x05\u{192}\u{ca}\x02\u{60d}\
	\u{60f}\x05\u{178}\u{bd}\x02\u{60e}\u{60d}\x03\x02\x02\x02\u{60e}\u{60f}\
	\x03\x02\x02\x02\u{60f}\u{610}\x03\x02\x02\x02\u{610}\u{611}\x07\x05\x02\
	\x02\u{611}\u{177}\x03\x02\x02\x02\u{612}\u{613}\x07\x04\x02\x02\u{613}\
	\u{614}\x07\x0e\x02\x02\u{614}\u{615}\x05\u{17a}\u{be}\x02\u{615}\u{616}\
	\x07\x0f\x02\x02\u{616}\u{179}\x03\x02\x02\x02\u{617}\u{61c}\x05\u{188}\
	\u{c5}\x02\u{618}\u{619}\x07\x04\x02\x02\u{619}\u{61b}\x05\u{188}\u{c5}\
	\x02\u{61a}\u{618}\x03\x02\x02\x02\u{61b}\u{61e}\x03\x02\x02\x02\u{61c}\
	\u{61a}\x03\x02\x02\x02\u{61c}\u{61d}\x03\x02\x02\x02\u{61d}\u{17b}\x03\
	\x02\x02\x02\u{61e}\u{61c}\x03\x02\x02\x02\u{61f}\u{626}\x05\u{17e}\u{c0}\
	\x02\u{620}\u{621}\x05\u{17e}\u{c0}\x02\u{621}\u{622}\x07\x10\x02\x02\u{622}\
	\u{623}\x05\u{17c}\u{bf}\x02\u{623}\u{626}\x03\x02\x02\x02\u{624}\u{626}\
	\x05\u{184}\u{c3}\x02\u{625}\u{61f}\x03\x02\x02\x02\u{625}\u{620}\x03\x02\
	\x02\x02\u{625}\u{624}\x03\x02\x02\x02\u{626}\u{17d}\x03\x02\x02\x02\u{627}\
	\u{62e}\x05\u{18a}\u{c6}\x02\u{628}\u{62e}\x05\u{180}\u{c1}\x02\u{629}\u{62e}\
	\x05\u{130}\u{99}\x02\u{62a}\u{62e}\x05\u{190}\u{c9}\x02\u{62b}\u{62e}\x07\
	\x62\x02\x02\u{62c}\u{62e}\x05\u{182}\u{c2}\x02\u{62d}\u{627}\x03\x02\x02\
	\x02\u{62d}\u{628}\x03\x02\x02\x02\u{62d}\u{629}\x03\x02\x02\x02\u{62d}\
	\u{62a}\x03\x02\x02\x02\u{62d}\u{62b}\x03\x02\x02\x02\u{62d}\u{62c}\x03\
	\x02\x02\x02\u{62e}\u{17f}\x03\x02\x02\x02\u{62f}\u{630}\x05\u{18a}\u{c6}\
	\x02\u{630}\u{631}\x07\x0c\x02\x02\u{631}\u{632}\x05\u{186}\u{c4}\x02\u{632}\
	\u{633}\x07\x0d\x02\x02\u{633}\u{181}\x03\x02\x02\x02\u{634}\u{635}\x07\
	\x29\x02\x02\u{635}\u{636}\x05\x1c\x0f\x02\u{636}\u{637}\x07\x0d\x02\x02\
	\u{637}\u{649}\x03\x02\x02\x02\u{638}\u{639}\x07\x2a\x02\x02\u{639}\u{63a}\
	\x05\x64\x33\x02\u{63a}\u{63b}\x07\x0d\x02\x02\u{63b}\u{649}\x03\x02\x02\
	\x02\u{63c}\u{63d}\x07\x2b\x02\x02\u{63d}\u{63e}\x05\u{b4}\x5b\x02\u{63e}\
	\u{63f}\x07\x0d\x02\x02\u{63f}\u{649}\x03\x02\x02\x02\u{640}\u{641}\x07\
	\x2c\x02\x02\u{641}\u{642}\x05\u{f4}\x7b\x02\u{642}\u{643}\x07\x0d\x02\x02\
	\u{643}\u{649}\x03\x02\x02\x02\u{644}\u{645}\x07\x2d\x02\x02\u{645}\u{646}\
	\x05\u{e4}\x73\x02\u{646}\u{647}\x07\x0d\x02\x02\u{647}\u{649}\x03\x02\x02\
	\x02\u{648}\u{634}\x03\x02\x02\x02\u{648}\u{638}\x03\x02\x02\x02\u{648}\
	\u{63c}\x03\x02\x02\x02\u{648}\u{640}\x03\x02\x02\x02\u{648}\u{644}\x03\
	\x02\x02\x02\u{649}\u{183}\x03\x02\x02\x02\u{64a}\u{650}\x07\x13\x02\x02\
	\u{64b}\u{64c}\x07\x0e\x02\x02\u{64c}\u{64d}\x05\u{186}\u{c4}\x02\u{64d}\
	\u{64e}\x07\x0f\x02\x02\u{64e}\u{650}\x03\x02\x02\x02\u{64f}\u{64a}\x03\
	\x02\x02\x02\u{64f}\u{64b}\x03\x02\x02\x02\u{650}\u{185}\x03\x02\x02\x02\
	\u{651}\u{656}\x05\u{17c}\u{bf}\x02\u{652}\u{653}\x07\x04\x02\x02\u{653}\
	\u{655}\x05\u{17c}\u{bf}\x02\u{654}\u{652}\x03\x02\x02\x02\u{655}\u{658}\
	\x03\x02\x02\x02\u{656}\u{654}\x03\x02\x02\x02\u{656}\u{657}\x03\x02\x02\
	\x02\u{657}\u{187}\x03\x02\x02\x02\u{658}\u{656}\x03\x02\x02\x02\u{659}\
	\u{65c}\x05\u{18a}\u{c6}\x02\u{65a}\u{65c}\x07\x52\x02\x02\u{65b}\u{659}\
	\x03\x02\x02\x02\u{65b}\u{65a}\x03\x02\x02\x02\u{65c}\u{189}\x03\x02\x02\
	\x02\u{65d}\u{65e}\x09\x09\x02\x02\u{65e}\u{18b}\x03\x02\x02\x02\u{65f}\
	\u{660}\x07\x5d\x02\x02\u{660}\u{18d}\x03\x02\x02\x02\u{661}\u{662}\x07\
	\x5e\x02\x02\u{662}\u{18f}\x03\x02\x02\x02\u{663}\u{664}\x09\x0a\x02\x02\
	\u{664}\u{191}\x03\x02\x02\x02\u{665}\u{666}\x07\x61\x02\x02\u{666}\u{193}\
	\x03\x02\x02\x02\x7f\u{197}\u{19e}\u{1a7}\u{1b0}\u{1bd}\u{1c8}\u{1d3}\u{1de}\
	\u{1e9}\u{1f4}\u{1fb}\u{201}\u{207}\u{20c}\u{215}\u{221}\u{22e}\u{23b}\u{248}\
	\u{258}\u{25d}\u{26c}\u{27e}\u{283}\u{29e}\u{2a7}\u{2b0}\u{2ba}\u{2c6}\u{2d3}\
	\u{2de}\u{2ea}\u{2f1}\u{2f6}\u{2fd}\u{302}\u{306}\u{30e}\u{31a}\u{327}\u{333}\
	\u{341}\u{346}\u{350}\u{368}\u{36f}\u{376}\u{381}\u{38b}\u{392}\u{399}\u{3a4}\
	\u{3ae}\u{3b8}\u{3bf}\u{3c6}\u{3d1}\u{3de}\u{3ec}\u{3f3}\u{3fd}\u{404}\u{415}\
	\u{41a}\u{41e}\u{429}\u{42d}\u{431}\u{439}\u{445}\u{452}\u{45c}\u{46a}\u{471}\
	\u{47a}\u{480}\u{490}\u{494}\u{49e}\u{4a6}\u{4ad}\u{4b5}\u{4ba}\u{4d0}\u{4d7}\
	\u{4e1}\u{4e8}\u{4ef}\u{4f7}\u{501}\u{508}\u{50d}\u{517}\u{51b}\u{521}\u{535}\
	\u{539}\u{54f}\u{55b}\u{562}\u{567}\u{578}\u{57f}\u{584}\u{58c}\u{595}\u{59a}\
	\u{5a4}\u{5ad}\u{5bc}\u{5c3}\u{5c9}\u{5cd}\u{5db}\u{5e2}\u{604}\u{609}\u{60e}\
	\u{61c}\u{625}\u{62d}\u{648}\u{64f}\u{656}\u{65b}";

