# Cargo Configuration & Project Architecture

Documento de planificación para una futura sesión de **Rust Foundations**.

Esta sesión explicará que `Cargo.toml` no solo contiene el nombre y las
dependencias de un proyecto: también permite configurar features, perfiles,
workspaces, binarios, librerías, targets y herramientas de compilación.

## Objetivos de la sesión

Al terminar esta sesión, el estudiante debería poder:

- leer y organizar un `Cargo.toml`;
- distinguir un package, una crate y un target;
- activar y desactivar features;
- declarar dependencias opcionales;
- entender para qué sirve `build.rs`;
- organizar varios paquetes mediante un workspace;
- declarar dependencias específicas para cada sistema operativo;
- distinguir configuración de compilación y código de la aplicación.

## Mapa de `Cargo.toml`

```text
Cargo.toml
├── [package]                  → metadatos del paquete
├── [dependencies]             → dependencias normales
├── [dev-dependencies]          → dependencias para tests y benchmarks
├── [build-dependencies]       → dependencias usadas por build.rs
├── [features]                 → funcionalidades opcionales
├── [profile.dev]              → perfil de desarrollo
├── [profile.release]          → perfil optimizado
├── [[bin]]                    → binarios adicionales
├── [lib]                      → configuración de la librería
├── [target.*]                 → configuración por plataforma
├── [lints]                    → warnings y Clippy
└── [workspace]                → varios paquetes relacionados
```

## Features

Las features son opciones de compilación que permiten activar o desactivar
funcionalidades. Se declaran en `Cargo.toml`:

```toml
[features]
default = ["json"]
json = ["dep:serde", "dep:serde_json"]
servidor = ["dep:tokio"]

[dependencies]
serde = { version = "1", features = ["derive"], optional = true }
serde_json = { version = "1", optional = true }
tokio = { version = "1", optional = true }
```

Se pueden activar desde la terminal:

```bash
cargo run --features servidor
cargo build --no-default-features
cargo build --all-features
```

En el código se pueden consultar con `cfg`:

```rust
#[cfg(feature = "servidor")]
fn iniciar_servidor() {
    println!("Servidor activado");
}
```

Las features son útiles para mantener una dependencia opcional, crear
variantes de una aplicación o permitir que los usuarios elijan qué partes
compilar.

## `build.rs`

`build.rs` es un script opcional de compilación que se coloca en la raíz del
package, junto a `Cargo.toml`:

```text
mi_proyecto/
├── Cargo.toml
├── build.rs
└── src/
    └── main.rs
```

Cargo compila y ejecuta `build.rs` antes de compilar la crate principal. Por eso
`build.rs` ocurre durante **compile time**, no durante **run time**.

Se utiliza cuando el proyecto necesita realizar una tarea previa a la
compilación, por ejemplo:

- generar código Rust;
- detectar bibliotecas instaladas en el sistema;
- compilar código C o C++;
- enlazar una biblioteca nativa;
- procesar archivos de configuración;
- generar constantes a partir de datos externos;
- indicar a Cargo cuándo debe volver a ejecutar el script.

Un ejemplo mínimo:

```rust
fn main() {
    println!("cargo:rerun-if-changed=datos/config.txt");
    println!("cargo:warning=Ejecutando build.rs");
}
```

Las líneas que empiezan con `cargo:` son instrucciones que Cargo interpreta.

### Generar código con `build.rs`

`build.rs` puede escribir archivos en la carpeta que Cargo proporciona mediante
la variable de entorno `OUT_DIR`:

```rust
use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let destino = out_dir.join("config_generada.rs");

    fs::write(
        destino,
        "pub const VERSION_GENERADA: &str = \"1.0\";",
    )
    .unwrap();
}
```

La crate puede incluir el archivo generado con `include!`:

```rust
include!(concat!(env!("OUT_DIR"), "/config_generada.rs"));

fn main() {
    println!("{VERSION_GENERADA}");
}
```

En un proyecto real, los errores de `unwrap()` del script deberían manejarse
con mensajes claros. También conviene generar archivos solamente dentro de
`OUT_DIR`, porque esa carpeta pertenece a los artefactos de compilación.

### Dependencias para `build.rs`

Si el script necesita una crate, se declara en `[build-dependencies]`:

```toml
[build-dependencies]
cc = "1"
```

Estas dependencias se usan para compilar `build.rs`; no son automáticamente
dependencias disponibles para `src/main.rs`.

### `build.rs` no es `main.rs`

```text
build.rs  → se ejecuta una vez durante la compilación
main.rs   → contiene el programa que se ejecuta durante run time
```

Si se modifica un archivo del que depende el script, se puede indicar a Cargo
que debe volver a ejecutarlo:

```rust
fn main() {
    println!("cargo:rerun-if-changed=datos/config.txt");
}
```

## Workspaces

Un workspace permite organizar varios packages relacionados dentro de un mismo
repositorio:

```text
proyecto/
├── Cargo.toml
├── crates/
│   ├── nucleo/
│   │   └── Cargo.toml
│   └── interfaz/
│       └── Cargo.toml
└── aplicacion/
    └── Cargo.toml
```

El `Cargo.toml` raíz puede declarar:

```toml
[workspace]
members = [
    "crates/nucleo",
    "crates/interfaz",
    "aplicacion",
]
resolver = "3"
```

Un workspace permite compartir `Cargo.lock` y la carpeta `target/`, ejecutar
comandos sobre todos los paquetes y separar el proyecto en crates más pequeñas.

## Package, crate y target

Estos conceptos suelen confundirse:

```text
Package → proyecto gestionado por Cargo; tiene Cargo.toml
Crate   → unidad de compilación Rust; puede ser binaria o librería
Target  → artefacto que Cargo compila, como un binario o una librería
```

Un package puede contener varias crates:

```text
mi_package/
├── src/main.rs       → crate binaria
├── src/lib.rs        → crate de librería
└── src/bin/admin.rs  → crate binaria adicional
```

## Configuración por plataforma

`Cargo.toml` permite declarar dependencias específicas para un sistema:

```toml
[target.'cfg(target_os = "windows")'.dependencies]
windows = "..."

[target.'cfg(target_os = "linux")'.dependencies]
nix = "..."
```

El código puede usar atributos `cfg` para compilar partes diferentes:

```rust
#[cfg(target_os = "windows")]
fn plataforma() {
    println!("Windows");
}

#[cfg(target_os = "linux")]
fn plataforma() {
    println!("Linux");
}
```

## Orden recomendado para enseñar el tema

```text
1. Cargo.toml y Cargo.lock
2. Package, crate y target
3. Dependencias normales y opcionales
4. Features
5. Configuración por plataforma
6. build.rs
7. Workspaces
8. Proyecto modular completo
```

La idea central es que Cargo no solo compila un archivo `main.rs`: administra
la arquitectura completa del proyecto, sus variantes, sus dependencias y el
proceso que ocurre antes de generar el binario.
