use proc_macro::TokenStream;
use proc_macro_error::ResultExt;
use pulldown_cmark::{Event, Options, Parser, Tag};
use quote::quote;

pub fn parse_commonmark(input: &str) -> TokenStream {
    let parser = Parser::new_ext(input, Options::all());

    let mut toks = TokenStream::new();
    toks.extend::<TokenStream>("<>".parse().unwrap());

    parser.for_each(|evt| {
        dbg!(&evt);
        let new_toks: TokenStream = match evt {
            Event::End(Tag::Heading(lvl, ..)) => format!("</{}>", lvl).parse().unwrap(),
            Event::End(Tag::Paragraph) => "</p>".parse().unwrap(),
            Event::Start(tag) => match tag {
                Tag::Paragraph => "<p>".parse().unwrap(),
                Tag::Heading(lvl, ..) => format!("<{}>", lvl).parse().unwrap(),
                Tag::BlockQuote => "<blockquote>".parse().unwrap(),
                Tag::CodeBlock(kind) => match kind {
                    pulldown_cmark::CodeBlockKind::Indented => {
                        format!("<pre><code>").parse().unwrap()
                    }
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        format!(r#"<pre><code class="language-{}">"#, lang)
                            .parse()
                            .unwrap()
                    }
                },
                Tag::List(_) => "<ul>".parse().unwrap(),
                Tag::Item => "<li>".parse().unwrap(),
                Tag::FootnoteDefinition(_) => todo!(),
                Tag::Table(_) => todo!(),
                Tag::TableHead => todo!(),
                Tag::TableRow => todo!(),
                Tag::TableCell => todo!(),
                Tag::Emphasis => "<em>".parse().unwrap(),
                Tag::Strong => "<strong>".parse().unwrap(),
                Tag::Strikethrough => "<s>".parse().unwrap(),
                Tag::Link(_type, url, _title) => format!("<a href=\"{}\">", url).parse().unwrap(),
                Tag::Image(_type, url, title) => format!(r#"<img src="{url}" title="{title}"/>"#)
                    .parse()
                    .unwrap(),
            },
            Event::End(tag) => match tag {
                Tag::Paragraph => "</p>".parse().unwrap(),
                Tag::Heading(lvl, ..) => format!("</{}>", lvl).parse().unwrap(),
                Tag::BlockQuote => "</blockquote>".parse().unwrap(),
                Tag::CodeBlock(_) => format!("</code></pre>").parse().unwrap(),
                Tag::List(_) => "</ul>".parse().unwrap(),
                Tag::Item => "</li>".parse().unwrap(),
                Tag::FootnoteDefinition(_) => todo!(),
                Tag::Table(_) => todo!(),
                Tag::TableHead => todo!(),
                Tag::TableRow => todo!(),
                Tag::TableCell => todo!(),
                Tag::Emphasis => "</em>".parse().unwrap(),
                Tag::Strong => "</strong>".parse().unwrap(),
                Tag::Strikethrough => "</s>".parse().unwrap(),
                Tag::Link(_type, _url, _title) => "</a>".parse().unwrap(),
                Tag::Image(_type, _url, _title) => "".parse().unwrap(),
            },
            Event::Text(txt) => format!("{{r###\"{}\"###}}", txt).parse().unwrap(),
            Event::Code(code) => format!("<code>{{r###\"{}\"###}}</code>", code)
                .parse()
                .unwrap(),
            Event::Rule => "<hr />".parse().unwrap(),
            Event::SoftBreak => "{{\" \"}}".parse().unwrap(),
            Event::Html(html) => html.parse().unwrap(),
            _ => quote! {}.into(),
        };
        toks.extend(new_toks);
    });

    toks.extend::<TokenStream>("</>".parse().unwrap());

    toks
}
