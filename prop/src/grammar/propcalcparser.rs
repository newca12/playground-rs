// Generated from propcalc.g4 by ANTLR 4.8
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
use super::propcalclistener::*;
use super::propcalcvisitor::*;

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
		pub const AND:isize=2; 
		pub const OR:isize=3; 
		pub const NOT:isize=4; 
		pub const EQ:isize=5; 
		pub const IMPLIES:isize=6; 
		pub const THEREFORE:isize=7; 
		pub const EQUIV:isize=8; 
		pub const LPAREN:isize=9; 
		pub const RPAREN:isize=10; 
		pub const LETTER:isize=11; 
		pub const WS:isize=12;
	pub const RULE_proposition:usize = 0; 
	pub const RULE_expression:usize = 1; 
	pub const RULE_relExpression:usize = 2; 
	pub const RULE_atoms:usize = 3; 
	pub const RULE_atom:usize = 4; 
	pub const RULE_equiv:usize = 5; 
	pub const RULE_implies:usize = 6; 
	pub const RULE_variable:usize = 7;
	pub const ruleNames: [&'static str; 8] =  [
		"proposition", "expression", "relExpression", "atoms", "atom", "equiv", 
		"implies", "variable"
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


type BaseParserType<'input, I> =
	BaseParser<'input,propcalcParserExt<'input>, I, propcalcParserContextType , dyn propcalcListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type propcalcTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, propcalcParserContextType , dyn propcalcListener<'input> + 'a>;

/// Parser for propcalc grammar
pub struct propcalcParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> propcalcParser<'input, I, H>
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
				propcalcParserExt{
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

impl<'input, I> propcalcParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> propcalcParser<'input, I, DefaultErrorStrategy<'input,propcalcParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for propcalcParser
pub trait propcalcParserContext<'input>:
	for<'x> Listenable<dyn propcalcListener<'input> + 'x > + 
	for<'x> Visitable<dyn propcalcVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=propcalcParserContextType>
{}

antlr_rust::coerce_from!{ 'input : propcalcParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn propcalcParserContext<'input> + 'input
where
    T: propcalcVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn propcalcVisitor<'input> + 'x))
    }
}

impl<'input> propcalcParserContext<'input> for TerminalNode<'input,propcalcParserContextType> {}
impl<'input> propcalcParserContext<'input> for ErrorNode<'input,propcalcParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn propcalcParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn propcalcListener<'input> + 'input }

pub struct propcalcParserContextType;
antlr_rust::tid!{propcalcParserContextType}

impl<'input> ParserNodeType<'input> for propcalcParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn propcalcParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct propcalcParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> propcalcParserExt<'input>{
}
antlr_rust::tid! { propcalcParserExt<'a> }

impl<'input> TokenAware<'input> for propcalcParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for propcalcParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for propcalcParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "propcalc.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- proposition ----------------
pub type PropositionContextAll<'input> = PropositionContext<'input>;


pub type PropositionContext<'input> = BaseParserRuleContext<'input,PropositionContextExt<'input>>;

#[derive(Clone)]
pub struct PropositionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> propcalcParserContext<'input> for PropositionContext<'input>{}

impl<'input,'a> Listenable<dyn propcalcListener<'input> + 'a> for PropositionContext<'input>{
		fn enter(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_proposition(self);
		}
		fn exit(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.exit_proposition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn propcalcVisitor<'input> + 'a> for PropositionContext<'input>{
	fn accept(&self,visitor: &mut (dyn propcalcVisitor<'input> + 'a)) {
		visitor.visit_proposition(self);
	}
}

impl<'input> CustomRuleContext<'input> for PropositionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = propcalcParserContextType;
	fn get_rule_index(&self) -> usize { RULE_proposition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_proposition }
}
antlr_rust::tid!{PropositionContextExt<'a>}

impl<'input> PropositionContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PropositionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PropositionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PropositionContextAttrs<'input>: propcalcParserContext<'input> + BorrowMut<PropositionContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token THEREFORE
/// Returns `None` if there is no child corresponding to token THEREFORE
fn THEREFORE(&self) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(THEREFORE, 0)
}

}

impl<'input> PropositionContextAttrs<'input> for PropositionContext<'input>{}

impl<'input, I, H> propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn proposition(&mut self,)
	-> Result<Rc<PropositionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PropositionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_proposition);
        let mut _localctx: Rc<PropositionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(16);
			recog.expression()?;

			recog.base.set_state(17);
			recog.base.match_token(THEREFORE,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(18);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> propcalcParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn propcalcListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn propcalcVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn propcalcVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = propcalcParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: propcalcParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn relExpression_all(&self) ->  Vec<Rc<RelExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relExpression(&self, i: usize) -> Option<Rc<RelExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AND in current rule
fn AND_all(&self) -> Vec<Rc<TerminalNode<'input,propcalcParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
/// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(AND, i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,propcalcParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(OR, i)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule relExpression*/
			recog.base.set_state(20);
			recog.relExpression()?;

			recog.base.set_state(25);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==AND || _la==OR {
				{
				{
				recog.base.set_state(21);
				_la = recog.base.input.la(1);
				if { !(_la==AND || _la==OR) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule relExpression*/
				recog.base.set_state(22);
				recog.relExpression()?;

				}
				}
				recog.base.set_state(27);
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
//------------------- relExpression ----------------
pub type RelExpressionContextAll<'input> = RelExpressionContext<'input>;


pub type RelExpressionContext<'input> = BaseParserRuleContext<'input,RelExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct RelExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> propcalcParserContext<'input> for RelExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn propcalcListener<'input> + 'a> for RelExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relExpression(self);
		}
		fn exit(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.exit_relExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn propcalcVisitor<'input> + 'a> for RelExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn propcalcVisitor<'input> + 'a)) {
		visitor.visit_relExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = propcalcParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relExpression }
}
antlr_rust::tid!{RelExpressionContextExt<'a>}

impl<'input> RelExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelExpressionContextAttrs<'input>: propcalcParserContext<'input> + BorrowMut<RelExpressionContextExt<'input>>{

fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn equiv(&self) -> Option<Rc<EquivContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn implies(&self) -> Option<Rc<ImpliesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RelExpressionContextAttrs<'input> for RelExpressionContext<'input>{}

impl<'input, I, H> propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relExpression(&mut self,)
	-> Result<Rc<RelExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_relExpression);
        let mut _localctx: Rc<RelExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(31);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule atom*/
					recog.base.set_state(28);
					recog.atom()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule equiv*/
					recog.base.set_state(29);
					recog.equiv()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule implies*/
					recog.base.set_state(30);
					recog.implies()?;

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
//------------------- atoms ----------------
pub type AtomsContextAll<'input> = AtomsContext<'input>;


pub type AtomsContext<'input> = BaseParserRuleContext<'input,AtomsContextExt<'input>>;

#[derive(Clone)]
pub struct AtomsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> propcalcParserContext<'input> for AtomsContext<'input>{}

impl<'input,'a> Listenable<dyn propcalcListener<'input> + 'a> for AtomsContext<'input>{
		fn enter(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_atoms(self);
		}
		fn exit(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.exit_atoms(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn propcalcVisitor<'input> + 'a> for AtomsContext<'input>{
	fn accept(&self,visitor: &mut (dyn propcalcVisitor<'input> + 'a)) {
		visitor.visit_atoms(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = propcalcParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atoms }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atoms }
}
antlr_rust::tid!{AtomsContextExt<'a>}

impl<'input> AtomsContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AtomsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AtomsContextAttrs<'input>: propcalcParserContext<'input> + BorrowMut<AtomsContextExt<'input>>{

fn atom_all(&self) ->  Vec<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn atom(&self, i: usize) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AtomsContextAttrs<'input> for AtomsContext<'input>{}

impl<'input, I, H> propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atoms(&mut self,)
	-> Result<Rc<AtomsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_atoms);
        let mut _localctx: Rc<AtomsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atom*/
			recog.base.set_state(33);
			recog.atom()?;

			recog.base.set_state(38);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__0 {
				{
				{
				recog.base.set_state(34);
				recog.base.match_token(T__0,&mut recog.err_handler)?;

				/*InvokeRule atom*/
				recog.base.set_state(35);
				recog.atom()?;

				}
				}
				recog.base.set_state(40);
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
//------------------- atom ----------------
pub type AtomContextAll<'input> = AtomContext<'input>;


pub type AtomContext<'input> = BaseParserRuleContext<'input,AtomContextExt<'input>>;

#[derive(Clone)]
pub struct AtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> propcalcParserContext<'input> for AtomContext<'input>{}

impl<'input,'a> Listenable<dyn propcalcListener<'input> + 'a> for AtomContext<'input>{
		fn enter(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_atom(self);
		}
		fn exit(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.exit_atom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn propcalcVisitor<'input> + 'a> for AtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn propcalcVisitor<'input> + 'a)) {
		visitor.visit_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = propcalcParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}
antlr_rust::tid!{AtomContextExt<'a>}

impl<'input> AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AtomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AtomContextAttrs<'input>: propcalcParserContext<'input> + BorrowMut<AtomContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NOT
/// Returns `None` if there is no child corresponding to token NOT
fn NOT(&self) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(NOT, 0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input>{}

impl<'input, I, H> propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atom(&mut self,)
	-> Result<Rc<AtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_atom);
        let mut _localctx: Rc<AtomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(48);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EOF | T__0 | AND | OR | IMPLIES | THEREFORE | EQUIV | RPAREN | LETTER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule variable*/
					recog.base.set_state(41);
					recog.variable()?;

					}
				}

			 NOT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(42);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					/*InvokeRule atom*/
					recog.base.set_state(43);
					recog.atom()?;

					}
				}

			 LPAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(44);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(45);
					recog.expression()?;

					recog.base.set_state(46);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- equiv ----------------
pub type EquivContextAll<'input> = EquivContext<'input>;


pub type EquivContext<'input> = BaseParserRuleContext<'input,EquivContextExt<'input>>;

#[derive(Clone)]
pub struct EquivContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> propcalcParserContext<'input> for EquivContext<'input>{}

impl<'input,'a> Listenable<dyn propcalcListener<'input> + 'a> for EquivContext<'input>{
		fn enter(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_equiv(self);
		}
		fn exit(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.exit_equiv(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn propcalcVisitor<'input> + 'a> for EquivContext<'input>{
	fn accept(&self,visitor: &mut (dyn propcalcVisitor<'input> + 'a)) {
		visitor.visit_equiv(self);
	}
}

impl<'input> CustomRuleContext<'input> for EquivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = propcalcParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equiv }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equiv }
}
antlr_rust::tid!{EquivContextExt<'a>}

impl<'input> EquivContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EquivContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EquivContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EquivContextAttrs<'input>: propcalcParserContext<'input> + BorrowMut<EquivContextExt<'input>>{

fn atom_all(&self) ->  Vec<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn atom(&self, i: usize) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EQUIV
/// Returns `None` if there is no child corresponding to token EQUIV
fn EQUIV(&self) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(EQUIV, 0)
}

}

impl<'input> EquivContextAttrs<'input> for EquivContext<'input>{}

impl<'input, I, H> propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equiv(&mut self,)
	-> Result<Rc<EquivContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EquivContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_equiv);
        let mut _localctx: Rc<EquivContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atom*/
			recog.base.set_state(50);
			recog.atom()?;

			recog.base.set_state(51);
			recog.base.match_token(EQUIV,&mut recog.err_handler)?;

			/*InvokeRule atom*/
			recog.base.set_state(52);
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
//------------------- implies ----------------
pub type ImpliesContextAll<'input> = ImpliesContext<'input>;


pub type ImpliesContext<'input> = BaseParserRuleContext<'input,ImpliesContextExt<'input>>;

#[derive(Clone)]
pub struct ImpliesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> propcalcParserContext<'input> for ImpliesContext<'input>{}

impl<'input,'a> Listenable<dyn propcalcListener<'input> + 'a> for ImpliesContext<'input>{
		fn enter(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_implies(self);
		}
		fn exit(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.exit_implies(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn propcalcVisitor<'input> + 'a> for ImpliesContext<'input>{
	fn accept(&self,visitor: &mut (dyn propcalcVisitor<'input> + 'a)) {
		visitor.visit_implies(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImpliesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = propcalcParserContextType;
	fn get_rule_index(&self) -> usize { RULE_implies }
	//fn type_rule_index() -> usize where Self: Sized { RULE_implies }
}
antlr_rust::tid!{ImpliesContextExt<'a>}

impl<'input> ImpliesContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImpliesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImpliesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImpliesContextAttrs<'input>: propcalcParserContext<'input> + BorrowMut<ImpliesContextExt<'input>>{

fn atom_all(&self) ->  Vec<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn atom(&self, i: usize) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token IMPLIES
/// Returns `None` if there is no child corresponding to token IMPLIES
fn IMPLIES(&self) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(IMPLIES, 0)
}

}

impl<'input> ImpliesContextAttrs<'input> for ImpliesContext<'input>{}

impl<'input, I, H> propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn implies(&mut self,)
	-> Result<Rc<ImpliesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImpliesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_implies);
        let mut _localctx: Rc<ImpliesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule atom*/
			recog.base.set_state(54);
			recog.atom()?;

			recog.base.set_state(55);
			recog.base.match_token(IMPLIES,&mut recog.err_handler)?;

			/*InvokeRule atom*/
			recog.base.set_state(56);
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
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;


pub type VariableContext<'input> = BaseParserRuleContext<'input,VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> propcalcParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn propcalcListener<'input> + 'a> for VariableContext<'input>{
		fn enter(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variable(self);
		}
		fn exit(&self,listener: &mut (dyn propcalcListener<'input> + 'a)) {
			listener.exit_variable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn propcalcVisitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn propcalcVisitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = propcalcParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr_rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableContextAttrs<'input>: propcalcParserContext<'input> + BorrowMut<VariableContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token LETTER in current rule
fn LETTER_all(&self) -> Vec<Rc<TerminalNode<'input,propcalcParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LETTER, starting from 0.
/// Returns `None` if number of children corresponding to token LETTER is less or equal than `i`.
fn LETTER(&self, i: usize) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(LETTER, i)
}

}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

impl<'input, I, H> propcalcParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable(&mut self,)
	-> Result<Rc<VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(61);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==LETTER {
				{
				{
				recog.base.set_state(58);
				recog.base.match_token(LETTER,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(63);
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
	\x0e\x43\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x03\x02\
	\x03\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x07\x03\x1a\x0a\x03\x0c\
	\x03\x0e\x03\x1d\x0b\x03\x03\x04\x03\x04\x03\x04\x05\x04\x22\x0a\x04\x03\
	\x05\x03\x05\x03\x05\x07\x05\x27\x0a\x05\x0c\x05\x0e\x05\x2a\x0b\x05\x03\
	\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\x33\x0a\x06\
	\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x09\
	\x07\x09\x3e\x0a\x09\x0c\x09\x0e\x09\x41\x0b\x09\x03\x09\x02\x02\x0a\x02\
	\x04\x06\x08\x0a\x0c\x0e\x10\x02\x03\x03\x02\x04\x05\x02\x41\x02\x12\x03\
	\x02\x02\x02\x04\x16\x03\x02\x02\x02\x06\x21\x03\x02\x02\x02\x08\x23\x03\
	\x02\x02\x02\x0a\x32\x03\x02\x02\x02\x0c\x34\x03\x02\x02\x02\x0e\x38\x03\
	\x02\x02\x02\x10\x3f\x03\x02\x02\x02\x12\x13\x05\x04\x03\x02\x13\x14\x07\
	\x09\x02\x02\x14\x15\x05\x04\x03\x02\x15\x03\x03\x02\x02\x02\x16\x1b\x05\
	\x06\x04\x02\x17\x18\x09\x02\x02\x02\x18\x1a\x05\x06\x04\x02\x19\x17\x03\
	\x02\x02\x02\x1a\x1d\x03\x02\x02\x02\x1b\x19\x03\x02\x02\x02\x1b\x1c\x03\
	\x02\x02\x02\x1c\x05\x03\x02\x02\x02\x1d\x1b\x03\x02\x02\x02\x1e\x22\x05\
	\x0a\x06\x02\x1f\x22\x05\x0c\x07\x02\x20\x22\x05\x0e\x08\x02\x21\x1e\x03\
	\x02\x02\x02\x21\x1f\x03\x02\x02\x02\x21\x20\x03\x02\x02\x02\x22\x07\x03\
	\x02\x02\x02\x23\x28\x05\x0a\x06\x02\x24\x25\x07\x03\x02\x02\x25\x27\x05\
	\x0a\x06\x02\x26\x24\x03\x02\x02\x02\x27\x2a\x03\x02\x02\x02\x28\x26\x03\
	\x02\x02\x02\x28\x29\x03\x02\x02\x02\x29\x09\x03\x02\x02\x02\x2a\x28\x03\
	\x02\x02\x02\x2b\x33\x05\x10\x09\x02\x2c\x2d\x07\x06\x02\x02\x2d\x33\x05\
	\x0a\x06\x02\x2e\x2f\x07\x0b\x02\x02\x2f\x30\x05\x04\x03\x02\x30\x31\x07\
	\x0c\x02\x02\x31\x33\x03\x02\x02\x02\x32\x2b\x03\x02\x02\x02\x32\x2c\x03\
	\x02\x02\x02\x32\x2e\x03\x02\x02\x02\x33\x0b\x03\x02\x02\x02\x34\x35\x05\
	\x0a\x06\x02\x35\x36\x07\x0a\x02\x02\x36\x37\x05\x0a\x06\x02\x37\x0d\x03\
	\x02\x02\x02\x38\x39\x05\x0a\x06\x02\x39\x3a\x07\x08\x02\x02\x3a\x3b\x05\
	\x0a\x06\x02\x3b\x0f\x03\x02\x02\x02\x3c\x3e\x07\x0d\x02\x02\x3d\x3c\x03\
	\x02\x02\x02\x3e\x41\x03\x02\x02\x02\x3f\x3d\x03\x02\x02\x02\x3f\x40\x03\
	\x02\x02\x02\x40\x11\x03\x02\x02\x02\x41\x3f\x03\x02\x02\x02\x07\x1b\x21\
	\x28\x32\x3f";

