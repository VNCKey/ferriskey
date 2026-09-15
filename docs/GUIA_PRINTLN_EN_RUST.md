# Guía de `println!` en Rust

`println!` es una macro de la biblioteca estándar que imprime texto en la
salida estándar y agrega un salto de línea. Utiliza el sistema de formato de
Rust para representar valores de distintas maneras.

## Uso básico

```rust
println!("Hola, Rust");

let nombre = "Alek";
let edad = 25;

println!("Me llamo {nombre} y tengo {edad} años");
println!("Resultado: {}", 2 + 3);
```

Las llaves indican dónde se insertará un valor. Rust también permite capturar
directamente una variable por su nombre.

## Formatos de visualización

### `Display`: `{}`

Muestra un valor en un formato legible para humanos. El tipo debe implementar
el trait `Display`.

```rust
println!("Nombre: {}", "Rust");
println!("Número: {}", 42);
```

### `Debug`: `{:?}`

Muestra información útil para depuración. Es habitual usarlo con vectores,
tuplas, `Option`, `Result`, enums y structs que implementen `Debug`.

```rust
let valores = vec![1, 2, 3];
println!("{:?}", valores);
```

### Debug estructurado: `{:#?}`

Es la versión “bonita” de `Debug`: agrega saltos de línea e indentación.

```rust
println!("{:#?}", valores);
```

### Pointer: `{:p}`

Muestra en hexadecimal la dirección apuntada por una referencia o puntero.

```rust
let numero = 42;
println!("Dirección: {:p}", &numero);
```

## Bases numéricas

Estos formatos se aplican principalmente a enteros:

```rust
println!("Binario: {:b}", 255); // 11111111
println!("Hexadecimal: {:x}", 255); // ff
println!("Hexadecimal: {:X}", 255); // FF
println!("Octal: {:o}", 255); // 377
```

También se puede solicitar el prefijo de la base:

```rust
println!("{:#b}", 10); // 0b1010
println!("{:#x}", 255); // 0xff
```

## Ancho, relleno y alineación

El ancho es mínimo: si el valor ocupa más espacio, no se corta.

```rust
println!("|{:5}|", 42);    // alineación derecha por defecto
println!("|{:<5}|", 42);   // izquierda
println!("|{:^5}|", 42);   // centrada
println!("|{:0>5}|", 42);  // ceros a la izquierda
println!("|{:*>5}|", 42);  // asteriscos a la izquierda
println!("{:08b}", 5);     // 00000101
```

En `{:08b}`, `b` selecciona binario, `8` indica el ancho mínimo y `0` indica
que se debe rellenar con ceros.

## Precisión y notación científica

La precisión controla la representación visual; no modifica el valor original.

```rust
println!("{:.2}", 3.14159); // 3.14
println!("{:e}", 10000.0);  // 1e4
println!("{:E}", 10000.0);  // 1E4
```

El formato de precisión redondea lo que se muestra, pero no convierte el valor
en uno con menos decimales.

## Reutilizar y nombrar argumentos

```rust
println!("{1} {0}", "Mundo", "Hola"); // Hola Mundo

println!(
    "{saludo} {nombre}",
    saludo = "Hola",
    nombre = "Rustacean"
);
```

## Llaves literales

Para imprimir llaves sin que Rust las interprete como marcadores de formato,
se duplican:

```rust
println!("JSON vacío: {{}}"); // JSON vacío: {}
```

## Macros relacionadas

```rust
print!("Sin salto de línea");
eprintln!("Mensaje en stderr");

let texto = format!("Valor: {}", 42); // construye un String
let resultado = dbg!(2 + 3);          // muestra información de depuración
```

- `print!` imprime sin agregar un salto de línea.
- `eprintln!` escribe en la salida estándar de errores.
- `format!` construye y devuelve un `String`, pero no imprime nada.
- `dbg!` muestra archivo, línea, expresión y valor; además devuelve el valor
  para poder continuar usándolo.

## Idea clave

`println!` no es una función normal: es una **macro**. El compilador puede
analizar la cadena de formato y comprobar que los argumentos sean compatibles
con el formato solicitado. Los formatos se apoyan en traits como `Display`,
`Debug` y `Pointer`.
