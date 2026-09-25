# 🧪 Code Lab: Sesión 01 - Rust Foundations

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/pilares/anatomy.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/pilares/anatomy.rs))  
> **Herramientas activas**: Editor Syntect (Rust y TOML), Terminal Embebida, Explorador de Archivos y Analizador de Binarios.

---

## 🎯 Descripción del Entorno de Práctica

El Code Lab de la Sesión 1 te permite interactuar en vivo con proyectos reales en tu disco duro:
1. **Selector de Proyecto y Archivo**: Selecciona o crea proyectos de Cargo directamente en la interfaz. Al hacer clic en un archivo (`src/main.rs`, `Cargo.toml`, etc.), se carga automáticamente en el editor.
2. **Editor de Código**: Resaltado sintáctico con numeración de líneas y atajos (`Ctrl + S` para guardar, `Ctrl + Enter` o `F5` para compilar y ejecutar).
3. **Consola / Terminal Embebida**: Muestra la salida estándar (*stdout*), diagnósticos de error (*stderr*) y permite interactuar con la shell del sistema operativo.
4. **Ficha de Binarios (Binary Analyzer)**: Inspecciona los ejecutables reales generados en `target/debug` y `target/release`.

---

## 🏁 Ruta de los 6 Retos Prácticos

### 📌 Reto 1: Verificación e Inicialización
- **Nivel**: Fácil | **Tags**: Setup & Entorno
- **Objetivos**:
  1. Verificar la disponibilidad del compilador y gestor de paquetes ejecutando en la terminal:
     ```bash
     rustc --version
     cargo --version
     rustup --version
     ```
  2. Inicializar un nuevo proyecto ejecutable con Cargo:
     ```bash
     cargo new hello_world
     ```
  3. Inspeccionar el archivo `src/main.rs` generado automáticamente:
     ```rust
     fn main() {
         println!("Hello, world!");
     }
     ```
  4. Compilar y ejecutar con `cargo run`.

---

### 📌 Reto 2: Manifiesto `Cargo.toml` y Dependencias
- **Nivel**: Intermedio | **Tags**: Cargo.toml & Crates
- **Objetivos**:
  1. Abrir `Cargo.toml` en el editor interactivo e identificar los campos del paquete:
     ```toml
     [package]
     name = "hello_world"
     version = "0.1.0"
     edition = "2024"
     ```
  2. Añadir una dependencia externa bajo la sección `[dependencies]`. Por ejemplo, la crate matemática o de aleatoriedad `rand`:
     ```toml
     [dependencies]
     rand = "0.8"
     ```
  3. Actualizar `src/main.rs` para utilizar la librería importada:
     ```rust
     use rand::Rng;

     fn main() {
         let mut rng = rand::thread_rng();
         let numero_secreto: u32 = rng.gen_range(1..=100);
         println!("¡Número aleatorio generado por rand!: {numero_secreto}");
     }
     ```
  4. Ejecutar `cargo run` y observar cómo Cargo descarga el crate, compila sus dependencias y genera el archivo `Cargo.lock` con los hashes criptográficos exactos.

---

### 📌 Reto 3: Ciclo de Compilación: Debug vs. Release
- **Nivel**: Esencial | **Tags**: target/ & Optimización
- **Objetivos**:
  1. Compilar el proyecto en modo depuración (desarrollo rápido):
     ```bash
     cargo build
     ```
     Verificar que el binario resultante se aloja en `target/debug/hello_world`.
  2. Compilar el proyecto en modo producción (optimización completa):
     ```bash
     cargo build --release
     ```
     Verificar que el binario resultante se ubica en `target/release/hello_world`.
  3. Comparar las características de ambos ejecutables:
     - El binario `debug` compila casi al instante pero incluye símbolos DWARF pesados y carece de optimizaciones de bucle.
     - El binario `release` toma más tiempo de compilación pero optimiza funciones inline y reduce el tiempo de CPU en órdenes de magnitud.

---

### 📌 Reto 4: Optimización Avanzada en `Cargo.toml`
- **Nivel**: Intermedio | **Tags**: Perfiles & Rendimiento
- **Objetivos**:
  1. Configurar un perfil de lanzamiento personalizado al final de `Cargo.toml`:
     ```toml
     [profile.release]
     opt-level = 3
     lto = "thin"
     codegen-units = 1
     strip = "symbols"
     ```
  2. Volver a compilar en release:
     ```bash
     cargo build --release
     ```
  3. Observar la reducción drástica de tamaño en kilobytes del ejecutable final. `strip = "symbols"` retira la tabla de símbolos innecesaria para el despliegue en servidores o distribución.

---

### 📌 Reto 5: Configuración del Toolchain (`rust-toolchain.toml`)
- **Nivel**: Avanzado | **Tags**: Rustup & Reproducibilidad
- **Objetivos**:
  1. Inspeccionar el toolchain activo del sistema:
     ```bash
     rustup show
     rustup toolchain list
     ```
  2. Crear en la raíz del proyecto el archivo `rust-toolchain.toml` para fijar la versión exacta que todo el equipo de desarrollo debe utilizar:
     ```toml
     [toolchain]
     channel = "stable"
     components = ["rustfmt", "clippy"]
     profile = "minimal"
     ```
  3. Ejecutar `cargo check` y confirmar que el proyecto respeta la configuración declarada.
  4. Ejecutar el linter oficial para asegurar código idiomático:
     ```bash
     cargo clippy
     ```

---

### 📌 Reto 6: Resumen e Inspección con el Binary Analyzer
- **Nivel**: Avanzado | **Tags**: Binary Analysis & ELF/PE/Mach-O
- **Objetivos**:
  1. Abrir la tarjeta interactiva del **Binary Analyzer** integrada en FerrisKey.
  2. Inspeccionar las propiedades físicas del binario compilado:
     - **Tamaño en disco**: Comparación cuantitativa entre `debug` (e.g. ~4.2 MB) vs `release` optimizado (e.g. ~400 KB).
     - **Formato y Arquitectura**: Confirmar ELF64 (Linux), PE32+ (Windows) o Mach-O (macOS), junto con endianness y dirección de entrada (*entry point*).
     - **Tabla de Secciones**:
       - `.text`: Código máquina ejecutable.
       - `.rodata`: Constantes y cadenas de texto literales.
       - `.data` / `.bss`: Variables globales y estáticas.
     - **Dependencias Dinámicas**: Identificar las bibliotecas del sistema enlazadas dinámicamente (`libc.so`, `libm.so`, etc.).
