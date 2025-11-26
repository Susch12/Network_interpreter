use logos::Logos;

// ============================================================================
// DEFINICIÓN DE TOKENS
// ============================================================================

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // ============ PALABRAS RESERVADAS ============
    #[token("programa")]
    Programa,
    
    #[token("define")]
    Define,
    
    #[token("maquinas")]
    Maquinas,
    
    #[token("concentradores")]
    Concentradores,
    
    #[token("coaxial")]
    Coaxial,
    
    #[token("segmento")] // ⚡ NUEVO: Soporte para "define segmento"
    Segmento,
    
    #[token("modulo")]
    Modulo,
    
    #[token("inicio")]
    Inicio,
    
    #[token("fin")]
    Fin,
    
    #[token("si")]
    Si,
    
    #[token("sino")]
    Sino,
    
    // ============ FUNCIONES DEL LENGUAJE ============
    #[token("coloca")]
    Coloca,
    
    #[token("colocaCoaxial")]
    ColocaCoaxial,
    
    #[token("colocaCoaxialConcentrador")]
    ColocaCoaxialConcentrador,
    
    #[token("uneMaquinaPuerto")]
    UneMaquinaPuerto,
    
    #[token("asignaPuerto")]
    AsignaPuerto,
    
    #[token("maquinaCoaxial")]
    MaquinaCoaxial,
    
    #[token("asignaMaquinaCoaxial")]
    AsignaMaquinaCoaxial,
    
    #[token("escribe")]
    Escribe,
    
    // ============ DIRECCIONES ============
    #[token("arriba")]
    Arriba,
    
    #[token("abajo")]
    Abajo,
    
    #[token("izquierda")]
    Izquierda,
    
    #[token("derecha")]
    Derecha,
    
    // ============ OPERADORES RELACIONALES ============
    #[token("=")]
    Igual,
    
    #[token("<")]
    Menor,
    
    #[token(">")]
    Mayor,
    
    #[token("<=")]
    MenorIgual,
    
    #[token(">=")]
    MayorIgual,
    
    #[token("<>")]
    Diferente,
    
    // ============ OPERADORES LÓGICOS ============
    #[token("&&")]
    And,
    
    #[token("||")]
    Or,
    
    #[token("!")]
    Not,
    
    // ============ DELIMITADORES ============
    #[token(",")]
    Coma,
    
    #[token(";")]
    PuntoYComa,
    
    #[token(".")]
    Punto,
    
    #[token("(")]
    ParenIzq,
    
    #[token(")")]
    ParenDer,
    
    #[token("[")]
    CorcheteIzq,
    
    #[token("]")]
    CorcheteDer,
    
    // ============ LITERALES ============
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identificador(String),
    
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Numero(i32),
    
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    Cadena(String),
    
    // ============ IGNORADOS ============
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
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

// ============================================================================
// LEXER
// ============================================================================

pub struct Lexer {
    pub input: String,
    tokens: Vec<TokenInfo>,
    errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> Result<&Vec<TokenInfo>, &Vec<LexerError>> {
        let mut lex = Token::lexer(&self.input);
        let mut line = 1;
        let mut column = 1;

        while let Some(result) = lex.next() {
            let span = lex.span();
            let lexeme = lex.slice().to_string();
            let length = span.len();

            match result {
                Ok(token) => {
                    // Verificar si es palabra reservada (case-insensitive para funciones)
                    let final_token = if let Token::Identificador(ref id) = token {
                        self.check_keyword(id).unwrap_or(token)
                    } else {
                        token
                    };

                    self.tokens.push(TokenInfo {
                        token: final_token,
                        line,
                        column,
                        length,
                        lexeme: lexeme.clone(),
                    });
                }
                Err(_) => {
                    self.errors.push(LexerError {
                        line,
                        column,
                        length,
                        message: format!("Token inválido: '{}'", lexeme),
                    });
                }
            }

            // Actualizar posición
            for c in lexeme.chars() {
                if c == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
            }
        }

        if self.errors.is_empty() {
            Ok(&self.tokens)
        } else {
            Err(&self.errors)
        }
    }

    fn check_keyword(&self, id: &str) -> Option<Token> {
        // Case-insensitive para palabras reservadas
        match id.to_lowercase().as_str() {
            "programa" => Some(Token::Programa),
            "define" => Some(Token::Define),
            "maquinas" => Some(Token::Maquinas),
            "concentradores" => Some(Token::Concentradores),
            "coaxial" => Some(Token::Coaxial),
            "segmento" => Some(Token::Segmento),
            "modulo" => Some(Token::Modulo),
            "inicio" => Some(Token::Inicio),
            "fin" => Some(Token::Fin),
            "si" => Some(Token::Si),
            "sino" => Some(Token::Sino),
            "arriba" => Some(Token::Arriba),
            "abajo" => Some(Token::Abajo),
            "izquierda" => Some(Token::Izquierda),
            "derecha" => Some(Token::Derecha),
            "escribe" => Some(Token::Escribe),
            // Funciones con camelCase - buscar exactamente
            "coloca" if id == "coloca" => Some(Token::Coloca),
            "colocacoaxial" if id == "colocaCoaxial" => Some(Token::ColocaCoaxial),
            "colocacoaxialconcentrador" if id == "colocaCoaxialConcentrador" => Some(Token::ColocaCoaxialConcentrador),
            "unemaquinapuerto" if id == "uneMaquinaPuerto" => Some(Token::UneMaquinaPuerto),
            "asignapuerto" if id == "asignaPuerto" => Some(Token::AsignaPuerto),
            "maquinacoaxial" if id == "maquinaCoaxial" => Some(Token::MaquinaCoaxial),
            "asignamaquinacoaxial" if id == "asignaMaquinaCoaxial" => Some(Token::AsignaMaquinaCoaxial),
            _ => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palabras_reservadas() {
        let input = "programa define maquinas concentradores coaxial segmento";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token, Token::Programa);
        assert_eq!(tokens[1].token, Token::Define);
        assert_eq!(tokens[2].token, Token::Maquinas);
        assert_eq!(tokens[3].token, Token::Concentradores);
        assert_eq!(tokens[4].token, Token::Coaxial);
        assert_eq!(tokens[5].token, Token::Segmento);
    }

    #[test]
    fn test_identificadores_y_numeros() {
        let input = "A B123 x_y 42 100";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].token, Token::Identificador(_)));
        assert!(matches!(tokens[1].token, Token::Identificador(_)));
        assert!(matches!(tokens[2].token, Token::Identificador(_)));
        assert_eq!(tokens[3].token, Token::Numero(42));
        assert_eq!(tokens[4].token, Token::Numero(100));
    }

    #[test]
    fn test_cadenas() {
        let input = r#""hola mundo" "test""#;
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::Cadena("hola mundo".to_string()));
        assert_eq!(tokens[1].token, Token::Cadena("test".to_string()));
    }

    #[test]
    fn test_operadores() {
        let input = "= < > <= >= <> && || !";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].token, Token::Igual);
        assert_eq!(tokens[1].token, Token::Menor);
        assert_eq!(tokens[2].token, Token::Mayor);
        assert_eq!(tokens[3].token, Token::MenorIgual);
        assert_eq!(tokens[4].token, Token::MayorIgual);
        assert_eq!(tokens[5].token, Token::Diferente);
        assert_eq!(tokens[6].token, Token::And);
        assert_eq!(tokens[7].token, Token::Or);
        assert_eq!(tokens[8].token, Token::Not);
    }

    #[test]
    fn test_delimitadores() {
        let input = ", ; . ( ) [ ]";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0].token, Token::Coma);
        assert_eq!(tokens[1].token, Token::PuntoYComa);
        assert_eq!(tokens[2].token, Token::Punto);
        assert_eq!(tokens[3].token, Token::ParenIzq);
        assert_eq!(tokens[4].token, Token::ParenDer);
        assert_eq!(tokens[5].token, Token::CorcheteIzq);
        assert_eq!(tokens[6].token, Token::CorcheteDer);
    }

    #[test]
    fn test_comentarios_ignorados() {
        let input = "programa // esto es un comentario\ndefine";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::Programa);
        assert_eq!(tokens[1].token, Token::Define);
    }

    #[test]
    fn test_programa_simple() {
        let input = r#"
programa test;
define maquinas
    A, B;
inicio
fin.
"#;
        let mut lexer = Lexer::new(input.to_string());
        let result = lexer.tokenize();
        
        assert!(result.is_ok());
        let tokens = result.unwrap();
        
        // Verificar algunos tokens clave
        assert_eq!(tokens[0].token, Token::Programa);
        assert!(matches!(tokens[1].token, Token::Identificador(_)));
        assert_eq!(tokens[2].token, Token::PuntoYComa);
    }
}
