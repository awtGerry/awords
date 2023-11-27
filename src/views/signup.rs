use leptos::*;

use crate::components::db::new_user;

#[allow(unused, non_snake_case)]
pub fn Signup(cx: Scope) -> impl IntoView {
    let email = create_rw_signal(cx, "".to_string());
    let username = create_rw_signal(cx, "".to_string());
    let password = create_rw_signal(cx, "".to_string());
    return view! {
        cx,
        <main class="bg-aw-bg h-screen w-full my-0 mx-auto text-center">
            <p class="opacity-0">"."</p>
            <a href="/" class="text-4xl font-pacifico text-aw-green mt-16 delay-300 block">"AWORDS"</a>
            <h2 class="text-aw-fg font-serif text-2xl my-8 text-center">
                "Nice to meet you."
            </h2>
            <form class="flex flex-col items-center">
                <div class="my-2">
                    <div class="w-full relative inline-flex">
                        <input type="text" placeholder="E-mail address"
                            class="bg-aw-fg font-serif text-lg pl-16 rounded-lg px-4 py-4 ring-1
                            focus:outline-none
                            focus:border-aw-green
                            focus:ring-2
                            focus:ring-aw-green
                            focus:invalid:border-red-500  focus:invalid:ring-red-500"
                            value={email}
                        />
                        <img src="/email.svg" class="absolute leading-none text-center h-full w-12 h-12 px-2 left-0 top-0 flex align-center items-center rounded-xl" />
                    </div>
                </div>
                <div class="my-2">
                    <div class="w-full relative inline-flex">
                        <input type="text" placeholder="Username"
                            class="bg-aw-fg font-serif text-lg pl-16 rounded-lg px-4 py-4 ring-1
                            focus:outline-none
                            focus:border-aw-green
                            focus:ring-2
                            focus:ring-aw-green
                            focus:invalid:border-red-500  focus:invalid:ring-red-500"
                            value={username}
                        />
                        <img src="/user_login.svg" class="absolute leading-none text-center h-full w-12 h-12 px-2 left-0 top-0 flex align-center items-center rounded-xl" />
                    </div>
                </div>
                <div class="mt-2">
                    <div class="w-full relative inline-flex">
                        <input type="password" placeholder="Password"
                            class="bg-aw-fg font-serif text-lg pl-16 rounded-lg px-4 py-4 ring-1
                            focus:outline-none
                            focus:border-aw-green
                            focus:ring-2
                            focus:ring-aw-green
                            focus:invalid:border-red-500
                            focus:invalid:ring-red-500"
                            value={password}
                        />
                        <img src="/lock.svg" class="absolute leading-none text-center h-full w-12 h-12 px-2 left-0 top-0 flex align-center items-center rounded-xl" />
                    </div>
                </div>
                <div class="my-2">
                    <button type="submit"
                        class="bg-aw-green text-aw-bg font-mono font-bold text-normal mt-8 px-8 py-4 rounded-full text-center"
                        on:click=move |_| {
                            let new_user = new_user(email.get(), username.get(), password.get());
                            email.set("".to_string());
                            username.set("".to_string());
                            password.set("".to_string());
                        }
                    >
                        "Create your account"
                    </button>
                </div>
                <div class="my-4">
                    <div class="text-aw-fg text-sm font-bold font-mono">
                        "Already have an account? "
                        <a href="/login" class="text-aw-green-light">
                            " Log in."
                        </a>
                    </div>
                </div>
            </form>
        </main>
    }
}
