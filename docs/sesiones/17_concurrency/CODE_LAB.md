# 🧪 Code Lab: Sesión 17 - Concurrencia y Multihilo (*Fearless Concurrency*)

---

## 🎯 Reto 1: Procesamiento Paralelo con `std::thread` y Canales

### Enunciado
Crea un programa que divida una lista de 10 números en dos hilos, cada uno sume su mitad y envíe el subtotal a un hilo principal mediante un canal `mpsc`.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (tx, rx) = mpsc::channel();

    let mitad = numeros.len() / 2;
    let parte1 = numeros[..mitad].to_vec();
    let parte2 = numeros[mitad..].to_vec();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let suma: i32 = parte1.iter().sum();
        tx1.send(suma).unwrap();
    });

    thread::spawn(move || {
        let suma: i32 = parte2.iter().sum();
        tx.send(suma).unwrap();
    });

    let total = rx.recv().unwrap() + rx.recv().unwrap();
    println!("Suma total paralela: {}", total); // 55
}
```

---

## 🎯 Reto 2: Acumulador Seguro con `Arc<Mutex<Vec<String>>>`

### Enunciado
Implementa un colector de logs concurrentes donde 4 hilos agreguen eventos simultáneamente a un vector protegido.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let logs = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..4 {
        let logs_ref = Arc::clone(&logs);
        let h = thread::spawn(move || {
            let mut l = logs_ref.lock().unwrap();
            l.push(format!("Evento registrado por hilo #{}", i));
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Total de logs registrados: {}", logs.lock().unwrap().len());
}
```
