use bstr::io::BufReadExt;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let sub_command = args.nth(1).ok_or("Need sub-command")?;
    let filename = args.next(); // possibly %f
    let needs_failure = filename.as_deref().map_or(false, |file| file.ends_with("fail"));
    if needs_failure {
        panic!("failure requested for {sub_command}");
    }

    match sub_command.as_str() {
        "clean" => {
            let mut stdin = stdin().lock();
            let mut stdout = stdout().lock();
            stdin.for_byte_line_with_terminator(|mut line| {
                if line.starts_with(b"\t") {
                    line = &line[1..];
                }
                stdout.write_all(line).map(|_| true)
            })?;
        }
        "smudge" => {
            let mut stdin = stdin().lock();
            let mut stdout = stdout().lock();
            stdin.for_byte_line_with_terminator(|line| {
                if !line.starts_with(b"\t") {
                    stdout.write_all(b"\t")?;
                }
                stdout.write_all(line).map(|_| true)
            })?;
        }
        unknown => panic!("Unknown sub-command: {unknown}"),
    }
    Ok(())
}
