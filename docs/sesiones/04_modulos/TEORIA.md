# 📖 Teoría: Sesión 04 - Módulos y Visibilidad

---

## 1. El Sistema de Módulos de Rust

A medida que un proyecto crece, colocar todo el código en un único archivo `main.rs` o `lib.rs` se vuelve insostenible. El sistema de módulos de Rust permite:
- **Organización lógica**: Agrupar funciones, estructuras, enums y traits relacionados bajo un mismo espacio de nombres (*namespace*).
- **Encapsulamiento y Privacidad**: Ocultar detalles internos de implementación para que los cambios internos no rompan el código que consume el módulo.
- **Jerarquía y Rutas**: Crear un árbol predecible que mapea directamente a la arquitectura de archivos del disco.

---

## 2. Declaración de Módulos (`mod`)

La palabra clave `mod` declara un módulo. Puede escribirse de forma *inline*:

```rust
mod red {
    pub fn conectar() {
        println!("Conectando al servidor...");
    }
}
```

O puede dividir el código en archivos independientes. Cuando Rust ve `mod red;` en `src/main.rs`, busca automáticamente en el disco:
1. `src/red.rs` (Estilo recomendado en Rust moderno).
2. `src/red/mod.rs` (Estilo heredado de Rust 2015).

---

## 3. Niveles de Visibilidad y Encapsulamiento

En Rust, **todos los elementos son privados por defecto** respecto a sus módulos padre y módulos hermanos.

| Modificador | Alcance de Visibilidad | Caso de Uso Típico |
|---|---|---|
| *(Ninguno / Privado)* | Solo el módulo actual y sus submódulos hijos. | Funciones auxiliares y detalles internos de implementación. |
| `pub` | Completamente público para cualquiera que pueda acceder al módulo. | Métodos y tipos que forman parte de la API pública externa. |
| `pub(crate)` | Visible en cualquier parte dentro del mismo crate, pero oculto fuera. | Funciones internas compartidas entre módulos sin exponerlas al usuario de la librería. |
| `pub(super)` | Visible únicamente para el módulo padre inmediato. | Funciones de coordinación entre submódulos hermanos. |
| `pub(in ruta)` | Visible dentro de un ancestro específico del árbol. | Control fino en arquitecturas complejas de múltiples niveles. |

```rust
mod seguridad {
    pub fn autenticar_usuario() {
        // Público para el exterior
        descifrar_clave_secreta(); // ✅ Válido: dentro del mismo módulo
    }

    pub(crate) fn auditoria_interna() {
        // Visible en cualquier parte de este crate
    }

    fn descifrar_clave_secreta() {
        // Privado: inaccesible desde fuera del módulo `seguridad`
    }
}
```

### Visibilidad en Structs y Enums
- **Enums**: Si un enum se declara `pub`, **todas sus variantes son automáticamente públicas**.
- **Structs**: Si un struct se declara `pub`, **sus campos permanecen privados por defecto**. Debes anteponer `pub` individualmente a cada campo que desees exponer. Si un struct tiene al menos un campo privado, los consumidores externos están obligados a utilizar un constructor (como `new()`) para inicializarlo.

---

## 4. Rutas (Paths): Absolutas vs. Relativas

Para referenciar un elemento en el árbol de módulos existen dos caminos:

1. **Ruta Absoluta**: Comienza desde la raíz del crate usando `crate::`:
   ```rust
   crate::red::http::cliente::enviar();
   ```
2. **Ruta Relativa**: Comienza desde el módulo actual usando `self::` o subiendo con `super::`:
   ```rust
   super::configuracion::obtener_puerto();
   ```

### Simplificación de Rutas con `use`
Para no escribir rutas largas repetidamente, la palabra clave `use` crea un acceso directo en el ámbito local:

```rust
use crate::red::http::cliente;

fn main() {
    cliente::enviar(); // Breve y legible
}
```

---

## 5. Re-exportación y el Patrón Façade (`pub use`)

Cuando desarrollas una librería o un sistema complejo, la estructura de carpetas más cómoda para programar (con múltiples subdirectorios) suele ser incómoda y engorrosa para el usuario que la consume.

El operador `pub use` permite tomar un elemento de un módulo interno profundo y **re-exportarlo en un nivel superior**:

```rust
mod motor {
    pub mod fisica {
        pub struct Gravedad;
    }
}

// Re-exportamos Gravedad en la raíz pública:
pub use motor::fisica::Gravedad;
```

Ahora un usuario externo puede escribir simplemente:
```rust
use mi_crate::Gravedad; // En lugar de use mi_crate::motor::fisica::Gravedad;
```
Este principio de diseño se conoce como el **Patrón Façade** (*Fachada*): una arquitectura interna rica pero una interfaz externa simple y pulida.

---

## 6. Organización de Archivos en el Sistema de Archivos

Estructura moderna recomendada:

```text
src/
├── main.rs              # Raíz del binario: declara `mod red;`
├── red.rs               # Define el módulo `red` y declara `pub mod cliente;`
└── red/                 # Subcarpeta que contiene los submódulos de `red`
    ├── cliente.rs       # Código del submódulo `red::cliente`
    └── servidor.rs      # Código del submódulo `red::servidor`
```
Esta disposición elimina la proliferación de archivos idénticos llamados `mod.rs` en el editor, facilitando la búsqueda y navegación por pestañas.
