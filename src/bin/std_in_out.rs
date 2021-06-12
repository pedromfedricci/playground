use io::BufWriter;
use std::fs::File;
use std::io::{self, Write};

//use io::Stdout;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    write!(stdout, "User input: ")?;
    // Also works:
    //
    //write!(&mut stdout, "User input: ")?;
    //
    //stdout.write_fmt(format_args!("{}", "User input: "))?;
    //
    //<Stdout as Write>::write_fmt(&mut stdout, format_args!("{}", "User input: "))?;

    stdout.flush()?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    writeln!(stdout, "\"{}\" was provided", buf.trim())?;
    let file = File::create("output.txt")?;
    write!(BufWriter::new(file), "{}", buf)?;

    Ok(())
}
