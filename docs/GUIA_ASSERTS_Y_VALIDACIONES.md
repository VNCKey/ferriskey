# Aserciones y validaciones en Rust

Las macros de aserción comprueban que una condición importante del programa se
cumpla. Si la comprobación falla, provocan un `panic!` y muestran información
sobre el problema.

## `assert!`

Recibe una expresión booleana. No devuelve `true` o `false`: espera que la
condición sea verdadera.

```rust
let edad = 20;

assert!(edad >= 18);
```

Si la condición es `true`, el programa continúa. Si es `false`, se produce un
`panic!`.

```rust
assert!(edad < 18, "La edad no cumple la condición");
```

## `assert_eq!`

Comprueba que dos valores sean iguales:

```rust
let resultado = 2 + 3;
let esperado = 5;

assert_eq!(resultado, esperado);
```

Si falla, muestra normalmente los valores `left` y `right`, lo que ayuda a
encontrar la diferencia.

```rust
assert_eq!(resultado, 10, "El cálculo no produjo el valor esperado");
```

## `assert_ne!`

Comprueba que dos valores sean diferentes:

```rust
assert_ne!("activo", "inactivo");
```

## `debug_assert!`

Tiene el mismo propósito que `assert!`, pero se ejecuta principalmente en
compilaciones de desarrollo. Normalmente se elimina en perfiles optimizados de
producción.

```rust
debug_assert!(contador >= 0);
debug_assert_eq!(resultado, esperado);
```

## Uso en tests

Las aserciones son especialmente comunes en pruebas:

```rust
#[test]
fn suma_correctamente() {
    assert_eq!(2 + 3, 5);
}
```

Una aserción correcta no imprime nada. Si falla, el test falla y Rust informa
qué condición no se cumplió.

## Aserciones frente a errores esperados

Las aserciones sirven para invariantes o situaciones que indican un error de
programación:

```rust
assert!(indice < elementos.len());
```

No conviene usarlas para datos inválidos introducidos por un usuario, archivos
que pueden no existir o problemas de red. Esas situaciones son esperables y se
representan mejor con `Result` u `Option`:

```rust
fn validar_edad(edad: u8) -> Result<(), String> {
    if edad < 18 {
        return Err("La edad debe ser al menos 18".to_string());
    }

    Ok(())
}
```

## Resumen

```text
assert!(condición)       → espera que la condición sea true
assert_eq!(a, b)         → espera que a == b
assert_ne!(a, b)         → espera que a != b
debug_assert!(...)       → aserción orientada al desarrollo
Result / Option          → errores esperados y recuperables
```

Las aserciones no devuelven un booleano. Comprueban una condición y provocan
`panic!` cuando el resultado no coincide con lo esperado.
