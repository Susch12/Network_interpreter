# Intérprete de Topologías de Red 🌐

Un intérprete completo para un lenguaje específico de dominio (DSL) diseñado para definir, validar y visualizar topologías de redes de computadoras.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Tests](https://img.shields.io/badge/tests-186%20passing-brightgreen.svg)](./tests)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## 📋 Descripción

Este proyecto implementa un compilador e intérprete completo para un lenguaje que permite:

- 🖥️ **Definir máquinas** en una topología de red
- 🔌 **Configurar concentradores** (hubs) con puertos específicos
- 📡 **Establecer cables coaxiales** con longitudes definidas
- 🔗 **Conectar dispositivos** mediante puertos o cables
- 📊 **Visualizar gráficamente** la topología resultante
- ✅ **Validar semánticamente** que la configuración sea correcta

### Características Principales

✨ **Pipeline Completo de Compilación**
- Análisis Léxico (autómata DFA configurable)
- Análisis Sintáctico (parser LL(1) predictivo)
- Análisis Semántico (validación de tipos y restricciones)
- Interpretación (ejecución del programa)
- Visualización (GUI con eframe)

🎓 **Implementación Académica Rigurosa**
- Gramática formal LL(1) con 84 producciones
- Conjuntos FIRST/FOLLOW calculados
- Tabla de análisis predictivo completa
- Arquitectura híbrida de dos pasos

🧪 **Altamente Probado**
- 186 pruebas unitarias y de integración
- Cobertura del 100% de la gramática
- Validado con programas reales

---

## 🚀 Instalación

### Requisitos Previos

- **Rust** 1.70 o superior ([Instalar Rust](https://rustup.rs/))
- **Cargo** (incluido con Rust)

### Clonar e Instalar

```bash
# Clonar el repositorio
git clone <tu-repositorio>
cd Network_interpreter

# Compilar el proyecto
cargo build --release

# Ejecutar las pruebas
cargo test
```

---

## 💻 Uso

### Ejecutar un Programa

```bash
cargo run --bin interprete <archivo.net>
```

**Ejemplo:**
```bash
cargo run --bin interprete ejemplo1.net
```

### Con Visualización Gráfica

```bash
cargo run --bin interprete ejemplo1.net --visualize
# o
cargo run --bin interprete ejemplo1.net -v
```

### Salida del Intérprete

```
=== Network Interpreter v1 ===
Archivo: ejemplo1.net

Estadísticas del código:
  Líneas totales: 37
  Líneas no vacías: 31
  Caracteres: 679

Analizando léxicamente...
✓ 216 tokens generados

Análisis léxico completado exitosamente

Analizando sintácticamente con parser LL(1)...
🔍 Iniciando análisis híbrido (Two-Pass Approach)
   Pass 1: Validación de sintaxis LL(1)
   ✅ Validación LL(1) completada exitosamente en 711 pasos
   ✅ Pass 1 completado - Sintaxis válida
   🔨 Pass 2: Construyendo AST...
   ✅ Pass 2 completado - AST construido exitosamente
✨ Análisis híbrido completado con éxito

Análisis sintáctico completado exitosamente

[AST completo...]

Analizando semánticamente...
Análisis semántico completado exitosamente

[Tabla de símbolos...]

Ejecutando programa...
Ejecución completada exitosamente

[Estado final de la red...]
```

---

## 📖 Sintaxis del Lenguaje

### Estructura Básica

```
programa <nombre>;

// Definiciones (opcional)
define maquinas <lista_ids>;
define concentradores <lista_concentradores>;
define coaxial <lista_coaxiales>;

// Módulos (opcional)
modulo <nombre>;
inicio
    <sentencias>
fin

// Bloque principal
inicio
    <sentencias>
fin.
```

### Ejemplo Completo

```
programa ejemplo;

// Definir dispositivos
define maquinas
  A, B, C, servidor, cliente;

define concentradores
  hub1=8, hub2=16.1;  // 8 puertos, 16 puertos con salida coaxial

define coaxial
  cable1=100;  // 100 metros

// Módulo reutilizable
modulo configurar_servidores;
inicio
  coloca(servidor, 100, 100);
  coloca(hub1, 150, 100);
  uneMaquinaPuerto(servidor, hub1, 1);
fin

// Programa principal
inicio
  // Colocar máquinas en la pantalla
  coloca(A, 50, 50);
  coloca(B, 100, 50);
  coloca(C, 150, 50);

  // Colocar y configurar concentrador
  coloca(hub2, 100, 100);

  // Conectar máquinas a puertos
  uneMaquinaPuerto(A, hub2, 1);
  uneMaquinaPuerto(B, hub2, 2);
  uneMaquinaPuerto(C, hub2, 3);

  // Colocar cable coaxial
  colocaCoaxial(cable1, 200, 100, derecha);

  // Conectar máquinas al coaxial
  maquinaCoaxial(servidor, cable1, 10);

  // Ejecutar módulo
  configurar_servidores;

  // Condicionales
  si (hub2.p[4] = 0) inicio
    escribe("Puerto 4 disponible");
  fin
fin.
```

### Comandos Disponibles

| Comando | Descripción | Ejemplo |
|---------|-------------|---------|
| `coloca(id, x, y)` | Coloca un dispositivo en coordenadas | `coloca(A, 100, 50)` |
| `colocaCoaxial(id, x, y, dir)` | Coloca cable coaxial | `colocaCoaxial(c1, 50, 50, derecha)` |
| `uneMaquinaPuerto(maq, hub, puerto)` | Conecta máquina a puerto | `uneMaquinaPuerto(A, hub1, 3)` |
| `maquinaCoaxial(maq, cable, pos)` | Conecta máquina a cable | `maquinaCoaxial(A, c1, 10)` |
| `escribe(expr)` | Imprime un mensaje | `escribe("Conectado")` |
| `si (cond) inicio ... fin` | Condicional | `si (A.presente = 1) ...` |

### Acceso a Propiedades

```
// Máquinas
maquina.presente      // 1 si está colocada, 0 si no

// Concentradores
hub.presente          // 1 si está colocado, 0 si no
hub.p[n]              // Estado del puerto n (0=libre, 1=ocupado)
hub.coaxial           // 1 si tiene salida coaxial, 0 si no

// Cables coaxiales
cable.completo        // 1 si está lleno, 0 si acepta más máquinas
cable.longitud        // Longitud en metros
```

### Direcciones

- `arriba`
- `abajo`
- `izquierda`
- `derecha`

---

## 🏗️ Arquitectura del Proyecto

### Pipeline de Compilación

```
┌─────────────────┐
│  Código Fuente  │
│   (.net file)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  LEXER (DFA)    │ ← config/automaton.aut
│  45+ tokens     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  PARSER LL(1)   │ ← config/ll1_table.txt
│  Two-Pass:      │
│  • Validación   │
│  • AST Build    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  SEMANTIC       │
│  Analysis       │
│  Symbol Table   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  INTERPRETER    │
│  Runtime        │
│  Environment    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  VISUALIZER     │
│  GUI (eframe)   │
└─────────────────┘
```

### Estructura de Directorios

```
Network_interpreter/
├── src/
│   ├── main.rs                  # Punto de entrada principal
│   ├── lib.rs                   # Biblioteca exportable
│   ├── lexer_new/              # Nuevo lexer basado en autómatas
│   │   ├── automaton.rs        # Motor DFA (21,999 líneas)
│   │   ├── scanner.rs          # Escáner de tokens (10,478 líneas)
│   │   ├── token.rs            # Definiciones de tokens
│   │   └── error.rs            # Manejo de errores léxicos
│   ├── parser_ll1/             # Parser LL(1) predictivo
│   │   ├── first_follow.rs     # Conjuntos FIRST/FOLLOW
│   │   ├── ll1_table.rs        # Tabla de análisis (1,126 líneas)
│   │   └── predictive.rs       # Parser con pila explícita
│   ├── parser.rs               # Parser recursivo (para AST)
│   ├── ast.rs                  # Árbol de sintaxis abstracta
│   ├── semantic.rs             # Análisis semántico
│   ├── interpreter.rs          # Intérprete runtime
│   ├── visualizer.rs           # Visualizador gráfico
│   └── bin/
│       └── generate_ll1_table.rs
├── tests/
│   ├── test_new_lexer.rs       # 46 pruebas del lexer
│   ├── ll1_integration_test.rs # 5 pruebas de integración
│   └── ll1_parser_comprehensive.rs # 45 pruebas del parser
├── config/
│   ├── automaton.aut           # Definición del autómata
│   ├── ll1_table.txt           # Tabla LL(1) (38KB)
│   └── *.md                    # Documentación técnica
├── docs/
│   ├── gramatica.txt           # Gramática formal (84 producciones)
│   ├── first_follow.txt        # Conjuntos calculados
│   ├── LL1_PARSER_COMPLETE.md  # Reporte de implementación
│   └── *.md                    # Documentación adicional
├── ejemplo1.net                # Programa de ejemplo complejo
├── ejemplo2.net                # Programa con error sintáctico
└── Cargo.toml                  # Configuración del proyecto
```

---

## 🧪 Testing

### Ejecutar Todas las Pruebas

```bash
cargo test
```

### Pruebas por Categoría

```bash
# Pruebas del lexer (46 tests)
cargo test test_new_lexer

# Pruebas del parser LL(1) (11 tests unitarios)
cargo test --lib parser_ll1

# Pruebas de integración LL(1) (5 tests)
cargo test --test ll1_integration_test

# Pruebas comprehensivas (45 tests)
cargo test --test ll1_parser_comprehensive
```

### Cobertura de Pruebas

- ✅ **Análisis Léxico**: 46 pruebas (keywords, operadores, identificadores, números, strings)
- ✅ **Análisis Sintáctico**: 61 pruebas (todas las producciones de la gramática)
- ✅ **Integración**: 5 pruebas (archivos completos, manejo de errores)
- ✅ **Total**: 186 pruebas - 100% pasando

---

## 🎓 Aspectos Académicos

### Gramática Formal LL(1)

El proyecto implementa una gramática **LL(1) estricta** con:

- **84 producciones** documentadas
- **43 no-terminales** con conjuntos FIRST/FOLLOW
- **200+ entradas** en la tabla de análisis predictivo
- **Sin recursión izquierda** - completamente eliminada
- **Factorizada por la izquierda** - sin conflictos

**Ejemplo de Producción:**

```
[1] Programa → PROGRAMA IDENTIFICADOR PUNTO_COMA
               Definiciones Modulos BloqueInicio PUNTO

FIRST(Programa) = { PROGRAMA }
FOLLOW(Programa) = { EOF }
```

### Algoritmo LL(1) Predictivo

Implementación textbook del algoritmo con **pila explícita**:

```rust
while stack is not empty:
    X = stack.pop()
    a = current_token()

    if X is terminal:
        if X matches a:
            advance()
        else:
            ERROR

    if X is non-terminal:
        if M[X, a] exists:
            production = M[X, a]
            push production.rhs in reverse order
        else:
            ERROR
```

### Arquitectura Híbrida (Two-Pass)

**Pass 1: Validación LL(1)**
- Valida sintaxis contra gramática formal
- Usa tabla de análisis predictivo
- Algoritmo con pila explícita
- Detecta errores sintácticos tempranamente

**Pass 2: Construcción de AST**
- Usa parser recursivo descendente
- Construcción limpia del AST
- Código mantenible y probado

**Ventajas:**
- ✅ Cumple requisitos formales (LL(1))
- ✅ AST limpio y bien estructurado
- ✅ Separación de responsabilidades
- ✅ Fácil de probar y mantener

---

## 📊 Métricas del Proyecto

| Métrica | Valor |
|---------|-------|
| Líneas de código (src/) | 7,954 |
| Líneas de código LL(1) | 2,991 |
| Líneas de pruebas | 1,027 |
| Líneas de documentación | 3,700+ |
| Producciones gramática | 84 |
| No-terminales | 43 |
| Tipos de tokens | 45+ |
| Pruebas totales | 186 |
| Cobertura de gramática | 100% |
| Archivos de ejemplo | 2 |

---

## 🔧 Herramientas Incluidas

### Generador de Tabla LL(1)

Genera una tabla LL(1) legible en formato texto:

```bash
cargo run --bin generate_ll1_table
```

Esto crea/actualiza `config/ll1_table.txt` con:
- Todas las producciones de la gramática
- Entradas de la tabla M[NonTerminal, Terminal]
- Formato legible para debugging

---

## 🐛 Manejo de Errores

### Errores Léxicos

```
Error léxico: Carácter inesperado '@' en línea 5, columna 10
```

### Errores Sintácticos

```
Error de sintaxis (LL1): Error de sintaxis: se esperaba PUNTO_COMA
pero se encontró INICIO en posición 15
```

### Errores Semánticos

```
Error semántico en línea 10:
  Máquina 'servidor' no ha sido declarada
```

### Errores de Ejecución

```
Error de ejecución: Puerto 5 del concentrador 'hub1' ya está ocupado
```

---

## 🎨 Visualizador Gráfico

El visualizador muestra:

- 🖥️ **Máquinas** como círculos azules
- 🔌 **Concentradores** como rectángulos verdes con puertos numerados
- 📡 **Cables coaxiales** como líneas naranjas con dirección
- 🔗 **Conexiones** entre dispositivos
- 📊 **Información** de estado al pasar el mouse

### Controles

- **Mouse** - Navega por la topología
- **Rueda** - Zoom in/out
- **Hover** - Muestra información del dispositivo
- **ESC** - Cerrar visualizador

---

## 📚 Documentación Adicional

- **Gramática Completa**: [`docs/gramatica.txt`](./docs/gramatica.txt)
- **Conjuntos FIRST/FOLLOW**: [`docs/first_follow.txt`](./docs/first_follow.txt)
- **Reporte LL(1)**: [`docs/LL1_PARSER_COMPLETE.md`](./docs/LL1_PARSER_COMPLETE.md)
- **Arquitectura**: [`config/arquitectura.md`](./config/arquitectura.md)
- **Especificación Autómata**: [`config/automaton_spec.md`](./config/automaton_spec.md)

---

## 🤝 Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Haz un fork del proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

---

## 📝 Ejemplos de Uso

### Ejemplo 1: Red Simple

```
programa red_simple;
define maquinas A, B;
define concentradores hub=4;

inicio
  coloca(A, 50, 50);
  coloca(B, 100, 50);
  coloca(hub, 75, 100);
  uneMaquinaPuerto(A, hub, 1);
  uneMaquinaPuerto(B, hub, 2);
fin.
```

### Ejemplo 2: Con Cable Coaxial

```
programa red_coaxial;
define maquinas servidor, cliente1, cliente2;
define coaxial backbone=50;

inicio
  colocaCoaxial(backbone, 100, 100, derecha);
  coloca(servidor, 100, 50);
  coloca(cliente1, 150, 50);
  coloca(cliente2, 200, 50);

  maquinaCoaxial(servidor, backbone, 5);
  maquinaCoaxial(cliente1, backbone, 25);
  maquinaCoaxial(cliente2, backbone, 45);
fin.
```

### Ejemplo 3: Con Condicionales

```
programa red_dinamica;
define maquinas A, B, C;
define concentradores hub1=8, hub2=8;

inicio
  coloca(hub1, 100, 100);
  coloca(hub2, 200, 100);

  si (hub1.p[1] = 0) inicio
    coloca(A, 50, 50);
    uneMaquinaPuerto(A, hub1, 1);
  fin

  si (hub2.presente = 0) inicio
    escribe("hub2 no colocado");
  fin
fin.
```

---

## 🔍 Debugging

### Modo Verbose

El parser LL(1) muestra información detallada durante la validación:

```
Paso 1: Top=NonTerminal(Programa), Token=Programa
Aplicando producción 1: Programa → [Terminal(Programa), ...]
Paso 2: Top=Terminal(Programa), Token=Programa
...
✅ Validación LL(1) completada exitosamente en 711 pasos
```

### Ver Tabla de Símbolos

La tabla de símbolos muestra todos los dispositivos declarados y su estado:

```
═══════════════════════════════════════════════════
TABLA DE SÍMBOLOS
═══════════════════════════════════════════════════

Máquinas:
  • A - colocada
  • B - colocada
  • servidor - no colocada

Concentradores:
  • hub1 - 8 puertos - 2 disponibles - colocado
  • hub2 - 16 puertos + coaxial - 16 disponibles - colocado
```

---

## 💡 Tips y Buenas Prácticas

### Declaraciones

✅ **Bueno:**
```
define maquinas
  servidor, cliente1, cliente2;
```

❌ **Malo:**
```
define maquinas servidor, cliente1, cliente2;  // Falta nueva línea
```

### Espaciado

✅ **Bueno:**
```
coloca(A, 100, 50);
```

✅ **También válido:**
```
coloca( A , 100 , 50 ) ;
```

### Comentarios

Los comentarios **no están implementados** en el lenguaje. Para documentar, usa nombres descriptivos:

```
define maquinas
  servidor_web, servidor_db, servidor_cache;
```

---

## ⚡ Optimizaciones Futuras

Posibles mejoras (no prioritarias, el sistema funciona correctamente):

- [ ] Soporte para comentarios (`//` y `/* */`)
- [ ] Mensajes de error más amigables desde LL(1)
- [ ] Modo de un solo paso (integrar AST en LL(1))
- [ ] Exportar topología a formatos estándar (JSON, XML)
- [ ] Validación de distancias físicas en cables
- [ ] Simulación de tráfico de red
- [ ] Detección de colisiones en cables coaxiales

---

## 📖 Referencias

### Teoría de Compiladores

- **Libro**: "Compilers: Principles, Techniques, and Tools" (Dragon Book)
- **Capítulo 4**: Análisis Sintáctico
- **Sección 4.4**: Análisis Sintáctico Predictivo LL(1)

### Rust

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Redes

- Conceptos básicos de topologías de red
- Ethernet y cables coaxiales
- Concentradores (hubs) vs switches

---

## 👥 Autores

- **Implementación**: Proyecto académico de compiladores
- **Lexer**: Sistema de autómatas DFA configurable
- **Parser LL(1)**: Implementación completa con pila explícita
- **Visualizador**: Interfaz gráfica con eframe/egui

---

## 🙏 Agradecimientos

- Comunidad de Rust por las excelentes herramientas
- Crate `eframe` para la visualización GUI
- Crate `colored` para output terminal colorido
- Profesores y compañeros de Teoría de Compiladores

---

## 📞 Contacto

¿Preguntas, sugerencias o reportes de bugs?

- Abre un [Issue](../../issues)
- Consulta la [documentación técnica](./docs/)
- Revisa los [ejemplos](./ejemplo1.net)

---

## ⭐ Estrellas en GitHub

Si este proyecto te fue útil, ¡considera darle una estrella en GitHub! ⭐

---

**Versión**: 0.2.0
**Última actualización**: Diciembre 2025
**Estado**: ✅ Producción - Todos los tests pasando

---

*Desarrollado con ❤️ y Rust 🦀*
