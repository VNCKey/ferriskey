# 🗺️ Roadmap: Sesión 01 - Rust Foundations

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Rust Foundations`  
> **Ruta interna**: `AppRoute::TutorialCargo`  
> **Vistas asociadas**: [`src/app/views/pilares/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/pilares/)

---

## 🎯 Objetivos de Aprendizaje

1. **Comprender el Ecosistema y Filosofía de Rust**:
   - Conocer el origen de Rust (creado por Graydon Hoare en 2006, la metáfora de los *Rust Fungi*).
   - Entender las 4 garantías fundamentales: **Speed** (sin Garbage Collector), **Concurrency** (sin *Data Races*), **Safety** (sin *Null Pointers* ni fallos de segmentación) y **Portability** (multiplataforma: Servidores, Escritorio, Móvil, Embedded y WebAssembly).
   - Distinguir las responsabilidades del compilador (`rustc`), el gestor de paquetes (`Cargo`) y el registro comunitario (`crates.io`).

2. **Dominar el Pipeline de Compilación**:
   - Visualizar las 5 etapas desde el código fuente hasta el código máquina:
     1. `Parse / AST`: Análisis léxico, sintáctico y generación del Abstract Syntax Tree.
     2. `Types + Borrow`: Tipado estático, inferencia y validación del *Borrow Checker*.
     3. `MIR`: Mid-Level Intermediate Representation para optimizaciones de flujo y monomorfización.
     4. `LLVM Codegen`: Generación de código nativo optimizado y archivos objeto (`.o`).
     5. `Linker`: Enlace de dependencias estáticas/dinámicas y producción del binario ejecutable (ELF / PE / Mach-O).

3. **Anatomía del Proyecto y Configuración**:
   - Estructura de directorios estándar de Cargo (`src/main.rs`, `src/lib.rs`, `src/bin/`, `tests/`, `examples/`, `benches/`, `target/`).
   - Anatomía de `Cargo.toml`: secciones `[package]`, `[dependencies]`, ediciones (2015, 2018, 2021, 2024) y perfiles `[profile.dev]` y `[profile.release]`.
   - Papel determinista de `Cargo.lock`.
   - Configuración de Toolchains (`rustup`, canales `stable`, `beta`, `nightly` y archivo `rust-toolchain.toml`).

4. **Calidad de Código y Herramientas Satélite**:
   - `Clippy`: Linter con más de 650 reglas para código idiomático.
   - `Rustfmt`: Formateador oficial estándar.
   - `Rust-Analyzer`: Servidor de lenguaje (LSP) para IDEs.

5. **Inspección Física del Binario (Binary Analyzer)**:
   - Medir el impacto de las banderas de optimización (`opt-level`, `lto`, `codegen-units`, `strip`).
   - Inspeccionar encabezados, tamaño en disco, memoria virtual, tabla de secciones y dependencias dinámicas con el motor integrado de análisis binario.

---

## 🧭 Estructura de Navegación en FerrisKey

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        BARRA SUPERIOR DE NAVEGACIÓN                    │
├──────────────────────────────────────┬─────────────────────────────────┤
│ 📖 TEORÍA (Lado Izquierdo)           │ 🧪 PRÁCTICA (Lado Derecho)      │
│  [0] Rust Ecosystem                  │  [1] Code Lab (Retos 1 a 6)     │
│  [2] Crates (Explorador Crates.io)   │                                 │
└──────────────────────────────────────┴─────────────────────────────────┘
```

### 1. Pestañas de Teoría
- **`Rust Ecosystem`**:
  - ¿Qué es Rust?, Historia y Origen (hongos de la roya).
  - Compilación nativa y las 4 ventajas clave (*Speed, Concurrency, Safety, Portability*).
  - Los 3 pilares del núcleo: `rustc`, `Cargo`, `Crates`.
  - Herramientas de calidad: `Clippy`, `Rustfmt`, `Rust-Analyzer`.
  - Pipeline de compilación interactivo con visualización animada de etapas.
  - Anatomía de proyectos y estructura del directorio `target/`.
  - Workflow de comandos de Cargo y gestión de Toolchains con `rustup`.
- **`Crates`**:
  - Explorador interactivo en tiempo real conectado a la API oficial de [crates.io](https://crates.io).
  - Filtrado por categorías temáticas (Web, Criptografía, CLI, Juegos, Embedded, GUI).
  - Búsqueda en vivo de crates populares (`serde`, `tokio`, `rand`, `syn`, etc.).
  - Ficha técnica completa de cada crate: descargas, versiones, dependencias, características (*features*) y licencia.

### 2. Pestaña de Práctica: Code Lab
- Interfaz interactiva dividida en:
  - Selector de proyecto y archivo activo en tiempo real.
  - Editor de código interactivo con resaltado sintáctico Syntect (Rust y TOML).
  - Drawer desplegable con explorador de archivos y terminal interactiva embebida.
  - Consola de ejecución de compilación y ejecución nativa (`cargo run` / `rustc`).
  - Ruta de 6 Retos Guiados:
    1. **Reto 1**: Verificación del Entorno y Primer Ejecutable.
    2. **Reto 2**: Anatomía de `Cargo.toml` y Metadatos.
    3. **Reto 3**: Compilación Debug vs. Release.
    4. **Reto 4**: Perfiles de Optimización Extrema (`LTO`, `strip`, `codegen-units`).
    5. **Reto 5**: Toolchains y Linters (`clippy`, `rustfmt`).
    6. **Reto 6**: Análisis Físico del Binario con el Binary Analyzer.

---

## ⏱️ Matriz de Competencias

| Concepto Clave | Vista / Componente en App | Nivel | Verificación Práctica |
|---|---|---|---|
| Filosofía y Ventajas de Rust | `entorno.rs` | Básico | Comprensión de ausencia de GC y prevención de Data Races |
| Pipeline de `rustc` | `pipeline.rs` | Intermedio | Análisis del flujo Parse ➔ Borrow ➔ MIR ➔ LLVM ➔ Linker |
| Gestión de Paquetes con Cargo | `cargo_workflow.rs`, `project_anatomy.rs` | Básico | Creación de binarios (`cargo new`) vs librerías (`--lib`) |
| Manifiesto y Metadatos | `anatomy.rs` (Reto 2) | Básico | Configuración de dependencias y ediciones en `Cargo.toml` |
| Perfiles de Compilación | `estructura_tiempos.rs`, `anatomy.rs` (Retos 3 y 4) | Avanzado | Comparación física de tamaños y rendimiento (`dev` vs `release`) |
| Ecosistema de Librerías | `crates_io.rs` | Básico | Búsqueda, evaluación de dependencias y versiones SemVer |
| Inspección de Binarios Nativa | `binary_analyzer.rs` (Reto 6) | Avanzado | Análisis de secciones de memoria (ELF/PE/Mach-O) y dependencias |
