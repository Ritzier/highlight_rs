use highlight_rs::Language;
use leptos::prelude::*;
use strum::IntoEnumIterator;

#[component]
pub fn DemoPage() -> impl IntoView {
    let (language, set_language) = signal(Language::Rust);
    let (code, set_code) = signal(String::new());

    view! {
        <div class="demo">
            <header>
                <span>"Highlight"</span>

                <SelectLangauage language set_language />
            </header>

            <main>
                <div class="editor">
                    <EditorView code set_code />
                </div>

                <div class="highlight">
                    <HighlightView code language />
                </div>
            </main>
        </div>
    }
}

#[component]
fn SelectLangauage(
    language: ReadSignal<Language>,
    set_language: WriteSignal<Language>,
) -> impl IntoView {
    view! {
        <select class="language-select" prop:value=move || { language.get().to_string() }>
            {Language::iter()
                .map(|language| {
                    view! {
                        <option on:click=move |_| {
                            set_language.set(language.clone());
                        }>{language.to_string()}</option>
                    }
                })
                .collect_view()}
        </select>
    }
}

#[component]
fn EditorView(code: ReadSignal<String>, set_code: WriteSignal<String>) -> impl IntoView {
    view! {
        <textarea
            prop:value=code
            on:input=move |ev| {
                set_code.set(event_target_value(&ev));
            }
        />
    }
}

#[component]
fn HighlightView(code: ReadSignal<String>, language: ReadSignal<Language>) -> impl IntoView {
    let (inner, set_inner) = signal(String::new());
    Effect::new(move |_| {
        let result = highlight_rs::highlight(&code.read(), &language.read().to_string());
        set_inner.set(result);
    });

    view! {
        <pre class="code">
            <code class="hljs" inner_html=inner></code>
        </pre>
    }
}
