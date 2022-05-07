fn main() {
    let numeros = [1,2,3,4,5];
    println!("EL valor del arreglo es {:?}",numeros);
    // Type and max capacity
    let numeros:[i32;5] = [1,2,3,4,5];
    println!("EL valor del arreglo es {:?}",numeros);
    // Array of fixed size and common value in all elements
    let mut numeros = [5.5;10];
    println!("EL valor del arreglo es {:?}",numeros);
    
    let primer_elemento = numeros[0];
    println!("El primer elemento es {}",primer_elemento);

    let ultimo_elemento = numeros[numeros.len()-1];
    println!("El ultimo elemento es {}",ultimo_elemento);

    //OUr array must be mutable
    numeros[2]= 30.9;
    println!("EL valor del arreglo es {:?}",numeros);
}
