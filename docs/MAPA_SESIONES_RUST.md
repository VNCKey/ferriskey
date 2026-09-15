# Mapa de sesiones de FerrisKey

Este documento registra la evolución prevista del curso interactivo de FerrisKey.

La intención no es añadir todos los conceptos de Rust de una sola vez, sino ordenar
los temas según la progresión mental del estudiante. Cada sesión debe explicar un
concepto, mostrarlo visualmente y terminar con una práctica pequeña.

## Documento complementario

La guía [Cargo, toolchain y compilación en Rust](CARGO_TOOLCHAIN_Y_COMPILACION.md)
reúne el funcionamiento de `Cargo.toml`, `Cargo.lock`, los perfiles `dev` y
`release`, los comandos de Cargo, los toolchains, los targets y la inspección de
binarios con `ldd` y `file`.

La guía [Compile Time y Run Time en Rust](COMPILE_TIME_Y_RUN_TIME.md) explica
qué ocurre antes y durante la ejecución, qué errores detecta el compilador y
qué problemas deben manejarse durante run time.

La guía [Perfiles `dev` y `release` en Rust](PERFILES_DEV_Y_RELEASE.md) compara
los perfiles de desarrollo y producción, sus comandos, optimizaciones, rutas de
salida y comportamiento frente al overflow.

La guía [Cargo Configuration & Project Architecture](CARGO_CONFIGURACION_Y_ARQUITECTURA.md)
queda como planificación para una futura sesión sobre `Cargo.toml`, features,
`build.rs`, workspaces, crates y configuración por plataforma.

La [Guía de `println!` en Rust](GUIA_PRINTLN_EN_RUST.md) reúne los formatos de
visualización, bases numéricas, alineación, precisión, argumentos nombrados y
macros relacionadas como `print!`, `eprintln!`, `format!` y `dbg!`.

La guía [Aserciones y validaciones en Rust](GUIA_ASSERTS_Y_VALIDACIONES.md)
explica `assert!`, `assert_eq!`, `assert_ne!`, `debug_assert!` y la diferencia
entre comprobar invariantes y manejar errores esperados con `Result` u
`Option`.

La guía [Conversiones y traits estándar de Rust](CONVERSIONES_Y_TRAITS_ESTANDAR.md)
explica `From`, `Into`, `TryFrom` y `TryInto`, sus diferencias con `as`, y su
uso para transformar modelos de aplicación y mensajes gRPC.

El [Plan de la sesión `Custom Types`](PLAN_CUSTOM_TYPES.md) organiza structs,
campos, visibilidad, `impl`, traits, `derive`, composición, patrones y
serialización, dejando generics y lifetimes para sesiones independientes.

## Estado general

FerrisKey ya cubre una base amplia:

- entorno de Rust, `rustc`, Cargo, crates, Clippy, Rustfmt y rustup;
- compile time, run time y perfiles Debug/Release;
- variables, mutabilidad, tipos primitivos, scopes, expresiones y funciones;
- arrays, slices y tuplas;
- strings, stack, heap, ownership, moves, `Copy`, `Clone`, `Drop` y borrowing;
- módulos, rutas, visibilidad y reexportación con `pub use`;
- colecciones, control de flujo, closures, iteradores, structs, enums, errores,
  generics y traits.

Las ampliaciones deben priorizar conexiones y profundidad, no únicamente más
terminología.

## Sesión nueva: Anatomy of a Cargo Project

Nombre recomendado en inglés: **Anatomy of a Cargo Project**.

Nombre alternativo en español: **Anatomía de un proyecto Cargo**.

Esta sesión explica cómo se organiza un proyecto real y qué función cumple cada
archivo, carpeta y comando.

### 1. `Cargo.toml`

- `[package]`;
- `name`, `version` y `edition`;
- `[dependencies]`;
- versiones compatibles y SemVer;
- features;
- perfiles `[profile.dev]` y `[profile.release]`;
- dependencias normales;
- `[dev-dependencies]` para tests y herramientas de desarrollo;
- `[build-dependencies]` para `build.rs`;
- diferencia entre una dependencia directa y sus dependencias transitivas.

Práctica sugerida: crear una dependencia, cambiar una feature y observar cómo
Cargo actualiza el proyecto.

### 2. `Cargo.lock`

- versiones exactas que fueron resueltas;
- checksum y origen de los paquetes;
- diferencia entre una versión compatible y una versión fijada;
- cuándo se debe subir al repositorio;
- diferencia práctica entre proyectos binarios y librerías;
- reproducibilidad de una compilación.

Regla didáctica: explicar que una aplicación/binario normalmente conserva su
`Cargo.lock`, mientras que una librería publicada suele dejar que sus usuarios
resuelvan sus propias versiones compatibles.

### 3. `target/`

- `target/debug/`;
- `target/release/`;
- ejecutables compilados;
- dependencias compiladas;
- archivos incrementales;
- artefactos intermedios;
- por qué normalmente aparece en `.gitignore`;
- qué ocurre con `cargo clean`;
- por qué borrar `target/` obliga a recompilar, pero no borra el código fuente.

Práctica sugerida: comparar `cargo build` con `cargo build --release`, localizar
ambos ejecutables y observar el efecto de `cargo clean`.

### 4. Estructura de archivos

- `src/main.rs` para un binario;
- `src/lib.rs` para la librería del paquete;
- `src/bin/` para binarios adicionales;
- `tests/` para tests de integración;
- `examples/` para ejemplos ejecutables;
- `benches/` para benchmarks;
- `build.rs` para lógica ejecutada antes de compilar el paquete;
- diferencia entre paquete, crate binaria y crate biblioteca.

Práctica sugerida: convertir un proyecto de un único `main.rs` en una estructura
con librería, binario, ejemplo y test de integración.

### 5. Comandos esenciales

```text
cargo new
cargo init
cargo check
cargo build
cargo run
cargo test
cargo fmt
cargo clippy
cargo doc
```

Para cada comando conviene explicar: qué hace, si genera archivos, si ejecuta el
programa y cuándo lo usaría un desarrollador.

## Sesión nueva: Rust Toolchains and Versions

Nombre recomendado en inglés: **Rust Toolchains and Versions**.

Nombre alternativo: **Versiones y toolchains de Rust**.

La palabra correcta es `Nightly`, no “modo noche”. Es un canal de compilación de
Rust que contiene características experimentales.

### Canales de Rust

- `stable`: canal recomendado para la mayoría de proyectos;
- `beta`: próxima versión que probablemente llegará a stable;
- `nightly`: características experimentales y cambios aún no estabilizados.

### Comandos de `rustup`

```bash
rustup show
rustup toolchain list
rustup default stable
rustup update
rustup override set nightly
```

También se debe explicar que un override local afecta al directorio actual y sus
subdirectorios, mientras que `rustup default` cambia la herramienta global.

### `rust-toolchain.toml`

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

Este archivo permite que un proyecto declare el toolchain esperado y facilite la
reproducción del entorno en otros equipos.

### Conceptos que deben diferenciarse

- versión del compilador (`rustc`);
- edición del lenguaje: 2018, 2021 y 2024;
- versión de una crate, por ejemplo `egui = "0.35"`;
- canal stable, beta o nightly;
- plataforma host donde se compila;
- plataforma target para la que se genera el binario;
- targets como Linux, Windows, macOS o WebAssembly.

## Memoria: ampliaciones y límites

La sesión actual ya cubre `Copy`, `Clone`, `Drop`, `move`, `mut`, ownership,
borrowing, stack, heap, `String` y `&str`. No es necesario añadir todo lo demás
en esta sesión.

### Ampliación inmediata recomendada

- lifetimes básicos;
- por qué una referencia no puede sobrevivir al valor que presta;
- anotaciones como `<'a>` solo después de mostrar ejemplos donde el compilador
  las necesite;
- operador `*` de dereferencia;
- diferencia entre `&valor` y `*referencia`;
- relación entre referencias, slices y lifetimes.

### Temas que deben esperar

Para no saturar la sesión inicial de memoria, dejar para módulos posteriores:

- `Box<T>`;
- `Rc<T>`;
- `Arc<T>`;
- `RefCell<T>`;
- `Mutex<T>`;
- `Send` y `Sync`;
- concurrencia y asincronía.

Orden recomendado: lifetimes → smart pointers → interior mutability →
concurrencia → async.

## Tipos y conversiones: qué verificar y qué ampliar

Algunos conceptos ya aparecen en sesiones anteriores y no deben duplicarse sin
motivo. Conviene crear una tabla de repaso para confirmar dónde se explican:

- `const` frente a `static`;
- aliases con `type`;
- tipo unitario `()`;
- tipo nunca `!`;
- `Option<T>` y `Result<T, E>` como enums;
- `String` y `&str`;
- casteo con `as`;
- inferencia de tipos y anotaciones.

Ampliaciones que sí conviene añadir:

- `From` y `Into`;
- `TryFrom` y `TryInto`;
- `parse` y errores de conversión;
- `OsString` y `OsStr` para texto del sistema operativo;
- `PathBuf` y `Path` para rutas de archivos.

`OsString` y `PathBuf` deben presentarse como tipos prácticos para interactuar
con el sistema operativo, no como otra forma general de manejar strings.

## Control de flujo y pattern matching

Esta sesión puede crecer bastante, pero debe conservar una progresión clara.

### Primera ampliación

- `if let`;
- `while let`;
- `let else`;
- relación de estas formas con `Option` y `Result`;
- múltiples patrones con `|`;
- comodín `_`;
- rangos;
- guards con `if`;
- bloques como expresiones;
- `break` devolviendo valores.

### Segunda ampliación

- labels en bucles;
- patrones de tuplas;
- patrones de structs;
- patrones de enums;
- patrones de arrays y slices;
- patrones de referencia;
- binding con `@`;
- patrón `..` para ignorar campos o elementos;
- exhaustividad y patrones inalcanzables.

Los patrones de arrays y slices pueden añadirse pronto porque arrays y slices ya
aparecen antes en el recorrido. Los patrones de structs pueden retomarse después
de la sesión de structs para evitar introducir tipos que el estudiante todavía
no conoce.

## Funciones, closures e iteradores

Esta área requiere una ampliación gradual.

### Funciones y closures

- funciones asociadas frente a métodos;
- parámetros `self`, `&self` y `&mut self`;
- funciones `fn` frente a closures;
- captura por referencia;
- captura mutable;
- captura por movimiento con `move`;
- traits `Fn`, `FnMut` y `FnOnce`;
- `impl Fn`;
- `dyn Fn`;
- función como argumento y función como retorno.

### Iteradores

- `Iterator` como trait;
- `iter`, `iter_mut` e `into_iter`;
- iteradores lazy;
- adaptadores y consumidores;
- `collect`;
- anotaciones turbofish como `::<T>`;
- ownership durante una cadena de iteradores;
- crear un iterador propio;
- diferencia entre devolver un iterador concreto y `impl Iterator`.

La pregunta central de esta sesión debe ser: “¿qué se mueve, qué se presta y qué
se consume en esta cadena?”. Así se conectan iteradores con ownership.

## Patrones de Rust relacionados con el curso

### Patrones del lenguaje

Estos sí pertenecen al lenguaje y deben aparecer dentro de las sesiones:

- pattern matching;
- desestructuración;
- `if let`, `while let` y `let else`;
- guards;
- `|`, `_`, `..` y `@`;
- ownership y move;
- borrowing;
- RAII mediante `Drop`;
- iterator pipeline;
- propagación de errores con `?`;
- newtype;
- state machine mediante enums;
- trait-based polymorphism;
- type-state;
- zero-cost abstractions.

### Patrones de diseño aplicables a FerrisKey

- **Facade**: ya se explica con `pub use`;
- **State Machine**: navegación mediante `AppRoute` y pasos de tutorial;
- **Adapter**: conectar el editor con distintos motores de ejecución;
- **Strategy**: elegir entre `rustc`, Cargo o Rust Playground;
- **Builder**: construir configuraciones o proyectos;
- **Newtype**: representar IDs y rutas de forma segura;
- **Command**: representar acciones como ejecutar, compilar o navegar;
- **Repository**: separar la gestión de proyectos de la interfaz;
- **Composite**: componer tarjetas, tabs y paneles de UI;
- **Dependency Injection**: proporcionar distintos ejecutores o proveedores.

Los patrones de async, actores, canales, producer/consumer y runtimes deben
quedar fuera del mapa inicial hasta completar ownership, lifetimes, traits,
errores y testing.

## Orden de trabajo recomendado

1. Anatomy of a Cargo Project.
2. Rust Toolchains and Versions.
3. Ampliación de pattern matching y control de flujo.
4. Lifetimes y operador de dereferencia.
5. Funciones y closures avanzadas.
6. Iteradores como trait y `collect`.
7. Conversiones y tipos del sistema operativo.
8. Testing y documentación.
9. Patrones de diseño aplicados a FerrisKey.
10. Smart pointers, concurrencia y async.

Cada ampliación debe revisarse contra las sesiones existentes para evitar explicar
dos veces `const`, `static`, `Option`, `Result`, `String`, `&str` u otros temas que
ya estén cubiertos.
