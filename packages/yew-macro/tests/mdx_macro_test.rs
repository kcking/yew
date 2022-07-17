use yew_macro::{html, mdx};

#[test]
fn simple() {
    assert_eq!(
        mdx! {
            r#"
        # hi
        [this is a link](google.com)
        "#
        },
        html! {
            <>
            <h1>{"hi"}</h1>
            <a href={"google.com"}>{"this is a link"}</a>
            </>
        }
    );
}
