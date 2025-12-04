// src/config/loader.rs
// Cargador de archivos de configuración con caché

use crate::lexer_new::Automaton;
use once_cell::sync::Lazy;

/// Autómata cargado desde archivo (singleton lazy)
pub static AUTOMATON: Lazy<Automaton> = Lazy::new(|| {
    Automaton::from_file("config/automaton.aut")
        .expect("Error fatal: no se pudo cargar el autómata desde config/automaton.aut")
});

/// Carga el autómata (devuelve referencia estática)
pub fn load_automaton() -> &'static Automaton {
    &AUTOMATON
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_automaton() {
        let automaton = load_automaton();
        // Verificar que se cargó correctamente
        assert!(automaton.is_final(automaton.initial_state()).is_none());
    }
    
    #[test]
    fn test_automaton_singleton() {
        let a1 = load_automaton();
        let a2 = load_automaton();
        
        // Ambas referencias apuntan al mismo objeto
        assert!(std::ptr::eq(a1, a2));
    }
}
