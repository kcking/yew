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
            pulldown_cmark::Event::Text(txt) => format!("{{\"{}\"}}", txt).parse().unwrap(),
            _ => quote! {}.into(),
        };
        toks.extend(new_toks);
    });

    toks
}
