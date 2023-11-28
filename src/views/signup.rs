use leptos::*;
use leptos_router::*;

use crate::components::db::*;

#[allow(unused, non_snake_case)]
pub fn Signup() -> impl IntoView {
    let new_user = create_server_multi_action::<AddUser>();

    let email = create_rw_signal("".to_string());
    let username = create_rw_signal("".to_string());
    let password = create_rw_signal("".to_string());

    let navigate = leptos_router::use_navigate();
    let mut err = create_rw_signal(false);

    view! {
        <div class="bg-aw-bg h-screen w-full my-0 mx-auto text-center">
            <p class="opacity-0">"."</p>
            <a href="/" class="text-4xl font-pacifico text-aw-green mt-16 delay-300 block">"AWORDS"</a>
            <h2 class="text-aw-fg font-serif text-2xl my-8 text-center">
                "Nice to meet you."
            </h2>
            <MultiActionForm
                on:submit=move |ev| {
                    let user = AddUser::from_event(&ev).expect("could not get user the user");
                    if user.email == "" || user.username == "" || user.password == "" {
                        ev.prevent_default();
                        err.set(true);
                    } else {
                        err.set(false);
                        // Go to the home page
                        // let s = format!("/{}", user.id);
                        // navigate(&s, Default::default());
                        navigate("/login", Default::default());
                    }
                }
                action=new_user
            >
                <div class="flex flex-col items-center">
                    <div class="my-2">
                        <div class="w-full relative inline-flex">
                            <input type="text" placeholder="E-mail address"
                                class="bg-aw-fg font-serif text-lg pl-16 rounded-lg px-4 py-4 ring-1
                                focus:outline-none
                                focus:border-aw-green
                                focus:ring-2
                                focus:ring-aw-green
                                focus:invalid:border-red-500  focus:invalid:ring-red-500"
                                name="email"
                            />
                            <svg class="absolute leading-none text-center h-full w-12 h-12 px-2 left-0 top-0 flex align-center items-center rounded-xl"
                                viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M21 12C21 10.1195 20.411 8.28625 19.3156 6.7577C18.2203 5.22915 16.6736 4.08209 14.893 3.47763C13.1123 2.87316 11.187 2.84166 9.38744 3.38754C7.58792 3.93341 6.00459 5.02925 4.85982 6.52115C3.71505 8.01304 3.06635 9.82604 3.00482 11.7055C2.94329 13.585 3.47203 15.4366 4.51677 17.0001C5.56152 18.5637 7.06979 19.7608 8.82975 20.4232C10.5897 21.0856 12.513 21.18 14.3294 20.6933" stroke="#56704C" stroke-linecap="round"/>
                                <circle cx="12" cy="12" r="4" stroke="#56704C"/>
                                <path d="M16 9V13.5C16 14.8807 17.1193 16 18.5 16V16C19.8807 16 21 14.8807 21 13.5V12" stroke="#56704C" stroke-linecap="round"/>
                            </svg>
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
                                name="username"
                            />
                            <svg class="absolute leading-none text-center h-full w-12 h-12 px-2 left-0 top-0 flex align-center items-center rounded-xl"
                                viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M19.7274 20.4471C19.2716 19.1713 18.2672 18.0439 16.8701 17.2399C15.4729 16.4358 13.7611 16 12 16C10.2389 16 8.52706 16.4358 7.12991 17.2399C5.73276 18.0439 4.72839 19.1713 4.27259 20.4471" stroke="#56704C" stroke-linecap="round"/>
                                <circle cx="12" cy="8" r="4" stroke="#56704C" stroke-linecap="round"/>
                            </svg>
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
                                name="password"
                            />
                            <svg class="absolute leading-none text-center h-full w-12 h-12 px-2 left-0 top-0 flex align-center items-center rounded-xl"
                                viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"
                            >
                                <path d="M4 13C4 11.1144 4 10.1716 4.58579 9.58579C5.17157 9 6.11438 9 8 9H16C17.8856 9 18.8284 9 19.4142 9.58579C20 10.1716 20 11.1144 20 13V15C20 17.8284 20 19.2426 19.1213 20.1213C18.2426 21 16.8284 21 14 21H10C7.17157 21 5.75736 21 4.87868 20.1213C4 19.2426 4 17.8284 4 15V13Z" stroke="#56704C"/>
                                <path d="M16 8V7C16 4.79086 14.2091 3 12 3V3C9.79086 3 8 4.79086 8 7V8" stroke="#56704C" stroke-linecap="round"/>
                                <circle cx="12" cy="15" r="2" fill="#56704C"/>
                            </svg>
                        </div>
                    </div>
                    <div class="mt-2">
                        <p class="text-aw-fg text-sm font-mono font-bold">
                            {move || { if err.get() {
                                "Please fill all the fields."
                            } else {
                                ""
                            }}}
                        </p>
                    </div>
                    <div class="my-2">
                        <button type="submit"
                            class="bg-aw-green text-aw-bg font-mono font-bold text-normal mt-8 px-8 py-4 rounded-full text-center"
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
                </div>
            </MultiActionForm>
        </div>
    }
}
