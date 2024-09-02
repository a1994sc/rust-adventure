#[macro_use]
extern crate rocket;

mod info;
mod link;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            crate::info::info::index,
            crate::info::info::todo,
            crate::info::info::healthz,
            crate::info::info::version,
            crate::info::info::packages,
            crate::link::linkage::separate,
            crate::link::linkage::pair
        ],
    )
}
