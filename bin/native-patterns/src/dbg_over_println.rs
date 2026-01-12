pub fn dbg_over_println() {
    let value = 42;
    println!("The value is: {}", value);
    dbg!(value);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("Person: name = {}, age = {}", person.name, person.age);
    dbg!(person);
}
