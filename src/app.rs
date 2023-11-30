use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::views::awords::Awords;
use crate::views::awords::AwordsLogged;
use crate::views::login::*;
// use crate::views::signup::Todos;
use crate::views::signup::Signup;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/awords.css"/>
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Pacifico&display=swap"/>
        <Link rel="shortcut icon" type_="image/ico" href="/logo.ico"/>
        <Router>
            <Routes>
                <Route path="/" view=Awords/>
                <Route path="/:id" view=AwordsLogged/>
                <Route path="/login" view=Login/>
                <Route path="/signup" view=Signup/>
            </Routes>
        </Router>
    }
}
