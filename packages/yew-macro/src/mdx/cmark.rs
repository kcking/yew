use proc_macro::TokenStream;
use pulldown_cmark::{Options, Parser, Tag};
use quote::quote;

pub fn parse_commonmark(input: &str) -> TokenStream {
    let parser = Parser::new_ext(input, Options::all());

    let mut toks = TokenStream::new();

    parser.for_each(|evt| {
        dbg!(&evt);
        let new_toks: TokenStream = match evt {
            pulldown_cmark::Event::Start(Tag::Heading(lvl, ..)) => {
                format!("<{}>", lvl).parse().unwrap()
            }
            pulldown_cmark::Event::End(Tag::Heading(lvl, ..)) => {
                format!("</{}>", lvl).parse().unwrap()
            }
            pulldown_cmark::Event::Start(Tag::Paragraph) => "<p>".parse().unwrap(),
            pulldown_cmark::Event::End(Tag::Paragraph) => "</p>".parse().unwrap(),
            pulldown_cmark::Event::Start(Tag::Link(_type, url, _title)) => {
                format!("<a href=\"{}\">", url).parse().unwrap()
            }
            pulldown_cmark::Event::End(Tag::Link(..)) => "</a>".parse().unwrap(),
            pulldown_cmark::Event::Text(txt) => format!("{{\"{}\"}}", txt).parse().unwrap(),
            pulldown_cmark::Event::Code(code) => {
                format!("<code>{{\"{}\"}}</code>", code).parse().unwrap()
            }
            pulldown_cmark::Event::SoftBreak => "{{\" \"}}".parse().unwrap(),
            _ => quote! {}.into(),
        };
        toks.extend(new_toks);
    });

    toks
}
