# 🗺️ Roadmap: Sesión 04 - Módulos y Visibilidad

> **Ubicación en la App**: Menú Lateral `CURSO RUST COMPLETO` ➔ `Módulos`  
> **Ruta interna**: `AppRoute::TutorialModulos`  
> **Vistas asociadas**: [`src/app/views/modulos/`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/modulos/)

---

## 🎯 Objetivos de Aprendizaje

1. **El Árbol Jerárquico de Módulos (`mod`)**:
   - Comprender cómo Rust organiza el código en un árbol lógico de espacios de nombres (*namespaces*).
   - Distinguir módulos definidos *inline* (dentro del mismo archivo) de módulos mapeados a archivos externos en `src/`.

2. **Control de Visibilidad y Encapsulamiento**:
   - Dominar la regla de **privacidad por defecto**: todo elemento en Rust es estrictamente privado a su módulo padre a menos que se indique lo contrario.
   - Usar `pub` para exportar funciones, tipos, structs y constantes.
   - Usar modificadores de visibilidad restringida:
     - `pub(crate)`: visible únicamente dentro del crate actual (ideal para APIs internas de librerías).
     - `pub(super)`: visible únicamente para el módulo padre inmediato.
     - `pub(self)` / privado: visible únicamente en el módulo actual.
   - Visibilidad granular en Structs: la visibilidad de los campos es independiente de la visibilidad de la estructura.

3. **Navegación de Rutas y Accesos Directos**:
   - Rutas absolutas que comienzan desde la raíz: `crate::`.
   - Rutas relativas: `self::` (módulo actual) y `super::` (módulo padre).
   - Importaciones limpias con `use` para evitar rutas verbose redundantes.
   - Renombrado de importaciones conflictivas con `as` (ej. `use std::io::Result as IoResult;`).

4. **Diseño de APIs Públicas y Patrón Façade (`pub use`)**:
   - Desacoplar la estructura interna de archivos de la interfaz que consumen los usuarios de la librería.
   - Re-exportar elementos con `pub use` para aplanar rutas profundas y ofrecer una API idiomática.

5. **Organización Física de Archivos en Rust Moderno (Ediciones 2018–2024)**:
   - Comparar el estilo tradicional (`src/red/mod.rs`) con el estilo moderno estándar (`src/red.rs` junto a la subcarpeta `src/red/`).

---

## 🧭 Estructura de Navegación en FerrisKey

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        BARRA SUPERIOR DE NAVEGACIÓN                    │
├──────────────────────────────────────┬─────────────────────────────────┤
│ 📖 TEORÍA (Lado Izquierdo)           │ 🧪 PRÁCTICA (Lado Derecho)      │
│  [0] Overview (Encapsulamiento)      │  [2] Code Lab (Retos 1 a 6)     │
│  [1] File Structure (src/ & Mod)     │                                 │
└──────────────────────────────────────┴─────────────────────────────────┘
```

### 1. Pestañas de Teoría
- **`Overview` (`overview.rs`)**:
  - Mapa conceptual del sistema de módulos.
  - Tabla comparativa de especificadores de visibilidad (`pub`, `pub(crate)`, `pub(super)`, privado).
  - Reglas de oro para diseño de APIs seguras y mantenibles.
- **`File Structure` (`estructura.rs`)**:
  - Convenciones de carpetas y archivos en proyectos medianos y grandes.
  - El rol de `src/lib.rs` como raíz del árbol de librerías y `src/main.rs` para binarios.

### 2. Pestaña de Práctica: Code Lab
- Interfaz interactiva dividida en 6 retos guiados:
  1. **Reto 1**: Declaration & Module Tree (`mod utilidades { pub fn saludar() ... }`).
  2. **Reto 2**: Visibility (`pub`, `pub(crate)` y campos privados).
  3. **Reto 3**: Paths & Imports (`crate::`, `super::`, `self::` y `use`).
  4. **Reto 4**: Re-export & Public API (`pub use` y patrón Façade).
  5. **Reto 5**: Module File Structure (creación de archivos `.rs` y subdirectorios).
  6. **Reto 6**: Questions (evaluación técnica de 5 preguntas interactivas).

---

## ⏱️ Matriz de Competencias

| Concepto Clave | Vista / Componente en App | Nivel | Verificación Práctica |
|---|---|---|---|
| Declaración de Módulos | `overview.rs`, `Reto 1` | Básico | Creación de módulos y llamadas cualificadas |
| Visibilidad y Seguridad | `overview.rs`, `Reto 2` | Fundamental | Protección de secretos y APIs con `pub(crate)` |
| Rutas Absolutas y Relativas | `Reto 3` | Básico | Uso de `crate::`, `super::` y `use` para imports |
| Re-exportación de APIs | `Reto 4` | Intermedio | Aplanamiento de estructuras complejas con `pub use` |
| Arquitectura de Archivos | `estructura.rs`, `Reto 5` | Intermedio | Separación física de código en `src/modulo.rs` y carpetas |
