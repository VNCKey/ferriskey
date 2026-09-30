# 🧪 Code Lab: Sesión 16 - Punteros Inteligentes (Smart Pointers)

---

## 🎯 Reto 1: Árbol Binario con `Box<T>`

### Enunciado
Implementa un árbol binario donde cada nodo posea punteros a sus hijos izquierdo y derecho utilizando `Option<Box<Nodo<T>>>`.

```rust
struct Nodo<T> {
    valor: T,
    izq: Option<Box<Nodo<T>>>,
    der: Option<Box<Nodo<T>>>,
}

impl<T> Nodo<T> {
    fn new(valor: T) -> Self {
        Self { valor, izq: None, der: None }
    }
}

fn main() {
    let mut raiz = Nodo::new("Raíz");
    raiz.izq = Some(Box::new(Nodo::new("Hijo Izquierdo")));
    raiz.der = Some(Box::new(Nodo::new("Hijo Derecho")));
    println!("Nodo raíz: {}", raiz.valor);
}
```

---

## 🎯 Reto 2: Contador de Accesos Compartido con `Rc<RefCell<T>>`

### Enunciado
Crea un sistema donde dos objetos `Observador` compartan una referencia a un historial mutable usando `Rc<RefCell<Vec<String>>>`.

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Logger {
    historial: Rc<RefCell<Vec<String>>>,
}

impl Logger {
    fn log(&self, msg: &str) {
        self.historial.borrow_mut().push(msg.to_string());
    }
}

fn main() {
    let historial = Rc::new(RefCell::new(Vec::new()));
    let logger1 = Logger { historial: Rc::clone(&historial) };
    let logger2 = Logger { historial: Rc::clone(&historial) };

    logger1.log("Inicio de sesión");
    logger2.log("Acceso a base de datos");

    println!("Total logs: {:?}", historial.borrow());
}
```
