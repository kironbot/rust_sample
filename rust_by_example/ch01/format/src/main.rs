fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{s} {v} {o}", o="the lazy dog", v="jumps over", s="the quick brown fox");
    println!("{:b} of {:b} people know binary, the other half doesn't", 1, 8);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:0>width$}", number=1, width=6);
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");
    println!("Now {:?} will print.", Structure(32));
    println!("Now {:#?} will print.", Deep(Structure(32)));

    let peter = Person {name:"Peter", age:27};
    println!("{:#?}", peter);
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}