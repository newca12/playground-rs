// Generated from propcalc.g4 by ANTLR 4.13.2
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
	pub const AND:i32=2; 
	pub const OR:i32=3; 
	pub const NOT:i32=4; 
	pub const EQ:i32=5; 
	pub const IMPLIES:i32=6; 
	pub const THEREFORE:i32=7; 
	pub const EQUIV:i32=8; 
	pub const LPAREN:i32=9; 
	pub const RPAREN:i32=10; 
	pub const LETTER:i32=11; 
	pub const WS:i32=12;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;12] = [
		"T__0", "AND", "OR", "NOT", "EQ", "IMPLIES", "THEREFORE", "EQUIV", "LPAREN", 
		"RPAREN", "LETTER", "WS"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;11] = [
		None, Some("','"), Some("'^'"), Some("'v'"), Some("'!'"), Some("'='"), 
		Some("'->'"), Some("'|-'"), Some("'<->'"), Some("'('"), Some("')'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;13]  = [
		None, None, Some("AND"), Some("OR"), Some("NOT"), Some("EQ"), Some("IMPLIES"), 
		Some("THEREFORE"), Some("EQUIV"), Some("LPAREN"), Some("RPAREN"), Some("LETTER"), 
		Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct propcalcLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,propcalcLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr4rust::tid! { impl<'input,Input> TidAble<'input> for propcalcLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for propcalcLexer<'input,Input>{
	type Target = BaseLexer<'input,propcalcLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for propcalcLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> propcalcLexer<'input,Input>{
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
        "propcalcLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr4rust::recognizer::check_version("0","3");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				propcalcLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> propcalcLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		propcalcLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct propcalcLexerActions {
}

impl propcalcLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,propcalcLexerActions,Input,LocalTokenFactory<'input>>> for propcalcLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> propcalcLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,propcalcLexerActions,Input,LocalTokenFactory<'input>>> for propcalcLexerActions{
}
impl<'input> TokenAware<'input> for propcalcLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for propcalcLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> i32 {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> i32 {
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
	const _serializedATN: [i32; 532] = [
		4, 0, 12, 59, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 
		4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 
		10, 7, 10, 2, 11, 7, 11, 1, 0, 1, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 
		3, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 
		7, 1, 8, 1, 8, 1, 9, 1, 9, 1, 10, 3, 10, 51, 8, 10, 1, 11, 4, 11, 54, 
		8, 11, 11, 11, 12, 11, 55, 1, 11, 1, 11, 0, 0, 12, 1, 1, 3, 2, 5, 3, 7, 
		4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 11, 23, 12, 1, 0, 2, 
		2, 0, 65, 90, 97, 122, 3, 0, 9, 10, 13, 13, 32, 32, 59, 0, 1, 1, 0, 0, 
		0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 
		0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 
		0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 0, 0, 0, 0, 23, 1, 0, 0, 0, 1, 25, 
		1, 0, 0, 0, 3, 27, 1, 0, 0, 0, 5, 29, 1, 0, 0, 0, 7, 31, 1, 0, 0, 0, 9, 
		33, 1, 0, 0, 0, 11, 35, 1, 0, 0, 0, 13, 38, 1, 0, 0, 0, 15, 41, 1, 0, 
		0, 0, 17, 45, 1, 0, 0, 0, 19, 47, 1, 0, 0, 0, 21, 50, 1, 0, 0, 0, 23, 
		53, 1, 0, 0, 0, 25, 26, 5, 44, 0, 0, 26, 2, 1, 0, 0, 0, 27, 28, 5, 94, 
		0, 0, 28, 4, 1, 0, 0, 0, 29, 30, 5, 118, 0, 0, 30, 6, 1, 0, 0, 0, 31, 
		32, 5, 33, 0, 0, 32, 8, 1, 0, 0, 0, 33, 34, 5, 61, 0, 0, 34, 10, 1, 0, 
		0, 0, 35, 36, 5, 45, 0, 0, 36, 37, 5, 62, 0, 0, 37, 12, 1, 0, 0, 0, 38, 
		39, 5, 124, 0, 0, 39, 40, 5, 45, 0, 0, 40, 14, 1, 0, 0, 0, 41, 42, 5, 
		60, 0, 0, 42, 43, 5, 45, 0, 0, 43, 44, 5, 62, 0, 0, 44, 16, 1, 0, 0, 0, 
		45, 46, 5, 40, 0, 0, 46, 18, 1, 0, 0, 0, 47, 48, 5, 41, 0, 0, 48, 20, 
		1, 0, 0, 0, 49, 51, 7, 0, 0, 0, 50, 49, 1, 0, 0, 0, 51, 22, 1, 0, 0, 0, 
		52, 54, 7, 1, 0, 0, 53, 52, 1, 0, 0, 0, 54, 55, 1, 0, 0, 0, 55, 53, 1, 
		0, 0, 0, 55, 56, 1, 0, 0, 0, 56, 57, 1, 0, 0, 0, 57, 58, 6, 11, 0, 0, 
		58, 24, 1, 0, 0, 0, 3, 0, 50, 55, 1, 0, 1, 0
	];