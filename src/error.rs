// src/error.rs
// Sistema de manejo de errores con formato estilo rustc

use colored::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    LexicalError,
    SyntaxError,
    SemanticError,
    RuntimeError,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub message: String,
    pub help: Option<String>,
}

impl Diagnostic {
    pub fn lexical_error(line: usize, column: usize, length: usize, message: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::LexicalError,
            line,
            column,
            length,
            message,
            help: None,
        }
    }

    pub fn syntax_error(line: usize, column: usize, length: usize, message: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SyntaxError,
            line,
            column,
            length,
            message,
            help: None,
        }
    }

    pub fn semantic_error(line: usize, column: usize, length: usize, message: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::SemanticError,
            line,
            column,
            length,
            message,
            help: None,
        }
    }

    pub fn runtime_error(line: usize, column: usize, length: usize, message: String) -> Self {
        Diagnostic {
            kind: DiagnosticKind::RuntimeError,
            line,
            column,
            length,
            message,
            help: None,
        }
    }

    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind_str = match self.kind {
            DiagnosticKind::LexicalError => "Error Léxico",
            DiagnosticKind::SyntaxError => "Error Sintáctico",
            DiagnosticKind::SemanticError => "Error Semántico",
            DiagnosticKind::RuntimeError => "Error de Ejecución",
        };

        write!(
            f,
            "{} en línea {}, columna {}: {}",
            kind_str, self.line, self.column, self.message
        )
    }
}

/// Reporta un error al estilo rustc con contexto visual mejorado
pub fn report_error(error: &Diagnostic, source: &str, filename: &str) {
    let kind_label = match error.kind {
        DiagnosticKind::LexicalError => "error léxico",
        DiagnosticKind::SyntaxError => "error sintáctico",
        DiagnosticKind::SemanticError => "error semántico",
        DiagnosticKind::RuntimeError => "error de ejecución",
    };

    // Header: error: mensaje
    println!("{}{} {}",
             "error".red().bold(),
             ":".bold(),
             error.message.bold());

    // Ubicación: --> archivo:línea:columna
    println!("  {} {}:{}:{}",
             "-->".blue().bold(),
             filename,
             error.line,
             error.column);

    // Obtener las líneas del código fuente
    let lines: Vec<&str> = source.lines().collect();

    // Manejar error al final del archivo (línea 0)
    if error.line == 0 && !lines.is_empty() {
        let last_line_num = lines.len();
        let last_line = lines[last_line_num - 1];
        let line_num_width = last_line_num.to_string().len().max(2);

        println!("   {}", "|".blue().bold());

        // Mostrar última línea del archivo
        if last_line_num > 1 {
            println!("{:>width$} {} {}",
                     (last_line_num - 1).to_string().blue().bold(),
                     "|".blue().bold(),
                     lines[last_line_num - 2].dimmed(),
                     width = line_num_width);
        }

        println!("{:>width$} {} {}",
                 last_line_num.to_string().blue().bold(),
                 "|".blue().bold(),
                 last_line,
                 width = line_num_width);

        // Indicador después del final
        let spaces = " ".repeat(last_line.len());
        println!("{:>width$} {} {}{} {} (al final del archivo)",
                 "".blue().bold(),
                 "|".blue().bold(),
                 spaces,
                 "^".red().bold(),
                 kind_label.red().bold(),
                 width = line_num_width);

        println!("   {}", "|".blue().bold());
    } else if error.line > 0 && error.line <= lines.len() {
        let line_idx = error.line - 1;
        let line_content = lines[line_idx];

        // Calcular el ancho necesario para los números de línea
        let line_num_width = error.line.to_string().len().max(2);

        println!("   {}", "|".blue().bold());

        // Línea anterior (contexto)
        if line_idx > 0 {
            let prev_line_num = error.line - 1;
            println!("{:>width$} {} {}",
                     prev_line_num.to_string().blue().bold(),
                     "|".blue().bold(),
                     lines[line_idx - 1].dimmed(),
                     width = line_num_width);
        }

        // Línea con el error
        println!("{:>width$} {} {}",
                 error.line.to_string().blue().bold(),
                 "|".blue().bold(),
                 line_content,
                 width = line_num_width);

        // Indicador de error (^^^)
        let spaces = " ".repeat(error.column);
        let carets = "^".repeat(error.length.max(1));
        println!("{:>width$} {} {}{} {}",
                 "".blue().bold(),
                 "|".blue().bold(),
                 spaces,
                 carets.red().bold(),
                 kind_label.red().bold(),
                 width = line_num_width);

        // Línea siguiente (contexto)
        if line_idx + 1 < lines.len() {
            let next_line_num = error.line + 1;
            println!("{:>width$} {} {}",
                     next_line_num.to_string().blue().bold(),
                     "|".blue().bold(),
                     lines[line_idx + 1].dimmed(),
                     width = line_num_width);
        }

        println!("   {}", "|".blue().bold());
    }

    // Mensaje de ayuda si existe
    if let Some(ref help_msg) = error.help {
        println!("   {} {}: {}",
                 "=".blue().bold(),
                 "ayuda".bold(),
                 help_msg);
    }

    println!(); // Línea en blanco
}

/// Reporta múltiples errores
pub fn report_errors(errors: &[Diagnostic], source: &str, filename: &str) {
    for error in errors {
        report_error(error, source, filename);
    }

    let error_count = errors.len();
    println!("{}{} no se pudo compilar debido a {} error{}",
             "error".red().bold(),
             ":".bold(),
             error_count,
             if error_count == 1 { "" } else { "es" });
}
