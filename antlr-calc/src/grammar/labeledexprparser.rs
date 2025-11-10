// Generated from LabeledExpr.g4 by ANTLR 4.13.2
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
use super::labeledexprlistener::*;
use super::labeledexprvisitor::*;

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

		pub const LabeledExpr_T__0:i32=1; 
		pub const LabeledExpr_T__1:i32=2; 
		pub const LabeledExpr_T__2:i32=3; 
		pub const LabeledExpr_MUL:i32=4; 
		pub const LabeledExpr_DIV:i32=5; 
		pub const LabeledExpr_ADD:i32=6; 
		pub const LabeledExpr_SUB:i32=7; 
		pub const LabeledExpr_ID:i32=8; 
		pub const LabeledExpr_INT:i32=9; 
		pub const LabeledExpr_NEWLINE:i32=10; 
		pub const LabeledExpr_WS:i32=11;
	pub const LabeledExpr_EOF:i32=EOF;
	pub const RULE_prog:usize = 0; 
	pub const RULE_stat:usize = 1; 
	pub const RULE_expr:usize = 2;
	pub const ruleNames: [&'static str; 3] =  [
		"prog", "stat", "expr"
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


type BaseParserType<'input, I> =
	BaseParser<'input,LabeledExprParserExt<'input>, I, LabeledExprParserContextType , dyn LabeledExprListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type LabeledExprTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, LabeledExprParserContextType , dyn LabeledExprListener<'input> + 'a>;

/// Parser for LabeledExpr grammar
pub struct LabeledExprParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> LabeledExprParser<'input, I>
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
				LabeledExprParserExt{
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

impl<'input, I> LabeledExprParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> LabeledExprParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for LabeledExprParser
pub trait LabeledExprParserContext<'input>:
	for<'x> Listenable<dyn LabeledExprListener<'input> + 'x > + 
	for<'x> Visitable<dyn LabeledExprVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=LabeledExprParserContextType>
{}

antlr4rust::coerce_from!{ 'input : LabeledExprParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn LabeledExprParserContext<'input> + 'input
where
    T: LabeledExprVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn LabeledExprVisitor<'input> + 'x))
    }
}

impl<'input> LabeledExprParserContext<'input> for TerminalNode<'input,LabeledExprParserContextType> {}
impl<'input> LabeledExprParserContext<'input> for ErrorNode<'input,LabeledExprParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn LabeledExprParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn LabeledExprListener<'input> + 'input }

pub struct LabeledExprParserContextType;
antlr4rust::tid!{LabeledExprParserContextType}

impl<'input> ParserNodeType<'input> for LabeledExprParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn LabeledExprParserContext<'input> + 'input;
}

impl<'input, I> Deref for LabeledExprParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for LabeledExprParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct LabeledExprParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> LabeledExprParserExt<'input>{
}
antlr4rust::tid! { LabeledExprParserExt<'a> }

impl<'input> TokenAware<'input> for LabeledExprParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for LabeledExprParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for LabeledExprParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "LabeledExpr.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn LabeledExprParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					2 => LabeledExprParser::<'input,I>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> LabeledExprParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 5)
				}
				1=>{
					recog.precpred(None, 4)
				}
			_ => true
		}
	}
}
//------------------- prog ----------------
pub type ProgContextAll<'input> = ProgContext<'input>;


pub type ProgContext<'input> = BaseParserRuleContext<'input,ProgContextExt<'input>>;

#[derive(Clone)]
pub struct ProgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LabeledExprParserContext<'input> for ProgContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for ProgContext<'input>{
		fn enter(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_prog(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_prog(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for ProgContext<'input>{
	fn accept(&self,visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) {
		visitor.visit_prog(self);
	}
}

impl<'input> CustomRuleContext<'input> for ProgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_prog }
	//fn type_rule_index() -> usize where Self: Sized { RULE_prog }
}
antlr4rust::tid!{ProgContextExt<'a>}

impl<'input> ProgContextExt<'input>{
	fn new(parent: Option<Rc<dyn LabeledExprParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ProgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ProgContextAttrs<'input>: LabeledExprParserContext<'input> + BorrowMut<ProgContextExt<'input>>{

fn stat_all(&self) ->  Vec<Rc<StatContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stat(&self, i: usize) -> Option<Rc<StatContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgContextAttrs<'input> for ProgContext<'input>{}

impl<'input, I> LabeledExprParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn prog(&mut self,)
	-> Result<Rc<ProgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_prog);
        let mut _localctx: Rc<ProgContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(7); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule stat*/
				recog.base.set_state(6);
				recog.stat()?;

				}
				}
				recog.base.set_state(9); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 1796) != 0)) {break}
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
//------------------- stat ----------------
#[derive(Debug)]
pub enum StatContextAll<'input>{
	BlankContext(BlankContext<'input>),
	PrintExprContext(PrintExprContext<'input>),
	AssignContext(AssignContext<'input>),
Error(StatContext<'input>)
}
antlr4rust::tid!{StatContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for StatContextAll<'input>{}

impl<'input> LabeledExprParserContext<'input> for StatContextAll<'input>{}

impl<'input> Deref for StatContextAll<'input>{
	type Target = dyn StatContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use StatContextAll::*;
		match self{
			BlankContext(inner) => inner,
			PrintExprContext(inner) => inner,
			AssignContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for StatContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for StatContextAll<'input>{
    fn enter(&self, listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type StatContext<'input> = BaseParserRuleContext<'input,StatContextExt<'input>>;

#[derive(Clone)]
pub struct StatContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LabeledExprParserContext<'input> for StatContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for StatContext<'input>{
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for StatContext<'input>{
}

impl<'input> CustomRuleContext<'input> for StatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stat }
}
antlr4rust::tid!{StatContextExt<'a>}

impl<'input> StatContextExt<'input>{
	fn new(parent: Option<Rc<dyn LabeledExprParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StatContextAll<'input>> {
		Rc::new(
		StatContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait StatContextAttrs<'input>: LabeledExprParserContext<'input> + BorrowMut<StatContextExt<'input>>{


}

impl<'input> StatContextAttrs<'input> for StatContext<'input>{}

pub type BlankContext<'input> = BaseParserRuleContext<'input,BlankContextExt<'input>>;

pub trait BlankContextAttrs<'input>: LabeledExprParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NEWLINE
	/// Returns `None` if there is no child corresponding to token NEWLINE
	fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_NEWLINE, 0)
	}
}

impl<'input> BlankContextAttrs<'input> for BlankContext<'input>{}

pub struct BlankContextExt<'input>{
	base:StatContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{BlankContextExt<'a>}

impl<'input> LabeledExprParserContext<'input> for BlankContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for BlankContext<'input>{
	fn enter(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_blank(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_blank(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for BlankContext<'input>{
	fn accept(&self,visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) {
		visitor.visit_blank(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlankContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stat }
}

impl<'input> Borrow<StatContextExt<'input>> for BlankContext<'input>{
	fn borrow(&self) -> &StatContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatContextExt<'input>> for BlankContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatContextExt<'input> { &mut self.base }
}

impl<'input> StatContextAttrs<'input> for BlankContext<'input> {}

impl<'input> BlankContextExt<'input>{
	fn new(ctx: &dyn StatContextAttrs<'input>) -> Rc<StatContextAll<'input>>  {
		Rc::new(
			StatContextAll::BlankContext(
				BaseParserRuleContext::copy_from(ctx,BlankContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PrintExprContext<'input> = BaseParserRuleContext<'input,PrintExprContextExt<'input>>;

pub trait PrintExprContextAttrs<'input>: LabeledExprParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token NEWLINE
	/// Returns `None` if there is no child corresponding to token NEWLINE
	fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_NEWLINE, 0)
	}
}

impl<'input> PrintExprContextAttrs<'input> for PrintExprContext<'input>{}

pub struct PrintExprContextExt<'input>{
	base:StatContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PrintExprContextExt<'a>}

impl<'input> LabeledExprParserContext<'input> for PrintExprContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for PrintExprContext<'input>{
	fn enter(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_printExpr(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_printExpr(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for PrintExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) {
		visitor.visit_printExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrintExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stat }
}

impl<'input> Borrow<StatContextExt<'input>> for PrintExprContext<'input>{
	fn borrow(&self) -> &StatContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatContextExt<'input>> for PrintExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatContextExt<'input> { &mut self.base }
}

impl<'input> StatContextAttrs<'input> for PrintExprContext<'input> {}

impl<'input> PrintExprContextExt<'input>{
	fn new(ctx: &dyn StatContextAttrs<'input>) -> Rc<StatContextAll<'input>>  {
		Rc::new(
			StatContextAll::PrintExprContext(
				BaseParserRuleContext::copy_from(ctx,PrintExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AssignContext<'input> = BaseParserRuleContext<'input,AssignContextExt<'input>>;

pub trait AssignContextAttrs<'input>: LabeledExprParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_ID, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token NEWLINE
	/// Returns `None` if there is no child corresponding to token NEWLINE
	fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_NEWLINE, 0)
	}
}

impl<'input> AssignContextAttrs<'input> for AssignContext<'input>{}

pub struct AssignContextExt<'input>{
	base:StatContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AssignContextExt<'a>}

impl<'input> LabeledExprParserContext<'input> for AssignContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for AssignContext<'input>{
	fn enter(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_assign(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_assign(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for AssignContext<'input>{
	fn accept(&self,visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) {
		visitor.visit_assign(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stat }
}

impl<'input> Borrow<StatContextExt<'input>> for AssignContext<'input>{
	fn borrow(&self) -> &StatContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatContextExt<'input>> for AssignContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatContextExt<'input> { &mut self.base }
}

impl<'input> StatContextAttrs<'input> for AssignContext<'input> {}

impl<'input> AssignContextExt<'input>{
	fn new(ctx: &dyn StatContextAttrs<'input>) -> Rc<StatContextAll<'input>>  {
		Rc::new(
			StatContextAll::AssignContext(
				BaseParserRuleContext::copy_from(ctx,AssignContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> LabeledExprParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn stat(&mut self,)
	-> Result<Rc<StatContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_stat);
        let mut _localctx: Rc<StatContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(20);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					let tmp = PrintExprContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule expr*/
					recog.base.set_state(11);
					recog.expr_rec(0)?;

					recog.base.set_state(12);
					recog.base.match_token(LabeledExpr_NEWLINE,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = AssignContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(14);
					recog.base.match_token(LabeledExpr_ID,&mut recog.err_handler)?;

					recog.base.set_state(15);
					recog.base.match_token(LabeledExpr_T__0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(16);
					recog.expr_rec(0)?;

					recog.base.set_state(17);
					recog.base.match_token(LabeledExpr_NEWLINE,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = BlankContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(19);
					recog.base.match_token(LabeledExpr_NEWLINE,&mut recog.err_handler)?;

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
//------------------- expr ----------------
#[derive(Debug)]
pub enum ExprContextAll<'input>{
	ParensContext(ParensContext<'input>),
	MulDivContext(MulDivContext<'input>),
	AddSubContext(AddSubContext<'input>),
	IdContext(IdContext<'input>),
	IntContext(IntContext<'input>),
Error(ExprContext<'input>)
}
antlr4rust::tid!{ExprContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ExprContextAll<'input>{}

impl<'input> LabeledExprParserContext<'input> for ExprContextAll<'input>{}

impl<'input> Deref for ExprContextAll<'input>{
	type Target = dyn ExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			ParensContext(inner) => inner,
			MulDivContext(inner) => inner,
			AddSubContext(inner) => inner,
			IdContext(inner) => inner,
			IntContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for ExprContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for ExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LabeledExprParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for ExprContext<'input>{
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for ExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr4rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn LabeledExprParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExprContextAll<'input>> {
		Rc::new(
		ExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExprContextAttrs<'input>: LabeledExprParserContext<'input> + BorrowMut<ExprContextExt<'input>>{


}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

pub type ParensContext<'input> = BaseParserRuleContext<'input,ParensContextExt<'input>>;

pub trait ParensContextAttrs<'input>: LabeledExprParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ParensContextAttrs<'input> for ParensContext<'input>{}

pub struct ParensContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ParensContextExt<'a>}

impl<'input> LabeledExprParserContext<'input> for ParensContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for ParensContext<'input>{
	fn enter(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_parens(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_parens(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for ParensContext<'input>{
	fn accept(&self,visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) {
		visitor.visit_parens(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParensContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ParensContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ParensContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ParensContext<'input> {}

impl<'input> ParensContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ParensContext(
				BaseParserRuleContext::copy_from(ctx,ParensContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MulDivContext<'input> = BaseParserRuleContext<'input,MulDivContextExt<'input>>;

pub trait MulDivContextAttrs<'input>: LabeledExprParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token MUL
	/// Returns `None` if there is no child corresponding to token MUL
	fn MUL(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_MUL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token DIV
	/// Returns `None` if there is no child corresponding to token DIV
	fn DIV(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_DIV, 0)
	}
}

impl<'input> MulDivContextAttrs<'input> for MulDivContext<'input>{}

pub struct MulDivContextExt<'input>{
	base:ExprContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MulDivContextExt<'a>}

impl<'input> LabeledExprParserContext<'input> for MulDivContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for MulDivContext<'input>{
	fn enter(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_MulDiv(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_MulDiv(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for MulDivContext<'input>{
	fn accept(&self,visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) {
		visitor.visit_MulDiv(self);
	}
}

impl<'input> CustomRuleContext<'input> for MulDivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for MulDivContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for MulDivContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for MulDivContext<'input> {}

impl<'input> MulDivContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::MulDivContext(
				BaseParserRuleContext::copy_from(ctx,MulDivContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AddSubContext<'input> = BaseParserRuleContext<'input,AddSubContextExt<'input>>;

pub trait AddSubContextAttrs<'input>: LabeledExprParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token ADD
	/// Returns `None` if there is no child corresponding to token ADD
	fn ADD(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_ADD, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SUB
	/// Returns `None` if there is no child corresponding to token SUB
	fn SUB(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_SUB, 0)
	}
}

impl<'input> AddSubContextAttrs<'input> for AddSubContext<'input>{}

pub struct AddSubContextExt<'input>{
	base:ExprContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AddSubContextExt<'a>}

impl<'input> LabeledExprParserContext<'input> for AddSubContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for AddSubContext<'input>{
	fn enter(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_AddSub(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_AddSub(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for AddSubContext<'input>{
	fn accept(&self,visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) {
		visitor.visit_AddSub(self);
	}
}

impl<'input> CustomRuleContext<'input> for AddSubContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for AddSubContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for AddSubContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for AddSubContext<'input> {}

impl<'input> AddSubContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::AddSubContext(
				BaseParserRuleContext::copy_from(ctx,AddSubContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdContext<'input> = BaseParserRuleContext<'input,IdContextExt<'input>>;

pub trait IdContextAttrs<'input>: LabeledExprParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_ID, 0)
	}
}

impl<'input> IdContextAttrs<'input> for IdContext<'input>{}

pub struct IdContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IdContextExt<'a>}

impl<'input> LabeledExprParserContext<'input> for IdContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for IdContext<'input>{
	fn enter(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_id(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_id(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for IdContext<'input>{
	fn accept(&self,visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) {
		visitor.visit_id(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for IdContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for IdContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for IdContext<'input> {}

impl<'input> IdContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::IdContext(
				BaseParserRuleContext::copy_from(ctx,IdContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntContext<'input> = BaseParserRuleContext<'input,IntContextExt<'input>>;

pub trait IntContextAttrs<'input>: LabeledExprParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INT
	/// Returns `None` if there is no child corresponding to token INT
	fn INT(&self) -> Option<Rc<TerminalNode<'input,LabeledExprParserContextType>>> where Self:Sized{
		self.get_token(LabeledExpr_INT, 0)
	}
}

impl<'input> IntContextAttrs<'input> for IntContext<'input>{}

pub struct IntContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IntContextExt<'a>}

impl<'input> LabeledExprParserContext<'input> for IntContext<'input>{}

impl<'input,'a> Listenable<dyn LabeledExprListener<'input> + 'a> for IntContext<'input>{
	fn enter(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_int(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn LabeledExprListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_int(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn LabeledExprVisitor<'input> + 'a> for IntContext<'input>{
	fn accept(&self,visitor: &mut (dyn LabeledExprVisitor<'input> + 'a)) {
		visitor.visit_int(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LabeledExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for IntContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for IntContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for IntContext<'input> {}

impl<'input> IntContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::IntContext(
				BaseParserRuleContext::copy_from(ctx,IntContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> LabeledExprParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: i32)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 4, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 4;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(29);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			LabeledExpr_INT 
				=> {
					{
					let mut tmp = IntContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();

					recog.base.set_state(23);
					recog.base.match_token(LabeledExpr_INT,&mut recog.err_handler)?;

					}
				}

			LabeledExpr_ID 
				=> {
					{
					let mut tmp = IdContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(24);
					recog.base.match_token(LabeledExpr_ID,&mut recog.err_handler)?;

					}
				}

			LabeledExpr_T__1 
				=> {
					{
					let mut tmp = ParensContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(25);
					recog.base.match_token(LabeledExpr_T__1,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(26);
					recog.expr_rec(0)?;

					recog.base.set_state(27);
					recog.base.match_token(LabeledExpr_T__2,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(39);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(37);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MulDivContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr)?;
							_localctx = tmp;
							recog.base.set_state(31);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(32);
							if let ExprContextAll::MulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==LabeledExpr_MUL || _la==LabeledExpr_DIV) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::MulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(33);
							recog.expr_rec(6)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AddSubContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr)?;
							_localctx = tmp;
							recog.base.set_state(34);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(35);
							if let ExprContextAll::AddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==LabeledExpr_ADD || _la==LabeledExpr_SUB) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::AddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(36);
							recog.expr_rec(5)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(41);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
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
		4, 1, 11, 43, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 1, 0, 4, 0, 8, 8, 0, 
		11, 0, 12, 0, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
		3, 1, 21, 8, 1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 30, 8, 
		2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 5, 2, 38, 8, 2, 10, 2, 12, 2, 41, 
		9, 2, 1, 2, 0, 1, 4, 3, 0, 2, 4, 0, 2, 1, 0, 4, 5, 1, 0, 6, 7, 46, 0, 
		7, 1, 0, 0, 0, 2, 20, 1, 0, 0, 0, 4, 29, 1, 0, 0, 0, 6, 8, 3, 2, 1, 0, 
		7, 6, 1, 0, 0, 0, 8, 9, 1, 0, 0, 0, 9, 7, 1, 0, 0, 0, 9, 10, 1, 0, 0, 
		0, 10, 1, 1, 0, 0, 0, 11, 12, 3, 4, 2, 0, 12, 13, 5, 10, 0, 0, 13, 21, 
		1, 0, 0, 0, 14, 15, 5, 8, 0, 0, 15, 16, 5, 1, 0, 0, 16, 17, 3, 4, 2, 0, 
		17, 18, 5, 10, 0, 0, 18, 21, 1, 0, 0, 0, 19, 21, 5, 10, 0, 0, 20, 11, 
		1, 0, 0, 0, 20, 14, 1, 0, 0, 0, 20, 19, 1, 0, 0, 0, 21, 3, 1, 0, 0, 0, 
		22, 23, 6, 2, -1, 0, 23, 30, 5, 9, 0, 0, 24, 30, 5, 8, 0, 0, 25, 26, 5, 
		2, 0, 0, 26, 27, 3, 4, 2, 0, 27, 28, 5, 3, 0, 0, 28, 30, 1, 0, 0, 0, 29, 
		22, 1, 0, 0, 0, 29, 24, 1, 0, 0, 0, 29, 25, 1, 0, 0, 0, 30, 39, 1, 0, 
		0, 0, 31, 32, 10, 5, 0, 0, 32, 33, 7, 0, 0, 0, 33, 38, 3, 4, 2, 6, 34, 
		35, 10, 4, 0, 0, 35, 36, 7, 1, 0, 0, 36, 38, 3, 4, 2, 5, 37, 31, 1, 0, 
		0, 0, 37, 34, 1, 0, 0, 0, 38, 41, 1, 0, 0, 0, 39, 37, 1, 0, 0, 0, 39, 
		40, 1, 0, 0, 0, 40, 5, 1, 0, 0, 0, 41, 39, 1, 0, 0, 0, 5, 9, 20, 29, 37, 
		39
	];
}
