# 📖 Teoría: Sesión 07 - Closures (Funciones Anónimas)

---

## 1. ¿Qué es un Closure en Rust?

Un **Closure** es una función anónima que puede almacenarse en una variable, pasarse como argumento a otras funciones y, a diferencia de las funciones regulares declaradas con `fn`, tiene la capacidad única de **capturar variables del entorno léxico** donde fue definido.

```rust
let factor = 2;
// El closure captura `factor` desde su entorno exterior:
let duplicar = |numero: i32| numero * factor;

println!("Resultado: {}", duplicar(5)); // 10
```

---

## 2. Inferencia de Tipos en Closures

A diferencia de las funciones `fn` que obligan a declarar el tipo de cada parámetro y del retorno, los closures permiten omitir las anotaciones de tipo si el compilador puede inferirlos a partir del contexto:

```rust
let sumar_uno = |x| x + 1;

let n = sumar_uno(5); // El compilador infiere que x es i32
// sumar_uno(5.5);    // ❌ ERROR: x ya quedó fijado como i32 en la primera invocación
```

---

## 3. Los Tres Modos de Captura y los Traits `Fn`, `FnMut` y `FnOnce`

Rust genera de forma transparente una estructura anónima entre bastidores para almacenar las variables capturadas. Dependiendo de cómo el cuerpo del closure utiliza esas variables, implementará automáticamente uno o más de los siguientes tres traits:

| Trait | Modo de Captura | Comportamiento | Veces que puede ejecutarse |
|---|---|---|---|
| **`Fn`** | Referencia Inmutable (`&T`) | Solo lectura. No modifica el entorno. | Ilimitadas veces (múltiples hilos concurrentes). |
| **`FnMut`** | Referencia Mutable (`&mut T`) | Modifica las variables capturadas. | Múltiples veces en serie. |
| **`FnOnce`** | Movimiento de Propiedad (`T`) | Consume las variables capturadas. | **Exactamente UNA vez**. |

### Ejemplo: Captura Mutable (`FnMut`)
```rust
let mut contador = 0;
let mut incrementar = || {
    contador += 1;
    println!("Contador: {contador}");
};

incrementar(); // 1
incrementar(); // 2
```

---

## 4. La Cláusula `move`

Por defecto, Rust intenta capturar variables por referencia inmutable o mutable para minimizar costes de copia. Sin embargo, cuando el closure necesita sobrevivir más allá del ámbito de la función actual (por ejemplo, al pasarlo a un nuevo hilo con `std::thread::spawn`), las referencias provocarían un error de *dangling pointer*.

La palabra clave `move` fuerza al closure a tomar la **propiedad completa** (*Ownership*) de todas las variables que utiliza:

```rust
use std::thread;

let lista = vec![1, 2, 3];

// `move` transfiere la propiedad de `lista` al hilo secundario:
let manejador = thread::spawn(move || {
    println!("Lista en el hilo: {:?}", lista);
});

manejador.join().unwrap();
// println!("{:?}", lista); // ❌ ERROR: `lista` fue movida al closure
```
