// ============================================================================
// DEFINICIÓN DE TOKENS (Legacy - used by parser)
// ============================================================================
// This module defines the token types used by the parser.
// The actual lexical analysis is now done by the new automaton-based lexer
// in lexer_new/, and converted to these types via lexer_bridge.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // ============ PALABRAS RESERVADAS ============
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

    // ============ FUNCIONES DEL LENGUAJE ============
    Coloca,
    ColocaCoaxial,
    ColocaCoaxialConcentrador,
    UneMaquinaPuerto,
    AsignaPuerto,
    MaquinaCoaxial,
    AsignaMaquinaCoaxial,
    Escribe,

    // ============ DIRECCIONES ============
    Arriba,
    Abajo,
    Izquierda,
    Derecha,

    // ============ OPERADORES RELACIONALES ============
    Igual,
    Menor,
    Mayor,
    MenorIgual,
    MayorIgual,
    Diferente,

    // ============ OPERADORES LÓGICOS ============
    And,
    Or,
    Not,

    // ============ DELIMITADORES ============
    Coma,
    PuntoYComa,
    Punto,
    ParenIzq,
    ParenDer,
    CorcheteIzq,
    CorcheteDer,

    // ============ LITERALES ============
    Identificador(String),
    Numero(i32),
    Cadena(String),

    // ============ IGNORADOS ============
    Whitespace,
}

// ============================================================================
// INFORMACIÓN DE TOKENS
// ============================================================================

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub lexeme: String,
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub message: String,
}
