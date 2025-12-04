// Módulo de análisis léxico

pub mod automaton;
pub mod token;
pub mod scanner;
pub mod error;

// Re-export commonly used types
pub use automaton::Automaton;
pub use scanner::Scanner;
pub use token::{Token, TokenType};
pub use error::LexicalError;
