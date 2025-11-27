// AST - Abstract Syntax Tree
// Representa la estructura sintáctica del programa

use crate::lexer::TokenInfo;

// ============================================================================
// UBICACIÓN EN EL CÓDIGO FUENTE
// ============================================================================

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl Location {
    pub fn from_token(token: &TokenInfo) -> Self {
        Self {
            line: token.line,
            column: token.column,
            length: token.length,
        }
    }

    pub fn unknown() -> Self {
        Self {
            line: 0,
            column: 0,
            length: 0,
        }
    }
}

// ============================================================================
// PROGRAMA COMPLETO
// ============================================================================

#[derive(Debug, Clone)]
pub struct Program {
    pub nombre: String,
    pub definiciones: Definitions,
    pub modulos: Vec<Modulo>,
    pub sentencias: Vec<Statement>,
    pub location: Location,
}

// ============================================================================
// DEFINICIONES
// ============================================================================

#[derive(Debug, Clone)]
pub struct Definitions {
    pub maquinas: Vec<MaquinaDecl>,
    pub concentradores: Vec<ConcentradorDecl>,
    pub coaxiales: Vec<CoaxialDecl>,
    pub location: Location,
}

impl Definitions {
    pub fn empty() -> Self {
        Self {
            maquinas: Vec::new(),
            concentradores: Vec::new(),
            coaxiales: Vec::new(),
            location: Location::unknown(),
        }
    }
}

// ============================================================================
// DECLARACIÓN DE MÁQUINA
// ============================================================================

#[derive(Debug, Clone)]
pub struct MaquinaDecl {
    pub nombre: String,
    pub location: Location,
}

// ============================================================================
// DECLARACIÓN DE CONCENTRADOR
// ============================================================================

#[derive(Debug, Clone)]
pub struct ConcentradorDecl {
    pub nombre: String,
    pub puertos: i32,
    pub tiene_coaxial: bool, // true si se declara con .1
    pub location: Location,
}

// ============================================================================
// DECLARACIÓN DE COAXIAL
// ============================================================================

#[derive(Debug, Clone)]
pub struct CoaxialDecl {
    pub nombre: String,
    pub longitud: i32,
    pub location: Location,
}

// ============================================================================
// MÓDULOS
// ============================================================================

#[derive(Debug, Clone)]
pub struct Modulo {
    pub nombre: String,
    pub sentencias: Vec<Statement>,
    pub location: Location,
}

// ============================================================================
// SENTENCIAS
// ============================================================================

#[derive(Debug, Clone)]
pub enum Statement {
    // coloca(objeto, x, y);
    Coloca {
        objeto: String,
        x: Expr,
        y: Expr,
        location: Location,
    },

    // colocaCoaxial(coaxial, x, y, direccion);
    ColocaCoaxial {
        coaxial: String,
        x: Expr,
        y: Expr,
        direccion: Direccion,
        location: Location,
    },

    // colocaCoaxialConcentrador(coaxial, concentrador);
    ColocaCoaxialConcentrador {
        coaxial: String,
        concentrador: String,
        location: Location,
    },

    // uneMaquinaPuerto(maquina, concentrador, puerto);
    UneMaquinaPuerto {
        maquina: String,
        concentrador: String,
        puerto: Expr,
        location: Location,
    },

    // asignaPuerto(maquina, concentrador);
    AsignaPuerto {
        maquina: String,
        concentrador: String,
        location: Location,
    },

    // maquinaCoaxial(maquina, coaxial, pos);
    MaquinaCoaxial {
        maquina: String,
        coaxial: String,
        posicion: Expr,
        location: Location,
    },

    // asignaMaquinaCoaxial(maquina, coaxial);
    AsignaMaquinaCoaxial {
        maquina: String,
        coaxial: String,
        location: Location,
    },

    // escribe(expr);
    Escribe {
        contenido: Expr,
        location: Location,
    },

    // si (condicion) inicio sentencias fin sino inicio sentencias fin
    Si {
        condicion: Expr,
        entonces: Vec<Statement>,
        sino: Option<Vec<Statement>>,
        location: Location,
    },

    // Llamada a módulo
    LlamadaModulo {
        nombre: String,
        location: Location,
    },
}

// ============================================================================
// EXPRESIONES
// ============================================================================

#[derive(Debug, Clone)]
pub enum Expr {
    // Literales
    Numero(i32),
    Cadena(String),
    Identificador(String),

    // Acceso a campos: obj.campo
    AccesoCampo {
        objeto: String,
        campo: String,
    },

    // Acceso a arreglo: obj[indice]
    AccesoArreglo {
        objeto: String,
        indice: Box<Expr>,
    },

    // Expresiones relacionales: a < b, a = b, etc.
    Relacional {
        izq: Box<Expr>,
        op: OpRelacional,
        der: Box<Expr>,
    },

    // Expresiones lógicas: a && b, a || b
    Logico {
        izq: Box<Expr>,
        op: OpLogico,
        der: Box<Expr>,
    },

    // Negación lógica: !expr
    Not(Box<Expr>),
}

// ============================================================================
// OPERADORES
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum OpRelacional {
    Igual,      // =
    Diferente,  // <>
    Menor,      // <
    Mayor,      // >
    MenorIgual, // <=
    MayorIgual, // >=
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpLogico {
    And, // &&
    Or,  // ||
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

// ============================================================================
// HELPER: Para imprimir el AST de manera legible
// ============================================================================

impl Program {
    pub fn pretty_print(&self) {
        use std::io::{self, Write};

        println!("\n{}", "═".repeat(80));
        println!("AST del Programa: {}", self.nombre);
        println!("{}", "═".repeat(80));

        self.definiciones.pretty_print();

        if !self.modulos.is_empty() {
            println!("\n Módulos definidos: {}", self.modulos.len());
            for (i, modulo) in self.modulos.iter().enumerate() {
                println!("   {}. modulo {} (línea {}) - {} sentencias",
                         i + 1, modulo.nombre, modulo.location.line, modulo.sentencias.len());
            }
        }

        if !self.sentencias.is_empty() {
            println!("\n Sentencias principales: {}", self.sentencias.len());
            for (i, stmt) in self.sentencias.iter().enumerate() {
                println!("   {}. {:?}", i + 1, format!("{:?}", stmt).chars().take(60).collect::<String>());
            }
        }

        println!("{}\n", "═".repeat(80));
        let _ = io::stdout().flush();
    }
}

impl Definitions {
    pub fn pretty_print(&self) {
        use std::io::{self, Write};

        if !self.maquinas.is_empty() {
            println!("\n Máquinas declaradas: {}", self.maquinas.len());
            for (i, maq) in self.maquinas.iter().enumerate() {
                println!("   {}. {} (línea {})", i + 1, maq.nombre, maq.location.line);
            }
        }

        if !self.concentradores.is_empty() {
            println!("\n Concentradores declarados: {}", self.concentradores.len());
            for (i, conc) in self.concentradores.iter().enumerate() {
                let coax_info = if conc.tiene_coaxial { " + coaxial" } else { "" };
                println!("   {}. {} = {} puertos{} (línea {})",
                         i + 1, conc.nombre, conc.puertos, coax_info, conc.location.line);
            }
        }

        if !self.coaxiales.is_empty() {
            println!("\n Cables coaxiales declarados: {}", self.coaxiales.len());
            for (i, coax) in self.coaxiales.iter().enumerate() {
                println!("   {}. {} = {}m (línea {})",
                         i + 1, coax.nombre, coax.longitud, coax.location.line);
            }
        }

        let _ = io::stdout().flush();
    }
}
