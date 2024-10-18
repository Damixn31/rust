fn main() {
    // Ejercicio 1: Conversión de tipos
    //Define una variable que inicialmente contenga un número como string, luego conviértelo a un número entero usando shadowing, y realiza una operación aritmética con el valor convertido.

    //Instrucciones:
    //1. Declara una variable num_str que contenga el string "50".
    //2. Usa shadowing para convertir num_str a un número entero.
    //3. Multiplica el valor convertido por 2 y almacénalo usando shadowing nuevamente.
    //4. Imprime el resultado.
    //Resultado esperado:
    //El número final debería ser 100.

    println!("----Ejercicio numero 1-----");
    let num_str = "50";
    let convert_num: i32 = num_str.parse().expect("no se pudo convertir");
    let result_multi = convert_num * 2;

    println!("el Resultado es: {}", result_multi);

    println!("---Ejercicio numero 2----");
    //Ejercicio 2: Operaciones matemáticas
    //Usa shadowing para realizar una serie de operaciones matemáticas sobre una misma variable.

    //Instrucciones:
    //1. Declara una variable x con el valor 10.
    //2. Usa shadowing para:
    // Sumar 5 a x.
    // Multiplicar el resultado por 2.
    // Restar 3 al valor final.
    //3. Imprime el valor final de x.
    //Resultado esperado:
    //El valor final de x debería ser 27.

    let x = 10;
    let x = x + 5;
    let x = x * 2;
    let x = x - 3;

    println!("El resultado final de x es : {}", x);

    println!("----Ejercicio 3-----");
    //    Ejercicio 3: Reutilización de nombres en bloques
    //Usa shadowing dentro de bloques para trabajar con diferentes valores de una misma variable.

    //Instrucciones:
    //1. Declara una variable total con valor 20.
    //2. Dentro de un bloque {}, usa shadowing para modificar total sumando 10.
    //3. Dentro de otro bloque {}, usa shadowing para dividir total por 2.
    //4. Imprime el valor de total tanto dentro como fuera de cada bloque.
    //Resultado esperado:
    //El valor dentro de los bloques será diferente al de fuera.

    let total = 20;
    println!("el resultado del total es: {}", total);
    {
	let total = total + 10;
	println!("el total de la suma es: {}", total);
    };

    {
	let total = total / 2;
	println!("el total de la division es: {}", total);
    };

    //Ejercicio 4: Limpiar datos
    //Imagina que tienes un string con caracteres adicionales al principio y al final, y quieres limpiarlo usando shadowing.

    //Instrucciones:
    //Declara una variable raw_data que contenga el string " 42 ".
    //Usa shadowing para:
    //1. Limpiar los espacios al principio y al final del string.
    //2. Convertir el string limpio en un número entero.
    //3. Imprime el número final.
    //Resultado esperado:
    //Deberías imprimir el número 42 sin espacios ni errores.
    println!("------Ejercicio 3------");
    let raw_data = " 42 ";
    let raw_data = raw_data.trim(); // trim() saca los espacios en blanco de ambos lados.
    let raw_data: i32 = raw_data.parse().expect("No se pudo convertir"); // parse() converierte a entero.

    println!("El resultado de es: {}", raw_data);

    //    Ejercicio 5: Shadowing y tipos de datos
    //Define una variable con un número decimal y luego conviértelo en un número entero usando shadowing.

    //Instrucciones:
    //1. Declara una variable price con el valor 19.99.
    //2. Usa shadowing para redondear el valor hacia abajo y convertirlo en un número entero.
    //3.Imprime el valor entero final de price.
    //Resultado esperado:
    //El número final debería ser 19.

    println!("----Ejercicio 4----");
    let prince: f32 = 19.99;
    let prince = prince.floor() as i32; // floor() redondea haca abajo.
    
    println!("El resultado final de prince es: {}", prince);

    //  Ejercicio 6: Manipulación de listas (Avanzado)
    //Crea una lista de números y usa shadowing para manipular el mismo nombre de la variable con diferentes estados.

    //Instrucciones:
    //Declara una variable numbers que contenga una lista de números [1, 2, 3, 4, 5].
    //Usa shadowing para:
    //1. Filtrar solo los números pares.
    //2. Sumar 1 a cada número.
    //3.Imprime la lista final de numbers.
    //Resultado esperado:
    //La lista final debería ser [3, 5].

    println!("---Ejercicio 6----");
    let numbers = vec![1,2,3,4,5];
    let numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).map(|x| x + 1).collect();
    
    println!("Los numeros pares son: {:?}", numbers);

    //    Ejercicio 7: Cálculo de descuento
    //Tienes el precio original de un producto y luego aplicas un descuento. Usa shadowing para calcular el precio final.

    //Instrucciones:
    //1. Declara una variable price con el valor 100.0 (un número decimal).
    //2. Usa shadowing para aplicar un descuento del 15%.
    //3. Imprime el precio final después del descuento.
    println!("----Ejercicio 7----");
    let prince2 = 100.0;
    let prince2 = prince2 * 0.85;
    println!("El resultado del descuenta es: {}", prince2);

    //  Ejercicio 8: Cambio de tipo de dato
    //Trabaja con una cadena de texto que representa un número y luego convierte esa cadena a un número entero.

    //Instrucciones:
    //1. Declara una variable num_str que contenga el string "123".
    //2. Usa shadowing para convertir ese string a un número entero.
    //3. Suma 10 al número y almacénalo usando shadowing.
    //Imprime el resultado final.
    println!("---Ejercicio 8---");
    let num_str = "123";
    let num_str: i32 = num_str.parse().expect("No se pudo convertir a entero");
    let num_str = num_str + 10;
    println!("El resultado es: {}", num_str);

    //    Ejercicio 9: Manipulación de strings
    //Haz algunas modificaciones a un string utilizando shadowing en varios pasos.

    //Instrucciones:
    //1. Declara una variable word con el valor "rustacean".
    //2. Usa shadowing para convertir la cadena a mayúsculas.
    //3. Usa shadowing nuevamente para contar el número de caracteres en el string.
    //Imprime el número de caracteres.
    println!("---Ejercicio 9---");
    let word = "rustacean";
    let word = word.to_uppercase();
    println!("convertido a mayuscula: {}", word);
    let word = String::from(word).len();
    println!("El string tiene: {} caracters", word);

    //Ejercicio 10: Suma de números pares
    //Filtra una lista de números enteros para quedarte solo con los números pares y luego suma esos números.

    //Instrucciones:
    //1. Declara una lista de números [1, 2, 3, 4, 5, 6].
    //2. Usa shadowing para filtrar solo los números pares.
    //3.Usa shadowing nuevamente para sumar los números pares.
    //4. Imprime el resultado final.

    let num = vec![1,2,3,4,5,6];
    let num: Vec<i32> = num.into_iter().filter(|&x| x % 2 == 0).collect();
    let num: i32 = num.into_iter().sum();
    println!("Los numeros pares son: {:?}", num);

    //    Ejercicio 11: Cálculo del área de un círculo
    //Calcula el área de un círculo dado un radio. Usa shadowing para realizar las transformaciones.

    //Instrucciones:
    //1. Declara una variable radius con el valor 5.5 (un número decimal).
    //2. Usa shadowing para calcular el área del círculo (π * radio^2). Para π, puedes usar std::f64::consts::PI.
    //3. Imprime el área.
    println!("----Ejercicio 11---");
    let radius: f64 = 5.5;
    let radius: f64 = std::f64::consts::PI * radius.powi(2);
    println!("El area es: {}", radius);

    //    Ejercicio 12: Control de temperaturas
    //Transforma una temperatura en grados Celsius a grados Fahrenheit.

    //Instrucciones:
    //1. Declara una variable celsius con el valor 25.0.
    //2. Usa shadowing para convertirla a Fahrenheit usando la fórmula fahrenheit = (celsius * 9.0/5.0) + 32.0.
    //3. Imprime el valor en grados Fahrenheit.
    println!("----Ejercicio 12-----");
    let celsius: f64 = 25.0;
    let celsius: f64 = celsius * 9.0 / 5.0 + 32.0;
    println!("El resultado de la conversion es: {}", celsius);

    //    Ejercicio 13: Conteo de vocales
    //Cuenta el número de vocales en una palabra utilizando shadowing.

    //Instrucciones:
    //1. Declara una variable text que contenga el string "shadowing".
    //2. Usa shadowing para contar cuántas vocales (a, e, i, o, u) tiene el string.
    //	Imprime el número de vocales.
    println!("-----Ejercicio 13----");
    let text = "shadowing";
    let text = text.chars().filter(|&c| "aeiou".contains(c)).count();
    println!("la cantidad de vocales: {}", text);


    //	Ejercicio 14: Reutilización de variables en funciones
    //Crea una función que calcule el cuadrado de un número y luego llámala varias veces con valores diferentes usando shadowing.

    //Instrucciones:
    //1. Crea una función square que acepte un número y retorne su cuadrado.
    //2. Declara una variable num con el valor 4 y usa shadowing para almacenar su cuadrado.
    //3. Redefine num y cálcula su cuadrado de nuevo con un valor diferente.
    //4. Imprime ambos resultados.
    println!("---Ejercicio 14---");
    fn square(n: i32) -> i32 {
	n * n
    }
    let num3 = 4;
    let num3 = square(num3);
    println!("El valor de num3 al cuadrado es: {}", num3);

    let num3 = 10;
    let num3 = square(num3);
    println!("El valor de num3 redefinido es: {}", num3);
    

}
