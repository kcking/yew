use chumsky::prelude::*;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;

pub fn mdx(input: TokenStream) -> TokenStream {
    let parsed = input
        .into_iter()
        .map(|token| match token {
            lit @ TokenTree::Literal(_) => {
                let mdx_str = lit.to_string();
                let mdx_str = mdx_str
                    .strip_prefix("r#\"")
                    .unwrap()
                    .strip_suffix("\"#")
                    .unwrap();
                let parsed = parse_mdx().parse(mdx_str).unwrap();
                // dbg!(&parsed);
                let evaled = parsed.eval_outer();
                dbg!(&evaled.to_string());
                evaled
            }
            _ => panic!("mdx! expected literal"),
        })
        .collect::<TokenStream>();

    parsed
}

fn parse_mdx() -> impl Parser<char, Expr, Error = Simple<char>> {
    let operators = &['(', ')', '[', ']'];

    let text = filter(|c| !operators.contains(c))
        .repeated()
        .at_least(1)
        .collect::<String>();

    let expr = recursive(|expr| {
        let link_text = expr.clone().delimited_by(just('['), just(']'));
        let link_url = text.delimited_by(just('('), just(')'));

        let link = link_text.then(link_url).map(|(text, url)| Expr::Link {
            text: Box::new(text),
            url,
        });

        link.or(text.map(Expr::Text))
    });

    let title = expr
        .clone()
        .repeated()
        .delimited_by(just('#').padded(), text::newline().or(end()))
        .map(|t| Expr::Title(Box::new(Expr::from_list(t))));

    title
        .or(expr)
        .repeated()
        .map(Expr::from_list)
        .then_ignore(end())
}

impl Expr {
    fn eval_outer(&self) -> TokenStream {
        match self {
            Expr::Exprs(_) => format!("<>{}</>", self.eval()).parse().unwrap(),
            _ => self.eval(),
        }
    }

    fn eval(&self) -> TokenStream {
        let evaled: TokenStream = match self {
            Expr::Title(t) => format!("<h1>{}</h1>", t.eval()).parse().unwrap(),
            Expr::Text(t) => format!("{{\"{}\"}}", t).parse().unwrap(),
            Expr::Link { text, url } => {
                let text = text.eval();
                format!("<a href=\"{url}\">{text}</a>").parse().unwrap()
            }
            Expr::Exprs(exprs) => {
                format!("{}", exprs.iter().map(Expr::eval).collect::<TokenStream>())
                    .parse()
                    .unwrap()
            }
            _ => quote! {}.into(),
        };
        dbg!(&evaled.to_string());
        evaled
    }

    fn from_list(v: Vec<Self>) -> Self {
        //  canonicalize single-item list into inner element
        if v.len() == 1 {
            return v.into_iter().next().unwrap();
        }
        Expr::Exprs(v)
    }
}

#[derive(Debug)]
enum Expr {
    Title(Box<Expr>),
    Text(String),
    Exprs(Vec<Expr>),
    Link { text: Box<Expr>, url: String },
    Newline,
}
