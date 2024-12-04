use std::env;
use std::fs;
use std::io;

fn read_file(filename: &str, debug: bool) -> io::Result<String> {
    let md = fs::read_to_string(filename)?;
    if debug {
        println!("{filename} read");
    }
    Ok(md)
}

fn write_file(filename: &str, html: &str, debug: bool) -> io::Result<()> {
    fs::write(filename, html)?;
    if debug {
        println!("{filename} written");
    }
    Ok(())
}

fn get_lines(md: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::from("");
    for c in md.chars() {
        if c == '\n' {
            if !line.is_empty() {
                lines.push(line.clone());
            }
            line.clear();
        } else {
            line.push(c);
        }
    }
    lines
}

fn close_list_tags(tag_ul: &mut bool, tag_ol: &mut bool, html: &mut String, debug: bool) {
    if *tag_ul {
        html.push_str("</ul>");
        *tag_ul = false;
        if debug {
            print!("ul | ");
        }
    }
    if *tag_ol {
        html.push_str("</ol>");
        *tag_ol = false;
        if debug {
            print!("ol | ");
        }
    }
}

fn process_line(l: &str) -> String {
    let mut chars = l.chars().peekable();
    let mut line = String::from("");

    let mut tag_code = false;
    let mut tag_b = false;
    let mut tag_i = false;

    while let Some(c) = chars.next() {
        if c == '`' {
            tag_code = !tag_code;
            if tag_code {
                line.push_str("<code>");
            } else {
                line.push_str("</code>");
            }
        } else if c == '*' {
            if chars.peek() == Some(&'*') {
                chars.next();
                tag_b = !tag_b;
                if tag_b {
                    line.push_str("<b>");
                } else {
                    line.push_str("</b>");
                }
            }
        } else if c == '_' {
            tag_i = !tag_i;
            if tag_i {
                line.push_str("<i>");
            } else {
                line.push_str("</i>");
            }
        } else if c == '[' {
            let mut text = String::new();
            while let Some(&next) = chars.peek() {
                if next == ']' {
                    chars.next();
                    break;
                }
                text.push(next);
                chars.next();
            }

            if chars.peek() == Some(&'(') {
                chars.next();
                let mut url = String::new();
                while let Some(&next) = chars.peek() {
                    if next == ')' {
                        chars.next();
                        break;
                    }
                    url.push(next);
                    chars.next();
                }

                line.push_str(&format!("<a class=\"link\" href=\"{url}\">{text}</a>"));
            } else {
                line.push('[');
                line.push_str(&text);
                line.push(']');
            }
        } else {
            line.push(c);
        }
    }

    line.to_string()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let md_file = if args.len() > 1 {
        &args[1]
    } else {
        panic!("Error: You must provide a Markdown file.");
    };
    let mut html_file = "./temp/index.html".to_string();
    if let Some(pos) = args.iter().position(|x| x == "-o") {
        if pos + 1 < args.len() {
            html_file = args[pos + 1].clone();
        } else {
            panic!("Error: Missing output file name after '-o' flag.");
        }
    }
    let mut html_view_file = "./views/index.html".to_string();
    if let Some(pos) = args.iter().position(|x| x == "-v") {
        if pos + 1 < args.len() {
            html_view_file = args[pos + 1].clone();
        } else {
            panic!("Error: Missing view file name after '-v' flag.");
        }
    }
    let mut debug = false;
    if args.contains(&"-d".to_string()) {
        debug = true;
    }

    let md = read_file(md_file, debug).expect("Failed to read file");
    let lines: Vec<String> = get_lines(&md);
    let mut html = String::from("");

    let mut tag_pre = false;
    let mut tag_ul = false;
    let mut tag_ol = false;

    for (i, l) in lines.iter().enumerate() {
        if debug {
            print!("{:4} | ", i);
        }
        for (j, c) in l.chars().enumerate() {
            if j == 0 && c == '#' {
                // heading
                close_list_tags(&mut tag_ul, &mut tag_ol, &mut html, debug);
                let count_hash = l.chars().filter(|&c| c == '#').count();
                if debug {
                    print!("h{count_hash} | ");
                }
                let line = process_line(&l[count_hash + 1..]);
                let line = format!("<h{count_hash}>{line}</h{count_hash}>");
                html.push_str(&line);
                if debug {
                    print!("{line}");
                }
            } else if j == 0 && c == '>' {
                // blockquote
                close_list_tags(&mut tag_ul, &mut tag_ol, &mut html, debug);
                if debug {
                    print!("blockquote");
                }
                let line = process_line(&l[2..]);
                let line = format!("<blockquote>{line}</blockquote>");
                html.push_str(&line);
                if debug {
                    print!(" | {line}");
                }
            } else if j == 0 && c == '-' {
                // unordered list
                if l.trim().chars().all(|ch| ch == '-') {
                    html.push_str("<hr>");
                    if debug {
                        print!("hr | ");
                    }
                    break;
                }
                if !tag_ul {
                    html.push_str("<ul>");
                    if debug {
                        print!("ul | ");
                    }
                }
                tag_ul = true;
                if debug {
                    print!("li | {tag_ul}");
                }
                let line = process_line(&l[2..]);
                let line = format!("<li>{line}</li>");
                html.push_str(&line);
                if debug {
                    print!(" | {line}");
                }
            } else if j == 0 && c.is_ascii_digit() {
                // ordered list
                if !tag_ol {
                    if debug {
                        print!("ol | ");
                    }
                }
                tag_ol = true;
                if debug {
                    print!("li | {tag_ol}");
                }
                let line = process_line(&l[3..]);
                let line = format!("<li>{line}</li>");
                html.push_str(&line);
                if debug {
                    print!(" | {line}");
                }
            } else if j == 0 && c == '`' {
                // codeblock
                close_list_tags(&mut tag_ul, &mut tag_ol, &mut html, debug);
                let count_backtick = l.chars().filter(|&c| c == '`').count();
                if count_backtick == 3 {
                    tag_pre = !tag_pre;
                    if tag_pre {
                        html.push_str("<pre>");
                    } else {
                        html.push_str("</pre>");
                    }
                    if debug {
                        print!("pre | {tag_pre}");
                    }
                } else if count_backtick % 2 == 0 {
                    if debug {
                        print!("p | ");
                    }
                    let line = process_line(l);
                    let line = format!("<p>{line}</p>");
                    html.push_str(&line);
                    if debug {
                        print!("{line}");
                    }
                }
            } else if j == 0 && tag_pre {
                // paragraph
                if debug {
                    print!("code");
                }
                let line = format!("<code>{l}</code>\n");
                html.push_str(&line);
                if debug {
                    print!(" | {line}");
                }
            } else if j == 0 && !tag_pre {
                // paragraph
                close_list_tags(&mut tag_ul, &mut tag_ol, &mut html, debug);
                if debug {
                    print!("p | ");
                }
                let line = process_line(l);
                let line = format!("<p>{line}</p>");
                html.push_str(&line);
                if debug {
                    print!("{line}");
                }
            }
        }
        if debug {
            println!(" | {l}");
        }
    }

    if debug {
        println!("\n{html}");
    }
    write_file(&html_file, &html, debug).expect("Failed to write file.");
    let html = format!("<!doctype html><html lang=\"en\"><head><meta charset=\"UTF-8\" /><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" /><title>Hashtag</title><link rel=\"stylesheet\" type=\"text/css\" href=\"https://pr4j3sh.github.io/ui/style.css\" /></head><body><div class=\"container\"><main>{html}</main></div></body></html>");
    write_file(&html_view_file, &html, debug).expect("Failed to write file.");

    Ok(())
}
