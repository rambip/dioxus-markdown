#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

use dioxus_markdown::*;

use std::rc::Rc;
use std::collections::BTreeMap;

static MARKDOWN_SOURCE: &str = r#"
## Here is a counter:
<Counter initial="5"/>

## Here is a Box:
<box>

**I am in a blue box !**

</box>
"#;

#[component]
fn Counter(cx: Scope, initial: i32) -> Element {
    let mut count = use_state(cx, || *initial);

    cx.render(rsx!{
        div{
            button {
                onclick: move |_| count-=1,
                "-"
            },
            "{count}",
            button {
                onclick: move |_| count+=1,
                "+"
            }
        }
    })
}

#[component]
fn ColorBox<'a>(cx: Scope, children: Element<'a>) -> Element<'a> {
    cx.render(rsx!{
        div{
            style: "border: 2px solid blue",
            children
        }
    })
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let count: HtmlCallback<MdComponentProps> = Rc::new(
        move |cx, props| cx.render(
            rsx!{Counter {initial: props.get_attribute("initial").unwrap_or_default()}}
        )
    );

    let color_box: HtmlCallback<MdComponentProps> = Rc::new(
        move |cx, props| cx.render(rsx!{ColorBox {props.children}})
    );

    let components: BTreeMap<&'static str, HtmlCallback<MdComponentProps>>
        = BTreeMap::from([
        ("Counter", count),
        ("box", color_box)
    ]);

    cx.render(rsx! {
        h1 {"Source"}
        Markdown {
            src: "```md\n{MARKDOWN_SOURCE}\n``"
        }

        h1 {"Result"}
        Markdown {
            src: MARKDOWN_SOURCE,
            components: components
        }
    })
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
