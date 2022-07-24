// Generated from Math.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const T__0:isize=1; 
	pub const T__1:isize=2; 
	pub const OP_ADD:isize=3; 
	pub const OP_SUB:isize=4; 
	pub const OP_MUL:isize=5; 
	pub const OP_DIV:isize=6; 
	pub const NUM:isize=7; 
	pub const ID:isize=8; 
	pub const WS:isize=9;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;9] = [
		"T__0", "T__1", "OP_ADD", "OP_SUB", "OP_MUL", "OP_DIV", "NUM", "ID", "WS"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;7] = [
		None, Some("'('"), Some("')'"), Some("'+'"), Some("'-'"), Some("'*'"), 
		Some("'/'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;10]  = [
		None, None, None, Some("OP_ADD"), Some("OP_SUB"), Some("OP_MUL"), Some("OP_DIV"), 
		Some("NUM"), Some("ID"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct MathLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,MathLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for MathLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for MathLexer<'input,Input>{
	type Target = BaseLexer<'input,MathLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for MathLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> MathLexer<'input,Input>{
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
        "MathLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","3");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				MathLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> MathLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		MathLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct MathLexerActions {
}

impl MathLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,MathLexerActions,Input,LocalTokenFactory<'input>>> for MathLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> MathLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,MathLexerActions,Input,LocalTokenFactory<'input>>> for MathLexerActions{
}
impl<'input> TokenAware<'input> for MathLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for MathLexer<'input,Input>{
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x0b\x42\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x03\x04\x03\x05\
		\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\x03\x08\x06\x08\x23\x0a\x08\x0d\
		\x08\x0e\x08\x24\x03\x08\x03\x08\x06\x08\x29\x0a\x08\x0d\x08\x0e\x08\x2a\
		\x05\x08\x2d\x0a\x08\x03\x08\x03\x08\x05\x08\x31\x0a\x08\x03\x08\x06\x08\
		\x34\x0a\x08\x0d\x08\x0e\x08\x35\x05\x08\x38\x0a\x08\x03\x09\x06\x09\x3b\
		\x0a\x09\x0d\x09\x0e\x09\x3c\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x02\x02\x0b\
		\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\x08\x0f\x09\x11\x0a\x13\x0b\
		\x03\x02\x07\x03\x02\x32\x3b\x04\x02\x47\x47\x67\x67\x04\x02\x2d\x2d\x2f\
		\x2f\x04\x02\x43\x5c\x63\x7c\x05\x02\x0b\x0c\x0f\x0f\x22\x22\x02\x48\x02\
		\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\x02\
		\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\x02\x0d\x03\x02\x02\x02\x02\
		\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\x02\x02\x13\x03\x02\x02\x02\x03\
		\x15\x03\x02\x02\x02\x05\x17\x03\x02\x02\x02\x07\x19\x03\x02\x02\x02\x09\
		\x1b\x03\x02\x02\x02\x0b\x1d\x03\x02\x02\x02\x0d\x1f\x03\x02\x02\x02\x0f\
		\x22\x03\x02\x02\x02\x11\x3a\x03\x02\x02\x02\x13\x3e\x03\x02\x02\x02\x15\
		\x16\x07\x2a\x02\x02\x16\x04\x03\x02\x02\x02\x17\x18\x07\x2b\x02\x02\x18\
		\x06\x03\x02\x02\x02\x19\x1a\x07\x2d\x02\x02\x1a\x08\x03\x02\x02\x02\x1b\
		\x1c\x07\x2f\x02\x02\x1c\x0a\x03\x02\x02\x02\x1d\x1e\x07\x2c\x02\x02\x1e\
		\x0c\x03\x02\x02\x02\x1f\x20\x07\x31\x02\x02\x20\x0e\x03\x02\x02\x02\x21\
		\x23\x09\x02\x02\x02\x22\x21\x03\x02\x02\x02\x23\x24\x03\x02\x02\x02\x24\
		\x22\x03\x02\x02\x02\x24\x25\x03\x02\x02\x02\x25\x2c\x03\x02\x02\x02\x26\
		\x28\x07\x30\x02\x02\x27\x29\x09\x02\x02\x02\x28\x27\x03\x02\x02\x02\x29\
		\x2a\x03\x02\x02\x02\x2a\x28\x03\x02\x02\x02\x2a\x2b\x03\x02\x02\x02\x2b\
		\x2d\x03\x02\x02\x02\x2c\x26\x03\x02\x02\x02\x2c\x2d\x03\x02\x02\x02\x2d\
		\x37\x03\x02\x02\x02\x2e\x30\x09\x03\x02\x02\x2f\x31\x09\x04\x02\x02\x30\
		\x2f\x03\x02\x02\x02\x30\x31\x03\x02\x02\x02\x31\x33\x03\x02\x02\x02\x32\
		\x34\x09\x02\x02\x02\x33\x32\x03\x02\x02\x02\x34\x35\x03\x02\x02\x02\x35\
		\x33\x03\x02\x02\x02\x35\x36\x03\x02\x02\x02\x36\x38\x03\x02\x02\x02\x37\
		\x2e\x03\x02\x02\x02\x37\x38\x03\x02\x02\x02\x38\x10\x03\x02\x02\x02\x39\
		\x3b\x09\x05\x02\x02\x3a\x39\x03\x02\x02\x02\x3b\x3c\x03\x02\x02\x02\x3c\
		\x3a\x03\x02\x02\x02\x3c\x3d\x03\x02\x02\x02\x3d\x12\x03\x02\x02\x02\x3e\
		\x3f\x09\x06\x02\x02\x3f\x40\x03\x02\x02\x02\x40\x41\x08\x0a\x02\x02\x41\
		\x14\x03\x02\x02\x02\x0a\x02\x24\x2a\x2c\x30\x35\x37\x3c\x03\x02\x03\x02";
