// ============================================================================
// INTÉRPRETE - FASE 4
// ============================================================================
// Ejecuta el programa después del análisis semántico
// Mantiene el estado de la red (máquinas, concentradores, coaxiales)
// Evalúa expresiones y ejecuta sentencias

use crate::ast::*;
use crate::semantic::SymbolTable;
use std::collections::HashMap;

// ============================================================================
// VALORES EN RUNTIME
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    String(String),
    Bool(bool),
    Void,
}

impl Value {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Value::Int(n) => Some(*n),
            Value::Bool(b) => Some(if *b { 1 } else { 0 }),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            Value::Int(n) => Some(*n != 0),
            _ => None,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Void => "void".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        self.as_string()
    }
}

// ============================================================================
// OBJETOS EN RUNTIME
// ============================================================================

#[derive(Debug, Clone)]
pub struct RuntimeMaquina {
    pub nombre: String,
    pub x: i32,
    pub y: i32,
    pub colocada: bool,
    pub conectada_a: Option<ConexionMaquina>,
}

#[derive(Debug, Clone)]
pub enum ConexionMaquina {
    Puerto { concentrador: String, puerto: i32 },
    Coaxial { coaxial: String, posicion: i32 },
}

#[derive(Debug, Clone)]
pub struct RuntimeConcentrador {
    pub nombre: String,
    pub puertos: i32,
    pub tiene_coaxial: bool,
    pub x: i32,
    pub y: i32,
    pub colocado: bool,
    pub puertos_ocupados: Vec<bool>,
    pub disponibles: i32,
    pub coaxial_asignado: Option<String>,
}

impl RuntimeConcentrador {
    pub fn new(nombre: String, puertos: i32, tiene_coaxial: bool) -> Self {
        Self {
            nombre,
            puertos,
            tiene_coaxial,
            x: 0,
            y: 0,
            colocado: false,
            puertos_ocupados: vec![false; puertos as usize],
            disponibles: puertos,
            coaxial_asignado: None,
        }
    }

    pub fn asignar_puerto(&mut self, puerto: usize) -> bool {
        if puerto > 0 && puerto <= self.puertos as usize && !self.puertos_ocupados[puerto - 1] {
            self.puertos_ocupados[puerto - 1] = true;
            self.disponibles -= 1;
            true
        } else {
            false
        }
    }

    pub fn primer_puerto_disponible(&self) -> Option<usize> {
        for (i, &ocupado) in self.puertos_ocupados.iter().enumerate() {
            if !ocupado {
                return Some(i + 1);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeCoaxial {
    pub nombre: String,
    pub longitud: i32,
    pub x: i32,
    pub y: i32,
    pub direccion: String,
    pub colocado: bool,
    pub maquinas: Vec<(String, i32)>, // (nombre_maquina, posicion)
    pub completo: bool,
}

impl RuntimeCoaxial {
    pub fn new(nombre: String, longitud: i32) -> Self {
        Self {
            nombre,
            longitud,
            x: 0,
            y: 0,
            direccion: String::new(),
            colocado: false,
            maquinas: Vec::new(),
            completo: false,
        }
    }

    pub fn num_maquinas(&self) -> i32 {
        self.maquinas.len() as i32
    }

    pub fn agregar_maquina(&mut self, nombre: String, posicion: i32) {
        self.maquinas.push((nombre, posicion));
        // Marcar como completo si ya hay muchas máquinas (heurística)
        if self.maquinas.len() >= 10 {
            self.completo = true;
        }
    }
}

// ============================================================================
// AMBIENTE DE EJECUCIÓN
// ============================================================================

#[derive(Clone)]
pub struct Environment {
    pub maquinas: HashMap<String, RuntimeMaquina>,
    pub concentradores: HashMap<String, RuntimeConcentrador>,
    pub coaxiales: HashMap<String, RuntimeCoaxial>,
    pub modulos: HashMap<String, Vec<Statement>>,
    pub output: Vec<String>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            maquinas: HashMap::new(),
            concentradores: HashMap::new(),
            coaxiales: HashMap::new(),
            modulos: HashMap::new(),
            output: Vec::new(),
        }
    }

    pub fn inicializar_desde_simbolos(&mut self, symbol_table: &SymbolTable) {
        // Inicializar máquinas
        for (nombre, _) in &symbol_table.maquinas {
            self.maquinas.insert(nombre.clone(), RuntimeMaquina {
                nombre: nombre.clone(),
                x: 0,
                y: 0,
                colocada: false,
                conectada_a: None,
            });
        }

        // Inicializar concentradores
        for (nombre, sym) in &symbol_table.concentradores {
            self.concentradores.insert(
                nombre.clone(),
                RuntimeConcentrador::new(nombre.clone(), sym.puertos, sym.tiene_coaxial)
            );
        }

        // Inicializar coaxiales
        for (nombre, sym) in &symbol_table.coaxiales {
            self.coaxiales.insert(
                nombre.clone(),
                RuntimeCoaxial::new(nombre.clone(), sym.longitud)
            );
        }
    }

    pub fn escribir(&mut self, mensaje: String) {
        self.output.push(mensaje);
    }

    pub fn obtener_output(&self) -> String {
        self.output.join("\n")
    }
}

// ============================================================================
// INTÉRPRETE
// ============================================================================

pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn new(symbol_table: &SymbolTable) -> Self {
        let mut env = Environment::new();
        env.inicializar_desde_simbolos(symbol_table);
        Self { env }
    }

    pub fn ejecutar(&mut self, program: &Program) -> Result<(), String> {
        // Registrar módulos
        for modulo in &program.modulos {
            self.env.modulos.insert(modulo.nombre.clone(), modulo.sentencias.clone());
        }

        // Ejecutar sentencias principales
        for stmt in &program.sentencias {
            self.exec_statement(stmt)?;
        }

        Ok(())
    }

    // Evaluar expresiones
    pub fn eval_expression(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Numero(n) => Ok(Value::Int(*n)),

            Expr::Cadena(s) => Ok(Value::String(s.clone())),

            Expr::Identificador(nombre) => {
                // Los identificadores de objetos no se pueden evaluar directamente
                Err(format!("No se puede evaluar el identificador '{}' como valor", nombre))
            }

            Expr::AccesoCampo { objeto, campo } => {
                self.eval_campo_acceso(objeto, campo)
            }

            Expr::AccesoArreglo { objeto, indice } => {
                self.eval_arreglo_acceso(objeto, indice)
            }

            Expr::Relacional { izq, op, der } => {
                let val_izq = self.eval_expression(izq)?;
                let val_der = self.eval_expression(der)?;
                self.eval_relacional(&val_izq, op, &val_der)
            }

            Expr::Logico { izq, op, der } => {
                let val_izq = self.eval_expression(izq)?;
                let val_der = self.eval_expression(der)?;
                self.eval_logico(&val_izq, op, &val_der)
            }

            Expr::Not(expr) => {
                let val = self.eval_expression(expr)?;
                match val.as_bool() {
                    Some(b) => Ok(Value::Bool(!b)),
                    None => Err(format!("No se puede aplicar NOT a {:?}", val))
                }
            }
        }
    }

    fn eval_campo_acceso(&self, objeto: &str, campo: &str) -> Result<Value, String> {
        // Verificar si es un concentrador
        if let Some(conc) = self.env.concentradores.get(objeto) {
            return match campo {
                "puertos" => Ok(Value::Int(conc.puertos)),
                "disponibles" => Ok(Value::Int(conc.disponibles)),
                "presente" => Ok(Value::Bool(conc.colocado)),
                "coaxial" => {
                    if conc.tiene_coaxial {
                        Ok(Value::Int(1))
                    } else {
                        Ok(Value::Int(0))
                    }
                }
                _ => Err(format!("Campo '{}' no válido para concentrador", campo))
            };
        }

        // Verificar si es un coaxial
        if let Some(coax) = self.env.coaxiales.get(objeto) {
            return match campo {
                "longitud" => Ok(Value::Int(coax.longitud)),
                "completo" => Ok(Value::Bool(coax.completo)),
                "num" => Ok(Value::Int(coax.num_maquinas())),
                "presente" => Ok(Value::Bool(coax.colocado)),
                _ => Err(format!("Campo '{}' no válido para coaxial", campo))
            };
        }

        Err(format!("Objeto '{}' no encontrado", objeto))
    }

    fn eval_arreglo_acceso(&self, objeto: &str, indice: &Expr) -> Result<Value, String> {
        // Evaluar índice
        let idx_val = self.eval_expression(indice)?;
        let idx = idx_val.as_int().ok_or("El índice debe ser entero")?;

        // Acceso a arreglo p[] de concentradores
        if objeto.contains('.') {
            let parts: Vec<&str> = objeto.split('.').collect();
            if parts.len() == 2 && parts[1] == "p" {
                let conc_nombre = parts[0];
                if let Some(conc) = self.env.concentradores.get(conc_nombre) {
                    if idx > 0 && idx <= conc.puertos {
                        let ocupado = conc.puertos_ocupados[(idx - 1) as usize];
                        return Ok(Value::Bool(ocupado));
                    } else {
                        return Err(format!("Índice {} fuera de rango para concentrador '{}'", idx, conc_nombre));
                    }
                }
            }
        }

        Err(format!("Acceso a arreglo inválido: '{}'", objeto))
    }

    fn eval_relacional(&self, izq: &Value, op: &OpRelacional, der: &Value) -> Result<Value, String> {
        // Intentar comparar como enteros
        if let (Some(a), Some(b)) = (izq.as_int(), der.as_int()) {
            let resultado = match op {
                OpRelacional::Igual => a == b,
                OpRelacional::Diferente => a != b,
                OpRelacional::Menor => a < b,
                OpRelacional::Mayor => a > b,
                OpRelacional::MenorIgual => a <= b,
                OpRelacional::MayorIgual => a >= b,
            };
            return Ok(Value::Bool(resultado));
        }

        // Comparar como strings
        let resultado = match op {
            OpRelacional::Igual => izq.as_string() == der.as_string(),
            OpRelacional::Diferente => izq.as_string() != der.as_string(),
            _ => return Err(format!("No se puede comparar {:?} con {:?}", izq, der))
        };
        Ok(Value::Bool(resultado))
    }

    fn eval_logico(&self, izq: &Value, op: &OpLogico, der: &Value) -> Result<Value, String> {
        let a = izq.as_bool().ok_or("Operando izquierdo no es booleano")?;
        let b = der.as_bool().ok_or("Operando derecho no es booleano")?;

        let resultado = match op {
            OpLogico::And => a && b,
            OpLogico::Or => a || b,
        };

        Ok(Value::Bool(resultado))
    }

    // Ejecutar sentencias
    fn exec_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Coloca { objeto, x, y, .. } => {
                let x_val = self.eval_expression(x)?.as_int()
                    .ok_or("La coordenada X debe ser un entero")?;
                let y_val = self.eval_expression(y)?.as_int()
                    .ok_or("La coordenada Y debe ser un entero")?;

                // Colocar máquina
                if let Some(maq) = self.env.maquinas.get_mut(objeto) {
                    maq.x = x_val;
                    maq.y = y_val;
                    maq.colocada = true;
                    return Ok(());
                }

                // Colocar concentrador
                if let Some(conc) = self.env.concentradores.get_mut(objeto) {
                    conc.x = x_val;
                    conc.y = y_val;
                    conc.colocado = true;
                    return Ok(());
                }

                Err(format!("Objeto '{}' no encontrado", objeto))
            }

            Statement::ColocaCoaxial { coaxial, x, y, direccion, .. } => {
                let x_val = self.eval_expression(x)?.as_int()
                    .ok_or("La coordenada X debe ser un entero")?;
                let y_val = self.eval_expression(y)?.as_int()
                    .ok_or("La coordenada Y debe ser un entero")?;

                if let Some(coax) = self.env.coaxiales.get_mut(coaxial) {
                    coax.x = x_val;
                    coax.y = y_val;
                    coax.direccion = format!("{:?}", direccion);
                    coax.colocado = true;
                    Ok(())
                } else {
                    Err(format!("Coaxial '{}' no encontrado", coaxial))
                }
            }

            Statement::ColocaCoaxialConcentrador { coaxial, concentrador, .. } => {
                // Verificar que el concentrador tenga salida coaxial
                if let Some(conc) = self.env.concentradores.get_mut(concentrador) {
                    if !conc.tiene_coaxial {
                        return Err(format!("El concentrador '{}' no tiene salida para coaxial", concentrador));
                    }
                    conc.coaxial_asignado = Some(coaxial.clone());
                } else {
                    return Err(format!("Concentrador '{}' no encontrado", concentrador));
                }

                // Conectar el coaxial al concentrador
                if let Some(_coax) = self.env.coaxiales.get(coaxial) {
                    Ok(())
                } else {
                    Err(format!("Coaxial '{}' no encontrado", coaxial))
                }
            }

            Statement::UneMaquinaPuerto { maquina, concentrador, puerto, .. } => {
                let puerto_num = self.eval_expression(puerto)?.as_int()
                    .ok_or("El puerto debe ser un entero")?;

                // Verificar que la máquina existe
                if !self.env.maquinas.contains_key(maquina) {
                    return Err(format!("Máquina '{}' no encontrada", maquina));
                }

                // Asignar puerto en el concentrador
                if let Some(conc) = self.env.concentradores.get_mut(concentrador) {
                    if conc.asignar_puerto(puerto_num as usize) {
                        // Conectar la máquina
                        if let Some(maq) = self.env.maquinas.get_mut(maquina) {
                            maq.conectada_a = Some(ConexionMaquina::Puerto {
                                concentrador: concentrador.clone(),
                                puerto: puerto_num,
                            });
                        }
                        Ok(())
                    } else {
                        Err(format!("No se pudo asignar el puerto {} del concentrador '{}'", puerto_num, concentrador))
                    }
                } else {
                    Err(format!("Concentrador '{}' no encontrado", concentrador))
                }
            }

            Statement::AsignaPuerto { maquina, concentrador, .. } => {
                // Verificar que la máquina existe
                if !self.env.maquinas.contains_key(maquina) {
                    return Err(format!("Máquina '{}' no encontrada", maquina));
                }

                // Buscar primer puerto disponible
                if let Some(conc) = self.env.concentradores.get_mut(concentrador) {
                    if let Some(puerto) = conc.primer_puerto_disponible() {
                        conc.asignar_puerto(puerto);
                        // Conectar la máquina
                        if let Some(maq) = self.env.maquinas.get_mut(maquina) {
                            maq.conectada_a = Some(ConexionMaquina::Puerto {
                                concentrador: concentrador.clone(),
                                puerto: puerto as i32,
                            });
                        }
                        Ok(())
                    } else {
                        Err(format!("No hay puertos disponibles en el concentrador '{}'", concentrador))
                    }
                } else {
                    Err(format!("Concentrador '{}' no encontrado", concentrador))
                }
            }

            Statement::MaquinaCoaxial { maquina, coaxial, posicion, .. } => {
                let pos_val = self.eval_expression(posicion)?.as_int()
                    .ok_or("La posición debe ser un entero")?;

                // Verificar que la máquina existe
                if !self.env.maquinas.contains_key(maquina) {
                    return Err(format!("Máquina '{}' no encontrada", maquina));
                }

                // Agregar máquina al coaxial
                if let Some(coax) = self.env.coaxiales.get_mut(coaxial) {
                    coax.agregar_maquina(maquina.clone(), pos_val);
                    // Conectar la máquina
                    if let Some(maq) = self.env.maquinas.get_mut(maquina) {
                        maq.conectada_a = Some(ConexionMaquina::Coaxial {
                            coaxial: coaxial.clone(),
                            posicion: pos_val,
                        });
                    }
                    Ok(())
                } else {
                    Err(format!("Coaxial '{}' no encontrado", coaxial))
                }
            }

            Statement::AsignaMaquinaCoaxial { maquina, coaxial, .. } => {
                // Verificar que la máquina existe
                if !self.env.maquinas.contains_key(maquina) {
                    return Err(format!("Máquina '{}' no encontrada", maquina));
                }

                // Buscar posición disponible en el coaxial (heurística simple)
                if let Some(coax) = self.env.coaxiales.get_mut(coaxial) {
                    // Buscar la primera posición disponible con separación de 3m
                    let mut posicion = 0;
                    loop {
                        let mut valido = true;
                        for (_, pos_existente) in &coax.maquinas {
                            if (posicion - pos_existente).abs() < 3 {
                                valido = false;
                                break;
                            }
                        }
                        if valido && posicion <= coax.longitud {
                            break;
                        }
                        posicion += 3;
                        if posicion > coax.longitud {
                            return Err(format!("No hay posiciones disponibles en el coaxial '{}'", coaxial));
                        }
                    }

                    coax.agregar_maquina(maquina.clone(), posicion);
                    // Conectar la máquina
                    if let Some(maq) = self.env.maquinas.get_mut(maquina) {
                        maq.conectada_a = Some(ConexionMaquina::Coaxial {
                            coaxial: coaxial.clone(),
                            posicion,
                        });
                    }
                    Ok(())
                } else {
                    Err(format!("Coaxial '{}' no encontrado", coaxial))
                }
            }

            Statement::Escribe { contenido, .. } => {
                let valor = self.eval_expression(contenido)?;
                self.env.escribir(valor.as_string());
                Ok(())
            }

            Statement::Si { condicion, entonces, sino, .. } => {
                let cond_val = self.eval_expression(condicion)?;
                let es_verdadero = cond_val.as_bool()
                    .ok_or("La condición debe ser booleana")?;

                if es_verdadero {
                    for stmt in entonces {
                        self.exec_statement(stmt)?;
                    }
                } else if let Some(sino_stmts) = sino {
                    for stmt in sino_stmts {
                        self.exec_statement(stmt)?;
                    }
                }
                Ok(())
            }

            Statement::LlamadaModulo { nombre, .. } => {
                // Obtener las sentencias del módulo
                if let Some(stmts) = self.env.modulos.get(nombre).cloned() {
                    for stmt in &stmts {
                        self.exec_statement(stmt)?;
                    }
                    Ok(())
                } else {
                    Err(format!("Módulo '{}' no encontrado", nombre))
                }
            }
        }
    }
}
