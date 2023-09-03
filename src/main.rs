use regex::Regex;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    process(&mut io::stdin().lock());
    Ok(())
}

fn process(src: &mut dyn BufRead) {
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
                    let _ = println!("{}", link);
                    eprintln!("output a link {}", link)
                }
                link_refs.clear();
                println!("")
            }

            let _ = println!("{}", s);
        }
    }

    // Print any remaining links on exit
    println!("");
    for link in &link_refs {
        let _ = println!("{}", link);
        eprintln!("output a link {}", link)
    }
}
