use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore_router::navigate;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct NewPet {
    pub name: String,
    pub birth_date: String,
    pub kind_id: i32,
}

#[derive(Prop)]
pub struct MyProps {
    owner_id: i32,
}

#[component]
pub async fn PetForm<G: Html>(cx: Scope<'_>, props: MyProps) -> View<G> {
    let name = create_signal(cx, String::new());
    let birth_date = create_signal(cx, String::new());
    let kind = create_signal(cx, String::from("1"));
    let owner_id = props.owner_id;

    let add_pet = move || {
        let pet_name = name.get().to_string();
        let pet_birth_date = birth_date.get().to_string();
        let pet_type = kind.get().parse().unwrap();
        wasm_bindgen_futures::spawn_local(async move {
            let pet_created =
                Request::post(&format!("http://localhost:3000/owners/{owner_id}/pets/new"))
                    .body(
                        serde_json::to_string(&NewPet {
                            name: pet_name,
                            birth_date: pet_birth_date,
                            kind_id: pet_type,
                        })
                        .unwrap(),
                    )
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap();
            if 201 == pet_created.status() {
                navigate("/owners")
            }
        });
    };

    view! {cx,
        h1 {
            "New Pet"
        }
        label(for="name") {
            "Name"
        }
        input(bind:value=name, type="text", id="name")

        label(for="birth-date") {
            "Birth Date"
        }
        input(bind:value=birth_date, type="date", id="birth-date")

        label(for="type") {
            "Type"
        }
        select(bind:value=kind, id="type") {
            option(value="1") {
                "Cat"
            }
            option(value="2") {
                "Dog"
            }
        }

        br

        button(class="add-pet", on:click=move |_| Box::new(add_pet)()) {
            "Add Pet"
        }
    }
}
