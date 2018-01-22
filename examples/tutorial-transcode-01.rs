// to run:
// cargo run --example tutorial-transcode-01 < examples/data/smallpop.csv

// This makes the csv crate accessible to your program.
extern crate csv;
extern crate serde_transcode;

// Import the standard library's I/O module so we can read from stdin.
use std::io;

// The `main` function is where your program starts executing.
fn main() {
    let reader = io::stdin();
    let writer = io::stdout();

    let mut deserializer = csv::deserializer::from_reader(reader);
    let mut serializer = csv::serializer::to_writer(writer);
    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
    serializer.into_inner().flush().unwrap();
}
