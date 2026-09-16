use leptos::html;
use leptos::prelude::*;
use crate::codemirror::{create_editor, EditorView, destroy};

#[component]
pub fn CodeMirror(text: &'static str) -> impl IntoView {
    let div = NodeRef::<html::Div>::new();

    let editor = StoredValue::new(None::<EditorView>);

    Effect::new(move |_| {
        if let Some(parent) = div.get() {
            let cm = create_editor(parent, text);
            editor.set_value(Some(cm));
        }
    });

    on_cleanup(move || {
        if let Some(Some(editor)) = editor.try_get_value() {
            destroy(&editor);
        }
    });

    view! {
        <div
            node_ref=div
            // TODO
            style="height:600px;border:3px solid #444;"
        />
    }
}