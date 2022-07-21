use proc_macro::TokenStream;
use pulldown_cmark::{Event, Options, Parser, Tag};
use quote::quote;

pub fn parse_commonmark(input: &str) -> TokenStream {
    let parser = Parser::new_ext(input, Options::all());

    let mut toks = TokenStream::new();

    parser.for_each(|evt| {
        dbg!(&evt);
        let new_toks: TokenStream = match evt {
            Event::Start(Tag::Heading(lvl, ..)) => format!("<{}>", lvl).parse().unwrap(),
            Event::End(Tag::Heading(lvl, ..)) => format!("</{}>", lvl).parse().unwrap(),
            Event::Start(Tag::Paragraph) => "<p>".parse().unwrap(),
            Event::End(Tag::Paragraph) => "</p>".parse().unwrap(),
            Event::Start(Tag::Link(_type, url, _title)) => {
                format!("<a href=\"{}\">", url).parse().unwrap()
            }
            Event::End(Tag::Link(..)) => "</a>".parse().unwrap(),
            Event::Start(Tag::List(_)) => "<ul>".parse().unwrap(),
            Event::End(Tag::List(_)) => "</ul>".parse().unwrap(),
            Event::Start(Tag::Item) => "<li>".parse().unwrap(),
            Event::End(Tag::Item) => "</li>".parse().unwrap(),
            Event::Text(txt) => format!("{{\"{}\"}}", txt).parse().unwrap(),
            Event::Code(code) => format!("<code>{{\"{}\"}}</code>", code).parse().unwrap(),
            Event::SoftBreak => "{{\" \"}}".parse().unwrap(),
            _ => quote! {}.into(),
        };
        toks.extend(new_toks);
    });

    toks
}
