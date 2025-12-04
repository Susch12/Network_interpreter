# Especificación del Formato de Autómata (.aut)

**Versión**: 1.0  
**Propósito**: Definir autómatas finitos deterministas (DFA) para análisis léxico  
**Extensión**: `.aut`  
**Codificación**: UTF-8  

---

## 1. Estructura General

Un archivo `.aut` consta de 4 secciones principales:
```
METADATA
  <metadatos del autómata>
END_METADATA

STATES
  <definición de estados>
END_STATES

TRANSITIONS
  <definición de transiciones>
END_TRANSITIONS

KEYWORDS
  <tabla de palabras reservadas>
END_KEYWORDS
```

---

## 2. Sección METADATA

Define información general del autómata.

### Formato:
```
METADATA
name: <nombre del autómata>
version: <versión>
description: <descripción opcional>
initial_state: <nombre del estado inicial>
END_METADATA
```

### Ejemplo:
```
METADATA
name: NetworkLanguageLexer
version: 1.0
description: Lexer para lenguaje de topologías de red
initial_state: q0
END_METADATA
```

### Reglas:
- `name`: alfanumérico, sin espacios
- `version`: formato X.Y o X.Y.Z
- `initial_state`: debe existir en la sección STATES

---

## 3. Sección STATES

Define todos los estados del autómata.

### Formato:
```
STATES
<nombre_estado> [FINAL:<tipo_token>] [#comentario]
END_STATES
```

### Ejemplo:
```
STATES
q0                                    # Estado inicial
q_id FINAL:IDENTIFIER                # Identificadores
q_num FINAL:NUMBER                   # Números
q_str FINAL:STRING                   # Cadenas
q_op_eq FINAL:EQUAL                  # Operador =
q_op_lt                              # Estado intermedio para 
q_op_lt_eq FINAL:LESS_EQUAL         # Operador <=
q_comment                            # Comentarios (se ignoran)
END_STATES
```

### Reglas:
- **Nombre de estado**: `[a-zA-Z_][a-zA-Z0-9_]*`
- **FINAL**: Marca estado final y especifica el tipo de token generado
- **Tipo de token**: Debe estar en mayúsculas con guiones bajos
- **Comentarios**: Inician con `#`, llegan hasta fin de línea

### Tipos de Token Estándar:
```
# Palabras reservadas
PROGRAMA, DEFINE, MAQUINAS, CONCENTRADORES, COAXIAL, SEGMENTO
MODULO, INICIO, FIN, SI, SINO
COLOCA, COLOCA_COAXIAL, COLOCA_COAXIAL_CONCENTRADOR
UNE_MAQUINA_PUERTO, ASIGNA_PUERTO
MAQUINA_COAXIAL, ASIGNA_MAQUINA_COAXIAL
ESCRIBE
ARRIBA, ABAJO, IZQUIERDA, DERECHA

# Operadores
EQUAL, LESS, GREATER, LESS_EQUAL, GREATER_EQUAL, NOT_EQUAL
AND, OR, NOT

# Delimitadores
COMMA, SEMICOLON, DOT, LPAREN, RPAREN, LBRACKET, RBRACKET

# Literales
IDENTIFIER, NUMBER, STRING

# Especiales
WHITESPACE, COMMENT, ERROR
```

---

## 4. Sección TRANSITIONS

Define las transiciones entre estados.

### Formato:
```
TRANSITIONS
<estado_origen>, <clase_caracteres>, <estado_destino> [#comentario]
END_TRANSITIONS
```

### Clases de Caracteres:

#### 4.1 Carácter Simple:
```
q0, a, q1          # Carácter literal 'a'
q0, +, q_plus      # Símbolo '+'
```

#### 4.2 Rango de Caracteres:
```
q0, [a-z], q_id    # Minúsculas
q0, [A-Z], q_id    # Mayúsculas
q0, [0-9], q_num   # Dígitos
```

#### 4.3 Múltiples Caracteres/Rangos:
```
q0, [a-zA-Z_], q_id              # Letras y guion bajo
q_id, [a-zA-Z0-9_], q_id         # Alfanuméricos y guion bajo
q_str, [^"\\], q_str             # Cualquier excepto " y \
```

#### 4.4 Caracteres Especiales Escapados:
```
q0, \n, q_newline      # Salto de línea
q0, \t, q_tab          # Tabulador
q0, \r, q_return       # Retorno de carro
q0, \\, q_backslash    # Barra invertida
q0, \", q_quote        # Comillas
q0, \s, q_space        # Espacio (equivalente a ' ')
```

#### 4.5 Clases Predefinidas:
```
ALPHA     = [a-zA-Z]           # Alfabético
DIGIT     = [0-9]              # Dígito
ALNUM     = [a-zA-Z0-9]        # Alfanumérico
SPACE     = [ \t\r\n]          # Espacios en blanco
PRINTABLE = [ -~]              # ASCII imprimible
ANY       = [todo carácter]    # Cualquier carácter
```

Uso:
```
q0, ALPHA, q_id
q0, DIGIT, q_num
q_str, ANY, q_str
```

#### 4.6 Negación:
```
q_str, [^"], q_str     # Cualquier cosa excepto comillas
q_str, [^\n], q_str    # Cualquier cosa excepto salto de línea
```

### Ejemplo Completo:
```
TRANSITIONS
# Identificadores y palabras reservadas
q0, [a-zA-Z_], q_id
q_id, [a-zA-Z0-9_], q_id

# Números
q0, [0-9], q_num
q_num, [0-9], q_num

# Cadenas
q0, ", q_str
q_str, [^"\\], q_str
q_str, \\, q_str_esc
q_str_esc, ANY, q_str
q_str, ", q_str_end

# Operadores relacionales
q0, =, q_op_eq
q0, <, q_op_lt
q_op_lt, =, q_op_le
q_op_lt, >, q_op_ne
q0, >, q_op_gt
q_op_gt, =, q_op_ge

# Operadores lógicos
q0, &, q_and1
q_and1, &, q_and2
q0, |, q_or1
q_or1, |, q_or2
q0, !, q_not

# Delimitadores
q0, ,, q_comma
q0, ;, q_semicolon
q0, ., q_dot
q0, (, q_lparen
q0, ), q_rparen
q0, [, q_lbracket
q0, ], q_rbracket

# Espacios en blanco (se ignoran)
q0, SPACE, q_ws
q_ws, SPACE, q_ws

# Comentarios // hasta fin de línea
q0, /, q_slash
q_slash, /, q_comment
q_comment, [^\n], q_comment
q_comment, \n, q0

END_TRANSITIONS
```

### Reglas:
1. Cada transición debe estar en una línea
2. Los campos se separan por comas
3. Los espacios alrededor de comas son opcionales
4. El estado destino debe existir en STATES
5. Si hay múltiples transiciones con la misma clase desde el mismo estado, se toma la primera (orden de prioridad)

---

## 5. Sección KEYWORDS

Define palabras reservadas que deben ser reconocidas como tokens especiales en lugar de identificadores.

### Formato:
```
KEYWORDS
<palabra>, <tipo_token> [#comentario]
END_KEYWORDS
```

### Ejemplo:
```
KEYWORDS
# Estructura del programa
programa, PROGRAMA
define, DEFINE
maquinas, MAQUINAS
concentradores, CONCENTRADORES
coaxial, COAXIAL
segmento, SEGMENTO
modulo, MODULO
inicio, INICIO
fin, FIN

# Control de flujo
si, SI
sino, SINO

# Funciones del lenguaje (case-sensitive)
coloca, COLOCA
colocaCoaxial, COLOCA_COAXIAL
colocaCoaxialConcentrador, COLOCA_COAXIAL_CONCENTRADOR
uneMaquinaPuerto, UNE_MAQUINA_PUERTO
asignaPuerto, ASIGNA_PUERTO
maquinaCoaxial, MAQUINA_COAXIAL
asignaMaquinaCoaxial, ASIGNA_MAQUINA_COAXIAL
escribe, ESCRIBE

# Direcciones
arriba, ARRIBA
abajo, ABAJO
izquierda, IZQUIERDA
derecha, DERECHA

END_KEYWORDS
```

### Reglas:
1. Las palabras se comparan **case-insensitive** por defecto
2. Para comparación **case-sensitive**, usar el modificador `[CASE_SENSITIVE]`:
```
   colocaCoaxial [CASE_SENSITIVE], COLOCA_COAXIAL
```
3. La palabra no debe contener espacios
4. El tipo de token debe ser único

---

## 6. Comentarios

### Formato:
```
# Esto es un comentario de línea completa

q0, [a-z], q_id    # Comentario al final de línea
```

### Reglas:
- Inician con `#`
- Se extienden hasta el final de la línea
- Pueden aparecer en cualquier lugar
- Son completamente ignorados por el parser

---

## 7. Estados Especiales

### 7.1 Estados de Error:
```
STATES
q_error FINAL:ERROR    # Estado de error léxico
END_STATES
```

### 7.2 Estados Ignorados:
```
STATES
q_ws FINAL:WHITESPACE    # Espacios (se ignoran)
q_comment FINAL:COMMENT  # Comentarios (se ignoran)
END_STATES
```

Los tokens de tipo `WHITESPACE` y `COMMENT` son automáticamente descartados por el scanner.

---

## 8. Validación del Archivo

### Reglas de Validación:

1. **Estructura**:
   - ✓ Todas las secciones deben estar presentes
   - ✓ Las secciones deben aparecer en orden
   - ✓ Cada sección debe tener su marcador de fin

2. **Estados**:
   - ✓ El estado inicial debe existir
   - ✓ Nombres de estados únicos
   - ✓ Al menos un estado final
   - ✓ Todos los estados referenciados en transiciones deben estar definidos

3. **Transiciones**:
   - ✓ El estado origen debe existir
   - ✓ El estado destino debe existir
   - ✓ Clases de caracteres válidas
   - ✓ Sin transiciones ambiguas desde el mismo estado

4. **Keywords**:
   - ✓ Tipos de token únicos
   - ✓ Palabras únicas
   - ✓ Todos los tipos de token deben existir en STATES finales

### Herramienta de Validación:
```bash
# Comando para validar un archivo .aut
cargo run --bin validate-automaton config/lexer.aut
```

Salida esperada:
```
✓ Estructura válida
✓ 45 estados definidos
✓ Estado inicial 'q0' encontrado
✓ 12 estados finales
✓ 123 transiciones válidas
✓ 0 conflictos detectados
✓ 28 palabras reservadas
✓ Autómata válido y listo para usar
```

---

## 9. Ejemplo Completo Mínimo
```
METADATA
name: MinimalLexer
version: 1.0
initial_state: q0
END_METADATA

STATES
q0
q_id FINAL:IDENTIFIER
q_num FINAL:NUMBER
END_STATES

TRANSITIONS
q0, [a-zA-Z], q_id
q_id, [a-zA-Z0-9], q_id
q0, [0-9], q_num
q_num, [0-9], q_num
END_TRANSITIONS

KEYWORDS
if, IF
else, ELSE
END_KEYWORDS
```

---

## 10. Convenciones de Nomenclatura

### Estados:
- Prefijo `q_` para todos los estados
- Nombre descriptivo del propósito
- Snake_case: `q_identifier`, `q_op_less_equal`

### Tipos de Token:
- SCREAMING_SNAKE_CASE: `IDENTIFIER`, `LESS_EQUAL`
- Sin prefijo `TOKEN_`

### Palabras Reservadas:
- Minúsculas para case-insensitive
- camelCase para case-sensitive

---

## 11. Extensiones Futuras

Características planeadas para versiones futuras:

- [ ] Soporte para rangos Unicode: `[\u0000-\uFFFF]`
- [ ] Acciones semánticas: `q0, [0-9], q_num {valor += ch}`
- [ ] Autómatas parametrizados: `#include "common.aut"`
- [ ] Optimización automática: `#optimize minimize`
- [ ] Generación de visualización: `#export_graphviz`

---

**Fin de Especificación**
