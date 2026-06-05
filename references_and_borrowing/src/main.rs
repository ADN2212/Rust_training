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


    //About dangling references:
    //let pointer_to_nowhere = dangle();

    let good_one = no_dangle();
    println!("{}", good_one);    

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


//El problema que esta funcion genera es que s y dropped cuando dangle retorna.
// fn dangle() -> &String {
//     let s = String::from("eh?");
//     return &s;
// }

//En este caso no hay problema porque el caller de esta funcion pasa a ser el owner del valor
//Anteiormente se pasaba una referencia de un valor iba a ser dropeado antes de ser usado,
//De ahi la idea de referencia colgada.
//todo esto sirve para evitar el famoso null pointer error.
fn no_dangle() -> String {
    return String::from("chill");
}

//En resumen, estas son las reglas de las referencias:
//1-At any given time, you can have either one mutable reference or any number of immutable references.
//2-References must be always be valid.

//Sobre uno
//El hecho de que no puedan coexistir referencias inmutables con mutables a un mismo valor es porque el compilador busca evitar las reace conditions.
//solo cuando es seguro que no se va a leer un determinado valor desde una referencia inmutable es cuando se permite declarar una mutable ref.

//Sobre dos
//La finalidad de esto es evitar punteros hacia valores nulos. (Como en el ejemplo de dangle).
