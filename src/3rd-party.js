const default_to_light = window.matchMedia(
    "(prefers-color-scheme: light)"
).matches;

let editor;
export function initialize_editor(theme) {
    if (!editor) {
        editor = CodeMirror.fromTextArea(
            document.getElementById("script-input"),
            {
                theme: theme,
                mode: "luau",
                matchBrackets: true,
                lineNumbers: true,
                smartIndent: true,
                indentWithTabs: false,
                indentUnit: 2,
            }
        );
    }
}

export function set_editor_theme(theme) {
    editor.setOption("theme", theme);
}

export function set_editor_text(text) {
    editor.setValue(text);
}

export function get_editor_text() {
    return editor.getValue();
}

export function display_return(value) {
    const code = document.createElement("code");
    // Did you know: you can XSS yourself with this?
    code.textContent = JSON.stringify(value, null, "  ");
    hljs.highlightElement(code);
    return_values.appendChild(code);
}
