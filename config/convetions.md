# Convenciones de Codificación y Estilo

## 1. Nomenclatura

### 1.1 Estados del Autómata:
```
Formato: q_<descripcion>

Ejemplos:
✓ q0              # Estado inicial
✓ q_id            # Identificadores
✓ q_num           # Números
✓ q_str           # Cadenas
✓ q_op_eq         # Operador igual
✓ q_op_lt_eq      # Operador menor-igual
✓ q_comment       # Comentarios
✓ q_ws            # Whitespace

✗ state1          # No descriptivo
✗ Q_ID            # No usar mayúsculas
✗ q-id            # No usar guiones
```

### 1.2 Tipos de Token:
```
Formato: SCREAMING_SNAKE_CASE

Ejemplos:
✓ IDENTIFIER
✓ NUMBER
✓ STRING
✓ LESS_EQUAL
✓ COLOCA_COAXIAL
✓ UNE_MAQUINA_PUERTO

✗ identifier      # No minúsculas
✗ LessEqual       # No PascalCase
✗ LESS-EQUAL      # No guiones
```

### 1.3 No-Terminales de Gramática:
```
Formato: PascalCase

Ejemplos:
✓ Programa
✓ Expresion
✓ ListaMaquinas
✓ SentenciaColoca
✓ ExpresionOr'    # Apóstrofe permitido

✗ programa        # No minúsculas
✗ EXPRESION       # No mayúsculas
✗ lista_maquinas  # No snake_case
```

### 1.4 Terminales de Gramática:
```
Formato: SCREAMING_SNAKE_CASE (igual que tipos de token)

Ejemplos:
✓ PROGRAMA
✓ IDENTIFIER
✓ SEMICOLON
✓ LPAREN

✗ programa
✗ Programa
```

---

## 2. Formato de Archivos

### 2.1 Indentación:
- **Espacios**: 2 o 4 espacios (no tabs)
- **Consistencia**: Mismo nivel en toda la sección

### 2.2 Líneas:
- **Máximo**: 100 caracteres
- **Líneas en blanco**: 1 entre secciones
- **Comentarios**: Alineados verticalmente si es posible

### 2.3 Ejemplo de Formato:
```
STATES
q0                          # Estado inicial
q_id FINAL:IDENTIFIER      # Identificadores
q_num FINAL:NUMBER         # Números enteros
q_str FINAL:STRING         # Cadenas literales
END_STATES
```

---

## 3. Comentarios

### 3.1 Estilo:
```
# Comentario de sección completa explicando el propósito

q0, [a-z], q_id    # Comentario inline breve
```

### 3.2 Cuándo Comentar:
- **Siempre**: Secciones principales
- **Recomendado**: Estados finales (tipo de token que generan)
- **Opcional**: Transiciones obvias
- **Necesario**: Transiciones complejas o no obvias

### 3.3 Ejemplos:

✓ **Bueno**:
```
# Operadores relacionales compuestos
q_op_lt, =, q_op_le    # <= (menor o igual)
q_op_lt, >, q_op_ne    # <> (diferente)
```

✗ **Malo**:
```
q_op_lt, =, q_op_le    # Transición de q_op_lt a q_op_le con =
```

---

## 4. Organización de Secciones

### 4.1 Orden en STATES:
```
1. Estado inicial (q0)
2. Estados de identificadores/keywords
3. Estados de números
4. Estados de cadenas
5. Estados de operadores
6. Estados de delimitadores
7. Estados especiales (whitespace, comments, error)
```

### 4.2 Orden en TRANSITIONS:
```
1. Identificadores y keywords
2. Números
3. Cadenas
4. Operadores (por tipo)
5. Delimitadores
6. Whitespace
7. Comentarios
```

### 4.3 Orden en KEYWORDS:
```
1. Estructura del programa
2. Control de flujo
3. Funciones del lenguaje
4. Direcciones/constantes
5. Otros
```

---

## 5. Buenas Prácticas

### 5.1 Autómatas:

✓ **Hacer**:
- Usar nombres descriptivos
- Agrupar transiciones relacionadas
- Documentar estados finales
- Mantener consistencia en nomenclatura

✗ **Evitar**:
- Estados redundantes
- Nombres genéricos (q1, q2, q3)
- Transiciones ambiguas
- Falta de documentación

### 5.2 Tablas LL(1):

✓ **Hacer**:
- Ordenar entradas por no-terminal
- Documentar con números de producción
- Usar nombres consistentes con la gramática
- Incluir todas las combinaciones necesarias

✗ **Evitar**:
- Entradas duplicadas
- Conflictos LL(1)
- Símbolos no definidos
- Producciones incompletas

---

## 6. Control de Versiones

### 6.1 Archivos a Versionar:
```
config/
├── lexer.aut          # Autómata del lexer
├── parser.ll1         # Tabla LL(1)
└── README.md          # Documentación

docs/
├── gramatica.txt      # Gramática formal
├── first_follow.txt   # Conjuntos FIRST/FOLLOW
└── conflictos.txt     # Análisis de conflictos
```

### 6.2 Mensajes de Commit:
```
✓ "feat(lexer): agregar soporte para números hexadecimales"
✓ "fix(parser): corregir conflicto LL(1) en expresiones"
✓ "docs(grammar): actualizar tabla FIRST/FOLLOW"

✗ "actualización"
✗ "cambios"
✗ "fix"
```

---

## 7. Testing

### 7.1 Casos de Prueba Obligatorios:

Para **autómatas**:
- [ ] Tokens válidos
- [ ] Tokens inválidos
- [ ] Palabras reservadas vs identificadores
- [ ] Operadores compuestos
- [ ] Cadenas con escapes
- [ ] Comentarios
- [ ] Whitespace

Para **tablas LL(1)**:
- [ ] Programas válidos completos
- [ ] Errores de sintaxis
- [ ] Producciones-ε
- [ ] Expresiones con precedencia
- [ ] Sentencias anidadas

---

## 8. Documentación

### 8.1 Archivos Requeridos:
```
config/
├── automaton_spec.md      # Especificación formato .aut
├── ll1_table_spec.md      # Especificación formato .ll1
└── validation_schema.md   # Esquema de validación

docs/
├── conventions.md         # Este archivo
├── examples/
│   ├── minimal.aut       # Ejemplo mínimo
│   ├── full.aut          # Ejemplo completo
│   ├── minimal.ll1       # Ejemplo mínimo
│   └── full.ll1          # Ejemplo completo
└── tutorials/
    ├── creating_automaton.md
    └── creating_ll1_table.md
```

---

**Fin de Convenciones**
```

---

## 📊 Resumen de Fase 1.2

### ✅ Checklist de Completitud
```
[✓] 1. Especificación de formato .aut completa
[✓] 2. Especificación de formato .ll1 completa
[✓] 3. Esquema de validación diseñado
[✓] 4. Convenciones documentadas
[✓] 5. Ejemplos mínimos incluidos
[✓] 6. Validadores especificados
[✓] 7. Test suite planificada
[✓] 8. Mensajes de error definidos
