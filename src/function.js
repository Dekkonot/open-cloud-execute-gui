const default_to_light = window.matchMedia(
    "(prefers-color-scheme: light)"
).matches;

const initial_value =
    'print("Hello World!")\n\nreturn "This is an example script."';

const output = document.getElementById("output");
const return_values = document.getElementById("return-values");
const wordwrap_toggle = document.getElementById("wordwrap-toggle");
const theme_toggle = document.getElementById("theme-toggle");

if (!default_to_light) {
    document.body.dataset.theme = "dark";
}

const editor = CodeMirror.fromTextArea(
    document.getElementById("script-input"),
    {
        theme: default_to_light ? "light" : "dark",
        mode: "luau",
        matchBrackets: true,
        lineNumbers: true,
        smartIndent: true,
        indentWithTabs: false,
        indentUnit: 2,
    }
);
editor.setValue(initial_value);

export function script_editor_content() {
    return editor.getValue();
}

export function display_return(value) {
    return_values.replaceChildren();
    const code = document.createElement("code");
    // Did you know: you can XSS yourself with this?
    code.textContent = JSON.stringify(value, null, "  ");
    hljs.highlightElement(code);
    return_values.appendChild(code);
}

function toggle_output_wrap() {
    if (wordwrap_toggle.checked) {
        output.className = "";
    } else {
        output.className = "no-wrap";
    }
}

let is_light = default_to_light;
function toggle_theme() {
    if (is_light) {
        editor.setOption("theme", "dark");
        document.body.dataset.theme = "dark";
        is_light = false;
    } else {
        editor.setOption("theme", "light");
        document.body.dataset.theme = "light";
        is_light = true;
    }
}

window.addEventListener("DOMContentLoaded", () => {
    theme_toggle.onclick = function () {
        toggle_theme();
    };

    // This will be our little secret. :-)
    theme_toggle.addEventListener("contextmenu", (event) => {
        editor.setOption("theme", "hotdog");
        document.body.dataset.theme = "hotdog";
        is_light = false;
        event.preventDefault();
    });

    wordwrap_toggle.onclick = function () {
        toggle_output_wrap();
    };
});
