trait Product {
    fn convert(&self, _ : String) -> String;
}

struct Factory;

impl Factory {
    fn convert<P, F>(&self, s: String, create_product: F) -> String
        where P: Product,
              F: FnOnce() -> P
    {
        create_product().convert(s)
    }
}

struct ConcreteProductX;
impl Product for ConcreteProductX {
    fn convert(&self, s: String) -> String {
        s.to_uppercase()
    }
}

fn main() {
    let f = Factory;
    println!("{}", f.convert("hogehoge piyopiyo".to_string(), || ConcreteProductX));
}
