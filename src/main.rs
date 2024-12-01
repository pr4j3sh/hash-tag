fn main() {
    let md = "# Heading 1\n## Heading 2";
    println!("[debug] {md}");
    let mut html = String::from("");
    let mut h1 = String::from("");
    let mut h2 = String::from("");
    let mut heading_count = 0;

    let mut tag_h1 = false;
    let mut tag_h2 = false;

    for i in md.chars() {
        if tag_h1 {
            h1.push(i);
        }
        if tag_h2 {
            h2.push(i);
        }
        if heading_count == 2 && i != '#' {
            tag_h2 = true;
        }
        if heading_count == 1 && i != '#' {
            tag_h1 = true;
        }
        if heading_count > 0 && i == '#' {
            heading_count += 1;
        }
        if i == '#' {
            heading_count = 0;
            tag_h1 = false;
            tag_h2 = false;
            heading_count += 1;
        }
    }

    for i in h1.chars() {
        println!("[debug] {i}");
    }

    // for i in html.chars() {
    //     println!("[debug] {i}");
    // }
}
