# Patrones Nativos, Idiomas y Buenas Prácticas en Rust

Este documento reúne la **guía definitiva de patrones de diseño idiomáticos, reglas de oro (*Idioms*) y técnicas arquitectónicas exclusivas de Rust**. A diferencia de la programación orientada a objetos clásica, el ecosistema de Rust se fundamenta en el **sistema de tipos estricto, la seguridad de memoria sin Garbage Collector (RAII), el Borrow Checker y las abstracciones de costo cero (*Zero-Cost Abstractions*)**.

---

## 📑 Tabla de Contenidos

- [🏛️ 1. Patrones Creacionales y de Construcción](#️-1-patrones-creacionales-y-de-construcción)
  - [1.1 Builder Pattern (Constructor Fluido)](#11-builder-pattern-constructor-fluido)
  - [1.2 Type-State Pattern (Estados en Compilación)](#12-type-state-pattern-estados-en-compilación)
  - [1.3 Constructor Idiomático (`new` y `Default`)](#13-constructor-idiomático-new-y-default)
  - [1.4 Fold / Factory con Closures](#14-fold--factory-con-closures)
- [🏗️ 2. Patrones Estructurales y de Tipos (Memoria y Sistema)](#️-2-patrones-estructurales-y-de-tipos-memoria-y-sistema)
  - [2.1 Newtype Pattern (Seguridad de Dominio)](#21-newtype-pattern-seguridad-de-dominio)
  - [2.2 RAII (Resource Acquisition Is Initialization) y `Drop`](#22-raii-resource-acquisition-is-initialization-y-drop)
  - [2.3 Deref Coercion y Polimorfismo Transparente](#23-deref-coercion-y-polimorfismo-transparente)
  - [2.4 FFI Wrapper (Envoltorios Seguros para C)](#24-ffi-wrapper-envoltorios-seguros-para-c)
  - [2.5 Zero-Sized Types (ZST) y `PhantomData`](#25-zero-sized-types-zst-y-phantomdata)
  - [2.6 Extension Traits](#26-extension-traits)
  - [2.7 Clone-On-Write (`Cow<'a, B>`)](#27-clone-on-write-cowa-b)
- [⚙️ 3. Patrones de Comportamiento y Control de Flujo](#️-3-patrones-de-comportamiento-y-control-de-flujo)
  - [3.1 RAII Guard Pattern (Guardianes de Recursos)](#31-raii-guard-pattern-guardianes-de-recursos)
  - [3.2 Functional Command Dispatch (Closures como Comandos)](#32-functional-command-dispatch-closures-como-comandos)
  - [3.3 Strategy Pattern con Trait Bounds y `dyn Trait`](#33-strategy-pattern-con-trait-bounds-y-dyn-trait)
  - [3.4 Visitor Pattern sobre Árboles Sintácticos (AST)](#34-visitor-pattern-sobre-árboles-sintácticos-ast)
- [🛡️ 4. Idiomas de Rust (*Rust Idioms*)](#️-4-idiomas-de-rust-rust-idioms)
  - [4.1 `Option` y `Result` en lugar de `null` o Excepciones](#41-option-y-result-en-lugar-de-null-o-excepciones)
  - [4.2 El Operador `?` para Propagación de Errores](#42-el-operador--para-propagación-de-errores)
  - [4.3 Borrowing sobre Ownership en APIs (`&str`, `&[T]`, `AsRef`)](#43-borrowing-sobre-ownership-en-apis-str-t-asref)
  - [4.4 Intercambio Seguro de Memoria (`std::mem::take` / `replace`)](#44-intercambio-seguro-de-memoria-stdmemtake--replace)
  - [4.5 Conversiones Falibles (`TryFrom` / `TryInto`)](#45-conversiones-falibles-tryfrom--tryinto)
  - [4.6 `#[non_exhaustive]` para Extensibilidad](#46-non_exhaustive-para-extensibilidad)
  - [4.7 Iterar sobre `Option`](#47-iterar-sobre-option)
  - [4.8 API `entry` para Mapas](#48-api-entry-para-mapas)
  - [4.9 No usar `clone()` para silenciar el Borrow Checker](#49-no-usar-clone-para-silenciar-el-borrow-checker)
  - [4.10 Encapsulación para Invariantes de Dominio](#410-encapsulación-para-invariantes-de-dominio)
  - [4.11 Pattern Matching Exhaustivo con `match`](#411-pattern-matching-exhaustivo-con-match)
- [🚀 5. Patrones Concurrencia y Arquitectura](#-5-patrones-concurrencia-y-arquitectura)
  - [5.1 Actor Pattern (Paso de Mensajes con Canales)](#51-actor-pattern-paso-de-mensajes-con-canales)
  - [5.2 Model-View-Update (MVU / Elm Architecture)](#52-model-view-update-mvu--elm-architecture)
  - [5.3 Entity-Component-System (ECS / Data-Oriented Design)](#53-entity-component-system-ecs--data-oriented-design)
- [💡 6. Los 5 Patrones Esenciales que Todo Rustacean Debe Dominar](#-6-los-5-patrones-esenciales-que-todo-rustacean-debe-dominar)

---

## 🏛️ 1. Patrones Creacionales y de Construcción

### 1.1 Builder Pattern (Constructor Fluido)
Permite construir objetos complejos con múltiples parámetros opcionales de forma clara y sin constructores sobrecargados.

```rust
pub struct Servidor {
    pub host: String,
    pub puerto: u16,
    pub tls: bool,
}

pub struct ServidorBuilder {
    host: String,
    puerto: u16,
    tls: bool,
}

impl ServidorBuilder {
    pub fn new() -> Self {
        Self { host: "127.0.0.1".into(), puerto: 8080, tls: false }
    }
    pub fn puerto(mut self, p: u16) -> Self { self.puerto = p; self }
    pub fn con_tls(mut self, activo: bool) -> Self { self.tls = activo; self }
    pub fn build(self) -> Servidor {
        Servidor { host: self.host, puerto: self.puerto, tls: self.tls }
    }
}
```

### 1.2 Type-State Pattern (Estados en Compilación)
Garantiza a nivel de tipos que no se puedan invocar métodos en el orden incorrecto (ej. no puedes llamar a `.enviar()` antes de `.conectar()`).

```rust
pub struct Desconectado;
pub struct Conectado;

pub struct Socket<Estado> {
    direccion: String,
    _estado: std::marker::PhantomData<Estado>,
}

impl Socket<Desconectado> {
    pub fn new(dir: &str) -> Self {
        Self { direccion: dir.to_string(), _estado: std::marker::PhantomData }
    }
    // Consume el estado anterior y devuelve el nuevo tipo
    pub fn conectar(self) -> Socket<Conectado> {
        Socket { direccion: self.direccion, _estado: std::marker::PhantomData }
    }
}

impl Socket<Conectado> {
    pub fn enviar(&self, datos: &[u8]) {
        println!("Enviando a {}: {:?}", self.direccion, datos);
    }
}
```

### 1.3 Constructor Idiomático (`new` y `Default`)
En Rust las funciones asociadas `new()` actúan como constructores estándar, y el trait `Default` permite inicializar estados por defecto de forma universal.

```rust
#[derive(Debug)]
pub struct BufferConfig {
    pub capacidad: usize,
    pub auto_flush: bool,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self { capacidad: 1024, auto_flush: true }
    }
}

impl BufferConfig {
    pub fn new(capacidad: usize) -> Self {
        Self { capacidad, ..Default::default() }
    }
}
```

### 1.4 Fold / Factory con Closures
Construir estructuras o aplicar transformaciones dinámicas pasando funciones o closures constructoras.

```rust
fn crear_elementos<F>(cantidad: usize, mut factory: F) -> Vec<String>
where
    F: FnMut(usize) -> String,
{
    (0..cantidad).map(&mut factory).collect()
}
```

---

## 🏗️ 2. Patrones Estructurales y de Tipos (Memoria y Sistema)

### 2.1 Newtype Pattern (Seguridad de Dominio)
Envolver un tipo primitivo en una tupla struct de un solo campo (`struct Metros(f64); struct Segundos(f64);`). Evita sumar metros con segundos por error en compilación sin sobrecoste en RAM (**0 overhead**).

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metros(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segundos(pub f64);

// El compilador rechaza: let error = Metros(10.0) + Segundos(5.0);
```

### 2.2 RAII (Resource Acquisition Is Initialization) y `Drop`
La memoria, archivos, sockets y mutexes se adquieren al crear la variable y se liberan de forma determinista e infalible al salir del bloque `{}` mediante el trait `Drop`.

```rust
pub struct TempFile {
    pub path: String,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        println!("[RAII]: Eliminando archivo temporal en {}", self.path);
        let _ = std::fs::remove_file(&self.path);
    }
}
```

### 2.3 Deref Coercion y Polimorfismo Transparente
Implementar `Deref` permite que un tipo inteligente (como `String` o `Box<T>`) se comporte transparentemente como una referencia a su contenido (`&str` o `&T`).

```rust
use std::ops::Deref;

pub struct Envoltorio<T>(pub T);

impl<T> Deref for Envoltorio<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}
```

### 2.4 FFI Wrapper (Envoltorios Seguros para C)
Envolver punteros crudos (`*mut c_void`) en structs seguros de Rust que implementan `Drop`, garantizando cero fugas de memoria al interactuar con librerías en C.

```rust
extern "C" {
    fn free_c_resource(ptr: *mut u8);
}

pub struct CResourceSafe {
    ptr: *mut u8,
}

impl Drop for CResourceSafe {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { free_c_resource(self.ptr); }
        }
    }
}
```

### 2.5 Zero-Sized Types (ZST) y `PhantomData`
Estructuras que ocupan 0 bytes en memoria RAM, usadas para validaciones lógicas estáticas en compilación.

### 2.6 Extension Traits
Añadir métodos a tipos existentes en crates externos mediante traits públicos.

```rust
pub trait NumericExt {
    fn al_cuadrado(&self) -> Self;
}

impl NumericExt for i32 {
    fn al_cuadrado(&self) -> Self { self * self }
}
```

### 2.7 Clone-On-Write (`Cow<'a, B>`)
Trabajar con referencias prestadas inmutables y clonar en memoria (*Heap*) únicamente si es necesario mutar el contenido.

---

## ⚙️ 3. Patrones de Comportamiento y Control de Flujo

### 3.1 RAII Guard Pattern (Guardianes de Recursos)
Objetos que conceden acceso exclusivo mientras existen en memoria. Cuando salen de su scope, el cerrojo o recurso se libera automáticamente.

```rust
use std::sync::Mutex;

let mutex = Mutex::new(42);
{
    let mut guard = mutex.lock().unwrap(); // RAII Guard adquirido
    *guard += 1;
} // El guard sale de scope y libera el cerrojo automáticamente
```

### 3.2 Functional Command Dispatch (Closures como Comandos)
Usar `Box<dyn FnOnce()>` o punteros a funciones en lugar de pesadas jerarquías de clases para encolar o despachar tareas.

```rust
pub struct TareaQueue {
    tareas: Vec<Box<dyn FnOnce()>>,
}

impl TareaQueue {
    pub fn encolar(&mut self, f: impl FnOnce() + 'static) {
        self.tareas.push(Box::new(f));
    }
    pub fn ejecutar_todas(self) {
        for tarea in self.tareas {
            tarea();
        }
    }
}
```

### 3.3 Strategy Pattern con Trait Bounds y `dyn Trait`
Elegir entre despacho estático ultra rápido (*Monomorphization* con `<T: Strategy>`) o despacho dinámico en tiempo de ejecución (`&dyn Strategy`).

### 3.4 Visitor Pattern sobre Árboles Sintácticos (AST)
Recorrido exhaustivo de nodos de árboles sintácticos mediante `match` o traits de visita tipados.

---

## 🛡️ 4. Idiomas de Rust (*Rust Idioms*)

### 4.1 `Option` y `Result` en lugar de `null` o Excepciones
Toda ausencia de dato o posible error es un tipo explícito en la firma de la función, forzando al llamador a manejarlo exhaustivamente.

### 4.2 El Operador `?` para Propagación de Errores
Sustituye bloques infinitos de `try/catch` por una sintaxis limpia de una sola línea que desempaqueta el éxito o retorna anticipadamente el error.

```rust
fn leer_puerto(texto: &str) -> Result<u16, std::num::ParseIntError> {
    let puerto = texto.trim().parse::<u16>()?;
    Ok(puerto)
}
```

### 4.3 Borrowing sobre Ownership en APIs (`&str`, `&[T]`, `AsRef`)
Las funciones deben preferir recibir referencias (`&str` o `&[T]`) en lugar de tipos adueñados (`String` o `Vec<T>`) para evitar clonaciones innecesarias impuestas al usuario.

```rust
// Idiomático (acepta &str, String, etc. sin alocar):
fn buscar(texto: &str, clave: &str) -> bool { texto.contains(clave) }
```

### 4.4 Intercambio Seguro de Memoria (`std::mem::take` / `replace`)
Mover un valor fuera de una referencia mutable sin violar las reglas del Borrow Checker (dejando un valor por defecto o un reemplazo en su lugar).

```rust
use std::mem;

struct Buffer {
    datos: Vec<u8>,
}

impl Buffer {
    pub fn vaciar_y_procesar(&mut self) -> Vec<u8> {
        // Extrae el vector actual dejando un Vec vacío en self.datos
        mem::take(&mut self.datos)
    }
}
```

### 4.5 Conversiones Falibles (`TryFrom` / `TryInto`)
Garantizar que las transformaciones entre tipos de datos que puedan fallar devuelvan un `Result` claro y tipado.

```rust
struct Puerto(u16);

impl TryFrom<u32> for Puerto {
    type Error = &'static str;
    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if val <= 65535 {
            Ok(Puerto(val as u16))
        } else {
            Err("Puerto fuera del rango 0-65535")
        }
    }
}
```

### 4.6 `#[non_exhaustive]` para Extensibilidad
Cuando una librería puede crecer en versiones futuras, `#[non_exhaustive]` evita que sus usuarios externos dependan de una lista cerrada de campos o variantes. Así puedes añadir nuevos datos sin romper necesariamente el código consumidor.

```rust
#[non_exhaustive]
pub enum ErrorRed {
    Desconectado,
    TiempoAgotado,
}

pub fn describir(error: ErrorRed) -> &'static str {
    match error {
        ErrorRed::Desconectado => "sin conexión",
        ErrorRed::TiempoAgotado => "tiempo agotado",
        _ => "otro error de red",
    }
}
```

En una API pública, el consumidor debe incluir un caso comodín (`_`) porque en el futuro pueden aparecer variantes nuevas. Úsalo cuando la evolución de la API sea más importante que exigir coincidencias exhaustivas externas.

### 4.7 Iterar sobre `Option`
`Option<T>` puede verse como una colección de cero o un elemento. Esto permite combinarlo con APIs que reciben iteradores sin crear una colección temporal.

```rust
let etiqueta = Some("rust");
let mut etiquetas = vec!["sistemas"];

etiquetas.extend(etiqueta);
```

Si existe un valor, se añade; si es `None`, no se añade nada. Para una decisión puntual, `if let Some(valor) = opcion` suele ser más claro. Este idiom resulta útil al combinar valores opcionales con `extend`, `chain` o adaptadores de iteradores.

### 4.8 API `entry` para Mapas
La API `entry` permite consultar y modificar una clave en un `HashMap` dentro de una sola operación. Evita buscar primero y volver a insertar después.

```rust
use std::collections::HashMap;

let mut visitas = HashMap::new();

*visitas.entry("inicio").or_insert(0) += 1;
*visitas.entry("inicio").or_insert(0) += 1;
```

El primer acceso crea la clave con `0`; el siguiente reutiliza el valor existente. También puedes usar `and_modify` y `or_insert_with` cuando la actualización o la creación necesiten una operación más específica.

### 4.9 No usar `clone()` para silenciar el Borrow Checker
`clone()` no es una solución automática para todos los errores de Ownership. Puede copiar datos del Heap, aumentar el consumo de memoria y ocultar un diseño que podría resolverse con una referencia, un cambio de Scope o `std::mem::take`.

```rust
struct Usuario {
    nombre: String,
    activo: bool,
}

fn desactivar(usuario: &mut Usuario) {
    println!("Desactivando {}", usuario.nombre);
    usuario.activo = false;
}
```

Primero intenta reducir el alcance de un préstamo o separar la lectura de la mutación. Usa `clone()` cuando realmente necesites dos propietarios independientes o cuando la copia sea una decisión explícita del dominio, no solamente para hacer desaparecer un error del compilador.

### 4.10 Encapsulación para Invariantes de Dominio
Campos privados en `structs` para forzar que cualquier instanciación pase por funciones de validación.

### 4.11 Pattern Matching Exhaustivo con `match`
Obligar al compilador a comprobar que todas las variantes de un `enum` estén controladas. Si se añade una nueva variante en el futuro, el compilador señalará exactamente todos los puntos del código que deben actualizarse.

---

## 🚀 5. Patrones Concurrencia y Arquitectura

### 5.1 Actor Pattern (Paso de Mensajes con Canales)
Aislamiento total de estado. Cada actor posee sus datos privados y se comunica enviando mensajes a través de canales `mpsc`, eliminando las condiciones de carrera por diseño.

### 5.2 Model-View-Update (MVU / Elm Architecture)
Flujo unidireccional de datos ideal para aplicaciones gráficas (egui / iced):
$$\text{Model} \xrightarrow{\text{Render}} \text{View} \xrightarrow{\text{Event}} \text{Message} \xrightarrow{\text{Update}} \text{New Model}$$

### 5.3 Entity-Component-System (ECS / Data-Oriented Design)
Organización de memoria en arrays contiguos alineados con la jerarquía de memoria caché del procesador para máximo rendimiento.

---

## 💡 6. Los 5 Patrones Esenciales que Todo Rustacean Debe Dominar

Si debes priorizar el aprendizaje o la aplicación en proyectos como FerrisKey o tu libro, domina estos 5:

1. 🥇 **RAII / Drop Guard:** Gestión automática y determinista de memoria, cerrojos y recursos sin Garbage Collector.
2. 🥈 **Builder Pattern con Type-State:** Construcción ergonómica y validación de secuencias de llamadas en tiempo de compilación.
3. 🥉 **Newtype Pattern:** Seguridad de tipos en el dominio con costo de ejecución cero (*Zero-Cost*).
4. 🏅 **Option / Result con el operador `?`:** Manejo elegante, seguro y explícito de la ausencia de datos y errores.
5. 🎖️ **Deref Coercion y Préstamos Flexibles (`&str` / `&[T]` / `AsRef`):** APIs que evitan asignaciones de memoria y copias superfluas.
