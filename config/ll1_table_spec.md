# Especificación del Formato de Tabla LL(1) (.ll1)

**Versión**: 1.0  
**Propósito**: Definir tablas de análisis predictivo para parsing LL(1)  
**Extensión**: `.ll1`  
**Codificación**: UTF-8  

---

## 1. Estructura General

Un archivo `.ll1` consta de 4 secciones:
```
METADATA
  <metadatos de la tabla>
END_METADATA

TERMINALS
  <lista de símbolos terminales>
END_TERMINALS

NONTERMINALS
  <lista de símbolos no-terminales>
END_NONTERMINALS

TABLE
  <entradas de la tabla M[No-terminal, Terminal]>
END_TABLE
```

---

## 2. Sección METADATA

### Formato:
```
METADATA
name: <nombre de la gramática>
version: <versión>
start_symbol: <símbolo inicial>
description: <descripción opcional>
END_METADATA
```

### Ejemplo:
```
METADATA
name: NetworkLanguageGrammar
version: 1.0
start_symbol: Programa
description: Gramática LL(1) para lenguaje de topologías de red
END_METADATA
```

---

## 3. Sección TERMINALS

Define todos los símbolos terminales (tokens).

### Formato:
```
TERMINALS
<nombre_terminal> [#comentario]
END_TERMINALS
```

### Ejemplo:
```
TERMINALS
# Palabras reservadas
PROGRAMA
DEFINE
MAQUINAS
CONCENTRADORES
COAXIAL
SEGMENTO
MODULO
INICIO
FIN
SI
SINO

# Funciones
COLOCA
COLOCA_COAXIAL
COLOCA_COAXIAL_CONCENTRADOR
UNE_MAQUINA_PUERTO
ASIGNA_PUERTO
MAQUINA_COAXIAL
ASIGNA_MAQUINA_COAXIAL
ESCRIBE

# Direcciones
ARRIBA
ABAJO
IZQUIERDA
DERECHA

# Operadores
EQUAL
LESS
GREATER
LESS_EQUAL
GREATER_EQUAL
NOT_EQUAL
AND
OR
NOT

# Delimitadores
COMMA
SEMICOLON
DOT
LPAREN
RPAREN
LBRACKET
RBRACKET

# Literales
IDENTIFIER
NUMBER
STRING

# Especiales
EOF

END_TERMINALS
```

### Reglas:
- Nombres en SCREAMING_SNAKE_CASE
- Deben coincidir con los tipos de token del autómata
- `EOF` es obligatorio (marca fin de entrada)

---

## 4. Sección NONTERMINALS

Define todos los símbolos no-terminales.

### Formato:
```
NONTERMINALS
<nombre_no_terminal> [#comentario]
END_NONTERMINALS
```

### Ejemplo:
```
NONTERMINALS
# Estructura principal
Programa
Definiciones
DefMaquinas
DefConcentradores
DefCoaxiales
TipoCoaxial

# Listas
ListaMaquinas
ListaMaquinas'
ListaConcentradores
ListaConcentradores'
DeclConcentrador
OpcionCoaxial
ListaCoaxiales
ListaCoaxiales'
DeclCoaxial

# Módulos
Modulos
Modulo
BloqueInicio

# Sentencias
Sentencias
Sentencia
SentenciaColoca
SentenciaColocaCoaxial
SentenciaColocaCoaxialConcentrador
SentenciaUneMaquinaPuerto
SentenciaAsignaPuerto
SentenciaMaquinaCoaxial
SentenciaAsignaMaquinaCoaxial
SentenciaEscribe
SentenciaSi
OpcionSino
LlamadaModulo

# Direcciones
Direccion

# Expresiones
Expresion
ExpresionOr
ExpresionOr'
ExpresionAnd
ExpresionAnd'
ExpresionRelacional
OpRelacional
OperadorRelacional
ExpresionNot
ExpresionPrimaria
Accesos
AccesoCampo
AccesoArreglo

END_NONTERMINALS
```

### Reglas:
- Nombres en PascalCase
- El primer no-terminal es el símbolo inicial (debe coincidir con METADATA)
- Apóstrofe `'` permitido para producciones auxiliares

---

## 5. Sección TABLE

Define las entradas de la tabla de análisis predictivo M[No-terminal, Terminal].

### Formato:
```
TABLE
<no_terminal>, <terminal>, <produccion> [#comentario]
END_TABLE
```

### Formato de Producción:

#### 5.1 Producción Simple:
```
Programa, PROGRAMA, PROGRAMA IDENTIFICADOR SEMICOLON Definiciones Modulos BloqueInicio DOT
```

#### 5.2 Producción Vacía (ε):
```
Definiciones, MODULO, EPSILON
Definiciones, INICIO, EPSILON
```

**Nota**: `EPSILON` (o `ε`) representa la producción vacía.

#### 5.3 Símbolos en Producción:
- **Terminales**: En SCREAMING_SNAKE_CASE
- **No-terminales**: En PascalCase
- **Separados por espacio**

### Ejemplo Completo:
```
TABLE
# [1] Programa → PROGRAMA IDENTIFICADOR PUNTO_COMA ...
Programa, PROGRAMA, PROGRAMA IDENTIFICADOR SEMICOLON Definiciones Modulos BloqueInicio DOT

# [2][3] Definiciones → ...
Definiciones, DEFINE, DefMaquinas DefConcentradores DefCoaxiales
Definiciones, MODULO, EPSILON
Definiciones, INICIO, EPSILON

# [4][5] DefMaquinas → ...
DefMaquinas, DEFINE, DEFINE MAQUINAS ListaMaquinas SEMICOLON
DefMaquinas, MODULO, EPSILON
DefMaquinas, INICIO, EPSILON

# [12] ListaMaquinas → IDENTIFICADOR ListaMaquinas'
ListaMaquinas, IDENTIFIER, IDENTIFIER ListaMaquinas'

# [13][14] ListaMaquinas' → ...
ListaMaquinas', COMMA, COMMA IDENTIFIER ListaMaquinas'
ListaMaquinas', SEMICOLON, EPSILON

# [31-40] Sentencia → ...
Sentencia, COLOCA, SentenciaColoca
Sentencia, COLOCA_COAXIAL, SentenciaColocaCoaxial
Sentencia, COLOCA_COAXIAL_CONCENTRADOR, SentenciaColocaCoaxialConcentrador
Sentencia, UNE_MAQUINA_PUERTO, SentenciaUneMaquinaPuerto
Sentencia, ASIGNA_PUERTO, SentenciaAsignaPuerto
Sentencia, MAQUINA_COAXIAL, SentenciaMaquinaCoaxial
Sentencia, ASIGNA_MAQUINA_COAXIAL, SentenciaAsignaMaquinaCoaxial
Sentencia, ESCRIBE, SentenciaEscribe
Sentencia, SI, SentenciaSi
Sentencia, IDENTIFIER, LlamadaModulo

# [41] SentenciaColoca → COLOCA LPAREN IDENTIFICADOR COMA Expresion COMA Expresion RPAREN SEMICOLON
SentenciaColoca, COLOCA, COLOCA LPAREN IDENTIFIER COMMA Expresion COMMA Expresion RPAREN SEMICOLON

# [57] Expresion → ExpresionOr
Expresion, NOT, ExpresionOr
Expresion, NUMBER, ExpresionOr
Expresion, STRING, ExpresionOr
Expresion, IDENTIFIER, ExpresionOr
Expresion, LPAREN, ExpresionOr

# [58] ExpresionOr → ExpresionAnd ExpresionOr'
ExpresionOr, NOT, ExpresionAnd ExpresionOr'
ExpresionOr, NUMBER, ExpresionAnd ExpresionOr'
ExpresionOr, STRING, ExpresionAnd ExpresionOr'
ExpresionOr, IDENTIFIER, ExpresionAnd ExpresionOr'
ExpresionOr, LPAREN, ExpresionAnd ExpresionOr'

# [59][60] ExpresionOr' → ...
ExpresionOr', OR, OR ExpresionAnd ExpresionOr'
ExpresionOr', RPAREN, EPSILON
ExpresionOr', COMMA, EPSILON
ExpresionOr', RBRACKET, EPSILON
ExpresionOr', SEMICOLON, EPSILON
ExpresionOr', INICIO, EPSILON

END_TABLE
```

### Reglas:
1. Cada entrada M[A, a] debe tener **máximo una producción** (propiedad LL(1))
2. Si M[A, a] está vacía, el parser reportará error de sintaxis
3. Las producciones deben estar listadas en orden de no-terminal
4. Comentarios pueden referenciar el número de producción de la gramática

---

## 6. Validación de la Tabla

### Reglas de Validación:

1. **Completitud**:
   - ✓ Todas las combinaciones necesarias deben estar cubiertas
   - ✓ No puede haber entradas ambiguas (múltiples producciones para M[A, a])

2. **Consistencia**:
   - ✓ Todos los terminales usados deben estar en TERMINALS
   - ✓ Todos los no-terminales usados deben estar en NONTERMINALS
   - ✓ El símbolo inicial debe aparecer

3. **Corrección LL(1)**:
   - ✓ Para cada M[A, a], máximo una producción
   - ✓ Las producciones deben corresponder a la gramática

### Herramienta de Validación:
```bash
# Comando para validar tabla LL(1)
cargo run --bin validate-ll1 config/parser.ll1
```

Salida esperada:
```
✓ Metadata válido
✓ 45 terminales definidos
✓ 42 no-terminales definidos
✓ Símbolo inicial: Programa
✓ 387 entradas en la tabla
✓ 0 conflictos LL(1)
✓ 0 entradas ambiguas
✓ Tabla LL(1) válida y completa
```

---

## 7. Algoritmo de Uso

### Pseudocódigo del Parser:
```python
def parse(input_tokens, ll1_table):
    stack = [EOF, start_symbol]
    input_cursor = 0
    
    while stack is not empty:
        top = stack.top()
        current_token = input_tokens[input_cursor]
        
        if top is terminal:
            if top == current_token:
                stack.pop()
                input_cursor += 1
            else:
                error("Expected", top, "but found", current_token)
        
        elif top is nonterminal:
            production = ll1_table[top, current_token]
            
            if production is not None:
                stack.pop()
                push_reverse(stack, production.right_side)
            else:
                error("Unexpected token", current_token, 
                      "for non-terminal", top)
        
        elif top == EOF:
            if current_token == EOF:
                return SUCCESS
            else:
                error("Unexpected tokens after end")
    
    return SUCCESS
```

---

## 8. Formato Compacto (Opcional)

Para tablas grandes, se permite un formato compacto:

### Formato Extendido:
```
TABLE_COMPACT
<no_terminal>, <produccion>, <lista_terminales>
END_TABLE_COMPACT
```

### Ejemplo:
```
TABLE_COMPACT
# Múltiples terminales comparten la misma producción
ExpresionOr', EPSILON, [RPAREN, COMMA, RBRACKET, SEMICOLON, INICIO]
Definiciones, EPSILON, [MODULO, INICIO]
END_TABLE_COMPACT
```

Equivalente a:
```
TABLE
ExpresionOr', RPAREN, EPSILON
ExpresionOr', COMMA, EPSILON
ExpresionOr', RBRACKET, EPSILON
ExpresionOr', SEMICOLON, EPSILON
ExpresionOr', INICIO, EPSILON
Definiciones, MODULO, EPSILON
Definiciones, INICIO, EPSILON
END_TABLE
```

---

## 9. Mensajes de Error

La tabla puede incluir mensajes de error personalizados:

### Formato:
```
ERROR_MESSAGES
<no_terminal>, <terminal>, <mensaje>
END_ERROR_MESSAGES
```

### Ejemplo:
```
ERROR_MESSAGES
Programa, IDENTIFIER, "Se esperaba 'programa' al inicio del archivo"
Sentencia, NUMBER, "Las sentencias no pueden empezar con números"
Expresion, FIN, "Expresión incompleta antes de 'fin'"
END_ERROR_MESSAGES
```

---

## 10. Ejemplo Completo Mínimo
```
METADATA
name: TinyGrammar
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
S'
END_NONTERMINALS

TABLE
S, A, A S'
S', A, A S'
S', B, EPSILON
S', EOF, EPSILON
END_TABLE
```

---

**Fin de Especificación**
