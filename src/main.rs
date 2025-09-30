use rocket::{fs::{relative, FileServer}, get, launch, post, routes, tokio::{sync::Mutex, time::Instant, time::Duration}, State};

const WAITING_DURATION: u64 = 600;

struct MessageState {
    time_till_change: Instant,
    message: String
}

#[get("/")]
async fn get_message(message_state: &State<Mutex<MessageState>>) -> String {
    message_state.lock().await.message.clone()
}

#[post("/update", data="<msg>")]
async fn update_message(message_state: &State<Mutex<MessageState>>, msg: String) -> String {
    let mut outer_guard = message_state.lock().await;

    if Instant::now().duration_since(outer_guard.time_till_change) >= Duration::from_secs(WAITING_DURATION) {
        outer_guard.message = format!("\"{}\"", msg);
        outer_guard.time_till_change = Instant::now() + Duration::from_secs(WAITING_DURATION);

        "Success!".to_owned()
    }
    else {
        format!("{}{}{}", "Too late! Wait ", ((outer_guard.time_till_change - Instant::now()).as_secs_f32() / 60.0).ceil(), " minutes!")
    }
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(Mutex::new(MessageState{time_till_change: Instant::now() - Duration::from_secs(WAITING_DURATION), message: "\"Default quote\"".to_owned()}))
        .mount("/", FileServer::from(relative!("frontend/dist")))
        .mount("/api", routes![get_message, update_message])
}