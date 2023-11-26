use leptos::*;
use leptos_meta::*;

use crate::components::timer::Timer;
use crate::components::timer::TimerProps;

use getrandom::getrandom;

#[allow(unused)]
const WORDS: &str = include_str!("../../dictionary/English.txt");

#[allow(unused, non_snake_case)]
pub fn Awords(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    // let random_words = create_rw_signal(cx, get_random_words(40));
    let (random_words, set_random_words) = create_signal(cx, get_random_words(40));
    let userinput = create_rw_signal(cx, "".to_string());

    let timer = create_rw_signal(cx, 30);
    let start = create_rw_signal(cx, false);

    return view! {cx,
        <main class="bg-aw-bg h-screen w-full my-0 mx-auto text-center">
            <p class="opacity-0">"."</p>
            /* LOGO AND MENU */
            <div class="flex justify-evenly items-center">
                <div class="opacity-0"></div>
                <div class="absolute">
                    <h1 class="text-4xl font-pacifico text-aw-green mt-4">"AWORDS"</h1>
                </div>
                <div class="flex flex-row">
                    <a href="#" class="text-aw-green-light text-2xl font-bold ml-64">
                        <img src="/trophy.svg" class="mt-4 w-12 h-12"/>
                    </a>
                    <a href="/login" class="text-aw-green-light text-2xl font-bold mx-4">
                        <img src="/user.svg" class="mt-4 w-12 h-12"/>
                    </a>
                </div>
            </div>
            <div class="flex flex-col mt-12 items-center">
                /* USER INPUT */
                <input class="opacity-0 absolute -z-1" type="text" autofocus
                    on:input=move |ev| {
                        /* Set the user input */
                        userinput.update(|c| *c = event_target_value(&ev));
                        if !start.get() && userinput.get().len() > 0 {
                            /* Start the timer */
                            start.update(|c| *c = true);
                        }
                    }
                    prop:value=userinput
                />
                /* TIMER */
                <Timer signal={start} timer={timer} userinput={userinput} />
                /* RANDOM WORDS PREVIEW */
                <p class="text-aw-fg font-mono text-2xl max-w-4xl mx-auto my-8 text-justify">
                    {random_words}
                </p>
                /* USER INPUT CHECKOUT */
                { move || {
                    let binding = random_words.clone().get();
                    let binding_ui = userinput.clone().get();
                    let random_words_by_space: Vec<&str> = binding.split(' ').collect();
                    let userinput_by_space: Vec<&str> = binding_ui.split(' ').collect();
                    view! {cx,
                        <div class="text-aw-fg font-mono text-2xl max-w-4xl mx-auto my-8">
                            <div class="flex flex-row flex-wrap whitespace-normal text-justify">
                                {random_words_by_space.into_iter().zip(userinput_by_space.into_iter())
                                    .map(|(rw, uw)| view! {cx,
                                        // Get the word character by character
                                        <div class="mr-2">
                                            {rw.chars().zip(uw.chars())
                                                .map(|(rc, uc)| view! {cx,
                                                    // If the character is the same, then it's green
                                                    // If the character is different, then it's red
                                                    <span class={if rc == uc {"text-aw-green"} else {"text-aw-red"}}>{uc}</span>
                                                })
                                                .collect::<Vec<_>>()
                                            }
                                        </div>
                                    })
                                    .collect::<Vec<_>>()
                                }
                            </div>
                        </div>
                    }
                }}

                /* RESTART BUTTON */
                // <button class="bg-aw-green text-aw-bg font-bold text-2xl px-4 py-2 rounded-lg"
                <div class="cursor-pointer"
                    on:click=move |_| {
                        userinput.update(|c| *c = "".to_string());
                        start.set(false);
                        set_random_words(get_random_words(40));
                        timer.set(30);
                    }
                >
                    <img src="/refresh.svg" class="w-12 h-12"/>
                </div>
            </div>
        </main>
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
