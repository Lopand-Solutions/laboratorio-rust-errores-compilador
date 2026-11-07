//! Ejercicio 2: intercambiar a los dos primeros de la tabla.
//! `subir_al_segundo` deja al segundo lugar en el primero y al primero en el segundo.

pub fn subir_al_segundo(tabla: &mut [String]) {
    // swap toma un solo préstamo mutable de la tabla y hace el cambio por dentro.
    tabla.swap(0, 1);
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn intercambia_los_dos_primeros() {
        let mut tabla = vec![
            String::from("Kira"),
            String::from("Ale"),
            String::from("Tomás"),
        ];
        subir_al_segundo(&mut tabla);
        assert_eq!(tabla, ["Ale", "Kira", "Tomás"]);
    }
}
