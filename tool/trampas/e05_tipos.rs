// qué: la sugerencia del compilador, .into(), que divide enteros
//! Ejercicio 5: el porcentaje de vida para la barra del HUD.
//! `porcentaje_vida(30, 120)` debe dar 25.0.

pub fn porcentaje_vida(vida: u32, maxima: u32) -> f64 {
    // Trampa: la sugerencia del compilador. Compila, pero divide enteros.
    (vida / maxima * 100).into()
}

