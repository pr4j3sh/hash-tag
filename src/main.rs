use hash_tag::parse;
use std::env;
use std::fs;
use std::io;

fn read_file(filename: &str) -> io::Result<String> {
    let md = fs::read_to_string(filename)?;
    Ok(md)
}

fn write_file(filename: &str, html: &str) -> io::Result<()> {
    fs::write(filename, html)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let md_file = if args.len() > 1 {
        args[1].clone()
    } else {
        panic!("Error: You must provide a Markdown file.");
    };

    let mut html_file = String::from("index.html");
    let mut html_view_file = String::from("view.html");
    let mut generate_view = false;

    if let Some(pos) = args.iter().position(|x| x == "-o") {
        if pos + 1 < args.len() {
            html_file = args[pos + 1].clone();
        } else {
            panic!("Error: Missing output file name after '-o' flag.");
        }
    }

    if let Some(pos) = args.iter().position(|x| x == "-v") {
        if pos + 1 < args.len() {
            html_view_file = args[pos + 1].clone();
            generate_view = true;
        } else {
            panic!("Error: Missing view file name after '-v' flag.");
        }
    }

    let md = read_file(&md_file).expect("Failed to read file");

    let html = parse(md);

    write_file(&html_file, &html).expect("Failed to write file.");
    if generate_view {
        let view_html = format!("<!doctype html><html lang=\"en\"><head><meta charset=\"UTF-8\" /><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" /><title>Hashtag</title><link rel=\"stylesheet\" type=\"text/css\" href=\"https://pr4j3sh.github.io/ui/style.css\" /></head><body><div class=\"container\"><main>{html}</main></div></body></html>");
        write_file(&html_view_file, &view_html).expect("Failed to write file.");
    }
}
