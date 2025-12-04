// src/parser_ll1/ll1_table.rs
// Tabla de análisis predictivo LL(1)

use std::collections::HashMap;
use crate::lexer::Token;
use super::first_follow::{Symbol, NonTerminal};

/// Representa una producción de la gramática
#[derive(Debug, Clone, PartialEq)]
pub struct Production {
    pub id: usize,
    pub lhs: NonTerminal,
    pub rhs: Vec<Symbol>,
}

/// Tabla de análisis predictivo LL(1)
/// M[NonTerminal, Terminal] = Production
pub struct LL1Table {
    table: HashMap<(NonTerminal, TokenClass), Production>,
    productions: Vec<Production>,
}

/// Clase de token para la tabla (ignora valores en Some variants)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenClass {
    Programa,
    Define,
    Maquinas,
    Concentradores,
    Coaxial,
    Segmento,
    Modulo,
    Inicio,
    Fin,
    Si,
    Sino,
    Coloca,
    ColocaCoaxial,
    ColocaCoaxialConcentrador,
    UneMaquinaPuerto,
    AsignaPuerto,
    MaquinaCoaxial,
    AsignaMaquinaCoaxial,
    Escribe,
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
    Igual,
    Menor,
    Mayor,
    MenorIgual,
    MayorIgual,
    Diferente,
    And,
    Or,
    Not,
    Coma,
    PuntoYComa,
    Punto,
    ParenIzq,
    ParenDer,
    CorcheteIzq,
    CorcheteDer,
    Identificador,
    Numero,
    Cadena,
    Eof,
}

impl TokenClass {
    /// Convierte un Token a TokenClass
    pub fn from_token(token: &Token) -> Self {
        match token {
            Token::Programa => TokenClass::Programa,
            Token::Define => TokenClass::Define,
            Token::Maquinas => TokenClass::Maquinas,
            Token::Concentradores => TokenClass::Concentradores,
            Token::Coaxial => TokenClass::Coaxial,
            Token::Segmento => TokenClass::Segmento,
            Token::Modulo => TokenClass::Modulo,
            Token::Inicio => TokenClass::Inicio,
            Token::Fin => TokenClass::Fin,
            Token::Si => TokenClass::Si,
            Token::Sino => TokenClass::Sino,
            Token::Coloca => TokenClass::Coloca,
            Token::ColocaCoaxial => TokenClass::ColocaCoaxial,
            Token::ColocaCoaxialConcentrador => TokenClass::ColocaCoaxialConcentrador,
            Token::UneMaquinaPuerto => TokenClass::UneMaquinaPuerto,
            Token::AsignaPuerto => TokenClass::AsignaPuerto,
            Token::MaquinaCoaxial => TokenClass::MaquinaCoaxial,
            Token::AsignaMaquinaCoaxial => TokenClass::AsignaMaquinaCoaxial,
            Token::Escribe => TokenClass::Escribe,
            Token::Arriba => TokenClass::Arriba,
            Token::Abajo => TokenClass::Abajo,
            Token::Izquierda => TokenClass::Izquierda,
            Token::Derecha => TokenClass::Derecha,
            Token::Igual => TokenClass::Igual,
            Token::Menor => TokenClass::Menor,
            Token::Mayor => TokenClass::Mayor,
            Token::MenorIgual => TokenClass::MenorIgual,
            Token::MayorIgual => TokenClass::MayorIgual,
            Token::Diferente => TokenClass::Diferente,
            Token::And => TokenClass::And,
            Token::Or => TokenClass::Or,
            Token::Not => TokenClass::Not,
            Token::Coma => TokenClass::Coma,
            Token::PuntoYComa => TokenClass::PuntoYComa,
            Token::Punto => TokenClass::Punto,
            Token::ParenIzq => TokenClass::ParenIzq,
            Token::ParenDer => TokenClass::ParenDer,
            Token::CorcheteIzq => TokenClass::CorcheteIzq,
            Token::CorcheteDer => TokenClass::CorcheteDer,
            Token::Identificador(_) => TokenClass::Identificador,
            Token::Numero(_) => TokenClass::Numero,
            Token::Cadena(_) => TokenClass::Cadena,
            Token::Whitespace => panic!("Whitespace should be filtered out"),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TokenClass::Programa => "programa",
            TokenClass::Define => "define",
            TokenClass::Maquinas => "maquinas",
            TokenClass::Concentradores => "concentradores",
            TokenClass::Coaxial => "coaxial",
            TokenClass::Segmento => "segmento",
            TokenClass::Modulo => "modulo",
            TokenClass::Inicio => "inicio",
            TokenClass::Fin => "fin",
            TokenClass::Si => "si",
            TokenClass::Sino => "sino",
            TokenClass::Coloca => "coloca",
            TokenClass::ColocaCoaxial => "colocaCoaxial",
            TokenClass::ColocaCoaxialConcentrador => "colocaCoaxialConcentrador",
            TokenClass::UneMaquinaPuerto => "uneMaquinaPuerto",
            TokenClass::AsignaPuerto => "asignaPuerto",
            TokenClass::MaquinaCoaxial => "maquinaCoaxial",
            TokenClass::AsignaMaquinaCoaxial => "asignaMaquinaCoaxial",
            TokenClass::Escribe => "escribe",
            TokenClass::Arriba => "arriba",
            TokenClass::Abajo => "abajo",
            TokenClass::Izquierda => "izquierda",
            TokenClass::Derecha => "derecha",
            TokenClass::Igual => "=",
            TokenClass::Menor => "<",
            TokenClass::Mayor => ">",
            TokenClass::MenorIgual => "<=",
            TokenClass::MayorIgual => ">=",
            TokenClass::Diferente => "<>",
            TokenClass::And => "&&",
            TokenClass::Or => "||",
            TokenClass::Not => "!",
            TokenClass::Coma => ",",
            TokenClass::PuntoYComa => ";",
            TokenClass::Punto => ".",
            TokenClass::ParenIzq => "(",
            TokenClass::ParenDer => ")",
            TokenClass::CorcheteIzq => "[",
            TokenClass::CorcheteDer => "]",
            TokenClass::Identificador => "identificador",
            TokenClass::Numero => "numero",
            TokenClass::Cadena => "cadena",
            TokenClass::Eof => "$",
        }
    }
}

impl LL1Table {
    /// Crea una nueva tabla LL(1)
    pub fn new() -> Self {
        let mut table = LL1Table {
            table: HashMap::new(),
            productions: Vec::new(),
        };

        table.initialize_productions();
        table.build_table();
        table
    }

    /// Inicializa todas las producciones de la gramática
    fn initialize_productions(&mut self) {
        use Symbol::{Terminal, Epsilon, NonTerminal as NT_Symbol};
        use Token::*;
        use NonTerminal as NT;

        // [1] Programa → PROGRAMA IDENTIFICADOR PUNTO_COMA Definiciones Modulos BloqueInicio PUNTO
        self.add_production(1, NT::Programa, vec![
            Terminal(Programa),
            Terminal(Identificador(String::new())),
            Terminal(PuntoYComa),
            NT_Symbol(NT::Definiciones),
            NT_Symbol(NT::Modulos),
            NT_Symbol(NT::BloqueInicio),
            Terminal(Punto),
        ]);

        // [2] Definiciones → DefMaquinas DefConcentradores DefCoaxiales
        self.add_production(2, NT::Definiciones, vec![
            NT_Symbol(NT::DefMaquinas),
            NT_Symbol(NT::DefConcentradores),
            NT_Symbol(NT::DefCoaxiales),
        ]);

        // [3] Definiciones → ε
        self.add_production(3, NT::Definiciones, vec![Epsilon]);

        // [4] DefMaquinas → DEFINE MAQUINAS ListaMaquinas PUNTO_COMA
        self.add_production(4, NT::DefMaquinas, vec![
            Terminal(Define),
            Terminal(Maquinas),
            NT_Symbol(NT::ListaMaquinas),
            Terminal(PuntoYComa),
        ]);

        // [5] DefMaquinas → ε
        self.add_production(5, NT::DefMaquinas, vec![Epsilon]);

        // [6] DefConcentradores → DEFINE CONCENTRADORES ListaConcentradores PUNTO_COMA
        self.add_production(6, NT::DefConcentradores, vec![
            Terminal(Define),
            Terminal(Concentradores),
            NT_Symbol(NT::ListaConcentradores),
            Terminal(PuntoYComa),
        ]);

        // [7] DefConcentradores → ε
        self.add_production(7, NT::DefConcentradores, vec![Epsilon]);

        // [8] DefCoaxiales → DEFINE TipoCoaxial ListaCoaxiales PUNTO_COMA
        self.add_production(8, NT::DefCoaxiales, vec![
            Terminal(Define),
            NT_Symbol(NT::TipoCoaxial),
            NT_Symbol(NT::ListaCoaxiales),
            Terminal(PuntoYComa),
        ]);

        // [9] DefCoaxiales → ε
        self.add_production(9, NT::DefCoaxiales, vec![Epsilon]);

        // [10] TipoCoaxial → COAXIAL
        self.add_production(10, NT::TipoCoaxial, vec![Terminal(Coaxial)]);

        // [11] TipoCoaxial → SEGMENTO
        self.add_production(11, NT::TipoCoaxial, vec![Terminal(Segmento)]);

        // [12] ListaMaquinas → IDENTIFICADOR ListaMaquinas'
        self.add_production(12, NT::ListaMaquinas, vec![
            Terminal(Identificador(String::new())),
            NT_Symbol(NT::ListaMaquinasPrime),
        ]);

        // [13] ListaMaquinas' → COMA IDENTIFICADOR ListaMaquinas'
        self.add_production(13, NT::ListaMaquinasPrime, vec![
            Terminal(Coma),
            Terminal(Identificador(String::new())),
            NT_Symbol(NT::ListaMaquinasPrime),
        ]);

        // [14] ListaMaquinas' → ε
        self.add_production(14, NT::ListaMaquinasPrime, vec![Epsilon]);

        // [15] ListaConcentradores → DeclConcentrador ListaConcentradores'
        self.add_production(15, NT::ListaConcentradores, vec![
            NT_Symbol(NT::DeclConcentrador),
            NT_Symbol(NT::ListaConcentradoresPrime),
        ]);

        // [16] ListaConcentradores' → COMA DeclConcentrador ListaConcentradores'
        self.add_production(16, NT::ListaConcentradoresPrime, vec![
            Terminal(Coma),
            NT_Symbol(NT::DeclConcentrador),
            NT_Symbol(NT::ListaConcentradoresPrime),
        ]);

        // [17] ListaConcentradores' → ε
        self.add_production(17, NT::ListaConcentradoresPrime, vec![Epsilon]);

        // [18] DeclConcentrador → IDENTIFICADOR IGUAL NUMERO OpcionCoaxial
        self.add_production(18, NT::DeclConcentrador, vec![
            Terminal(Identificador(String::new())),
            Terminal(Igual),
            Terminal(Numero(0)),
            NT_Symbol(NT::OpcionCoaxial),
        ]);

        // [19] OpcionCoaxial → PUNTO NUMERO
        self.add_production(19, NT::OpcionCoaxial, vec![
            Terminal(Punto),
            Terminal(Numero(0)),
        ]);

        // [20] OpcionCoaxial → ε
        self.add_production(20, NT::OpcionCoaxial, vec![Epsilon]);

        // [21] ListaCoaxiales → DeclCoaxial ListaCoaxiales'
        self.add_production(21, NT::ListaCoaxiales, vec![
            NT_Symbol(NT::DeclCoaxial),
            NT_Symbol(NT::ListaCoaxialesPrime),
        ]);

        // [22] ListaCoaxiales' → COMA DeclCoaxial ListaCoaxiales'
        self.add_production(22, NT::ListaCoaxialesPrime, vec![
            Terminal(Coma),
            NT_Symbol(NT::DeclCoaxial),
            NT_Symbol(NT::ListaCoaxialesPrime),
        ]);

        // [23] ListaCoaxiales' → ε
        self.add_production(23, NT::ListaCoaxialesPrime, vec![Epsilon]);

        // [24] DeclCoaxial → IDENTIFICADOR IGUAL NUMERO
        self.add_production(24, NT::DeclCoaxial, vec![
            Terminal(Identificador(String::new())),
            Terminal(Igual),
            Terminal(Numero(0)),
        ]);

        // [25] Modulos → Modulo Modulos
        self.add_production(25, NT::Modulos, vec![
            NT_Symbol(NT::Modulo),
            NT_Symbol(NT::Modulos),
        ]);

        // [26] Modulos → ε
        self.add_production(26, NT::Modulos, vec![Epsilon]);

        // [27] Modulo → MODULO IDENTIFICADOR PUNTO_COMA BloqueInicio
        self.add_production(27, NT::Modulo, vec![
            Terminal(Modulo),
            Terminal(Identificador(String::new())),
            Terminal(PuntoYComa),
            NT_Symbol(NT::BloqueInicio),
        ]);

        // [28] BloqueInicio → INICIO Sentencias FIN
        self.add_production(28, NT::BloqueInicio, vec![
            Terminal(Inicio),
            NT_Symbol(NT::Sentencias),
            Terminal(Fin),
        ]);

        // [29] Sentencias → Sentencia Sentencias
        self.add_production(29, NT::Sentencias, vec![
            NT_Symbol(NT::Sentencia),
            NT_Symbol(NT::Sentencias),
        ]);

        // [30] Sentencias → ε
        self.add_production(30, NT::Sentencias, vec![Epsilon]);

        // [31] Sentencia → SentenciaColoca
        self.add_production(31, NT::Sentencia, vec![NT_Symbol(NT::SentenciaColoca)]);

        // [32] Sentencia → SentenciaColocaCoaxial
        self.add_production(32, NT::Sentencia, vec![NT_Symbol(NT::SentenciaColocaCoaxial)]);

        // [33] Sentencia → SentenciaColocaCoaxialConcentrador
        self.add_production(33, NT::Sentencia, vec![NT_Symbol(NT::SentenciaColocaCoaxialConcentrador)]);

        // [34] Sentencia → SentenciaUneMaquinaPuerto
        self.add_production(34, NT::Sentencia, vec![NT_Symbol(NT::SentenciaUneMaquinaPuerto)]);

        // [35] Sentencia → SentenciaAsignaPuerto
        self.add_production(35, NT::Sentencia, vec![NT_Symbol(NT::SentenciaAsignaPuerto)]);

        // [36] Sentencia → SentenciaMaquinaCoaxial
        self.add_production(36, NT::Sentencia, vec![NT_Symbol(NT::SentenciaMaquinaCoaxial)]);

        // [37] Sentencia → SentenciaAsignaMaquinaCoaxial
        self.add_production(37, NT::Sentencia, vec![NT_Symbol(NT::SentenciaAsignaMaquinaCoaxial)]);

        // [38] Sentencia → SentenciaEscribe
        self.add_production(38, NT::Sentencia, vec![NT_Symbol(NT::SentenciaEscribe)]);

        // [39] Sentencia → SentenciaSi
        self.add_production(39, NT::Sentencia, vec![NT_Symbol(NT::SentenciaSi)]);

        // [40] Sentencia → LlamadaModulo
        self.add_production(40, NT::Sentencia, vec![NT_Symbol(NT::LlamadaModulo)]);

        // [41] SentenciaColoca → COLOCA PAREN_IZQ IDENTIFICADOR COMA Expresion COMA Expresion PAREN_DER PUNTO_COMA
        self.add_production(41, NT::SentenciaColoca, vec![
            Terminal(Coloca),
            Terminal(ParenIzq),
            Terminal(Identificador(String::new())),
            Terminal(Coma),
            NT_Symbol(NT::Expresion),
            Terminal(Coma),
            NT_Symbol(NT::Expresion),
            Terminal(ParenDer),
            Terminal(PuntoYComa),
        ]);

        // [42] SentenciaColocaCoaxial → COLOCA_COAXIAL PAREN_IZQ IDENTIFICADOR COMA Expresion COMA Expresion COMA Direccion PAREN_DER PUNTO_COMA
        self.add_production(42, NT::SentenciaColocaCoaxial, vec![
            Terminal(ColocaCoaxial),
            Terminal(ParenIzq),
            Terminal(Identificador(String::new())),
            Terminal(Coma),
            NT_Symbol(NT::Expresion),
            Terminal(Coma),
            NT_Symbol(NT::Expresion),
            Terminal(Coma),
            NT_Symbol(NT::Direccion),
            Terminal(ParenDer),
            Terminal(PuntoYComa),
        ]);

        // [43] SentenciaColocaCoaxialConcentrador → COLOCA_COAXIAL_CONCENTRADOR PAREN_IZQ IDENTIFICADOR COMA IDENTIFICADOR PAREN_DER PUNTO_COMA
        self.add_production(43, NT::SentenciaColocaCoaxialConcentrador, vec![
            Terminal(ColocaCoaxialConcentrador),
            Terminal(ParenIzq),
            Terminal(Identificador(String::new())),
            Terminal(Coma),
            Terminal(Identificador(String::new())),
            Terminal(ParenDer),
            Terminal(PuntoYComa),
        ]);

        // [44] SentenciaUneMaquinaPuerto → UNE_MAQUINA_PUERTO PAREN_IZQ IDENTIFICADOR COMA IDENTIFICADOR COMA Expresion PAREN_DER PUNTO_COMA
        self.add_production(44, NT::SentenciaUneMaquinaPuerto, vec![
            Terminal(UneMaquinaPuerto),
            Terminal(ParenIzq),
            Terminal(Identificador(String::new())),
            Terminal(Coma),
            Terminal(Identificador(String::new())),
            Terminal(Coma),
            NT_Symbol(NT::Expresion),
            Terminal(ParenDer),
            Terminal(PuntoYComa),
        ]);

        // [45] SentenciaAsignaPuerto → ASIGNA_PUERTO PAREN_IZQ IDENTIFICADOR COMA IDENTIFICADOR PAREN_DER PUNTO_COMA
        self.add_production(45, NT::SentenciaAsignaPuerto, vec![
            Terminal(AsignaPuerto),
            Terminal(ParenIzq),
            Terminal(Identificador(String::new())),
            Terminal(Coma),
            Terminal(Identificador(String::new())),
            Terminal(ParenDer),
            Terminal(PuntoYComa),
        ]);

        // [46] SentenciaMaquinaCoaxial → MAQUINA_COAXIAL PAREN_IZQ IDENTIFICADOR COMA IDENTIFICADOR COMA Expresion PAREN_DER PUNTO_COMA
        self.add_production(46, NT::SentenciaMaquinaCoaxial, vec![
            Terminal(MaquinaCoaxial),
            Terminal(ParenIzq),
            Terminal(Identificador(String::new())),
            Terminal(Coma),
            Terminal(Identificador(String::new())),
            Terminal(Coma),
            NT_Symbol(NT::Expresion),
            Terminal(ParenDer),
            Terminal(PuntoYComa),
        ]);

        // [47] SentenciaAsignaMaquinaCoaxial → ASIGNA_MAQUINA_COAXIAL PAREN_IZQ IDENTIFICADOR COMA IDENTIFICADOR PAREN_DER PUNTO_COMA
        self.add_production(47, NT::SentenciaAsignaMaquinaCoaxial, vec![
            Terminal(AsignaMaquinaCoaxial),
            Terminal(ParenIzq),
            Terminal(Identificador(String::new())),
            Terminal(Coma),
            Terminal(Identificador(String::new())),
            Terminal(ParenDer),
            Terminal(PuntoYComa),
        ]);

        // [48] SentenciaEscribe → ESCRIBE PAREN_IZQ Expresion PAREN_DER PUNTO_COMA
        self.add_production(48, NT::SentenciaEscribe, vec![
            Terminal(Escribe),
            Terminal(ParenIzq),
            NT_Symbol(NT::Expresion),
            Terminal(ParenDer),
            Terminal(PuntoYComa),
        ]);

        // [49] SentenciaSi → SI Expresion INICIO Sentencias FIN OpcionSino
        self.add_production(49, NT::SentenciaSi, vec![
            Terminal(Si),
            NT_Symbol(NT::Expresion),
            Terminal(Inicio),
            NT_Symbol(NT::Sentencias),
            Terminal(Fin),
            NT_Symbol(NT::OpcionSino),
        ]);

        // [50] OpcionSino → SINO INICIO Sentencias FIN
        self.add_production(50, NT::OpcionSino, vec![
            Terminal(Sino),
            Terminal(Inicio),
            NT_Symbol(NT::Sentencias),
            Terminal(Fin),
        ]);

        // [51] OpcionSino → ε
        self.add_production(51, NT::OpcionSino, vec![Epsilon]);

        // [52] LlamadaModulo → IDENTIFICADOR PUNTO_COMA
        self.add_production(52, NT::LlamadaModulo, vec![
            Terminal(Identificador(String::new())),
            Terminal(PuntoYComa),
        ]);

        // [53] Direccion → ARRIBA
        self.add_production(53, NT::Direccion, vec![Terminal(Arriba)]);

        // [54] Direccion → ABAJO
        self.add_production(54, NT::Direccion, vec![Terminal(Abajo)]);

        // [55] Direccion → IZQUIERDA
        self.add_production(55, NT::Direccion, vec![Terminal(Izquierda)]);

        // [56] Direccion → DERECHA
        self.add_production(56, NT::Direccion, vec![Terminal(Derecha)]);

        // [57] Expresion → ExpresionOr
        self.add_production(57, NT::Expresion, vec![NT_Symbol(NT::ExpresionOr)]);

        // [58] ExpresionOr → ExpresionAnd ExpresionOr'
        self.add_production(58, NT::ExpresionOr, vec![
            NT_Symbol(NT::ExpresionAnd),
            NT_Symbol(NT::ExpresionOrPrime),
        ]);

        // [59] ExpresionOr' → OR ExpresionAnd ExpresionOr'
        self.add_production(59, NT::ExpresionOrPrime, vec![
            Terminal(Or),
            NT_Symbol(NT::ExpresionAnd),
            NT_Symbol(NT::ExpresionOrPrime),
        ]);

        // [60] ExpresionOr' → ε
        self.add_production(60, NT::ExpresionOrPrime, vec![Epsilon]);

        // [61] ExpresionAnd → ExpresionRelacional ExpresionAnd'
        self.add_production(61, NT::ExpresionAnd, vec![
            NT_Symbol(NT::ExpresionRelacional),
            NT_Symbol(NT::ExpresionAndPrime),
        ]);

        // [62] ExpresionAnd' → AND ExpresionRelacional ExpresionAnd'
        self.add_production(62, NT::ExpresionAndPrime, vec![
            Terminal(And),
            NT_Symbol(NT::ExpresionRelacional),
            NT_Symbol(NT::ExpresionAndPrime),
        ]);

        // [63] ExpresionAnd' → ε
        self.add_production(63, NT::ExpresionAndPrime, vec![Epsilon]);

        // [64] ExpresionRelacional → ExpresionNot OpRelacional
        self.add_production(64, NT::ExpresionRelacional, vec![
            NT_Symbol(NT::ExpresionNot),
            NT_Symbol(NT::OpRelacional),
        ]);

        // [65] OpRelacional → OperadorRelacional ExpresionNot
        self.add_production(65, NT::OpRelacional, vec![
            NT_Symbol(NT::OperadorRelacional),
            NT_Symbol(NT::ExpresionNot),
        ]);

        // [66] OpRelacional → ε
        self.add_production(66, NT::OpRelacional, vec![Epsilon]);

        // [67] OperadorRelacional → IGUAL
        self.add_production(67, NT::OperadorRelacional, vec![Terminal(Igual)]);

        // [68] OperadorRelacional → DIFERENTE
        self.add_production(68, NT::OperadorRelacional, vec![Terminal(Diferente)]);

        // [69] OperadorRelacional → MENOR
        self.add_production(69, NT::OperadorRelacional, vec![Terminal(Menor)]);

        // [70] OperadorRelacional → MAYOR
        self.add_production(70, NT::OperadorRelacional, vec![Terminal(Mayor)]);

        // [71] OperadorRelacional → MENOR_IGUAL
        self.add_production(71, NT::OperadorRelacional, vec![Terminal(MenorIgual)]);

        // [72] OperadorRelacional → MAYOR_IGUAL
        self.add_production(72, NT::OperadorRelacional, vec![Terminal(MayorIgual)]);

        // [73] ExpresionNot → NOT ExpresionNot
        self.add_production(73, NT::ExpresionNot, vec![
            Terminal(Not),
            NT_Symbol(NT::ExpresionNot),
        ]);

        // [74] ExpresionNot → ExpresionPrimaria
        self.add_production(74, NT::ExpresionNot, vec![NT_Symbol(NT::ExpresionPrimaria)]);

        // [75] ExpresionPrimaria → NUMERO
        self.add_production(75, NT::ExpresionPrimaria, vec![Terminal(Numero(0))]);

        // [76] ExpresionPrimaria → CADENA
        self.add_production(76, NT::ExpresionPrimaria, vec![Terminal(Cadena(String::new()))]);

        // [77] ExpresionPrimaria → IDENTIFICADOR Accesos
        self.add_production(77, NT::ExpresionPrimaria, vec![
            Terminal(Identificador(String::new())),
            NT_Symbol(NT::Accesos),
        ]);

        // [78] ExpresionPrimaria → PAREN_IZQ Expresion PAREN_DER
        self.add_production(78, NT::ExpresionPrimaria, vec![
            Terminal(ParenIzq),
            NT_Symbol(NT::Expresion),
            Terminal(ParenDer),
        ]);

        // [79] Accesos → AccesoCampo
        self.add_production(79, NT::Accesos, vec![NT_Symbol(NT::AccesoCampo)]);

        // [80] Accesos → AccesoArreglo
        self.add_production(80, NT::Accesos, vec![NT_Symbol(NT::AccesoArreglo)]);

        // [81] Accesos → ε
        self.add_production(81, NT::Accesos, vec![Epsilon]);

        // [82] AccesoCampo → PUNTO IDENTIFICADOR AccesoArreglo
        self.add_production(82, NT::AccesoCampo, vec![
            Terminal(Punto),
            Terminal(Identificador(String::new())),
            NT_Symbol(NT::AccesoArreglo),
        ]);

        // [83] AccesoArreglo → CORCHETE_IZQ Expresion CORCHETE_DER
        self.add_production(83, NT::AccesoArreglo, vec![
            Terminal(CorcheteIzq),
            NT_Symbol(NT::Expresion),
            Terminal(CorcheteDer),
        ]);

        // [84] AccesoArreglo → ε
        self.add_production(84, NT::AccesoArreglo, vec![Epsilon]);
    }

    /// Añade una producción
    fn add_production(&mut self, id: usize, lhs: NonTerminal, rhs: Vec<Symbol>) {
        self.productions.push(Production { id, lhs, rhs });
    }

    /// Construye la tabla de análisis predictivo LL(1) completa
    /// Para cada no-terminal A y terminal a:
    ///   Si a ∈ FIRST(α), entonces M[A, a] = A → α
    ///   Si ε ∈ FIRST(α) y a ∈ FOLLOW(A), entonces M[A, a] = A → α
    fn build_table(&mut self) {
        use NonTerminal as NT;
        use TokenClass::*;

        // [1] Programa → PROGRAMA IDENTIFICADOR ; Definiciones Modulos BloqueInicio .
        self.add_entry(NT::Programa, TokenClass::Programa, 1);

        // [2] Definiciones → DefMaquinas DefConcentradores DefCoaxiales (cuando FIRST = DEFINE)
        // [3] Definiciones → ε (cuando FOLLOW = MODULO, INICIO)
        self.add_entry(NT::Definiciones, Define, 2);
        self.add_entry(NT::Definiciones, Modulo, 3);
        self.add_entry(NT::Definiciones, Inicio, 3);

        // [4] DefMaquinas → DEFINE MAQUINAS ListaMaquinas ;
        // [5] DefMaquinas → ε (FOLLOW = DEFINE, MODULO, INICIO)
        self.add_entry(NT::DefMaquinas, Define, 4);
        self.add_entry(NT::DefMaquinas, Modulo, 5);
        self.add_entry(NT::DefMaquinas, Inicio, 5);

        // [6] DefConcentradores → DEFINE CONCENTRADORES ListaConcentradores ;
        // [7] DefConcentradores → ε (FOLLOW = DEFINE, MODULO, INICIO)
        self.add_entry(NT::DefConcentradores, Define, 6);
        self.add_entry(NT::DefConcentradores, Modulo, 7);
        self.add_entry(NT::DefConcentradores, Inicio, 7);

        // [8] DefCoaxiales → DEFINE TipoCoaxial ListaCoaxiales ;
        // [9] DefCoaxiales → ε (FOLLOW = MODULO, INICIO)
        self.add_entry(NT::DefCoaxiales, Define, 8);
        self.add_entry(NT::DefCoaxiales, Modulo, 9);
        self.add_entry(NT::DefCoaxiales, Inicio, 9);

        // [10] TipoCoaxial → COAXIAL
        // [11] TipoCoaxial → SEGMENTO
        self.add_entry(NT::TipoCoaxial, Coaxial, 10);
        self.add_entry(NT::TipoCoaxial, Segmento, 11);

        // [12] ListaMaquinas → IDENTIFICADOR ListaMaquinas'
        self.add_entry(NT::ListaMaquinas, Identificador, 12);

        // [13] ListaMaquinas' → , IDENTIFICADOR ListaMaquinas'
        // [14] ListaMaquinas' → ε (FOLLOW = ;)
        self.add_entry(NT::ListaMaquinasPrime, Coma, 13);
        self.add_entry(NT::ListaMaquinasPrime, PuntoYComa, 14);

        // [15] ListaConcentradores → DeclConcentrador ListaConcentradores'
        self.add_entry(NT::ListaConcentradores, Identificador, 15);

        // [16] ListaConcentradores' → , DeclConcentrador ListaConcentradores'
        // [17] ListaConcentradores' → ε (FOLLOW = ;)
        self.add_entry(NT::ListaConcentradoresPrime, Coma, 16);
        self.add_entry(NT::ListaConcentradoresPrime, PuntoYComa, 17);

        // [18] DeclConcentrador → IDENTIFICADOR = NUMERO OpcionCoaxial
        self.add_entry(NT::DeclConcentrador, Identificador, 18);

        // [19] OpcionCoaxial → . NUMERO
        // [20] OpcionCoaxial → ε (FOLLOW = , ;)
        self.add_entry(NT::OpcionCoaxial, Punto, 19);
        self.add_entry(NT::OpcionCoaxial, Coma, 20);
        self.add_entry(NT::OpcionCoaxial, PuntoYComa, 20);

        // [21] ListaCoaxiales → DeclCoaxial ListaCoaxiales'
        self.add_entry(NT::ListaCoaxiales, Identificador, 21);

        // [22] ListaCoaxiales' → , DeclCoaxial ListaCoaxiales'
        // [23] ListaCoaxiales' → ε (FOLLOW = ;)
        self.add_entry(NT::ListaCoaxialesPrime, Coma, 22);
        self.add_entry(NT::ListaCoaxialesPrime, PuntoYComa, 23);

        // [24] DeclCoaxial → IDENTIFICADOR = NUMERO
        self.add_entry(NT::DeclCoaxial, Identificador, 24);

        // [25] Modulos → Modulo Modulos
        // [26] Modulos → ε (FOLLOW = INICIO)
        self.add_entry(NT::Modulos, Modulo, 25);
        self.add_entry(NT::Modulos, Inicio, 26);

        // [27] Modulo → MODULO IDENTIFICADOR ; BloqueInicio
        self.add_entry(NT::Modulo, TokenClass::Modulo, 27);

        // [28] BloqueInicio → INICIO Sentencias FIN
        self.add_entry(NT::BloqueInicio, Inicio, 28);

        // [29] Sentencias → Sentencia Sentencias
        // [30] Sentencias → ε (FOLLOW = FIN)
        self.add_entry(NT::Sentencias, Coloca, 29);
        self.add_entry(NT::Sentencias, ColocaCoaxial, 29);
        self.add_entry(NT::Sentencias, ColocaCoaxialConcentrador, 29);
        self.add_entry(NT::Sentencias, UneMaquinaPuerto, 29);
        self.add_entry(NT::Sentencias, AsignaPuerto, 29);
        self.add_entry(NT::Sentencias, MaquinaCoaxial, 29);
        self.add_entry(NT::Sentencias, AsignaMaquinaCoaxial, 29);
        self.add_entry(NT::Sentencias, Escribe, 29);
        self.add_entry(NT::Sentencias, Si, 29);
        self.add_entry(NT::Sentencias, Identificador, 29);
        self.add_entry(NT::Sentencias, Fin, 30);

        // [31-40] Sentencia → variantes (según FIRST de cada sentencia específica)
        self.add_entry(NT::Sentencia, Coloca, 31);
        self.add_entry(NT::Sentencia, ColocaCoaxial, 32);
        self.add_entry(NT::Sentencia, ColocaCoaxialConcentrador, 33);
        self.add_entry(NT::Sentencia, UneMaquinaPuerto, 34);
        self.add_entry(NT::Sentencia, AsignaPuerto, 35);
        self.add_entry(NT::Sentencia, MaquinaCoaxial, 36);
        self.add_entry(NT::Sentencia, AsignaMaquinaCoaxial, 37);
        self.add_entry(NT::Sentencia, Escribe, 38);
        self.add_entry(NT::Sentencia, Si, 39);
        self.add_entry(NT::Sentencia, Identificador, 40);

        // [41] SentenciaColoca → COLOCA ( IDENTIFICADOR , Expresion , Expresion ) ;
        self.add_entry(NT::SentenciaColoca, Coloca, 41);

        // [42] SentenciaColocaCoaxial → COLOCA_COAXIAL ( IDENTIFICADOR , Expresion , Expresion , Direccion ) ;
        self.add_entry(NT::SentenciaColocaCoaxial, ColocaCoaxial, 42);

        // [43] SentenciaColocaCoaxialConcentrador → COLOCA_COAXIAL_CONCENTRADOR ( IDENTIFICADOR , IDENTIFICADOR ) ;
        self.add_entry(NT::SentenciaColocaCoaxialConcentrador, ColocaCoaxialConcentrador, 43);

        // [44] SentenciaUneMaquinaPuerto → UNE_MAQUINA_PUERTO ( IDENTIFICADOR , IDENTIFICADOR , Expresion ) ;
        self.add_entry(NT::SentenciaUneMaquinaPuerto, UneMaquinaPuerto, 44);

        // [45] SentenciaAsignaPuerto → ASIGNA_PUERTO ( IDENTIFICADOR , IDENTIFICADOR ) ;
        self.add_entry(NT::SentenciaAsignaPuerto, AsignaPuerto, 45);

        // [46] SentenciaMaquinaCoaxial → MAQUINA_COAXIAL ( IDENTIFICADOR , IDENTIFICADOR , Expresion ) ;
        self.add_entry(NT::SentenciaMaquinaCoaxial, MaquinaCoaxial, 46);

        // [47] SentenciaAsignaMaquinaCoaxial → ASIGNA_MAQUINA_COAXIAL ( IDENTIFICADOR , IDENTIFICADOR ) ;
        self.add_entry(NT::SentenciaAsignaMaquinaCoaxial, AsignaMaquinaCoaxial, 47);

        // [48] SentenciaEscribe → ESCRIBE ( Expresion ) ;
        self.add_entry(NT::SentenciaEscribe, Escribe, 48);

        // [49] SentenciaSi → SI Expresion INICIO Sentencias FIN OpcionSino
        self.add_entry(NT::SentenciaSi, Si, 49);

        // [50] OpcionSino → SINO INICIO Sentencias FIN
        // [51] OpcionSino → ε (FOLLOW = statement followers + FIN)
        self.add_entry(NT::OpcionSino, Sino, 50);
        self.add_entry(NT::OpcionSino, Coloca, 51);
        self.add_entry(NT::OpcionSino, ColocaCoaxial, 51);
        self.add_entry(NT::OpcionSino, ColocaCoaxialConcentrador, 51);
        self.add_entry(NT::OpcionSino, UneMaquinaPuerto, 51);
        self.add_entry(NT::OpcionSino, AsignaPuerto, 51);
        self.add_entry(NT::OpcionSino, MaquinaCoaxial, 51);
        self.add_entry(NT::OpcionSino, AsignaMaquinaCoaxial, 51);
        self.add_entry(NT::OpcionSino, Escribe, 51);
        self.add_entry(NT::OpcionSino, Si, 51);
        self.add_entry(NT::OpcionSino, Identificador, 51);
        self.add_entry(NT::OpcionSino, Fin, 51);

        // [52] LlamadaModulo → IDENTIFICADOR ;
        self.add_entry(NT::LlamadaModulo, Identificador, 52);

        // [53-56] Direccion → ARRIBA | ABAJO | IZQUIERDA | DERECHA
        self.add_entry(NT::Direccion, Arriba, 53);
        self.add_entry(NT::Direccion, Abajo, 54);
        self.add_entry(NT::Direccion, Izquierda, 55);
        self.add_entry(NT::Direccion, Derecha, 56);

        // [57] Expresion → ExpresionOr (FIRST = NOT, NUMERO, CADENA, IDENTIFICADOR, ()
        self.add_entry(NT::Expresion, Not, 57);
        self.add_entry(NT::Expresion, Numero, 57);
        self.add_entry(NT::Expresion, Cadena, 57);
        self.add_entry(NT::Expresion, Identificador, 57);
        self.add_entry(NT::Expresion, ParenIzq, 57);

        // [58] ExpresionOr → ExpresionAnd ExpresionOr'
        self.add_entry(NT::ExpresionOr, Not, 58);
        self.add_entry(NT::ExpresionOr, Numero, 58);
        self.add_entry(NT::ExpresionOr, Cadena, 58);
        self.add_entry(NT::ExpresionOr, Identificador, 58);
        self.add_entry(NT::ExpresionOr, ParenIzq, 58);

        // [59] ExpresionOr' → || ExpresionAnd ExpresionOr'
        // [60] ExpresionOr' → ε (FOLLOW = ), ,, ], ;, INICIO)
        self.add_entry(NT::ExpresionOrPrime, Or, 59);
        self.add_entry(NT::ExpresionOrPrime, ParenDer, 60);
        self.add_entry(NT::ExpresionOrPrime, Coma, 60);
        self.add_entry(NT::ExpresionOrPrime, CorcheteDer, 60);
        self.add_entry(NT::ExpresionOrPrime, PuntoYComa, 60);
        self.add_entry(NT::ExpresionOrPrime, Inicio, 60);

        // [61] ExpresionAnd → ExpresionRelacional ExpresionAnd'
        self.add_entry(NT::ExpresionAnd, Not, 61);
        self.add_entry(NT::ExpresionAnd, Numero, 61);
        self.add_entry(NT::ExpresionAnd, Cadena, 61);
        self.add_entry(NT::ExpresionAnd, Identificador, 61);
        self.add_entry(NT::ExpresionAnd, ParenIzq, 61);

        // [62] ExpresionAnd' → && ExpresionRelacional ExpresionAnd'
        // [63] ExpresionAnd' → ε (FOLLOW includes ||)
        self.add_entry(NT::ExpresionAndPrime, And, 62);
        self.add_entry(NT::ExpresionAndPrime, Or, 63);
        self.add_entry(NT::ExpresionAndPrime, ParenDer, 63);
        self.add_entry(NT::ExpresionAndPrime, Coma, 63);
        self.add_entry(NT::ExpresionAndPrime, CorcheteDer, 63);
        self.add_entry(NT::ExpresionAndPrime, PuntoYComa, 63);
        self.add_entry(NT::ExpresionAndPrime, Inicio, 63);

        // [64] ExpresionRelacional → ExpresionNot OpRelacional
        self.add_entry(NT::ExpresionRelacional, Not, 64);
        self.add_entry(NT::ExpresionRelacional, Numero, 64);
        self.add_entry(NT::ExpresionRelacional, Cadena, 64);
        self.add_entry(NT::ExpresionRelacional, Identificador, 64);
        self.add_entry(NT::ExpresionRelacional, ParenIzq, 64);

        // [65] OpRelacional → OperadorRelacional ExpresionNot
        // [66] OpRelacional → ε (FOLLOW includes &&, ||, etc.)
        self.add_entry(NT::OpRelacional, Igual, 65);
        self.add_entry(NT::OpRelacional, Diferente, 65);
        self.add_entry(NT::OpRelacional, Menor, 65);
        self.add_entry(NT::OpRelacional, Mayor, 65);
        self.add_entry(NT::OpRelacional, MenorIgual, 65);
        self.add_entry(NT::OpRelacional, MayorIgual, 65);
        self.add_entry(NT::OpRelacional, And, 66);
        self.add_entry(NT::OpRelacional, Or, 66);
        self.add_entry(NT::OpRelacional, ParenDer, 66);
        self.add_entry(NT::OpRelacional, Coma, 66);
        self.add_entry(NT::OpRelacional, CorcheteDer, 66);
        self.add_entry(NT::OpRelacional, PuntoYComa, 66);
        self.add_entry(NT::OpRelacional, Inicio, 66);

        // [67-72] OperadorRelacional → = | <> | < | > | <= | >=
        self.add_entry(NT::OperadorRelacional, Igual, 67);
        self.add_entry(NT::OperadorRelacional, Diferente, 68);
        self.add_entry(NT::OperadorRelacional, Menor, 69);
        self.add_entry(NT::OperadorRelacional, Mayor, 70);
        self.add_entry(NT::OperadorRelacional, MenorIgual, 71);
        self.add_entry(NT::OperadorRelacional, MayorIgual, 72);

        // [73] ExpresionNot → ! ExpresionNot
        // [74] ExpresionNot → ExpresionPrimaria
        self.add_entry(NT::ExpresionNot, Not, 73);
        self.add_entry(NT::ExpresionNot, Numero, 74);
        self.add_entry(NT::ExpresionNot, Cadena, 74);
        self.add_entry(NT::ExpresionNot, Identificador, 74);
        self.add_entry(NT::ExpresionNot, ParenIzq, 74);

        // [75] ExpresionPrimaria → NUMERO
        // [76] ExpresionPrimaria → CADENA
        // [77] ExpresionPrimaria → IDENTIFICADOR Accesos
        // [78] ExpresionPrimaria → ( Expresion )
        self.add_entry(NT::ExpresionPrimaria, Numero, 75);
        self.add_entry(NT::ExpresionPrimaria, Cadena, 76);
        self.add_entry(NT::ExpresionPrimaria, Identificador, 77);
        self.add_entry(NT::ExpresionPrimaria, ParenIzq, 78);

        // [79] Accesos → AccesoCampo
        // [80] Accesos → AccesoArreglo
        // [81] Accesos → ε (FOLLOW = relational ops + expression followers)
        self.add_entry(NT::Accesos, Punto, 79);
        self.add_entry(NT::Accesos, CorcheteIzq, 80);
        self.add_entry(NT::Accesos, Igual, 81);
        self.add_entry(NT::Accesos, Diferente, 81);
        self.add_entry(NT::Accesos, Menor, 81);
        self.add_entry(NT::Accesos, Mayor, 81);
        self.add_entry(NT::Accesos, MenorIgual, 81);
        self.add_entry(NT::Accesos, MayorIgual, 81);
        self.add_entry(NT::Accesos, And, 81);
        self.add_entry(NT::Accesos, Or, 81);
        self.add_entry(NT::Accesos, ParenDer, 81);
        self.add_entry(NT::Accesos, Coma, 81);
        self.add_entry(NT::Accesos, CorcheteDer, 81);
        self.add_entry(NT::Accesos, PuntoYComa, 81);
        self.add_entry(NT::Accesos, Inicio, 81);

        // [82] AccesoCampo → . IDENTIFICADOR AccesoArreglo
        self.add_entry(NT::AccesoCampo, Punto, 82);

        // [83] AccesoArreglo → [ Expresion ]
        // [84] AccesoArreglo → ε (FOLLOW = same as Accesos)
        self.add_entry(NT::AccesoArreglo, CorcheteIzq, 83);
        self.add_entry(NT::AccesoArreglo, Igual, 84);
        self.add_entry(NT::AccesoArreglo, Diferente, 84);
        self.add_entry(NT::AccesoArreglo, Menor, 84);
        self.add_entry(NT::AccesoArreglo, Mayor, 84);
        self.add_entry(NT::AccesoArreglo, MenorIgual, 84);
        self.add_entry(NT::AccesoArreglo, MayorIgual, 84);
        self.add_entry(NT::AccesoArreglo, And, 84);
        self.add_entry(NT::AccesoArreglo, Or, 84);
        self.add_entry(NT::AccesoArreglo, ParenDer, 84);
        self.add_entry(NT::AccesoArreglo, Coma, 84);
        self.add_entry(NT::AccesoArreglo, CorcheteDer, 84);
        self.add_entry(NT::AccesoArreglo, PuntoYComa, 84);
        self.add_entry(NT::AccesoArreglo, Inicio, 84);
    }

    /// Añade una entrada a la tabla
    fn add_entry(&mut self, nt: NonTerminal, tc: TokenClass, prod_id: usize) {
        let prod = self.productions.iter()
            .find(|p| p.id == prod_id)
            .expect(&format!("Production {} not found", prod_id))
            .clone();

        self.table.insert((nt, tc), prod);
    }

    /// Consulta la tabla LL(1)
    pub fn get(&self, nt: NonTerminal, token: &Token) -> Option<&Production> {
        let tc = TokenClass::from_token(token);
        self.table.get(&(nt, tc))
    }

    /// Obtiene una producción por ID
    pub fn get_production(&self, id: usize) -> Option<&Production> {
        self.productions.iter().find(|p| p.id == id)
    }

    /// Exporta la tabla LL(1) en formato legible para humanos
    pub fn export_table(&self) -> String {
        use std::fmt::Write;
        let mut output = String::new();

        writeln!(&mut output, "════════════════════════════════════════════════════════════════════════").unwrap();
        writeln!(&mut output, "TABLA DE ANÁLISIS PREDICTIVO LL(1)").unwrap();
        writeln!(&mut output, "════════════════════════════════════════════════════════════════════════").unwrap();
        writeln!(&mut output).unwrap();
        writeln!(&mut output, "Esta tabla define la acción del parser para cada combinación de:").unwrap();
        writeln!(&mut output, "  - No-Terminal (fila): símbolo en el tope de la pila").unwrap();
        writeln!(&mut output, "  - Terminal (columna): token actual del input").unwrap();
        writeln!(&mut output).unwrap();
        writeln!(&mut output, "Formato: M[NoTerminal, Terminal] = Producción").unwrap();
        writeln!(&mut output).unwrap();
        writeln!(&mut output, "Total de entradas: {}", self.table.len()).unwrap();
        writeln!(&mut output, "Total de producciones: {}", self.productions.len()).unwrap();
        writeln!(&mut output, "════════════════════════════════════════════════════════════════════════").unwrap();
        writeln!(&mut output).unwrap();

        // Collect all entries and sort them
        let mut entries: Vec<_> = self.table.iter().collect();
        entries.sort_by_key(|(k, v)| (k.0.clone(), format!("{:?}", k.1), v.id));

        let mut current_nt: Option<NonTerminal> = None;

        for ((nt, tc), prod) in entries {
            // Print header when changing non-terminal
            if current_nt.as_ref() != Some(nt) {
                if current_nt.is_some() {
                    writeln!(&mut output).unwrap();
                }
                writeln!(&mut output, "────────────────────────────────────────────────────────────────────────").unwrap();
                writeln!(&mut output, "No-Terminal: {}", nt.as_str()).unwrap();
                writeln!(&mut output, "────────────────────────────────────────────────────────────────────────").unwrap();
                current_nt = Some(nt.clone());
            }

            // Format production RHS
            let rhs_str = Self::format_rhs(&prod.rhs);

            // Print table entry
            writeln!(&mut output, "  M[{}, {:?}] = [{}] {} → {}",
                nt.as_str(),
                tc,
                prod.id,
                prod.lhs.as_str(),
                rhs_str
            ).unwrap();
        }

        writeln!(&mut output).unwrap();
        writeln!(&mut output, "════════════════════════════════════════════════════════════════════════").unwrap();
        writeln!(&mut output, "LISTA COMPLETA DE PRODUCCIONES").unwrap();
        writeln!(&mut output, "════════════════════════════════════════════════════════════════════════").unwrap();
        writeln!(&mut output).unwrap();

        let mut sorted_prods = self.productions.clone();
        sorted_prods.sort_by_key(|p| p.id);

        for prod in sorted_prods {
            let rhs_str = Self::format_rhs(&prod.rhs);
            writeln!(&mut output, "[{:2}] {} → {}",
                prod.id,
                prod.lhs.as_str(),
                rhs_str
            ).unwrap();
        }

        writeln!(&mut output).unwrap();
        writeln!(&mut output, "════════════════════════════════════════════════════════════════════════").unwrap();

        output
    }

    /// Formatea el lado derecho de una producción para visualización
    fn format_rhs(rhs: &[Symbol]) -> String {
        if rhs.is_empty() || (rhs.len() == 1 && rhs[0] == Symbol::Epsilon) {
            return "ε".to_string();
        }

        rhs.iter()
            .map(|sym| match sym {
                Symbol::Terminal(t) => Self::format_terminal(t),
                Symbol::NonTerminal(nt) => nt.as_str().to_string(),
                Symbol::Epsilon => "ε".to_string(),
                Symbol::Eof => "$".to_string(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Formatea un terminal para visualización
    fn format_terminal(token: &Token) -> String {
        match token {
            Token::Programa => "PROGRAMA".to_string(),
            Token::Define => "DEFINE".to_string(),
            Token::Maquinas => "MAQUINAS".to_string(),
            Token::Concentradores => "CONCENTRADORES".to_string(),
            Token::Coaxial => "COAXIAL".to_string(),
            Token::Segmento => "SEGMENTO".to_string(),
            Token::Modulo => "MODULO".to_string(),
            Token::Inicio => "INICIO".to_string(),
            Token::Fin => "FIN".to_string(),
            Token::Si => "SI".to_string(),
            Token::Sino => "SINO".to_string(),
            Token::Coloca => "coloca".to_string(),
            Token::ColocaCoaxial => "colocaCoaxial".to_string(),
            Token::ColocaCoaxialConcentrador => "colocaCoaxialConcentrador".to_string(),
            Token::UneMaquinaPuerto => "uneMaquinaPuerto".to_string(),
            Token::AsignaPuerto => "asignaPuerto".to_string(),
            Token::MaquinaCoaxial => "maquinaCoaxial".to_string(),
            Token::AsignaMaquinaCoaxial => "asignaMaquinaCoaxial".to_string(),
            Token::Escribe => "escribe".to_string(),
            Token::Arriba => "arriba".to_string(),
            Token::Abajo => "abajo".to_string(),
            Token::Izquierda => "izquierda".to_string(),
            Token::Derecha => "derecha".to_string(),
            Token::Igual => "=".to_string(),
            Token::Menor => "<".to_string(),
            Token::Mayor => ">".to_string(),
            Token::MenorIgual => "<=".to_string(),
            Token::MayorIgual => ">=".to_string(),
            Token::Diferente => "<>".to_string(),
            Token::And => "&&".to_string(),
            Token::Or => "||".to_string(),
            Token::Not => "!".to_string(),
            Token::Coma => ",".to_string(),
            Token::PuntoYComa => ";".to_string(),
            Token::Punto => ".".to_string(),
            Token::ParenIzq => "(".to_string(),
            Token::ParenDer => ")".to_string(),
            Token::CorcheteIzq => "[".to_string(),
            Token::CorcheteDer => "]".to_string(),
            Token::Identificador(_) => "IDENTIFICADOR".to_string(),
            Token::Numero(_) => "NUMERO".to_string(),
            Token::Cadena(_) => "CADENA".to_string(),
            Token::Whitespace => "WHITESPACE".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = LL1Table::new();
        assert!(!table.productions.is_empty());
    }

    #[test]
    fn test_token_class_conversion() {
        let token = Token::Programa;
        let tc = TokenClass::from_token(&token);
        assert_eq!(tc, TokenClass::Programa);
    }

    #[test]
    fn test_table_lookup() {
        let table = LL1Table::new();
        let prod = table.get(NonTerminal::Programa, &Token::Programa);
        assert!(prod.is_some());
        assert_eq!(prod.unwrap().id, 1);
    }
}
