use leptos::{leptos_dom::helpers::IntervalHandle, *};
use std::time::Duration;

#[allow(unused)]
#[component]
pub fn Timer(signal: RwSignal<bool>, timer: RwSignal<usize>, userinput: RwSignal<String>)
-> impl IntoView {
    let msg = create_rw_signal("".to_string());
    return view! {
        <div>
        {move || {
            if signal.get() {
                use_interval(1000, move || {
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
                view! {
                    <div class="flex">
                        <svg class="w-12 h-12" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                            <circle cx="12" cy="12" r="8.5" stroke="#C5EE4F"/>
                            <path d="M5 2.80385C4.08789 3.33046 3.33046 4.08788 2.80385 5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M19 2.80385C19.9121 3.33046 20.6695 4.08788 21.1962 5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M12 6.5V11.75C12 11.8881 12.1119 12 12.25 12H16.5" stroke="#C5EE4F" stroke-linecap="round"/>
                        </svg>
                        <h1 class="text-aw-green-light ml-2 text-4xl text-bold font-pacifico self-stretch">{timer}</h1>
                        <span class="text-aw-green-light ml-2 text-4xl text-bold font-pacifico self-stretch">{msg}</span>
                    </div>
                }
            } else {
                view! {
                    <div class="flex">
                        <svg class="w-12 h-12" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                            <circle cx="12" cy="12" r="8.5" stroke="#C5EE4F"/>
                            <path d="M5 2.80385C4.08789 3.33046 3.33046 4.08788 2.80385 5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M19 2.80385C19.9121 3.33046 20.6695 4.08788 21.1962 5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M12 6.5V11.75C12 11.8881 12.1119 12 12.25 12H16.5" stroke="#C5EE4F" stroke-linecap="round"/>
                        </svg>
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
fn use_interval<T, F>(interval_millis: T, f: F)
where
    F: Fn() + Clone + 'static,
    T: Into<MaybeSignal<u64>> + 'static,
{
    let interval_millis = interval_millis.into();
    create_effect(move |prev_handle: Option<IntervalHandle>| {
        // effects get their previous return value as an argument
        // each time the effect runs, it will return the interval handle
        // so if we have a previous one, we cancel it
        if let Some(prev_handle) = prev_handle {
            prev_handle.clear();
        };

        // here, we return the handle
        set_interval_with_handle(
            f.clone(),
            // this is the only reactive access, so this effect will only
            // re-run when the interval changes
            Duration::from_millis(interval_millis.get()),
        )
        .expect("could not create interval")
    });
}
