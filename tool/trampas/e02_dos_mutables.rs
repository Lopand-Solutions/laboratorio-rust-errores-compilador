// qué: copiar en lugar de intercambiar
//! Ejercicio 2: intercambiar a los dos primeros de la tabla.
//! `subir_al_segundo` deja al segundo lugar en el primero y al primero en el segundo.

pub fn subir_al_segundo(tabla: &mut [String]) {
    // Trampa: compila, pero copia en lugar de intercambiar.
    tabla[0] = tabla[1].clone();
    tabla[1] = tabla[0].clone();
}

