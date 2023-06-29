use bstr::io::BufReadExt;
use bstr::{ByteSlice, ByteVec};
use gix_filter::driver::process;
use std::io::{stdin, stdout, Read, Write};
use std::time::Duration;

static PREFIX: &str = "➡";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let sub_command = args.nth(1).ok_or("Need sub-command")?;
    let filename = args.next(); // possibly %f
    let needs_failure = filename.as_deref().map_or(false, |file| file.ends_with("fail"));
    if needs_failure {
        panic!("failure requested for {sub_command}");
    }

    match sub_command.as_str() {
        "process" => {
            let mut srv = gix_filter::driver::process::Server::handshake(
                stdin(),
                stdout(),
                "git-filter",
                |versions| versions.contains(&2).then_some(2),
                &["clean", "smudge", "wait-1-s"],
            )?;

            let mut next_smudge_aborts = false;
            let mut next_smudge_fails_permanently = false; // a test validates that we don't actually hang
            while let Some(mut request) = srv.next_request()? {
                let needs_failure = request
                    .meta
                    .iter()
                    .find_map(|(key, value)| (key == "pathname").then_some(value))
                    .map_or(false, |path| path.ends_with(b"fail"));
                if needs_failure {
                    panic!("process failure requested: {:?}", request.meta);
                }
                match request.command.as_str() {
                    "clean" => {
                        let mut buf = Vec::new();
                        request.as_read().read_to_end(&mut buf)?;
                        request.write_status(process::Status::success())?;

                        let mut lines = Vec::new();
                        for mut line in buf.lines_with_terminator() {
                            if line.starts_with(PREFIX.as_bytes()) {
                                line = &line[PREFIX.len()..];
                            }
                            lines.push_str(line);
                        }
                        request.as_write().write_all(&lines)?;
                        request.write_status(process::Status::Previous)?;
                    }
                    "smudge" => {
                        let mut buf = Vec::new();
                        request.as_read().read_to_end(&mut buf)?;
                        let status = if next_smudge_aborts {
                            next_smudge_aborts = false;
                            process::Status::abort()
                        } else if next_smudge_fails_permanently {
                            process::Status::exit()
                        } else {
                            process::Status::success()
                        };
                        request.write_status(status)?;

                        let mut lines = Vec::new();
                        for line in buf.lines_with_terminator() {
                            if !line.starts_with(PREFIX.as_bytes()) {
                                lines.push_str(PREFIX.as_bytes());
                            }
                            lines.push_str(line);
                        }
                        request.as_write().write_all(&lines)?;
                        request.write_status(process::Status::Previous)?;
                    }
                    "wait-1-s" => {
                        std::io::copy(&mut request.as_read(), &mut std::io::sink())?;
                        request.write_status(process::Status::success())?;
                        std::thread::sleep(Duration::from_secs(1));
                    }
                    "next-smudge-aborts" => {
                        std::io::copy(&mut request.as_read(), &mut std::io::sink())?;
                        request.write_status(process::Status::success())?;
                        next_smudge_aborts = true;
                    }
                    "next-invocation-returns-strange-status-and-smudge-fails-permanently" => {
                        std::io::copy(&mut request.as_read(), &mut std::io::sink())?;
                        request.write_status(process::Status::success())?;
                        next_smudge_fails_permanently = true;
                    }
                    unknown => panic!("Unknown capability requested: {unknown}"),
                }
            }
        }
        "clean" => {
            let mut stdin = stdin().lock();
            let mut stdout = stdout().lock();
            stdin.for_byte_line_with_terminator(|mut line| {
                if line.starts_with(PREFIX.as_bytes()) {
                    line = &line[PREFIX.len()..];
                }
                stdout.write_all(line).map(|_| true)
            })?;
        }
        "smudge" => {
            let mut stdin = stdin().lock();
            let mut stdout = stdout().lock();
            stdin.for_byte_line_with_terminator(|line| {
                if !line.starts_with(PREFIX.as_bytes()) {
                    stdout.write_all(PREFIX.as_bytes())?;
                }
                stdout.write_all(line).map(|_| true)
            })?;
        }
        unknown => panic!("Unknown sub-command: {unknown}"),
    }
    Ok(())
}
