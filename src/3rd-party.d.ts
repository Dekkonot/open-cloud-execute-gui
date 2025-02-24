export type ThemeType = "light" | "dark" | "hotdog";

export function initialize_editor(theme: ThemeType);

export function set_editor_theme(theme: ThemeType);

export function set_editor_text(text: string);

export function get_editor_text(): string;

export function display_return(value: any);
