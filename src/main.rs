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
          label { r#for: "subfile", "Subtitle file: "}
          input { r#type: "file" , id: "subfile"}
        }
      }
      
    }
    
    
  })
}
