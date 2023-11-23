use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// use crate::timer::Timer;

use getrandom::getrandom;
use std::time::Duration;

#[allow(unused)]
const WORDS: &str = include_str!("../dictionary/English.txt");

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let random_words = create_rw_signal(cx, get_random_words(40));
    let (userinput, set_userinput) = create_signal(cx, "".to_string());

    let timer = create_rw_signal(cx, 30);
    let start = create_rw_signal(cx, false);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Pacifico&display=swap"/>
        <Link rel="shortcut icon" type_="image/ico" href="/logo.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move |cx| view! {
                    cx,
                    <main class="bg-aw-bg h-screen w-full my-0 mx-auto text-center">
                        <p class="">"."</p>
                        <h1 class="text-4xl font-pacifico text-aw-green mt-4">"AWORDS"</h1>
                        <div class="flex flex-col mt-20 items-center">
                            /* TIMER */
                            <Timer signal={start} timer={timer}/>
                            /* RANDOM WORDS */
                            <p class="text-aw-fg font-mono text-2xl max-w-4xl mx-auto my-8 text-justify">
                                {random_words}
                            </p>

                            /* RESTART BUTTON */
                            // <button class="bg-aw-green text-aw-bg font-bold text-2xl px-4 py-2 rounded-lg"
                            <div class="cursor-pointer"
                                on:click=move |_| {
                                    random_words.set(get_random_words(40));
                                    set_userinput("".to_string());
                                    timer.set(30);
                                }
                            >
                                <img src="/refresh.svg" class="w-12 h-12"/>
                            </div>
                        </div>

                        /* USER INPUT */
                        <input class="opacity-0 absolute -z-1" type="text" autofocus
                            on:input=move |ev| {
                                /* Set the user input */
                                set_userinput(event_target_value(&ev));
                                /* Start the timer when the user starts typing */
                                if userinput.get() != "" && !start.get() {
                                    start.set(true);
                                }
                            }
                            prop:value=userinput
                        />

                        /* USER INPUT CHECKOUT */
                        { move || {
                            let random_words_chars: Vec<char> = random_words.get().chars().collect();
                            let userinput_chars: Vec<char> = userinput.get().chars().collect();
                            let result = create_rw_signal::<Vec<(char, bool)>>(cx, Vec::new());
                            for i in 0..userinput_chars.len() {
                                if !(userinput_chars[i] == random_words_chars[i]) {
                                    // typo = true;
                                    result.update(|c| c.push((userinput_chars[i], true)));
                                } else {
                                    // typo = false;
                                    result.update(|c| c.push((userinput_chars[i], false)));
                                }
                            }
                            view! {cx,
                                <For
                                    each=result
                                    key=|i| match i {
                                        _ => "other",
                                    }
                                    view=move |i| view! {cx,
                                        <div class={if i.1 { "text-aw-red" } else { "text-aw-green" }}>
                                            {i.0}
                                        </div>
                                    }
                                />
                            }
                        }}
                    </main>
            }/>
            </Routes>
        </Router>
    }
}

// Get the words from English.txt
#[allow(unused)]
fn get_random_words(amount: u16) -> String {
    let words: Vec<&str> = WORDS.split('\n').collect();
    let mut buf = [0u8; 4];
    getrandom(&mut buf).unwrap();
    let mut result = String::new();
    let max = words.len() - 1;
    for _ in 0..amount {
        let index = u32::from_le_bytes(buf) % max as u32;
        result.push_str(words[index as usize]);
        result.push(' ');
        getrandom(&mut buf).unwrap();
    }
    result
}

#[component]
pub fn Timer(cx: Scope, signal: RwSignal<bool>, timer: RwSignal<usize>)
    -> impl IntoView
{
    return view! {cx,
        <div>
        {move || {
            if signal.get() {
                use_interval(cx, 1000, move || {
                    /* TODO: Deal with this later */
                    /* It shoud save the wpm of the user in the db */
                    if timer.get() <= 0 {
                        timer.set(30);
                        signal.set(false);
                        // set_userinput("".to_string());
                    }
                    timer.update(|c| *c = *c - 1);
                });
                view! {cx,
                    <div class="flex">
                        <img src="/clock.svg" class="w-12 h-12"/>
                        <h1 class="text-aw-green-light ml-2 text-4xl text-bold font-pacifico self-stretch">{timer}</h1>
                    </div>
                }
            } else {
                view! {cx,
                    <div class="flex">
                        <img src="/clock.svg" class="w-12 h-12"/>
                        <h1 class="text-aw-green-light ml-2 text-4xl text-bold font-pacifico self-stretch">{timer}</h1>
                    </div>
                }
            }
        }}
        </div>
    }
}

#[allow(unused)]
fn use_interval<T, F>(cx: Scope, interval_millis: T, f: F)
where
    F: Fn() + Clone + 'static,
    T: Into<MaybeSignal<u64>> + 'static,
{
    let interval_millis = interval_millis.into();
    create_effect(cx, move |prev_handle: Option<IntervalHandle>| {
        if let Some(prev_handle) = prev_handle {
            prev_handle.clear();
        };

        set_interval(
            f.clone(),
            // this is the only reactive access, so this effect will only
            // re-run when the interval changes
            Duration::from_millis(interval_millis.get()),
        )
        .expect("could not create interval")
    });
}

/* #[component]
fn Example(cx: Scope, random_words: RwSignal<String>, userinput: ReadSignal<String>) -> impl IntoView {
    let random_words_chars: Vec<char> = random_words.get().chars().collect();
    let userinput_chars: Vec<char> = userinput.get().chars().collect();
    return view! {cx,
        <div>
        {move || {
            let result = create_rw_signal::<Vec<(char, bool)>>(cx, Vec::new());
            for i in 0..userinput_chars.len() {
                if !(userinput_chars[i] == random_words_chars[i]) {
                    // typo = true;
                    result.update(|c| c.push((userinput_chars[i], true)));
                } else {
                    // typo = false;
                    result.update(|c| c.push((userinput_chars[i], false)));
                }
            }
            view! {cx,
                <For
                    each=result
                    key=|i| match i {
                        _ => "other",
                    }
                    view=move |i| view! {cx,
                        <div class={if i.1 { "text-aw-red" } else { "text-aw-green" }}>
                            {i}
                        </div>
                    }
                />
            }
        }}
        </div>
    }
} */
