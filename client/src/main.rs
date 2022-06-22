pub mod owners;

use owners::Owner;
use owners::OwnersList;
use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let owners = use_state(|| vec![]);
    {
        let owners = owners.clone();
        use_effect_with_deps(
            move |_| {
                let owners = owners.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_owners: Vec<Owner> = Request::get("http://localhost:3000/owners")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    owners.set(fetched_owners);
                });
                || ()
            },
            (),
        );
    }
    html! {
        <OwnersList owners={(*owners).clone()} />
    }
}

fn main() {
    yew::start_app::<App>();
}
