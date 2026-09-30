# 📖 Teoría Exhaustiva: Sesión 18 - Asincronía (Async / Await con Tokio)

---

## 1. El Modelo de Concurrencia Asíncrona en Rust

A diferencia de lenguajes como JavaScript o Go, **Rust no incluye un runtime asíncrono en su biblioteca estándar**. Rust provee únicamente la sintaxis (`async`/`await`) y el trait fundamental `Future`. La ejecución real se delega a un *Runtime* de alto rendimiento como **Tokio**.

---

## 2. Los *Futures* son Perezosos (*Zero-Cost Pull Model*)

En Rust, un bloque o función `async` **no ejecuta una sola instrucción hasta que se le aplica `.await`**:

```rust
async fn obtener_datos() -> String {
    // Simula una petición I/O
    String::from("Respuesta de servidor")
}

#[tokio::main]
async fn main() {
    let futuro = obtener_datos(); // No hace NADA todavía
    let resultado = futuro.await; // Aquí se ejecuta y espera el resultado
    println!("{}", resultado);
}
```

---

## 3. Tareas Concurrentes con `tokio::spawn`

Las tareas de Tokio son hilos verdes (*Green Threads*) extremadamente ligeros. Puedes ejecutar cientos de miles de tareas simultáneas consumiendo solo unos pocos megabytes de memoria:

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "Tarea 1 completada"
    });

    let handle2 = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        "Tarea 2 completada"
    });

    let (res1, res2) = tokio::join!(handle1, handle2);
    println!("Resultados: {}, {}", res1.unwrap(), res2.unwrap());
}
```

---

## 4. Canales Asíncronos y la Macro `tokio::select!`

`tokio::select!` permite esperar múltiples eventos asíncronos y responder al primero que se complete, cancelando los demás:

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx.send("Datos recibidos").await.unwrap();
    });

    tokio::select! {
        Some(msg) = rx.recv() => {
            println!("Mensaje: {}", msg);
        }
        _ = sleep(Duration::from_millis(200)) => {
            println!("Timeout alcanzado!");
        }
    }
}
```
