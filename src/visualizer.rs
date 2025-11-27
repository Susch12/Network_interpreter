use eframe::egui;
use std::collections::HashMap;
use crate::interpreter;

// ============================================================================
// ESTRUCTURAS DE DATOS
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
// CONVERSIÓN
// ============================================================================

impl From<interpreter::Environment> for Environment {
    fn from(interp_env: interpreter::Environment) -> Self {
        let mut env = Environment {
            maquinas: HashMap::new(),
            concentradores: HashMap::new(),
            coaxiales: HashMap::new(),
        };

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
                posicion: if maq.colocada { Some((maq.x, maq.y)) } else { None },
                conexion,
            });
        }

        for (nombre, conc) in interp_env.concentradores {
            let puertos_usados = (conc.puertos - conc.disponibles) as usize;
            env.concentradores.insert(nombre.clone(), Concentrador {
                nombre,
                puertos: conc.puertos as usize,
                colocado: conc.colocado,
                posicion: if conc.colocado { Some((conc.x, conc.y)) } else { None },
                puertos_usados,
                tiene_coaxial: conc.tiene_coaxial,
            });
        }

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
                posicion: if coax.colocado { Some((coax.x, coax.y)) } else { None },
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

const MACHINE_SIZE: f32 = 70.0;      // Tamaño de computadora desktop
const HUB_WIDTH: f32 = 120.0;        // Ancho del switch
const HUB_HEIGHT: f32 = 60.0;        // Alto del switch
const CABLE_WIDTH: f32 = 16.0;
const CONNECTION_WIDTH: f32 = 10.0;
const SPACING_MULTIPLIER: f32 = 20.0;
const OFFSET_X: f32 = 150.0;
const OFFSET_Y: f32 = 150.0;

// Paleta de colores - TEMA OSCURO
const COLOR_BG: egui::Color32 = egui::Color32::from_rgb(10, 10, 15);
const COLOR_GRID: egui::Color32 = egui::Color32::from_rgb(30, 30, 40);

// Colores para cables
const COLOR_CABLE: egui::Color32 = egui::Color32::from_rgb(148, 163, 184);
const COLOR_UTP_GLOW: egui::Color32 = egui::Color32::from_rgb(134, 239, 172);
const COLOR_UTP_CORE: egui::Color32 = egui::Color32::from_rgb(74, 222, 128);
const COLOR_UTP_OUTER: egui::Color32 = egui::Color32::from_rgb(250, 204, 21);

// ============================================================================
// APLICACIÓN
// ============================================================================

struct NetworkVisualizerApp {
    env: Environment,
    zoom: f32,
    pan_offset: egui::Vec2,
    animation_time: f32,
}

impl NetworkVisualizerApp {
    fn new(env: Environment) -> Self {
        Self {
            env,
            zoom: 0.6,
            pan_offset: egui::Vec2::ZERO,
            animation_time: 0.0,
        }
    }

    // ========================================================================
    // FUNCIONES DE DIBUJO DE OBJETOS
    // ========================================================================

    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        let spacing = 50.0 * self.zoom;
        let color = egui::Color32::from_rgba_premultiplied(80, 80, 100, 30);
        
        let mut x = rect.min.x;
        while x < rect.max.x {
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                egui::Stroke::new(1.0, color)
            );
            x += spacing;
        }
        
        let mut y = rect.min.y;
        while y < rect.max.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(1.0, color)
            );
            y += spacing;
        }
    }

    fn draw_glow(&self, painter: &egui::Painter, pos: egui::Pos2, radius: f32, color: egui::Color32) {
        for i in 0..7 {
            let alpha = 200 - (i * 28);
            let r = radius + (i as f32 * 6.0);
            painter.circle_filled(
                pos,
                r,
                egui::Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha.max(0) as u8)
            );
        }
    }

    fn draw_label_with_bg(&self, painter: &egui::Painter, pos: egui::Pos2, text: String, 
                          font_size: f32, text_color: egui::Color32, bg_color: egui::Color32) {
        let galley = painter.layout_no_wrap(
            text,
            egui::FontId::proportional(font_size),
            text_color,
        );
        
        let label_pos = egui::pos2(pos.x - galley.size().x / 2.0, pos.y);
        let label_rect = egui::Rect::from_min_size(label_pos, galley.size());
        
        let shadow_rect = label_rect.expand(8.0).translate(egui::vec2(2.0, 2.0));
        painter.rect_filled(shadow_rect, 5.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 30));
        
        painter.rect_filled(label_rect.expand(8.0), 5.0, bg_color);
        painter.rect_stroke(
            label_rect.expand(8.0),
            5.0,
            egui::Stroke::new(2.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 100))
        );
        
        painter.galley(label_pos, galley, text_color);
    }

    // ========================================================================
    // DIBUJAR COMPUTADORA DESKTOP REALISTA
    // ========================================================================
    
    fn draw_computer(&self, painter: &egui::Painter, pos: egui::Pos2, zoom: f32) {
        let size = MACHINE_SIZE * zoom;
        
        // Posiciones de los componentes
        let monitor_rect = egui::Rect::from_center_size(
            egui::pos2(pos.x, pos.y - size * 0.25),
            egui::vec2(size * 0.8, size * 0.5)
        );
        
        let case_rect = egui::Rect::from_center_size(
            egui::pos2(pos.x, pos.y + size * 0.25),
            egui::vec2(size * 0.5, size * 0.4)
        );
        
        // Glow azul alrededor de todo
        self.draw_glow(painter, pos, size * 0.6, egui::Color32::from_rgb(96, 165, 250));
        
        // ===== MONITOR =====
        
        // Sombra del monitor
        painter.rect_filled(
            monitor_rect.translate(egui::vec2(4.0, 4.0)),
            3.0,
            egui::Color32::from_rgba_premultiplied(0, 0, 0, 100)
        );
        
        // Marco del monitor (gris oscuro)
        painter.rect_filled(monitor_rect, 3.0, egui::Color32::from_rgb(50, 50, 55));
        
        // Pantalla (azul brillante - encendida)
        let screen_rect = monitor_rect.shrink(size * 0.04);
        painter.rect_filled(screen_rect, 2.0, egui::Color32::from_rgb(30, 60, 100));
        
        // Brillo de la pantalla (gradiente simulado)
        painter.rect_filled(
            egui::Rect::from_min_max(
                screen_rect.min,
                egui::pos2(screen_rect.max.x, screen_rect.center().y)
            ),
            2.0,
            egui::Color32::from_rgb(50, 100, 150)
        );
        
        // Reflejo en la pantalla
        painter.rect_filled(
            egui::Rect::from_min_max(
                screen_rect.min,
                egui::pos2(screen_rect.max.x, screen_rect.min.y + screen_rect.height() * 0.3)
            ),
            2.0,
            egui::Color32::from_rgba_premultiplied(200, 220, 255, 60)
        );
        
        // Base del monitor
        let stand_rect = egui::Rect::from_center_size(
            egui::pos2(monitor_rect.center().x, monitor_rect.max.y + size * 0.05),
            egui::vec2(size * 0.3, size * 0.08)
        );
        painter.rect_filled(stand_rect, 2.0, egui::Color32::from_rgb(40, 40, 45));
        
        // ===== CASE (TORRE) =====
        
        // Sombra del case
        painter.rect_filled(
            case_rect.translate(egui::vec2(3.0, 3.0)),
            2.0,
            egui::Color32::from_rgba_premultiplied(0, 0, 0, 100)
        );
        
        // Case principal (gris metálico)
        painter.rect_filled(case_rect, 2.0, egui::Color32::from_rgb(70, 75, 80));
        
        // Panel frontal más claro (efecto 3D)
        let front_panel = egui::Rect::from_min_max(
            case_rect.min,
            egui::pos2(case_rect.max.x, case_rect.max.y)
        );
        painter.rect_filled(front_panel, 2.0, egui::Color32::from_rgb(85, 90, 95));
        
        // Highlight superior (brillo metálico)
        painter.rect_filled(
            egui::Rect::from_min_max(
                case_rect.min,
                egui::pos2(case_rect.max.x, case_rect.min.y + case_rect.height() * 0.15)
            ),
            2.0,
            egui::Color32::from_rgba_premultiplied(255, 255, 255, 40)
        );
        
        // LED de encendido (verde pulsante)
        let pulse = (self.animation_time * 3.0).sin() * 0.3 + 0.7;
        let led_pos = egui::pos2(case_rect.min.x + size * 0.08, case_rect.min.y + size * 0.08);
        painter.circle_filled(
            led_pos,
            size * 0.025,
            egui::Color32::from_rgb((0.0 + 100.0 * pulse) as u8, (200.0 * pulse) as u8, 0)
        );
        
        // Drive bay (ranura de unidad)
        let drive_rect = egui::Rect::from_min_size(
            egui::pos2(case_rect.min.x + size * 0.05, case_rect.center().y),
            egui::vec2(case_rect.width() * 0.9, size * 0.06)
        );
        painter.rect_filled(drive_rect, 1.0, egui::Color32::from_rgb(40, 45, 50));
    }

    // ========================================================================
    // DIBUJAR HUB/SWITCH REALISTA
    // ========================================================================
    
    fn draw_switch(&self, painter: &egui::Painter, pos: egui::Pos2, zoom: f32, puertos: usize, puertos_usados: usize) {
        let width = HUB_WIDTH * zoom;
        let height = HUB_HEIGHT * zoom;
        
        let switch_rect = egui::Rect::from_center_size(
            pos,
            egui::vec2(width, height)
        );
        
        // Glow amarillo alrededor
        self.draw_glow(painter, pos, width * 0.7, egui::Color32::from_rgb(253, 224, 71));
        
        // ===== CUERPO DEL SWITCH =====
        
        // Sombra
        painter.rect_filled(
            switch_rect.translate(egui::vec2(5.0, 5.0)),
            4.0,
            egui::Color32::from_rgba_premultiplied(0, 0, 0, 120)
        );
        
        // Cuerpo metálico (gris claro)
        painter.rect_filled(switch_rect, 4.0, egui::Color32::from_rgb(160, 165, 170));
        
        // Panel frontal más oscuro
        let front_panel = egui::Rect::from_min_max(
            switch_rect.min,
            egui::pos2(switch_rect.max.x, switch_rect.max.y)
        );
        painter.rect_filled(front_panel, 4.0, egui::Color32::from_rgb(140, 145, 150));
        
        // Highlight superior (brillo metálico)
        painter.rect_filled(
            egui::Rect::from_min_max(
                switch_rect.min,
                egui::pos2(switch_rect.max.x, switch_rect.min.y + height * 0.2)
            ),
            4.0,
            egui::Color32::from_rgba_premultiplied(255, 255, 255, 80)
        );
        
        // Borde inferior oscuro (sombra interna)
        painter.rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(switch_rect.min.x, switch_rect.max.y - height * 0.15),
                switch_rect.max
            ),
            4.0,
            egui::Color32::from_rgba_premultiplied(0, 0, 0, 40)
        );
        
        // ===== PUERTOS RJ45 (parte inferior) =====
        
        let port_width = width * 0.08;
        let port_height = height * 0.25;
        let port_spacing = width * 0.1;
        let ports_y = switch_rect.max.y - height * 0.35;
        
        let start_x = switch_rect.center().x - (puertos as f32 * port_spacing / 2.0);
        
        for i in 0..puertos.min(8) {  // Máximo 8 puertos visibles
            let port_x = start_x + (i as f32 * port_spacing);
            let port_rect = egui::Rect::from_min_size(
                egui::pos2(port_x, ports_y),
                egui::vec2(port_width, port_height)
            );
            
            // Puerto (negro)
            painter.rect_filled(port_rect, 1.0, egui::Color32::from_rgb(20, 20, 25));
            
            // Borde metálico
            painter.rect_stroke(port_rect, 1.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 105)));
        }
        
        // ===== LEDs DE ESTADO (parte superior) =====
        
        let led_y = switch_rect.min.y + height * 0.35;
        let led_size = width * 0.025;
        
        for i in 0..puertos.min(8) {
            let led_x = start_x + (i as f32 * port_spacing) + port_width * 0.5;
            let led_pos = egui::pos2(led_x, led_y);
            
            if i < puertos_usados {
                // LED verde (puerto activo) - pulsante
                let pulse = (self.animation_time * 4.0 + i as f32 * 0.5).sin() * 0.3 + 0.7;
                painter.circle_filled(led_pos, led_size, egui::Color32::from_rgb(0, (200.0 * pulse) as u8, 0));
                
                // Glow del LED
                painter.circle_filled(
                    led_pos,
                    led_size * 2.0,
                    egui::Color32::from_rgba_premultiplied(0, 255, 0, (100.0 * pulse) as u8)
                );
            } else {
                // LED apagado (gris oscuro)
                painter.circle_filled(led_pos, led_size, egui::Color32::from_rgb(40, 40, 45));
            }
        }
        
        // ===== LOGO/MARCA (centro superior) =====
        
        let logo_rect = egui::Rect::from_center_size(
            egui::pos2(switch_rect.center().x, switch_rect.min.y + height * 0.15),
            egui::vec2(width * 0.3, height * 0.12)
        );
        painter.rect_filled(logo_rect, 2.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 60));
    }

    // ========================================================================
    // FUNCIÓN PRINCIPAL DE DIBUJO
    // ========================================================================
    
    fn draw_network(&self, ui: &mut egui::Ui, painter: &egui::Painter, rect: egui::Rect) {
        let offset = egui::vec2(OFFSET_X, OFFSET_Y) + self.pan_offset;
        let zoom = self.zoom;

        let transform = |x: f32, y: f32| -> egui::Pos2 {
            egui::pos2(
                rect.min.x + (x * SPACING_MULTIPLIER + offset.x) * zoom,
                rect.min.y + (y * SPACING_MULTIPLIER + offset.y) * zoom
            )
        };

        self.draw_grid(painter, rect);

        // ====================================================================
        // 1. CABLES COAXIALES
        // ====================================================================
        
        for (nombre, coaxial) in &self.env.coaxiales {
            if coaxial.colocado {
                if let Some((x, y)) = coaxial.posicion {
                    if let Some(dir) = coaxial.direccion {
                        let start = transform(x as f32, y as f32);
                        let end = match dir {
                            Direccion::Derecha => transform((x + coaxial.longitud as i32) as f32, y as f32),
                            Direccion::Izquierda => transform((x - coaxial.longitud as i32) as f32, y as f32),
                            Direccion::Abajo => transform(x as f32, (y + coaxial.longitud as i32) as f32),
                            Direccion::Arriba => transform(x as f32, (y - coaxial.longitud as i32) as f32),
                        };
                        
                        let w = CABLE_WIDTH * zoom;
                        
                        painter.line_segment(
                            [egui::pos2(start.x + 3.0, start.y + 3.0), egui::pos2(end.x + 3.0, end.y + 3.0)],
                            egui::Stroke::new(w + 10.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 50))
                        );
                        
                        painter.line_segment([start, end], egui::Stroke::new(w + 8.0, egui::Color32::BLACK));
                        painter.line_segment([start, end], egui::Stroke::new(w, COLOR_CABLE));
                        
                        painter.line_segment(
                            [egui::pos2(start.x, start.y - w * 0.3), egui::pos2(end.x, end.y - w * 0.3)],
                            egui::Stroke::new(w * 0.4, egui::Color32::from_rgba_premultiplied(255, 255, 255, 80))
                        );
                        
                        for pos in [start, end] {
                            painter.circle_filled(pos, w * 0.9, egui::Color32::from_rgb(120, 120, 120));
                            painter.circle_filled(pos, w * 0.7, egui::Color32::from_rgb(80, 80, 80));
                            painter.circle_filled(pos, w * 0.3, egui::Color32::from_rgb(180, 180, 180));
                        }
                        
                        let label = format!("{} • {}m", nombre, coaxial.longitud);
                        self.draw_label_with_bg(
                            painter,
                            egui::pos2(start.x, start.y - 60.0 * zoom),
                            label,
                            22.0 * zoom,
                            egui::Color32::WHITE,
                            egui::Color32::from_rgb(71, 85, 105)
                        );
                    }
                }
            }
        }

        // ====================================================================
        // 2. CONCENTRADORES (SWITCHES)
        // ====================================================================
        
        for (nombre, hub) in &self.env.concentradores {
            if hub.colocado {
                if let Some((x, y)) = hub.posicion {
                    let pos = transform(x as f32, y as f32);
                    
                    self.draw_switch(painter, pos, zoom, hub.puertos, hub.puertos_usados);
                    
                    let label = format!("{}\n{}/{} puertos", nombre, hub.puertos_usados, hub.puertos);
                    self.draw_label_with_bg(
                        painter,
                        egui::pos2(pos.x, pos.y + HUB_HEIGHT * zoom / 2.0 + 25.0 * zoom),
                        label,
                        22.0 * zoom,
                        egui::Color32::WHITE,
                        egui::Color32::from_rgb(140, 145, 150)
                    );
                }
            }
        }

        // ====================================================================
        // 3. COMPUTADORAS
        // ====================================================================
        
        for (nombre, maquina) in &self.env.maquinas {
            if maquina.colocada {
                let pos_opt = if let Some(ConexionMaquina::Coaxial { coaxial, posicion }) = &maquina.conexion {
                    if let Some(cable) = self.env.coaxiales.get(coaxial) {
                        if let (Some((cx, cy)), Some(dir)) = (cable.posicion, cable.direccion) {
                            let (mx, my) = match dir {
                                Direccion::Derecha => (cx + *posicion as i32, cy),
                                Direccion::Izquierda => (cx - *posicion as i32, cy),
                                Direccion::Abajo => (cx, cy + *posicion as i32),
                                Direccion::Arriba => (cx, cy - *posicion as i32),
                            };
                            Some((transform(mx as f32, my as f32), Some(*posicion)))
                        } else { None }
                    } else { None }
                } else {
                    maquina.posicion.map(|(x, y)| (transform(x as f32, y as f32), None))
                };

                if let Some((pos, cable_pos)) = pos_opt {
                    self.draw_computer(painter, pos, zoom);
                    
                    self.draw_label_with_bg(
                        painter,
                        egui::pos2(pos.x, pos.y - MACHINE_SIZE * zoom / 2.0 - 55.0 * zoom),
                        nombre.clone(),
                        24.0 * zoom,
                        egui::Color32::WHITE,
                        egui::Color32::from_rgb(59, 130, 246)
                    );
                    
                    if let Some(pos_m) = cable_pos {
                        self.draw_label_with_bg(
                            painter,
                            egui::pos2(pos.x, pos.y + MACHINE_SIZE * zoom / 2.0 + 25.0 * zoom),
                            format!("{}m", pos_m),
                            20.0 * zoom,
                            egui::Color32::from_rgb(71, 85, 105),
                            egui::Color32::from_rgb(254, 243, 199)
                        );
                    }
                }
            }
        }

        // ====================================================================
        // 4. CONEXIONES UTP ANIMADAS
        // ====================================================================
        
        for (_nombre, maquina) in &self.env.maquinas {
            if maquina.colocada {
                if let Some(ConexionMaquina::Puerto { concentrador, puerto }) = &maquina.conexion {
                    if let (Some((mx, my)), Some(hub)) = (maquina.posicion, self.env.concentradores.get(concentrador)) {
                        if let Some((hx, hy)) = hub.posicion {
                            let m_pos = transform(mx as f32, my as f32);
                            let h_pos = transform(hx as f32, hy as f32);
                            let w = CONNECTION_WIDTH * zoom;
                            
                            let pulse = (self.animation_time * 2.0).sin() * 0.3 + 0.7;
                            painter.line_segment([m_pos, h_pos], egui::Stroke::new(w + 24.0 * pulse, COLOR_UTP_GLOW));
                            painter.line_segment([m_pos, h_pos], egui::Stroke::new(w + 18.0, COLOR_UTP_OUTER));
                            painter.line_segment([m_pos, h_pos], egui::Stroke::new(w + 10.0, egui::Color32::BLACK));
                            painter.line_segment([m_pos, h_pos], egui::Stroke::new(w, COLOR_UTP_CORE));
                            
                            for pos in [m_pos, h_pos] {
                                painter.circle_filled(pos, w * 2.5, egui::Color32::from_rgb(220, 38, 38));
                                painter.circle_filled(pos, w * 2.0, egui::Color32::from_rgb(239, 68, 68));
                                painter.circle_filled(pos, w * 1.2, egui::Color32::from_rgb(254, 202, 202));
                            }
                            
                            let mid = egui::pos2((m_pos.x + h_pos.x) / 2.0, (m_pos.y + h_pos.y) / 2.0);
                            self.draw_label_with_bg(
                                painter,
                                mid,
                                format!("Puerto {}", puerto),
                                22.0 * zoom,
                                egui::Color32::WHITE,
                                COLOR_UTP_CORE
                            );
                        }
                    }
                }
            }
        }
    }
}

impl eframe::App for NetworkVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.animation_time += ctx.input(|i| i.stable_dt);
        ctx.request_repaint();
        
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
            
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(15, 15, 20))
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(
                            egui::RichText::new("🌐 Network Topology Visualizer")
                                .size(24.0)
                                .strong()
                                .color(egui::Color32::from_rgb(147, 197, 253))
                        );
                        ui.separator();
                        
                        let machines = self.env.maquinas.values().filter(|m| m.colocada).count();
                        let hubs = self.env.concentradores.values().filter(|h| h.colocado).count();
                        let cables = self.env.coaxiales.values().filter(|c| c.colocado).count();
                        let utp = self.env.maquinas.values()
                            .filter(|m| matches!(m.conexion, Some(ConexionMaquina::Puerto { .. })))
                            .count();
                        
                        ui.label(egui::RichText::new(format!("💻 {} PCs", machines)).size(16.0).color(egui::Color32::from_rgb(147, 197, 253)));
                        ui.separator();
                        ui.label(egui::RichText::new(format!("🔌 {} Switches", hubs)).size(16.0).color(egui::Color32::from_rgb(253, 224, 71)));
                        ui.separator();
                        ui.label(egui::RichText::new(format!("📡 {} Cables", cables)).size(16.0).color(egui::Color32::from_rgb(148, 163, 184)));
                        ui.separator();
                        ui.label(egui::RichText::new(format!("🔗 {} UTP", utp)).size(16.0).color(egui::Color32::from_rgb(134, 239, 172)));
                    });
                });
        });

        egui::SidePanel::right("side").min_width(300.0).show(ctx, |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(20, 20, 28))
                .inner_margin(16.0)
                .show(ui, |ui| {
                    ui.heading(egui::RichText::new("⚙️ Controles").size(22.0).color(egui::Color32::from_rgb(147, 197, 253)));
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.label(egui::RichText::new("🔍 Zoom").size(16.0).color(egui::Color32::WHITE));
                    ui.add(egui::Slider::new(&mut self.zoom, 0.2..=4.0).show_value(false));
                    ui.label(egui::RichText::new(format!("Zoom: {:.1}x", self.zoom)).color(egui::Color32::from_rgb(200, 200, 220)));
                    
                    if ui.button(egui::RichText::new("🔄 Reset Zoom").size(15.0)).clicked() {
                        self.zoom = 0.6;
                    }
                    
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.label(egui::RichText::new("📍 Vista").size(16.0).color(egui::Color32::WHITE));
                    if ui.button(egui::RichText::new("🔄 Reset Pan").size(15.0)).clicked() {
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    if ui.button(egui::RichText::new("🎯 Centrar Todo").size(15.0)).clicked() {
                        self.zoom = 0.6;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.heading(egui::RichText::new("📖 Leyenda").size(20.0).color(egui::Color32::from_rgb(253, 224, 71)));
                    ui.add_space(10.0);
                    
                    ui.label(egui::RichText::new("💻 Computadoras Desktop").size(15.0).color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new("🔌 Switches de Red").size(15.0).color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new("📡 Cable Coaxial").size(15.0).color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new("🔗 Conexión UTP").size(15.0).color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new("🟢 LEDs Verdes = Activo").size(15.0).color(egui::Color32::WHITE));
                    
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.label(egui::RichText::new("💡 Tips").size(16.0).color(egui::Color32::from_rgb(253, 224, 71)));
                    ui.label(egui::RichText::new("• Scroll para zoom").color(egui::Color32::from_rgb(200, 200, 220)));
                    ui.label(egui::RichText::new("• Arrastra para mover").color(egui::Color32::from_rgb(200, 200, 220)));
                    ui.label(egui::RichText::new("• LEDs parpadean = activos").color(egui::Color32::from_rgb(200, 200, 220)));
                    ui.label(egui::RichText::new("• ESC para salir").color(egui::Color32::from_rgb(200, 200, 220)));
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            painter.rect_filled(response.rect, 0.0, COLOR_BG);
            
            if response.dragged() {
                self.pan_offset += response.drag_delta() / self.zoom;
            }
            
            if response.hovered() {
                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll != 0.0 {
                    self.zoom *= 1.0 + scroll * 0.001;
                    self.zoom = self.zoom.clamp(0.2, 4.0);
                }
            }
            
            self.draw_network(ui, &painter, response.rect);
        });
    }
}

// ============================================================================
// FUNCIÓN PÚBLICA
// ============================================================================

pub fn run(interp_env: interpreter::Environment) -> Result<(), String> {
    let env: Environment = interp_env.into();
    
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_title("Network Topology Visualizer - Realistic Mode"),
        ..Default::default()
    };
    
    eframe::run_native(
        "network_visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(NetworkVisualizerApp::new(env)))),
    )
    .map_err(|e| format!("Error: {}", e))
}
