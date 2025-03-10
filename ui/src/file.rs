use dioxus::prelude::*;

const FILE_CSS: Asset = asset!("/assets/styling/file.css");

/// File component that demonstrates fullstack server functions.
#[component]
pub fn File() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: FILE_CSS }
        div {
            id: "file",
            h4 { "ServerFn File" }
            input {
                placeholder: "Type filename to get contents...",
                oninput:  move |event| async move {
                    let data = server::get_file_contents(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server file contents: "
                    i { "{response}" }
                }
            }
        }
    }
}
