# 🧪 Code Lab: Sesión 04 - Módulos y Visibilidad

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/modulos/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/modulos/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Explorador de proyectos, Terminal embebida y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab estructurarás una arquitectura modular completa. Aprenderás a declarar módulos inline, separar código en múltiples archivos en `src/`, aplicar visibilidad granular para proteger datos internos y exponer una API pública limpia mediante `pub use`.

---

## 🏁 Ruta de los 6 Retos Prácticos

### 📌 Reto 1: Declaración del Árbol de Módulos (Declaration & Module Tree)
- **Nivel**: Fundamental | **Tags**: mod & Namespaces
- **Código Base**:
  ```rust
  mod utilidades {
      // Función con visibilidad pública dentro de su módulo
      pub fn saludar() {
          println!("¡Hola desde el módulo utilidades!");
      }

      fn secreto_interno() {
          println!("Esto es privado");
      }
  }

  fn main() {
      // Acceso cualificado con ::
      utilidades::saludar();

      // ❌ Intenta descomentar la siguiente línea:
      // utilidades::secreto_interno(); // ERROR: function `secreto_interno` is private
  }
  ```
- **Tu Tarea**:
  1. Declara un submódulo anidado `mod matematicas { pub fn sumar(a: i32, b: i32) -> i32 { a + b } }` dentro de `utilidades` e invócalo desde `main()`.

---

### 📌 Reto 2: Control Granular de Visibilidad (Visibility)
- **Nivel**: Fundamental | **Tags**: pub, pub(crate) & Privacidad
- **Código Base**:
  ```rust
  mod red {
      // Accesible por cualquier código externo
      pub fn conectar() {
          println!("Conectado al socket");
      }

      // Accesible dentro de este proyecto, pero oculto para consumidores externos
      pub(crate) fn diagnostico() {
          println!("Diagnóstico interno del crate");
      }

      // Estrictamente privado al módulo `red`
      fn clave_encriptacion() -> &'static str {
          "token-secreto-123"
      }
  }

  fn main() {
      red::conectar();
      red::diagnostico();
  }
  ```
- **Tu Tarea**:
  1. Diseña un struct `pub struct Configuracion` dentro de `mod red` donde el campo `pub host: String` sea público, pero el campo `puerto_secreto: u16` sea privado.
  2. Implementa una función constructora `pub fn new(host: String) -> Self` para inicializar el struct.

---

### 📌 Reto 3: Rutas e Importaciones (Paths & Imports)
- **Nivel**: Navegación | **Tags**: crate, self, super, use
- **Código Base**:
  ```rust
  mod hardware {
      pub mod red {
          pub fn encender() {
              println!("Antena de red encendida");
          }
      }

      pub mod wifi {
          pub fn conectar() {
              // Uso de `super::` para acceder a un módulo hermano
              super::red::encender();
              println!("Wifi conectado");
          }
      }
  }

  // Importación limpia con ruta absoluta `crate::`
  use crate::hardware::wifi;

  fn main() {
      wifi::conectar();
  }
  ```
- **Tu Tarea**:
  1. Importa directamente la función `encender` renombrándola como `encender_antena` utilizando la sintaxis `use ... as ...`:
     ```rust
     use crate::hardware::red::encender as encender_antena;
     ```

---

### 📌 Reto 4: Re-exportación y Diseño de API (Re-export & Public API)
- **Nivel**: Diseño de API | **Tags**: pub use & Patrón Façade
- **Código Base**:
  ```rust
  mod capas_internas {
      pub mod motor {
          pub fn arrancar_procesador() {
              println!("Núcleo de cálculo iniciado a máxima potencia");
          }
      }
  }

  // Re-exportamos la función a la raíz para no obligar al usuario
  // a escribir rutas profundas y complejas:
  pub use capas_internas::motor::arrancar_procesador;

  fn main() {
      // Llamada directa y cómoda gracias a `pub use`:
      arrancar_procesador();
  }
  ```
- **Tu Tarea**:
  1. Re-exporta un módulo completo con `pub use capas_internas::motor;` y comprueba cómo cambia la llamada en `main`.

---

### 📌 Reto 5: Estructura Física en Disco (Module File Structure)
- **Nivel**: Arquitectura | **Tags**: src/ & Separación de Archivos
- **Estructura a construir**:
  ```text
  src/
  ├── main.rs
  ├── sistema.rs
  └── sistema/
      └── red.rs
  ```
- **Código para `src/main.rs`**:
  ```rust
  mod sistema;

  fn main() {
      sistema::iniciar();
      sistema::red::ping();
  }
  ```
- **Tu Tarea**:
  1. Crea los archivos `src/sistema.rs` y `src/sistema/red.rs` utilizando la terminal o el explorador de archivos integrado.
  2. Conéctalos declarando `pub mod red;` dentro de `src/sistema.rs`.

---

### 📌 Reto 6: Evaluación Técnica (Questions)
- **Nivel**: Autoevaluación | **Tags**: 5 Preguntas Clave
- Cuestionario de 5 preguntas técnicas sobre:
  1. La visibilidad por defecto en Rust.
  2. La diferencia entre `pub` y `pub(crate)`.
  3. Cuándo utilizar `super::` frente a `crate::`.
  4. La regla de visibilidad en los campos de un Struct público.
  5. El objetivo del operador `pub use` en el diseño de librerías idiomáticas.
