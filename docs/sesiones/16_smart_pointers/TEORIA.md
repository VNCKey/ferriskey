# 📖 Teoría Exhaustiva: Sesión 16 - Punteros Inteligentes (Smart Pointers)

---

## 1. ¿Qué es un Smart Pointer en Rust?

Un puntero inteligente es una estructura de datos que actúa como un puntero pero contiene metadatos y capacidades adicionales (como gestión automática de memoria mediante el trait `Drop` y acceso transparente mediante el trait `Deref`).

---

## 2. `Box<T>`: Alocación en Heap y Tipos Recursivos

`Box<T>` guarda un puntero de tamaño fijo en el *Stack* que apunta a los datos reales alojados en el *Heap*. Es imprescindible para crear tipos recursivos cuyo tamaño no se conoce en tiempo de compilación:

```rust
// Lista Enlazada (Cons List)
enum Lista {
    Cons(i32, Box<Lista>),
    Nil,
}

use Lista::{Cons, Nil};

fn main() {
    let lista = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

---

## 3. `Rc<T>` y `Arc<T>`: Propiedad Múltiple (Reference Counting)

Por defecto, en Rust un valor tiene exactamente un único propietario. Cuando múltiples partes de un programa necesitan leer el mismo dato simultáneamente sin clonar su contenido:

* **`Rc<T>` (Monohilo):** Incrementa y decrementa un contador no atómico. Muy rápido, pero no implementa `Send` (no se puede enviar entre hilos).
* **`Arc<T>` (Multihilo):** Utiliza operaciones atómicas en CPU para contar referencias de forma segura entre hilos.

```rust
use std::rc::Rc;

let texto_compartido = Rc::new(String::from("Datos de solo lectura"));
let usuario1 = Rc::clone(&texto_compartido); // Incrementa el contador (no clona el String)
let usuario2 = Rc::clone(&texto_compartido);

println!("Contador de referencias: {}", Rc::strong_count(&texto_compartido)); // 3
```

---

## 4. `RefCell<T>`: Mutabilidad Interior (*Interior Mutability*)

`RefCell<T>` permite mutar datos incluso cuando tienes una referencia inmutable (`&T`), trasladando las reglas del *Borrow Checker* del **tiempo de compilación** al **tiempo de ejecución**. Si violas las reglas (ej. dos préstamos mutables simultáneos), el programa entra en `panic` en lugar de fallar al compilar.

```rust
use std::cell::RefCell;

let dato = RefCell::new(42);

{
    let mut mut_ref = dato.borrow_mut();
    *mut_ref += 10;
} // mut_ref sale de ámbito y libera el préstamo mutable

println!("Nuevo valor: {}", *dato.borrow()); // 52
```

---

## 5. El Patrón Combo `Rc<RefCell<T>>` / `Arc<Mutex<T>>`

Para tener **múltiples propietarios y mutabilidad al mismo tiempo**:
* En **monohilo**: `Rc<RefCell<T>>`.
* En **multihilo**: `Arc<Mutex<T>>` o `Arc<RwLock<T>>`.
