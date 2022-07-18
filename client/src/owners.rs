use sycamore::prelude::*;

#[derive(Prop)]
pub struct MyProps<'a> {
    pub owners: &'a ReadSignal<Vec<dto::Owner>>,
}

#[component]
pub fn OwnersList<'a, G: Html>(cx: Scope<'a>, props: MyProps<'a>) -> View<G> {
    view! {cx,
        h1 {
            "Owners"
        }
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
                    th {}
                }
            }
            tbody {
                Indexed {
                    iterable: props.owners,
                    view: |cx, owner| view! { cx,
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
                            td {
                                a(href=(format!("/owners/{}/pets/new", owner.id))) {
                                    "Add Pet"
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
