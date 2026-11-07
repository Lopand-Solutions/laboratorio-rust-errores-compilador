//! Ejercicio 8: el nombre más largo, para calcular el ancho de la tabla de puntajes.
//! `mas_largo` devuelve el más largo de los dos; si miden igual, el primero.

// 'a: lo que devuelve vive mientras vivan a y b.
pub fn mas_largo<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.chars().count() > a.chars().count() {
        b
    } else {
        a
    }
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn elige_el_mas_largo() {
        assert_eq!(mas_largo("Kira", "Tomás"), "Tomás");
        assert_eq!(mas_largo("Ale", "Leo"), "Ale");
    }
}
