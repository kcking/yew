use yew_macro::{html, mdx};

#[test]
fn text() {
    assert_eq!(
        mdx! {
            r#"hi"#
        },
        html! {{"hi"}}
    );
}

#[test]
fn h1() {
    dbg_eq(
        mdx! {
            r#"# hi"#
        },
        html! {
            <h1>{"hi"}</h1>
        },
    );
}

#[test]
fn a() {
    dbg_eq(
        mdx! {
            r#"[this is a link](google.com)"#
        },
        html! {
            <a href="google.com">{"this is a link"}</a>
        },
    )
}

fn dbg_eq<T: std::fmt::Debug>(a: T, b: T) {
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
}
