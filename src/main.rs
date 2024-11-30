fn main() {
    let html = "<h1>heading 1</h1>\n<h2>heading 2</h2>            <h3>heading 3</h3>";
    let mut md = String::from("");
    let mut start_tag = String::from("");
    println!("[debug] {}", html);
    let mut start_tag_starts = false;
    let mut start_tag_ends = false;
    let mut end_tag_starts = false;
    let mut end_tag_ends = true;

    for i in html.chars() {
        if end_tag_starts && i == '>' {
            end_tag_starts == false;
            end_tag_ends = true;
        }
        if start_tag_ends {
            if i == '<' {
                start_tag_ends = false;
                end_tag_starts = true;
            } else {
                md.push(i);
            }
        }
        if start_tag_starts {
            if i == '>' {
                println!("[debug] {}", start_tag);
                start_tag_ends = true;
                start_tag_starts = false;
                end_tag_ends = false;
            } else {
                start_tag.push(i);
            }
        }
        if i == '<' && end_tag_ends {
            start_tag_starts = true;
        }
    }

    println!("[debug] {}", md);
}
