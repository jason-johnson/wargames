use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
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
//    let (count, set_count) = signal(0);

    view! {
        <div class="container mx-auto">
            <h1 class="text-4xl font-semibold text-center mt-8">Welcome to Wargames</h1>
            <p class="text-center mt-4">This is a collection of wargames.</p>
        </div>
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