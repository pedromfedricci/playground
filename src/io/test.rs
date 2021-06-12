use std::fs::File;
use std::io::{self, BufRead, Write};

use io::BufWriter;

fn grep<R: BufRead>(target: &str, reader: R) -> io::Result<()> {
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

#[test]
fn mock_grep() -> io::Result<()> {
    let target = "target";
    let text = format!(
        "\
        My text is full of {target}s\n\
        {target} is what you want isn't it?\n\
        Maybe some lines won't have them though\n\
        Very nice indeed!\n\
        One more right here at the end: {target}.",
        target = target
    );

    grep(target, text.as_bytes())
    //Err(io::Error::new(io::ErrorKind::Other, "oops"))
}

#[test]
fn file_creation() -> io::Result<()> {
    let text = "\
        My first line of the text.\n\
        Second line of the text.\n\
        bla bla bla bla bla bla bla bla\n\
        hmmmmmmmmm.";

    let file = File::create("output.txt")?;
    writeln!(BufWriter::new(file), "{}", text)?;

    Ok(())
}
