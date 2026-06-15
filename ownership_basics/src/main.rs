//Ownership Rules!!!
//1-Each value in Rust has an owner.
//2-There can be only one owner at a time.
//3-When the owner goes out of scope, the value will be dropped.

fn main() {
    println!("Let see some basic examples of ownership");

    //Para valores que estan en el stack no hay problemas con esto
    let a = 5;
    let b = a;

    println!("{}", a);//No perdio la ownership
    println!("{}", b);//tiene su propia copia de 5


    //Ahora que pasa con valores que estan en el heap, es decir, valores por referencia.
    let s = String::from("Some potencia dinamic ");
    let s2 = s;//Ahora s2 es el nuevo owner del str origal, es decir, tiene el puntero (la direccion), a esto se le llama mover el value,
    //println!("{}", s);//Da un error porque s ya no es el owner del str, se podria decir que el pointer fue pasado a s2.
    println!("{}", s2);//esta linea si funciona porque s2 es el owner
    //Something importan to note:
    //Cuando se intenta usar s en el print el compilador lanza el error pero tambien sugiere usar el methodo clone(),
    //este meto crea una deep copy, es decir, crea una copia completa del str en el heap,
    //by desing Rust no crea deep copies para no afectar el uso de memoria, osea performance !!!

    //Ejemplo con scopes
    {
        let q = 10;
        //Doing some stuff with q
        println!("{}", q)
    }//Una vez "terminado" el scope se llama la funcion drop que quita los valores del stack 
    //println!("{}", q);//q no existe aqui, este comportamiento es parecido a muchos otros lenguages.


    let c = &5;
    let _d = c;
    println!("{}", c);//Porque no surge un problema de compilacion aqui ???

    //Mas sobre referencias:
    let mut str = String::from("hola");//En esta declaracion str tiene el puntero que apunta a "Hola"
    println!("{}", str);
    str = String::from("Marola");//cuando se esta asignacion el espacio en memoria de "hola" es liberado (use of the drop function) y str pasa a tener el puntero de "Marola"
    println!("{}", str);


    //Ownership and Functions
    let s = String::from("Yajaira");
    takes_ownership(s);
    //println!("{}", s);//El compilador lo dice claramente, la propiedad the s fue tomada por la funcion de arriba, so in this point s is already dropped.

    let z = 5;
    makes_copy(z);//Aqui no hay problema, ya que lo que pasa en la funcion es una copia de z

    let seneida = gives_ownership();
    println!("{} me pertenece", seneida);

    let jezabel = takes_and_gives_back(String::from("Jezabel"));
    println!("{} was mine, she let me alone, but she is here again", jezabel);


    let last_str = String::from("Foooo");
    let (las_str2, len) = calculate_lenght(last_str); 
    //println!("{} has len = {}", lastStr, len);//aqui lastStr perdio la ownership
    println!("{} has len = {}", las_str2, len)



}


fn takes_ownership(some_string: String) {
    println!("I have got the ownership of '{}'", some_string);
    //Cuando el scope the esta funcion se va, some_string is dropped.
}

fn makes_copy(some_integer: i32) {
    println!("I just take a copy of {}", some_integer);
    //Since some_integer is a copy, the original int can be userd afther this function call.
}

fn gives_ownership() -> String {
    //Originalmente esta funcion tiene la propiedad de Seneida
    let str = String::from("Seneida");
    return str;//el consumer de esta funcion optendra la propieda de Seneida.
}

fn takes_and_gives_back(str: String) -> String {
    //This is a tricky one
    //la funcion optiene la propiedad de str
    //pero tambien se la da al consumer debido a que retorna el mismo str
    //Ojo: eso de dar y resivir ownership no es mas que un puntero cambiado de un owner a otro.
    return str;
}


//De esta manera se puede hacer algo con el valor si que se pierda la referencia.
fn calculate_lenght(str: String) -> (String, usize) {
    let l = str.len();
    return (str, l);
}
