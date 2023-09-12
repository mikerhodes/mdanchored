/// mdanchored exists to slightly neaten link references within
/// markdown files. It moves link references to the end of the
/// section they are found within. A section is terminated with
/// either a new heading, regardless of level, or Hugo's
/// <!--more--> text.
///
/// We push the result through deno fmt, so that this app can be
/// used as a one-stop shop to make markdown files more to my
/// liking.
///
/// Placing links before the "more" ensures the Hugo's Summary
/// doesn't end up thinking there is more to come, avoiding
/// an uneeded "Read More" link.
/// This is purely selfish, of course, because it makes dx13
/// neater.
use regex::Regex;
use std::{
    io::{self, BufRead, Write},
    process::{Command, Stdio},
};

use argh::FromArgs;

#[derive(FromArgs)]
/// Neaten your Markdown anchors.
struct Args {
    /// post-process Markdown with deno fmt
    #[argh(switch)]
    deno: bool,
}

fn main() {
    let args: Args = argh::from_env();

    if args.deno {
        let denochk = Command::new("deno").arg("-V").status();
        if let Err(_) = denochk {
            eprintln!("Error: Cannot run deno; check it is on your $PATH.");
            std::process::exit(1);
        }

        // we set up a pipeline stdin -> process() -> deno -> stdout
        let mut deno = Command::new("deno")
            .args(["fmt", "--ext", "md", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to start deno process");

        // If the deno process fills its stdout buffer, it may end up
        // waiting until the parent reads the stdout, and not be able to
        // read stdin in the meantime, causing a deadlock.
        // Writing from another thread ensures that stdout is being read
        // at the same time, avoiding the problem.
        // https://doc.rust-lang.org/std/process/index.html.
        let mut deno_stdin = deno.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            let r = process(&mut io::stdin().lock(), &mut deno_stdin);
            if let Err(err) = r {
                println!("Error: {}", err);
                std::process::exit(1)
            }
        });

        deno.wait().expect("deno died");
    } else {
        let r = process(&mut io::stdin().lock(), &mut io::stdout().lock());
        if let Err(err) = r {
            println!("Error: {}", err);
            std::process::exit(1)
        }
    }
}

/// Reads a markdown document from src, moving link references
/// to the end of "sections", writing the result to dst.
/// A "section" is ended by a new heading or Hugo's <!--more-->
fn process<S: BufRead, D: Write>(src: &mut S, dst: &mut D) -> Result<(), io::Error> {
    let re = Regex::new(r"\[[^\]]+]: .+\s*$").unwrap();
    let more = "<!--more-->";
    let mut link_refs = Vec::new();

    // Iterate over our input from stdin
    for line in src.lines() {
        let s = line?;

        if re.is_match(&s) {
            // If we find a link, save it for later, and
            // don't print it to dst just yet.
            eprintln!("got a link {}", s);
            link_refs.push(s);
        } else {
            // If we find a header or <!--more-->, print
            // the links we've collected for this section
            // in link_refs, then print the header.
            let mut found = false;
            let trimmed = s.trim();
            if trimmed == more {
                found = true;
                eprintln!("line was more");
            } else if trimmed.starts_with("#") {
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
