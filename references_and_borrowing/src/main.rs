fn main() {
    println!("References ...");
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The len of '{s1}' is {len}");

    //Para que esto funcione la variable debe ser declarada como mutable.
    change(&mut s1);
    println!("{}", s1);

    //Ojo, es posible tener varias referencias inmutables de un valor, pero no pueden haber mas de una si existen al menos una inmutable
    let mut mutable_str = String::from("He: I wont change for you");
    //let ref1 = &mut mutable_str;
    //let ref2 = &mut mutable_str;//el compilador se va a quejar aqui porque ya existe una referencia mutable
    //Ojo, todo esto es para evitar race conditions.
    //println!("{ref1}, {ref2}");

    //Sin embargo se pueden usar scopes para poder declarar mas de una referencia mutable
    {
        let innner_ref = &mut mutable_str;
        //println!("{}", innner_ref);
        innner_ref.push_str(", Her: Are you");
    } //El truco aqui es que inner_ref get dropped al final de este scope, lo que significa que no existen dos mut ref al mismo tiempo.

    let outter_ref = &mut mutable_str; //No problem here.
    outter_ref.push_str(" sure?");
    println!("{}", outter_ref);


    //En este ejemplo se que pueden haber ref inmutable e inmutables
    //can la salvedad de que las inmutables no se usen cuando se empieze a usar la mutable
    let mut s = String::from("Marola");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    //de aqui en adelante se pueden usar las ref mutables porque ya no se estan usando las inmutables, es decir, no hay posibilidad de race conditions. 
    let r3 = &mut s; // no problem
    println!("{r3}");


}

//Esta funcion no se hace owner del str porque resive una reference a este, no el valor como tal
fn calculate_length(str: &String) -> usize {
    return str.len(); //cuando termina la ejecucion de la funcion el valor de str is not dropped
}

//Dado que las referencias no se pueden mutar esta funcion falla en tiempo de compilcaion
//cuz a reference can not be borrowed, unless it is mutable.
// fn change(str: &String) {
//     str.push_str(" added text");
// }

//En este caso si se puede mutar, porque se esta pasando una referencia mutable
fn change(str: &mut String) {
    str.push_str(" added text");
} //Pero igual s1 no pierde la ownership del valor porque se esta pasando una referencia.
