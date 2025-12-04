# Flujo de Datos del Intérprete

## 1. Diagrama de Secuencia Completo
```
Usuario        Main         Config      Scanner      Parser      Semantic    Interpreter    Visualizer
  │              │            │            │            │            │            │             │
  │ archivo.net  │            │            │            │            │            │             │
  │─────────────>│            │            │            │            │            │             │
  │              │            │            │            │            │            │             │
  │              │ load_config│            │            │            │            │             │
  │              │───────────>│            │            │            │            │             │
  │              │            │ read .aut  │            │            │            │             │
  │              │            │ read .ll1  │            │            │            │             │
  │              │<───────────│            │            │            │            │             │
  │              │ Automaton  │            │            │            │            │             │
  │              │ LL1Table   │            │            │            │            │             │
  │              │            │            │            │            │            │             │
  │              │ read source│            │            │            │            │             │
  │              │────────┐   │            │            │            │            │             │
  │              │<───────┘   │            │            │            │            │             │
  │              │ String     │            │            │            │            │             │
  │              │            │            │            │            │            │             │
  │              │ new Scanner│            │            │            │            │             │
  │              │────────────────────────>│            │            │            │             │
  │              │            │            │ scan_all   │            │            │             │
  │              │            │            │────────┐   │            │            │             │
  │              │            │            │<───────┘   │            │            │             │
  │              │<────────────────────────│            │            │            │             │
  │              │ Vec<Token> │            │            │            │            │             │
  │              │            │            │            │            │            │             │
  │              │ new Parser │            │            │            │            │             │
  │              │────────────────────────────────────>│            │            │             │
  │              │            │            │            │ parse      │            │             │
  │              │            │            │            │────────┐   │            │             │
  │              │            │            │            │<───────┘   │            │             │
  │              │<────────────────────────────────────│            │            │             │
  │              │ AST        │            │            │            │            │             │
  │              │            │            │            │            │            │             │
  │              │ new Analyzer│           │            │            │            │             │
  │              │────────────────────────────────────────────────>│            │             │
  │              │            │            │            │            │ analyze    │             │
  │              │            │            │            │            │────────┐   │             │
  │              │            │            │            │            │<───────┘   │             │
  │              │<────────────────────────────────────────────────│            │             │
  │              │ SymbolTable│            │            │            │            │             │
  │              │            │            │            │            │            │             │
  │              │ new Interpreter         │            │            │            │             │
  │              │────────────────────────────────────────────────────────────>│             │
  │              │            │            │            │            │            │ ejecutar    │
  │              │            │            │            │            │            │────────┐    │
  │              │            │            │            │            │            │<───────┘    │
  │              │<────────────────────────────────────────────────────────────│             │
  │              │ Environment│            │            │            │            │             │
  │              │            │            │            │            │            │             │
  │              │ if --visualize          │            │            │            │             │
  │              │────────────────────────────────────────────────────────────────────────────>│
  │              │            │            │            │            │            │             │
  │              │                                                                               │
  │<──────────────────────────────────────────────────────────────────────────────────────────│
  │              │                                                   GUI Interactiva             │
```

## 2. Transformaciones de Datos

### 2.1 String → Tokens
```
ENTRADA:
┌────────────────────────────────────┐
│ programa test;                     │
│ define maquinas                    │
│   A, B;                            │
│ inicio                             │
│   coloca(A, 10, 20);              │
│ fin.                               │
└────────────────────────────────────┘

        │ Scanner
        │ (DFA)
        ▼

SALIDA:
┌────────────────────────────────────┐
│ Token { PROGRAMA, "programa", 1:1 }│
│ Token { IDENTIFIER, "test", 1:10 } │
│ Token { SEMICOLON, ";", 1:14 }     │
│ Token { DEFINE, "define", 2:1 }    │
│ Token { MAQUINAS, "maquinas", 2:8 }│
│ Token { IDENTIFIER, "A", 3:3 }     │
│ Token { COMMA, ",", 3:4 }          │
│ Token { IDENTIFIER, "B", 3:6 }     │
│ Token { SEMICOLON, ";", 3:7 }      │
│ Token { INICIO, "inicio", 4:1 }    │
│ Token { COLOCA, "coloca", 5:3 }    │
│ Token { LPAREN, "(", 5:9 }         │
│ Token { IDENTIFIER, "A", 5:10 }    │
│ Token { COMMA, ",", 5:11 }         │
│ Token { NUMBER, "10", 5:13 }       │
│ Token { COMMA, ",", 5:15 }         │
│ Token { NUMBER, "20", 5:17 }       │
│ Token { RPAREN, ")", 5:19 }        │
│ Token { SEMICOLON, ";", 5:20 }     │
│ Token { FIN, "fin", 6:1 }          │
│ Token { DOT, ".", 6:4 }            │
│ Token { EOF, "", 6:5 }             │
└────────────────────────────────────┘
```

### 2.2 Tokens → AST
```
ENTRADA:
┌────────────────────────────────────┐
│ Vec<Token>                         │
│ [PROGRAMA, IDENTIFIER("test"), ...]│
└────────────────────────────────────┘

        │ Parser Predictivo
        │ (Tabla LL(1))
        ▼

SALIDA:
┌────────────────────────────────────┐
│ Program {                          │
│   nombre: "test",                  │
│   definiciones: Definitions {      │
│     maquinas: [                    │
│       MaquinaDecl { nombre: "A" }, │
│       MaquinaDecl { nombre: "B" }  │
│     ],                             │
│     concentradores: [],            │
│     coaxiales: []                  │
│   },                               │
│   modulos: [],                     │
│   sentencias: [                    │
│     Statement::Coloca {            │
│       objeto: "A",                 │
│       x: Expr::Numero(10),         │
│       y: Expr::Numero(20)          │
│     }                              │
│   ]                                │
│ }                                  │
└────────────────────────────────────┘
```

### 2.3 AST → AST Anotado + Symbol Table
```
ENTRADA:
┌────────────────────────────────────┐
│ Program { ... }                    │
└────────────────────────────────────┘

        │ Semantic Analyzer
        │
        ▼

SALIDA:
┌────────────────────────────────────┐
│ SymbolTable {                      │
│   maquinas: {                      │
│     "A": MaquinaSymbol {           │
│       nombre: "A",                 │
│       presente: false,             │
│       tipo: Type::Maquina          │
│     },                             │
│     "B": MaquinaSymbol { ... }     │
│   },                               │
│   concentradores: {},              │
│   coaxiales: {}                    │
│ }                                  │
│                                    │
│ + AST con tipos anotados           │
└────────────────────────────────────┘
```

### 2.4 AST + SymbolTable → Environment
```
ENTRADA:
┌────────────────────────────────────┐
│ Program + SymbolTable              │
└────────────────────────────────────┘

        │ Interpreter
        │
        ▼

SALIDA:
┌────────────────────────────────────┐
│ Environment {                      │
│   maquinas: {                      │
│     "A": RuntimeMaquina {          │
│       nombre: "A",                 │
│       x: 10,                       │
│       y: 20,                       │
│       colocada: true,              │
│       conectada_a: None            │
│     },                             │
│     "B": RuntimeMaquina {          │
│       colocada: false, ...         │
│     }                              │
│   },                               │
│   concentradores: {},              │
│   coaxiales: {},                   │
│   output: []                       │
│ }                                  │
└────────────────────────────────────┘
```

---

## 3. Estados del Sistema
```
┌──────────────┐
│  UNINITIALIZED│ ← Estado inicial
└──────┬───────┘
       │ load_config()
       ▼
┌──────────────┐
│  CONFIG_LOADED│ ← Autómata y tabla cargados
└──────┬───────┘
       │ scan()
       ▼
┌──────────────┐
│  TOKENIZED   │ ← Tokens generados
└──────┬───────┘
       │ parse()
       ▼
┌──────────────┐
│  PARSED      │ ← AST construido
└──────┬───────┘
       │ analyze()
       ▼
┌──────────────┐
│  ANALYZED    │ ← Semánticamente válido
└──────┬───────┘
       │ execute()
       ▼
┌──────────────┐
│  EXECUTED    │ ← Programa interpretado
└──────┬───────┘
       │ visualize() [opcional]
       ▼
┌──────────────┐
│  VISUALIZED  │ ← GUI mostrada
└──────────────┘
```

---

## 4. Manejo de Errores por Fase
```
┌─────────────────┐
│     Scanner     │
│                 │
│  Error léxico   │ → LexicalError → Report + Exit(1)
└─────────────────┘

┌─────────────────┐
│     Parser      │
│                 │
│  Error sintáctico│ → SyntaxError → Report + Exit(1)
└─────────────────┘

┌─────────────────┐
│ Semantic Analyzer│
│                 │
│  Error semántico │ → SemanticError → Report + Exit(1)
└─────────────────┘

┌─────────────────┐
│  Interpreter    │
│                 │
│  Error runtime  │ → RuntimeError → Report + Exit(1)
└─────────────────┘
```

**Estrategia**: Fail-fast. Si hay error en cualquier fase, reportar y detener.

---

**Fin de Flujo de Datos**
