[Rust inspiration from Haskell](https://www.reddit.com/r/haskell/comments/6ugzhe/walking_the_ast_without_the_visitor_pattern/)  
[Follow up](https://maikklein.github.io/traversing-ast/) = this module source code

Initial problem in : [rlsl](https://github.com/MaikKlein/rlsl) deprecated in favor of rust-gpu  
Ressource from : https://github.com/EmbarkStudios/rust-gpu/pull/149 :
* [type-based value visitor](https://github.com/rust-lang/rust/blob/master/compiler/rustc_mir/src/interpret/visitor.rs)
* [visitor that handles references in a type-driven way](https://github.com/rust-lang/rust/blob/2eb4fc800aaf5006f89af3af591e2aa34f469d81/compiler/rustc_mir/src/interpret/validity.rs#L545)