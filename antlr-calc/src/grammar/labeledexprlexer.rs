// Generated from LabeledExpr.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr4rust::atn::ATN;
use antlr4rust::char_stream::CharStream;
use antlr4rust::int_stream::IntStream;
use antlr4rust::tree::ParseTree;
use antlr4rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr4rust::PredictionContextCache;
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::TokenSource;
use antlr4rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr4rust::token::*;
use antlr4rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr4rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr4rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const T__0:i32=1; 
	pub const T__1:i32=2; 
	pub const T__2:i32=3; 
	pub const MUL:i32=4; 
	pub const DIV:i32=5; 
	pub const ADD:i32=6; 
	pub const SUB:i32=7; 
	pub const ID:i32=8; 
	pub const INT:i32=9; 
	pub const NEWLINE:i32=10; 
	pub const WS:i32=11;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;11] = [
		"T__0", "T__1", "T__2", "MUL", "DIV", "ADD", "SUB", "ID", "INT", "NEWLINE", 
		"WS"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;8] = [
		None, Some("'='"), Some("'('"), Some("')'"), Some("'*'"), Some("'/'"), 
		Some("'+'"), Some("'-'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
		None, None, None, None, Some("MUL"), Some("DIV"), Some("ADD"), Some("SUB"), 
		Some("ID"), Some("INT"), Some("NEWLINE"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct LabeledExprLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,LabeledExprLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr4rust::tid! { impl<'input,Input> TidAble<'input> for LabeledExprLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for LabeledExprLexer<'input,Input>{
	type Target = BaseLexer<'input,LabeledExprLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for LabeledExprLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> LabeledExprLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "LabeledExprLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr4rust::recognizer::check_version("0","5");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				LabeledExprLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> LabeledExprLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		LabeledExprLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct LabeledExprLexerActions {
}

impl LabeledExprLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,LabeledExprLexerActions,Input,LocalTokenFactory<'input>>> for LabeledExprLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> LabeledExprLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,LabeledExprLexerActions,Input,LocalTokenFactory<'input>>> for LabeledExprLexerActions{
}
impl<'input> TokenAware<'input> for LabeledExprLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for LabeledExprLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
    }

    fn get_dfa_string(&self) -> String {
        self.base.get_dfa_string()
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
			4, 0, 11, 59, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
			2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
			9, 2, 10, 7, 10, 1, 0, 1, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 3, 1, 4, 
			1, 4, 1, 5, 1, 5, 1, 6, 1, 6, 1, 7, 4, 7, 39, 8, 7, 11, 7, 12, 7, 40, 
			1, 8, 4, 8, 44, 8, 8, 11, 8, 12, 8, 45, 1, 9, 3, 9, 49, 8, 9, 1, 9, 1, 
			9, 1, 10, 4, 10, 54, 8, 10, 11, 10, 12, 10, 55, 1, 10, 1, 10, 0, 0, 11, 
			1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 
			11, 1, 0, 3, 2, 0, 65, 90, 97, 122, 1, 0, 48, 57, 2, 0, 9, 9, 32, 32, 
			62, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 
			0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 0, 0, 15, 
			1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 0, 0, 0, 
			1, 23, 1, 0, 0, 0, 3, 25, 1, 0, 0, 0, 5, 27, 1, 0, 0, 0, 7, 29, 1, 0, 
			0, 0, 9, 31, 1, 0, 0, 0, 11, 33, 1, 0, 0, 0, 13, 35, 1, 0, 0, 0, 15, 
			38, 1, 0, 0, 0, 17, 43, 1, 0, 0, 0, 19, 48, 1, 0, 0, 0, 21, 53, 1, 0, 
			0, 0, 23, 24, 5, 61, 0, 0, 24, 2, 1, 0, 0, 0, 25, 26, 5, 40, 0, 0, 26, 
			4, 1, 0, 0, 0, 27, 28, 5, 41, 0, 0, 28, 6, 1, 0, 0, 0, 29, 30, 5, 42, 
			0, 0, 30, 8, 1, 0, 0, 0, 31, 32, 5, 47, 0, 0, 32, 10, 1, 0, 0, 0, 33, 
			34, 5, 43, 0, 0, 34, 12, 1, 0, 0, 0, 35, 36, 5, 45, 0, 0, 36, 14, 1, 
			0, 0, 0, 37, 39, 7, 0, 0, 0, 38, 37, 1, 0, 0, 0, 39, 40, 1, 0, 0, 0, 
			40, 38, 1, 0, 0, 0, 40, 41, 1, 0, 0, 0, 41, 16, 1, 0, 0, 0, 42, 44, 7, 
			1, 0, 0, 43, 42, 1, 0, 0, 0, 44, 45, 1, 0, 0, 0, 45, 43, 1, 0, 0, 0, 
			45, 46, 1, 0, 0, 0, 46, 18, 1, 0, 0, 0, 47, 49, 5, 13, 0, 0, 48, 47, 
			1, 0, 0, 0, 48, 49, 1, 0, 0, 0, 49, 50, 1, 0, 0, 0, 50, 51, 5, 10, 0, 
			0, 51, 20, 1, 0, 0, 0, 52, 54, 7, 2, 0, 0, 53, 52, 1, 0, 0, 0, 54, 55, 
			1, 0, 0, 0, 55, 53, 1, 0, 0, 0, 55, 56, 1, 0, 0, 0, 56, 57, 1, 0, 0, 
			0, 57, 58, 6, 10, 0, 0, 58, 22, 1, 0, 0, 0, 5, 0, 40, 45, 48, 55, 1, 
			6, 0, 0
		];
	}