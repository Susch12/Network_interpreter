// Parser - Análisis Sintáctico
// Convierte tokens en AST

use crate::lexer::{Token, TokenInfo};
use crate::ast::*;
use colored::*;

// ============================================================================
// ERROR DE PARSEO
// ============================================================================

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub location: Location,
}

impl ParseError {
    pub fn new(message: String, location: Location) -> Self {
        Self { message, location }
    }
}

// ============================================================================
// PARSER
// ============================================================================

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, current: 0 }
    }

    // ========== UTILIDADES ==========

    fn peek(&self) -> &Token {
        if self.current < self.tokens.len() {
            &self.tokens[self.current].token
        } else {
            // Retornar un token punto como EOF
            &Token::Punto
        }
    }

    fn peek_info(&self) -> Option<&TokenInfo> {
        if self.current < self.tokens.len() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<&TokenInfo> {
        if self.current < self.tokens.len() {
            self.current += 1;
            Some(&self.tokens[self.current - 1])
        } else {
            None
        }
    }

    fn expect(&mut self, expected: Token) -> Result<&TokenInfo, ParseError> {
        let current_info = self.peek_info().cloned();
        
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            self.advance()
                .ok_or_else(|| ParseError::new(
                    "Token inesperado al final del archivo".into(),
                    Location::unknown(),
                ))
        } else {
            let loc = current_info
                .map(|t| Location::from_token(&t))
                .unwrap_or_else(Location::unknown);
            
            Err(ParseError::new(
                format!("Se esperaba {:?}, se encontró {:?}", expected, self.peek()),
                loc,
            ))
        }
    }

    fn current_location(&self) -> Location {
        self.peek_info()
            .map(Location::from_token)
            .unwrap_or_else(Location::unknown)
    }

    // Convierte un token en un nombre de campo (permite palabras reservadas)
    fn token_to_field_name(&self, token: &Token) -> Option<String> {
        match token {
            Token::Identificador(s) => Some(s.clone()),
            // Palabras reservadas que pueden ser nombres de campo
            Token::Coaxial => Some("coaxial".to_string()),
            Token::Segmento => Some("segmento".to_string()),
            Token::Maquinas => Some("maquinas".to_string()),
            Token::Concentradores => Some("concentradores".to_string()),
            Token::Derecha => Some("derecha".to_string()),
            Token::Izquierda => Some("izquierda".to_string()),
            Token::Arriba => Some("arriba".to_string()),
            Token::Abajo => Some("abajo".to_string()),
            _ => None,
        }
    }

    // ========== PARSER PRINCIPAL ==========

    pub fn parse(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut errors = Vec::new();

        match self.parse_programa() {
            Ok(program) => {
                if errors.is_empty() {
                    Ok(program)
                } else {
                    Err(errors)
                }
            }
            Err(e) => {
                errors.push(e);
                Err(errors)
            }
        }
    }

    // ========== PROGRAMA ==========
    // programa ::= "programa" IDENTIFICADOR ";" definiciones modulos* "inicio" sentencias "fin" "."

    fn parse_programa(&mut self) -> Result<Program, ParseError> {
        let loc = self.current_location();

        // Esperar "programa"
        self.expect(Token::Programa)?;

        // Esperar identificador (nombre del programa)
        let nombre = match self.peek() {
            Token::Identificador(name) => {
                let n = name.clone();
                self.advance();
                n
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre del programa".into(),
                    self.current_location(),
                ));
            }
        };

        // Esperar punto y coma
        self.expect(Token::PuntoYComa)?;

        // Parsear definiciones
        let definiciones = self.parse_definiciones()?;

        // Parsear módulos (opcionales)
        let mut modulos = Vec::new();
        while self.peek() == &Token::Modulo {
            modulos.push(self.parse_modulo()?);
        }

        // Parsear bloque principal: inicio sentencias fin.
        self.expect(Token::Inicio)?;
        let sentencias = self.parse_sentencias()?;
        self.expect(Token::Fin)?;
        self.expect(Token::Punto)?;

        Ok(Program {
            nombre,
            definiciones,
            modulos,
            sentencias,
            location: loc,
        })
    }

    // ========== MÓDULOS ==========
    // modulo ::= "modulo" IDENTIFICADOR ";" "inicio" sentencias "fin"

    fn parse_modulo(&mut self) -> Result<Modulo, ParseError> {
        let loc = self.current_location();
        self.expect(Token::Modulo)?;

        let nombre = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre del módulo".into(),
                    self.current_location(),
                ));
            }
        };

        self.expect(Token::PuntoYComa)?;
        self.expect(Token::Inicio)?;
        let sentencias = self.parse_sentencias()?;
        self.expect(Token::Fin)?;

        Ok(Modulo {
            nombre,
            sentencias,
            location: loc,
        })
    }

    // ========== DEFINICIONES ==========
    // definiciones ::= define_maquinas? define_concentradores? define_coaxial?

    fn parse_definiciones(&mut self) -> Result<Definitions, ParseError> {
        let loc = self.current_location();
        let mut maquinas = Vec::new();
        let mut concentradores = Vec::new();
        let mut coaxiales = Vec::new();

        // Intentar parsear define maquinas
        if self.peek() == &Token::Define {
            self.advance();
            
            match self.peek() {
                Token::Maquinas => {
                    self.advance();
                    maquinas = self.parse_lista_maquinas()?;
                    self.expect(Token::PuntoYComa)?;
                }
                _ => {
                    // No es define maquinas, retroceder
                    self.current -= 1;
                }
            }
        }

        // Intentar parsear define concentradores
        if self.peek() == &Token::Define {
            self.advance();
            
            match self.peek() {
                Token::Concentradores => {
                    self.advance();
                    concentradores = self.parse_lista_concentradores()?;
                    self.expect(Token::PuntoYComa)?;
                }
                _ => {
                    // No es define concentradores, retroceder
                    self.current -= 1;
                }
            }
        }

        // ⚡ Intentar parsear define coaxial o segmento
        if self.peek() == &Token::Define {
            self.advance();
            
            match self.peek() {
                Token::Coaxial | Token::Segmento => {
                    self.advance();
                    coaxiales = self.parse_lista_coaxial()?;
                    self.expect(Token::PuntoYComa)?;
                }
                _ => {
                    // No es define coaxial/segmento, retroceder
                    self.current -= 1;
                }
            }
        }

        Ok(Definitions {
            maquinas,
            concentradores,
            coaxiales,
            location: loc,
        })
    }

    // ========== LISTA DE MÁQUINAS ==========
    // lista_ids ::= IDENTIFICADOR ("," IDENTIFICADOR)*

    fn parse_lista_maquinas(&mut self) -> Result<Vec<MaquinaDecl>, ParseError> {
        let mut maquinas = Vec::new();

        loop {
            let loc = self.current_location();

            match self.peek() {
                Token::Identificador(nombre) => {
                    let n = nombre.clone();
                    self.advance();

                    maquinas.push(MaquinaDecl {
                        nombre: n,
                        location: loc,
                    });
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba identificador de máquina".into(),
                        self.current_location(),
                    ));
                }
            }

            // Si no hay coma, terminar
            if self.peek() != &Token::Coma {
                break;
            }
            self.advance(); // Consumir coma
        }

        Ok(maquinas)
    }

    // ========== LISTA DE CONCENTRADORES ==========
    // def_concentrador ::= IDENTIFICADOR "=" NUMERO ("." "1")?

    fn parse_lista_concentradores(&mut self) -> Result<Vec<ConcentradorDecl>, ParseError> {
        let mut concentradores = Vec::new();

        loop {
            let loc = self.current_location();

            // Nombre del concentrador
            let nombre = match self.peek() {
                Token::Identificador(n) => {
                    let name = n.clone();
                    self.advance();
                    name
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba nombre de concentrador".into(),
                        self.current_location(),
                    ));
                }
            };

            // Igual
            self.expect(Token::Igual)?;

            // Número de puertos
            let puertos = match self.peek() {
                Token::Numero(p) => {
                    let ports = *p;
                    self.advance();
                    ports
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba número de puertos".into(),
                        self.current_location(),
                    ));
                }
            };

            // ⚡ Verificar si tiene .1 (salida coaxial)
            let tiene_coaxial = if self.peek() == &Token::Punto {
                self.advance();
                match self.peek() {
                    Token::Numero(1) => {
                        self.advance();
                        true
                    }
                    _ => {
                        return Err(ParseError::new(
                            "Después del punto debe ir 1 para indicar salida coaxial".into(),
                            self.current_location(),
                        ));
                    }
                }
            } else {
                false
            };

            concentradores.push(ConcentradorDecl {
                nombre,
                puertos,
                tiene_coaxial,
                location: loc,
            });

            // Si no hay coma, terminar
            if self.peek() != &Token::Coma {
                break;
            }
            self.advance(); // Consumir coma
        }

        Ok(concentradores)
    }

    // ========== LISTA DE COAXIALES ==========
    // def_coaxial ::= IDENTIFICADOR "=" NUMERO

    fn parse_lista_coaxial(&mut self) -> Result<Vec<CoaxialDecl>, ParseError> {
        let mut coaxiales = Vec::new();

        loop {
            let loc = self.current_location();

            // Nombre del coaxial
            let nombre = match self.peek() {
                Token::Identificador(n) => {
                    let name = n.clone();
                    self.advance();
                    name
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba nombre de coaxial".into(),
                        self.current_location(),
                    ));
                }
            };

            // Igual
            self.expect(Token::Igual)?;

            // Longitud
            let longitud = match self.peek() {
                Token::Numero(l) => {
                    let len = *l;
                    self.advance();
                    len
                }
                _ => {
                    return Err(ParseError::new(
                        "Se esperaba longitud del coaxial".into(),
                        self.current_location(),
                    ));
                }
            };

            coaxiales.push(CoaxialDecl {
                nombre,
                longitud,
                location: loc,
            });

            // Si no hay coma, terminar
            if self.peek() != &Token::Coma {
                break;
            }
            self.advance(); // Consumir coma
        }

        Ok(coaxiales)
    }

    // ========== SENTENCIAS ==========

    fn parse_sentencias(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut sentencias = Vec::new();

        loop {
            // Si encontramos 'fin', terminamos
            if self.peek() == &Token::Fin {
                break;
            }

            // Si llegamos al final de los tokens, terminamos
            if self.current >= self.tokens.len() {
                break;
            }

            sentencias.push(self.parse_sentencia()?);
        }

        Ok(sentencias)
    }

    fn parse_sentencia(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();

        match self.peek() {
            Token::Coloca => self.parse_coloca(),
            Token::ColocaCoaxial => self.parse_coloca_coaxial(),
            Token::ColocaCoaxialConcentrador => self.parse_coloca_coaxial_concentrador(),
            Token::UneMaquinaPuerto => self.parse_une_maquina_puerto(),
            Token::AsignaPuerto => self.parse_asigna_puerto(),
            Token::MaquinaCoaxial => self.parse_maquina_coaxial(),
            Token::AsignaMaquinaCoaxial => self.parse_asigna_maquina_coaxial(),
            Token::Escribe => self.parse_escribe(),
            Token::Si => self.parse_si(),
            Token::Identificador(nombre) => {
                // Llamada a módulo
                let n = nombre.clone();
                self.advance();
                self.expect(Token::PuntoYComa)?;
                Ok(Statement::LlamadaModulo {
                    nombre: n,
                    location: loc,
                })
            }
            _ => Err(ParseError::new(
                format!("Sentencia inválida: {:?}", self.peek()),
                self.current_location(),
            )),
        }
    }

    // coloca(objeto, x, y);
    fn parse_coloca(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();
        self.expect(Token::Coloca)?;
        self.expect(Token::ParenIzq)?;

        let objeto = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de objeto en coloca()".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::Coma)?;
        let x = self.parse_expresion()?;
        self.expect(Token::Coma)?;
        let y = self.parse_expresion()?;
        self.expect(Token::ParenDer)?;
        self.expect(Token::PuntoYComa)?;

        Ok(Statement::Coloca {
            objeto,
            x,
            y,
            location: loc,
        })
    }

    // colocaCoaxial(coaxial, x, y, direccion);
    fn parse_coloca_coaxial(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();
        self.expect(Token::ColocaCoaxial)?;
        self.expect(Token::ParenIzq)?;

        let coaxial = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de coaxial".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::Coma)?;
        let x = self.parse_expresion()?;
        self.expect(Token::Coma)?;
        let y = self.parse_expresion()?;
        self.expect(Token::Coma)?;

        let direccion = match self.peek() {
            Token::Arriba => {
                self.advance();
                Direccion::Arriba
            }
            Token::Abajo => {
                self.advance();
                Direccion::Abajo
            }
            Token::Izquierda => {
                self.advance();
                Direccion::Izquierda
            }
            Token::Derecha => {
                self.advance();
                Direccion::Derecha
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba dirección (arriba, abajo, izquierda, derecha)".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::ParenDer)?;
        self.expect(Token::PuntoYComa)?;

        Ok(Statement::ColocaCoaxial {
            coaxial,
            x,
            y,
            direccion,
            location: loc,
        })
    }

    // colocaCoaxialConcentrador(coaxial, concentrador);
    fn parse_coloca_coaxial_concentrador(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();
        self.expect(Token::ColocaCoaxialConcentrador)?;
        self.expect(Token::ParenIzq)?;

        let coaxial = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de coaxial".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::Coma)?;

        let concentrador = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de concentrador".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::ParenDer)?;
        self.expect(Token::PuntoYComa)?;

        Ok(Statement::ColocaCoaxialConcentrador {
            coaxial,
            concentrador,
            location: loc,
        })
    }

    // uneMaquinaPuerto(maquina, concentrador, puerto);
    fn parse_une_maquina_puerto(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();
        self.expect(Token::UneMaquinaPuerto)?;
        self.expect(Token::ParenIzq)?;

        let maquina = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de máquina".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::Coma)?;

        let concentrador = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de concentrador".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::Coma)?;
        let puerto = self.parse_expresion()?;
        self.expect(Token::ParenDer)?;
        self.expect(Token::PuntoYComa)?;

        Ok(Statement::UneMaquinaPuerto {
            maquina,
            concentrador,
            puerto,
            location: loc,
        })
    }

    // asignaPuerto(maquina, concentrador);
    fn parse_asigna_puerto(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();
        self.expect(Token::AsignaPuerto)?;
        self.expect(Token::ParenIzq)?;

        let maquina = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de máquina".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::Coma)?;

        let concentrador = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de concentrador".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::ParenDer)?;
        self.expect(Token::PuntoYComa)?;

        Ok(Statement::AsignaPuerto {
            maquina,
            concentrador,
            location: loc,
        })
    }

    // maquinaCoaxial(maquina, coaxial, posicion);
    fn parse_maquina_coaxial(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();
        self.expect(Token::MaquinaCoaxial)?;
        self.expect(Token::ParenIzq)?;

        let maquina = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de máquina".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::Coma)?;

        let coaxial = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de coaxial".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::Coma)?;
        let posicion = self.parse_expresion()?;
        self.expect(Token::ParenDer)?;
        self.expect(Token::PuntoYComa)?;

        Ok(Statement::MaquinaCoaxial {
            maquina,
            coaxial,
            posicion,
            location: loc,
        })
    }

    // asignaMaquinaCoaxial(maquina, coaxial);
    fn parse_asigna_maquina_coaxial(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();
        self.expect(Token::AsignaMaquinaCoaxial)?;
        self.expect(Token::ParenIzq)?;

        let maquina = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de máquina".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::Coma)?;

        let coaxial = match self.peek() {
            Token::Identificador(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => {
                return Err(ParseError::new(
                    "Se esperaba nombre de coaxial".into(),
                    self.current_location(),
                ))
            }
        };

        self.expect(Token::ParenDer)?;
        self.expect(Token::PuntoYComa)?;

        Ok(Statement::AsignaMaquinaCoaxial {
            maquina,
            coaxial,
            location: loc,
        })
    }

    // escribe(expresion);
    fn parse_escribe(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();
        self.expect(Token::Escribe)?;
        self.expect(Token::ParenIzq)?;
        let contenido = self.parse_expresion()?;
        self.expect(Token::ParenDer)?;
        self.expect(Token::PuntoYComa)?;

        Ok(Statement::Escribe {
            contenido,
            location: loc,
        })
    }

    // si condicion inicio sentencias fin sino inicio sentencias fin
    // Los paréntesis se manejan como parte de la expresión
    fn parse_si(&mut self) -> Result<Statement, ParseError> {
        let loc = self.current_location();
        self.expect(Token::Si)?;

        // Parsear la condición (los paréntesis se manejan en parse_expresion)
        let condicion = self.parse_expresion()?;

        self.expect(Token::Inicio)?;

        // Parsear sentencias del bloque entonces
        let entonces = self.parse_sentencias()?;
        self.expect(Token::Fin)?;

        // Verificar si hay bloque sino
        let sino = if self.peek() == &Token::Sino {
            self.advance();
            self.expect(Token::Inicio)?;
            let sentencias_sino = self.parse_sentencias()?;
            self.expect(Token::Fin)?;
            Some(sentencias_sino)
        } else {
            None
        };

        Ok(Statement::Si {
            condicion,
            entonces,
            sino,
            location: loc,
        })
    }

    // ========== EXPRESIONES ==========
    // Implementa precedencia de operadores:
    // 1. Primarias: números, cadenas, identificadores, accesos
    // 2. NOT: !expr
    // 3. Relacionales: <, >, =, <>, <=, >=
    // 4. AND: &&
    // 5. OR: ||

    fn parse_expresion(&mut self) -> Result<Expr, ParseError> {
        self.parse_expresion_or()
    }

    // OR tiene menor precedencia
    fn parse_expresion_or(&mut self) -> Result<Expr, ParseError> {
        let mut izq = self.parse_expresion_and()?;

        while self.peek() == &Token::Or {
            self.advance();
            let der = self.parse_expresion_and()?;
            izq = Expr::Logico {
                izq: Box::new(izq),
                op: OpLogico::Or,
                der: Box::new(der),
            };
        }

        Ok(izq)
    }

    // AND tiene precedencia media
    fn parse_expresion_and(&mut self) -> Result<Expr, ParseError> {
        let mut izq = self.parse_expresion_relacional()?;

        while self.peek() == &Token::And {
            self.advance();
            let der = self.parse_expresion_relacional()?;
            izq = Expr::Logico {
                izq: Box::new(izq),
                op: OpLogico::And,
                der: Box::new(der),
            };
        }

        Ok(izq)
    }

    // Relacionales: =, <, >, <=, >=, <>
    fn parse_expresion_relacional(&mut self) -> Result<Expr, ParseError> {
        let izq = self.parse_expresion_not()?;

        let op = match self.peek() {
            Token::Igual => OpRelacional::Igual,
            Token::Diferente => OpRelacional::Diferente,
            Token::Menor => OpRelacional::Menor,
            Token::Mayor => OpRelacional::Mayor,
            Token::MenorIgual => OpRelacional::MenorIgual,
            Token::MayorIgual => OpRelacional::MayorIgual,
            _ => return Ok(izq),
        };

        self.advance();
        let der = self.parse_expresion_not()?;

        Ok(Expr::Relacional {
            izq: Box::new(izq),
            op,
            der: Box::new(der),
        })
    }

    // NOT: !expr
    fn parse_expresion_not(&mut self) -> Result<Expr, ParseError> {
        if self.peek() == &Token::Not {
            self.advance();
            let expr = self.parse_expresion_not()?;
            return Ok(Expr::Not(Box::new(expr)));
        }

        self.parse_expresion_primaria()
    }

    // Expresiones primarias: números, cadenas, identificadores, accesos
    fn parse_expresion_primaria(&mut self) -> Result<Expr, ParseError> {
        match self.peek().clone() {
            // Paréntesis
            Token::ParenIzq => {
                self.advance();
                let expr = self.parse_expresion()?;
                self.expect(Token::ParenDer)?;
                Ok(expr)
            }

            // Número
            Token::Numero(n) => {
                self.advance();
                Ok(Expr::Numero(n))
            }

            // Cadena
            Token::Cadena(s) => {
                self.advance();
                Ok(Expr::Cadena(s))
            }

            // Identificador (puede tener accesos)
            Token::Identificador(nombre) => {
                self.advance();
                self.parse_accesos(nombre)
            }

            _ => Err(ParseError::new(
                format!("Se esperaba una expresión, se encontró {:?}", self.peek()),
                self.current_location(),
            )),
        }
    }

    // Accesos: .campo o [indice]
    fn parse_accesos(&mut self, objeto: String) -> Result<Expr, ParseError> {
        match self.peek() {
            // Acceso a campo: objeto.campo
            Token::Punto => {
                self.advance();

                // Intentar obtener el nombre del campo (puede ser identificador o palabra reservada)
                let campo_opt = self.token_to_field_name(self.peek());

                match campo_opt {
                    Some(c) => {
                        self.advance();

                        // Verificar si hay acceso a arreglo después: obj.p[1]
                        if self.peek() == &Token::CorcheteIzq {
                            self.advance();
                            let indice = self.parse_expresion()?;
                            self.expect(Token::CorcheteDer)?;

                            // Crear identificador compuesto para el acceso
                            let campo_completo = format!("{}.{}", objeto, c);
                            return Ok(Expr::AccesoArreglo {
                                objeto: campo_completo,
                                indice: Box::new(indice),
                            });
                        }

                        Ok(Expr::AccesoCampo {
                            objeto,
                            campo: c,
                        })
                    }
                    None => Err(ParseError::new(
                        "Se esperaba nombre de campo después de '.'".into(),
                        self.current_location(),
                    )),
                }
            }

            // Acceso a arreglo: objeto[indice]
            Token::CorcheteIzq => {
                self.advance();
                let indice = self.parse_expresion()?;
                self.expect(Token::CorcheteDer)?;
                Ok(Expr::AccesoArreglo {
                    objeto,
                    indice: Box::new(indice),
                })
            }

            // Solo identificador
            _ => Ok(Expr::Identificador(objeto)),
        }
    }
}

// ============================================================================
// HELPER: Reportar errores de parseo
// ============================================================================

pub fn report_parse_errors(errors: &[ParseError], source: &str, filename: &str) {
    use crate::error::{Diagnostic, report_errors};

    let diagnostics: Vec<Diagnostic> = errors.iter().map(|err| {
        Diagnostic::syntax_error(
            err.location.line,
            err.location.column,
            err.location.length,
            err.message.clone()
        )
    }).collect();

    report_errors(&diagnostics, source, filename);
}
