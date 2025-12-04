// src/lexer/error.rs
// Errores del análisis léxico

use std::fmt;

/// Error léxico con información de ubicación
#[derive(Debug, Clone)]
pub struct LexicalError {
    /// Mensaje de error
    pub message: String,
    
    /// Línea donde ocurrió el error
    pub line: usize,
    
    /// Columna donde ocurrió el error
    pub column: usize,
    
    /// Longitud del texto problemático
    pub length: usize,
}

impl LexicalError {
    /// Crea un nuevo error léxico
    pub fn new(message: String, line: usize, column: usize, length: usize) -> Self {
        Self {
            message,
            line,
            column,
            length,
        }
    }
    
    /// Error por carácter inválido
    pub fn invalid_char(ch: char, line: usize, column: usize) -> Self {
        Self::new(
            format!("Carácter inválido: '{}'", ch),
            line,
            column,
            1,
        )
    }
    
    /// Error por cadena sin cerrar
    pub fn unterminated_string(line: usize, column: usize) -> Self {
        Self::new(
            "Cadena sin cerrar (falta comilla final)".to_string(),
            line,
            column,
            1,
        )
    }
    
    /// Error por número inválido
    pub fn invalid_number(lexeme: String, line: usize, column: usize) -> Self {
        Self::new(
            format!("Número inválido: '{}'", lexeme),
            line,
            column,
            lexeme.len(),
        )
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error léxico en {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for LexicalError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invalid_char_error() {
        let err = LexicalError::invalid_char('@', 5, 10);
        assert_eq!(err.line, 5);
        assert_eq!(err.column, 10);
        assert!(err.message.contains("@"));
    }
    
    #[test]
    fn test_unterminated_string_error() {
        let err = LexicalError::unterminated_string(3, 7);
        assert_eq!(err.line, 3);
        assert_eq!(err.column, 7);
        assert!(err.message.contains("sin cerrar"));
    }
    
    #[test]
    fn test_error_display() {
        let err = LexicalError::new(
            "Test error".to_string(),
            1,
            5,
            3,
        );
        let display = format!("{}", err);
        assert!(display.contains("1:5"));
        assert!(display.contains("Test error"));
    }
}
