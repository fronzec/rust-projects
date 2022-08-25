fn main() {
    // Indexes on tuples are based
     let tupla = (1,false,4.4);
     println!("El valor de la tupla {:?}",tupla);

     // We can specify the order of types and the size
     let mut tupla : (i32, bool, f64) = (2, true, 54.2);
     println!("El valor de la tupla {:?}",tupla);

     // Read single elements by index
     let primer_elemento = tupla.0;
     let ultimo_elemento = tupla.2;
     println!("El primer elemento es {}",primer_elemento);
     println!("El ultimo elemento es {}",ultimo_elemento);

    tupla.2 = 999.999;
    println!("El valor de la tupla {:?}",tupla);
     
}
