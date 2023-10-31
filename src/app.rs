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

#[server]
pub async fn delete_user(
    id: i32,
) -> Result<(), ServerFnError> {
    let mut conn = crate::models::get_connection()
        .map_err(ServerFnError::ServerError)?;

    crate::models::User::delete(&mut conn, id)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
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
    let add_user = create_server_multi_action::<AddUser>();
    let delete_user = create_server_action::<DeleteUser>();
    let submissions = add_user.submissions();

    // list of users is loaded from the server in reaction to changes
    let users = create_resource(
        move || (add_user.version().get(), delete_user.version().get()),
        move |_| get_users(),
    );

    view! {
        <div>
            <MultiActionForm action=add_user>
                <label>
                    "Add a User"
                    <input type="text" name="pseudo"/>
                </label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
            <Transition fallback=move || view! {<p>"Loading..."</p> }>
                {move || {
                    let existing_users = {
                        move || {
                            users.get()
                                .map(move |users| match users {
                                    Err(e) => {
                                        view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                    }
                                    Ok(users) => {
                                        if users.is_empty() {
                                            view! { <p>"No users were found."</p> }.into_view()
                                        } else {
                                            users
                                                .into_iter()
                                                .map(move |user| {
                                                    view! {

                                                        <li>
                                                            {user.pseudo}
                                                            <ActionForm action=delete_user>
                                                                <input type="hidden" name="id" value={user.id}/>
                                                                <input type="submit" value="X"/>
                                                            </ActionForm>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };

                    let pending_users = move || {
                        submissions
                            .get()
                            .into_iter()
                            .filter(|submission| submission.pending().get())
                            .map(|submission| {
                                view! {

                                    <li class="pending">{move || submission.input.get().map(|data| data.pseudo) }</li>
                                }
                            })
                            .collect_view()
                    };

                    view! {
                        <ul>
                            {existing_users}
                            {pending_users}
                        </ul>
                    }
                }
            }
            </Transition>
        </div>
    }
}