fn main() {
    let md = String::from(
        "# Title `1`\n## Title 2\n### title 3\n#### title 4\n##### Title 5\n###### Title 6\n",
    );
    let mut html = String::from("");

    let mut count_hash = 0;

    let mut i = 0;
    let size = md.len();

    let mut tag_head = false;
    let mut tag_head_h1 = false;
    let mut tag_head_h2 = false;
    let mut tag_head_h3 = false;
    let mut tag_head_h4 = false;
    let mut tag_head_h5 = false;
    let mut tag_head_h6 = false;

    while i < size {
        if let Some(c) = md.chars().nth(i) {
            println!("{:2} | {}", i, c);
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

            if tag_head && c != '#' && c != '\n' {
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

            i += 1;
        }
    }

    for (i, c) in html.chars().enumerate() {
        println!("{:2} | {}", i, c);
    }
    println!("{}", html);
}
