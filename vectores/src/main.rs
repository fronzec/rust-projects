fn main() {
    // 0 based index
    //let mut vector = vec![1,2,3];
    // let mut vector = Vec::new();
    let mut vector:Vec<i32> = Vec::new();

    println!("El valor del vector es {:?}",vector);
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);
    println!("El valor del vector es {:?}",vector);
    // insert by position
    vector.insert(0,0);
    println!("El valor del vector es {:?}",vector);

    vector.remove(vector.len()-1);
    println!("El valor del vector es {:?}",vector);

    let primer = vector[0];
    let ultimo = vector[vector.len()-1];
    println!("Primer elemento= {} Ultimo elemento={}",primer,ultimo);


    vector[0]=100;
    println!("El valor del vector es {:?}",vector);

    let ultimo_elemento = vector.pop().unwrap();//Option
    println!("El valor del vector es {:?} ultimo {}",vector, ultimo_elemento);

}
