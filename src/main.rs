use colored::*;
use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod ast;
mod error;
mod semantic;
mod interpreter;
mod visualizer;

use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;
use interpreter::{Interpreter, ConexionMaquina};

fn main() {
    println!("{}", "=== PROYECTO COMPLETO: FASES 1-5 ===".cyan().bold());
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", "Error: No se especificó archivo de entrada".red().bold());
        eprintln!("Uso: {} <archivo.net> [--visualize|-v]", args[0]);
        eprintln!("\n{}", "Opciones:".yellow());
        eprintln!("  {} o {}  - Mostrar visualización gráfica de la topología", "--visualize".green(), "-v".green());
        eprintln!("\n{}", "Ejemplos:".yellow());
        eprintln!("  {} test_interpreter_simple.net", args[0]);
        eprintln!("  {} test_interpreter_coaxial.net --visualize", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    // Leer archivo fuente
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}: {}", "Error al leer archivo".red().bold(), filename, e);
            process::exit(1);
        }
    };

    println!("{} {}\n", "Archivo:".green(), filename);
    
    // Mostrar estadísticas del código fuente
    let lines = source.lines().count();
    let chars = source.chars().count();
    let non_empty_lines = source.lines().filter(|l| !l.trim().is_empty()).count();
    
    println!("{}", "Estadísticas del código:".cyan());
    println!("  Líneas totales: {}", lines);
    println!("  Líneas no vacías: {}", non_empty_lines);
    println!("  Caracteres: {}", chars);
    println!();

    println!("{}", "Código fuente:".yellow());
    println!("{}", "─".repeat(80));
    
    // Mostrar con números de línea
    for (i, line) in source.lines().enumerate() {
        println!("{:3} | {}", (i + 1).to_string().blue(), line);
    }
    
    println!("{}", "─".repeat(80));
    println!();

    // ANÁLISIS LÉXICO
    println!("{}", "📝 Analizando léxicamente...".yellow().bold());
    
    let mut lexer = Lexer::new(source.clone());
    
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("{} {} tokens generados\n", "✓".green().bold(), tokens.len());
            
            // Estadísticas de tokens
            print_token_statistics(tokens);
            
            // Mostrar tabla de tokens
            println!("\n{}", "Tabla de Tokens:".cyan().bold());
            println!("{}", "─".repeat(90));
            println!("{:<6} {:<8} {:<25} {:<35}", 
                     "Línea", "Columna", "Token", "Lexema");
            println!("{}", "─".repeat(90));
            
            for token_info in tokens {
                let token_str = format!("{:?}", token_info.token);
                let token_display = if token_str.len() > 23 {
                    format!("{}...", &token_str[..20])
                } else {
                    token_str
                };
                
                let lexeme_display = if token_info.lexeme.len() > 33 {
                    format!("{}...", &token_info.lexeme[..30])
                } else {
                    token_info.lexeme.clone()
                };
                
                println!("{:<6} {:<8} {:<25} {:<35}", 
                         token_info.line,
                         token_info.column,
                         token_display,
                         lexeme_display);
            }
            println!("{}", "─".repeat(90));

            println!("\n{}", "✅ Análisis léxico completado exitosamente".green().bold());

            // ========== ANÁLISIS SINTÁCTICO ==========
            println!("\n{}", "📝 Analizando sintácticamente...".yellow().bold());

            let mut parser = Parser::new(tokens.clone());

            match parser.parse() {
                Ok(programa) => {
                    println!("{}", "✅ Análisis sintáctico completado exitosamente".green().bold());

                    // Mostrar AST
                    programa.pretty_print();

                    // ========== ANÁLISIS SEMÁNTICO ==========
                    println!("\n{}", "📝 Analizando semánticamente...".yellow().bold());

                    let mut semantic_analyzer = SemanticAnalyzer::new();

                    match semantic_analyzer.analyze(&programa) {
                        Ok(_) => {
                            println!("{}", "✅ Análisis semántico completado exitosamente".green().bold());

                            // Mostrar tabla de símbolos
                            print_symbol_table(&semantic_analyzer.symbol_table);

                            // ========== EJECUCIÓN DEL PROGRAMA ==========
                            println!("\n{}", "🚀 Ejecutando programa...".yellow().bold());

                            let mut interpreter = Interpreter::new(&semantic_analyzer.symbol_table);

                            match interpreter.ejecutar(&programa) {
                                Ok(_) => {
                                    println!("{}", "✅ Ejecución completada exitosamente".green().bold());

                                    // Mostrar output del programa
                                    let output = interpreter.env.obtener_output();
                                    if !output.is_empty() {
                                        println!("\n{}", "📤 OUTPUT DEL PROGRAMA:".cyan().bold());
                                        println!("{}", "═".repeat(80));
                                        println!("{}", output);
                                        println!("{}", "═".repeat(80));
                                    }

                                    // Mostrar estado de la red
                                    print_network_state(&interpreter.env);

                                    // Visualizar si se especificó la opción --visualize
                                    if args.contains(&"--visualize".to_string()) || args.contains(&"-v".to_string()) {
                                        println!("\n{}", "🖼️  Lanzando visualizador...".cyan().bold());
                                        if let Err(e) = visualizer::run(interpreter.env) {
                                            eprintln!("{} {}", "Error al lanzar visualizador:".red().bold(), e);
                                        }
                                    } else {
                                        println!("\n{}", "💡 Tip: Usa --visualize o -v para ver la topología gráficamente".yellow());
                                    }
                                }
                                Err(runtime_error) => {
                                    println!("\n{} {}", "❌ Error de ejecución:".red().bold(), runtime_error);
                                    process::exit(1);
                                }
                            }
                        }
                        Err(semantic_errors) => {
                            semantic::report_semantic_errors(&semantic_errors, &source, filename);
                            process::exit(1);
                        }
                    }
                }
                Err(parse_errors) => {
                    parser::report_parse_errors(&parse_errors, &source, filename);
                    process::exit(1);
                }
            }
        }
        Err(errors) => {
            use crate::error::{Diagnostic, report_errors};

            let diagnostics: Vec<Diagnostic> = errors.iter().map(|err| {
                Diagnostic::lexical_error(
                    err.line,
                    err.column,
                    err.length,
                    err.message.clone()
                )
            }).collect();

            report_errors(&diagnostics, &source, filename);
            process::exit(1);
        }
    }
}

fn print_symbol_table(table: &semantic::SymbolTable) {
    use colored::*;
    use std::io::{self, Write};

    println!("\n{}", "═".repeat(80));
    println!("{}", "TABLA DE SÍMBOLOS".cyan().bold());
    println!("{}", "═".repeat(80));

    if !table.maquinas.is_empty() {
        println!("\n{} Máquinas:", "📦".green());
        for (nombre, sym) in &table.maquinas {
            let estado = if sym.presente { "colocada".green() } else { "no colocada".yellow() };
            println!("  • {} - {}", nombre.bold(), estado);
        }
    }

    if !table.concentradores.is_empty() {
        println!("\n{} Concentradores:", "🔌".green());
        for (nombre, sym) in &table.concentradores {
            let estado = if sym.presente { "colocado".green() } else { "no colocado".yellow() };
            let coax = if sym.tiene_coaxial { "+ coaxial" } else { "" };
            println!("  • {} - {} puertos {} - {} disponibles - {}",
                     nombre.bold(),
                     sym.puertos,
                     coax,
                     sym.disponibles,
                     estado);
        }
    }

    if !table.coaxiales.is_empty() {
        println!("\n{} Cables Coaxiales:", "📡".green());
        for (nombre, sym) in &table.coaxiales {
            let estado = if sym.presente { "colocado".green() } else { "no colocado".yellow() };
            let completo = if sym.completo { "completo".red() } else { "disponible".green() };
            println!("  • {} - {}m - {} máquinas - {} - {}",
                     nombre.bold(),
                     sym.longitud,
                     sym.num_maquinas,
                     completo,
                     estado);
        }
    }

    if !table.modulos.is_empty() {
        println!("\n{} Módulos:", "📦".green());
        for nombre in table.modulos.keys() {
            println!("  • {}", nombre.bold());
        }
    }

    println!("\n{}", "═".repeat(80));
    let _ = io::stdout().flush();
}

fn print_network_state(env: &interpreter::Environment) {
    use std::io::{self, Write};

    println!("\n{}", "═".repeat(80));
    println!("{}", "ESTADO DE LA RED DESPUÉS DE LA EJECUCIÓN".cyan().bold());
    println!("{}", "═".repeat(80));

    // Mostrar máquinas
    if !env.maquinas.is_empty() {
        println!("\n{} Máquinas:", "💻".green());
        for (nombre, maq) in &env.maquinas {
            let estado = if maq.colocada {
                format!("colocada en ({}, {})", maq.x, maq.y).green()
            } else {
                "no colocada".yellow()
            };

            let conexion = match &maq.conectada_a {
                Some(ConexionMaquina::Puerto { concentrador, puerto }) => {
                    format!(" → conectada al puerto {} de '{}'", puerto, concentrador).cyan().to_string()
                }
                Some(ConexionMaquina::Coaxial { coaxial, posicion }) => {
                    format!(" → conectada al coaxial '{}' en posición {}m", coaxial, posicion).cyan().to_string()
                }
                None => String::new()
            };

            println!("  • {} - {}{}",
                     nombre.bold(),
                     estado,
                     conexion);
        }
    }

    // Mostrar concentradores
    if !env.concentradores.is_empty() {
        println!("\n{} Concentradores:", "🔌".green());
        for (nombre, conc) in &env.concentradores {
            let estado = if conc.colocado {
                format!("colocado en ({}, {})", conc.x, conc.y).green()
            } else {
                "no colocado".yellow()
            };

            let puertos_usados = conc.puertos - conc.disponibles;
            let coax_info = if let Some(ref coax) = conc.coaxial_asignado {
                format!(" [coaxial: {}]", coax).cyan().to_string()
            } else {
                String::new()
            };

            println!("  • {} - {} - {}/{} puertos usados{}",
                     nombre.bold(),
                     estado,
                     puertos_usados,
                     conc.puertos,
                     coax_info);
        }
    }

    // Mostrar coaxiales
    if !env.coaxiales.is_empty() {
        println!("\n{} Cables Coaxiales:", "📡".green());
        for (nombre, coax) in &env.coaxiales {
            let estado = if coax.colocado {
                format!("colocado en ({}, {}) - dirección: {}", coax.x, coax.y, coax.direccion).green()
            } else {
                "no colocado".yellow()
            };

            let num_maq = coax.maquinas.len();
            let completo = if coax.completo { " [COMPLETO]".red().to_string() } else { String::new() };

            println!("  • {} - {}m - {} - {} máquinas{}",
                     nombre.bold(),
                     coax.longitud,
                     estado,
                     num_maq,
                     completo);

            // Mostrar máquinas conectadas
            if !coax.maquinas.is_empty() {
                for (maq_nombre, pos) in &coax.maquinas {
                    println!("    ╰→ {} en posición {}m", maq_nombre, pos);
                }
            }
        }
    }

    println!("\n{}", "═".repeat(80));
    let _ = io::stdout().flush();
}

fn print_token_statistics(tokens: &[lexer::TokenInfo]) {
    use std::collections::HashMap;
    
    let mut token_counts: HashMap<String, usize> = HashMap::new();
    
    for token_info in tokens {
        let token_type = format!("{:?}", token_info.token)
            .split('(')
            .next()
            .unwrap_or("Unknown")
            .to_string();
        *token_counts.entry(token_type).or_insert(0) += 1;
    }
    
    println!("{}", "Estadísticas de Tokens:".cyan());
    
    let mut sorted_counts: Vec<_> = token_counts.iter().collect();
    sorted_counts.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    
    // Mostrar top 10
    for (i, (token_type, count)) in sorted_counts.iter().take(10).enumerate() {
        if i == 0 {
            println!("  {} {} ({})", "→".green(), token_type, count);
        } else {
            println!("  {} {} ({})", " ", token_type, count);
        }
    }
    
    if sorted_counts.len() > 10 {
        println!("  ... y {} tipos más", sorted_counts.len() - 10);
    }
}
