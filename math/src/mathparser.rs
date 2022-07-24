// Generated from Math.g4 by ANTLR 4.8
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
use super::mathlistener::*;
use super::mathvisitor::*;

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
		pub const OP_ADD:isize=3; 
		pub const OP_SUB:isize=4; 
		pub const OP_MUL:isize=5; 
		pub const OP_DIV:isize=6; 
		pub const NUM:isize=7; 
		pub const ID:isize=8; 
		pub const WS:isize=9;
	pub const RULE_compileUnit:usize = 0; 
	pub const RULE_expr:usize = 1;
	pub const ruleNames: [&'static str; 2] =  [
		"compileUnit", "expr"
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


type BaseParserType<'input, I> =
	BaseParser<'input,MathParserExt<'input>, I, MathParserContextType , dyn MathListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type MathTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, MathParserContextType , dyn MathListener<'input> + 'a>;

/// Parser for Math grammar
pub struct MathParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> MathParser<'input, I, H>
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
				MathParserExt{
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

impl<'input, I> MathParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> MathParser<'input, I, DefaultErrorStrategy<'input,MathParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for MathParser
pub trait MathParserContext<'input>:
	for<'x> Listenable<dyn MathListener<'input> + 'x > + 
	for<'x> Visitable<dyn MathVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=MathParserContextType>
{}

antlr_rust::coerce_from!{ 'input : MathParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn MathParserContext<'input> + 'input
where
    T: MathVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn MathVisitor<'input> + 'x))
    }
}

impl<'input> MathParserContext<'input> for TerminalNode<'input,MathParserContextType> {}
impl<'input> MathParserContext<'input> for ErrorNode<'input,MathParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn MathParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn MathListener<'input> + 'input }

pub struct MathParserContextType;
antlr_rust::tid!{MathParserContextType}

impl<'input> ParserNodeType<'input> for MathParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn MathParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for MathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for MathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct MathParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> MathParserExt<'input>{
}
antlr_rust::tid! { MathParserExt<'a> }

impl<'input> TokenAware<'input> for MathParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for MathParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for MathParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Math.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn MathParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					1 => MathParser::<'input,I,_>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> MathParser<'input, I, DefaultErrorStrategy<'input,MathParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 4)
				}
				1=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
}
//------------------- compileUnit ----------------
pub type CompileUnitContextAll<'input> = CompileUnitContext<'input>;


pub type CompileUnitContext<'input> = BaseParserRuleContext<'input,CompileUnitContextExt<'input>>;

#[derive(Clone)]
pub struct CompileUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MathParserContext<'input> for CompileUnitContext<'input>{}

impl<'input,'a> Listenable<dyn MathListener<'input> + 'a> for CompileUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compileUnit(self);
		}
		fn exit(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
			listener.exit_compileUnit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn MathVisitor<'input> + 'a> for CompileUnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn MathVisitor<'input> + 'a)) {
		visitor.visit_compileUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompileUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compileUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compileUnit }
}
antlr_rust::tid!{CompileUnitContextExt<'a>}

impl<'input> CompileUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn MathParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompileUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompileUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompileUnitContextAttrs<'input>: MathParserContext<'input> + BorrowMut<CompileUnitContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,MathParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> CompileUnitContextAttrs<'input> for CompileUnitContext<'input>{}

impl<'input, I, H> MathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compileUnit(&mut self,)
	-> Result<Rc<CompileUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompileUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_compileUnit);
        let mut _localctx: Rc<CompileUnitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr*/
			recog.base.set_state(4);
			recog.expr_rec(0)?;

			recog.base.set_state(5);
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
//------------------- expr ----------------
#[derive(Debug)]
pub enum ExprContextAll<'input>{
	InfixExprContext(InfixExprContext<'input>),
	UnaryExprContext(UnaryExprContext<'input>),
	FuncExprContext(FuncExprContext<'input>),
	NumberExprContext(NumberExprContext<'input>),
	ParensExprContext(ParensExprContext<'input>),
Error(ExprContext<'input>)
}
antlr_rust::tid!{ExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExprContextAll<'input>{}

impl<'input> MathParserContext<'input> for ExprContextAll<'input>{}

impl<'input> Deref for ExprContextAll<'input>{
	type Target = dyn ExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			InfixExprContext(inner) => inner,
			UnaryExprContext(inner) => inner,
			FuncExprContext(inner) => inner,
			NumberExprContext(inner) => inner,
			ParensExprContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn MathVisitor<'input> + 'a> for ExprContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn MathVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn MathListener<'input> + 'a> for ExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn MathListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn MathListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> MathParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn MathListener<'input> + 'a> for ExprContext<'input>{
}

impl<'input,'a> Visitable<dyn MathVisitor<'input> + 'a> for ExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn MathParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
		ExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExprContextAttrs<'input>: MathParserContext<'input> + BorrowMut<ExprContextExt<'input>>{


}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

pub type InfixExprContext<'input> = BaseParserRuleContext<'input,InfixExprContextExt<'input>>;

pub trait InfixExprContextAttrs<'input>: MathParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OP_MUL
	/// Returns `None` if there is no child corresponding to token OP_MUL
	fn OP_MUL(&self) -> Option<Rc<TerminalNode<'input,MathParserContextType>>> where Self:Sized{
		self.get_token(OP_MUL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OP_DIV
	/// Returns `None` if there is no child corresponding to token OP_DIV
	fn OP_DIV(&self) -> Option<Rc<TerminalNode<'input,MathParserContextType>>> where Self:Sized{
		self.get_token(OP_DIV, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OP_ADD
	/// Returns `None` if there is no child corresponding to token OP_ADD
	fn OP_ADD(&self) -> Option<Rc<TerminalNode<'input,MathParserContextType>>> where Self:Sized{
		self.get_token(OP_ADD, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OP_SUB
	/// Returns `None` if there is no child corresponding to token OP_SUB
	fn OP_SUB(&self) -> Option<Rc<TerminalNode<'input,MathParserContextType>>> where Self:Sized{
		self.get_token(OP_SUB, 0)
	}
}

impl<'input> InfixExprContextAttrs<'input> for InfixExprContext<'input>{}

pub struct InfixExprContextExt<'input>{
	base:ExprContextExt<'input>,
	pub left: Option<Rc<ExprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub right: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{InfixExprContextExt<'a>}

impl<'input> MathParserContext<'input> for InfixExprContext<'input>{}

impl<'input,'a> Listenable<dyn MathListener<'input> + 'a> for InfixExprContext<'input>{
	fn enter(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_infixExpr(self);
	}
	fn exit(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.exit_infixExpr(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn MathVisitor<'input> + 'a> for InfixExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn MathVisitor<'input> + 'a)) {
		visitor.visit_infixExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for InfixExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for InfixExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for InfixExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for InfixExprContext<'input> {}

impl<'input> InfixExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::InfixExprContext(
				BaseParserRuleContext::copy_from(ctx,InfixExprContextExt{
					op:None, 
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UnaryExprContext<'input> = BaseParserRuleContext<'input,UnaryExprContextExt<'input>>;

pub trait UnaryExprContextAttrs<'input>: MathParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token OP_ADD
	/// Returns `None` if there is no child corresponding to token OP_ADD
	fn OP_ADD(&self) -> Option<Rc<TerminalNode<'input,MathParserContextType>>> where Self:Sized{
		self.get_token(OP_ADD, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OP_SUB
	/// Returns `None` if there is no child corresponding to token OP_SUB
	fn OP_SUB(&self) -> Option<Rc<TerminalNode<'input,MathParserContextType>>> where Self:Sized{
		self.get_token(OP_SUB, 0)
	}
}

impl<'input> UnaryExprContextAttrs<'input> for UnaryExprContext<'input>{}

pub struct UnaryExprContextExt<'input>{
	base:ExprContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{UnaryExprContextExt<'a>}

impl<'input> MathParserContext<'input> for UnaryExprContext<'input>{}

impl<'input,'a> Listenable<dyn MathListener<'input> + 'a> for UnaryExprContext<'input>{
	fn enter(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unaryExpr(self);
	}
	fn exit(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.exit_unaryExpr(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn MathVisitor<'input> + 'a> for UnaryExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn MathVisitor<'input> + 'a)) {
		visitor.visit_unaryExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for UnaryExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for UnaryExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for UnaryExprContext<'input> {}

impl<'input> UnaryExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::UnaryExprContext(
				BaseParserRuleContext::copy_from(ctx,UnaryExprContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FuncExprContext<'input> = BaseParserRuleContext<'input,FuncExprContextExt<'input>>;

pub trait FuncExprContextAttrs<'input>: MathParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,MathParserContextType>>> where Self:Sized{
		self.get_token(ID, 0)
	}
}

impl<'input> FuncExprContextAttrs<'input> for FuncExprContext<'input>{}

pub struct FuncExprContextExt<'input>{
	base:ExprContextExt<'input>,
	pub func: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FuncExprContextExt<'a>}

impl<'input> MathParserContext<'input> for FuncExprContext<'input>{}

impl<'input,'a> Listenable<dyn MathListener<'input> + 'a> for FuncExprContext<'input>{
	fn enter(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_funcExpr(self);
	}
	fn exit(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.exit_funcExpr(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn MathVisitor<'input> + 'a> for FuncExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn MathVisitor<'input> + 'a)) {
		visitor.visit_funcExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for FuncExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for FuncExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for FuncExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for FuncExprContext<'input> {}

impl<'input> FuncExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::FuncExprContext(
				BaseParserRuleContext::copy_from(ctx,FuncExprContextExt{
					func:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NumberExprContext<'input> = BaseParserRuleContext<'input,NumberExprContextExt<'input>>;

pub trait NumberExprContextAttrs<'input>: MathParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NUM
	/// Returns `None` if there is no child corresponding to token NUM
	fn NUM(&self) -> Option<Rc<TerminalNode<'input,MathParserContextType>>> where Self:Sized{
		self.get_token(NUM, 0)
	}
}

impl<'input> NumberExprContextAttrs<'input> for NumberExprContext<'input>{}

pub struct NumberExprContextExt<'input>{
	base:ExprContextExt<'input>,
	pub value: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NumberExprContextExt<'a>}

impl<'input> MathParserContext<'input> for NumberExprContext<'input>{}

impl<'input,'a> Listenable<dyn MathListener<'input> + 'a> for NumberExprContext<'input>{
	fn enter(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_numberExpr(self);
	}
	fn exit(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.exit_numberExpr(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn MathVisitor<'input> + 'a> for NumberExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn MathVisitor<'input> + 'a)) {
		visitor.visit_numberExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for NumberExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for NumberExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for NumberExprContext<'input> {}

impl<'input> NumberExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::NumberExprContext(
				BaseParserRuleContext::copy_from(ctx,NumberExprContextExt{
					value:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParensExprContext<'input> = BaseParserRuleContext<'input,ParensExprContextExt<'input>>;

pub trait ParensExprContextAttrs<'input>: MathParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ParensExprContextAttrs<'input> for ParensExprContext<'input>{}

pub struct ParensExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParensExprContextExt<'a>}

impl<'input> MathParserContext<'input> for ParensExprContext<'input>{}

impl<'input,'a> Listenable<dyn MathListener<'input> + 'a> for ParensExprContext<'input>{
	fn enter(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parensExpr(self);
	}
	fn exit(&self,listener: &mut (dyn MathListener<'input> + 'a)) {
		listener.exit_parensExpr(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn MathVisitor<'input> + 'a> for ParensExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn MathVisitor<'input> + 'a)) {
		visitor.visit_parensExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParensExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = MathParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ParensExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ParensExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ParensExprContext<'input> {}

impl<'input> ParensExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ParensExprContext(
				BaseParserRuleContext::copy_from(ctx,ParensExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> MathParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: isize)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 2, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 2;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(20);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__0 
				=> {
					{
					let mut tmp = ParensExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(8);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(9);
					recog.expr_rec(0)?;

					recog.base.set_state(10);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

			 OP_ADD | OP_SUB 
				=> {
					{
					let mut tmp = UnaryExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(12);
					if let ExprContextAll::UnaryExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
					_la = recog.base.input.la(1);
					if { !(_la==OP_ADD || _la==OP_SUB) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						if let ExprContextAll::UnaryExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
						ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expr*/
					recog.base.set_state(13);
					recog.expr_rec(5)?;

					}
				}

			 ID 
				=> {
					{
					let mut tmp = FuncExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(14);
					let tmp = recog.base.match_token(ID,&mut recog.err_handler)?;
					if let ExprContextAll::FuncExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.func = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(15);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(16);
					recog.expr_rec(0)?;

					recog.base.set_state(17);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

			 NUM 
				=> {
					{
					let mut tmp = NumberExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(19);
					let tmp = recog.base.match_token(NUM,&mut recog.err_handler)?;
					if let ExprContextAll::NumberExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.value = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(30);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(28);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = InfixExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::InfixExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(22);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(23);
							if let ExprContextAll::InfixExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==OP_MUL || _la==OP_DIV) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::InfixExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(24);
							let tmp = recog.expr_rec(5)?;
							if let ExprContextAll::InfixExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = InfixExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::InfixExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(25);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(26);
							if let ExprContextAll::InfixExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==OP_ADD || _la==OP_SUB) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::InfixExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(27);
							let tmp = recog.expr_rec(4)?;
							if let ExprContextAll::InfixExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(32);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
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
	\x0b\x24\x04\x02\x09\x02\x04\x03\x09\x03\x03\x02\x03\x02\x03\x02\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x05\x03\x17\x0a\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x07\x03\x1f\x0a\x03\x0c\x03\x0e\x03\x22\x0b\x03\x03\
	\x03\x02\x03\x04\x04\x02\x04\x02\x04\x03\x02\x05\x06\x03\x02\x07\x08\x02\
	\x26\x02\x06\x03\x02\x02\x02\x04\x16\x03\x02\x02\x02\x06\x07\x05\x04\x03\
	\x02\x07\x08\x07\x02\x02\x03\x08\x03\x03\x02\x02\x02\x09\x0a\x08\x03\x01\
	\x02\x0a\x0b\x07\x03\x02\x02\x0b\x0c\x05\x04\x03\x02\x0c\x0d\x07\x04\x02\
	\x02\x0d\x17\x03\x02\x02\x02\x0e\x0f\x09\x02\x02\x02\x0f\x17\x05\x04\x03\
	\x07\x10\x11\x07\x0a\x02\x02\x11\x12\x07\x03\x02\x02\x12\x13\x05\x04\x03\
	\x02\x13\x14\x07\x04\x02\x02\x14\x17\x03\x02\x02\x02\x15\x17\x07\x09\x02\
	\x02\x16\x09\x03\x02\x02\x02\x16\x0e\x03\x02\x02\x02\x16\x10\x03\x02\x02\
	\x02\x16\x15\x03\x02\x02\x02\x17\x20\x03\x02\x02\x02\x18\x19\x0c\x06\x02\
	\x02\x19\x1a\x09\x03\x02\x02\x1a\x1f\x05\x04\x03\x07\x1b\x1c\x0c\x05\x02\
	\x02\x1c\x1d\x09\x02\x02\x02\x1d\x1f\x05\x04\x03\x06\x1e\x18\x03\x02\x02\
	\x02\x1e\x1b\x03\x02\x02\x02\x1f\x22\x03\x02\x02\x02\x20\x1e\x03\x02\x02\
	\x02\x20\x21\x03\x02\x02\x02\x21\x05\x03\x02\x02\x02\x22\x20\x03\x02\x02\
	\x02\x05\x16\x1e\x20";

