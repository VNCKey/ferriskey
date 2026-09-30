# Patrones de Diseño: De POO Clásica (GoF / Refactoring Guru) a Rust Idiomático

Este documento mapea los **22 patrones de diseño clásicos de la Banda de los Cuatro (GoF / Refactoring Guru)** y su traducción al modelo de programación de **Rust**.

En Rust no existe herencia de clases (`extends`) ni punteros nulos (`null`). El lenguaje favorece la **composición sobre la herencia**, los **tipos algebraicos (`enum`)**, los **`traits`** y el sistema de **propiedad y préstamos (`ownership & borrowing`)**, logrando que muchos patrones se simplifiquen, cambien de nombre o sean absorbidos directamente por el compilador como abstracciones de costo cero (*Zero-Cost Abstractions*).

> 💡 **Documento complementario:** Para consultar los patrones exclusivos nacidos en el ecosistema de Rust (como *Newtype*, *RAII/Drop*, *ZST & PhantomData*, *Actor Pattern*, *ECS*, *MVU*, *ROP*, etc.), consulta [**`docs/patterns/RUST_IDIOMATIC_PATTERNS.md`**](file:///home/alek/VNC/repos/egui_vnc/docs/patterns/RUST_IDIOMATIC_PATTERNS.md).

---

## 📊 Tabla Resumen de Mapeo de Patrones (22 Patrones)

| N° | Patrón Clásico (Guru/GoF) | Categoría | ¿Mantiene el nombre? | Nombre / Equivalente en Rust | Mecanismo Clave en Rust |
| :---: | :--- | :--- | :---: | :--- | :--- |
| **1** | **Builder** | Creacional | ✅ Sí | **Builder / Type-State Builder** | `struct Builder` con métodos encadenados o verificación en compilación con genéricos |
| **2** | **Factory Method** | Creacional | 🔄 Cambia | **Constructores Asociados / Trait `From`** | `Tipo::new()`, `Tipo::from()` y traits `From<T>` / `Into<T>` |
| **3** | **Abstract Factory** | Creacional | 🔄 Cambia | **Factory Traits con Genéricos o `dyn Trait`** | `trait GUIFactory { fn crear_boton(&self) -> impl Boton; }` |
| **4** | **Prototype** | Creacional | 📦 Absorbido | **Trait `Clone` / `Copy`** | `#[derive(Clone)]` y método `.clone()` |
| **5** | **Singleton** | Creacional | ⚠️ Anti-patrón | **`std::sync::OnceLock<T>` / `lazy_static`** | Inicialización perezosa segura en concurrencia (se evita estado global mutable) |
| **6** | **Adapter** | Estructural | 🔄 Cambia | **Newtype Pattern / Traits de Conversión** | `struct Adaptador(TipoExterno);` o `impl Trait for TipoExterno` |
| **7** | **Bridge** | Estructural | 🔄 Cambia | **Composición + Trait Bounds Genéricos** | `struct Abstraccion<T: Implementor> { impl: T }` |
| **8** | **Composite** | Estructural | 🔄 Cambia | **Enums Recursivos con `Box<T>`** | `enum Nodo { Hoja(Dato), Rama(Vec<Box<Nodo>>) }` + `match` |
| **9** | **Decorator** | Estructural | 🔄 Cambia | **Extension Traits / Wrapper Types** | `trait ExtensionTrait { ... } impl<T: Base> ExtensionTrait for T` |
| **10** | **Facade** | Estructural | ✅ Sí | **Módulos Públicos y Re-exportaciones (`pub use`)** | `mod interno; pub use interno::ServicioFacil;` en `lib.rs` |
| **11** | **Flyweight** | Estructural | 🔄 Cambia | **Punteros Compartidos (`Arc<T>` / `Rc<T>`) o Interning** | `Arc<str>`, `Arc<DatosInmutables>` o librerías de interning |
| **12** | **Proxy** | Estructural | 🔄 Cambia | **Smart Pointers con Trait `Deref` / `DerefMut`** | `Deref` para delegar automáticamente al objeto subyacente |
| **13** | **Chain of Responsibility** | Comportamental | 🔄 Cambia | **Pipelines con `Result` / Iterador de Closures** | Encadenamiento de `.and_then()`, combinador `?` o `Vec<Box<dyn Handler>>` |
| **14** | **Command** | Comportamental | 🔄 Cambia | **Action Enums / Closures (`FnOnce` / `FnMut`)** | `enum Command { Mover(i32, i32), Salir }` o `Box<dyn FnOnce()>` |
| **15** | **Iterator** | Comportamental | 📦 Absorbido | **Trait `Iterator` Estándar** | `impl Iterator for MiColeccion { fn next(&mut self) -> Option<Item>; }` |
| **16** | **Mediator** | Comportamental | 🔄 Cambia | **Modelo de Actores / Canales (`mpsc` / `broadcast`)** | Paso de mensajes por canales seguros evitando referencias cruzadas mutables |
| **17** | **Memento** | Comportamental | 🔄 Cambia | **Snapshots Inmutables / Serialización `serde`** | `#[derive(Serialize, Deserialize, Clone)]` guardando copias compactas de estado |
| **18** | **Observer** | Comportamental | 🔄 Cambia | **Canales de Eventos / Callbacks (`Fn`)** | Canales `mpsc::Sender<Evento>` o `Vec<Box<dyn Fn(&Evento)>>` |
| **19** | **State** | Comportamental | 🔄 Cambia | **Type-State Pattern / Enum State Machine** | Verificación estática en compilación (Zero-Cost) o `enum Estado` + `match` |
| **20** | **Strategy** | Comportamental | 🔄 Cambia | **Traits Genéricos (`impl Trait`) / Closures** | `fn ejecutar<S: Strategy>(s: S)` o pasar closures directas |
| **21** | **Template Method** | Comportamental | 🔄 Cambia | **Default Trait Methods** | `trait Algoritmo { fn paso1(&self); fn ejecutar(&self) { self.paso1(); } }` |
| **22** | **Visitor** | Comportamental | 🔄 Cambia | **Pattern Matching Exhaustivo sobre `enum`** | Expresiones `match` que obligan a contemplar todas las variantes sin fallos |
| *(Extra)* | **Null Object** | Comportamental | 📦 Absorbido | **Tipo Estándar `Option<T>`** | `Some(valor)` o `None` integrado en el lenguaje (cero `NullPointerException`) |

---

## 🛠️ Detalle e Implementación por Categoría

### 1. Patrones Creacionales

#### 1.1 Builder (Se mantiene el nombre)
* **En POO:** Construcción compleja de un objeto paso a paso usando una clase auxiliar.
* **En Rust:** Se usa idénticamente, pero con dos variantes:
  1. *Builder Básico:* Métodos encadenados que consumen o mutan `self`.
  2. *Type-State Builder:* Cada llamada de configuración transforma el tipo del struct (`ConfigIncompleta` $\rightarrow$ `ConfigLista`), impidiendo llamar a `.build()` si falta un campo obligatorio.

```rust
pub struct ServidorBuilder {
    puerto: u16,
    hilos: usize,
}

impl ServidorBuilder {
    pub fn new() -> Self {
        Self { puerto: 8080, hilos: 4 }
    }
    pub fn puerto(mut self, p: u16) -> Self {
        self.puerto = p;
        self
    }
    pub fn build(self) -> Servidor {
        Servidor { puerto: self.puerto, hilos: self.hilos }
    }
}
```

#### 1.2 Factory Method $\rightarrow$ Constructores Asociados & Trait `From`
* **En POO:** Crear subclases con un método fábrica que instancie objetos concretos.
* **En Rust:** Funciones asociadas estáticas y el trait estándar `From` / `Into`.

```rust
struct Coordenada { x: f64, y: f64 }

impl Coordenada {
    // Constructor estándar
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }
    // Constructor con nombre de dominio (Factory method)
    pub fn polar(radio: f64, angulo: f64) -> Self {
        Self { x: radio * angulo.cos(), y: radio * angulo.sin() }
    }
}

// Conversión idiomática (Factory via Trait)
impl From<(f64, f64)> for Coordenada {
    fn from((x, y): (f64, f64)) -> Self { Self::new(x, y) }
}
```

#### 1.3 Abstract Factory $\rightarrow$ Factory Traits con Genéricos
* **En POO:** Familias de fábricas con interfaces complejas.
* **En Rust:** Un `trait` cuyos métodos devuelven implementaciones de otros traits o tipos concretos asociados.

```rust
trait Boton { fn render(&self); }
trait FabricaGUI {
    type B: Boton;
    fn crear_boton(&self) -> Self::B;
}
```

#### 1.4 Prototype $\rightarrow$ Trait `Clone`
* **En POO:** Interfaz `Prototype` con método `clone()`.
* **En Rust:** Es parte del prelude estándar. Basta con añadir `#[derive(Clone)]`.

```rust
#[derive(Clone, Debug)]
pub struct Celula {
    pub adn: String,
    pub energia: u32,
}
```

#### 1.5 Singleton $\rightarrow$ `std::sync::OnceLock<T>`
* **En POO:** Constructor privado con una instancia estática mutable.
* **En Rust:** El estado global mutable se considera inseguro y anti-patrón. Cuando se necesita una única configuración global inmutable o sincronizada:

```rust
use std::sync::OnceLock;

static CONFIG: OnceLock<ConfiguracionGlobal> = OnceLock::new();

pub fn obtener_config() -> &'static ConfiguracionGlobal {
    CONFIG.get_or_init(|| ConfiguracionGlobal::cargar_desde_disco())
}
```

---

### 2. Patrones Estructurales

#### 2.1 Adapter $\rightarrow$ Newtype Pattern
* **En POO:** Clase adaptadora que hereda o envuelve la clase antigua para exponer la nueva interfaz.
* **En Rust:** Se usa una tupla-struct de un solo elemento (*Newtype Pattern*) o se implementa el trait deseado para el tipo externo.

```rust
struct Segundos(u64);
struct Milisegundos(u64);

impl From<Segundos> for Milisegundos {
    fn from(s: Segundos) -> Self { Milisegundos(s.0 * 1000) }
}
```

#### 2.2 Bridge $\rightarrow$ Composición + Trait Bounds
* **En POO:** Jerarquía de abstracción separada de la jerarquía de implementación mediante punteros base.
* **En Rust:** Un struct genérico parametrizado por un trait.

```rust
trait Dispositivo { fn encender(&mut self); }
struct ControlRemoto<D: Dispositivo> {
    dispositivo: D,
}
```

#### 2.3 Composite $\rightarrow$ Enums Recursivos con `Box<T>`
* **En POO:** Árboles con clases base `Component`, `Leaf` y `Composite`.
* **En Rust:** Un `enum` recursivo envuelto en `Box<T>` para conocer su tamaño en compilación.

```rust
enum Expresion {
    Numero(f64),
    Suma(Box<Expresion>, Box<Expresion>),
    Multiplicacion(Box<Expresion>, Box<Expresion>),
}

fn evaluar(exp: &Expresion) -> f64 {
    match exp {
        Expresion::Numero(n) => *n,
        Expresion::Suma(a, b) => evaluar(a) + evaluar(b),
        Expresion::Multiplicacion(a, b) => evaluar(a) * evaluar(b),
    }
}
```

#### 2.4 Decorator $\rightarrow$ Extension Traits / Struct Wrappers
* **En POO:** Subclase decoradora que envuelve al componente original y añade funcionalidad.
* **En Rust:**
  1. *Extension Trait:* Añade métodos a cualquier tipo existente sin herencia.
  2. *Wrapper Struct:* Envuelve un tipo y re-implementa el trait delegando y modificando llamadas.

```rust
pub trait StringExt {
    fn a_titulo(&self) -> String;
}

impl StringExt for str {
    fn a_titulo(&self) -> String {
        // Lógica de capitalización...
        self.to_uppercase()
    }
}
```

#### 2.5 Facade $\rightarrow$ Sistema de Módulos y `pub use`
* **En POO:** Clase fachada que simplifica subsistemas complejos.
* **En Rust:** Los módulos `mod` organizan los subsistemas privados, y `pub use` expone sólo la interfaz limpia en la raíz del crate/módulo.

```rust
// lib.rs
mod subsistema_audio;
mod subsistema_video;
mod subsistema_red;

// Fachada pública limpia:
pub use subsistema_red::ClienteSimple;
```

#### 2.6 Flyweight $\rightarrow$ `Arc<T>` / `Rc<T>` o Interning
* **En POO:** Fábrica de objetos compartidos para ahorrar RAM.
* **En Rust:** Punteros contadores de referencias inmutables (`Arc<T>` para multihilo, `Rc<T>` para monohilo) o tablas de interning de strings.

```rust
use std::sync::Arc;

struct Sprite {
    textura_pesada: Arc<Vec<u8>>, // Memoria compartida sin duplicar
    pos_x: i32,
    pos_y: i32,
}
```

#### 2.7 Proxy $\rightarrow$ Trait `Deref` / `DerefMut`
* **En POO:** Objeto sustituto que intercepta llamadas antes de enviarlas al objeto real.
* **En Rust:** Implementar `Deref` permite que el struct actúe de forma transparente como el tipo subyacente.

```rust
use std::ops::Deref;

struct ProxyConLog<T> {
    interior: T,
}

impl<T> Deref for ProxyConLog<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        println!("[LOG]: Accediendo a recurso interior");
        &self.interior
    }
}
```

---

### 3. Patrones Comportamentales

#### 3.1 Chain of Responsibility $\rightarrow$ Pipelines Funcionales o `Result`
* **En POO:** Lista enlazada de manejadores con método `setNext()`.
* **En Rust:** Encadenamiento de `Option`/`Result` con combinadores (`and_then`, `or_else`) o un vector de closures.

```rust
fn procesar_peticion(req: Peticion) -> Result<Respuesta, Error> {
    validar_auth(req)
        .and_then(validar_permisos)
        .and_then(ejecutar_logica)
}
```

#### 3.2 Command $\rightarrow$ Action Enums o Closures
* **En POO:** Interfaz `Command` con método `execute()`.
* **En Rust:**
  1. *Action Enum:* Ideal para colas de eventos reproducibles, deshacer/rehacer y serialización.
  2. *Closures:* `Box<dyn FnOnce()>` cuando solo se requiere ejecución diferida.

```rust
enum Comando {
    EscribirTexto(String),
    BorrarTexto { cantidad: usize },
    GuardarDocumento,
}

fn ejecutar_comando(cmd: Comando, doc: &mut Documento) {
    match cmd {
        Comando::EscribirTexto(t) => doc.contenido.push_str(&t),
        Comando::BorrarTexto { cantidad } => { doc.contenido.truncate(doc.contenido.len() - cantidad); },
        Comando::GuardarDocumento => doc.guardar(),
    }
}
```

#### 3.3 Iterator $\rightarrow$ Trait `Iterator` Nativo
* **En POO:** Interfaz `Iterator` con `hasNext()` y `next()`.
* **En Rust:** El trait nativo `Iterator` del lenguaje. Al implementar solo `fn next(&mut self) -> Option<Item>`, obtienes automáticamente métodos como `map`, `filter`, `fold`, `zip`, `collect`, etc.

```rust
struct Contador { actual: u32, maximo: u32 }

impl Iterator for Contador {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.actual < self.maximo {
            self.actual += 1;
            Some(self.actual)
        } else {
            None
        }
    }
}
```

#### 3.4 Mediator $\rightarrow$ Canales / Arquitectura de Actores
* **En POO:** Objeto central que conoce a todos los componentes para coordinar su comunicación.
* **En Rust:** Canales (`std::sync::mpsc` o `tokio::sync::mpsc`). Los componentes envían mensajes al canal sin necesidad de conocerse entre sí ni compartir referencias mutables.

```rust
use std::sync::mpsc::{channel, Sender};

enum MensajeMediador {
    UsuarioEscribio(String),
    AlertaSistema(String),
}
```

#### 3.5 Memento $\rightarrow$ Snapshots con `Clone` o `serde`
* **En POO:** Objeto `Memento` opaco para restaurar estados internos.
* **En Rust:** `#[derive(Clone, Serialize, Deserialize)]`. El estado se almacena como una estructura de datos inmutable en un historial `Vec<Estado>`.

#### 3.6 Observer $\rightarrow$ Canales / Event Loop / Vector de Callbacks
* **En POO:** Interfaz `Subject` con `attach(Observer)` y `notify()`.
* **En Rust:** Canales de broadcast (`tokio::sync::broadcast`) o listas de listeners basadas en closures:

```rust
struct EventBus {
    listeners: Vec<Box<dyn Fn(&str)>>,
}

impl EventBus {
    pub fn suscribir(&mut self, callback: impl Fn(&str) + 'static) {
        self.listeners.push(Box::new(callback));
    }
    pub fn emitir(&self, evento: &str) {
        for listener in &self.listeners {
            listener(evento);
        }
    }
}
```

#### 3.7 State $\rightarrow$ Type-State Pattern & Enum State Machine
* **En POO:** Interfaz `State` con clases de estado concreto y delegación dinámica.
* **En Rust:**
  1. *Type-State Pattern:* Estados como tipos distintos con transiciones de consumo (`self`). ¡Los estados inválidos no compilan!
  2. *Enum State Machine:* Un `enum` evaluado exhaustivamente con `match`.

```rust
// Ejemplo Type-State (Compilación segura)
struct Desconectado;
struct Conectado { ip: String }

struct Conexion<E> { estado: E }

impl Conexion<Desconectado> {
    pub fn conectar(self, ip: String) -> Conexion<Conectado> {
        Conexion { estado: Conectado { ip } }
    }
}

impl Conexion<Conectado> {
    pub fn enviar_datos(&self, datos: &[u8]) { /* ... */ }
}
```

#### 3.8 Strategy $\rightarrow$ Traits Genéricos / Closures
* **En POO:** Interfaz `Strategy` inyectada en el constructor del contexto.
* **En Rust:** Se usa polimorfismo estático (*Monomorphization*, cero costo) con genéricos `T: Estrategia` o closures directas.

```rust
trait CompresionStrategy {
    fn comprimir(&self, datos: &[u8]) -> Vec<u8>;
}

struct Archivador<S: CompresionStrategy> {
    estrategia: S,
}
```

#### 3.9 Template Method $\rightarrow$ Default Trait Methods
* **En POO:** Clase abstracta con método plantilla que llama a métodos abstractos sobreescritos en subclases.
* **En Rust:** Un `trait` con métodos con implementación por defecto que invocan métodos requeridos.

```rust
trait ProcesadorMinero {
    fn extraer_datos(&self) -> Vec<u8>;
    fn analizar(&self, datos: &[u8]);

    // Método plantilla (Template Method)
    fn procesar(&self) {
        let datos = self.extraer_datos();
        self.analizar(&datos);
    }
}
```

#### 3.10 Visitor $\rightarrow$ Pattern Matching Exhaustivo
* **En POO:** Doble despacho con métodos `accept(Visitor)` y `visit(Element)`.
* **En Rust:** Innecesario. Los `enum` junto con `match` proporcionan un recorrido exhaustivo y tipado sin necesidad de alterar las estructuras de datos.

```rust
enum DocumentoElemento {
    Parrafo(String),
    Imagen { url: String, ancho: u32 },
}

fn exportar_a_html(elem: &DocumentoElemento) -> String {
    match elem {
        DocumentoElemento::Parrafo(p) => format!("<p>{}</p>", p),
        DocumentoElemento::Imagen { url, ancho } => format!("<img src='{}' width='{}' />", url, ancho),
    }
}
```

---

## 🎯 Conclusión: Los 3 Pilares del Diseño en Rust

1. **Composición y Traits en vez de Herencia:** La modularidad se consigue implementando capacidades (`traits`) sobre estructuras de datos claras.
2. **Estados en Tiempo de Compilación (*Type-State*):** Rust permite mover la detección de bugs de lógica del *tiempo de ejecución* al *tiempo de compilación*.
3. **Paso de Mensajes y Tipos Algebraicos:** La concurrencia y la gestión de eventos se resuelven con canales y enums exhaustivos, eliminando la necesidad de jerarquías complejas de punteros compartidos.
