# Laboratorio: arregla diez errores del compilador

Repositorio del laboratorio de [Lopand Academy](https://academy.lopandsolutions.com/tutoriales/laboratorio-rust-errores-compilador).
Diez ejercicios cortos en Rust que **no compilan**, con ejemplos de un juego: un inventario, una tabla de puntajes, un
tablero de tareas. Cada uno trae un error real del compilador (ownership, préstamos, mutabilidad, tipos, `match`,
lifetimes y traits) y una prueba que debe pasar cuando lo arregles.

## Ramas

- **`inicio`**: los diez ejercicios sin resolver. Desde aquí trabajas.
- **`main`**: una solución de cada ejercicio, por si te atoras. Hay más de una forma correcta de arreglar varios.

```bash
git clone -b inicio https://github.com/Lopand-Solutions/laboratorio-rust-errores-compilador.git errores-compilador
cd errores-compilador
```

## Córrelo

Cada ejercicio es un paquete aparte dentro de un workspace de Cargo, así uno que no compila no impide probar los demás:

```bash
cargo test -p e01_movido
```

Lee el error, arregla `ejercicios/e01_movido/src/lib.rs` y vuelve a correr la prueba. **No cambies las pruebas**: el
comprobador revisa que sigan como venían.

Si quieres empezar de nuevo un ejercicio, `tool/originales/` guarda cada uno como venía:

```bash
cp tool/originales/e01_movido.rs ejercicios/e01_movido/src/lib.rs
```

## Cómo saber si quedó

```bash
./tool/comprobar_laboratorio.sh
```

Dice cuáles de los diez ya compilan y pasan su prueba y cuáles faltan, revisa que las pruebas sigan intactas, corre
`cargo clippy -- -D warnings` y `cargo fmt --check`, y pone a prueba tus pruebas con código que compila pero está mal,
en una copia. No necesita red ni toca tus archivos.

## Requisitos

Rust 1.85 o superior (edición 2024), con `clippy` y `rustfmt`. Los mensajes de error de la lección salieron de
rustc 1.97.0; con otra versión el texto puede cambiar un poco, pero el código de error (`E0382`, `E0499`…) es el mismo.

Código libre para usar y adaptar (licencia MIT).
