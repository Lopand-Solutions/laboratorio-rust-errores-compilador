//! Ejercicio 4: subir de nivel.
//! `subir_nivel` suma 1 al nivel y devuelve la vida al máximo.

pub struct Personaje {
    pub nivel: u32,
    pub vida: u32,
    pub vida_maxima: u32,
}

pub fn subir_nivel(personaje: &Personaje) {
    personaje.nivel += 1;
    personaje.vida = personaje.vida_maxima;
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn sube_y_cura() {
        let mut kira = Personaje {
            nivel: 2,
            vida: 10,
            vida_maxima: 100,
        };
        subir_nivel(&mut kira);
        assert_eq!((kira.nivel, kira.vida), (3, 100));
    }
}
