# 📖 Teoría Exhaustiva: Sesión 17 - Concurrencia y Multihilo (*Fearless Concurrency*)

---

## 1. Concurrencia sin Miedo en Rust

El sistema de tipos y las reglas de propiedad de Rust impiden las **condiciones de carrera (*Data Races*)** en tiempo de compilación. En Rust, dos hilos no pueden acceder simultáneamente al mismo dato en memoria si al menos uno intenta escribir, a menos que esté protegido por un mecanismo de sincronización seguro.

---

## 2. Hilos con `std::thread::spawn` y `move`

Para que un nuevo hilo pueda utilizar variables de su entorno, debe tomar posesión de ellas mediante `move`:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let vector = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector en el hilo: {:?}", vector);
        thread::sleep(Duration::from_millis(50));
        42
    });

    let resultado = handle.join().unwrap();
    println!("El hilo retornó: {}", resultado);
}
```

---

## 3. Canales de Mensajes (`std::sync::mpsc`)

La filosofía de Rust sigue el principio: *"No te comuniques compartiendo memoria; en su lugar, comparte memoria comunicándote"*.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // Permite múltiples productores

    thread::spawn(move || {
        tx1.send("Mensaje desde hilo 1").unwrap();
    });

    thread::spawn(move || {
        tx.send("Mensaje desde hilo 2").unwrap();
    });

    for recibido in rx {
        println!("Recibido: {}", recibido);
    }
}
```

---

## 4. Estado Compartido con `Arc<Mutex<T>>`

Para modificar un valor compartido entre 10 hilos en paralelo:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let contador_clon = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            let mut num = contador_clon.lock().unwrap();
            *num += 1; // MutexGuard se libera automáticamente al salir del scope
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Resultado final: {}", *contador.lock().unwrap()); // 10
}
```

---

## 5. Traits Marcadores: `Send` y `Sync`

* **`Send`**: Indica que el tipo puede transferirse a través de los límites de un hilo (`move`). Casi todos los tipos de Rust son `Send` (excepto `Rc<T>` y punteros crudos `*const T`).
* **`Sync`**: Indica que es seguro que múltiples hilos accedan al tipo mediante referencias inmutables (`&T`). Un tipo `T` es `Sync` si y solo si `&T` es `Send`.
