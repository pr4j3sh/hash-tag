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
    let md = read_file("test.md")?;

    let lines: Vec<String> = get_lines(&md);

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
    let mut tag_ul = false;
    let mut tag_ol = false;
    let mut tag_p = false;

    for (i, l) in lines.iter().enumerate() {
        let n = l.len();
        print!("{:4} | {:4} | ", i, n);
        for (j, c) in l.chars().enumerate() {
            if j == 0 && c == '#' {
                // heading
                count_hash = l.chars().filter(|&c| c == '#').count();
                print!("h{count_hash}");
            } else if j == 0 && c == '>' {
                // blockquote
                print!("blockquote");
            } else if j == 0 && c == '-' {
                // list
                print!("ul ");
            } else if j == 0 && c.is_ascii_digit() {
                // codeblock
                print!("ol ");
            } else if j == 0 && c == '`' {
                // codeblock
                count_backtick = l.chars().filter(|&c| c == '`').count();
                if count_backtick == 3 {
                    tag_pre = !tag_pre;
                    print!("pre | {tag_pre}");
                } else if count_backtick % 2 == 0 {
                    print!("p  ");
                }
            } else if j == 0 && tag_pre {
                // paragraph
                print!("code");
            } else if j == 0 && !tag_pre {
                // paragraph
                print!("p  ");
            }
        }
        println!(" | {l}");
    }

    let _ = write_file("index.html", &html);

    Ok(())
}
