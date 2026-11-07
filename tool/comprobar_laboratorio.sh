#!/usr/bin/env bash
# Comprueba el laboratorio «Arregla diez errores del compilador». Uso: ./tool/comprobar_laboratorio.sh
# No necesita red: dice qué ejercicios compilan y pasan su prueba, revisa que las pruebas sigan intactas, corre clippy
# y rustfmt, y pone a prueba tus pruebas con código que compila pero está mal, en una copia. No toca tus archivos.
set -uo pipefail
cd "$(dirname "$0")/.."
RAIZ=$(pwd)
# Las copias compilan en una carpeta aparte de target/ para no ensuciar tu compilación.
DESTINO="$RAIZ/target/comprobador"
bien=0
mal=0
resueltos=()
faltan=()

check() { # check <0 si salió bien> <texto si bien> <texto si mal>
  if [ "$1" -eq 0 ]; then
    echo "  ✓ $2"
    bien=$((bien + 1))
  else
    echo "  ✗ $3"
    mal=$((mal + 1))
  fi
}

huella() { # La huella de las pruebas: todo lo que sigue a «NO CAMBIES LAS PRUEBAS».
  local datos
  datos=$(sed -n '/NO CAMBIES LAS PRUEBAS/,$p' "$1")
  if command -v sha256sum >/dev/null; then
    printf '%s\n' "$datos" | sha256sum | cut -d' ' -f1
  else
    printf '%s\n' "$datos" | shasum -a 256 | cut -d' ' -f1
  fi
}

echo "1. Los diez ejercicios (cargo test -p <ejercicio>)"
for carpeta in ejercicios/*/; do
  n=$(basename "$carpeta")
  salida=$(cargo test --quiet -p "$n" 2>&1)
  if [ $? -eq 0 ]; then
    check 0 "$n" ""
    resueltos+=("$n")
  elif echo "$salida" | grep -q 'could not compile'; then
    primero=$(echo "$salida" | grep -m1 -E '^error(\[E[0-9]+\])?: ')
    check 1 "" "$n no compila: $primero"
    faltan+=("$n")
  else
    check 1 "" "$n compila, pero su prueba falla: el arreglo cambió lo que hace la función"
    faltan+=("$n")
  fi
done

echo "2. Las pruebas siguen como venían"
while read -r esperada n; do
  archivo="ejercicios/$n/src/lib.rs"
  if [ ! -f "$archivo" ]; then
    check 1 "" "falta $archivo"
  elif [ "$(huella "$archivo")" = "$esperada" ]; then
    check 0 "pruebas de $n intactas" ""
  else
    check 1 "" "cambiaste las pruebas de $n: arregla la función, no la prueba (git diff $archivo)"
  fi
done <tool/pruebas.sha256

echo "3. Clippy sin advertencias en los ejercicios resueltos"
if [ ${#resueltos[@]} -eq 0 ]; then
  echo "  · todavía no hay ejercicios resueltos"
else
  paquetes=()
  for n in "${resueltos[@]}"; do paquetes+=(-p "$n"); done
  salida=$(cargo clippy --quiet "${paquetes[@]}" --all-targets -- -D warnings 2>&1)
  codigo=$?
  check $codigo "clippy limpio en ${#resueltos[@]} ejercicio(s)" "clippy encontró algo:"
  [ $codigo -ne 0 ] && echo "$salida" | grep -m3 -E '^(error|warning)' | sed 's/^/      /'
fi

echo "4. Formato (cargo fmt --all --check)"
cargo fmt --all --check >/dev/null 2>&1
check $? "el código tiene el formato de rustfmt" "falta formato: corre cargo fmt --all"

echo "5. Tus pruebas detectan código que compila pero está mal (en una copia)"
for trampa in tool/trampas/*.rs; do
  n=$(basename "$trampa" .rs)
  if [[ " ${resueltos[*]} " != *" $n "* ]]; then
    echo "  · $n: primero resuélvelo"
    continue
  fi
  copia=$(mktemp -d)
  cp -r Cargo.toml ejercicios "$copia"/
  [ -f Cargo.lock ] && cp Cargo.lock "$copia"/
  # El código de la trampa con TUS pruebas: si tus pruebas siguen en verde, no están revisando nada.
  {
    cat "$trampa"
    sed -n '/NO CAMBIES LAS PRUEBAS/,$p' "ejercicios/$n/src/lib.rs"
  } >"$copia/ejercicios/$n/src/lib.rs"
  CARGO_TARGET_DIR="$DESTINO" cargo test --quiet --manifest-path "$copia/Cargo.toml" -p "$n" >/dev/null 2>&1
  codigo=$?
  rm -rf "$copia"
  if [ $codigo -ne 0 ]; then r=0; else r=1; fi
  check $r "la prueba de $n detecta $(sed -n '1s/^\/\/ qué: //p' "$trampa")" \
    "la prueba de $n NO detecta la trampa: revisa que siga como venía"
done

echo
if [ ${#faltan[@]} -gt 0 ]; then
  echo "Te faltan ${#faltan[@]} de 10: ${faltan[*]}"
fi
if [ $mal -eq 0 ]; then
  echo "Laboratorio completo: $bien de $bien."
  exit 0
fi
if [ $mal -eq 1 ]; then echo "Falta 1 punto ($bien bien)."; else echo "Faltan $mal puntos ($bien bien)."; fi
exit 1
