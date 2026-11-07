//! Ejercicio 5: el porcentaje de vida para la barra del HUD.
//! `porcentaje_vida(30, 120)` debe dar 25.0.

pub fn porcentaje_vida(vida: u32, maxima: u32) -> f64 {
    vida / maxima * 100
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn calcula_el_porcentaje() {
        assert_eq!(porcentaje_vida(30, 120), 25.0);
        assert_eq!(porcentaje_vida(120, 120), 100.0);
        assert_eq!(porcentaje_vida(0, 120), 0.0);
    }
}
