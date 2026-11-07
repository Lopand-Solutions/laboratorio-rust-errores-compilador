//! Ejercicio 10: una lista separada por comas para el registro del servidor.
//! `en_lista` sirve para cualquier tipo que se pueda mostrar con "{}".

pub fn en_lista<T>(elementos: &[T]) -> String {
    let mut textos = Vec::new();
    for e in elementos {
        textos.push(format!("{e}"));
    }
    textos.join(", ")
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn sirve_para_numeros_y_textos() {
        assert_eq!(en_lista(&[1200, 800, 450]), "1200, 800, 450");
        assert_eq!(en_lista(&["Kira", "Ale"]), "Kira, Ale");
        assert_eq!(en_lista::<u32>(&[]), "");
    }
}
