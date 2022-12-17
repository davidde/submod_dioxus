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
          h3 { "Select a subtitle file:" }
          input { r#type: "file" , id: "subfile"}
        }
        li {
          h3 { "Choose seconds modifier:" }
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
      }
      
    }
    
    
  })
}
