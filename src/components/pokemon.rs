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
    let poke = reqwest::get("https://pokeapi.co/api/v2/pokemon/25")
        .await?
        .json::<Pokemon>()
        .await?;
    Ok(poke)
}

#[component]
pub fn PokemonView() -> impl IntoView {
    let pokemon_resource = Resource::new(|| (), |_| async move { get_pokemon().await });

    view! {
        <Suspense>
            <div>
                {move || match pokemon_resource.get() {
                    None => view! { <p>{"Loading...".to_string()}</p> },
                    Some(Ok(poke)) => view! { <p>{format!("Pokémon Name: {}", poke.name)}</p> },
                    Some(Err(err)) => view! { <p>{format!("Error: {}", err)}</p> },
                }}
            </div>
        </Suspense>
    }
}
