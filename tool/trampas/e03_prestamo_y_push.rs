// qué: olvidar el puntaje nuevo al calcular el récord
//! Ejercicio 3: registrar un puntaje y devolver el récord.
//! `registrar` agrega `nuevo` a la lista y devuelve el puntaje más alto, contando el nuevo.

pub fn registrar(puntajes: &mut Vec<u32>, nuevo: u32) -> u32 {
    // Con * copiamos el u32: record ya no es un préstamo de puntajes.
    let record = *puntajes.iter().max().unwrap_or(&0);
    puntajes.push(nuevo);
    // Trampa: compila, pero olvida contar el puntaje nuevo.
    record
}

