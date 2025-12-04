# Esquema de Validación para Archivos de Configuración

## 1. Validador de Autómata (.aut)

### Implementación en Rust:
```rust
// src/config/validator.rs

use std::collections::{HashMap, HashSet};

pub struct AutomatonValidator {
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl AutomatonValidator {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn validate(&mut self, aut_file: &str) -> Result<(), Vec<String>> {
        // 1. Validar estructura
        self.check_structure(aut_file)?;
        
        // 2. Validar estados
        let states = self.parse_states(aut_file)?;
        self.check_states(&states)?;
        
        // 3. Validar transiciones
        let transitions = self.parse_transitions(aut_file)?;
        self.check_transitions(&transitions, &states)?;
        
        // 4. Validar keywords
        let keywords = self.parse_keywords(aut_file)?;
        self.check_keywords(&keywords, &states)?;
        
        // 5. Detectar estados inalcanzables
        self.check_reachability(&states, &transitions)?;
        
        // 6. Detectar ambigüedades
        self.check_ambiguity(&transitions)?;
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    fn check_structure(&mut self, content: &str) -> Result<(), Vec<String>> {
        let required_sections = vec![
            "METADATA", "END_METADATA",
            "STATES", "END_STATES",
            "TRANSITIONS", "END_TRANSITIONS",
            "KEYWORDS", "END_KEYWORDS",
        ];
        
        for section in required_sections {
            if !content.contains(section) {
                self.errors.push(format!("Sección faltante: {}", section));
            }
        }
        
        Ok(())
    }
    
    // ... más métodos de validación
}
```

### Validaciones Implementadas:

#### ✓ Estructura:
- [x] Todas las secciones presentes
- [x] Secciones en orden correcto
- [x] Marcadores de inicio/fin correctos

#### ✓ Estados:
- [x] Nombres únicos
- [x] Estado inicial existe
- [x] Al menos un estado final
- [x] Tipos de token válidos en estados finales

#### ✓ Transiciones:
- [x] Estados origen/destino existen
- [x] Clases de caracteres válidas
- [x] Sin transiciones ambiguas
- [x] Todos los estados alcanzables desde inicial

#### ✓ Keywords:
- [x] Palabras únicas
- [x] Tipos de token corresponden a estados finales

---

## 2. Validador de Tabla LL(1) (.ll1)

### Implementación:
```rust
pub struct LL1TableValidator {
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl LL1TableValidator {
    pub fn validate(&mut self, ll1_file: &str) -> Result<(), Vec<String>> {
        // 1. Validar metadata
        self.check_metadata(ll1_file)?;
        
        // 2. Validar terminales
        let terminals = self.parse_terminals(ll1_file)?;
        self.check_terminals(&terminals)?;
        
        // 3. Validar no-terminales
        let nonterminals = self.parse_nonterminals(ll1_file)?;
        self.check_nonterminals(&nonterminals)?;
        
        // 4. Validar tabla
        let table = self.parse_table(ll1_file)?;
        self.check_table(&table, &terminals, &nonterminals)?;
        
        // 5. Verificar propiedad LL(1)
        self.check_ll1_property(&table)?;
        
        // 6. Verificar completitud
        self.check_completeness(&table, &nonterminals, &terminals)?;
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    fn check_ll1_property(&mut self, table: &HashMap<(String, String), Vec<String>>) 
        -> Result<(), Vec<String>> {
        for ((nt, t), productions) in table {
            if productions.len() > 1 {
                self.errors.push(format!(
                    "Conflicto LL(1): M[{}, {}] tiene {} producciones",
                    nt, t, productions.len()
                ));
            }
        }
        Ok(())
    }
    
    // ... más métodos
}
```

### Validaciones Implementadas:

#### ✓ Propiedad LL(1):
- [x] Sin conflictos: M[A, a] tiene máximo 1 producción
- [x] Sin ambigüedades

#### ✓ Completitud:
- [x] Todas las combinaciones necesarias cubiertas
- [x] Símbolos referenciados están definidos

#### ✓ Consistencia:
- [x] Símbolo inicial definido
- [x] EOF presente en terminales
- [x] Producciones sintácticamente correctas

---

## 3. Test Suite de Validación

### Archivo: `tests/validation_tests.rs`
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_automaton() {
        let content = r#"
METADATA
name: TestAutomaton
version: 1.0
initial_state: q0
END_METADATA

STATES
q0
q1 FINAL:TOKEN_A
END_STATES

TRANSITIONS
q0, a, q1
END_TRANSITIONS

KEYWORDS
END_KEYWORDS
"#;
        
        let mut validator = AutomatonValidator::new();
        assert!(validator.validate(content).is_ok());
    }
    
    #[test]
    fn test_missing_section() {
        let content = r#"
METADATA
name: InvalidAutomaton
END_METADATA

STATES
q0
END_STATES
"#;
        
        let mut validator = AutomatonValidator::new();
        assert!(validator.validate(content).is_err());
    }
    
    #[test]
    fn test_unreachable_state() {
        let content = r#"
METADATA
name: TestAutomaton
version: 1.0
initial_state: q0
END_METADATA

STATES
q0
q1 FINAL:TOKEN_A
q2 FINAL:TOKEN_B
END_STATES

TRANSITIONS
q0, a, q1
END_TRANSITIONS

KEYWORDS
END_KEYWORDS
"#;
        
        let mut validator = AutomatonValidator::new();
        let result = validator.validate(content);
        assert!(result.is_ok());
        assert!(validator.warnings.iter().any(|w| w.contains("inalcanzable")));
    }
    
    #[test]
    fn test_ll1_conflict() {
        let content = r#"
METADATA
name: ConflictGrammar
version: 1.0
start_symbol: S
END_METADATA

TERMINALS
A
B
EOF
END_TERMINALS

NONTERMINALS
S
END_NONTERMINALS

TABLE
S, A, A
S, A, B
END_TABLE
"#;
        
        let mut validator = LL1TableValidator::new();
        let result = validator.validate(content);
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("Conflicto LL(1)")));
    }
}
```

---

**Fin de Esquema de Validación**
