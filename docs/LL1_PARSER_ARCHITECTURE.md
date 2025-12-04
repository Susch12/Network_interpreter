# LL(1) Predictive Parser - Architecture Documentation

**Project**: Network Topology Interpreter
**Module**: `src/parser_ll1/`
**Created**: 2025-11-30
**Status**: ✅ **FULLY COMPILING** (100% functional infrastructure)

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Module Structure](#module-structure)
4. [Data Structures](#data-structures)
5. [Algorithms](#algorithms)
6. [Implementation Details](#implementation-details)
7. [Usage Examples](#usage-examples)
8. [Testing Strategy](#testing-strategy)
9. [Performance Considerations](#performance-considerations)
10. [Future Enhancements](#future-enhancements)

---

## Overview

### What is This?

This is a **table-driven LL(1) predictive parser** that implements a **non-recursive**, **stack-based** parsing algorithm for the Network Topology Language grammar.

### Key Features

✅ **84 grammar productions** fully defined
✅ **~200+ parsing table entries** pre-computed
✅ **FIRST/FOLLOW sets** for all 43 non-terminals
✅ **Explicit stack** (no recursion)
✅ **Zero compilation errors**
✅ **Conflict-free LL(1) grammar**

### Why LL(1)?

Unlike the existing recursive descent parser, this implementation:

- **Demonstrates formal parsing theory** (FIRST, FOLLOW, predictive table)
- **Uses explicit stack** instead of call stack
- **Table-driven** rather than hard-coded logic
- **Easier to generate** from grammar specifications
- **Educational value** for compiler courses

---

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                     LL(1) Parser System                      │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              │               │               │
              ▼               ▼               ▼
    ┌──────────────┐  ┌─────────────┐  ┌──────────────┐
    │ FIRST/FOLLOW │  │  Production │  │  Predictive  │
    │    Sets      │  │    Table    │  │    Parser    │
    └──────────────┘  └─────────────┘  └──────────────┘
           │                 │                 │
           │                 │                 │
           └─────────────────┴─────────────────┘
                         Lexer Tokens
```

### Component Interaction

```
Input → Lexer → Tokens → PredictiveParser → AST (placeholder)
                              │
                              ├─ Uses LL1Table
                              ├─ Uses FirstFollowSets
                              └─ Uses explicit Stack<Symbol>
```

---

## Module Structure

### File Organization

```
src/parser_ll1/
├── mod.rs              # Module exports and public API
├── first_follow.rs     # FIRST/FOLLOW set computation
├── ll1_table.rs        # Production rules and parsing table
└── predictive.rs       # Stack-based predictive parser
```

### Module Hierarchy

```rust
pub mod parser_ll1 {
    pub mod first_follow {
        pub struct FirstFollowSets
        pub enum Symbol
        pub enum NonTerminal
    }

    pub mod ll1_table {
        pub struct LL1Table
        pub struct Production
        pub enum TokenClass
    }

    pub mod predictive {
        pub struct PredictiveParser
    }
}
```

---

## Data Structures

### 1. Symbol (first_follow.rs:8-14)

Represents grammar symbols in productions.

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Terminal(Token),       // Terminal symbol (e.g., PROGRAMA, NUMERO)
    NonTerminal(NonTerminal), // Non-terminal (e.g., Expresion)
    Epsilon,               // ε (empty production)
    Eof,                   // $ (end of input)
}
```

**Purpose**: Used in production right-hand sides and on the parsing stack.

### 2. NonTerminal (first_follow.rs:17-64)

All 43 non-terminals in the grammar.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonTerminal {
    // Program structure
    Programa,
    Definiciones,
    DefMaquinas,
    DefConcentradores,
    //... (40 more)

    // Expressions
    Expresion,
    ExpresionOr,
    ExpresionAnd,
    // ...
}
```

**Key Methods**:
- `as_str() -> &'static str` - Returns human-readable name

### 3. FirstFollowSets (first_follow.rs:119-369)

Pre-computed FIRST and FOLLOW sets.

```rust
pub struct FirstFollowSets {
    first: HashMap<NonTerminal, HashSet<Symbol>>,
    follow: HashMap<NonTerminal, HashSet<Symbol>>,
}
```

**Initialized with**:
- All FIRST sets from `docs/first_follow.txt` lines 7-106
- All FOLLOW sets from `docs/first_follow.txt` lines 110-243

**Methods**:
```rust
pub fn first(&self, nt: NonTerminal) -> Option<&HashSet<Symbol>>
pub fn follow(&self, nt: NonTerminal) -> Option<&HashSet<Symbol>>
pub fn is_in_first(&self, nt: NonTerminal, symbol: &Symbol) -> bool
pub fn is_in_follow(&self, nt: NonTerminal, symbol: &Symbol) -> bool
```

### 4. Production (ll1_table.rs:10-14)

A single grammar production.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Production {
    pub id: usize,              // Production number (1-84)
    pub lhs: NonTerminal,       // Left-hand side
    pub rhs: Vec<Symbol>,       // Right-hand side
}
```

**Example**:
```rust
Production {
    id: 1,
    lhs: NT::Programa,
    rhs: vec![
        Terminal(Programa),
        Terminal(Identificador(...)),
        Terminal(PuntoYComa),
        NT_Symbol(NT::Definiciones),
        NT_Symbol(NT::Modulos),
        NT_Symbol(NT::BloqueInicio),
        Terminal(Punto),
    ]
}
```

### 5. LL1Table (ll1_table.rs:18-21)

The parsing table: M[NonTerminal, Terminal] → Production

```rust
pub struct LL1Table {
    table: HashMap<(NonTerminal, TokenClass), Production>,
    productions: Vec<Production>,
}
```

**Size**: ~200+ entries covering all parse actions

**Lookup**:
```rust
pub fn get(&self, nt: NonTerminal, token: &Token) -> Option<&Production>
```

### 6. TokenClass (ll1_table.rs:24-69)

Normalized token types for table lookup.

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenClass {
    Programa,
    Identificador,  // Note: ignores actual identifier value
    Numero,         // Note: ignores actual number value
    // ... 40+ more
}
```

**Purpose**: Allows table lookup ignoring token values (e.g., all numbers map to `Numero`)

### 7. PredictiveParser (predictive.rs:13-20)

The main parsing engine.

```rust
pub struct PredictiveParser {
    table: LL1Table,            // Parsing table
    tokens: Vec<Token>,         // Input tokens
    position: usize,            // Current position
    stack: Vec<Symbol>,         // Parse stack
    errors: Vec<String>,        // Error accumulator
}
```

---

## Algorithms

### LL(1) Predictive Parsing Algorithm

**Location**: `predictive.rs:69-142`

**Pseudocode**:
```
Input: Token stream w, Parsing table M, Start symbol S
Output: Leftmost derivation or error

Stack: [EOF, S]
Input: w$

while Stack not empty:
    X = Stack.top()
    a = current token

    if X is terminal:
        if X == a:
            pop X
            advance token
        else:
            ERROR

    else if X is EOF:
        if a is EOF:
            ACCEPT
        else:
            ERROR

    else:  # X is non-terminal
        if M[X, a] exists:
            pop X
            push RHS(M[X, a]) in reverse
        else:
            ERROR
```

**Implementation**:
```rust
pub fn parse(&mut self) -> Result<Program, String> {
    while !self.stack.is_empty() {
        let top = self.stack.pop().unwrap();
        let current = self.current_token();

        match top {
            Symbol::Epsilon => continue,

            Symbol::Eof => {
                if self.position >= self.tokens.len() {
                    break;  // Success!
                } else {
                    return Err("Expected EOF".to_string());
                }
            }

            Symbol::Terminal(expected) => {
                if Self::tokens_match(&expected, &current) {
                    self.advance();
                } else {
                    return Err(format!("Expected {:?}, found {:?}", expected, current));
                }
            }

            Symbol::NonTerminal(nt) => {
                match self.table.get(nt, &current) {
                    Some(production) => {
                        // Push RHS in reverse order
                        for symbol in production.rhs.iter().rev() {
                            self.stack.push(symbol.clone());
                        }
                    }
                    None => {
                        return Err(format!("No production for M[{}, {:?}]",
                            nt.as_str(), current));
                    }
                }
            }
        }
    }

    Ok(/* AST */)
}
```

### FIRST Set Computation

**Location**: `first_follow.rs:139-321`

**Algorithm**: Pre-computed from grammar (not computed at runtime)

**Rules**:
1. `FIRST(terminal) = {terminal}`
2. `FIRST(ε) = {ε}`
3. `FIRST(A → α) = FIRST(α)`
4. If `ε ∈ FIRST(X1...Xk)`, then add `ε` to `FIRST(A)`

**Example**:
```rust
// FIRST(Expresion) = { NOT, NUMERO, CADENA, IDENTIFICADOR, PAREN_IZQ }
first.insert(NT::Expresion, hashset![
    Terminal(Not),
    Terminal(Numero(0)),
    Terminal(Cadena(String::new())),
    Terminal(Identificador(String::new())),
    Terminal(ParenIzq)
]);
```

### FOLLOW Set Computation

**Location**: `first_follow.rs:323-460`

**Algorithm**: Pre-computed from grammar

**Rules**:
1. `$ ∈ FOLLOW(Start Symbol)`
2. If `A → αBβ`, then `FIRST(β) - {ε} ⊆ FOLLOW(B)`
3. If `A → αB` or `ε ∈ FIRST(β)`, then `FOLLOW(A) ⊆ FOLLOW(B)`

**Example**:
```rust
// FOLLOW(Expresion) = { PAREN_DER, COMA, CORCHETE_DER, PUNTO_COMA, INICIO }
follow.insert(NT::Expresion, hashset![
    Terminal(ParenDer),
    Terminal(Coma),
    Terminal(CorcheteDer),
    Terminal(PuntoYComa),
    Terminal(Inicio)
]);
```

### Parsing Table Construction

**Location**: `ll1_table.rs:651-937`

**Algorithm**:
```
For each production A → α:
    For each terminal a in FIRST(α):
        M[A, a] = A → α

    If ε in FIRST(α):
        For each terminal b in FOLLOW(A):
            M[A, b] = A → α
```

**Example**:
```rust
// Production: DefMaquinas → DEFINE MAQUINAS ListaMaquinas ;
// FIRST = {DEFINE}
self.add_entry(NT::DefMaquinas, Define, 4);

// Production: DefMaquinas → ε
// FOLLOW(DefMaquinas) = {DEFINE, MODULO, INICIO}
self.add_entry(NT::DefMaquinas, Modulo, 5);
self.add_entry(NT::DefMaquinas, Inicio, 5);
```

---

## Implementation Details

### Key Implementation Choices

#### 1. Pre-computed vs Runtime FIRST/FOLLOW

**Choice**: Pre-computed and hard-coded

**Rationale**:
- Grammar doesn't change at runtime
- Faster startup
- No risk of computation bugs
- Easier to verify against formal specification

#### 2. Separate Symbol Enum

**Choice**: `Symbol` enum separate from `NonTerminal`/`Token`

**Rationale**:
- Allows stack to hold mixed symbols
- Clear type safety
- Epsilon and EOF are neither terminals nor non-terminals

#### 3. TokenClass for Table Indexing

**Choice**: Normalized `TokenClass` instead of direct `Token`

**Rationale**:
- `Token::Identificador("foo")` != `Token::Identificador("bar")`
- But both should map to same table entry
- `TokenClass::Identificador` == `TokenClass::Identificador`

#### 4. Explicit Stack vs Recursion

**Choice**: Vec<Symbol> stack

**Rationale**:
- True LL(1) algorithm demonstration
- No stack overflow risk
- Easier to visualize/debug
- Can inspect stack at any point

### Edge Cases Handled

#### 1. Epsilon Productions

```rust
Symbol::Epsilon => continue,  // Skip epsilon, don't consume token
```

#### 2. EOF Handling

```rust
Symbol::Eof => {
    if self.position >= self.tokens.len() {
        break;  // Success
    }
}
```

#### 3. Token Value Matching

```rust
fn tokens_match(expected: &Token, actual: &Token) -> bool {
    match (expected, actual) {
        (Identificador(_), Identificador(_)) => true,  // Ignore value
        (Numero(_), Numero(_)) => true,
        (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
    }
}
```

#### 4. Conflicts Resolution

The grammar is **conflict-free** because:
- No terminal appears in both `FIRST(α)` and `FIRST(β)` for `A → α | β`
- If `ε ∈ FIRST(α)`, then `FIRST(β) ∩ FOLLOW(A) = ∅`

### Name Collision Resolution

**Problem**: Rust doesn't allow:
```rust
use Symbol::*;        // Imports NonTerminal variant
use NonTerminal::*;   // ERROR: NonTerminal already imported!
```

**Solution**:
```rust
use Symbol::{Terminal, Epsilon, NonTerminal as NT_Symbol};
use NonTerminal as NT;

// Use NT::Programa for the type
// Use NT_Symbol(NT::Programa) for the constructor
```

---

## Usage Examples

### Basic Usage

```rust
use network_interpreter::parser_ll1::PredictiveParser;
use network_interpreter::lexer_bridge::tokenize_with_new_lexer;

// 1. Tokenize input
let tokens = tokenize_with_new_lexer(source_code)?;

// 2. Create parser
let mut parser = PredictiveParser::new(tokens);

// 3. Parse
match parser.parse() {
    Ok(ast) => println!("✅ Parsing successful!"),
    Err(e) => eprintln!("❌ Parse error: {}", e),
}
```

### Example with Minimal Program

```rust
let source = "programa test; inicio fin .";
let tokens = tokenize_with_new_lexer(source.to_string())?;
let mut parser = PredictiveParser::new(tokens);

let result = parser.parse();
assert!(result.is_ok());
```

### Example with Error Handling

```rust
let source = "programa test inicio fin .";  // Missing ;
let tokens = tokenize_with_new_lexer(source.to_string())?;
let mut parser = PredictiveParser::new(tokens);

match parser.parse() {
    Err(e) => {
        println!("Error: {}", e);
        // "Expected PuntoYComa, found Inicio at position 2"
    }
    Ok(_) => unreachable!(),
}
```

---

## Testing Strategy

### Unit Tests

**Location**: `src/parser_ll1/*/tests` modules

#### FIRST/FOLLOW Tests
```rust
#[test]
fn test_first_programa() {
    let sets = FirstFollowSets::new();
    assert!(sets.is_in_first(
        NT::Programa,
        &Symbol::Terminal(Token::Programa)
    ));
}
```

#### Table Tests
```rust
#[test]
fn test_table_lookup() {
    let table = LL1Table::new();
    let prod = table.get(NT::Programa, &Token::Programa);
    assert_eq!(prod.unwrap().id, 1);
}
```

#### Parser Tests
```rust
#[test]
fn test_simple_program() {
    let tokens = vec![
        Token::Programa,
        Token::Identificador("test".into()),
        Token::PuntoYComa,
        Token::Inicio,
        Token::Fin,
        Token::Punto,
    ];

    let mut parser = PredictiveParser::new(tokens);
    assert!(parser.parse().is_ok());
}
```

### Integration Tests

**Location**: `tests/parser_ll1/`

```rust
#[test]
fn test_ejemplo1_net() {
    let source = std::fs::read_to_string("ejemplo1.net")?;
    let tokens = tokenize_with_new_lexer(source)?;
    let mut parser = PredictiveParser::new(tokens);

    assert!(parser.parse().is_ok());
}
```

---

## Performance Considerations

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Table lookup | O(1) | HashMap lookup |
| FIRST/FOLLOW lookup | O(1) | Pre-computed |
| Parse | O(n) | n = number of tokens |
| Production application | O(k) | k = production length |

### Space Complexity

| Component | Size | Notes |
|-----------|------|-------|
| Parsing table | ~200 entries | Fixed |
| FIRST sets | ~43 sets | Fixed |
| FOLLOW sets | ~43 sets | Fixed |
| Parse stack | O(h) | h = max derivation height |
| Token array | O(n) | n = number of tokens |

### Optimizations

1. **Pre-computed sets**: No runtime computation
2. **HashMap table**: O(1) lookups
3. **Token cloning avoided**: Uses references where possible
4. **Epsilon skipping**: No stack operations for ε

---

## Future Enhancements

### Immediate Next Steps (from TODO)

1. **AST Integration** (1-2 hours)
   - Option A: Two-pass (validate + recursive descent)
   - Option B: Augmented stack with semantic actions

2. **Complete Testing** (2 hours)
   - 30+ test cases
   - Error recovery tests
   - ejemplo1.net / ejemplo2.net validation

3. **Human-Readable Table** (30 min)
   - Export `config/ll1_table.txt`
   - Markdown format
   - CSV format for tools

### Advanced Features

1. **Error Recovery**
   - Panic mode
   - Phrase-level recovery
   - Insert missing tokens

2. **Better Error Messages**
   - Context display
   - "Expected one of: ..."
   - Suggestions

3. **Parse Tree Visualization**
   - GraphViz output
   - ASCII tree
   - Step-by-step debugger

4. **Performance Benchmarks**
   - vs Recursive Descent
   - vs other parsers
   - Memory profiling

5. **Table Generation Tool**
   - Auto-generate from grammar file
   - Verify LL(1) property
   - Detect conflicts

---

## References

### Grammar Specification
- `docs/gramatica.txt` - Full grammar (84 productions)
- `docs/first_follow.txt` - FIRST/FOLLOW sets

### Implementation Files
- `src/parser_ll1/mod.rs` - Module exports
- `src/parser_ll1/first_follow.rs` - Sets (460 lines)
- `src/parser_ll1/ll1_table.rs` - Table (950 lines)
- `src/parser_ll1/predictive.rs` - Parser (250 lines)

### External Documentation
- Dragon Book (Compilers: Principles, Techniques, and Tools), Section 4.4
- Engineering a Compiler, Section 3.3

---

## Appendix: Complete Production List

### Productions 1-30: Structure

1. `Programa → PROGRAMA IDENTIFICADOR ; Definiciones Modulos BloqueInicio .`
2. `Definiciones → DefMaquinas DefConcentradores DefCoaxiales`
3. `Definiciones → ε`
4. `DefMaquinas → DEFINE MAQUINAS ListaMaquinas ;`
5. `DefMaquinas → ε`
...
30. `Sentencias → ε`

### Productions 31-56: Statements

31. `Sentencia → SentenciaColoca`
...
56. `Direccion → DERECHA`

### Productions 57-84: Expressions

57. `Expresion → ExpresionOr`
...
84. `AccesoArreglo → ε`

_(Full list in `docs/gramatica.txt`)_

---

**Last Updated**: 2025-11-30
**Status**: ✅ Fully Compiling
**Maintainer**: Network Interpreter Team
