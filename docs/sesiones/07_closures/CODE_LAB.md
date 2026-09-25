# 🧪 Code Lab: Sesión 07 - Funciones y Closures

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/funciones/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/funciones/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Terminal embebida y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab programarás funciones anónimas (*Closures*), analizando cómo capturan variables de su entorno circundante por referencia o por movimiento con `move`.

---

## 🏁 Ruta de Ejercicios Prácticos

### 📌 Ejercicio 1: Closures Básicos e Inferencia
- **Código Base**:
  ```rust
  fn main() {
      // Closure simple con inferencia de tipos
      let elevar_cuadrado = |x: i32| x * x;
      println!("5 al cuadrado: {}", elevar_cuadrado(5));

      // Captura de variable inmutable del entorno exterior (Trait Fn)
      let multiplicador = 10;
      let multiplicar = |num| num * multiplicador;
      println!("3 x 10 = {}", multiplicar(3));
  }
  ```
- **Tu Tarea**:
  1. Diseña un closure que reciba dos palabras y las concatene separadas por un guion.

---

### 📌 Ejercicio 2: Captura Mutable con `FnMut`
- **Código Base**:
  ```rust
  fn main() {
      let mut total_acumulado = 0;

      // El closure debe ser `mut` para permitir la mutación interna
      let mut registrar_visita = || {
          total_acumulado += 1;
          println!("Visitas registradas: {total_acumulado}");
      };

      registrar_visita();
      registrar_visita();
      registrar_visita();
  }
  ```
- **Tu Tarea**:
  1. Comprueba qué error arroja el compilador si intentas leer o imprimir `total_acumulado` mientras el closure mutable sigue en uso activo.

---

### 📌 Ejercicio 3: Transferencia de Propiedad con `move`
- **Código Base**:
  ```rust
  fn main() {
      let saludo = String::from("Hola Rustáceo");

      // `move` transfiere `saludo` al interior del closure
      let consumir = move || {
          println!("Mensaje consumido: {saludo}");
      };

      consumir();
      // println!("{saludo}"); // ❌ ERROR: valor movido
  }
  ```
- **Tu Tarea**:
  1. Crea un vector de números, muévelo a un closure con `move` y calcula su suma total.
