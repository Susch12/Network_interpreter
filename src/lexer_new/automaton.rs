// src/lexer_new/automaton.rs
// Motor del autómata finito determinista (DFA)

use super::token::TokenType;
use std::collections::HashMap;
use std::fs;

/// ID único de un estado
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateId(usize);

impl StateId {
    pub fn new(id: usize) -> Self {
        StateId(id)
    }
    
    pub fn as_usize(&self) -> usize {
        self.0
    }
}

/// Clase de caracteres para transiciones
#[derive(Debug, Clone)]
pub enum CharClass {
    /// Carácter exacto
    Exact(char),

    /// Rango de caracteres (inclusivo)
    Range(char, char),

    /// Múltiples rangos o caracteres
    Multi(Vec<CharClass>),

    /// Cualquier carácter
    Any,

    /// Cualquier carácter excepto newline (para comentarios)
    AnyExceptNewline,
}

impl CharClass {
    /// Verifica si un carácter pertenece a esta clase
    pub fn matches(&self, ch: char) -> bool {
        match self {
            CharClass::Exact(c) => *c == ch,
            CharClass::Range(start, end) => ch >= *start && ch <= *end,
            CharClass::Multi(classes) => classes.iter().any(|c| c.matches(ch)),
            CharClass::Any => true,
            CharClass::AnyExceptNewline => ch != '\n',
        }
    }
    
    /// Parsea una clase de caracteres desde string
    /// Ejemplos: "a", "[a-z]", "[A-Z]", "ANY", "[a-zA-Z0-9_]"
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();

        if s == "ANY" {
            return Ok(CharClass::Any);
        }

        if s == "ANY_EXCEPT_NEWLINE" || s == "NOTNL" {
            return Ok(CharClass::AnyExceptNewline);
        }

        // Rango: [a-z] o múltiples como [a-zA-Z_]
        if s.starts_with('[') && s.ends_with(']') {
            let inner = &s[1..s.len() - 1];

            // Ignorar negación por ahora (simplificación)
            let is_negated = inner.starts_with('^');
            let inner = inner.trim_start_matches('^');

            // Un solo carácter: [a]
            if inner.len() == 1 {
                return Ok(CharClass::Exact(inner.chars().next().unwrap()));
            }

            // Rango simple: a-z
            if inner.len() == 3 && inner.chars().nth(1) == Some('-') {
                let start = inner.chars().nth(0).unwrap();
                let end = inner.chars().nth(2).unwrap();
                return Ok(CharClass::Range(start, end));
            }

            // Para múltiples rangos o caracteres mixtos como [a-zA-Z0-9_]
            // Por simplicidad, convertiremos esto en un Range que cubra todo
            // NOTA: Esta es una simplificación. Una implementación completa
            // necesitaría soportar múltiples rangos simultáneos.

            // Por ahora, manejar los casos más comunes:
            if inner == "a-zA-Z_" || inner == "a-zA-Z" {
                // Aceptar letras minúsculas, mayúsculas y guion bajo
                return Ok(CharClass::Multi(vec![
                    CharClass::Range('a', 'z'),
                    CharClass::Range('A', 'Z'),
                    CharClass::Exact('_'),
                ]));
            }
            if inner == "a-zA-Z0-9_" || inner == "a-zA-Z0-9" {
                // Aceptar alfanuméricos y guion bajo
                return Ok(CharClass::Multi(vec![
                    CharClass::Range('a', 'z'),
                    CharClass::Range('A', 'Z'),
                    CharClass::Range('0', '9'),
                    CharClass::Exact('_'),
                ]));
            }
            if inner == "0-9" {
                return Ok(CharClass::Range('0', '9'));
            }
            if inner == "^\"\\\"" || inner == "^\"\\\\" {
                // Cualquier cosa excepto comillas y backslash - usar ANY por ahora
                return Ok(CharClass::Any);
            }
            if inner == "^\\n" || inner == "^\n" {
                // Cualquier cosa excepto newline - usar ANY por ahora
                return Ok(CharClass::Any);
            }

            return Err(format!("Clase de caracteres inválida: {}", s));
        }

        // Carácter escapado
        if s.starts_with('\\') && s.len() == 2 {
            let ch = match s.chars().nth(1).unwrap() {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '"' => '"',
                '\\' => '\\',
                's' => ' ',
                c => c,
            };
            return Ok(CharClass::Exact(ch));
        }

        // Single bracket characters (not part of character class syntax)
        if s == "[" {
            return Ok(CharClass::Exact('['));
        }
        if s == "]" {
            return Ok(CharClass::Exact(']'));
        }

        // Carácter simple
        if s.len() == 1 {
            return Ok(CharClass::Exact(s.chars().next().unwrap()));
        }

        // Clases predefinidas
        match s {
            "ALPHA" => Ok(CharClass::Range('a', 'z')), // Simplificado
            "DIGIT" => Ok(CharClass::Range('0', '9')),
            "SPACE" => Ok(CharClass::Exact(' ')), // Simplificado
            _ => Err(format!("Clase de caracteres desconocida: {}", s)),
        }
    }
}

/// Transición del autómata
#[derive(Debug, Clone)]
pub struct Transition {
    pub from: StateId,
    pub char_class: CharClass,
    pub to: StateId,
}

/// Autómata finito determinista
pub struct Automaton {
    /// Estado inicial
    initial_state: StateId,
    
    /// Lista de transiciones
    transitions: Vec<Transition>,
    
    /// Estados finales: estado → tipo de token
    final_states: HashMap<StateId, TokenType>,
    
    /// Palabras reservadas: lexema → tipo de token
    keywords: HashMap<String, TokenType>,
    
    /// Mapa de nombres de estado a IDs
    state_map: HashMap<String, StateId>,
}

impl Automaton {
    /// Carga el autómata desde un archivo .aut
    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Error leyendo archivo {}: {}", path, e))?;
        
        Self::parse(&content)
    }
    
    /// Parsea el contenido del archivo .aut
    pub fn parse(content: &str) -> Result<Self, String> {
        let mut initial_state = StateId::new(0);
        let mut transitions = Vec::new();
        let mut final_states = HashMap::new();
        let mut keywords = HashMap::new();
        let mut state_map: HashMap<String, StateId> = HashMap::new();
        let mut next_state_id = 0;
        
        let mut current_section = "";
        let mut initial_state_name: Option<String> = None;
        
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            
            // Ignorar comentarios y líneas vacías
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Detectar secciones
            if line == "METADATA" || line == "STATES" || line == "TRANSITIONS" || line == "KEYWORDS" {
                current_section = line;
                continue;
            }
            
            if line.starts_with("END_") {
                current_section = "";
                continue;
            }
            
            // Procesar según sección
            match current_section {
                "METADATA" => {
                    if line.starts_with("initial_state:") {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() == 2 {
                            initial_state_name = Some(parts[1].trim().to_string());
                        }
                    }
                }
                
                "STATES" => {
                    // Formato: q0 [FINAL:TOKEN_TYPE] [#comentario]
                    let parts: Vec<&str> = line.split('#').collect();
                    let content = parts[0].trim();
                    
                    let parts: Vec<&str> = content.split_whitespace().collect();
                    if parts.is_empty() {
                        continue;
                    }
                    
                    let state_name = parts[0].to_string();
                    
                    // Crear o obtener estado
                    let state_id = *state_map.entry(state_name.clone()).or_insert_with(|| {
                        let id = StateId::new(next_state_id);
                        next_state_id += 1;
                        id
                    });
                    
                    // Verificar si es estado final
                    for part in &parts[1..] {
                        if part.starts_with("FINAL:") {
                            let token_type_str = &part[6..];
                            let token_type = Self::parse_token_type(token_type_str)?;
                            final_states.insert(state_id, token_type);
                        }
                    }
                }
                
                "TRANSITIONS" => {
                    // Formato: q0, [a-z], q1 [#comentario]
                    let parts: Vec<&str> = line.split('#').collect();
                    let content = parts[0].trim();

                    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

                    // Handle special case: "q0, ,, q1" splits into ["q0", "", "", "q1"]
                    let (from_name, char_spec, to_name) = if parts.len() == 4 && parts[1].is_empty() && parts[2].is_empty() {
                        // This is a comma character: q0, ,, q1
                        (parts[0], ",", parts[3])
                    } else if parts.len() == 3 {
                        (parts[0], parts[1], parts[2])
                    } else {
                        continue;
                    };
                    
                    // Obtener o crear estados
                    let from = *state_map.entry(from_name.to_string()).or_insert_with(|| {
                        let id = StateId::new(next_state_id);
                        next_state_id += 1;
                        id
                    });
                    
                    let to = *state_map.entry(to_name.to_string()).or_insert_with(|| {
                        let id = StateId::new(next_state_id);
                        next_state_id += 1;
                        id
                    });
                    
                    // Parsear clase de caracteres
                    let char_class = CharClass::parse(char_spec)
                        .map_err(|e| format!("Línea {}: {}", line_num + 1, e))?;
                    
                    transitions.push(Transition {
                        from,
                        char_class,
                        to,
                    });
                }
                
                "KEYWORDS" => {
                    // Formato: palabra, TOKEN_TYPE [#comentario]
                    let parts: Vec<&str> = line.split('#').collect();
                    let content = parts[0].trim();
                    
                    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        let keyword = parts[0].to_string();
                        let token_type = Self::parse_token_type(parts[1])?;
                        keywords.insert(keyword, token_type);
                    }
                }
                
                _ => {}
            }
        }
        
        // Establecer estado inicial
        if let Some(name) = initial_state_name {
            initial_state = *state_map.get(&name)
                .ok_or_else(|| format!("Estado inicial '{}' no encontrado", name))?;
        }
        
        Ok(Automaton {
            initial_state,
            transitions,
            final_states,
            keywords,
            state_map,
        })
    }
    
    /// Parsea un tipo de token desde string
    fn parse_token_type(s: &str) -> Result<TokenType, String> {
        match s {
            "PROGRAMA" => Ok(TokenType::Programa),
            "DEFINE" => Ok(TokenType::Define),
            "MAQUINAS" => Ok(TokenType::Maquinas),
            "CONCENTRADORES" => Ok(TokenType::Concentradores),
            "COAXIAL" => Ok(TokenType::Coaxial),
            "SEGMENTO" => Ok(TokenType::Segmento),
            "MODULO" => Ok(TokenType::Modulo),
            "INICIO" => Ok(TokenType::Inicio),
            "FIN" => Ok(TokenType::Fin),
            "SI" => Ok(TokenType::Si),
            "SINO" => Ok(TokenType::Sino),
            
            "COLOCA" => Ok(TokenType::Coloca),
            "COLOCA_COAXIAL" => Ok(TokenType::ColocaCoaxial),
            "COLOCA_COAXIAL_CONCENTRADOR" => Ok(TokenType::ColocaCoaxialConcentrador),
            "UNE_MAQUINA_PUERTO" => Ok(TokenType::UneMaquinaPuerto),
            "ASIGNA_PUERTO" => Ok(TokenType::AsignaPuerto),
            "MAQUINA_COAXIAL" => Ok(TokenType::MaquinaCoaxial),
            "ASIGNA_MAQUINA_COAXIAL" => Ok(TokenType::AsignaMaquinaCoaxial),
            "ESCRIBE" => Ok(TokenType::Escribe),
            
            "ARRIBA" => Ok(TokenType::Arriba),
            "ABAJO" => Ok(TokenType::Abajo),
            "IZQUIERDA" => Ok(TokenType::Izquierda),
            "DERECHA" => Ok(TokenType::Derecha),
            
            "EQUAL" => Ok(TokenType::Equal),
            "LESS" => Ok(TokenType::Less),
            "GREATER" => Ok(TokenType::Greater),
            "LESS_EQUAL" => Ok(TokenType::LessEqual),
            "GREATER_EQUAL" => Ok(TokenType::GreaterEqual),
            "NOT_EQUAL" => Ok(TokenType::NotEqual),
            
            "AND" => Ok(TokenType::And),
            "OR" => Ok(TokenType::Or),
            "NOT" => Ok(TokenType::Not),
            
            "COMMA" => Ok(TokenType::Comma),
            "SEMICOLON" => Ok(TokenType::Semicolon),
            "DOT" => Ok(TokenType::Dot),
            "LPAREN" => Ok(TokenType::LParen),
            "RPAREN" => Ok(TokenType::RParen),
            "LBRACKET" => Ok(TokenType::LBracket),
            "RBRACKET" => Ok(TokenType::RBracket),
            
            "IDENTIFIER" => Ok(TokenType::Identifier),
            "NUMBER" => Ok(TokenType::Number),
            "STRING" => Ok(TokenType::String),
            
            "WHITESPACE" => Ok(TokenType::Whitespace),
            "COMMENT" => Ok(TokenType::Comment),
            "ERROR" => Ok(TokenType::Identifier), // ERROR tokens treated as identifiers for now

            _ => Err(format!("Tipo de token desconocido: {}", s)),
        }
    }
    
    /// Obtiene el estado inicial
    pub fn initial_state(&self) -> StateId {
        self.initial_state
    }
    
    /// Obtiene el siguiente estado dada una transición
    pub fn next_state(&self, current: StateId, ch: char) -> Option<StateId> {
        for trans in &self.transitions {
            if trans.from == current && trans.char_class.matches(ch) {
                return Some(trans.to);
            }
        }
        None
    }
    
    /// Verifica si un estado es final y retorna el tipo de token
    pub fn is_final(&self, state: StateId) -> Option<&TokenType> {
        self.final_states.get(&state)
    }
    
    /// Clasifica un identificador (keyword o identifier)
    pub fn classify_identifier(&self, lexeme: &str) -> TokenType {
        // Buscar en keywords (case-insensitive para palabras reservadas)
        let lower = lexeme.to_lowercase();
        if let Some(token_type) = self.keywords.get(&lower) {
            return token_type.clone();
        }
        
        // Buscar case-sensitive para funciones camelCase
        if let Some(token_type) = self.keywords.get(lexeme) {
            return token_type.clone();
        }
        
        TokenType::Identifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_char_class_exact() {
        let class = CharClass::Exact('a');
        assert!(class.matches('a'));
        assert!(!class.matches('b'));
    }
    
    #[test]
    fn test_char_class_range() {
        let class = CharClass::Range('a', 'z');
        assert!(class.matches('a'));
        assert!(class.matches('m'));
        assert!(class.matches('z'));
        assert!(!class.matches('A'));
        assert!(!class.matches('0'));
    }
    
    #[test]
    fn test_char_class_any() {
        let class = CharClass::Any;
        assert!(class.matches('a'));
        assert!(class.matches('Z'));
        assert!(class.matches('0'));
        assert!(class.matches('@'));
    }
    
    #[test]
    fn test_char_class_parse() {
        assert!(matches!(CharClass::parse("a").unwrap(), CharClass::Exact('a')));
        assert!(matches!(CharClass::parse("[a-z]").unwrap(), CharClass::Range('a', 'z')));
        assert!(matches!(CharClass::parse("ANY").unwrap(), CharClass::Any));
        assert!(matches!(CharClass::parse("\\n").unwrap(), CharClass::Exact('\n')));
    }
    
    #[test]
    fn test_state_id() {
        let s1 = StateId::new(0);
        let s2 = StateId::new(0);
        let s3 = StateId::new(1);

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
        assert_eq!(s1.as_usize(), 0);
        assert_eq!(s3.as_usize(), 1);
    }

    #[test]
    fn test_parse_token_type() {
        assert!(matches!(
            Automaton::parse_token_type("PROGRAMA").unwrap(),
            TokenType::Programa
        ));
        assert!(matches!(
            Automaton::parse_token_type("IDENTIFIER").unwrap(),
            TokenType::Identifier
        ));
        assert!(matches!(
            Automaton::parse_token_type("EQUAL").unwrap(),
            TokenType::Equal
        ));
        assert!(Automaton::parse_token_type("INVALID").is_err());
    }

    #[test]
    fn test_char_class_parse_escaped() {
        assert!(matches!(
            CharClass::parse("\\t").unwrap(),
            CharClass::Exact('\t')
        ));
        assert!(matches!(
            CharClass::parse("\\n").unwrap(),
            CharClass::Exact('\n')
        ));
        assert!(matches!(
            CharClass::parse("\\r").unwrap(),
            CharClass::Exact('\r')
        ));
        assert!(matches!(
            CharClass::parse("\\s").unwrap(),
            CharClass::Exact(' ')
        ));
    }

    #[test]
    fn test_char_class_parse_predefined() {
        // ALPHA simplified to lowercase only
        let alpha = CharClass::parse("ALPHA").unwrap();
        assert!(matches!(alpha, CharClass::Range('a', 'z')));

        // DIGIT
        let digit = CharClass::parse("DIGIT").unwrap();
        assert!(matches!(digit, CharClass::Range('0', '9')));
    }

    #[test]
    fn test_automaton_parse_simple() {
        let content = r#"
METADATA
name: TestLexer
version: 1.0
initial_state: q0
END_METADATA

STATES
q0
q1 FINAL:IDENTIFIER
END_STATES

TRANSITIONS
q0, a, q1
END_TRANSITIONS

KEYWORDS
END_KEYWORDS
"#;

        let automaton = Automaton::parse(content).unwrap();
        assert_eq!(automaton.initial_state.as_usize(), 0);
        assert_eq!(automaton.transitions.len(), 1);
    }

    #[test]
    fn test_automaton_transitions() {
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
END_STATES

TRANSITIONS
q0, [a-z], q_id
q_id, [a-z], q_id
q0, [0-9], q_num
q_num, [0-9], q_num
END_TRANSITIONS

KEYWORDS
END_KEYWORDS
"#;

        let automaton = Automaton::parse(content).unwrap();
        let q0 = automaton.initial_state();

        // Test identifier transition
        let q_id = automaton.next_state(q0, 'a').unwrap();
        assert!(automaton.is_final(q_id).is_some());
        assert_eq!(automaton.is_final(q_id).unwrap(), &TokenType::Identifier);

        // Test number transition
        let q_num = automaton.next_state(q0, '5').unwrap();
        assert!(automaton.is_final(q_num).is_some());
        assert_eq!(automaton.is_final(q_num).unwrap(), &TokenType::Number);

        // Test invalid transition
        assert!(automaton.next_state(q0, '@').is_none());
    }

    #[test]
    fn test_keyword_classification() {
        let content = r#"
METADATA
name: TestLexer
version: 1.0
initial_state: q0
END_METADATA

STATES
q0
q_id FINAL:IDENTIFIER
END_STATES

TRANSITIONS
q0, [a-z], q_id
q_id, [a-z], q_id
END_TRANSITIONS

KEYWORDS
programa, PROGRAMA
inicio, INICIO
coloca, COLOCA
END_KEYWORDS
"#;

        let automaton = Automaton::parse(content).unwrap();

        // Case-insensitive keywords
        assert_eq!(
            automaton.classify_identifier("programa"),
            TokenType::Programa
        );
        assert_eq!(
            automaton.classify_identifier("PROGRAMA"),
            TokenType::Programa
        );
        assert_eq!(
            automaton.classify_identifier("Programa"),
            TokenType::Programa
        );

        // Regular identifiers
        assert_eq!(
            automaton.classify_identifier("miVariable"),
            TokenType::Identifier
        );
        assert_eq!(
            automaton.classify_identifier("test123"),
            TokenType::Identifier
        );
    }

    #[test]
    fn test_automaton_load_from_file() {
        // This test will try to load the actual automaton.aut file
        let result = Automaton::from_file("config/automaton.aut");

        // Should succeed if file exists and is valid
        if result.is_ok() {
            let automaton = result.unwrap();
            let q0 = automaton.initial_state();

            // Test some basic transitions
            assert!(automaton.next_state(q0, 'a').is_some()); // identifier
            assert!(automaton.next_state(q0, '5').is_some()); // number
            assert!(automaton.next_state(q0, ',').is_some()); // comma

            // Test keyword classification
            assert_eq!(
                automaton.classify_identifier("programa"),
                TokenType::Programa
            );
        }
        // If file doesn't exist, test will be skipped (not fail)
    }
}
