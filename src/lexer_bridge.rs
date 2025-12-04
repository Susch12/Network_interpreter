// Bridge between new automaton-based lexer and old parser interface
// This allows the parser to continue using the old Token format while
// we use the new lexer implementation

use super::lexer::{Token as OldToken, TokenInfo};
use super::lexer_new::automaton::Automaton;
use super::lexer_new::scanner::Scanner;
use super::lexer_new::token::{Token as NewToken, TokenType};
use once_cell::sync::Lazy;

// Load the automaton once at startup
static AUTOMATON: Lazy<Automaton> = Lazy::new(|| {
    Automaton::from_file("config/automaton.aut")
        .expect("Failed to load automaton.aut")
});

/// Convert new TokenType to old Token format
fn convert_token(new_token: &NewToken) -> OldToken {
    match &new_token.token_type {
        // Keywords
        TokenType::Programa => OldToken::Programa,
        TokenType::Define => OldToken::Define,
        TokenType::Maquinas => OldToken::Maquinas,
        TokenType::Concentradores => OldToken::Concentradores,
        TokenType::Coaxial => OldToken::Coaxial,
        TokenType::Segmento => OldToken::Segmento,
        TokenType::Modulo => OldToken::Modulo,
        TokenType::Inicio => OldToken::Inicio,
        TokenType::Fin => OldToken::Fin,
        TokenType::Si => OldToken::Si,
        TokenType::Sino => OldToken::Sino,

        // Functions
        TokenType::Coloca => OldToken::Coloca,
        TokenType::ColocaCoaxial => OldToken::ColocaCoaxial,
        TokenType::ColocaCoaxialConcentrador => OldToken::ColocaCoaxialConcentrador,
        TokenType::UneMaquinaPuerto => OldToken::UneMaquinaPuerto,
        TokenType::AsignaPuerto => OldToken::AsignaPuerto,
        TokenType::MaquinaCoaxial => OldToken::MaquinaCoaxial,
        TokenType::AsignaMaquinaCoaxial => OldToken::AsignaMaquinaCoaxial,
        TokenType::Escribe => OldToken::Escribe,

        // Directions
        TokenType::Arriba => OldToken::Arriba,
        TokenType::Abajo => OldToken::Abajo,
        TokenType::Izquierda => OldToken::Izquierda,
        TokenType::Derecha => OldToken::Derecha,

        // Operators
        TokenType::Equal => OldToken::Igual,
        TokenType::Less => OldToken::Menor,
        TokenType::Greater => OldToken::Mayor,
        TokenType::LessEqual => OldToken::MenorIgual,
        TokenType::GreaterEqual => OldToken::MayorIgual,
        TokenType::NotEqual => OldToken::Diferente,
        TokenType::And => OldToken::And,
        TokenType::Or => OldToken::Or,
        TokenType::Not => OldToken::Not,

        // Delimiters
        TokenType::Comma => OldToken::Coma,
        TokenType::Semicolon => OldToken::PuntoYComa,
        TokenType::Dot => OldToken::Punto,
        TokenType::LParen => OldToken::ParenIzq,
        TokenType::RParen => OldToken::ParenDer,
        TokenType::LBracket => OldToken::CorcheteIzq,
        TokenType::RBracket => OldToken::CorcheteDer,

        // Literals (these carry data in old format)
        TokenType::Identifier => OldToken::Identificador(new_token.lexeme.clone()),
        TokenType::Number => {
            // Parse the number from lexeme
            let num = new_token.lexeme.parse::<i32>().unwrap_or(0);
            OldToken::Numero(num)
        }
        TokenType::String => {
            // Remove quotes from string
            let s = new_token.lexeme.trim_matches('"').to_string();
            OldToken::Cadena(s)
        }

        // Special tokens
        TokenType::Whitespace | TokenType::Comment => {
            // These should have been filtered out
            panic!("Whitespace/Comment tokens should not reach here")
        }
        TokenType::Eof => {
            // EOF is handled specially - not converted
            panic!("EOF token should not be converted")
        }
    }
}

/// Tokenize source code using new lexer, return old format
pub fn tokenize_with_new_lexer(source: String) -> Result<Vec<TokenInfo>, String> {
    let mut scanner = Scanner::new(&source, &AUTOMATON);

    match scanner.scan_all() {
        Ok(tokens) => {
            let mut result = Vec::new();

            for new_token in tokens {
                // Skip EOF token - old lexer doesn't include it
                if new_token.token_type == TokenType::Eof {
                    continue;
                }

                // Skip whitespace and comments (should already be filtered)
                if new_token.token_type.should_ignore() {
                    continue;
                }

                let old_token = convert_token(&new_token);

                result.push(TokenInfo {
                    token: old_token,
                    line: new_token.line,
                    column: new_token.column,
                    length: new_token.length,
                    lexeme: new_token.lexeme.clone(),
                });
            }

            Ok(result)
        }
        Err(err) => Err(err.message),
    }
}
