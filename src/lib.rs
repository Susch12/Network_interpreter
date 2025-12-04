// src/lib.rs
// Biblioteca principal

// OLD lexer (legacy - to be removed after migration)
pub mod lexer;

// NEW lexer
#[path = "lexer_new/mod.rs"]
pub mod lexer_new;

pub use lexer_new::{Automaton, Scanner, Token as NewToken, TokenType};

// Bridge for converting between old and new lexer formats
pub mod lexer_bridge;

#[path = "config/mod.rs"]
pub mod config;

// Error reporting
pub mod error;

// Recursive Descent Parser (used by LL(1) for AST construction)
pub mod parser;

// LL(1) Predictive Parser
#[path = "parser_ll1/mod.rs"]
pub mod parser_ll1;

// AST (required by parser)
pub mod ast;
