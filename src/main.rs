use std::io::{ stdin, Read, Write, BufWriter, stdout };

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let chars = input.chars().collect::<Vec<_>>();
    let mut writer = BufWriter::new(stdout());
    for &c in chars.iter() {
        write!(writer, "{}\u{200C}", c).unwrap();
    }
    writer.flush().unwrap();
}
