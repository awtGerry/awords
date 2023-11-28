use leptos::*;

use crate::components::timer::Timer;

use getrandom::getrandom;

#[allow(unused)]
const WORDS: &str = include_str!("../../dictionary/English.txt");

#[allow(unused, non_snake_case)]
pub fn Awords() -> impl IntoView {
    // let random_words = create_rw_signal(get_random_words(40));
    let random_words = create_rw_signal(get_random_words(40));
    let userinput = create_rw_signal("".to_string());

    let timer = create_rw_signal(30);
    let start = create_rw_signal(false);

    view! {
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
                        // <img src="/trophy.svg" class="mt-4 w-12 h-12"/>
                        <svg class="w-12 h-12 mt-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M16.5 20.5H7.5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M12.5 18.5C12.5 18.7761 12.2761 19 12 19C11.7239 19 11.5 18.7761 11.5 18.5H12.5ZM11.5 18.5V16H12.5V18.5H11.5Z" fill="#C5EE4F"/>
                            <path d="M10.5 9.5H13.5" stroke="#2C5EE4F" stroke-linecap="round"/>
                            <path d="M5.5 14.5C5.5 14.5 3.5 13 3.5 10.5C3.5 9.73465 3.5 9.06302 3.5 8.49945C3.5 7.39488 4.39543 6.5 5.5 6.5V6.5C6.60457 6.5 7.5 7.39543 7.5 8.5V9.5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M18.5 14.5C18.5 14.5 20.5 13 20.5 10.5C20.5 9.73465 20.5 9.06302 20.5 8.49945C20.5 7.39488 19.6046 6.5 18.5 6.5V6.5C17.3954 6.5 16.5 7.39543 16.5 8.5V9.5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M16.5 11.3593V7.5C16.5 6.39543 15.6046 5.5 14.5 5.5H9.5C8.39543 5.5 7.5 6.39543 7.5 7.5V11.3593C7.5 12.6967 8.16841 13.9456 9.2812 14.6875L11.4453 16.1302C11.7812 16.3541 12.2188 16.3541 12.5547 16.1302L14.7188 14.6875C15.8316 13.9456 16.5 12.6967 16.5 11.3593Z" stroke="#C5EE4F"/>
                        </svg>
                    </a>
                    <a href="/login" class="text-aw-green-light text-2xl font-bold mx-4">
                        // <img src="/user.svg" class="mt-4 w-12 h-12"/>
                        <svg class="w-12 h-12 mt-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M19.7274 20.4471C19.2716 19.1713 18.2672 18.0439 16.8701 17.2399C15.4729 16.4358 13.7611 16 12 16C10.2389 16 8.52706 16.4358 7.12991 17.2399C5.73276 18.0439 4.72839 19.1713 4.27259 20.4471" stroke="#C5EE4F" stroke-linecap="round"/>
                            <circle cx="12" cy="8" r="4" stroke="#C5EE4F" stroke-linecap="round"/>
                        </svg>
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
                    view! {
                        <div class="text-aw-fg font-mono text-2xl max-w-4xl mx-auto my-8">
                            <div class="flex flex-row flex-wrap whitespace-normal text-justify">
                                {random_words_by_space.into_iter().zip(userinput_by_space.into_iter())
                                    .map(|(rw, uw)| view! {
                                        // Get the word character by character
                                        <div class="mr-2">
                                            {rw.chars().zip(uw.chars())
                                                .map(|(rc, uc)| view! {
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
                        random_words.set(get_random_words(40));
                        timer.set(30);
                    }
                >
                    <svg class="w-12 h-12" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M14 15L10 19L14 23" stroke="#C5EE4F"/>
                        <path d="M5.93782 15.5C5.14475 14.1264 4.84171 12.5241 5.07833 10.9557C5.31495 9.38734 6.07722 7.94581 7.24024 6.86729C8.40327 5.78877 9.8981 5.13721 11.4798 5.01935C13.0616 4.90149 14.6365 5.32432 15.9465 6.21856C17.2565 7.1128 18.224 8.42544 18.6905 9.94144C19.1569 11.4574 19.0947 13.0869 18.5139 14.5629C17.9332 16.0389 16.8684 17.2739 15.494 18.0656C14.1196 18.8573 12.517 19.1588 10.9489 18.9206" stroke="#C5EE4F" stroke-linecap="round"/>
                    </svg>
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

#[allow(unused, non_snake_case)]
pub fn AwordsLogged() -> impl IntoView {
    // let random_words = create_rw_signal(get_random_words(40));
    let random_words = create_rw_signal(get_random_words(40));
    let userinput = create_rw_signal("".to_string());

    let timer = create_rw_signal(30);
    let start = create_rw_signal(false);

    view! {
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
                        <svg class="w-12 h-12 mt-4" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M16.5 20.5H7.5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M12.5 18.5C12.5 18.7761 12.2761 19 12 19C11.7239 19 11.5 18.7761 11.5 18.5H12.5ZM11.5 18.5V16H12.5V18.5H11.5Z" fill="#C5EE4F"/>
                            <path d="M10.5 9.5H13.5" stroke="#2C5EE4F" stroke-linecap="round"/>
                            <path d="M5.5 14.5C5.5 14.5 3.5 13 3.5 10.5C3.5 9.73465 3.5 9.06302 3.5 8.49945C3.5 7.39488 4.39543 6.5 5.5 6.5V6.5C6.60457 6.5 7.5 7.39543 7.5 8.5V9.5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M18.5 14.5C18.5 14.5 20.5 13 20.5 10.5C20.5 9.73465 20.5 9.06302 20.5 8.49945C20.5 7.39488 19.6046 6.5 18.5 6.5V6.5C17.3954 6.5 16.5 7.39543 16.5 8.5V9.5" stroke="#C5EE4F" stroke-linecap="round"/>
                            <path d="M16.5 11.3593V7.5C16.5 6.39543 15.6046 5.5 14.5 5.5H9.5C8.39543 5.5 7.5 6.39543 7.5 7.5V11.3593C7.5 12.6967 8.16841 13.9456 9.2812 14.6875L11.4453 16.1302C11.7812 16.3541 12.2188 16.3541 12.5547 16.1302L14.7188 14.6875C15.8316 13.9456 16.5 12.6967 16.5 11.3593Z" stroke="#C5EE4F"/>
                        </svg>
                    </a>
                    <a href="#" class="text-aw-green-light text-2xl font-bold mx-4">
                        <p class="">"user"</p>
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
                    view! {
                        <div class="text-aw-fg font-mono text-2xl max-w-4xl mx-auto my-8">
                            <div class="flex flex-row flex-wrap whitespace-normal text-justify">
                                {random_words_by_space.into_iter().zip(userinput_by_space.into_iter())
                                    .map(|(rw, uw)| view! {
                                        // Get the word character by character
                                        <div class="mr-2">
                                            {rw.chars().zip(uw.chars())
                                                .map(|(rc, uc)| view! {
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
                        random_words.set(get_random_words(40));
                        timer.set(30);
                    }
                >
                    // <img src="/refresh.svg" class="w-12 h-12"/>
                    <svg class="w-12 h-12" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M14 15L10 19L14 23" stroke="#C5EE4F"/>
                        <path d="M5.93782 15.5C5.14475 14.1264 4.84171 12.5241 5.07833 10.9557C5.31495 9.38734 6.07722 7.94581 7.24024 6.86729C8.40327 5.78877 9.8981 5.13721 11.4798 5.01935C13.0616 4.90149 14.6365 5.32432 15.9465 6.21856C17.2565 7.1128 18.224 8.42544 18.6905 9.94144C19.1569 11.4574 19.0947 13.0869 18.5139 14.5629C17.9332 16.0389 16.8684 17.2739 15.494 18.0656C14.1196 18.8573 12.517 19.1588 10.9489 18.9206" stroke="#C5EE4F" stroke-linecap="round"/>
                    </svg>
                </div>
            </div>
        </main>
    }
}
