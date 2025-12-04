// src/lexer_new/scanner.rs
// Scanner principal que usa el autómata para tokenizar

use super::automaton::Automaton;
use super::token::{Token, TokenType};
use super::error::LexicalError;

/// Scanner de código fuente
pub struct Scanner {
    /// Código fuente
    source: Vec<char>,
    
    /// Posición actual
    pos: usize,
    
    /// Línea actual (1-indexed)
    line: usize,
    
    /// Columna actual (1-indexed)
    column: usize,
    
    /// Referencia al autómata
    automaton: &'static Automaton,
}

impl Scanner {
    /// Crea un nuevo scanner
    pub fn new(source: &str, automaton: &'static Automaton) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            automaton,
        }
    }
    
    /// Escanea todos los tokens del código fuente
    pub fn scan_all(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens = Vec::new();
        
        loop {
            match self.scan_token()? {
                Some(token) => {
                    // Ignorar whitespace y comentarios
                    if !token.token_type.should_ignore() {
                        tokens.push(token);
                    }
                }
                None => break,
            }
        }
        
        // Agregar token EOF
        tokens.push(Token::eof(self.line, self.column));
        
        Ok(tokens)
    }
    
    /// Escanea el siguiente token
    pub fn scan_token(&mut self) -> Result<Option<Token>, LexicalError> {
        // Fin del archivo
        if self.is_at_end() {
            return Ok(None);
        }
        
        let start_pos = self.pos;
        let start_line = self.line;
        let start_column = self.column;
        
        // Ejecutar el autómata
        let mut current_state = self.automaton.initial_state();
        let mut last_final_state = None;
        let mut last_final_pos = start_pos;
        
        while !self.is_at_end() {
            let ch = self.current_char();
            
            // Intentar transición
            if let Some(next_state) = self.automaton.next_state(current_state, ch) {
                self.advance();
                current_state = next_state;
                
                // Si llegamos a un estado final, recordarlo
                if let Some(token_type) = self.automaton.is_final(current_state) {
                    last_final_state = Some(token_type.clone());
                    last_final_pos = self.pos;
                }
            } else {
                break;
            }
        }
        
        // Verificar si terminamos en un estado final
        if let Some(token_type) = last_final_state {
            // Retroceder a la última posición válida
            self.pos = last_final_pos;
            self.update_position_from(start_pos);
            
            let lexeme: String = self.source[start_pos..last_final_pos].iter().collect();
            
            // Clasificar identificadores (keywords vs identifiers)
            let final_token_type = if token_type == TokenType::Identifier {
                self.automaton.classify_identifier(&lexeme)
            } else {
                token_type
            };
            
            let token = Token::new(
                final_token_type,
                lexeme,
                start_line,
                start_column,
            );
            
            return Ok(Some(token));
        }
        
        // No se reconoció ningún token válido
        let ch = self.current_char();
        Err(LexicalError::invalid_char(ch, start_line, start_column))
    }
    
    /// Obtiene el carácter actual sin avanzar
    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.pos]
        }
    }
    
    /// Avanza una posición
    fn advance(&mut self) {
        if !self.is_at_end() {
            let ch = self.source[self.pos];
            self.pos += 1;
            
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
    
    /// Verifica si llegamos al final
    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }
    
    /// Actualiza line y column basándose en el contenido desde start_pos
    fn update_position_from(&mut self, start_pos: usize) {
        self.line = 1;
        self.column = 1;
        
        for i in 0..self.pos {
            if i >= self.source.len() {
                break;
            }
            
            if self.source[i] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::automaton::Automaton;
    use once_cell::sync::Lazy;

    // Create a simple test automaton inline
    fn create_test_automaton() -> Automaton {
        let content = r#"
METADATA
name: TestLexer
version: 1.0
initial_state: q0
END_METADATA

STATES
q0
q_id FINAL:IDENTIFIER
q_num FINAL:NUMBER
q_comma FINAL:COMMA
q_semicolon FINAL:SEMICOLON
q_ws FINAL:WHITESPACE
END_STATES

TRANSITIONS
q0, [a-zA-Z_], q_id
q_id, [a-zA-Z0-9_], q_id
q0, [0-9], q_num
q_num, [0-9], q_num
q0, ,, q_comma
q0, ;, q_semicolon
q0, \s, q_ws
q0, \n, q_ws
q_ws, \s, q_ws
q_ws, \n, q_ws
END_TRANSITIONS

KEYWORDS
programa, PROGRAMA
inicio, INICIO
fin, FIN
END_KEYWORDS
"#;
        Automaton::parse(content).expect("Failed to parse test automaton")
    }

    static TEST_AUTOMATON: Lazy<Automaton> = Lazy::new(|| {
        create_test_automaton()
    });

    #[test]
    fn test_scanner_creation() {
        let scanner = Scanner::new("test", &TEST_AUTOMATON);
        assert_eq!(scanner.pos, 0);
        assert_eq!(scanner.line, 1);
        assert_eq!(scanner.column, 1);
    }

    #[test]
    fn test_is_at_end() {
        let mut scanner = Scanner::new("", &TEST_AUTOMATON);
        assert!(scanner.is_at_end());

        let mut scanner = Scanner::new("a", &TEST_AUTOMATON);
        assert!(!scanner.is_at_end());
        scanner.advance();
        assert!(scanner.is_at_end());
    }

    #[test]
    fn test_advance() {
        let mut scanner = Scanner::new("abc\ndef", &TEST_AUTOMATON);

        assert_eq!(scanner.line, 1);
        assert_eq!(scanner.column, 1);

        scanner.advance(); // a
        assert_eq!(scanner.column, 2);

        scanner.advance(); // b
        scanner.advance(); // c
        scanner.advance(); // \n
        assert_eq!(scanner.line, 2);
        assert_eq!(scanner.column, 1);
    }

    #[test]
    fn test_scan_identifier() {
        let mut scanner = Scanner::new("test", &TEST_AUTOMATON);
        let token = scanner.scan_token().unwrap().unwrap();

        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.lexeme, "test");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_scan_number() {
        let mut scanner = Scanner::new("12345", &TEST_AUTOMATON);
        let token = scanner.scan_token().unwrap().unwrap();

        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.lexeme, "12345");
    }

    #[test]
    fn test_scan_keyword() {
        let mut scanner = Scanner::new("programa", &TEST_AUTOMATON);
        let token = scanner.scan_token().unwrap().unwrap();

        assert_eq!(token.token_type, TokenType::Programa);
        assert_eq!(token.lexeme, "programa");
    }

    #[test]
    fn test_scan_keyword_case_insensitive() {
        let mut scanner = Scanner::new("PROGRAMA", &TEST_AUTOMATON);
        let token = scanner.scan_token().unwrap().unwrap();

        assert_eq!(token.token_type, TokenType::Programa);
        assert_eq!(token.lexeme, "PROGRAMA");
    }

    #[test]
    fn test_scan_delimiter() {
        let mut scanner = Scanner::new(",", &TEST_AUTOMATON);
        let token = scanner.scan_token().unwrap().unwrap();

        assert_eq!(token.token_type, TokenType::Comma);
        assert_eq!(token.lexeme, ",");
    }

    #[test]
    fn test_scan_multiple_tokens() {
        let mut scanner = Scanner::new("var1,var2", &TEST_AUTOMATON);

        let token1 = scanner.scan_token().unwrap().unwrap();
        assert_eq!(token1.token_type, TokenType::Identifier);
        assert_eq!(token1.lexeme, "var1");

        let token2 = scanner.scan_token().unwrap().unwrap();
        assert_eq!(token2.token_type, TokenType::Comma);

        let token3 = scanner.scan_token().unwrap().unwrap();
        assert_eq!(token3.token_type, TokenType::Identifier);
        assert_eq!(token3.lexeme, "var2");
    }

    #[test]
    fn test_scan_all_tokens() {
        let mut scanner = Scanner::new("programa inicio fin", &TEST_AUTOMATON);
        let tokens = scanner.scan_all().unwrap();

        // Should have 3 keywords + EOF (whitespace ignored)
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Programa);
        assert_eq!(tokens[1].token_type, TokenType::Inicio);
        assert_eq!(tokens[2].token_type, TokenType::Fin);
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_scan_with_whitespace() {
        let mut scanner = Scanner::new("  test  ", &TEST_AUTOMATON);
        let tokens = scanner.scan_all().unwrap();

        // Whitespace should be ignored
        assert_eq!(tokens.len(), 2); // identifier + EOF
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "test");
    }

    #[test]
    fn test_scan_multiline() {
        let mut scanner = Scanner::new("var1\nvar2", &TEST_AUTOMATON);
        let tokens = scanner.scan_all().unwrap();

        assert_eq!(tokens.len(), 3); // 2 identifiers + EOF
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 2);
    }

    #[test]
    fn test_invalid_character() {
        let mut scanner = Scanner::new("@", &TEST_AUTOMATON);
        let result = scanner.scan_token();

        assert!(result.is_err());
    }

    #[test]
    fn test_empty_input() {
        let mut scanner = Scanner::new("", &TEST_AUTOMATON);
        let tokens = scanner.scan_all().unwrap();

        // Should only have EOF
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }
}
