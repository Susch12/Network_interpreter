# Estructuras de Datos Detalladas

## 1. Módulo Lexer

### 1.1 Automaton
```rust
pub struct Automaton {
    /// Estado inicial del autómata
    initial_state: StateId,
    
    /// Mapa de transiciones: (estado_actual, carácter) → estado_siguiente
    transitions: HashMap<(StateId, char), StateId>,
    
    /// Transiciones por rango: (estado, inicio, fin) → estado_siguiente
    range_transitions: Vec<(StateId, char, char, StateId)>,
    
    /// Estados finales: estado → tipo de token
    final_states: HashMap<StateId, TokenType>,
    
    /// Palabras reservadas: lexema → tipo de token
    keywords: HashMap<String, TokenType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateId(usize);

impl Automaton {
    /// Carga autómata desde archivo .aut
    pub fn from_file(path: &str) -> Result<Self, String>;
    
    /// Obtiene el siguiente estado dada una transición
    pub fn next_state(&self, current: StateId, ch: char) -> Option<StateId>;
    
    /// Verifica si un estado es final
    pub fn is_final(&self, state: StateId) -> Option<&TokenType>;
    
    /// Clasifica un identificador (keyword o identifier)
    pub fn classify_identifier(&self, lexeme: &str) -> TokenType;
}
```

**Complejidad**:
- `next_state`: O(1) para transiciones exactas, O(n) para rangos
- `is_final`: O(1)
- `classify_identifier`: O(1)

---

### 1.2 Scanner
```rust
pub struct Scanner<'a> {
    /// Referencia al autómata (compartido)
    automaton: &'a Automaton,
    
    /// Código fuente como slice
    source: &'a str,
    
    /// Caracteres del source (para indexado eficiente)
    chars: Vec<char>,
    
    /// Posición actual en chars
    position: usize,
    
    /// Línea actual (para mensajes de error)
    line: usize,
    
    /// Columna actual (para mensajes de error)
    column: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(automaton: &'a Automaton, source: &'a str) -> Self;
    
    /// Escanea todos los tokens del source
    pub fn scan_all(&mut self) -> Result<Vec<Token>, LexicalError>;
    
    /// Obtiene el siguiente token
    fn next_token(&mut self) -> Result<Token, LexicalError>;
    
    /// Avanza la posición y actualiza línea/columna
    fn advance(&mut self) -> Option<char>;
    
    /// Mira el carácter actual sin avanzar
    fn peek(&self) -> Option<char>;
    
    /// Salta espacios en blanco
    fn skip_whitespace(&mut self);
}
```

**Uso de Memoria**:
- Automaton: compartido (referencia)
- chars: ~4 bytes por carácter
- state: mínimo

---

## 2. Módulo Parser

### 2.1 LL1Table
```rust
pub struct LL1Table {
    /// Símbolo inicial de la gramática
    start_symbol: NonTerminal,
    
    /// Conjuntos terminales
    terminals: HashSet<Terminal>,
    
    /// Conjuntos no-terminales
    nonterminals: HashSet<NonTerminal>,
    
    /// Tabla de análisis: M[No-terminal, Terminal] → Producción
    table: HashMap<(NonTerminal, Terminal), Production>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonTerminal(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Terminal(TokenType);

#[derive(Debug, Clone)]
pub struct Production {
    /// Lado izquierdo: no-terminal que se expande
    pub lhs: NonTerminal,
    
    /// Lado derecho: secuencia de símbolos (EPSILON si vacía)
    pub rhs: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

impl LL1Table {
    pub fn from_file(path: &str) -> Result<Self, String>;
    
    /// Consulta la tabla M[A, a]
    pub fn lookup(&self, nt: &NonTerminal, t: &Terminal) 
        -> Option<&Production>;
}
```

**Complejidad**:
- `lookup`: O(1) promedio

---

### 2.2 PredictiveParser
```rust
pub struct PredictiveParser<'a> {
    /// Tabla LL(1) (compartida)
    table: &'a LL1Table,
    
    /// Pila de parsing
    stack: Vec<Symbol>,
    
    /// Tokens de entrada
    tokens: Vec<Token>,
    
    /// Posición actual en tokens
    position: usize,
    
    /// AST builder (construye AST durante parsing)
    ast_builder: ASTBuilder,
}

impl<'a> PredictiveParser<'a> {
    pub fn new(table: &'a LL1Table, tokens: Vec<Token>) -> Self;
    
    /// Parsea y construye el AST
    pub fn parse(&mut self) -> Result<Program, SyntaxError>;
    
    /// Ejecuta un paso del algoritmo de parsing
    fn step(&mut self) -> Result<(), SyntaxError>;
    
    /// Hace match de un terminal
    fn match_terminal(&mut self, expected: &Terminal) 
        -> Result<(), SyntaxError>;
    
    /// Expande un no-terminal usando la tabla
    fn expand_nonterminal(&mut self, nt: &NonTerminal) 
        -> Result<(), SyntaxError>;
}
```

**Algoritmo**:
```
1. Push(EOF, Start_Symbol)
2. Mientras stack no vacía:
   a. Top = stack.top()
   b. Current = tokens[position]
   
   c. Si Top es Terminal:
      - Si Top == Current: pop y advance
      - Sino: ERROR
   
   d. Si Top es No-Terminal:
      - Prod = table[Top, Current]
      - Si Prod existe:
        * pop Top
        * push reverse(Prod.rhs)
      - Sino: ERROR
```

---

## 3. Módulo AST

### 3.1 Nodos Principales
```rust
/// Raíz del AST
pub struct Program {
    pub nombre: String,
    pub definiciones: Definitions,
    pub modulos: Vec<Modulo>,
    pub sentencias: Vec<Statement>,
    pub location: Location,
}

/// Definiciones de objetos
pub struct Definitions {
    pub maquinas: Vec<MaquinaDecl>,
    pub concentradores: Vec<ConcentradorDecl>,
    pub coaxiales: Vec<CoaxialDecl>,
}

/// Sentencias del lenguaje
pub enum Statement {
    Coloca {
        objeto: String,
        x: Expr,
        y: Expr,
        location: Location,
    },
    
    ColocaCoaxial {
        coaxial: String,
        x: Expr,
        y: Expr,
        direccion: Direccion,
        location: Location,
    },
    
    Si {
        condicion: Expr,
        entonces: Vec<Statement>,
        sino: Option<Vec<Statement>>,
        location: Location,
    },
    
    // ... 7 tipos más
}

/// Expresiones
pub enum Expr {
    Numero(i32),
    Cadena(String),
    Identificador(String),
    
    AccesoCampo {
        objeto: String,
        campo: String,
    },
    
    Relacional {
        izq: Box<Expr>,
        op: OpRelacional,
        der: Box<Expr>,
    },
    
    Logico {
        izq: Box<Expr>,
        op: OpLogico,
        der: Box<Expr>,
    },
    
    Not(Box<Expr>),
}

/// Ubicación en el código fuente
#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}
```

**Tamaño aproximado**:
- Program: ~200 bytes
- Statement: ~100 bytes
- Expr: ~50 bytes

---

## 4. Módulo Semantic

### 4.1 SymbolTable
```rust
pub struct SymbolTable {
    /// Tabla de máquinas
    pub maquinas: HashMap<String, MaquinaSymbol>,
    
    /// Tabla de concentradores
    pub concentradores: HashMap<String, ConcentradorSymbol>,
    
    /// Tabla de coaxiales
    pub coaxiales: HashMap<String, CoaxialSymbol>,
    
    /// Tabla de módulos
    pub modulos: HashMap<String, ModuloSymbol>,
}

pub struct MaquinaSymbol {
    pub nombre: String,
    pub presente: bool,      // Si fue colocada
    pub tipo: Type,          // Type::Maquina
    pub location: Location,
}

pub struct ConcentradorSymbol {
    pub nombre: String,
    pub puertos: i32,
    pub tiene_coaxial: bool,
    pub disponibles: i32,
    pub presente: bool,
    pub tipo: Type,
    pub location: Location,
}

pub struct CoaxialSymbol {
    pub nombre: String,
    pub longitud: i32,
    pub completo: bool,
    pub num_maquinas: usize,
    pub maquinas: Vec<String>,
    pub posiciones: Vec<i32>,
    pub presente: bool,
    pub tipo: Type,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    String,
    Bool,
    Maquina,
    Concentrador,
    Coaxial,
}
```

---

## 5. Módulo Interpreter

### 5.1 Environment
```rust
pub struct Environment {
    /// Estado runtime de máquinas
    pub maquinas: HashMap<String, RuntimeMaquina>,
    
    /// Estado runtime de concentradores
    pub concentradores: HashMap<String, RuntimeConcentrador>,
    
    /// Estado runtime de coaxiales
    pub coaxiales: HashMap<String, RuntimeCoaxial>,
    
    /// Output del programa (escribe)
    pub output: Vec<String>,
}

pub struct RuntimeMaquina {
    pub nombre: String,
    pub x: i32,
    pub y: i32,
    pub colocada: bool,
    pub conectada_a: Option<Conexion>,
}

pub struct RuntimeConcentrador {
    pub nombre: String,
    pub puertos: i32,
    pub tiene_coaxial: bool,
    pub x: i32,
    pub y: i32,
    pub colocado: bool,
    pub puertos_ocupados: Vec<bool>,
    pub disponibles: i32,
}

pub struct RuntimeCoaxial {
    pub nombre: String,
    pub longitud: i32,
    pub x: i32,
    pub y: i32,
    pub direccion: String,
    pub colocado: bool,
    pub maquinas: Vec<(String, i32)>,  // (nombre, posición)
    pub completo: bool,
}

pub enum Conexion {
    Puerto {
        concentrador: String,
        puerto: i32,
    },
    Coaxial {
        coaxial: String,
        posicion: i32,
    },
}
```

---

## 6. Complejidad Computacional

| Operación                  | Complejidad | Notas                    |
|----------------------------|-------------|--------------------------|
| Carga de autómata          | O(n)        | n = tamaño archivo .aut  |
| Carga de tabla LL(1)       | O(m)        | m = tamaño archivo .ll1  |
| Escaneo léxico             | O(n)        | n = caracteres source    |
| Parsing                    | O(n)        | n = número de tokens     |
| Análisis semántico         | O(n)        | n = nodos AST            |
| Interpretación             | O(n)        | n = sentencias           |
| Lookup en SymbolTable      | O(1)        | HashMap                  |
| Lookup en tabla LL(1)      | O(1)        | HashMap                  |
| Next state en autómata     | O(1)        | HashMap (transiciones)   |

---

**Fin de Estructuras**
```

---

## 📊 Resumen de Fase 1.3

### ✅ Checklist de Completitud
```
[✓] 1. Diagrama de arquitectura de alto nivel
[✓] 2. Estructura de módulos definida
[✓] 3. Diagrama de flujo de datos
[✓] 4. Estructuras de datos especificadas
[✓] 5. Patrones de diseño identificados
[✓] 6. Estrategias de optimización
[✓] 7. Gestión de memoria planificada
[✓] 8. Métricas de rendimiento definidas
[✓] 9. Pipeline de procesamiento documentado
[✓] 10. Transformaciones de datos explicadas
```

### 📈 Estadísticas

**Archivos creados**: 3  
**Diagramas**: 7  
**Estructuras especificadas**: 15+  
**Patrones de diseño**: 5  
**Optimizaciones planificadas**: 4  

---

## 🎯 Conclusión de Fase 1.3

**Estado**: ✅ COMPLETADO

**Tiempo invertido**: ~1 hora

**Entregables**:
1. ✅ Arquitectura completa del sistema
2. ✅ Flujo de datos documentado
3. ✅ Estructuras de datos detalladas
4. ✅ Diagramas de secuencia
5. ✅ Métricas de rendimiento

**Próxima fase**: FASE 2 - Implementación del Lexer

---

## 🚀 Estado General de la Fase 1
```
FASE 1: PREPARACIÓN Y DISEÑO
├── 1.1 Análisis de Gramática LL(1)     ✅ COMPLETADO
├── 1.2 Diseño del Formato Autómata     ✅ COMPLETADO
└── 1.3 Diseño de Arquitectura          ✅ COMPLETADO

Tiempo total invertido: ~3 horas
Próximo paso: FASE 2.1 - Módulo de Autómata Base
