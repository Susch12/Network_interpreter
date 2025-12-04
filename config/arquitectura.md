# Arquitectura del Intérprete de Red

**Versión**: 2.0  
**Tipo**: Sistema basado en autómatas y análisis LL(1)  
**Fecha**: 2024  

---

## 1. Visión General

### 1.1 Diagrama de Alto Nivel
```
┌─────────────────────────────────────────────────────────────┐
│                    ENTRADA DEL USUARIO                       │
│                     (archivo .net)                           │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  FASE 1: ANÁLISIS LÉXICO                     │
│  ┌────────────┐         ┌──────────────┐                    │
│  │ Automaton  │────────▶│   Scanner    │──────▶ Tokens      │
│  │ (lexer.aut)│         │              │                    │
│  └────────────┘         └──────────────┘                    │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                FASE 2: ANÁLISIS SINTÁCTICO                   │
│  ┌────────────┐         ┌──────────────┐                    │
│  │ LL1 Table  │────────▶│Predictive    │──────▶ AST         │
│  │(parser.ll1)│         │Parser        │                    │
│  └────────────┘         └──────────────┘                    │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│               FASE 3: ANÁLISIS SEMÁNTICO                     │
│  ┌──────────────────────────────────────────┐               │
│  │     Semantic Analyzer + Symbol Table     │──────▶ AST    │
│  │  - Validación de tipos                   │      Anotado  │
│  │  - Verificación de reglas Ethernet       │               │
│  │  - Construcción de tabla de símbolos     │               │
│  └──────────────────────────────────────────┘               │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  FASE 4: INTERPRETACIÓN                      │
│  ┌──────────────────────────────────────────┐               │
│  │           Interpreter Engine             │               │
│  │  - Ejecución de sentencias               │──────▶ Estado │
│  │  - Gestión de memoria runtime            │       Runtime │
│  │  - Evaluación de expresiones             │               │
│  └──────────────────────────────────────────┘               │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                 FASE 5: VISUALIZACIÓN                        │
│  ┌──────────────────────────────────────────┐               │
│  │          Visualizer (eframe/egui)        │               │
│  │  - Renderizado de topología              │──────▶ GUI    │
│  │  - Interacción con usuario               │               │
│  └──────────────────────────────────────────┘               │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Arquitectura de Componentes

### 2.1 Módulos Principales
```
network_interpreter/
├── src/
│   ├── main.rs                    # Punto de entrada
│   │
│   ├── config/                    # Configuración
│   │   ├── mod.rs
│   │   ├── loader.rs              # Carga de archivos .aut y .ll1
│   │   └── validator.rs           # Validación de configuración
│   │
│   ├── lexer/                     # Análisis Léxico
│   │   ├── mod.rs
│   │   ├── automaton.rs           # Motor DFA
│   │   ├── token.rs               # Definición de tokens
│   │   ├── scanner.rs             # Scanner principal
│   │   └── error.rs               # Errores léxicos
│   │
│   ├── parser/                    # Análisis Sintáctico
│   │   ├── mod.rs
│   │   ├── ll1_table.rs           # Tabla LL(1)
│   │   ├── predictive.rs          # Parser predictivo
│   │   ├── stack.rs               # Pila de parsing
│   │   └── error.rs               # Errores sintácticos
│   │
│   ├── ast/                       # Árbol de Sintaxis Abstracta
│   │   ├── mod.rs
│   │   ├── nodes.rs               # Nodos del AST
│   │   ├── visitor.rs             # Patrón Visitor
│   │   └── printer.rs             # Pretty-printer
│   │
│   ├── semantic/                  # Análisis Semántico
│   │   ├── mod.rs
│   │   ├── analyzer.rs            # Analizador semántico
│   │   ├── symbol_table.rs        # Tabla de símbolos
│   │   ├── type_checker.rs        # Verificador de tipos
│   │   └── error.rs               # Errores semánticos
│   │
│   ├── interpreter/               # Intérprete
│   │   ├── mod.rs
│   │   ├── engine.rs              # Motor de ejecución
│   │   ├── environment.rs         # Entorno runtime
│   │   ├── evaluator.rs           # Evaluador de expresiones
│   │   └── error.rs               # Errores runtime
│   │
│   ├── visualizer/                # Visualización
│   │   ├── mod.rs
│   │   ├── app.rs                 # Aplicación egui
│   │   ├── renderer.rs            # Renderizador
│   │   └── components.rs          # Componentes visuales
│   │
│   └── utils/                     # Utilidades
│       ├── mod.rs
│       ├── error_reporter.rs      # Reporte de errores
│       └── diagnostics.rs         # Sistema de diagnóstico
│
├── config/                        # Archivos de configuración
│   ├── lexer.aut                  # Autómata del lexer
│   └── parser.ll1                 # Tabla LL(1)
│
├── tests/                         # Tests
│   ├── lexer/
│   ├── parser/
│   ├── semantic/
│   ├── interpreter/
│   └── integration/
│
└── docs/                          # Documentación
    ├── gramatica.txt
    ├── first_follow.txt
    ├── arquitectura.md
    └── ...
```

---

## 3. Flujo de Datos

### 3.1 Pipeline de Procesamiento
```
┌──────────────┐
│ Archivo .net │
└──────┬───────┘
       │
       ▼
┌─────────────────────────────────────────────┐
│ 1. CARGA DE CONFIGURACIÓN (Una vez)        │
│                                              │
│  Automaton::load("config/lexer.aut")        │
│  LL1Table::load("config/parser.ll1")        │
│                                              │
│  [Cached en memoria estática]               │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│ 2. ANÁLISIS LÉXICO                          │
│                                              │
│  Input: String (código fuente)              │
│  Process:                                    │
│    - Scanner itera caracteres                │
│    - Usa DFA para reconocer tokens          │
│    - Clasifica keywords vs identifiers       │
│  Output: Vec<Token>                         │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│ 3. ANÁLISIS SINTÁCTICO                      │
│                                              │
│  Input: Vec<Token>                          │
│  Process:                                    │
│    - Parser predictivo con pila             │
│    - Consulta tabla M[No-terminal, Token]   │
│    - Construye AST durante parsing          │
│  Output: AST (Program)                      │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│ 4. ANÁLISIS SEMÁNTICO                       │
│                                              │
│  Input: AST                                 │
│  Process:                                    │
│    - Construye tabla de símbolos            │
│    - Valida tipos                           │
│    - Verifica reglas Ethernet               │
│    - Anota AST con información semántica    │
│  Output: AST anotado + SymbolTable          │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│ 5. INTERPRETACIÓN                           │
│                                              │
│  Input: AST anotado + SymbolTable           │
│  Process:                                    │
│    - Inicializa entorno runtime             │
│    - Ejecuta sentencias                     │
│    - Actualiza estado de la red             │
│  Output: Environment (estado final)         │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│ 6. VISUALIZACIÓN (Opcional)                 │
│                                              │
│  Input: Environment                         │
│  Process:                                    │
│    - Renderiza topología                    │
│    - Permite interacción                    │
│  Output: GUI interactiva                    │
└─────────────────────────────────────────────┘
```

---

## 4. Estructuras de Datos Clave

### 4.1 Token
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Palabras reservadas
    Programa,
    Define,
    // ... (45 tipos total)
    
    // Literales
    Identifier,
    Number,
    String,
    
    // Especiales
    Eof,
}
```

### 4.2 AST Node
```rust
#[derive(Debug, Clone)]
pub struct Program {
    pub nombre: String,
    pub definiciones: Definitions,
    pub modulos: Vec<Modulo>,
    pub sentencias: Vec<Statement>,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Coloca { objeto: String, x: Expr, y: Expr, location: Location },
    ColocaCoaxial { /* ... */ },
    Si { condicion: Expr, entonces: Vec<Statement>, sino: Option<Vec<Statement>>, /* ... */ },
    // ... (10 tipos de sentencias)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Numero(i32),
    Cadena(String),
    Identificador(String),
    AccesoCampo { objeto: String, campo: String },
    Relacional { izq: Box<Expr>, op: OpRelacional, der: Box<Expr> },
    Logico { izq: Box<Expr>, op: OpLogico, der: Box<Expr> },
    Not(Box<Expr>),
}
```

### 4.3 Symbol Table
```rust
#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub maquinas: HashMap<String, MaquinaSymbol>,
    pub concentradores: HashMap<String, ConcentradorSymbol>,
    pub coaxiales: HashMap<String, CoaxialSymbol>,
    pub modulos: HashMap<String, ModuloSymbol>,
}

#[derive(Debug, Clone)]
pub struct MaquinaSymbol {
    pub nombre: String,
    pub presente: bool,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct ConcentradorSymbol {
    pub nombre: String,
    pub puertos: i32,
    pub tiene_coaxial: bool,
    pub disponibles: i32,
    pub location: Location,
}
```

### 4.4 Runtime Environment
```rust
#[derive(Debug, Clone)]
pub struct Environment {
    pub maquinas: HashMap<String, RuntimeMaquina>,
    pub concentradores: HashMap<String, RuntimeConcentrador>,
    pub coaxiales: HashMap<String, RuntimeCoaxial>,
    pub output: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RuntimeMaquina {
    pub nombre: String,
    pub x: i32,
    pub y: i32,
    pub colocada: bool,
    pub conectada_a: Option<Conexion>,
}

#[derive(Debug, Clone)]
pub enum Conexion {
    Puerto { concentrador: String, puerto: i32 },
    Coaxial { coaxial: String, posicion: i32 },
}
```

---

## 5. Patrones de Diseño

### 5.1 Singleton (Lazy Loading)

**Propósito**: Cargar configuración una sola vez
```rust
use once_cell::sync::Lazy;

static AUTOMATON: Lazy<Automaton> = Lazy::new(|| {
    Automaton::from_file("config/lexer.aut")
        .expect("Error cargando autómata")
});

static LL1_TABLE: Lazy<LL1Table> = Lazy::new(|| {
    LL1Table::from_file("config/parser.ll1")
        .expect("Error cargando tabla LL(1)")
});

// Uso:
fn lex(source: &str) -> Vec<Token> {
    let scanner = Scanner::new(&AUTOMATON, source);
    scanner.scan_all()
}
```

**Ventajas**:
- ✅ Carga única en toda la ejecución
- ✅ Thread-safe
- ✅ Sin overhead de re-lectura

### 5.2 Visitor Pattern

**Propósito**: Recorrer AST sin modificar nodos
```rust
pub trait ASTVisitor {
    type Result;
    
    fn visit_program(&mut self, program: &Program) -> Self::Result;
    fn visit_statement(&mut self, stmt: &Statement) -> Self::Result;
    fn visit_expression(&mut self, expr: &Expr) -> Self::Result;
}

// Implementaciones:
// - SemanticAnalyzer (validación)
// - Interpreter (ejecución)
// - ASTPrinter (debug)
```

### 5.3 Builder Pattern

**Propósito**: Construir AST de forma incremental
```rust
pub struct ProgramBuilder {
    nombre: Option<String>,
    definiciones: Definitions,
    modulos: Vec<Modulo>,
    sentencias: Vec<Statement>,
}

impl ProgramBuilder {
    pub fn new() -> Self { /* ... */ }
    
    pub fn nombre(mut self, nombre: String) -> Self {
        self.nombre = Some(nombre);
        self
    }
    
    pub fn add_modulo(mut self, modulo: Modulo) -> Self {
        self.modulos.push(modulo);
        self
    }
    
    pub fn build(self) -> Result<Program, String> { /* ... */ }
}
```

### 5.4 Strategy Pattern

**Propósito**: Diferentes estrategias de evaluación
```rust
pub trait ExprEvaluator {
    fn eval(&self, expr: &Expr, env: &Environment) -> Result<Value, String>;
}

pub struct StandardEvaluator;
pub struct OptimizedEvaluator;

impl ExprEvaluator for StandardEvaluator {
    fn eval(&self, expr: &Expr, env: &Environment) -> Result<Value, String> {
        // Evaluación estándar
    }
}
```

### 5.5 Error Handling Pattern

**Propósito**: Manejo consistente de errores
```rust
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub location: Location,
    pub message: String,
    pub help: Option<String>,
}

pub enum DiagnosticKind {
    LexicalError,
    SyntaxError,
    SemanticError,
    RuntimeError,
}

pub type Result<T> = std::result::Result<T, Vec<Diagnostic>>;
```

---

## 6. Optimizaciones

### 6.1 Caché de Configuración
```rust
// ❌ Antes (carga en cada ejecución)
fn main() {
    let automaton = Automaton::from_file("config/lexer.aut")?;
    let ll1_table = LL1Table::from_file("config/parser.ll1")?;
    // ...
}

// ✅ Después (carga única)
static AUTOMATON: Lazy<Automaton> = Lazy::new(|| {
    Automaton::from_file("config/lexer.aut").unwrap()
});

fn main() {
    let scanner = Scanner::new(&AUTOMATON, source);
    // ...
}
```

**Ganancia**: ~50ms por ejecución

### 6.2 String Interning
```rust
use string_interner::StringInterner;

pub struct InternedStrings {
    interner: StringInterner,
}

impl InternedStrings {
    pub fn intern(&mut self, s: &str) -> Symbol {
        self.interner.get_or_intern(s)
    }
}
```

**Ganancia**: Reduce allocations en ~30%

### 6.3 Arena Allocation para AST
```rust
use typed_arena::Arena;

pub struct Parser<'a> {
    arena: &'a Arena<ASTNode>,
    // ...
}

impl<'a> Parser<'a> {
    fn parse_expr(&mut self) -> &'a Expr {
        self.arena.alloc(Expr::Numero(42))
    }
}
```

**Ganancia**: ~2x velocidad en construcción de AST

### 6.4 Tabla de Símbolos con HashMap
```rust
// ✅ Usar HashMap (O(1) promedio)
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

// ❌ Evitar Vec<(String, Symbol)> (O(n))
```

---

## 7. Gestión de Memoria

### 7.1 Ownership en Rust
```
┌─────────────────┐
│   main.rs       │ ← Posee source: String
└────────┬────────┘
         │ &str
         ▼
┌─────────────────┐
│   Scanner       │ ← Borrow inmutable
└────────┬────────┘
         │ Vec<Token>
         ▼
┌─────────────────┐
│   Parser        │ ← Posee tokens
└────────┬────────┘
         │ Program
         ▼
┌─────────────────┐
│   Interpreter   │ ← Posee AST
└─────────────────┘
```

### 7.2 Lifetimes
```rust
// Scanner no posee el source, solo lo referencia
pub struct Scanner<'a> {
    automaton: &'a Automaton,
    source: &'a str,
    position: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(automaton: &'a Automaton, source: &'a str) -> Self {
        Self { automaton, source, position: 0 }
    }
}
```

---

## 8. Concurrencia

### 8.1 Paralelización (Futuro)
```rust
use rayon::prelude::*;

// Análisis semántico paralelo de módulos
pub fn analyze_modules_parallel(modules: &[Modulo]) -> Vec<Result<()>> {
    modules
        .par_iter()
        .map(|modulo| analyze_module(modulo))
        .collect()
}
```

**Nota**: Implementación futura, no crítica para v1.0

---

## 9. Testing Strategy

### 9.1 Pirámide de Tests
```
         ┌───────┐
         │  E2E  │  10%  - Integration tests
         ├───────┤
         │ Integ │  20%  - Component integration
         ├───────┤
         │ Unit  │  70%  - Unit tests
         └───────┘
```

### 9.2 Coverage por Módulo

| Módulo          | Target Coverage |
|-----------------|-----------------|
| Lexer           | > 95%           |
| Parser          | > 90%           |
| Semantic        | > 85%           |
| Interpreter     | > 80%           |
| Visualizer      | > 60%           |

---

## 10. Métricas de Rendimiento

### 10.1 Objetivos

| Fase              | Target (archivo 1000 líneas) |
|-------------------|------------------------------|
| Carga config      | < 10ms (primera vez)         |
| Análisis léxico   | < 50ms                       |
| Análisis sintáctico| < 100ms                     |
| Análisis semántico| < 150ms                      |
| Interpretación    | < 200ms                      |
| **Total**         | **< 500ms**                  |

### 10.2 Uso de Memoria

| Componente        | Target                       |
|-------------------|------------------------------|
| Autómata          | < 100 KB                     |
| Tabla LL(1)       | < 500 KB                     |
| Tokens            | ~10 bytes/token              |
| AST               | ~50 bytes/nodo               |
| Symbol Table      | ~100 bytes/símbolo           |

---

**Fin de Arquitectura**
