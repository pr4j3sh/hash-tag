use wasm_bindgen::prelude::*;

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

    if !line.is_empty() {
        lines.push(line);
    }

    lines
}

fn close_list_tags(tag_ul: &mut bool, tag_ol: &mut bool, html: &mut String) {
    if *tag_ul {
        html.push_str("</ul>");
        *tag_ul = false;
    }
    if *tag_ol {
        html.push_str("</ol>");
        *tag_ol = false;
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

                if !url.is_empty() {
                    line.push_str(&format!(
                        "<a class=\"link\" href=\"{url}\" target=\"_blank\">{text}</a>"
                    ));
                } else {
                    line.push('[');
                    line.push_str(&text);
                    line.push(']');
                }
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

#[wasm_bindgen]
pub fn parse(md: String) -> String {
    let mut html = String::from("");

    let lines: Vec<String> = get_lines(&md);

    let mut tag_pre = false;
    let mut tag_ul = false;
    let mut tag_ol = false;

    for l in lines.iter() {
        for (j, c) in l.chars().enumerate() {
            if j == 0 && c == '#' {
                // heading
                close_list_tags(&mut tag_ul, &mut tag_ol, &mut html);
                let count_hash = l.chars().filter(|&c| c == '#').count();
                let line = process_line(&l[count_hash + 1..]);
                let line = format!("<h{count_hash}>{line}</h{count_hash}>");
                html.push_str(&line);
            } else if j == 0 && c == '>' {
                // blockquote
                close_list_tags(&mut tag_ul, &mut tag_ol, &mut html);
                let line = process_line(&l[2..]);
                let line = format!("<blockquote>{line}</blockquote>");
                html.push_str(&line);
            } else if j == 0 && c == '-' {
                // unordered list
                if l.trim().chars().all(|ch| ch == '-') {
                    html.push_str("<hr>");
                    break;
                }
                if !tag_ul {
                    html.push_str("<ul>");
                }
                tag_ul = true;
                let line = process_line(&l[2..]);
                let line = format!("<li>{line}</li>");
                html.push_str(&line);
            } else if j == 0 && c.is_ascii_digit() {
                // ordered list
                if !tag_ol {
                    html.push_str("<ol>");
                }
                tag_ol = true;
                let line = process_line(&l[3..]);
                let line = format!("<li>{line}</li>");
                html.push_str(&line);
            } else if j == 0 && c == '`' {
                // codeblock
                close_list_tags(&mut tag_ul, &mut tag_ol, &mut html);
                let count_backtick = l.chars().filter(|&c| c == '`').count();
                if count_backtick == 3 {
                    tag_pre = !tag_pre;
                    if tag_pre {
                        html.push_str("<pre>");
                    } else {
                        html.push_str("</pre>");
                    }
                } else if count_backtick % 2 == 0 {
                    let line = process_line(l);
                    let line = format!("<p>{line}</p>");
                    html.push_str(&line);
                }
            } else if j == 0 && tag_pre {
                // paragraph
                let line = format!("<code>{l}</code>\n");
                html.push_str(&line);
            } else if j == 0 && !tag_pre {
                // paragraph
                close_list_tags(&mut tag_ul, &mut tag_ol, &mut html);
                let line = process_line(l);
                let line = format!("<p>{line}</p>");
                html.push_str(&line);
            }
        }
    }

    html
}
