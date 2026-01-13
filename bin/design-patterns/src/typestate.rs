struct File<State> {
    state: State,
}

struct Closed;
struct Open;

impl File<Closed> {
    fn open(self) -> File<Open> {
        println!("Opening file...");
        File { state: Open }
    }
}

impl File<Open> {
    fn read(&self) {
        println!("Reading from file...");
    }

    fn write(&self) {
        println!("Writing to file...");
    }

    fn close(self) -> File<Closed> {
        println!("Closing file...");
        File { state: Closed }
    }
}

pub fn typestate_pattern() {
    let file = File { state: Closed };
    let file = file.open();
    file.read();
    file.write();
    file.close();
}
