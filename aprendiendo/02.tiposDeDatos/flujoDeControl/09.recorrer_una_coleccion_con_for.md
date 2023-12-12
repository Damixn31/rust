# Recorrer una coleccion con for

- Puedes elegir usar la construcción while para hacer un bucle sobre los elementos de una colección, como un array. Por ejemplo, el bucle del Listado imprime cada elemento del array a.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

- Aquí, el código cuenta hacia arriba a través de los elementos del array. Comienza en el índice 0, y luego hace un bucle hasta que alcanza el índice final en el array (es decir, cuando índice < 5 ya no es cierto). Al ejecutar este código se imprimirán todos los elementos de el array:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50

```

- Los cinco valores del array aparecen en el terminal, como era de esperar. Aunque el índice alcanzará un valor de 5 en algún momento, el bucle deja de ejecutarse antes de intentar obtener un sexto valor del array.

- Sin embargo, este enfoque es propenso a errores; podríamos hacer que el programa entrara en pánico si el valor de index o la condición de prueba es incorrecta. Por ejemplo, si cambiamos la definición del array a para que tenga cuatro elementos pero olvidamos actualizar la condición a while index < 4, el código entraría en pánico. También es lento, porque el compilador añade código en tiempo de ejecución para realizar la comprobación condicional de si el índice está dentro de los límites del array en cada iteración a través del bucle.

- Como alternativa más concisa, puede utilizar un bucle for y ejecutar algún código para cada elemento de una colección.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

- Cuando ejecutemos este código, veremos la misma salida Más importante aún, ahora hemos incrementado la seguridad del código y eliminado la posibilidad de errores que podrían resultar de ir más allá del final del arreglo o no ir lo suficientemente lejos y omitir algunos elementos.

- Usando el bucle for, no necesitarás recordar cambiar ningún otro código si cambias el número de valores en el array

- La seguridad y concisión de los bucles for los convierten en la construcción de bucle más utilizada en Rust. Incluso en situaciones en las que quiera ejecutar código un cierto número de veces la mayoría de los Rustceanos usarían un bucle for. La forma de hacerlo sería usar un Range, proporcionado por la librería estándar, que genera todos los números en secuencia empezando por un número y terminando antes de otro número.

- Así es como se vería la cuenta regresiva usando un bucle for y otro método del que aún no hemos hablado, rev, para invertir el rango:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```
