# Compile Time y Run Time en Rust

## Idea principal

La diferencia fundamental es:

```text
Compile time → Rust construye y revisa el programa
Run time     → el programa ya construido está funcionando
```

## Compile time

**Compile time** es el proceso que ocurre antes de que nuestro programa se
ejecute. Durante esta etapa Rust:

- analiza el código;
- valida la sintaxis;
- comprueba los tipos de datos;
- aplica las reglas de ownership y borrowing;
- expande macros;
- compila las dependencias;
- genera el código máquina;
- realiza optimizaciones según el perfil `dev` o `release`.

Por ejemplo:

```rust
fn main() {
    let edad: u32 = "veinte";
}
```

Este código produce un error durante **compile time**, porque el texto
`"veinte"` no puede asignarse a una variable de tipo `u32`.

Otros errores que Rust puede detectar antes de ejecutar el programa son:

```rust
fn main() {
    let numero = 10;
    numero = 20;
}
```

La variable es inmutable y el compilador detecta que se intenta modificar.

Los comandos habituales que activan esta etapa son:

```bash
cargo check
cargo build
cargo build --release
cargo run
```

`cargo check` comprueba el código sin generar el binario final. `cargo build`
lo compila en modo `dev`, y `cargo build --release` lo compila con el perfil de
producción. `cargo run` compila si hace falta y después ejecuta el programa.

Los perfiles `dev` y `release` son configuraciones de **compile time**. Se
aplican mientras Rust construye el binario, pero influyen en su comportamiento
durante **run time**, especialmente en optimizaciones, velocidad y depuración.
Consulta [Perfiles `dev` y `release` en Rust](PERFILES_DEV_Y_RELEASE.md) para
ver la comparación completa.

## Run time

**Run time** ocurre cuando el binario ya está ejecutándose. En esta etapa el
programa puede:

- leer la entrada del usuario;
- abrir archivos;
- conectarse a internet;
- hacer cálculos;
- reservar memoria;
- mostrar una ventana;
- comunicarse con otros procesos;
- producir un error si ocurre una situación inesperada.

Por ejemplo:

```rust
fn main() {
    let texto = "hola";
    let numero: u32 = texto.parse().unwrap();

    println!("{numero}");
}
```

Este código puede pasar la compilación porque los tipos y la sintaxis son
correctos. Sin embargo, durante **run time** el programa descubre que `"hola"`
no es un número y `unwrap()` provoca un `panic!`.

Otro ejemplo es la entrada del usuario:

```rust
let numero: u32 = entrada.parse().unwrap();
```

El compilador no puede saber qué escribirá el usuario. Si escribe un número, la
conversión puede funcionar; si escribe texto inválido, puede fallar durante la
ejecución.

## ¿Rust es seguro si puede fallar durante run time?

Sí, pero hay que precisar qué significa **seguridad** en Rust.

Rust evita muchas clases de errores graves en código seguro (*safe Rust*), entre
ellos:

- usar memoria después de liberarla (*use-after-free*);
- liberar dos veces la misma memoria (*double free*);
- acceder silenciosamente a memoria fuera de los límites;
- utilizar referencias inválidas;
- crear *data races* en código concurrente;
- usar datos con tipos incompatibles;
- romper las reglas de ownership y borrowing.

Pero seguridad de memoria no significa que Rust pueda garantizar que toda la
lógica del programa sea correcta o que el entorno externo siempre funcione.
Todavía pueden existir situaciones como:

```text
el archivo no existe
la red está desconectada
faltan permisos
la entrada del usuario es inválida
un servicio externo no responde
se intenta dividir entre cero
la lógica produce un resultado incorrecto
```

Estos problemas dependen de datos y circunstancias que solo se conocen durante
run time.

## Manejar errores durante run time

Rust ofrece tipos como `Result` y `Option` para representar explícitamente
operaciones que pueden fallar o valores que pueden no existir.

Un ejemplo poco controlado es:

```rust
let numero: u32 = texto.parse().unwrap();
```

Una forma más segura de manejar el resultado es:

```rust
let numero: u32 = match texto.parse() {
    Ok(valor) => valor,
    Err(_) => {
        println!("La entrada no es válida");
        return;
    }
};

println!("Número recibido: {numero}");
```

Aquí el programa decide qué hacer si la conversión falla, en lugar de terminar
con un `panic!`.

También podemos propagar el error con el operador `?`:

```rust
fn leer_numero(texto: &str) -> Result<u32, std::num::ParseIntError> {
    let numero = texto.parse()?;
    Ok(numero)
}
```

## Respuesta breve para un alumno

> Rust garantiza principalmente la seguridad de memoria y de concurrencia en
> código seguro, pero no garantiza que la lógica del programa sea correcta ni
> que el entorno externo siempre funcione. Por eso algunos errores solo pueden
> detectarse durante run time. Rust ofrece `Result` y `Option` para manejar esos
> casos de forma explícita y reducir la necesidad de usar `panic!`.

## Comparación final

| Aspecto | Compile time | Run time |
| --- | --- | --- |
| Momento | Antes de ejecutar | Mientras el programa funciona |
| Responsable principal | `rustc` y Cargo | El binario ejecutándose |
| Ejemplos | Tipos, sintaxis, ownership, macros | Entrada, archivos, red, cálculos |
| Resultado posible | Error de compilación | `Result`, `Option`, `panic!` o salida normal |
| Objetivo | Construir y verificar | Realizar el trabajo del programa |

La meta de Rust es trasladar tantos errores como sea posible desde run time hacia
compile time. Los errores que dependen del usuario, de archivos, de redes o de
otros recursos externos normalmente solo pueden resolverse durante la ejecución.
