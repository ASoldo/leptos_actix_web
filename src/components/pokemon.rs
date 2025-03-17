use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub name: String,
    pub height: u32,
    pub weight: u32,
}

#[server(GetPokemon, "/api")]
pub async fn get_pokemon() -> Result<Pokemon, ServerFnError> {
    Ok(reqwest::get("https://pokeapi.co/api/v2/pokemon/25")
        .await?
        .json::<Pokemon>()
        .await?)
}

#[component]
pub fn PokemonView() -> impl IntoView {
    let fetch_pokemon = Action::new(|_| async move { get_pokemon().await });

    view! {
        <div>
            <button class="bg-blue-500 rounded text-white p-4 m-2" on:click=move |_| { fetch_pokemon.dispatch(()); }>
                "Fetch Pokémon"
            </button>

            {move || match fetch_pokemon.value().get() {
                None => view! { <p>{"".to_string()}</p> },
                Some(Ok(poke)) => view! { <p>{format!("Pokémon Name: {}", poke.name)}</p> },
                Some(Err(err)) => view! { <p>{format!("Error: {}", err)}</p> },
            }}
        </div>
    }
}
