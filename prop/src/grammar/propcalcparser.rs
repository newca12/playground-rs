// Generated from propcalc.g4 by ANTLR 4.13.2
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
use super::propcalclistener::*;
use super::propcalcvisitor::*;

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

		pub const propcalc_T__0:i32=1; 
		pub const propcalc_AND:i32=2; 
		pub const propcalc_OR:i32=3; 
		pub const propcalc_NOT:i32=4; 
		pub const propcalc_EQ:i32=5; 
		pub const propcalc_IMPLIES:i32=6; 
		pub const propcalc_THEREFORE:i32=7; 
		pub const propcalc_EQUIV:i32=8; 
		pub const propcalc_LPAREN:i32=9; 
		pub const propcalc_RPAREN:i32=10; 
		pub const propcalc_LETTER:i32=11; 
		pub const propcalc_WS:i32=12;
	pub const propcalc_EOF:i32=EOF;
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

antlr4rust::coerce_from!{ 'input : propcalcParserContext<'input> }

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

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn propcalcParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn propcalcListener<'input> + 'input }

pub struct propcalcParserContextType;
antlr4rust::tid!{propcalcParserContextType}

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
antlr4rust::tid! { propcalcParserExt<'a> }

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
antlr4rust::tid!{PropositionContextExt<'a>}

impl<'input> PropositionContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PropositionContextAll<'input>> {
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
	self.get_token(propcalc_THEREFORE, 0)
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
			recog.base.match_token(propcalc_THEREFORE,&mut recog.err_handler)?;

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
antlr4rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExpressionContextAll<'input>> {
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
	self.get_token(propcalc_AND, i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,propcalcParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(propcalc_OR, i)
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
		let mut _la: i32 = -1;
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
			while _la==propcalc_AND || _la==propcalc_OR {
				{
				{
				recog.base.set_state(21);
				_la = recog.base.input.la(1);
				if { !(_la==propcalc_AND || _la==propcalc_OR) } {
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
antlr4rust::tid!{RelExpressionContextExt<'a>}

impl<'input> RelExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RelExpressionContextAll<'input>> {
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
antlr4rust::tid!{AtomsContextExt<'a>}

impl<'input> AtomsContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AtomsContextAll<'input>> {
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
		let mut _la: i32 = -1;
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
			while _la==propcalc_T__0 {
				{
				{
				recog.base.set_state(34);
				recog.base.match_token(propcalc_T__0,&mut recog.err_handler)?;

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
antlr4rust::tid!{AtomContextExt<'a>}

impl<'input> AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AtomContextAll<'input>> {
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
	self.get_token(propcalc_NOT, 0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(propcalc_LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,propcalcParserContextType>>> where Self:Sized{
	self.get_token(propcalc_RPAREN, 0)
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
			propcalc_EOF |propcalc_T__0 |propcalc_AND |propcalc_OR |propcalc_IMPLIES |
			propcalc_THEREFORE |propcalc_EQUIV |propcalc_RPAREN |propcalc_LETTER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule variable*/
					recog.base.set_state(41);
					recog.variable()?;

					}
				}

			propcalc_NOT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(42);
					recog.base.match_token(propcalc_NOT,&mut recog.err_handler)?;

					/*InvokeRule atom*/
					recog.base.set_state(43);
					recog.atom()?;

					}
				}

			propcalc_LPAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(44);
					recog.base.match_token(propcalc_LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(45);
					recog.expression()?;

					recog.base.set_state(46);
					recog.base.match_token(propcalc_RPAREN,&mut recog.err_handler)?;

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
antlr4rust::tid!{EquivContextExt<'a>}

impl<'input> EquivContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EquivContextAll<'input>> {
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
	self.get_token(propcalc_EQUIV, 0)
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
			recog.base.match_token(propcalc_EQUIV,&mut recog.err_handler)?;

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
antlr4rust::tid!{ImpliesContextExt<'a>}

impl<'input> ImpliesContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ImpliesContextAll<'input>> {
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
	self.get_token(propcalc_IMPLIES, 0)
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
			recog.base.match_token(propcalc_IMPLIES,&mut recog.err_handler)?;

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
antlr4rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn propcalcParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VariableContextAll<'input>> {
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
	self.get_token(propcalc_LETTER, i)
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
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(61);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==propcalc_LETTER {
				{
				{
				recog.base.set_state(58);
				recog.base.match_token(propcalc_LETTER,&mut recog.err_handler)?;

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
const _serializedATN: [i32; 544] = [
	4, 1, 12, 65, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 7, 
	4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 
	1, 1, 1, 5, 1, 24, 8, 1, 10, 1, 12, 1, 27, 9, 1, 1, 2, 1, 2, 1, 2, 3, 2, 
	32, 8, 2, 1, 3, 1, 3, 1, 3, 5, 3, 37, 8, 3, 10, 3, 12, 3, 40, 9, 3, 1, 
	4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 3, 4, 49, 8, 4, 1, 5, 1, 5, 1, 5, 
	1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 1, 7, 5, 7, 60, 8, 7, 10, 7, 12, 7, 63, 9, 
	7, 1, 7, 0, 0, 8, 0, 2, 4, 6, 8, 10, 12, 14, 0, 1, 1, 0, 2, 3, 63, 0, 16, 
	1, 0, 0, 0, 2, 20, 1, 0, 0, 0, 4, 31, 1, 0, 0, 0, 6, 33, 1, 0, 0, 0, 8, 
	48, 1, 0, 0, 0, 10, 50, 1, 0, 0, 0, 12, 54, 1, 0, 0, 0, 14, 61, 1, 0, 0, 
	0, 16, 17, 3, 2, 1, 0, 17, 18, 5, 7, 0, 0, 18, 19, 3, 2, 1, 0, 19, 1, 1, 
	0, 0, 0, 20, 25, 3, 4, 2, 0, 21, 22, 7, 0, 0, 0, 22, 24, 3, 4, 2, 0, 23, 
	21, 1, 0, 0, 0, 24, 27, 1, 0, 0, 0, 25, 23, 1, 0, 0, 0, 25, 26, 1, 0, 0, 
	0, 26, 3, 1, 0, 0, 0, 27, 25, 1, 0, 0, 0, 28, 32, 3, 8, 4, 0, 29, 32, 3, 
	10, 5, 0, 30, 32, 3, 12, 6, 0, 31, 28, 1, 0, 0, 0, 31, 29, 1, 0, 0, 0, 
	31, 30, 1, 0, 0, 0, 32, 5, 1, 0, 0, 0, 33, 38, 3, 8, 4, 0, 34, 35, 5, 1, 
	0, 0, 35, 37, 3, 8, 4, 0, 36, 34, 1, 0, 0, 0, 37, 40, 1, 0, 0, 0, 38, 36, 
	1, 0, 0, 0, 38, 39, 1, 0, 0, 0, 39, 7, 1, 0, 0, 0, 40, 38, 1, 0, 0, 0, 
	41, 49, 3, 14, 7, 0, 42, 43, 5, 4, 0, 0, 43, 49, 3, 8, 4, 0, 44, 45, 5, 
	9, 0, 0, 45, 46, 3, 2, 1, 0, 46, 47, 5, 10, 0, 0, 47, 49, 1, 0, 0, 0, 48, 
	41, 1, 0, 0, 0, 48, 42, 1, 0, 0, 0, 48, 44, 1, 0, 0, 0, 49, 9, 1, 0, 0, 
	0, 50, 51, 3, 8, 4, 0, 51, 52, 5, 8, 0, 0, 52, 53, 3, 8, 4, 0, 53, 11, 
	1, 0, 0, 0, 54, 55, 3, 8, 4, 0, 55, 56, 5, 6, 0, 0, 56, 57, 3, 8, 4, 0, 
	57, 13, 1, 0, 0, 0, 58, 60, 5, 11, 0, 0, 59, 58, 1, 0, 0, 0, 60, 63, 1, 
	0, 0, 0, 61, 59, 1, 0, 0, 0, 61, 62, 1, 0, 0, 0, 62, 15, 1, 0, 0, 0, 63, 
	61, 1, 0, 0, 0, 5, 25, 31, 38, 48, 61
];
