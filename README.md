# FerrisKey (v0.1.0-alpha)

> **Plataforma Educativa Interactiva para Aprender Rust de forma Visual e Intuitiva.**

FerrisKey es una aplicación de escritorio desarrollada en Rust con `egui`/`eframe` diseñada para enseñar los conceptos fundamentales e intermedios del lenguaje Rust mediante explicaciones conceptuales, tablas interactivas, visores de código y un editor ejecutable integrado en tiempo real.

---

## Plan de Estudios Integrado

FerrisKey incluye un currículo estructurado progresivamente sin tecnicismos redundantes:

1. **Pilares & Entorno**: Cargo, Estructura del proyecto, Tiempos de compilación y perfiles (`dev` vs `release`).
2. **Conceptos**: Variables, Mutabilidad, Tipos Primitivos, Scopes y Expresiones.
3. **Memoria & Ownership**: Reglas de Ownership, Borrowing (`&` vs `&mut`), Stack vs Heap y `String` vs `&str`.
4. **Módulos & Visibilidad**: Estructura de módulos (`mod`), Visibilidad (`pub`), Rutas y Patrón Facade.
5. **Tipos Compuestos**: Tuplas, Arrays estáticos y Slices (`&[T]`).
6. **Colecciones**: Vectores (`Vec<T>`), Mapas Hash (`HashMap<K, V>`) y Simulador interactivo de capacidad.
7. **Control de Flujo**: Condicionales `if/else`, Bucles (`loop`, `while`, `for`) y Expresiones `match`.
8. **Closures**: Captura de entorno, Parámetros y Movimiento de Ownership (`move`).
9. **Iteradores**: Modos `.iter()`, `.iter_mut()`, `.into_iter()`, Adaptadores Lazy y Consumidores Cero Costo.
10. **Custom Types**: Definición e implementación (`impl`) de Structs, Enums y Traits.
11. **Error Handling**: Manejo idiomático con `Option<T>`, `Result<T, E>` y el operador `?`.
12. **Generics**: Funciones genéricas, Estructuras genéricas y Restricciones de Traits (*Trait Bounds* `<T: Trait>`).
13. **Traits**: Derivación automática con `#[derive(...)]`, Traits Estándar (`Display`, `Clone`, `From`) y Despacho Estático vs Dinámico (`dyn Trait`).

---

## Requisitos Previos

Para compilar y ejecutar FerrisKey necesitas tener instalado Rust y Cargo en tu sistema.

### 1. Instalar Rust Toolchain
Si aún no tienes Rust instalado, ejecuta el instalador oficial:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Dependencias del Sistema (solo para Linux / Ubuntu / Debian)
En Linux se requieren las librerías gráficas X11 / Wayland para GUI con `egui`:

```bash
sudo apt update
sudo apt install -y build-essential libx11-dev libxcb1-dev libxcursor-dev libxrandr-dev libxi-dev libfontconfig1-dev libgl1-mesa-dev
```

---

## Instalación y Ejecución

Sigue estos sencillos pasos para clonar y correr la aplicación localmente:

### 1. Clonar el Repositorio
```bash
git clone https://github.com/VNCKey/ferriskey.git
cd ferriskey
```

### 2. Ejecutar en Modo Desarrollo
```bash
cargo run
```

### 3. Compilar en Modo Producción (Recomendado para Máximo Rendimiento)
```bash
cargo run --release
```

---

## Estructura del Código Fuente

```text
src/
├── main.rs                 # Punto de entrada de la aplicación eFrame/egui
├── app.rs                  # Estado global de la aplicación (PortfolioState)
├── routes.rs               # Definición de rutas y navegación
├── components/             # Componentes de UI (Sidebar, Modales, Editor de Código)
├── execution/              # Motor de compilación y ejecución en vivo
└── views/                  # Vistas del plan de estudios divididas por lección
    ├── pilares/
    ├── conceptos/
    ├── memoria/
    ├── modulos/
    ├── tipos_compuestos/
    ├── colecciones/
    ├── control_flujo/
    ├── funciones/          # Closures y funciones
    ├── iteradores/
    ├── structs/            # Custom Types
    ├── enums/              # Error Handling
    ├── genericos/          # Generics & Trait Bounds
    └── traits/             # Traits Estándar & derive
```

---

## Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo `LICENSE` para más detalles.
