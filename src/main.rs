use regex::Regex;
use std::{
    io::{self, BufRead, Write},
    process::{Command, Stdio},
};

fn main() {
    let mut deno = Command::new("deno")
        .args(["fmt", "--ext", "md", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to start sed process");

    // If the deno process fills its stdout buffer, it may end up
    // waiting until the parent reads the stdout, and not be able to
    // read stdin in the meantime, causing a deadlock.
    // Writing from another thread ensures that stdout is being read
    // at the same time, avoiding the problem.
    // https://doc.rust-lang.org/std/process/index.html.
    let mut stdin = deno.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        let r = process(&mut io::stdin().lock(), &mut stdin);
        if let Err(err) = r {
            println!("Error: {}", err);
            std::process::exit(1)
        }
    });

    deno.wait().expect("deno died");
}

fn process(src: &mut dyn BufRead, dst: &mut dyn Write) -> Result<(), io::Error> {
    let re = Regex::new(r"\[[^\]]+]: .+\s*$").unwrap();
    let more = "<!--more-->";
    let mut link_refs = Vec::new();

    // Iterate over our input from stdin
    for line in src.lines() {
        let s = line.unwrap();

        if re.is_match(&s) {
            // If we find a link, save it for later.
            eprintln!("got a link {}", s);
            link_refs.push(s);
        } else {
            // If we find a header or <!--more-->, print
            // the found links before the line.
            let mut found = false;
            if s.trim() == more {
                found = true;
                eprintln!("line was more");
            } else if s.starts_with("#") {
                found = true;
                eprintln!("line was header");
            }

            if found && link_refs.len() > 0 {
                for link in &link_refs {
                    writeln!(dst, "{}", link)?;
                    eprintln!("output a link {}", link);
                }
                link_refs.clear();
                writeln!(dst)?;
            }

            writeln!(dst, "{}", s)?;
        }
    }

    // Print any remaining links on exit
    writeln!(dst)?;
    for link in &link_refs {
        writeln!(dst, "{}", link)?;
        eprintln!("output a link {}", link)
    }
    Ok(())
}
