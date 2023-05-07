fn main() {
    println!("Hello, world!");
    let val_no_mutable=34; // Definir una variable, no se puede modificar
    println!("El valor es: {}", val_no_mutable); // Imprimir valor

    let mut val_mutable=0; // Definir una variabl, en este caso si se puede modificar gracias a 'mut'
    println!("El valor mutable es: {}",val_mutable);
    val_mutable=43; // Asignamos un valor nuevo
    println!("El valor ya modificado es: {}",val_mutable);

    const PI: f32 = 3.1416;  //Definiendo una constante, a diferencia de las variables no mutable, es que estas se definen en compilacion
    println!("El valor de PI es: {}",PI);

    let val_no_mutable="Hola"; // Redefiniendo variables inmutables (shadowing), se REDEFINE la variable
    println!("Variable redefinida: {}",val_no_mutable);

}
