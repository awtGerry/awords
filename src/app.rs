use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use getrandom::getrandom;
use std::time::Duration;

const WORDS: &str = include_str!("../dictionary/English.txt");

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let random_words = create_rw_signal(cx, get_random_words(40));
    let (userinput, set_userinput) = create_signal(cx, "".to_string());
    let timer = create_rw_signal(cx, 0);

    let vec_words = random_words.get().split(' ').collect::<Vec<&str>>();
    let vec_input = userinput.get().split(' ').collect::<Vec<&str>>();

    let msg = String::new();

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
                        <input class="opacity-0 absolute -z-1" type="text" autofocus
                            on:input=move |ev| {
                                set_userinput(event_target_value(&ev));
                            }
                            prop:value=userinput
                        />
                        <div class="flex mt-20 justify-center align-center">
                            <div class="flex-start">
                                <Timer signal={timer}/>
                            </div>
                            <div class="mt-12">
                                <p class="text-aw-fg font-mono text-2xl max-w-3xl mx-auto my-4 text-justify">
                                    {random_words}
                                </p>
                            </div>
                        </div>
                        <p class="text-aw-fg">{userinput}</p>
                    </main>
            }/>
            </Routes>
        </Router>
    }
}

// fn is_equal(input: &str, words: Vec<&str>) -> bool {
//     if vec1.len() != vec2.len() {
//         return false;
//     }
//     for i in 0..vec1.len() {
//         if vec1[i] != vec2[i] {
//             return false;
//         }
//     }
//     return true;
// }

// Get the words from English.txt
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

/* Check the input with the string, if the user input is right paint the
   current string to green, else to red.
*/
// #[component]
// fn check_string(cx: Scope, usr: String, dic: String, signal: RwSignal<usize>) -> impl IntoView {
//     return view! { cx,
//     }
// }

#[component]
fn Timer(cx: Scope, signal: RwSignal<usize>) -> impl IntoView {

    use_interval(cx, 1000, move || {
        /* Set the timer to reach 25 and then reset */
        if signal.get() == 0 {
            signal.set(30);
        }
        signal.update(|c| *c = *c - 1);
    });

    return view! {cx,
        <div class="flex items-center justify-center">
            <img src="/clock.svg" class="w-12 h-12"/>
            <h1 class="text-aw-green-light ml-2 text-4xl text-bold font-pacifico self-stretch">{signal}</h1>
        </div>
    }
}

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
