use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// use rand;
use std::time::Duration;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    // let words = get_random_words();
    let (userinput, set_userinput) = create_signal(cx, "".to_string());
    let timer = create_rw_signal(cx, 0);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move |cx| view! {
                    cx,
                    <main class="bg-slate-50 h-screen w-screen my-0 mx-auto max-w-3xl text-center">
                        <h1 class="text-4xl">"AWords"</h1>
                        <Timer signal={timer}/>
                        <input class="border-2 border-black" type="text"
                            on:input=move |ev| {
                                set_userinput(event_target_value(&ev));
                            }
                            prop:value=userinput
                        />
                        <p>{userinput}</p>
                    </main>
            }/>
            </Routes>
        </Router>
    }
}

// Get the words from English.txt
// fn get_random_words() -> String {
//     let words = include_str!("../dictionary/English.txt");
//     let mut rng = rand::thread_rng();
//     let mut words = words.lines().choose_multiple(&mut rng, 10);
//     words.sort();
//     words.join(" ")
// }

/// Timer example, demonstrating the use of `use_interval`.
/// Get the signal from the parent component (timer, set_timer)
/// Remove the warning about the never read signal
#[component]
fn Timer(cx: Scope, signal: RwSignal<usize>) -> impl IntoView {

    use_interval(cx, 1000, move || {
        /* Set the timer to reach 25 and then reset */
        if signal.get() == 25 {
            signal.set(0);
        }
        signal.update(|c| *c = *c + 1);
    });

    return view! {cx,
        <div>
            <div>"Counter: "{signal}</div>
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
