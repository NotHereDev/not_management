use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[server]
pub async fn add_user(
    pseudo: String,
) -> Result<String, ServerFnError> {
    let mut conn = crate::models::get_connection()
        .map_err(ServerFnError::ServerError)?;

    let user = crate::models::UserForm {
        pseudo,
    };
    user.insert(&mut conn)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(format!("Here, {user_pseudo}!", user_pseudo = user.pseudo))
}

#[server]
pub async fn get_users() -> Result<Vec<crate::models::User>, ServerFnError> {
    let mut conn = crate::models::get_connection()
        .map_err(ServerFnError::ServerError)?;

    let users = crate::models::User::all(&mut conn)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(users)
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
    let add_user = create_server_multi_action::<AddUser>();

    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
                " | Some more text"
            </button>
            <MultiActionForm
                action=add_user
            >
                <label>
                    "Add User"
                    <input type="text" name="pseudo"/>
                </label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
        </main>
    }
}