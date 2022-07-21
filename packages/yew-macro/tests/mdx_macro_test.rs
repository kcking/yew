use yew_macro::{html, mdx};

#[test]
fn text() {
    assert_eq!(
        mdx! {
            r#"hi"#
        },
        html! {<p>{"hi"}</p>}
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
            <p><a href="google.com">{"this is a link"}</a></p>
        },
    )
}

#[test]
fn nested() {
    dbg_eq(
        mdx! {
            r#"# Wow a [link](google.com) in a title"#
        },
        html! {
            <h1>{"Wow a "}<a href="google.com">{"link"}</a>{" in a title"}</h1>
        },
    )
}

#[test]
fn multiple() {
    dbg_eq(
        mdx! {
            r#"Some text [link](google.com)"#
        },
        html! {
            <p>
            {"Some text "}
            <a href="google.com">{"link"}</a>
            </p>
        },
    )
}

#[test]
fn multiline_text() {
    dbg_eq(
        mdx! { r#"this is some text that
        spans multiple lines"#
        },
        html! {<p>{"this is some text that"}{" "}{"spans multiple lines"}</p>},
    )
}

#[test]
fn multiline_link() {
    dbg_eq(
        mdx! { r#"[this is a
        multiline link wow](google.com)"#},
        html! {
            <p>
            <a href="google.com">{"this is a"} {" "} {"multiline link wow"}</a>
            </p>
        },
    )
}

#[test]
fn basic_code() {
    dbg_eq(
        mdx! {r#"here is some `inline code` ooo"#},
        html! {
            <p>
            {"here is some "}<code>{"inline code"}</code>{" ooo"}
            </p>
        },
    );
    dbg_eq(
        mdx! {r#"# header `inline code` ooo"#},
        html! {
            <h1>
                {"header "}<code>{"inline code"}</code>{" ooo"}
            </h1>
        },
    );
    dbg_eq(
        mdx! {r#"# header [link `inline code`](google.com) ooo"#},
        html! {
            <h1>
                {"header "}<a href="google.com">{"link "}<code>{"inline code"}</code></a>{" ooo"}
            </h1>
        },
    );
}

fn dbg_eq<T: std::fmt::Debug>(a: T, b: T) {
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
}
