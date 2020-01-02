use dominator::{html, Dom, with_node};

fn get_dom_tree() -> Dom {
    html!("div", {
        .with_node!(element => {
            .new(element)
        })
    })
}