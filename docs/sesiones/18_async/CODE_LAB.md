# 🧪 Code Lab: Sesión 18 - Asincronía (Async / Await con Tokio)

---

## 🎯 Reto 1: Descargador Concurrente con `join!`

### Enunciado
Simula la descarga de 3 recursos remotos con diferentes tiempos de espera ejecutados concurrentemente con `tokio::join!`.

```rust
use tokio::time::{sleep, Duration};

async fn descargar_recurso(id: u32, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("Recurso #{} descargado en {}ms", id, ms)
}

#[tokio::main]
async fn main() {
    let tarea1 = descargar_recurso(1, 150);
    let tarea2 = descargar_recurso(2, 50);
    let tarea3 = descargar_recurso(3, 100);

    let (r1, r2, r3) = tokio::join!(tarea1, tarea2, tarea3);
    println!("{}\n{}\n{}", r1, r2, r3);
}
```

---

## 🎯 Reto 2: Pipeline Productor-Consumidor con Canales Asíncronos

### Enunciado
Implementa un canal `tokio::sync::mpsc` donde un productor genere 5 eventos y un consumidor asíncrono los procese secuencialmente.

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(format!("Mensaje asíncrono #{}", i)).await.unwrap();
        }
    });

    while let Some(mensaje) = rx.recv().await {
        println!("Procesado: {}", mensaje);
    }
}
```
