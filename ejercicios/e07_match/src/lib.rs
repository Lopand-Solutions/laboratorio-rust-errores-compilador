//! Ejercicio 7: el tablero de tareas del estudio agregó un estado nuevo.
//! `etiqueta` devuelve el texto de cada estado; el de `Bloqueada` es "bloqueada".

pub enum Estado {
    Pendiente,
    EnCurso,
    Hecha,
    Bloqueada,
}

pub fn etiqueta(estado: &Estado) -> &'static str {
    match estado {
        Estado::Pendiente => "pendiente",
        Estado::EnCurso => "en curso",
        Estado::Hecha => "hecha",
    }
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn cada_estado_tiene_texto() {
        assert_eq!(etiqueta(&Estado::Pendiente), "pendiente");
        assert_eq!(etiqueta(&Estado::EnCurso), "en curso");
        assert_eq!(etiqueta(&Estado::Hecha), "hecha");
        assert_eq!(etiqueta(&Estado::Bloqueada), "bloqueada");
    }
}
