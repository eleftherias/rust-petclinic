use serde::Deserialize;
use sycamore::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Owner {
    id: i32,
    first_name: String,
    last_name: String,
    address: String,
    city: String,
    telephone: String,
    pets: Vec<Pet>
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Pet {
    pub name: String,
}

#[component(OwnersList<G>)]
pub fn owners_list(owners: ReadSignal<Vec<Owner>>) -> View<G> {
    view! {
        table(id="owners") {
            thead {
                tr {
                    th {
                        "Name"
                    }
                    th {
                        "Address"
                    }
                    th {
                        "City"
                    }
                    th {
                        "Telephone"
                    }
                    th {
                        "Pets"
                    }
                }
            }
            tbody {
                Indexed(IndexedProps {
                    iterable: owners,
                    template: |owner| view! {
                        tr {
                            td {
                                (format!("{} {}", owner.first_name, owner.last_name))
                            }
                            td {
                                (owner.address)
                            }
                            td {
                                (owner.city)
                            }
                            td {
                                (owner.telephone)
                            }
                            td {
                                (owner.pets.iter().map(|p| p.name.to_owned()).collect::<Vec<String>>().join(", "))
                            }
                         }
                    },
                })
            }
        }
    }
}
