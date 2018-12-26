trait Visitor<T> {
    fn visit(&mut self, _ : &T);
}

enum Entity {
    File(String),
    Dir(String, Vec<Entity>),
}

struct ConcreteFileVisitor;

impl Visitor<Entity> for ConcreteFileVisitor {
    fn visit(&mut self, e: &Entity) {
        use crate::Entity::{Dir, File};
        match *e {
            File(ref name) => println!("file: {}", name),
            Dir(ref name, ref files) => {
                println!("dir {}", name);
                for file in files {
                    self.visit(file)
                }
            }
        }
    }
}

fn main() {
    use crate::Entity::{Dir, File};
    let e = Dir("/".to_string(), vec![File("etc".to_string()), File("usr".to_string())]);
    let mut visitor = ConcreteFileVisitor;
    visitor.visit(&e);
}
