# Cargo, toolchain y compilación en Rust

Este documento reúne los conceptos básicos para entender qué ocurre cuando un
proyecto Rust se compila, cómo se configura Cargo y cómo se prepara un programa
para distintos sistemas operativos.

## 1. El flujo general

Un proyecto Rust normalmente sigue este flujo:

```text
Código fuente
    ↓
Cargo lee Cargo.toml y resuelve dependencias
    ↓
rustc analiza, verifica tipos y genera código
    ↓
Enlazador + bibliotecas del sistema
    ↓
Binario para el target elegido
```

El código fuente puede ser el mismo para varias plataformas, pero cada
plataforma necesita su propio binario:

```text
Mismo código Rust
    ├── target Linux   → ejecutable ELF
    ├── target Windows → ejecutable .exe
    └── target macOS   → ejecutable Mach-O
```

Por defecto, Cargo compila para el sistema en el que se está ejecutando. Esto
se llama *host*. No es el ejecutable el que detecta el sistema operativo al
iniciarse: el destino se decide durante la compilación.

## 2. `Cargo.toml`

`Cargo.toml` es el manifiesto del proyecto. Define la información del paquete,
las dependencias, las features y los perfiles de compilación.

```toml
[package]
name = "tallertest30"
version = "0.1.0"
edition = "2024"

[dependencies]

[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
debug = false
lto = true
```

### Secciones frecuentes

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }

[dev-dependencies]
criterion = "0.5"

[build-dependencies]
cc = "1"
```

- `[dependencies]`: dependencias necesarias para compilar y ejecutar el programa.
- `[dev-dependencies]`: dependencias usadas por tests, ejemplos o herramientas de desarrollo.
- `[build-dependencies]`: dependencias usadas por un archivo `build.rs`.

## 3. `Cargo.lock`

`Cargo.lock` guarda las versiones exactas que Cargo resolvió para el proyecto,
incluidas las dependencias transitivas.

Esto hace que una compilación pueda repetirse con las mismas versiones. En una
aplicación o binario normalmente se conserva `Cargo.lock` en el repositorio.

## 4. Perfiles `dev` y `release`

Los dos perfiles producen un ejecutable, pero con objetivos diferentes.

| Comando | Perfil | Carpeta | Propósito |
| --- | --- | --- | --- |
| `cargo build` | `dev` | `target/debug/` | Desarrollo y depuración |
| `cargo build --release` | `release` | `target/release/` | Publicación y rendimiento |

El perfil `dev` compila más rápido, conserva más información de depuración y
utiliza pocas optimizaciones. El perfil `release` tarda más en compilar, pero
genera un programa normalmente más rápido y con menos información de depuración.

Una configuración orientada a velocidad puede ser:

```toml
[profile.release]
opt-level = 3
debug = false
lto = true
codegen-units = 1
strip = "symbols"
```

Una configuración orientada principalmente a reducir el tamaño puede ser:

```toml
[profile.release]
opt-level = "z"
debug = false
lto = "fat"
codegen-units = 1
strip = "symbols"
panic = "abort"
incremental = false
```

Estas opciones tienen costes: pueden aumentar mucho el tiempo de compilación y
`panic = "abort"` hace que un `panic!` termine el proceso sin desenrollar la
pila. Conviene medir el resultado en vez de asumir que toda optimización siempre
mejora el programa.

## 5. Comandos esenciales de Cargo

```bash
cargo new proyecto       # crea un paquete binario nuevo
cargo init               # inicializa Cargo en una carpeta existente
cargo check              # verifica el código sin generar el binario final
cargo build              # compila en modo dev
cargo build --release    # compila en modo release
cargo run                # compila, si hace falta, y ejecuta en modo dev
cargo run --release      # compila, si hace falta, y ejecuta en release
cargo test               # ejecuta los tests
cargo test --release     # ejecuta los tests con el perfil release
cargo fmt                # formatea el código
cargo fmt --check        # comprueba el formato sin modificar archivos
cargo clippy             # ejecuta comprobaciones y sugerencias de Clippy
cargo clean              # elimina artefactos de target/
cargo tree               # muestra el árbol de dependencias
```

La diferencia entre estos comandos es importante:

```bash
cargo build --release
```

Solo compila. En cambio:

```bash
cargo run --release
```

compila si es necesario y después ejecuta el binario generado.

## 6. La carpeta `target/`

Cargo guarda sus artefactos en `target/`:

```text
target/
├── debug/
│   └── programa
└── release/
    └── programa
```

También contiene dependencias compiladas, archivos intermedios y datos de
compilación incremental. Normalmente se incluye `target/` en `.gitignore`.

Para eliminar esos artefactos:

```bash
cargo clean
```

Esto no elimina el código fuente ni `Cargo.toml`; solamente obliga a Cargo a
compilar de nuevo.

## 7. Toolchain: `rustup`, `rustc` y Cargo

El *toolchain* es el conjunto de herramientas que permite compilar Rust.

```bash
rustc --version
cargo --version
rustup --version
rustup show
```

- `rustc`: compilador de Rust.
- `cargo`: gestor de proyectos, dependencias y compilación.
- `rustup`: instalador y administrador de toolchains y targets.

Canales habituales:

```bash
rustup toolchain list
rustup default stable
rustup update
```

El canal `stable` es el recomendado para proyectos normales. También existen
`beta` y `nightly`, pero algunas características experimentales requieren
`nightly` y deben utilizarse conscientemente.

## 8. Targets y compilación para otra plataforma

Un *target* identifica una combinación de arquitectura, sistema operativo y
formato de binario. Para ver los targets disponibles:

```bash
rustup target list
```

Para instalar uno:

```bash
rustup target add x86_64-pc-windows-gnu
```

Y para compilar hacia él:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

El resultado se guarda en una carpeta parecida a:

```text
target/x86_64-pc-windows-gnu/release/
```

Instalar el target de Rust no siempre es suficiente para una compilación
cruzada: algunos destinos necesitan también un enlazador, un SDK o herramientas
del sistema. Por eso el código puede ser portable aunque la configuración para
generar el binario necesite herramientas específicas.

## 9. Código específico por plataforma

Si las bibliotecas utilizadas son multiplataforma, el mismo código suele
funcionar sin cambios. Cuando una parte necesita una API específica, se puede
usar compilación condicional:

```rust
#[cfg(target_os = "windows")]
fn sistema() {
    println!("Windows");
}

#[cfg(target_os = "linux")]
fn sistema() {
    println!("Linux");
}
```

También se pueden declarar dependencias específicas en `Cargo.toml`:

```toml
[target.'cfg(target_os = "windows")'.dependencies]
# dependencia usada solo en Windows

[target.'cfg(target_os = "linux")'.dependencies]
# dependencia usada solo en Linux
```

`#[cfg(...)]` no cambia el código mientras el programa se está ejecutando;
decide qué partes se compilan para el target seleccionado.

## 10. Enlazado dinámico y estático

Después de generar código, el enlazador combina el programa con bibliotecas.
Para inspeccionar un ejecutable Linux:

```bash
ldd target/release/programa
file target/release/programa
```

`ldd` muestra bibliotecas dinámicas que el sistema deberá encontrar al ejecutar
el programa. `file` informa el formato, arquitectura, enlazado y si el binario
fue *stripped*.

Un resultado como este:

```text
ELF 64-bit ... dynamically linked ... stripped
```

significa que es un ejecutable Linux de 64 bits, utiliza algunas bibliotecas del
sistema y ya no conserva símbolos de depuración innecesarios.

Un binario dinámico puede necesitar versiones compatibles de bibliotecas como
`glibc`. Un binario estático incluye más dependencias dentro del propio archivo,
pero suele ocupar más y no siempre es posible enlazar todo estáticamente.

## 11. Resumen mental

```text
Cargo.toml       → configuración del paquete
Cargo.lock       → versiones exactas de dependencias
rustup           → toolchains y targets
rustc            → compilador
cargo check      → verifica sin producir el binario final
cargo build      → compila dev
cargo build --release → compila release
target/debug     → artefactos de desarrollo
target/release   → artefactos optimizados
--target         → selecciona la plataforma de destino
#[cfg(...)]      → selecciona código durante la compilación
ldd / file       → inspeccionan el binario Linux
```

La idea central es: el código fuente puede ser compartido entre plataformas,
pero se genera un binario distinto para cada target, usando el perfil y las
herramientas apropiadas.
