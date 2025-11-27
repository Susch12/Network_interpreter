// Análisis Semántico - Fase 3.1
// Valida que el programa sea semánticamente correcto

use crate::ast::*;
use std::collections::HashMap;

// ============================================================================
// SISTEMA DE TIPOS
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    String,
    Bool,
    Void,
    Maquina,
    Concentrador,
    Coaxial,
    Unknown,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int => "Int".to_string(),
            Type::String => "String".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Void => "Void".to_string(),
            Type::Maquina => "Maquina".to_string(),
            Type::Concentrador => "Concentrador".to_string(),
            Type::Coaxial => "Coaxial".to_string(),
            Type::Unknown => "Unknown".to_string(),
        }
    }
}

// ============================================================================
// SÍMBOLOS
// ============================================================================

#[derive(Debug, Clone)]
pub struct MaquinaSymbol {
    pub nombre: String,
    pub presente: bool,        // Si fue colocada en pantalla
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct ConcentradorSymbol {
    pub nombre: String,
    pub puertos: i32,          // Total de puertos (4, 8, 16)
    pub tiene_coaxial: bool,   // Si tiene salida coaxial
    pub puertos_ocupados: Vec<bool>,  // Estado de cada puerto (true = ocupado)
    pub disponibles: i32,      // Puertos disponibles
    pub coaxial_asignado: Option<String>,  // Nombre del coaxial asignado
    pub presente: bool,        // Si fue colocado en pantalla
    pub location: Location,
}

impl ConcentradorSymbol {
    pub fn new(nombre: String, puertos: i32, tiene_coaxial: bool, location: Location) -> Self {
        let mut puertos_ocupados = vec![false; puertos as usize];
        Self {
            nombre,
            puertos,
            tiene_coaxial,
            puertos_ocupados,
            disponibles: puertos,
            coaxial_asignado: None,
            presente: false,
            location,
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
        for (i, ocupado) in self.puertos_ocupados.iter().enumerate() {
            if !*ocupado {
                return Some(i + 1);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct CoaxialSymbol {
    pub nombre: String,
    pub longitud: i32,                    // En metros
    pub completo: bool,                   // Si ya no acepta más máquinas
    pub num_maquinas: usize,              // Número de máquinas conectadas
    pub maquinas: Vec<String>,            // Nombres de máquinas
    pub posiciones: Vec<i32>,             // Posiciones de cada máquina
    pub presente: bool,                   // Si fue colocado en pantalla
    pub location: Location,
}

impl CoaxialSymbol {
    pub fn new(nombre: String, longitud: i32, location: Location) -> Self {
        Self {
            nombre,
            longitud,
            completo: false,
            num_maquinas: 0,
            maquinas: Vec::new(),
            posiciones: Vec::new(),
            presente: false,
            location,
        }
    }

    pub fn puede_agregar_maquina(&self, posicion: i32) -> Result<(), String> {
        // Validar longitud del cable (3m - 500m)
        if self.longitud < 3 {
            return Err(format!("Cable coaxial muy corto (mínimo 3m): {}m", self.longitud));
        }
        if self.longitud > 500 {
            return Err(format!("Cable coaxial muy largo (máximo 500m): {}m", self.longitud));
        }

        // Validar posición dentro del cable
        if posicion < 0 || posicion > self.longitud {
            return Err(format!("Posición {}m fuera del rango del cable (0-{}m)",
                             posicion, self.longitud));
        }

        // Validar separación de 3m con otras máquinas
        for &pos in &self.posiciones {
            if (pos - posicion).abs() < 3 {
                return Err(format!("Máquina muy cerca de otra (mínimo 3m de separación). Posición {}m conflicta con {}m",
                                 posicion, pos));
            }
        }

        Ok(())
    }

    pub fn agregar_maquina(&mut self, nombre: String, posicion: i32) {
        self.maquinas.push(nombre);
        self.posiciones.push(posicion);
        self.num_maquinas += 1;
    }

    pub fn encontrar_posicion_disponible(&self) -> Option<i32> {
        // Buscar una posición válida respetando separación de 3m
        for pos in (0..=self.longitud).step_by(3) {
            if self.puede_agregar_maquina(pos).is_ok() {
                return Some(pos);
            }
        }
        None
    }
}

// ============================================================================
// TABLA DE SÍMBOLOS
// ============================================================================

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub maquinas: HashMap<String, MaquinaSymbol>,
    pub concentradores: HashMap<String, ConcentradorSymbol>,
    pub coaxiales: HashMap<String, CoaxialSymbol>,
    pub modulos: HashMap<String, Location>,  // Nombre -> ubicación del módulo
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            maquinas: HashMap::new(),
            concentradores: HashMap::new(),
            coaxiales: HashMap::new(),
            modulos: HashMap::new(),
        }
    }

    // ========== Máquinas ==========

    pub fn definir_maquina(&mut self, nombre: String, location: Location) -> Result<(), String> {
        if self.maquinas.contains_key(&nombre) {
            return Err(format!("Máquina '{}' ya fue definida", nombre));
        }
        if self.concentradores.contains_key(&nombre) {
            return Err(format!("El nombre '{}' ya está en uso por un concentrador", nombre));
        }
        if self.coaxiales.contains_key(&nombre) {
            return Err(format!("El nombre '{}' ya está en uso por un coaxial", nombre));
        }

        self.maquinas.insert(nombre.clone(), MaquinaSymbol {
            nombre,
            presente: false,
            location,
        });
        Ok(())
    }

    pub fn obtener_maquina(&self, nombre: &str) -> Option<&MaquinaSymbol> {
        self.maquinas.get(nombre)
    }

    pub fn obtener_maquina_mut(&mut self, nombre: &str) -> Option<&mut MaquinaSymbol> {
        self.maquinas.get_mut(nombre)
    }

    // ========== Concentradores ==========

    pub fn definir_concentrador(&mut self, nombre: String, puertos: i32, tiene_coaxial: bool, location: Location) -> Result<(), String> {
        if self.concentradores.contains_key(&nombre) {
            return Err(format!("Concentrador '{}' ya fue definido", nombre));
        }
        if self.maquinas.contains_key(&nombre) {
            return Err(format!("El nombre '{}' ya está en uso por una máquina", nombre));
        }
        if self.coaxiales.contains_key(&nombre) {
            return Err(format!("El nombre '{}' ya está en uso por un coaxial", nombre));
        }

        // Validar número de puertos (4, 8, 16)
        if puertos != 4 && puertos != 8 && puertos != 16 {
            return Err(format!("Número de puertos inválido: {}. Debe ser 4, 8 o 16", puertos));
        }

        self.concentradores.insert(nombre.clone(), ConcentradorSymbol::new(nombre, puertos, tiene_coaxial, location));
        Ok(())
    }

    pub fn obtener_concentrador(&self, nombre: &str) -> Option<&ConcentradorSymbol> {
        self.concentradores.get(nombre)
    }

    pub fn obtener_concentrador_mut(&mut self, nombre: &str) -> Option<&mut ConcentradorSymbol> {
        self.concentradores.get_mut(nombre)
    }

    // ========== Coaxiales ==========

    pub fn definir_coaxial(&mut self, nombre: String, longitud: i32, location: Location) -> Result<(), String> {
        if self.coaxiales.contains_key(&nombre) {
            return Err(format!("Coaxial '{}' ya fue definido", nombre));
        }
        if self.maquinas.contains_key(&nombre) {
            return Err(format!("El nombre '{}' ya está en uso por una máquina", nombre));
        }
        if self.concentradores.contains_key(&nombre) {
            return Err(format!("El nombre '{}' ya está en uso por un concentrador", nombre));
        }

        // Validar reglas Ethernet: longitud del cable debe estar entre 3m y 500m
        if longitud < 3 {
            return Err(format!("Longitud de cable coaxial inválida: {}m. La longitud mínima según reglas Ethernet es 3m", longitud));
        }
        if longitud > 500 {
            return Err(format!("Longitud de cable coaxial inválida: {}m. La longitud máxima según reglas Ethernet es 500m", longitud));
        }

        self.coaxiales.insert(nombre.clone(), CoaxialSymbol::new(nombre, longitud, location));
        Ok(())
    }

    pub fn obtener_coaxial(&self, nombre: &str) -> Option<&CoaxialSymbol> {
        self.coaxiales.get(nombre)
    }

    pub fn obtener_coaxial_mut(&mut self, nombre: &str) -> Option<&mut CoaxialSymbol> {
        self.coaxiales.get_mut(nombre)
    }

    // ========== Módulos ==========

    pub fn definir_modulo(&mut self, nombre: String, location: Location) -> Result<(), String> {
        if self.modulos.contains_key(&nombre) {
            return Err(format!("Módulo '{}' ya fue definido", nombre));
        }
        self.modulos.insert(nombre, location);
        Ok(())
    }

    pub fn existe_modulo(&self, nombre: &str) -> bool {
        self.modulos.contains_key(nombre)
    }
}

// ============================================================================
// ERRORES SEMÁNTICOS
// ============================================================================

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
    pub location: Location,
}

impl SemanticError {
    pub fn new(message: String, location: Location) -> Self {
        Self { message, location }
    }
}

// ============================================================================
// ANALIZADOR SEMÁNTICO
// ============================================================================

pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
    pub errors: Vec<SemanticError>,
    // Rastrear asignaciones de máquinas a cables coaxiales durante el análisis
    // Mapa: nombre_coaxial -> Vec<(nombre_maquina, posicion)>
    coaxial_assignments: std::collections::HashMap<String, Vec<(String, i32)>>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            coaxial_assignments: std::collections::HashMap::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<SemanticError>> {
        // Paso 1: Analizar definiciones
        self.analyze_definitions(&program.definiciones);

        // Paso 2a: Registrar todos los módulos primero (sin analizar contenido)
        // Esto permite que los módulos se llamen entre sí sin importar el orden
        for modulo in &program.modulos {
            if let Err(msg) = self.symbol_table.definir_modulo(modulo.nombre.clone(), modulo.location.clone()) {
                self.errors.push(SemanticError::new(msg, modulo.location.clone()));
            }
        }

        // Paso 2b: Ahora analizar el contenido de los módulos
        for modulo in &program.modulos {
            for stmt in &modulo.sentencias {
                self.analyze_statement(stmt);
            }
        }

        // Paso 3: Analizar sentencias principales
        for stmt in &program.sentencias {
            self.analyze_statement(stmt);
        }

        // Retornar errores si los hay
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    // ========== Análisis de Definiciones ==========

    fn analyze_definitions(&mut self, defs: &Definitions) {
        // Definir máquinas
        for maq in &defs.maquinas {
            if let Err(msg) = self.symbol_table.definir_maquina(maq.nombre.clone(), maq.location.clone()) {
                self.errors.push(SemanticError::new(msg, maq.location.clone()));
            }
        }

        // Definir concentradores
        for conc in &defs.concentradores {
            if let Err(msg) = self.symbol_table.definir_concentrador(
                conc.nombre.clone(),
                conc.puertos,
                conc.tiene_coaxial,
                conc.location.clone()
            ) {
                self.errors.push(SemanticError::new(msg, conc.location.clone()));
            }
        }

        // Definir coaxiales
        for coax in &defs.coaxiales {
            if let Err(msg) = self.symbol_table.definir_coaxial(
                coax.nombre.clone(),
                coax.longitud,
                coax.location.clone()
            ) {
                self.errors.push(SemanticError::new(msg, coax.location.clone()));
            }
        }
    }

    // ========== Análisis de Módulos ==========

    fn analyze_module(&mut self, modulo: &Modulo) {
        // Registrar módulo
        if let Err(msg) = self.symbol_table.definir_modulo(modulo.nombre.clone(), modulo.location.clone()) {
            self.errors.push(SemanticError::new(msg, modulo.location.clone()));
        }

        // Analizar sentencias del módulo
        for stmt in &modulo.sentencias {
            self.analyze_statement(stmt);
        }
    }

    // ========== Análisis de Sentencias ==========

    fn analyze_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Coloca { objeto, x, y, location } => {
                self.check_object_exists(objeto, location);
                self.check_expression(x, &Type::Int, location);
                self.check_expression(y, &Type::Int, location);
            }

            Statement::ColocaCoaxial { coaxial, x, y, direccion: _, location } => {
                self.check_coaxial_exists(coaxial, location);
                self.check_expression(x, &Type::Int, location);
                self.check_expression(y, &Type::Int, location);
            }

            Statement::ColocaCoaxialConcentrador { coaxial, concentrador, location } => {
                self.check_coaxial_exists(coaxial, location);
                self.check_concentrador_exists(concentrador, location);

                // Validar que el concentrador tenga salida coaxial
                if let Some(conc) = self.symbol_table.obtener_concentrador(concentrador) {
                    if !conc.tiene_coaxial {
                        self.errors.push(SemanticError::new(
                            format!("El concentrador '{}' no tiene salida para coaxial", concentrador),
                            location.clone()
                        ));
                    }
                }
            }

            Statement::UneMaquinaPuerto { maquina, concentrador, puerto, location } => {
                // El primer argumento puede ser una máquina O un concentrador (para cascada)
                self.check_maquina_or_concentrador_exists(maquina, location);
                self.check_concentrador_exists(concentrador, location);
                self.check_expression(puerto, &Type::Int, location);
            }

            Statement::AsignaPuerto { maquina, concentrador, location } => {
                // El primer argumento puede ser una máquina O un concentrador (para cascada)
                self.check_maquina_or_concentrador_exists(maquina, location);
                self.check_concentrador_exists(concentrador, location);
            }

            Statement::MaquinaCoaxial { maquina, coaxial, posicion, location } => {
                self.check_maquina_exists(maquina, location);
                self.check_coaxial_exists(coaxial, location);

                // Inferir la posición (debe ser un número entero)
                let tipo_pos = self.check_expression(posicion, &Type::Int, location);

                // Validar reglas Ethernet para colocar máquina en coaxial
                if tipo_pos == Type::Int {
                    if let Expr::Numero(pos_val) = posicion {
                        self.validate_maquina_coaxial_placement(maquina, coaxial, *pos_val, location);
                    }
                }
            }

            Statement::AsignaMaquinaCoaxial { maquina, coaxial, location } => {
                self.check_maquina_exists(maquina, location);
                self.check_coaxial_exists(coaxial, location);
            }

            Statement::Escribe { contenido, location } => {
                self.check_expression(contenido, &Type::Unknown, location);
            }

            Statement::Si { condicion, entonces, sino, location } => {
                self.check_expression(condicion, &Type::Bool, location);

                for stmt in entonces {
                    self.analyze_statement(stmt);
                }

                if let Some(sino_stmts) = sino {
                    for stmt in sino_stmts {
                        self.analyze_statement(stmt);
                    }
                }
            }

            Statement::LlamadaModulo { nombre, location } => {
                if !self.symbol_table.existe_modulo(nombre) {
                    self.errors.push(SemanticError::new(
                        format!("Módulo '{}' no está definido", nombre),
                        location.clone()
                    ));
                }
            }
        }
    }

    // ========== Validación de Expresiones ==========

    fn check_expression(&mut self, expr: &Expr, expected_type: &Type, location: &Location) -> Type {
        let actual_type = self.infer_expression_type(expr, location);

        // Validar tipo si no es Unknown (Unknown permite cualquier tipo)
        if expected_type != &Type::Unknown && actual_type != Type::Unknown {
            if !self.types_are_compatible(&actual_type, expected_type) {
                self.errors.push(SemanticError::new(
                    format!("Incompatibilidad de tipos: se esperaba '{}' pero se encontró '{}'",
                            expected_type.to_string(),
                            actual_type.to_string()),
                    location.clone()
                ));
            }
        }

        actual_type
    }

    fn infer_expression_type(&mut self, expr: &Expr, location: &Location) -> Type {
        match expr {
            Expr::Numero(_) => Type::Int,

            Expr::Cadena(_) => Type::String,

            Expr::Identificador(nombre) => {
                // Verificar que el identificador existe
                if self.symbol_table.obtener_maquina(nombre).is_some() {
                    Type::Maquina
                } else if self.symbol_table.obtener_concentrador(nombre).is_some() {
                    Type::Concentrador
                } else if self.symbol_table.obtener_coaxial(nombre).is_some() {
                    Type::Coaxial
                } else {
                    self.errors.push(SemanticError::new(
                        format!("Identificador '{}' no está definido", nombre),
                        location.clone()
                    ));
                    Type::Unknown
                }
            }

            Expr::AccesoCampo { objeto, campo } => {
                // Validar acceso a campos de concentradores y coaxiales
                if let Some(_) = self.symbol_table.obtener_concentrador(objeto) {
                    match campo.as_str() {
                        "puertos" | "disponibles" | "presente" | "coaxial" => Type::Int,
                        _ => {
                            self.errors.push(SemanticError::new(
                                format!("Campo '{}' no existe en concentrador '{}'. Campos válidos: puertos, disponibles, presente, coaxial",
                                        campo, objeto),
                                location.clone()
                            ));
                            Type::Unknown
                        }
                    }
                } else if let Some(_) = self.symbol_table.obtener_coaxial(objeto) {
                    match campo.as_str() {
                        "longitud" | "completo" | "num" | "presente" => Type::Int,
                        _ => {
                            self.errors.push(SemanticError::new(
                                format!("Campo '{}' no existe en coaxial '{}'. Campos válidos: longitud, completo, num, presente",
                                        campo, objeto),
                                location.clone()
                            ));
                            Type::Unknown
                        }
                    }
                } else {
                    self.errors.push(SemanticError::new(
                        format!("Objeto '{}' no está definido o no soporta acceso a campos", objeto),
                        location.clone()
                    ));
                    Type::Unknown
                }
            }

            Expr::AccesoArreglo { objeto, indice } => {
                self.check_expression(indice, &Type::Int, location);

                // Validar acceso a arreglo p[] de concentradores
                if objeto.contains('.') {
                    let parts: Vec<&str> = objeto.split('.').collect();
                    if parts.len() == 2 && parts[1] == "p" {
                        if self.symbol_table.obtener_concentrador(parts[0]).is_some() {
                            Type::Bool
                        } else {
                            self.errors.push(SemanticError::new(
                                format!("Concentrador '{}' no está definido", parts[0]),
                                location.clone()
                            ));
                            Type::Unknown
                        }
                    } else {
                        self.errors.push(SemanticError::new(
                            format!("Acceso a arreglo inválido: '{}'", objeto),
                            location.clone()
                        ));
                        Type::Unknown
                    }
                } else {
                    Type::Unknown
                }
            }

            Expr::Relacional { izq, op, der } => {
                let tipo_izq = self.infer_expression_type(izq, location);
                let tipo_der = self.infer_expression_type(der, location);

                // Validar que los tipos sean compatibles para comparación
                if tipo_izq != Type::Unknown && tipo_der != Type::Unknown {
                    if !self.types_are_compatible(&tipo_izq, &tipo_der) {
                        self.errors.push(SemanticError::new(
                            format!("No se pueden comparar tipos incompatibles: '{}' {:?} '{}'",
                                    tipo_izq.to_string(),
                                    op,
                                    tipo_der.to_string()),
                            location.clone()
                        ));
                    }
                }
                Type::Bool
            }

            Expr::Logico { izq, op: _, der } => {
                self.check_expression(izq, &Type::Bool, location);
                self.check_expression(der, &Type::Bool, location);
                Type::Bool
            }

            Expr::Not(expr) => {
                self.check_expression(expr, &Type::Bool, location);
                Type::Bool
            }
        }
    }

    // ========== Helpers ==========

    fn types_are_compatible(&self, actual: &Type, expected: &Type) -> bool {
        // Unknown es compatible con todo (permisivo)
        if actual == &Type::Unknown || expected == &Type::Unknown {
            return true;
        }

        // Mismo tipo exacto
        if actual == expected {
            return true;
        }

        // Reglas especiales de compatibilidad
        match (actual, expected) {
            // Int puede usarse como Bool en contextos lógicos (0 = false, != 0 = true)
            (Type::Int, Type::Bool) => true,
            // Bool puede convertirse a Int (false = 0, true = 1)
            (Type::Bool, Type::Int) => true,
            // Cualquier otro caso es incompatible
            _ => false,
        }
    }

    fn check_maquina_exists(&mut self, nombre: &str, location: &Location) {
        if self.symbol_table.obtener_maquina(nombre).is_none() {
            self.errors.push(SemanticError::new(
                format!("Máquina '{}' no está definida", nombre),
                location.clone()
            ));
        }
    }

    fn check_concentrador_exists(&mut self, nombre: &str, location: &Location) {
        if self.symbol_table.obtener_concentrador(nombre).is_none() {
            self.errors.push(SemanticError::new(
                format!("Concentrador '{}' no está definido", nombre),
                location.clone()
            ));
        }
    }

    fn check_coaxial_exists(&mut self, nombre: &str, location: &Location) {
        if self.symbol_table.obtener_coaxial(nombre).is_none() {
            self.errors.push(SemanticError::new(
                format!("Coaxial '{}' no está definido", nombre),
                location.clone()
            ));
        }
    }

    fn check_object_exists(&mut self, nombre: &str, location: &Location) {
        if self.symbol_table.obtener_maquina(nombre).is_none()
            && self.symbol_table.obtener_concentrador(nombre).is_none() {
            self.errors.push(SemanticError::new(
                format!("Objeto '{}' no está definido (no es máquina ni concentrador)", nombre),
                location.clone()
            ));
        }
    }

    fn check_maquina_or_concentrador_exists(&mut self, nombre: &str, location: &Location) {
        // Para uneMaquinaPuerto, el primer argumento puede ser:
        // - Máquina (conexión normal)
        // - Concentrador (cascada de hubs)
        // - Coaxial (para conectar cable coaxial a un hub)
        if self.symbol_table.obtener_maquina(nombre).is_none()
            && self.symbol_table.obtener_concentrador(nombre).is_none()
            && self.symbol_table.obtener_coaxial(nombre).is_none() {
            self.errors.push(SemanticError::new(
                format!("'{}' no está definido (debe ser una máquina, concentrador o coaxial)", nombre),
                location.clone()
            ));
        }
    }

    // ========== Validaciones de Reglas Ethernet ==========

    fn validate_maquina_coaxial_placement(&mut self, maquina: &str, coaxial: &str, posicion: i32, location: &Location) {
        // Obtener información del coaxial
        if let Some(coax) = self.symbol_table.obtener_coaxial(coaxial) {
            let longitud = coax.longitud;

            // Regla 1: La posición debe estar dentro del rango del cable (0 a longitud)
            if posicion < 0 || posicion > longitud {
                self.errors.push(SemanticError::new(
                    format!("Posición inválida: {}m. La posición debe estar entre 0 y {} (longitud del cable '{}')",
                            posicion, longitud, coaxial),
                    location.clone()
                ));
                return;
            }

            // Regla 2: Cada máquina debe estar separada al menos 3m de otras máquinas
            // Usar el HashMap temporal para obtener las asignaciones durante el análisis
            if let Some(assignments) = self.coaxial_assignments.get(coaxial) {
                for (maq_existente, pos_existente) in assignments {
                    let distancia = (posicion - pos_existente).abs();
                    if distancia < 3 {
                        self.errors.push(SemanticError::new(
                            format!("Violación de regla Ethernet: La máquina '{}' está demasiado cerca ({}m) de la máquina '{}' en posición {}m. La separación mínima es 3m",
                                    maquina, distancia, maq_existente, pos_existente),
                            location.clone()
                        ));
                        return;
                    }
                }
            }

            // Registrar esta asignación si no hay errores
            self.coaxial_assignments
                .entry(coaxial.to_string())
                .or_insert_with(Vec::new)
                .push((maquina.to_string(), posicion));
        }
    }
}

// ============================================================================
// HELPER: Reportar errores semánticos
// ============================================================================

pub fn report_semantic_errors(errors: &[SemanticError], source: &str, filename: &str) {
    use crate::error::{Diagnostic, report_errors};

    let diagnostics: Vec<Diagnostic> = errors.iter().map(|err| {
        Diagnostic::semantic_error(
            err.location.line,
            err.location.column,
            err.location.length,
            err.message.clone()
        )
    }).collect();

    report_errors(&diagnostics, source, filename);
}
