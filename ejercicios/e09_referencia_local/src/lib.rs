//! Ejercicio 9: el título que sale sobre la cabeza del personaje.
//! `titulo("Kira", 3)` debe dar "Kira (nivel 3)".

// Devuelve un String: el texto nuevo se lo queda quien llama.
pub fn titulo(nombre: &str, nivel: u32) -> String {
    format!("{nombre} (nivel {nivel})")
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn arma_el_titulo() {
        assert_eq!(titulo("Kira", 3), "Kira (nivel 3)");
    }
}
