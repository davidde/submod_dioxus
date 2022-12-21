use dioxus::prelude::*;

fn main() {
  dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
  cx.render(rsx!{
    style { [include_str!("../src/style.css")] }
    div {
      h1 { "Subtitle Modifier"}
      ul {
        li {
          h2 { "Select a subtitle file:" }
          input { r#type: "file" , id: "subfile"}
        }
        li {
          h2 { "Choose seconds modifier:" }
          select {
            id: "sign",
            option {
              value: "+",
              "+"
            }
            option {
              value: "-",
              "-"
            }
          }
          input {
            r#type: "number",
            min: "0",
            step: "0.01",
          }
        }
        li {
          h2 { "Choose output file:" }
          input {
            r#type: "radio",
            name: "output_name",
            id: "overwrite_name",
            value: "overwrite",
          }
          label {
            r#for: "overwrite_name",
            "Overwrite input file"
          }
          br {}
          input {
            r#type: "radio",
            name: "output_name",
            id: "custom_name",
            value: "custom",
          }
          label {
            r#for: "custom_name",
            "Custom name: "
          }
          br {}
          input {
            r#type: "text",
            id: "filename",
            class: "hidden",
            size: "75"
          }
        }
      }
    }
  })
}
