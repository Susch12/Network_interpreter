// src/parser_ll1/first_follow.rs
// Cálculo de conjuntos FIRST y FOLLOW para gramática LL(1)

use std::collections::{HashMap, HashSet};
use crate::lexer::Token;

/// Representa un símbolo de la gramática (terminal o no-terminal)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Terminal(Token),
    NonTerminal(NonTerminal),
    Epsilon,
    Eof,
}

/// No-terminales de la gramática
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NonTerminal {
    Programa,
    Definiciones,
    DefMaquinas,
    DefConcentradores,
    DefCoaxiales,
    TipoCoaxial,
    ListaMaquinas,
    ListaMaquinasPrime,
    ListaConcentradores,
    ListaConcentradoresPrime,
    DeclConcentrador,
    OpcionCoaxial,
    ListaCoaxiales,
    ListaCoaxialesPrime,
    DeclCoaxial,
    Modulos,
    Modulo,
    BloqueInicio,
    Sentencias,
    Sentencia,
    SentenciaColoca,
    SentenciaColocaCoaxial,
    SentenciaColocaCoaxialConcentrador,
    SentenciaUneMaquinaPuerto,
    SentenciaAsignaPuerto,
    SentenciaMaquinaCoaxial,
    SentenciaAsignaMaquinaCoaxial,
    SentenciaEscribe,
    SentenciaSi,
    OpcionSino,
    LlamadaModulo,
    Direccion,
    Expresion,
    ExpresionOr,
    ExpresionOrPrime,
    ExpresionAnd,
    ExpresionAndPrime,
    ExpresionRelacional,
    OpRelacional,
    OperadorRelacional,
    ExpresionNot,
    ExpresionPrimaria,
    Accesos,
    AccesoCampo,
    AccesoArreglo,
}

impl NonTerminal {
    pub fn as_str(&self) -> &'static str {
        match self {
            NonTerminal::Programa => "Programa",
            NonTerminal::Definiciones => "Definiciones",
            NonTerminal::DefMaquinas => "DefMaquinas",
            NonTerminal::DefConcentradores => "DefConcentradores",
            NonTerminal::DefCoaxiales => "DefCoaxiales",
            NonTerminal::TipoCoaxial => "TipoCoaxial",
            NonTerminal::ListaMaquinas => "ListaMaquinas",
            NonTerminal::ListaMaquinasPrime => "ListaMaquinas'",
            NonTerminal::ListaConcentradores => "ListaConcentradores",
            NonTerminal::ListaConcentradoresPrime => "ListaConcentradores'",
            NonTerminal::DeclConcentrador => "DeclConcentrador",
            NonTerminal::OpcionCoaxial => "OpcionCoaxial",
            NonTerminal::ListaCoaxiales => "ListaCoaxiales",
            NonTerminal::ListaCoaxialesPrime => "ListaCoaxiales'",
            NonTerminal::DeclCoaxial => "DeclCoaxial",
            NonTerminal::Modulos => "Modulos",
            NonTerminal::Modulo => "Modulo",
            NonTerminal::BloqueInicio => "BloqueInicio",
            NonTerminal::Sentencias => "Sentencias",
            NonTerminal::Sentencia => "Sentencia",
            NonTerminal::SentenciaColoca => "SentenciaColoca",
            NonTerminal::SentenciaColocaCoaxial => "SentenciaColocaCoaxial",
            NonTerminal::SentenciaColocaCoaxialConcentrador => "SentenciaColocaCoaxialConcentrador",
            NonTerminal::SentenciaUneMaquinaPuerto => "SentenciaUneMaquinaPuerto",
            NonTerminal::SentenciaAsignaPuerto => "SentenciaAsignaPuerto",
            NonTerminal::SentenciaMaquinaCoaxial => "SentenciaMaquinaCoaxial",
            NonTerminal::SentenciaAsignaMaquinaCoaxial => "SentenciaAsignaMaquinaCoaxial",
            NonTerminal::SentenciaEscribe => "SentenciaEscribe",
            NonTerminal::SentenciaSi => "SentenciaSi",
            NonTerminal::OpcionSino => "OpcionSino",
            NonTerminal::LlamadaModulo => "LlamadaModulo",
            NonTerminal::Direccion => "Direccion",
            NonTerminal::Expresion => "Expresion",
            NonTerminal::ExpresionOr => "ExpresionOr",
            NonTerminal::ExpresionOrPrime => "ExpresionOr'",
            NonTerminal::ExpresionAnd => "ExpresionAnd",
            NonTerminal::ExpresionAndPrime => "ExpresionAnd'",
            NonTerminal::ExpresionRelacional => "ExpresionRelacional",
            NonTerminal::OpRelacional => "OpRelacional",
            NonTerminal::OperadorRelacional => "OperadorRelacional",
            NonTerminal::ExpresionNot => "ExpresionNot",
            NonTerminal::ExpresionPrimaria => "ExpresionPrimaria",
            NonTerminal::Accesos => "Accesos",
            NonTerminal::AccesoCampo => "AccesoCampo",
            NonTerminal::AccesoArreglo => "AccesoArreglo",
        }
    }
}

/// Conjuntos FIRST y FOLLOW pre-calculados
pub struct FirstFollowSets {
    first: HashMap<NonTerminal, HashSet<Symbol>>,
    follow: HashMap<NonTerminal, HashSet<Symbol>>,
}

impl FirstFollowSets {
    /// Crea los conjuntos FIRST y FOLLOW según la gramática
    pub fn new() -> Self {
        let mut first = HashMap::new();
        let mut follow = HashMap::new();

        // Inicializar FIRST sets (basado en docs/first_follow.txt)
        Self::initialize_first_sets(&mut first);

        // Inicializar FOLLOW sets
        Self::initialize_follow_sets(&mut follow);

        Self { first, follow }
    }

    fn initialize_first_sets(first: &mut HashMap<NonTerminal, HashSet<Symbol>>) {
        use Symbol::{Terminal, Epsilon};
        use Token::*;
        use NonTerminal as NT;

        // FIRST(Programa) = { PROGRAMA }
        first.insert(NT::Programa, hashset![Terminal(Token::Programa)]);

        // FIRST(Definiciones) = { DEFINE, ε }
        first.insert(NT::Definiciones, hashset![Terminal(Define), Epsilon]);

        // FIRST(DefMaquinas) = { DEFINE, ε }
        first.insert(NT::DefMaquinas, hashset![Terminal(Define), Epsilon]);

        // FIRST(DefConcentradores) = { DEFINE, ε }
        first.insert(NT::DefConcentradores, hashset![Terminal(Define), Epsilon]);

        // FIRST(DefCoaxiales) = { DEFINE, ε }
        first.insert(NT::DefCoaxiales, hashset![Terminal(Define), Epsilon]);

        // FIRST(TipoCoaxial) = { COAXIAL, SEGMENTO }
        first.insert(NT::TipoCoaxial, hashset![Terminal(Coaxial), Terminal(Segmento)]);

        // FIRST(ListaMaquinas) = { IDENTIFICADOR }
        first.insert(NT::ListaMaquinas, hashset![Terminal(Identificador(String::new()))]);

        // FIRST(ListaMaquinas') = { COMA, ε }
        first.insert(NT::ListaMaquinasPrime, hashset![Terminal(Coma), Epsilon]);

        // FIRST(ListaConcentradores) = { IDENTIFICADOR }
        first.insert(NT::ListaConcentradores, hashset![Terminal(Identificador(String::new()))]);

        // FIRST(ListaConcentradores') = { COMA, ε }
        first.insert(NT::ListaConcentradoresPrime, hashset![Terminal(Coma), Epsilon]);

        // FIRST(DeclConcentrador) = { IDENTIFICADOR }
        first.insert(NT::DeclConcentrador, hashset![Terminal(Identificador(String::new()))]);

        // FIRST(OpcionCoaxial) = { PUNTO, ε }
        first.insert(NT::OpcionCoaxial, hashset![Terminal(Punto), Epsilon]);

        // FIRST(ListaCoaxiales) = { IDENTIFICADOR }
        first.insert(NT::ListaCoaxiales, hashset![Terminal(Identificador(String::new()))]);

        // FIRST(ListaCoaxiales') = { COMA, ε }
        first.insert(NT::ListaCoaxialesPrime, hashset![Terminal(Coma), Epsilon]);

        // FIRST(DeclCoaxial) = { IDENTIFICADOR }
        first.insert(NT::DeclCoaxial, hashset![Terminal(Identificador(String::new()))]);

        // FIRST(Modulos) = { MODULO, ε }
        first.insert(NT::Modulos, hashset![Terminal(Modulo), Epsilon]);

        // FIRST(Modulo) = { MODULO }
        first.insert(NT::Modulo, hashset![Terminal(Modulo)]);

        // FIRST(BloqueInicio) = { INICIO }
        first.insert(NT::BloqueInicio, hashset![Terminal(Inicio)]);

        // FIRST(Sentencias) = { comandos..., IDENTIFICADOR, ε }
        first.insert(NT::Sentencias, hashset![
            Terminal(Coloca), Terminal(ColocaCoaxial),
            Terminal(ColocaCoaxialConcentrador),
            Terminal(UneMaquinaPuerto), Terminal(AsignaPuerto),
            Terminal(MaquinaCoaxial), Terminal(AsignaMaquinaCoaxial),
            Terminal(Escribe), Terminal(Si),
            Terminal(Identificador(String::new())),
            Epsilon
        ]);

        // FIRST(Sentencia) = { comandos..., IDENTIFICADOR }
        first.insert(NT::Sentencia, hashset![
            Terminal(Coloca), Terminal(ColocaCoaxial),
            Terminal(ColocaCoaxialConcentrador),
            Terminal(UneMaquinaPuerto), Terminal(AsignaPuerto),
            Terminal(MaquinaCoaxial), Terminal(AsignaMaquinaCoaxial),
            Terminal(Escribe), Terminal(Si),
            Terminal(Identificador(String::new()))
        ]);

        // FIRST individuales de sentencias
        first.insert(NT::SentenciaColoca, hashset![Terminal(Coloca)]);
        first.insert(NT::SentenciaColocaCoaxial, hashset![Terminal(ColocaCoaxial)]);
        first.insert(NT::SentenciaColocaCoaxialConcentrador, hashset![Terminal(ColocaCoaxialConcentrador)]);
        first.insert(NT::SentenciaUneMaquinaPuerto, hashset![Terminal(UneMaquinaPuerto)]);
        first.insert(NT::SentenciaAsignaPuerto, hashset![Terminal(AsignaPuerto)]);
        first.insert(NT::SentenciaMaquinaCoaxial, hashset![Terminal(MaquinaCoaxial)]);
        first.insert(NT::SentenciaAsignaMaquinaCoaxial, hashset![Terminal(AsignaMaquinaCoaxial)]);
        first.insert(NT::SentenciaEscribe, hashset![Terminal(Escribe)]);
        first.insert(NT::SentenciaSi, hashset![Terminal(Si)]);

        // FIRST(OpcionSino) = { SINO, ε }
        first.insert(NT::OpcionSino, hashset![Terminal(Sino), Epsilon]);

        // FIRST(LlamadaModulo) = { IDENTIFICADOR }
        first.insert(NT::LlamadaModulo, hashset![Terminal(Identificador(String::new()))]);

        // FIRST(Direccion) = { ARRIBA, ABAJO, IZQUIERDA, DERECHA }
        first.insert(NT::Direccion, hashset![
            Terminal(Arriba), Terminal(Abajo),
            Terminal(Izquierda), Terminal(Derecha)
        ]);

        // FIRST(Expresion) = { NOT, NUMERO, CADENA, IDENTIFICADOR, PAREN_IZQ }
        first.insert(NT::Expresion, hashset![
            Terminal(Not), Terminal(Numero(0)),
            Terminal(Cadena(String::new())),
            Terminal(Identificador(String::new())),
            Terminal(ParenIzq)
        ]);

        // FIRST(ExpresionOr) = { NOT, NUMERO, CADENA, IDENTIFICADOR, PAREN_IZQ }
        first.insert(NT::ExpresionOr, hashset![
            Terminal(Not), Terminal(Numero(0)),
            Terminal(Cadena(String::new())),
            Terminal(Identificador(String::new())),
            Terminal(ParenIzq)
        ]);

        // FIRST(ExpresionOr') = { OR, ε }
        first.insert(NT::ExpresionOrPrime, hashset![Terminal(Or), Epsilon]);

        // FIRST(ExpresionAnd) = { NOT, NUMERO, CADENA, IDENTIFICADOR, PAREN_IZQ }
        first.insert(NT::ExpresionAnd, hashset![
            Terminal(Not), Terminal(Numero(0)),
            Terminal(Cadena(String::new())),
            Terminal(Identificador(String::new())),
            Terminal(ParenIzq)
        ]);

        // FIRST(ExpresionAnd') = { AND, ε }
        first.insert(NT::ExpresionAndPrime, hashset![Terminal(And), Epsilon]);

        // FIRST(ExpresionRelacional) = { NOT, NUMERO, CADENA, IDENTIFICADOR, PAREN_IZQ }
        first.insert(NT::ExpresionRelacional, hashset![
            Terminal(Not), Terminal(Numero(0)),
            Terminal(Cadena(String::new())),
            Terminal(Identificador(String::new())),
            Terminal(ParenIzq)
        ]);

        // FIRST(OpRelacional) = { =, <>, <, >, <=, >=, ε }
        first.insert(NT::OpRelacional, hashset![
            Terminal(Igual), Terminal(Diferente),
            Terminal(Menor), Terminal(Mayor),
            Terminal(MenorIgual), Terminal(MayorIgual),
            Epsilon
        ]);

        // FIRST(OperadorRelacional) = { =, <>, <, >, <=, >= }
        first.insert(NT::OperadorRelacional, hashset![
            Terminal(Igual), Terminal(Diferente),
            Terminal(Menor), Terminal(Mayor),
            Terminal(MenorIgual), Terminal(MayorIgual)
        ]);

        // FIRST(ExpresionNot) = { NOT, NUMERO, CADENA, IDENTIFICADOR, PAREN_IZQ }
        first.insert(NT::ExpresionNot, hashset![
            Terminal(Not), Terminal(Numero(0)),
            Terminal(Cadena(String::new())),
            Terminal(Identificador(String::new())),
            Terminal(ParenIzq)
        ]);

        // FIRST(ExpresionPrimaria) = { NUMERO, CADENA, IDENTIFICADOR, PAREN_IZQ }
        first.insert(NT::ExpresionPrimaria, hashset![
            Terminal(Numero(0)),
            Terminal(Cadena(String::new())),
            Terminal(Identificador(String::new())),
            Terminal(ParenIzq)
        ]);

        // FIRST(Accesos) = { PUNTO, CORCHETE_IZQ, ε }
        first.insert(NT::Accesos, hashset![
            Terminal(Punto), Terminal(CorcheteIzq), Epsilon
        ]);

        // FIRST(AccesoCampo) = { PUNTO }
        first.insert(NT::AccesoCampo, hashset![Terminal(Punto)]);

        // FIRST(AccesoArreglo) = { CORCHETE_IZQ, ε }
        first.insert(NT::AccesoArreglo, hashset![Terminal(CorcheteIzq), Epsilon]);
    }

    fn initialize_follow_sets(follow: &mut HashMap<NonTerminal, HashSet<Symbol>>) {
        use Symbol::{Terminal, Eof};
        use Token::*;
        use NonTerminal as NT;

        // FOLLOW sets según docs/first_follow.txt líneas 110-243

        // FOLLOW(Programa) = { EOF }
        follow.insert(NT::Programa, hashset![Eof]);

        // FOLLOW(Definiciones) = { MODULO, INICIO }
        follow.insert(NT::Definiciones, hashset![Terminal(Modulo), Terminal(Inicio)]);

        // FOLLOW(DefMaquinas) = { DEFINE, MODULO, INICIO }
        follow.insert(NT::DefMaquinas, hashset![Terminal(Define), Terminal(Modulo), Terminal(Inicio)]);

        // FOLLOW(DefConcentradores) = { DEFINE, MODULO, INICIO }
        follow.insert(NT::DefConcentradores, hashset![Terminal(Define), Terminal(Modulo), Terminal(Inicio)]);

        // FOLLOW(DefCoaxiales) = { MODULO, INICIO }
        follow.insert(NT::DefCoaxiales, hashset![Terminal(Modulo), Terminal(Inicio)]);

        // FOLLOW(TipoCoaxial) = { IDENTIFICADOR }
        follow.insert(NT::TipoCoaxial, hashset![Terminal(Identificador(String::new()))]);

        // FOLLOW(ListaMaquinas) = { PUNTO_COMA }
        follow.insert(NT::ListaMaquinas, hashset![Terminal(PuntoYComa)]);

        // FOLLOW(ListaMaquinas') = { PUNTO_COMA }
        follow.insert(NT::ListaMaquinasPrime, hashset![Terminal(PuntoYComa)]);

        // FOLLOW(ListaConcentradores) = { PUNTO_COMA }
        follow.insert(NT::ListaConcentradores, hashset![Terminal(PuntoYComa)]);

        // FOLLOW(ListaConcentradores') = { PUNTO_COMA }
        follow.insert(NT::ListaConcentradoresPrime, hashset![Terminal(PuntoYComa)]);

        // FOLLOW(DeclConcentrador) = { COMA, PUNTO_COMA }
        follow.insert(NT::DeclConcentrador, hashset![Terminal(Coma), Terminal(PuntoYComa)]);

        // FOLLOW(OpcionCoaxial) = { COMA, PUNTO_COMA }
        follow.insert(NT::OpcionCoaxial, hashset![Terminal(Coma), Terminal(PuntoYComa)]);

        // FOLLOW(ListaCoaxiales) = { PUNTO_COMA }
        follow.insert(NT::ListaCoaxiales, hashset![Terminal(PuntoYComa)]);

        // FOLLOW(ListaCoaxiales') = { PUNTO_COMA }
        follow.insert(NT::ListaCoaxialesPrime, hashset![Terminal(PuntoYComa)]);

        // FOLLOW(DeclCoaxial) = { COMA, PUNTO_COMA }
        follow.insert(NT::DeclCoaxial, hashset![Terminal(Coma), Terminal(PuntoYComa)]);

        // FOLLOW(Modulos) = { INICIO }
        follow.insert(NT::Modulos, hashset![Terminal(Inicio)]);

        // FOLLOW(Modulo) = { MODULO, INICIO }
        follow.insert(NT::Modulo, hashset![Terminal(Modulo), Terminal(Inicio)]);

        // FOLLOW(BloqueInicio) - multiple tokens
        follow.insert(NT::BloqueInicio, hashset![
            Terminal(Punto), Terminal(Modulo), Terminal(Inicio), Terminal(Fin),
            Terminal(Coloca), Terminal(ColocaCoaxial), Terminal(ColocaCoaxialConcentrador),
            Terminal(UneMaquinaPuerto), Terminal(AsignaPuerto), Terminal(MaquinaCoaxial),
            Terminal(AsignaMaquinaCoaxial), Terminal(Escribe), Terminal(Si),
            Terminal(Identificador(String::new()))
        ]);

        // FOLLOW(Sentencias) = { FIN }
        follow.insert(NT::Sentencias, hashset![Terminal(Fin)]);

        // FOLLOW(Sentencia) - statement followers
        let stmt_followers = hashset![
            Terminal(Coloca), Terminal(ColocaCoaxial), Terminal(ColocaCoaxialConcentrador),
            Terminal(UneMaquinaPuerto), Terminal(AsignaPuerto), Terminal(MaquinaCoaxial),
            Terminal(AsignaMaquinaCoaxial), Terminal(Escribe), Terminal(Si),
            Terminal(Identificador(String::new())), Terminal(Fin)
        ];

        follow.insert(NT::Sentencia, stmt_followers.clone());
        follow.insert(NT::SentenciaColoca, stmt_followers.clone());
        follow.insert(NT::SentenciaColocaCoaxial, stmt_followers.clone());
        follow.insert(NT::SentenciaColocaCoaxialConcentrador, stmt_followers.clone());
        follow.insert(NT::SentenciaUneMaquinaPuerto, stmt_followers.clone());
        follow.insert(NT::SentenciaAsignaPuerto, stmt_followers.clone());
        follow.insert(NT::SentenciaMaquinaCoaxial, stmt_followers.clone());
        follow.insert(NT::SentenciaAsignaMaquinaCoaxial, stmt_followers.clone());
        follow.insert(NT::SentenciaEscribe, stmt_followers.clone());
        follow.insert(NT::SentenciaSi, stmt_followers.clone());
        follow.insert(NT::OpcionSino, stmt_followers.clone());
        follow.insert(NT::LlamadaModulo, stmt_followers);

        // FOLLOW(Direccion) = { PAREN_DER }
        follow.insert(NT::Direccion, hashset![Terminal(ParenDer)]);

        // FOLLOW(Expresion) = { PAREN_DER, COMA, CORCHETE_DER, PUNTO_COMA, INICIO }
        let expr_followers = hashset![
            Terminal(ParenDer), Terminal(Coma), Terminal(CorcheteDer),
            Terminal(PuntoYComa), Terminal(Inicio)
        ];
        follow.insert(NT::Expresion, expr_followers.clone());
        follow.insert(NT::ExpresionOr, expr_followers.clone());
        follow.insert(NT::ExpresionOrPrime, expr_followers.clone());

        // FOLLOW(ExpresionAnd) = { OR, PAREN_DER, COMA, CORCHETE_DER, PUNTO_COMA, INICIO }
        let and_followers = hashset![
            Terminal(Or), Terminal(ParenDer), Terminal(Coma),
            Terminal(CorcheteDer), Terminal(PuntoYComa), Terminal(Inicio)
        ];
        follow.insert(NT::ExpresionAnd, and_followers.clone());
        follow.insert(NT::ExpresionAndPrime, and_followers);

        // FOLLOW(ExpresionRelacional) = { AND, OR, PAREN_DER, COMA, CORCHETE_DER, PUNTO_COMA, INICIO }
        let rel_followers = hashset![
            Terminal(And), Terminal(Or), Terminal(ParenDer), Terminal(Coma),
            Terminal(CorcheteDer), Terminal(PuntoYComa), Terminal(Inicio)
        ];
        follow.insert(NT::ExpresionRelacional, rel_followers.clone());
        follow.insert(NT::OpRelacional, rel_followers.clone());

        // FOLLOW(OperadorRelacional) = { NOT, NUMERO, CADENA, IDENTIFICADOR, PAREN_IZQ }
        follow.insert(NT::OperadorRelacional, hashset![
            Terminal(Not), Terminal(Numero(0)), Terminal(Cadena(String::new())),
            Terminal(Identificador(String::new())), Terminal(ParenIzq)
        ]);

        // FOLLOW(ExpresionNot) - relational and logical operators plus expression followers
        let not_followers = hashset![
            Terminal(Igual), Terminal(Diferente), Terminal(Menor), Terminal(Mayor),
            Terminal(MenorIgual), Terminal(MayorIgual), Terminal(And), Terminal(Or),
            Terminal(ParenDer), Terminal(Coma), Terminal(CorcheteDer),
            Terminal(PuntoYComa), Terminal(Inicio)
        ];
        follow.insert(NT::ExpresionNot, not_followers.clone());
        follow.insert(NT::ExpresionPrimaria, not_followers.clone());
        follow.insert(NT::Accesos, not_followers.clone());
        follow.insert(NT::AccesoCampo, not_followers.clone());
        follow.insert(NT::AccesoArreglo, not_followers);
    }

    /// Obtiene el conjunto FIRST de un no-terminal
    pub fn first(&self, nt: NonTerminal) -> Option<&HashSet<Symbol>> {
        self.first.get(&nt)
    }

    /// Obtiene el conjunto FOLLOW de un no-terminal
    pub fn follow(&self, nt: NonTerminal) -> Option<&HashSet<Symbol>> {
        self.follow.get(&nt)
    }

    /// Verifica si un símbolo está en FIRST de un no-terminal
    pub fn is_in_first(&self, nt: NonTerminal, symbol: &Symbol) -> bool {
        self.first.get(&nt)
            .map(|set| set.contains(symbol))
            .unwrap_or(false)
    }

    /// Verifica si un símbolo está en FOLLOW de un no-terminal
    pub fn is_in_follow(&self, nt: NonTerminal, symbol: &Symbol) -> bool {
        self.follow.get(&nt)
            .map(|set| set.contains(symbol))
            .unwrap_or(false)
    }
}

// Macro helper para crear hashsets
macro_rules! hashset {
    ($($elem:expr),* $(,)?) => {{
        let mut set = HashSet::new();
        $(set.insert($elem);)*
        set
    }};
}

// Re-exportar la macro
use hashset;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_sets_creation() {
        let sets = FirstFollowSets::new();

        // Verificar FIRST(Programa)
        let first_programa = sets.first(NonTerminal::Programa).unwrap();
        assert!(first_programa.contains(&Symbol::Terminal(Token::Programa)));
        assert_eq!(first_programa.len(), 1);
    }

    #[test]
    fn test_first_contains_epsilon() {
        let sets = FirstFollowSets::new();

        // Verificar que Definiciones contiene epsilon
        let first_defs = sets.first(NonTerminal::Definiciones).unwrap();
        assert!(first_defs.contains(&Symbol::Epsilon));
        assert!(first_defs.contains(&Symbol::Terminal(Token::Define)));
    }

    #[test]
    fn test_is_in_first() {
        let sets = FirstFollowSets::new();

        assert!(sets.is_in_first(
            NonTerminal::Programa,
            &Symbol::Terminal(Token::Programa)
        ));
    }
}
