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

fn main() -> io::Result<()> {
    let md = read_file("README.md")?;

    let lines: Vec<String> = get_lines(&md);

    for (i, line) in lines.iter().enumerate() {
        println!("{:4} | {:4} | {line}", i, line.len());
    }
    let mut html = String::from("");

    let mut count_hash = 0;
    let mut count_backtick = 0;

    let mut i = 0;
    let n = md.len();

    let mut tag_head = false;
    let mut tag_head_h1 = false;
    let mut tag_head_h2 = false;
    let mut tag_head_h3 = false;
    let mut tag_head_h4 = false;
    let mut tag_head_h5 = false;
    let mut tag_head_h6 = false;
    let mut tag_backtick = false;
    let mut tag_code = false;
    let mut tag_pre = false;
    let mut tag_blockquote = false;
    let mut tag_list = false;
    let mut tag_paragraph = false;

    while i < n {
        if let Some(c) = md.chars().nth(i) {
            println!("{:4} | {}", i, c);
            if c == '#' {
                for j in i..i + 6 {
                    if let Some(l) = md.chars().nth(j) {
                        if l == '#' {
                            println!("{l}");
                            count_hash += 1;
                        }
                    }
                }
                println!("{count_hash}");
                i += count_hash;
                if count_hash == 1 {
                    html.push_str("<h1>");
                    tag_head_h1 = true;
                } else if count_hash == 2 {
                    html.push_str("<h2>");
                    tag_head_h2 = true;
                } else if count_hash == 3 {
                    html.push_str("<h3>");
                    tag_head_h3 = true;
                } else if count_hash == 4 {
                    html.push_str("<h4>");
                    tag_head_h4 = true;
                } else if count_hash == 5 {
                    html.push_str("<h5>");
                    tag_head_h5 = true;
                } else if count_hash == 6 {
                    html.push_str("<h6>");
                    tag_head_h6 = true;
                }
                tag_head = true;
                count_hash = 0;
            } else if c == '>' {
                html.push_str("<blockquote>");
                tag_blockquote = true;
            } else if c == '-' {
                html.push_str("<li>");
                tag_list = true;
            }

            if c == '`' && !tag_backtick {
                for j in i..i + 3 {
                    if let Some(l) = md.chars().nth(j) {
                        if l == '`' {
                            println!("{l}");
                            count_backtick += 1;
                        }
                    }
                }
                println!("{count_backtick}");
                i += count_backtick - 1;
                if count_backtick == 1 {
                    html.push_str("<code>");
                    tag_code = true;
                } else if count_backtick == 3 {
                    html.push_str("<pre><code>");
                    tag_pre = true;
                }
                tag_backtick = true;
                count_backtick = 0;
            } else if tag_backtick && c == '`' {
                if tag_code {
                    html.push_str("</code>");
                    tag_code = false;
                } else if tag_pre {
                    html.push_str("</code></pre>");
                    tag_pre = false;
                    i += 2;
                }

                tag_backtick = false;
                println!("{tag_backtick}");
            }

            if tag_backtick && c != '`' && c != '#' && c != '>' {
                html.push(c);
            }

            if tag_list && !tag_backtick && c != '-' && c != '\n' {
                html.push(c);
            }

            if tag_head && !tag_backtick && c != '#' && c != '\n' && c != '`' {
                html.push(c);
            }

            if tag_blockquote && !tag_backtick && c != '>' && c != '\n' && c != '#' && c != '`' {
                html.push(c);
            }

            if tag_list && c == '\n' {
                html.push_str("</li>");
                tag_list = false;
            }

            if tag_blockquote && c == '\n' {
                html.push_str("</blockquote>");
                tag_blockquote = false;
            }

            if tag_head && c == '\n' {
                if tag_head_h1 {
                    html.push_str("</h1>");
                    tag_head_h1 = false;
                } else if tag_head_h2 {
                    html.push_str("</h2>");
                    tag_head_h2 = false;
                } else if tag_head_h3 {
                    html.push_str("</h3>");
                    tag_head_h3 = false;
                } else if tag_head_h4 {
                    html.push_str("</h4>");
                    tag_head_h4 = false;
                } else if tag_head_h5 {
                    html.push_str("</h5>");
                    tag_head_h5 = false;
                } else if tag_head_h6 {
                    html.push_str("</h6>");
                    tag_head_h6 = false;
                }

                tag_head = false;
            }
        }
        i += 1;
    }

    println!("{}", html);

    write_file("index.html", &html);

    Ok(())
}
