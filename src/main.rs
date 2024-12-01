fn main() {
    let md = String::from("# Heading 1\n#### Heading 4\n## Heading 2");

    let mut count_hash = 0;
    let mut flag_h1 = false;
    let mut flag_h2 = false;
    let mut flag_h3 = false;
    let mut flag_h4 = false;
    let mut flag_h5 = false;
    let mut flag_h6 = false;
    let mut text_h1 = String::from("");
    let mut text_h2 = String::from("");
    let mut text_h3 = String::from("");
    let mut text_h4 = String::from("");
    let mut text_h5 = String::from("");
    let mut text_h6 = String::from("");

    for i in md.chars() {
        print!("|");
        print!("{i}");

        // ---------------- headings -------------------
        if flag_h1 {
            if i == '\n' {
                flag_h1 = false;
            } else {
                text_h1.push(i);
            }
        }
        if flag_h2 {
            if i == '\n' {
                flag_h2 = false;
            } else {
                text_h2.push(i);
            }
        }
        if flag_h3 {
            if i == '\n' {
                flag_h3 = false;
            } else {
                text_h3.push(i);
            }
        }
        if flag_h4 {
            if i == '\n' {
                flag_h4 = false;
            } else {
                text_h4.push(i);
            }
        }
        if flag_h5 {
            if i == '\n' {
                flag_h5 = false;
            } else {
                text_h5.push(i);
            }
        }
        if flag_h6 {
            if i == '\n' {
                flag_h6 = false;
            } else {
                text_h6.push(i);
            }
        }
        if i == '#' {
            count_hash += 1;
        }
        if count_hash > 0 && i == ' ' {
            if count_hash == 1 {
                flag_h1 = true;
            }
            if count_hash == 2 {
                flag_h2 = true;
            }
            if count_hash == 3 {
                flag_h3 = true;
            }
            if count_hash == 4 {
                flag_h4 = true;
            }
            if count_hash == 5 {
                flag_h5 = true;
            }
            if count_hash == 6 {
                flag_h6 = true;
            }
            count_hash = 0;
        }
        // ---------------- headings -------------------

        print!("|");
    }

    let tag_h1 = format!("<h1>{text_h1}</h1>");
    let tag_h2 = format!("<h2>{text_h2}</h2>");
    let tag_h3 = format!("<h3>{text_h3}</h3>");
    let tag_h4 = format!("<h4>{text_h4}</h4>");
    let tag_h5 = format!("<h5>{text_h5}</h5>");
    let tag_h6 = format!("<h6>{text_h6}</h6>");
    println!("\nh1 - {}", tag_h1);
    println!("h2 - {}", tag_h2);
    println!("h3 - {}", tag_h3);
    println!("h4 - {}", tag_h4);
    println!("h5 - {}", tag_h5);
    println!("h6 - {}", tag_h6);
}
