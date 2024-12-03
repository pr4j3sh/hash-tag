fn main() {
    let md = String::from(
        "# Title `okay`\nthis is `crazy`\n## Title 2\n```\nvar j = 4;\n```\n### title 3\n#### title 4\n##### Title 5\n###### Title 6\n",
    );
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
                }

                tag_backtick = false;
            }

            if tag_backtick && c != '`' {
                html.push(c);
            }

            if tag_head && !tag_backtick && c != '#' && c != '\n' && c != '`' {
                html.push(c);
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

    for (i, c) in html.chars().enumerate() {
        println!("{:4} | {}", i, c);
    }
    println!("{}", html);
}
