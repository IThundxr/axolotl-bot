import { EditorView, basicSetup } from "https://esm.sh/codemirror";

export function createEditor(parent, text) {
    return new EditorView({
        parent,
        doc: text,
        extensions: [basicSetup]
    });
}

export function getText(view) {
    return view.state.doc.toString();
}

export function setText(view, text) {
    view.dispatch({
        changes: {
            from: 0,
            to: view.state.doc.length,
            insert: text
        }
    });
}

export function destroy(view) {
    view.destroy();
}