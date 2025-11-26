use macroquad::prelude::*;
use std::collections::HashMap;
use crate::interpreter;

// ============================================================================
// ESTRUCTURAS DE DATOS PARA VISUALIZACIÓN
// ============================================================================

#[derive(Debug, Clone)]
pub struct Maquina {
    pub nombre: String,
    pub colocada: bool,
    pub posicion: Option<(i32, i32)>,
    pub conexion: Option<ConexionMaquina>,
}

#[derive(Debug, Clone)]
pub enum ConexionMaquina {
    Puerto { concentrador: String, puerto: usize },
    Coaxial { coaxial: String, posicion: usize },
}

#[derive(Debug, Clone)]
pub struct Concentrador {
    pub nombre: String,
    pub puertos: usize,
    pub colocado: bool,
    pub posicion: Option<(i32, i32)>,
    pub puertos_usados: usize,
    pub tiene_coaxial: bool,
}

#[derive(Debug, Clone)]
pub struct Coaxial {
    pub nombre: String,
    pub longitud: usize,
    pub colocado: bool,
    pub posicion: Option<(i32, i32)>,
    pub direccion: Option<Direccion>,
    pub maquinas: Vec<(String, usize)>,
}

#[derive(Debug, Clone, Copy)]
pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub maquinas: HashMap<String, Maquina>,
    pub concentradores: HashMap<String, Concentrador>,
    pub coaxiales: HashMap<String, Coaxial>,
}

// ============================================================================
// CONVERSIÓN DESDE EL INTÉRPRETE
// ============================================================================

impl From<interpreter::Environment> for Environment {
    fn from(interp_env: interpreter::Environment) -> Self {
        let mut env = Environment {
            maquinas: HashMap::new(),
            concentradores: HashMap::new(),
            coaxiales: HashMap::new(),
        };

        // Convertir máquinas
        for (nombre, maq) in interp_env.maquinas {
            let conexion = match maq.conectada_a {
                Some(interpreter::ConexionMaquina::Puerto { concentrador, puerto }) => {
                    Some(ConexionMaquina::Puerto {
                        concentrador,
                        puerto: puerto as usize,
                    })
                }
                Some(interpreter::ConexionMaquina::Coaxial { coaxial, posicion }) => {
                    Some(ConexionMaquina::Coaxial {
                        coaxial,
                        posicion: posicion as usize,
                    })
                }
                None => None,
            };

            env.maquinas.insert(nombre.clone(), Maquina {
                nombre,
                colocada: maq.colocada,
                posicion: if maq.colocada {
                    Some((maq.x, maq.y))
                } else {
                    None
                },
                conexion,
            });
        }

        // Convertir concentradores
        for (nombre, conc) in interp_env.concentradores {
            let puertos_usados = (conc.puertos - conc.disponibles) as usize;

            env.concentradores.insert(nombre.clone(), Concentrador {
                nombre,
                puertos: conc.puertos as usize,
                colocado: conc.colocado,
                posicion: if conc.colocado {
                    Some((conc.x, conc.y))
                } else {
                    None
                },
                puertos_usados,
                tiene_coaxial: conc.tiene_coaxial,
            });
        }

        // Convertir coaxiales
        for (nombre, coax) in interp_env.coaxiales {
            let direccion = if coax.colocado {
                Some(match coax.direccion.as_str() {
                    "Arriba" => Direccion::Arriba,
                    "Abajo" => Direccion::Abajo,
                    "Izquierda" => Direccion::Izquierda,
                    "Derecha" => Direccion::Derecha,
                    _ => Direccion::Derecha,
                })
            } else {
                None
            };

            let maquinas: Vec<(String, usize)> = coax.maquinas
                .iter()
                .map(|(n, p)| (n.clone(), *p as usize))
                .collect();

            env.coaxiales.insert(nombre.clone(), Coaxial {
                nombre,
                longitud: coax.longitud as usize,
                colocado: coax.colocado,
                posicion: if coax.colocado {
                    Some((coax.x, coax.y))
                } else {
                    None
                },
                direccion,
                maquinas,
            });
        }

        env
    }
}

// ============================================================================
// CONFIGURACIÓN VISUAL
// ============================================================================

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

// Tamaños
const MACHINE_RADIUS: f32 = 20.0;
const HUB_SIZE: f32 = 50.0;
const CABLE_WIDTH: f32 = 8.0;
const CONNECTION_WIDTH: f32 = 4.0;

// Colores
const COLOR_MACHINE: Color = Color::new(0.2, 0.6, 1.0, 1.0);
const COLOR_HUB: Color = Color::new(1.0, 0.7, 0.2, 1.0);
const COLOR_COAXIAL: Color = Color::new(0.3, 0.3, 0.3, 1.0);
const COLOR_UTP: Color = Color::new(0.4, 0.8, 0.4, 1.0);
const COLOR_BACKGROUND: Color = Color::new(0.95, 0.95, 0.95, 1.0);
const COLOR_TEXT: Color = BLACK;
const COLOR_BORDER: Color = Color::new(0.2, 0.2, 0.2, 1.0);

// Offset para centrar el dibujo
const OFFSET_X: f32 = 150.0;
const OFFSET_Y: f32 = 150.0;

// ============================================================================
// ESTADO DE LA APLICACIÓN
// ============================================================================

struct AppState {
    env: Environment,
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
}

impl AppState {
    fn new(env: Environment) -> Self {
        Self {
            env,
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
        }
    }
}

// ============================================================================
// FUNCIONES DE DIBUJO
// ============================================================================

fn draw_environment(state: &AppState) {
    // Aplicar transformación de cámara
    let offset_x = OFFSET_X + state.camera_x;
    let offset_y = OFFSET_Y + state.camera_y;
    let zoom = state.zoom;

    println!("\n🎨 ═══════ RENDERIZANDO CON MACROQUAD ═══════");
    
    // 1. Dibujar cables coaxiales (fondo)
    println!("🎨 [1] Dibujando cables coaxiales...");
    let mut cables_count = 0;
    for (nombre, coaxial) in &state.env.coaxiales {
        if coaxial.colocado {
            if let Some((x, y)) = coaxial.posicion {
                if let Some(dir) = coaxial.direccion {
                    let start_x = (x as f32 + offset_x) * zoom;
                    let start_y = (y as f32 + offset_y) * zoom;
                    
                    let (end_x, end_y) = match dir {
                        Direccion::Derecha => (start_x + (coaxial.longitud as f32 * zoom), start_y),
                        Direccion::Izquierda => (start_x - (coaxial.longitud as f32 * zoom), start_y),
                        Direccion::Abajo => (start_x, start_y + (coaxial.longitud as f32 * zoom)),
                        Direccion::Arriba => (start_x, start_y - (coaxial.longitud as f32 * zoom)),
                    };
                    
                    // Dibujar línea GRUESA
                    draw_line(start_x, start_y, end_x, end_y, CABLE_WIDTH * zoom, COLOR_COAXIAL);
                    
                    // Círculos en los extremos para mejor visibilidad
                    draw_circle(start_x, start_y, CABLE_WIDTH * 0.6 * zoom, COLOR_COAXIAL);
                    draw_circle(end_x, end_y, CABLE_WIDTH * 0.6 * zoom, COLOR_COAXIAL);
                    
                    // Etiqueta del cable
                    let label = format!("{} ({}m)", nombre, coaxial.longitud);
                    draw_text(&label, start_x, start_y - 15.0 * zoom, 16.0 * zoom, COLOR_TEXT);
                    
                    println!("   ✓ Cable '{}' desde ({},{}) hasta ({},{})", nombre, start_x, start_y, end_x, end_y);
                    cables_count += 1;
                }
            }
        }
    }
    println!("   Total cables dibujados: {}", cables_count);

    // 2. Dibujar conexiones UTP
    println!("🎨 [2] Dibujando conexiones UTP...");
    let mut utp_count = 0;
    for (nombre, maquina) in &state.env.maquinas {
        if maquina.colocada {
            if let Some(ConexionMaquina::Puerto { concentrador, puerto }) = &maquina.conexion {
                if let (Some((mx, my)), Some(hub)) = (maquina.posicion, state.env.concentradores.get(concentrador)) {
                    if let Some((hx, hy)) = hub.posicion {
                        let mx_screen = (mx as f32 + offset_x) * zoom;
                        let my_screen = (my as f32 + offset_y) * zoom;
                        let hx_screen = (hx as f32 + offset_x) * zoom;
                        let hy_screen = (hy as f32 + offset_y) * zoom;
                        
                        // Línea UTP
                        draw_line(mx_screen, my_screen, hx_screen, hy_screen, CONNECTION_WIDTH * zoom, COLOR_UTP);
                        
                        // Etiqueta del puerto en el medio
                        let mid_x = (mx_screen + hx_screen) / 2.0;
                        let mid_y = (my_screen + hy_screen) / 2.0;
                        let port_label = format!("P{}", puerto);
                        draw_text(&port_label, mid_x, mid_y - 5.0 * zoom, 14.0 * zoom, COLOR_TEXT);
                        
                        println!("   ✓ Conexión {} → {} [Puerto {}]", nombre, concentrador, puerto);
                        utp_count += 1;
                    }
                }
            }
        }
    }
    println!("   Total conexiones UTP dibujadas: {}", utp_count);

    // 3. Dibujar concentradores
    println!("🎨 [3] Dibujando concentradores...");
    let mut hubs_count = 0;
    for (nombre, hub) in &state.env.concentradores {
        if hub.colocado {
            if let Some((x, y)) = hub.posicion {
                let x_screen = (x as f32 + offset_x) * zoom;
                let y_screen = (y as f32 + offset_y) * zoom;
                let size = HUB_SIZE * zoom;
                
                // Borde
                draw_rectangle(x_screen - size/2.0 - 2.0, y_screen - size/2.0 - 2.0, 
                              size + 4.0, size + 4.0, COLOR_BORDER);
                
                // Cuadrado del hub
                draw_rectangle(x_screen - size/2.0, y_screen - size/2.0, size, size, COLOR_HUB);
                
                // Indicador de salida coaxial
                if hub.tiene_coaxial {
                    draw_circle(x_screen + size/2.0 - 8.0 * zoom, y_screen - size/2.0 + 8.0 * zoom, 
                               4.0 * zoom, RED);
                }
                
                // Etiqueta
                let label = format!("{}\n({}/{})", nombre, hub.puertos_usados, hub.puertos);
                draw_text(&label, x_screen - 20.0 * zoom, y_screen + size/2.0 + 20.0 * zoom, 
                         14.0 * zoom, COLOR_TEXT);
                
                println!("   ✓ Hub '{}' en ({},{})", nombre, x_screen, y_screen);
                hubs_count += 1;
            }
        }
    }
    println!("   Total hubs dibujados: {}", hubs_count);

    // 4. Dibujar máquinas en cables coaxiales
    println!("🎨 [4] Dibujando máquinas en cables coaxiales...");
    let mut machines_cable_count = 0;
    for (nombre, maquina) in &state.env.maquinas {
        if maquina.colocada {
            if let Some(ConexionMaquina::Coaxial { coaxial, posicion }) = &maquina.conexion {
                if let Some(cable) = state.env.coaxiales.get(coaxial) {
                    if let (Some((cx, cy)), Some(dir)) = (cable.posicion, cable.direccion) {
                        let (mx, my) = match dir {
                            Direccion::Derecha => (cx + *posicion as i32, cy),
                            Direccion::Izquierda => (cx - *posicion as i32, cy),
                            Direccion::Abajo => (cx, cy + *posicion as i32),
                            Direccion::Arriba => (cx, cy - *posicion as i32),
                        };
                        
                        let mx_screen = (mx as f32 + offset_x) * zoom;
                        let my_screen = (my as f32 + offset_y) * zoom;
                        let radius = MACHINE_RADIUS * zoom;
                        
                        // Borde
                        draw_circle(mx_screen, my_screen, radius + 2.0, COLOR_BORDER);
                        
                        // Círculo de la máquina
                        draw_circle(mx_screen, my_screen, radius, COLOR_MACHINE);
                        
                        // Etiquetas (nombre arriba, posición abajo)
                        draw_text(nombre, mx_screen - 15.0 * zoom, my_screen - radius - 5.0 * zoom, 
                                 14.0 * zoom, COLOR_TEXT);
                        draw_text(&format!("{}m", posicion), mx_screen - 10.0 * zoom, 
                                 my_screen + radius + 18.0 * zoom, 12.0 * zoom, COLOR_TEXT);
                        
                        println!("   ✓ Máquina '{}' en cable '{}' posición {}m", nombre, coaxial, posicion);
                        machines_cable_count += 1;
                    }
                }
            }
        }
    }
    println!("   Total máquinas en cable dibujadas: {}", machines_cable_count);

    // 5. Dibujar máquinas normales
    println!("🎨 [5] Dibujando máquinas normales...");
    let mut machines_count = 0;
    for (nombre, maquina) in &state.env.maquinas {
        if maquina.colocada {
            if matches!(maquina.conexion, Some(ConexionMaquina::Puerto { .. })) {
                if let Some((x, y)) = maquina.posicion {
                    let x_screen = (x as f32 + offset_x) * zoom;
                    let y_screen = (y as f32 + offset_y) * zoom;
                    let radius = MACHINE_RADIUS * zoom;
                    
                    // Borde
                    draw_circle(x_screen, y_screen, radius + 2.0, COLOR_BORDER);
                    
                    // Círculo de la máquina
                    draw_circle(x_screen, y_screen, radius, COLOR_MACHINE);
                    
                    // Etiqueta
                    draw_text(nombre, x_screen - 10.0 * zoom, y_screen + radius + 18.0 * zoom, 
                             14.0 * zoom, COLOR_TEXT);
                    
                    println!("   ✓ Máquina '{}' en ({},{})", nombre, x_screen, y_screen);
                    machines_count += 1;
                }
            }
        }
    }
    println!("   Total máquinas normales dibujadas: {}", machines_count);
    
    println!("🎨 ═══════ RENDERIZADO COMPLETO ═══════\n");
}

fn draw_ui(state: &AppState) {
    // Fondo para el header
    draw_rectangle(0.0, 0.0, WINDOW_WIDTH, 80.0, Color::new(0.2, 0.2, 0.25, 0.95));
    
    // Título
    draw_text("🌐 Topología de Red Ethernet", 20.0, 30.0, 28.0, WHITE);
    
    // Contadores
    let machines_count = state.env.maquinas.values().filter(|m| m.colocada).count();
    let hubs_count = state.env.concentradores.values().filter(|h| h.colocado).count();
    let cables_count = state.env.coaxiales.values().filter(|c| c.colocado).count();
    
    let mut utp_count = 0;
    for maquina in state.env.maquinas.values() {
        if let Some(ConexionMaquina::Puerto { .. }) = maquina.conexion {
            utp_count += 1;
        }
    }
    
    let stats = format!("📦 Máquinas: {}  🔌 Hubs: {}  📡 Cables: {}  🔗 UTP: {}", 
                        machines_count, hubs_count, cables_count, utp_count);
    draw_text(&stats, 20.0, 60.0, 18.0, Color::new(0.8, 0.8, 0.8, 1.0));
    
    // Leyenda en la esquina inferior derecha
    let legend_x = WINDOW_WIDTH - 250.0;
    let legend_y = WINDOW_HEIGHT - 180.0;
    
    // Fondo de la leyenda
    draw_rectangle(legend_x - 10.0, legend_y - 10.0, 240.0, 170.0, 
                   Color::new(1.0, 1.0, 1.0, 0.9));
    draw_rectangle_lines(legend_x - 10.0, legend_y - 10.0, 240.0, 170.0, 2.0, COLOR_BORDER);
    
    // Título de leyenda
    draw_text("Leyenda", legend_x, legend_y + 5.0, 20.0, COLOR_TEXT);
    
    // Elementos de la leyenda
    let mut y_offset = legend_y + 30.0;
    
    // Máquinas
    draw_circle(legend_x + 10.0, y_offset, 8.0, COLOR_MACHINE);
    draw_text("Máquinas", legend_x + 30.0, y_offset + 5.0, 16.0, COLOR_TEXT);
    y_offset += 25.0;
    
    // Hubs
    draw_rectangle(legend_x + 2.0, y_offset - 8.0, 16.0, 16.0, COLOR_HUB);
    draw_text("Concentradores", legend_x + 30.0, y_offset + 5.0, 16.0, COLOR_TEXT);
    y_offset += 25.0;
    
    // Cables coaxiales
    draw_line(legend_x, y_offset, legend_x + 20.0, y_offset, 6.0, COLOR_COAXIAL);
    draw_text("Cable Coaxial", legend_x + 30.0, y_offset + 5.0, 16.0, COLOR_TEXT);
    y_offset += 25.0;
    
    // Conexiones UTP
    draw_line(legend_x, y_offset, legend_x + 20.0, y_offset, 3.0, COLOR_UTP);
    draw_text("Conexión UTP", legend_x + 30.0, y_offset + 5.0, 16.0, COLOR_TEXT);
    y_offset += 25.0;
    
    // Salida coaxial
    draw_circle(legend_x + 10.0, y_offset, 6.0, RED);
    draw_text("Salida Coaxial", legend_x + 30.0, y_offset + 5.0, 16.0, COLOR_TEXT);
    
    // Controles
    let controls_y = WINDOW_HEIGHT - 25.0;
    draw_text("Controles: Rueda = Zoom | Flechas = Pan | ESC = Salir", 
              20.0, controls_y, 16.0, Color::new(0.4, 0.4, 0.4, 1.0));
}

// ============================================================================
// FUNCIÓN PRINCIPAL
// ============================================================================

fn window_conf() -> Conf {
    Conf {
        window_title: "Network Interpreter - Visualizador de Topología".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: true,
        ..Default::default()
    }
}

// ============================================================================
// FUNCIÓN PÚBLICA PARA INTEGRAR CON EL INTÉRPRETE
// ============================================================================

/// Función pública que se llama desde main.rs para visualizar la topología
/// Recibe el Environment del intérprete y lo convierte automáticamente
///
/// NOTA: Macroquad requiere un entorno gráfico (DISPLAY en Linux)
/// Si no tienes entorno gráfico, esta función mostrará un error.
pub fn run(interp_env: interpreter::Environment) -> Result<(), String> {
    println!("\n⚠️  VISUALIZACIÓN CON MACROQUAD");
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("La visualización gráfica con Macroquad requiere:");
    println!("  • Un entorno gráfico (DISPLAY en Linux)");
    println!("  • No funciona por SSH sin X11 forwarding");
    println!();
    println!("Para usar el visualizador:");
    println!("  1. Ejecuta desde un terminal con GUI");
    println!("  2. O usa X11 forwarding: ssh -X usuario@servidor");
    println!();
    println!("Topología de red guardada en memoria. Puedes ver el");
    println!("estado de la red en el output anterior.");
    println!();
    println!("═══════════════════════════════════════════════════════\n");

    // Convertir para mostrar un resumen text-based
    let env: Environment = interp_env.into();

    // Mostrar resumen en texto
    println!("📊 RESUMEN DE LA TOPOLOGÍA:");
    println!();

    let machines_placed = env.maquinas.values().filter(|m| m.colocada).count();
    let hubs_placed = env.concentradores.values().filter(|h| h.colocado).count();
    let cables_placed = env.coaxiales.values().filter(|c| c.colocado).count();

    println!("  💻 Máquinas colocadas: {}", machines_placed);
    println!("  🔌 Concentradores colocados: {}", hubs_placed);
    println!("  📡 Cables coaxiales colocados: {}", cables_placed);
    println!();

    if machines_placed > 0 {
        println!("  Máquinas:");
        for (nombre, maq) in env.maquinas.iter().filter(|(_, m)| m.colocada) {
            if let Some((x, y)) = maq.posicion {
                println!("    • {} en ({}, {})", nombre, x, y);
            }
        }
        println!();
    }

    println!("💡 Para visualización gráfica completa, ejecuta desde un entorno");
    println!("   con GUI o configura X11 forwarding.\n");

    Ok(())
}

