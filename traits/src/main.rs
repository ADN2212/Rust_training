use std::fmt::Display;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//De esta manera se espesifica que un tipo implementa un trait
impl Summary for NewsArticle {
    //Este metodo debe cumplir con la firma del que esta definido en el trait
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
//Un trait es como unsa interface, sirve para expresar comportamientos comunes entra varios tipos
pub trait Summary {
    //Esto es como un metodo abstracto, define la firma de la funcion pero no NECESARIAMENTE su implementacion
    //fn summarize(&self) -> String;

    //Se pueden agregar default implementations, de modo que si un tipo no implementa el metodo se ejecute esto
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    //Los metodos abstractos deberan ser implemetados por los tipos que quieran implementar el trait
    fn summarize_author(&self) -> String;
}

fn main() {
    println!("Anotations about traits ...");
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&article);
    notify_usign_generics(&post);

    //Aunque newSp es espesificamente un SocialPost, el compilador infiere un impl Sumary.
    let new_sp = returns_summarizable();
    notify_usign_generics(&new_sp);

    //Vease como el metodo com_display esta permitido para estos dos tipos de pares.
    let p1 = Pair{x:"Hola", y:"Marola"};
    p1.cmp_display();
    let p2 = Pair{x:String::from("XD"), y: String::from("XDuh")};
    p2.cmp_display();


}


//Esta funcion resive cualquier tipo que implemete el trait sumary
//en nuestro caso son SocialPost y NewsArticle
//es declarada como pub porque un consumer podria crear su porpio tipo que implemente Sumary y user este metodo.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//Esta version y la anterior son esquivalentes,
//pero en teste caso es mas explicito
//por que ve que T es "de tipo" Sumary, aunque lo mas claro es decir que
//T es un tipo tal que implemeta el trait Sumary
pub fn notify_usign_generics<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//Tambien es posible que un metodo resiva mas de un tipo que implemente el trait
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
//pub fn notify<T: Summary>(item1: &T, item2: &T) {


//Tambien es valido que un metodo resiva tipos que implementan mas de un trait
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

//Y hay formas mas compactas de escrivar las firmas de estas funciones;
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     unimplemented!()
// }

//Ejemplo de una funcion que implementa un tipo que implementa un trait
//con una salvedad, el tipo debe ser espesidfico, why ?
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

// fn returns_summarizable_2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         //Notece como aqui el compilador se queja porque NewsArticle y SocialPost no son el mismo tipo aunque si implemente el mism trait, why ??
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             repost: false,
//         }
//     }
// }

struct Pair<T> {
    x: T,
    y: T,
}

//Tambien es posible hacer que ciertos metodos esten o no desponible de forma condicional en funcion de los traits que implementan
impl<T> Pair<T> {
    //Ejemplo, el metodo new, esta disponible para todos los pares
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    //pero este metodo solo esta disponible para los pares de tipos que implementen los traits Display y ParitalOrd (ambos)
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


