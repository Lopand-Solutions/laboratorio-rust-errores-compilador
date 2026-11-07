//! Ejercicio 3: registrar un puntaje y devolver el récord.
//! `registrar` agrega `nuevo` a la lista y devuelve el puntaje más alto, contando el nuevo.

pub fn registrar(puntajes: &mut Vec<u32>, nuevo: u32) -> u32 {
    // Con * copiamos el u32: record ya no es un préstamo de puntajes.
    let record = *puntajes.iter().max().unwrap_or(&0);
    puntajes.push(nuevo);
    record.max(nuevo)
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn agrega_y_devuelve_el_record() {
        let mut puntajes = vec![1200, 800];
        assert_eq!(registrar(&mut puntajes, 450), 1200);
        assert_eq!(registrar(&mut puntajes, 2100), 2100);
        assert_eq!(puntajes, [1200, 800, 450, 2100]);
        assert_eq!(registrar(&mut Vec::new(), 30), 30);
    }
}
