use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub name: String,
    pub height: u32,
    pub weight: u32,
}
#[component]
pub fn PokemonView() -> impl IntoView {
    let (pokemon, set_pokemon) = signal(None::<Result<Pokemon, String>>);

    spawn_local(async move {
        let result = match Request::get("https://pokeapi.co/api/v2/pokemon/25")
            .send()
            .await
        {
            Ok(response) => match response.json::<Pokemon>().await {
                Ok(poke) => Ok(poke),
                Err(e) => Err(format!("JSON parse error: {e}")),
            },
            Err(e) => Err(format!("Request error: {e}")),
        };

        set_pokemon.set(Some(result));
    });

    view! {
        <Suspense>
            {
                move || match pokemon.get() {
                    Some(Ok(ref poke)) => view! {
                        <p>{format!("Pokémon Name: {}", poke.name)}</p>
                    },
                    Some(Err(ref err)) => view! {
                        <p>{format!("Error: {err}")}</p>
                    },
                    None => view! {
                        <p>{"Loading...".to_string()}</p>
                    },
                }
            }
        </Suspense>
    }
}
