use std::collections::HashMap;

fn main() {
    println!("Common Collections");
    //Los vecotres de Rust son como las listas de Python, los slices de Go o los arrays de JS,
    //Puedenc ontener cualquier tipo (Vec<T>) y su len es puede cambiar en runtime, osea, sus elementos se guardan en el heap
    let mut v: Vec<i32> = Vec::new();
    v.push(5); //Para que esto sea posible v tiene que ser declarado como mut
    v.push(6);
    v.push(7);
    v.push(8);

    let v2 = vec![1, 2, 3]; //vec! es un macro
    //Hacer referencia a los valores de un vec
    let sarutobi = &v2[2]; //esto es una referencia inmutable
    //de no user la referencia sarutobi tomaria el ownership de del tercer elemento del array
    println!("El tercer hokage es {}", sarutobi);

    //Tambien se puede user el option
    let third = v.get(10);
    match third {
        Some(t) => println!("there was an element {}", t),
        None => println!("There was no element"),
    }

    //Recuerda que el borrow checker tambien funciona aqui
    let first = &v[0]; //This is a mutable borrow
    //v.push(6);//this is an inmutable borrow
    println!("the first element is {}", first);

    //We can iterate over the vec as follows:
    let v3 = vec![10, 9, 8, 7, 6];
    for i in &v3 {
        println!("{}", i);
    }

    //Si el vector es mutable podemos cambiar sus elementos usando referencias
    let mut v4 = vec![100, 101, 102];
    for i in &mut v4 {
        //C like dereference syntax
        *i += 52;
    }

    println!("Plus 50 elemets ..");
    for i in &v4 {
        println!("{}", i);
    }

    //Se pueden usar enums para almacenar tipos complejos
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for r in &row {
        // r match {
        //     SpreadsheetCell::Int(_) => println!("Es un entero"),
        //     SpreadsheetCell::Float(_) => println!("Es un flotante"),
        //     SpreadsheetCell::Text(_)=> println!("Es texto"),
        // };
        println!("{:?}", r);
    }

    //Al igual que con las otras referencias, al salir de scope los vectores son dropeados
    {
        let inner_vec = vec![10, 20];
        println!("{}", inner_vec[0]);
    }//here innner_vec goes out of scope.
    //println!("{}", inner_vec[0]);//here inner_vec does not exists cuz it already has been dropped

    //Nota: Las strs son un tipo especial de colecciones, asi que las dejare para luego
    //Lets go with Hash Maps

    //Este se inicializo como unknown, unknown
    //Pero cambio de tipo cuando se hizo la primera asignacion
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    //Vease como el metodo get toma un referencia inmutable, esto es para no tomar ownewhip de su argumento
    let blue_team_score = scores.get(&team_name).copied().unwrap_or(0);//unwrap_or es como el getOrElse de Scala   
    println!("{}", blue_team_score);
    println!("{}", team_name);

    //como en otros lenguages es posible interar sobre los keys values de in map
    //observece como se usa & para anotar que es una referencia inmutable 
    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    //Ownership and HasMap
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    //Esta linea no compila porque en este punto map es owner the las dos strs
    //println!("{}:{}", field_name, field_value);    

    //Updating HasMaps:
    let mut another_map = HashMap::new();
    another_map.insert(String::from("Blue"), 10);
    //Como en todo HashMap, las key son un set, lo que quiere decir que son irrepetibles, por eso al user insert, si la key ya existe se actualiza su valor al dado.
    another_map.insert(String::from("Blue"), 25);
    println!("{another_map:?}");

    //the entry method
    //Escanea un valor, si existe lo actualiza y si no lo inserta
    another_map.entry(String::from("Yellow")).or_insert(50);
    another_map.entry(String::from("Blue")).or_insert(50);
    println!("{another_map:?}");//Aqui blue sigue siendo 25 porque ya existia

    //Esto sirve para resumir la casica forma de acumular con los mapas
    //Enjemplo, en vez de:
    // map = {}
    // for ch in str:
    //     if ch in map:
    //         map[ch] += 1 
    //     esle:
    //         map[ch] = 1
    //Con inser seria:
    let text = "Hellow world, wonderful world";
    let mut counter = HashMap::new();
    for w in text.split_whitespace() {
        let count = counter.entry(w).or_insert(0);
        *count += 1;//esto es el puntero al valor del la key en el map, por eso se puede modificar desde aqui.
    }

    println!("{counter:?}");


}
