# 🧪 Code Lab: Sesión 08 - Iteradores y Combinadores

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/iteradores/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/iteradores/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Terminal embebida y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab construirás pipelines de procesamiento funcional de datos sobre colecciones. Experimentarás la diferencia entre `.iter()`, `.iter_mut()` y `.into_iter()`, y combinarás adaptadores perezosos con consumidores eficientes.

---

## 🏁 Ruta de Ejercicios Prácticos

### 📌 Ejercicio 1: Modos de Iteración
- **Código Base**:
  ```rust
  fn main() {
      let mut numeros = vec![10, 20, 30];

      // 1. Préstamo inmutable (.iter)
      println!("Lectura con .iter():");
      for n in numeros.iter() {
          println!("Elemento: {n}");
      }

      // 2. Préstamo mutable (.iter_mut)
      for n in numeros.iter_mut() {
          *n += 5;
      }
      println!("Modificados: {:?}", numeros);

      // 3. Consumo por valor (.into_iter)
      let consumidos: Vec<i32> = numeros.into_iter().map(|x| x * 2).collect();
      println!("Consumidos y transformados: {:?}", consumidos);
      // numeros ya no está disponible
  }
  ```
- **Tu Tarea**:
  1. Verifica que después de `into_iter()`, intentar imprimir `numeros` causa un error de compilación por *use of moved value*.

---

### 📌 Ejercicio 2: Pipeline de Adaptadores y Consumidores
- **Código Base**:
  ```rust
  fn main() {
      let puntuaciones = vec![45, 82, 91, 33, 78, 100, 64];

      // Pipeline funcional: filtrar aprobados (>= 70), sumar bonificación (+5) y recolectar
      let aprobados_con_bono: Vec<i32> = puntuaciones
          .iter()
          .filter(|&&nota| nota >= 70)
          .map(|&nota| (nota + 5).min(100))
          .collect();

      println!("Notas aprobadas bonificadas: {:?}", aprobados_con_bono);

      // Agregación con sum() y fold()
      let suma_total: i32 = aprobados_con_bono.iter().sum();
      println!("Suma total: {suma_total}");
  }
  ```
- **Tu Tarea**:
  1. Utiliza `.take(3)` para seleccionar únicamente los tres primeros aprobados del vector.
  2. Utiliza `.enumerate()` para imprimir el número de ranking junto con cada nota.
