// src/parser_ll1/predictive.rs
// Parser LL(1) predictivo con pila explícita
// Este parser NO usa recursión, usa una pila explícita para implementar
// el análisis predictivo según la tabla LL(1)

use crate::lexer::{Token, TokenInfo};
use crate::ast::Program;
use crate::parser::Parser as RecursiveParser;
use super::first_follow::{Symbol, NonTerminal};
use super::ll1_table::{LL1Table, TokenClass};

/// Parser LL(1) predictivo
pub struct PredictiveParser {
    table: LL1Table,
    tokens: Vec<TokenInfo>,
    position: usize,
    stack: Vec<Symbol>,
    errors: Vec<String>,
}

impl PredictiveParser {
    /// Crea un nuevo parser predictivo
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self {
            table: LL1Table::new(),
            tokens,
            position: 0,
            stack: vec![Symbol::Eof, Symbol::NonTerminal(NonTerminal::Programa)],
            errors: Vec::new(),
        }
    }

    /// Obtiene el token actual
    fn current_token(&self) -> &Token {
        if self.position < self.tokens.len() {
            &self.tokens[self.position].token
        } else {
            &Token::Punto // EOF representado como Punto final
        }
    }

    /// Avanza al siguiente token
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    /// Registra un error
    fn error(&mut self, msg: String) {
        self.errors.push(msg);
    }

    /// Parser principal - Two-pass approach:
    /// Pass 1: LL(1) predictive parser validates syntax with parsing table
    /// Pass 2: Recursive descent parser builds AST
    ///
    /// This approach combines the strengths of both parsers:
    /// - LL(1) ensures rigorous validation against the formal grammar
    /// - Recursive descent provides clean AST construction
    pub fn parse(&mut self) -> Result<Program, String> {
        println!("🔍 Iniciando análisis híbrido (Two-Pass Approach)");
        println!("   Pass 1: Validación de sintaxis LL(1)");
        println!("   Pass 2: Construcción de AST con parser recursivo");

        // PASS 1: Validate syntax using LL(1) predictive parser
        self.validate_syntax()?;

        println!("   ✅ Pass 1 completado - Sintaxis válida");
        println!("   🔨 Pass 2: Construyendo AST...");

        // PASS 2: Build AST using recursive descent parser
        let mut recursive_parser = RecursiveParser::new(self.tokens.clone());
        match recursive_parser.parse() {
            Ok(program) => {
                println!("   ✅ Pass 2 completado - AST construido exitosamente");
                println!("✨ Análisis híbrido completado con éxito\n");
                Ok(program)
            }
            Err(errors) => {
                // This shouldn't happen if Pass 1 succeeded
                let error_msgs: Vec<String> = errors.iter()
                    .map(|e| format!("  - {}", e.message))
                    .collect();
                Err(format!(
                    "Error inesperado en Pass 2 (construcción de AST):\n{}",
                    error_msgs.join("\n")
                ))
            }
        }
    }

    /// Validates syntax only using LL(1) predictive algorithm
    ///
    /// Algoritmo LL(1) predictivo:
    /// 1. Inicializar pila con $ (EOF) y símbolo inicial
    /// 2. Mientras la pila no esté vacía:
    ///    a. Sea X el tope de la pila
    ///    b. Sea a el token actual
    ///    c. Si X es terminal:
    ///       - Si X == a: hacer pop y avanzar
    ///       - Si X != a: error
    ///    d. Si X es no-terminal:
    ///       - Consultar M[X, a]
    ///       - Si existe producción X → Y₁Y₂...Yₖ:
    ///         * Hacer pop de X
    ///         * Hacer push de Yₖ, Yₖ₋₁, ..., Y₁ (en orden inverso)
    ///       - Si no existe: error
    pub fn validate_syntax(&mut self) -> Result<(), String> {
        // Reset parser state for validation
        self.position = 0;
        self.stack = vec![Symbol::Eof, Symbol::NonTerminal(NonTerminal::Programa)];
        self.errors.clear();

        let mut step = 0;

        while !self.stack.is_empty() {
            step += 1;
            let top = self.stack.pop().unwrap();
            let current = self.current_token();

            if step <= 10 || step % 50 == 0 {
                println!("   Paso {}: Top={:?}, Token={:?}", step, top, current);
            }

            match top {
                Symbol::Epsilon => {
                    // Epsilon: no hacer nada, continuar
                    continue;
                }

                Symbol::Eof => {
                    // Verificar fin de archivo
                    if self.position >= self.tokens.len() {
                        println!("   ✅ Validación LL(1) completada exitosamente en {} pasos", step);
                        return Ok(());
                    } else {
                        self.error(format!(
                            "Se esperaba EOF pero se encontró {:?} en posición {}",
                            current, self.position
                        ));
                        return Err(self.errors.join("\n"));
                    }
                }

                Symbol::Terminal(expected) => {
                    // Comparar terminal con token actual
                    if Self::tokens_match(&expected, current) {
                        self.advance();
                    } else {
                        self.error(format!(
                            "Error de sintaxis: se esperaba {:?} pero se encontró {:?} en posición {}",
                            expected, current, self.position
                        ));
                        return Err(self.errors.join("\n"));
                    }
                }

                Symbol::NonTerminal(nt) => {
                    // Consultar tabla LL(1)
                    match self.table.get(nt, current) {
                        Some(production) => {
                            // Aplicar producción: hacer push de RHS en orden inverso
                            for symbol in production.rhs.iter().rev() {
                                self.stack.push(symbol.clone());
                            }

                            if step <= 10 {
                                println!("   Aplicando producción {}: {} → {:?}",
                                    production.id, nt.as_str(), production.rhs);
                            }
                        }
                        None => {
                            self.error(format!(
                                "Error de sintaxis: no hay producción para M[{}, {:?}] en posición {}",
                                nt.as_str(), TokenClass::from_token(current), self.position
                            ));
                            return Err(self.errors.join("\n"));
                        }
                    }
                }
            }
        }

        // Should never reach here if stack is properly initialized
        Err("Error interno: el stack quedó vacío sin completar el análisis".to_string())
    }

    /// Compara si dos tokens coinciden (ignorando valores en Some variants)
    ///
    /// Nota especial: Cuando esperamos un IDENTIFICADOR, también aceptamos palabras
    /// reservadas que pueden usarse como nombres de campo (ej: uno.coaxial, seg1.completo)
    fn tokens_match(expected: &Token, actual: &Token) -> bool {
        use Token::*;

        match (expected, actual) {
            // Tokens con valores: comparar solo el tipo
            (Identificador(_), Identificador(_)) => true,
            (Numero(_), Numero(_)) => true,
            (Cadena(_), Cadena(_)) => true,

            // Caso especial: permitir keywords como identificadores (nombres de campo)
            // Esto maneja casos como: uno.coaxial, seg1.completo, uno.presente
            (Identificador(_), keyword) if Self::is_valid_field_name(keyword) => true,

            // Tokens sin valores: comparación exacta
            (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
        }
    }

    /// Verifica si un token puede usarse como nombre de campo
    /// (compatible con parser.rs token_to_field_name)
    fn is_valid_field_name(token: &Token) -> bool {
        matches!(token,
            Token::Coaxial | Token::Segmento | Token::Maquinas | Token::Concentradores |
            Token::Derecha | Token::Izquierda | Token::Arriba | Token::Abajo |
            // Estos se usan en ejemplo1.net
            Token::Modulo  // para campos personalizados
        )
    }

    /// Obtiene los errores acumulados
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token_info(token: Token, line: usize, column: usize) -> TokenInfo {
        TokenInfo {
            token: token.clone(),
            line,
            column,
            length: 1,
            lexeme: format!("{:?}", token),
        }
    }

    #[test]
    fn test_parser_creation() {
        let tokens = vec![
            make_token_info(Token::Programa, 1, 1),
            make_token_info(Token::Identificador("test".to_string()), 1, 10),
            make_token_info(Token::PuntoYComa, 1, 14),
            make_token_info(Token::Inicio, 2, 1),
            make_token_info(Token::Fin, 3, 1),
            make_token_info(Token::Punto, 3, 4),
        ];

        let parser = PredictiveParser::new(tokens);
        assert_eq!(parser.position, 0);
        assert!(!parser.stack.is_empty());
    }

    #[test]
    fn test_tokens_match() {
        assert!(PredictiveParser::tokens_match(
            &Token::Identificador("a".to_string()),
            &Token::Identificador("b".to_string())
        ));

        assert!(PredictiveParser::tokens_match(
            &Token::Numero(0),
            &Token::Numero(42)
        ));

        assert!(PredictiveParser::tokens_match(
            &Token::Programa,
            &Token::Programa
        ));

        assert!(!PredictiveParser::tokens_match(
            &Token::Programa,
            &Token::Modulo
        ));
    }

    #[test]
    fn test_simple_program() {
        // programa test; inicio fin .
        let tokens = vec![
            make_token_info(Token::Programa, 1, 1),
            make_token_info(Token::Identificador("test".to_string()), 1, 10),
            make_token_info(Token::PuntoYComa, 1, 14),
            make_token_info(Token::Inicio, 2, 1),
            make_token_info(Token::Fin, 3, 1),
            make_token_info(Token::Punto, 3, 4),
        ];

        let mut parser = PredictiveParser::new(tokens);
        let result = parser.parse();

        if let Err(e) = &result {
            println!("Error: {}", e);
        }

        assert!(result.is_ok(), "Parser should accept valid program");
    }

    #[test]
    fn test_program_with_definitions() {
        // programa test; define maquinas m1; inicio fin .
        let tokens = vec![
            make_token_info(Token::Programa, 1, 1),
            make_token_info(Token::Identificador("test".to_string()), 1, 10),
            make_token_info(Token::PuntoYComa, 1, 14),
            make_token_info(Token::Define, 2, 1),
            make_token_info(Token::Maquinas, 2, 8),
            make_token_info(Token::Identificador("m1".to_string()), 2, 17),
            make_token_info(Token::PuntoYComa, 2, 19),
            make_token_info(Token::Inicio, 3, 1),
            make_token_info(Token::Fin, 4, 1),
            make_token_info(Token::Punto, 4, 4),
        ];

        let mut parser = PredictiveParser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok(), "Parser should accept program with definitions");
    }

    #[test]
    fn test_invalid_program() {
        // programa sin PUNTO_COMA
        let tokens = vec![
            make_token_info(Token::Programa, 1, 1),
            make_token_info(Token::Identificador("test".to_string()), 1, 10),
            // Falta PUNTO_COMA
            make_token_info(Token::Inicio, 2, 1),
            make_token_info(Token::Fin, 3, 1),
            make_token_info(Token::Punto, 3, 4),
        ];

        let mut parser = PredictiveParser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err(), "Parser should reject invalid program");
    }
}
