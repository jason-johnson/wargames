use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/wargames.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
          <header class="bg-teal-800 px-4 text-zinc-100">
            <nav class="flex justify-between mb-6">
              <a href="/"><span class="text-xl font-semibold">Wargames</span></a>
              <div class="flex items-center space-x-4">
                <a href="/games">Games</a>
              </div>
            </nav>
          </header>
          <main>
            // all our routes will appear inside <main>
            <Routes fallback=|| "Not found.">
              <Route path=path!("/") view=Home/>
              <Route path=path!("/games") view=Games/>
            </Routes>
          </main>
      </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, _set_count) = signal(0);

    view! {
        <div class="container mx-auto">
            <h1 class="text-4xl font-semibold text-center mt-8">Welcome to Wargames</h1>
            <p class="text-center mt-4">This is a collection of wargames.</p>
        </div>
        <div>Count: {count}</div>
    }
}

#[component]
fn Games() -> impl IntoView {
    view! {
        <div class="container mx-auto">
            <h1 class="text-4xl font-semibold text-center mt-8">Games</h1>
            <p class="text-center mt-4">Here are the games you can play.</p>
        </div>
    }
}