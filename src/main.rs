use hash_tag::parse;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let md_file = if args.len() > 1 {
        args[1].clone()
    } else {
        panic!("Error: You must provide a Markdown file.");
    };

    let mut html_file = None;
    if let Some(pos) = args.iter().position(|x| x == "-o") {
        if pos + 1 < args.len() {
            html_file = Some(args[pos + 1].clone());
        } else {
            panic!("Error: Missing output file name after '-o' flag.");
        }
    }

    let mut html_view_file = None;
    let mut generate_view = false;
    if let Some(pos) = args.iter().position(|x| x == "-v") {
        if pos + 1 < args.len() {
            html_view_file = Some(args[pos + 1].clone());
            generate_view = true;
        } else {
            panic!("Error: Missing view file name after '-v' flag.");
        }
    }

    let debug = args.contains(&"-d".to_string());

    parse(
        md_file.to_string(),
        html_file,
        html_view_file,
        Some(generate_view),
        Some(debug),
    )
    .expect("Unable to run hash-tag");
}
