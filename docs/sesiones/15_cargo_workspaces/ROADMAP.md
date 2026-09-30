# 🗺️ Roadmap: Sesión 15 - Anatomía de Proyectos Cargo y Workspaces

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Cargo & Workspaces`  
> **Ruta interna**: `AppRoute::TutorialCargoWorkspaces`  
> **Vistas asociadas**: [`src/app/views/cargo_workspaces/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/cargo_workspaces/)

---

## 🎯 Objetivos de Aprendizaje

1. **Binarios frente a Bibliotecas (`--bin` vs `--lib`)**:
   * Diferencias entre `src/main.rs` (punto de entrada ejecutable) y `src/lib.rs` (código reutilizable y exportable).
   * Organización híbrida con submódulos y binarios auxiliares en `src/bin/`.
2. **Estructura Estándar de Archivos y Carpetas**:
   * `src/main.rs` y `src/lib.rs`.
   * `src/bin/` (múltiples binarios ejecutables con `cargo run --bin nombre`).
   * `examples/` (ejemplos ejecutables con `cargo run --example nombre`).
   * `benches/` (benchmarks con `cargo bench`).
   * `tests/` (tests de integración).
3. **Gestión Avanzada de `Cargo.toml`**:
   * `[dependencies]`, `[dev-dependencies]` y `[build-dependencies]`.
   * Características opcionales (`[features]`) para compilación condicional.
   * Perfiles de compilación (`[profile.dev]` y `[profile.release]`).
4. **Workspaces de Cargo (Monorepos)**:
   * Crear proyectos multipaquete que comparten un único `Cargo.lock` y un único directorio `target/`.
   * `[workspace]` con declaración de `members = [...]`.
   * Inyección de dependencias locales entre paquetes con `path = "../otro_crate"`.
