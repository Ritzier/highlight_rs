use highlight_rs::Language;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{LazyRoute, lazy_route};
use strum::IntoEnumIterator;

pub struct DemoPage {
    data: DemoPageData,
}

struct DemoPageData {
    language: ReadSignal<Language>,
    set_language: WriteSignal<Language>,
    code: ReadSignal<String>,
    set_code: WriteSignal<String>,
}

impl Default for DemoPageData {
    fn default() -> Self {
        let (language, set_language) = signal(Language::Rust);
        let (code, set_code) = signal(String::new());

        Self {
            language,
            set_language,
            code,
            set_code,
        }
    }
}

#[lazy_route]
impl LazyRoute for DemoPage {
    fn data() -> Self {
        Self {
            data: DemoPageData::default(),
        }
    }

    fn view(this: Self) -> AnyView {
        let DemoPageData {
            code,
            set_code,
            language,
            set_language,
        } = this.data;

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
        .into_any()
    }
}

#[lazy]
fn highlight_code(code: String, language: String) -> String {
    highlight_rs::highlight(&code, &language)
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
        let code_val = code.get();
        let lang_val = language.read().to_string();

        if !code_val.is_empty() {
            spawn_local(async move {
                let result = highlight_code(code_val, lang_val).await;
                set_inner.set(result);
            });
        }
    });
    view! {
        <pre class="code">
            <code class="hljs" inner_html=inner></code>
        </pre>
    }
}
