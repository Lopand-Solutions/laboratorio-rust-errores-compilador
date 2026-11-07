//! Ejercicio 6: el precio de una compra en la tienda del juego.
//! `precio_total` devuelve el precio por la cantidad, o None si el objeto no está en la tienda.

pub fn buscar(tienda: &[(&str, u32)], nombre: &str) -> Option<u32> {
    for (objeto, precio) in tienda {
        if *objeto == nombre {
            return Some(*precio);
        }
    }
    None
}

pub fn precio_total(tienda: &[(&str, u32)], nombre: &str, cantidad: u32) -> Option<u32> {
    // ? devuelve None en cuanto buscar no encuentra el objeto.
    let precio = buscar(tienda, nombre)?;
    Some(precio * cantidad)
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    const TIENDA: [(&str, u32); 2] = [("Poción", 15), ("Escudo de roble", 120)];

    #[test]
    fn multiplica_o_no_encuentra() {
        assert_eq!(precio_total(&TIENDA, "Poción", 3), Some(45));
        assert_eq!(precio_total(&TIENDA, "Dragón", 1), None);
    }
}
