struct Markdown {
    text: String,
}

struct Html {
    text: String,
}

enum Element {
    Header(usize, String),
    List(Vec<String>),
    Paragraph(String),
    Bold(String),
    Italic(String),
    Link(String, String),
    Image(String, String),
    Code(String),
    CodeBlock(String),
    Quote(String),
    HorizontalRule,
}

impl Element {
    fn to_html(&self) -> String {
        match self {
            Element::Header(level, text) => format!("<h{}>{}</h{}>", level, text, level),
            Element::List(items) => {
                let items_html = items
                    .iter()
                    .map(|item| format!("<li>{}</li>", item))
                    .collect::<String>();
                format!("<ul>{}</ul>", items_html)
            }
            Element::Paragraph(text) => format!("<p>{}</p>", text),
            Element::Bold(text) => format!("<b>{}</b>", text),
            Element::Italic(text) => format!("<i>{}</i>", text),
            Element::Link(text, url) => format!("<a href=\"{}\">{}</a>", url, text),
            Element::Image(text, url) => format!("<img src=\"{}\" alt=\"{}\" />", url, text),
            Element::Code(text) => format!("<code>{}</code>", text),
            Element::CodeBlock(text) => format!("<pre><code>{}</code></pre>", text),
            Element::Quote(text) => format!("<blockquote>{}</blockquote>", text),
            Element::HorizontalRule => "<hr />".to_string(),
        }
    }
}

impl From<Markdown> for Html {
    fn from(markdown: Markdown) -> Self {
        let mut elements = vec![];
        let mut header_level = 0;
        let mut list_items = vec![];
        let mut is_bold = false;
        let mut is_italic = false;
        let mut is_link = false;
        let mut is_image = false;
        let mut is_code = false;
        let mut is_code_block = false;
        let mut is_quote = false;
        let mut is_horizontal_rule = false;
        let mut is_paragraph = false;
        let mut current_text = String::new();

        for c in markdown.text.chars() {
            match c {
                '#' => {
                    if !current_text.is_empty() {
                        elements.push(Element::Paragraph(current_text.clone()));
                        current_text.clear();
                    }
                    header_level += 1;
                }
                '*' => {
                    if !current_text.is_empty() {
                        elements.push(Element::Paragraph(current_text.clone()));
                        current_text.clear();
                    }
                    if is_bold {
                        elements.push(Element::Bold(current_text.clone()));
                    }
                    is_bold = !is_bold;
                }
                '_' => {
                    if !current_text.is_empty() {
                        elements.push(Element::Paragraph(current_text.clone()));
                        current_text.clear();
                    }
                    if is_italic {
                        elements.push(Element::Italic(current_text.clone()));
                    }
                    is_italic = !is_italic;
                }
                '[' => {
                    if !current_text.is_empty() {
                        elements.push(Element::Paragraph(current_text.clone()));
                        current_text.clear();
                    }
                    is_link = true;
                }
                ']' => {
                    is_link = false;
                }
                '(' => {
                    is_image = true;
                }
                ')' => {
                    is_image = false;
                }
                '`' => {
                    if !current_text.is_empty() {
                        elements.push(Element::Paragraph(current_text.clone()));
                        current_text.clear();
                    }
                    if is_code {
                        elements.push(Element::Code(current_text.clone()));
                    }
                    is_code = !is_code;
                }
                '\n' => {
                    if is_code_block {
                        elements.push(Element::CodeBlock(current_text.clone()));
                        is_code_block = false;
                    } else if is_quote {
                        elements.push(Element::Quote(current_text.clone()));
                        is_quote = false;
                    } else if is_list_item {
                        list_items.push(current_text.clone());
                        current_text.clear();
                    } else if is_paragraph {
                        elements.push(Element::Paragraph(current_text.clone()));
                        current_text.clear();
                        is_paragraph = false;
                    }
                }
                '-' if is_list_item => {
                    list_items.push(current_text.clone());
                    current_text.clear();
                }
                '|' if is_table_separator => {
                    elements.push(Element::Paragraph(current_text.clone()));
                    current_text.clear();
                    is_table_separator = false;
                }
                ':' if is_table_separator => {
                    is_table_separator_left_align = true;
                }
                '-' if is_table_separator_left_align => {
                    is_table_separator_center_align = true;
                    is_table_separator_left_align = false;
                }
                ':' if is_table_separator_center_align => {
                    is_table_separator_right_align = true;
                    is_table_separator_center_align = false;
                }
                ' ' if is_table_separator_right_align => {
                    is_table_separator_right_align = false;
                    is_table_separator = false;
                }
                '>' => {
                    if !current_text.is_empty() {
                        elements.push(Element::Paragraph(current_text.clone()));
                        current_text.clear();
                    }
                    is_quote = true;
                }
                '-' => {
                    if current_text.is_empty() {
                        is_horizontal_rule = true;
                    } else {
                        current_text.push(c);
                    }
                }
                '\t' | ' ' => {
                    if is_code_block {
                        current_text.push(c);
                    } else if is_table_separator {
                        current_text.push(c);
                    } else if is_list_item {
                        current_text.push(c);
                    } else if is_paragraph {
                        current_text.push(c);
                    }
                }
                _ => {
                    if is_link {
                        current_text.push(c);
                    } else if is_image {
                        current_text.push(c);
                    } else if is_code_block {
                        current_text.push(c);
                    } else if is_table_separator {
                        current_text.push(c);
                    } else if is_quote {
                        current_text.push(c);
                    } else if is_list_item {
                        current_text.push(c);
                    } else if is_paragraph {
                        current_text.push(c);
                    } else {
                        is_paragraph = true;
                        current_text.push(c);
                    }
                }
            }
            if is_horizontal_rule {
                elements.push(Element::HorizontalRule);
                current_text.clear();
                is_horizontal_rule = false;
            }
        }
        if !current_text.is_empty() {
            elements.push(Element::Paragraph(current_text));
        }

        if !list_items.is_empty() {
            elements.push(Element::List(list_items));
        }

        Html { elements, header_level }
    } 
}
