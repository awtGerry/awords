use leptos::*;
use std::time::Duration;

/// Timer example, demonstrating the use of `use_interval`.
#[component]
pub fn Timer(cx: Scope, signal: RwSignal<usize>) -> impl IntoView {
    use_interval(1000, move || {
        signal.update(|c| *c += 1);
    }, cx);

    return view! {cx,
        <div>
            <div>"Counter: "</div>
        </div>
    }
}

fn use_interval<T, F>(interval_millis: T, f: F, cx: Scope)
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
