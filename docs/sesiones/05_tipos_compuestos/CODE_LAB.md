# 🧪 Code Lab: 05 - Tipos Compuestos

## Ejercicio 01: Arreglos e Inicialización por Repetición
```rust
fn main() {
    let mut buffer: [u8; 8] = [0; 8];
    buffer[0] = 0xFF;
    buffer[1] = 0xFE;

    println!("Buffer de memoria en Stack: {:?}", buffer);
    println!("Tamaño total del buffer: {} bytes", buffer.len());
}
```

---

## Ejercicio 02: Tuplas y Desestructuración de Retornos
```rust
fn calcular_estadisticas(numeros: &[i32]) -> (i32, i32, usize) {
    let mut min = numeros[0];
    let mut max = numeros[0];
    
    for &val in numeros {
        if val < min { min = val; }
        if val > max { max = val; }
    }
    
    (min, max, numeros.len())
}

fn main() {
    let datos = [12, 45, 2, 89, 34];
    let (minimo, maximo, cantidad) = calcular_estadisticas(&datos);

    println!("Min: {} | Max: {} | Elementos: {}", minimo, maximo, cantidad);
}
```

---

## Ejercicio 03: Vistas Dinámicas con Slices (`&[T]`)
```rust
fn imprimir_slice(etiqueta: &str, slice: &[i32]) {
    println!("{}: {:?}", etiqueta, slice);
}

fn main() {
    let arreglo = [10, 20, 30, 40, 50, 60];
    
    imprimir_slice("Arreglo completo", &arreglo);
    imprimir_slice("Sub-slice [1..4]", &arreglo[1..4]); // [20, 30, 40]
}
```
