use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[server]
pub async fn save_user(
    number: i32,
) -> Result<String, ServerFnError> {
    let mut conn = crate::models::get_connection()
        .map_err(ServerFnError::ServerError)?;

    let user = crate::models::UserForm {
        pseudo: format!("User {}", number),
    };
    user.insert(&mut conn)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(format!("Here, {user_pseudo}!", user_pseudo = user.pseudo))
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/pkg/not_management.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (server, set_server) = create_signal("".to_string());

    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| {
                    spawn_local(async move {
                        let user_pseudo = save_user(count()).await.unwrap();
                        set_count.update(|count| *count += 1);
                        set_server.update(|server| *server = user_pseudo);
                    });
                }
            >
                "Something's here | "
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
                " | Some more text"
            </button>
            <p class="px-10 pb-10 text-left">This is a server response: {server}</p>
        </main>
    }
}