use dioxus::prelude::*;

fn main() {
  dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
  let hidden_status = use_state(&cx, || "hidden");
  let subtitle_path = use_state(&cx, || "".to_string());
  let sign = use_state(&cx, || 1);
  let number = use_state(&cx, || 0.0);
  let overwrite_name_bool = use_state(&cx, || false);
  let custom_name_bool = use_state(&cx, || false);
  let custom_output_name = use_state(&cx, || "".to_string());


  cx.render(rsx!{
    style { [rsx!{include_str!("../src/style.css")}].into_iter() }
    div {
      h1 { "Subtitle Modifier"}
      ul {
        li {
          h2 { "Select a subtitle file:" }
          input {
            r#type: "file",
            id: "subfile",
            value: "{subtitle_path}",
            oninput: move |evt| subtitle_path.set(evt.value.clone()),
          }
        }
        li {
          h2 { "Choose seconds modifier:" }
          select {
            id: "sign",
            value: "{sign}",
            oninput: move |evt| {
              if evt.value == "+" {
                sign.set(1);
              } else {
                sign.set(-1);
              }
              
            },
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
            value: "{number}",
            oninput: move |evt| {
              number.set(evt.value.clone().parse::<f64>().unwrap());
            }
          }
        }
        li {
          h2 { "Choose output file:" }
          input {
            r#type: "radio",
            name: "output_name",
            id: "overwrite_name",
            value: "{overwrite_name_bool}",
            onclick: move |_| {
              hidden_status.set("hidden");
              overwrite_name_bool.set(true);
              custom_name_bool.set(false);
            }
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
            value: "{custom_name_bool}",
            onclick: move |_| {
              hidden_status.set("");
              custom_name_bool.set(true);
              overwrite_name_bool.set(false);
            }
          }
          label {
            r#for: "custom_name",
            "Custom name: "
          }
          br {}
          input {
            r#type: "text",
            id: "custom_output_name",
            class: "{hidden_status}",
            size: "75",
            value: "{custom_output_name}",
            oninput: move |evt| {
              custom_output_name.set(evt.value.clone());
            }
          }
        }
        br {}
        button {
          onclick: move |_| {
            submod();
          },
          "Modify"
        }
        br {} br {} br {}
        textarea {
          id: "feedback",
          value: "{custom_output_name}",
          rows: "4",
          cols: "50"
        }
      }
    }
  })
}

fn submod() {

}