use leptos::*;
use std::time::Duration;

#[allow(unused)]
#[component]
pub fn Timer(cx: Scope, signal: RwSignal<bool>, timer: RwSignal<usize>, userinput: RwSignal<String>)
-> impl IntoView {
    let msg = create_rw_signal(cx, "".to_string());
    return view! {cx,
        <div>
        {move || {
            if signal.get() {
                use_interval(cx, 1000, move || {
                    /* TODO: Deal with this later */
                    /* It shoud save the wpm of the user in the db */
                    if timer.get() <= 0 {
                        signal.set(false);
                        timer.set(30);
                        msg.update(|c| *c = format!("{} wpm", userinput.get().len() / 5));
                        userinput.update(|c| *c = "".to_string());
                    }
                    timer.update(|c| *c = *c - 1);
                });
                view! {cx,
                    <div class="flex">
                        <img src="/clock.svg" class="w-12 h-12"/>
                        <h1 class="text-aw-green-light ml-2 text-4xl text-bold font-pacifico self-stretch">{timer}</h1>
                        <span class="text-aw-green-light ml-2 text-4xl text-bold font-pacifico self-stretch">{msg}</span>
                    </div>
                }
            } else {
                view! {cx,
                    <div class="flex">
                        <img src="/clock.svg" class="w-12 h-12"/>
                        <h1 class="text-aw-green-light ml-2 text-4xl text-bold font-pacifico self-stretch">{timer}</h1>
                        <span class="text-aw-green-light ml-2 text-4xl text-bold font-pacifico self-stretch">{msg}</span>
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
