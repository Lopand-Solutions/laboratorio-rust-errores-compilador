//! Ejercicio 1: el equipo que se movió.
//! `resumen` debe devolver, por ejemplo, "2 jugadores: Kira, Ale".

pub fn describir_equipo(equipo: Vec<String>) -> String {
    equipo.join(", ")
}

pub fn resumen(equipo: Vec<String>) -> String {
    let lista = describir_equipo(equipo);
    format!("{} jugadores: {lista}", equipo.len())
}

// NO CAMBIES LAS PRUEBAS: el comprobador revisa que sigan igual.
#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn resume_al_equipo() {
        let equipo = vec![String::from("Kira"), String::from("Ale")];
        assert_eq!(resumen(equipo), "2 jugadores: Kira, Ale");
    }
}
