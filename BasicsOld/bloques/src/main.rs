fn main() {
    let variable = "Variable en bloque main";

    let resultado = {
        let variable = "variable en bloque anidado";// shadowing only applies for same scope
        println!("{}",variable);
        variable // retornar el valor
    };
    println!("{}",variable);
    println!("resultado {}",resultado);


    let calificacion:i8 = 10;
    let mut mensaje = String::new();

    if calificacion == 10 {
        mensaje = String::from("felicitaciones!!");
    } else {
        mensaje = String::from("Necesitas estudiar mas");
    }

    println!("{}",mensaje);

    // Refactor to rust style

    let msg = if calificacion == 10 {
        String::from("felicitaciones!!")// returned value, no contains ; at the end
    } else {
        String::from("Necesitas estudiar mas")
    };

    println!("{}",msg);
}
