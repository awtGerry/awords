use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::views::awords::Awords;
use crate::views::login::Login;
// use crate::views::signup::Todos;
use crate::views::signup::Signup;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Pacifico&display=swap"/>
        <Link rel="shortcut icon" type_="image/ico" href="/logo.ico"/>
        <Router>
            <Routes>
                <Route path="/" view=Awords/>
                <Route path="/login" view=Login/>
                <Route path="/signup" view=Signup/>
            </Routes>
        </Router>
    }
}
