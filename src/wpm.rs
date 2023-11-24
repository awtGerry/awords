#[component]
fn Checkout(cx: Scope, random_words: RwSignal<String>, user_input: ReadSignal<String>) -> impl IntoView {
    let binding = random_words.clone().get();
    let binding_ui = user_input.clone().get();
    let random_words_by_space: Vec<&str> = binding.split(' ').collect();
    let userinput_by_space: Vec<&str> = binding_ui.split(' ').collect();
    return view! {cx,
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
}
