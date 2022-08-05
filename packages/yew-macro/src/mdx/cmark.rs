use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro_error::ResultExt;
use pulldown_cmark::{Event, Options, Parser, Tag};
use quote::quote;

//  styling idea:
//  caller passes in mapping of tag to yew component name
//      caller can implement component however they want, it can take children
//  instead of rendering <p> we render <SpecialP>
//  problem: has to be specified at every call-site, and it's verbose because it has to be the input
// to the proc macro
//
//  take 2: use yew dynamic tags to lookup in a named or global style array,
//  including fallbacks to defaults. how to control whether style is applied at
//  all? could have 2 different mdx macros, mdx/mdxs (global style) / mdxss
//  (user-specified name of style map)
//  Disadvantage: yew component name validation done at runtime
//  instead of mapping to component name, could just map to Fn(children)-> Html
//  dealbreaker?: do dynamic tags work with components anyways??

//  map static tag to dynamic tag, falling back to the given tag
#[derive(PartialEq)]
enum Side {
    Start,
    End,
}
fn dyn_tag(tag: &str, side: Side) -> TokenStream {
    let quoted_tag = "\"".to_string() + tag + "\"";
    match side {
        Side::Start => {
            "<@{".to_string()
                + &quote! {
                    if MDX_STYLE.contains_key(#tag){
                        MDX_STYLE.get(#tag).unwrap().clone()
                    } else {
                        #tag
                    }
                }
                .to_string()
                + "}>"
        }
        Side::End => "</@>".to_string(),
    }
    .parse()
    .unwrap()
}

pub fn parse_commonmark(input: &str) -> TokenStream {
    let parser = Parser::new_ext(input, Options::all());

    let mut toks = TokenStream::new();
    toks.extend::<TokenStream>("<>".parse().unwrap());

    parser.for_each(|evt| {
        // dbg!(&evt);
        let new_toks: TokenStream = match evt {
            Event::Start(tag) => match tag {
                Tag::Paragraph => dyn_tag("p", Side::Start),
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
                Tag::Paragraph => dyn_tag("p", Side::End),
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
