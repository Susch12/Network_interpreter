// src/lexer_new/token.rs
// Definición de tokens del lenguaje

use std::fmt;

/// Tipos de tokens reconocidos por el lexer
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // ============ Palabras Reservadas ============
    Programa,
    Define,
    Maquinas,
    Concentradores,
    Coaxial,
    Segmento,
    Modulo,
    Inicio,
    Fin,
    Si,
    Sino,
    
    // ============ Funciones del Lenguaje ============
    Coloca,
    ColocaCoaxial,
    ColocaCoaxialConcentrador,
    UneMaquinaPuerto,
    AsignaPuerto,
    MaquinaCoaxial,
    AsignaMaquinaCoaxial,
    Escribe,
    
    // ============ Direcciones ============
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
    
    // ============ Operadores Relacionales ============
    Equal,          // =
    Less,           // 
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    NotEqual,       // <>
    
    // ============ Operadores Lógicos ============
    And,            // &&
    Or,             // ||
    Not,            // !
    
    // ============ Delimitadores ============
    Comma,          // ,
    Semicolon,      // ;
    Dot,            // .
    LParen,         // (
    RParen,         // )
    LBracket,       // [
    RBracket,       // ]
    
    // ============ Literales ============
    Identifier,     // [a-zA-Z_][a-zA-Z0-9_]*
    Number,         // [0-9]+
    String,         // "..."
    
    // ============ Especiales ============
    Whitespace,     // Espacios, tabs, newlines (ignorado)
    Comment,        // // ... (ignorado)
    Eof,            // Fin de archivo
}

impl TokenType {
    /// Convierte el tipo de token a string para mensajes de error
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::Programa => "programa",
            TokenType::Define => "define",
            TokenType::Maquinas => "maquinas",
            TokenType::Concentradores => "concentradores",
            TokenType::Coaxial => "coaxial",
            TokenType::Segmento => "segmento",
            TokenType::Modulo => "modulo",
            TokenType::Inicio => "inicio",
            TokenType::Fin => "fin",
            TokenType::Si => "si",
            TokenType::Sino => "sino",
            
            TokenType::Coloca => "coloca",
            TokenType::ColocaCoaxial => "colocaCoaxial",
            TokenType::ColocaCoaxialConcentrador => "colocaCoaxialConcentrador",
            TokenType::UneMaquinaPuerto => "uneMaquinaPuerto",
            TokenType::AsignaPuerto => "asignaPuerto",
            TokenType::MaquinaCoaxial => "maquinaCoaxial",
            TokenType::AsignaMaquinaCoaxial => "asignaMaquinaCoaxial",
            TokenType::Escribe => "escribe",
            
            TokenType::Arriba => "arriba",
            TokenType::Abajo => "abajo",
            TokenType::Izquierda => "izquierda",
            TokenType::Derecha => "derecha",
            
            TokenType::Equal => "=",
            TokenType::Less => "<",
            TokenType::Greater => ">",
            TokenType::LessEqual => "<=",
            TokenType::GreaterEqual => ">=",
            TokenType::NotEqual => "<>",
            
            TokenType::And => "&&",
            TokenType::Or => "||",
            TokenType::Not => "!",
            
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Dot => ".",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBracket => "[",
            TokenType::RBracket => "]",
            
            TokenType::Identifier => "identificador",
            TokenType::Number => "número",
            TokenType::String => "cadena",
            
            TokenType::Whitespace => "espacio",
            TokenType::Comment => "comentario",
            TokenType::Eof => "fin de archivo",
        }
    }
    
    /// Verifica si el token debe ser ignorado por el parser
    pub fn should_ignore(&self) -> bool {
        matches!(self, TokenType::Whitespace | TokenType::Comment)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Información completa de un token
#[derive(Debug, Clone)]
pub struct Token {
    /// Tipo de token
    pub token_type: TokenType,
    
    /// Lexema (texto original)
    pub lexeme: String,
    
    /// Línea en el código fuente
    pub line: usize,
    
    /// Columna en el código fuente
    pub column: usize,
    
    /// Longitud del lexema
    pub length: usize,
}

impl Token {
    /// Crea un nuevo token
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: usize,
        column: usize,
    ) -> Self {
        let length = lexeme.len();
        Self {
            token_type,
            lexeme,
            line,
            column,
            length,
        }
    }
    
    /// Crea un token EOF
    pub fn eof(line: usize, column: usize) -> Self {
        Self {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            line,
            column,
            length: 0,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} '{}' at {}:{}",
            self.token_type, self.lexeme, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_creation() {
        let token = Token::new(
            TokenType::Identifier,
            "test".to_string(),
            1,
            5,
        );
        
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.lexeme, "test");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 5);
        assert_eq!(token.length, 4);
    }
    
    #[test]
    fn test_eof_token() {
        let token = Token::eof(10, 20);
        
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.lexeme, "");
        assert_eq!(token.line, 10);
        assert_eq!(token.column, 20);
    }
    
    #[test]
    fn test_should_ignore() {
        assert!(TokenType::Whitespace.should_ignore());
        assert!(TokenType::Comment.should_ignore());
        assert!(!TokenType::Identifier.should_ignore());
        assert!(!TokenType::Number.should_ignore());
    }
    
    #[test]
    fn test_token_type_display() {
        assert_eq!(TokenType::Programa.as_str(), "programa");
        assert_eq!(TokenType::Equal.as_str(), "=");
        assert_eq!(TokenType::LessEqual.as_str(), "<=");
    }
}
