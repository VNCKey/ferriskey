# 📖 Teoría Exhaustiva: Sesión 15 - Anatomía de Proyectos Cargo y Workspaces

---

## 1. Binarios (`--bin`) vs Bibliotecas (`--lib`)

Al inicializar un proyecto con Cargo existen dos opciones fundamentales:

```bash
cargo new mi_aplicacion --bin   # Crea src/main.rs (Genera un ejecutable binario)
cargo new mi_libreria --lib     # Crea src/lib.rs (Genera un rlib/crate reutilizable)
```

### Arquitectura de un Proyecto Mixto Profesional

Un paquete puede contener **una biblioteca principal (`src/lib.rs`)** y **múltiples ejecutables**:

```text
mi_proyecto/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── lib.rs              # Núcleo de la lógica (reutilizable)
│   ├── main.rs             # Ejecutable por defecto (`cargo run`)
│   └── bin/
│       ├── servidor.rs     # Ejecutable adicional: `cargo run --bin servidor`
│       └── migrador.rs     # Ejecutable adicional: `cargo run --bin migrador`
├── examples/
│   └── demo.rs             # Ejemplo: `cargo run --example demo`
├── tests/
│   └── integracion.rs      # Tests de integración: `cargo test --test integracion`
└── benches/
    └── rendimiento.rs      # Benchmarks: `cargo bench`
```

---

## 2. Configuración de `Cargo.toml` y Features

Las *features* permiten activar funcionalidades opcionales o dependencias pesadas únicamente cuando se solicitan:

```toml
[package]
name = "servidor_red"
version = "0.1.0"
edition = "2021"

[features]
default = ["json"]
json = ["dep:serde_json"]
cifrado_tls = ["dep:rustls"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", optional = true }
rustls = { version = "0.23", optional = true }

[dev-dependencies]
criterion = "0.5"
```

En el código Rust se condiciona con la directiva `#[cfg(feature = "...")]`:

```rust
#[cfg(feature = "cifrado_tls")]
pub fn iniciar_tls() {
    println!("TLS iniciado");
}
```

---

## 3. Workspaces de Cargo (Monorepos Multi-Crate)

Un **Workspace** agrupa múltiples paquetes relacionados bajo una misma carpeta raíz, compartiendo el directorio `target/` y el archivo `Cargo.lock` para optimizar los tiempos de compilación.

```text
mi_workspace/
├── Cargo.toml           # Cargo.toml raíz del Workspace
├── Cargo.lock
├── target/              # target compartido por todos los paquetes
├── crates/
│   ├── core/            # Crate de dominio (lib)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── api/             # Crate de API web (bin)
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── cli/             # Herramienta de línea de comandos (bin)
│       ├── Cargo.toml
│       └── src/main.rs
```

### `Cargo.toml` Raíz del Workspace

```toml
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/api",
    "crates/cli",
]

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

### Dependencia entre Crates del Workspace (`crates/api/Cargo.toml`)

```toml
[package]
name = "api"
version = "0.1.0"
edition = "2021"

[dependencies]
core = { path = "../core" } # Dependencia local inmediata
serde = { workspace = true }
tokio = { workspace = true }
```
