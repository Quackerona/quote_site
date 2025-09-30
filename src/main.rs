use rocket::{fs::{relative, FileServer}, get, post, routes, tokio::{sync::Mutex, time::Instant, time::Duration}, State};

const WAITING_DURATION: u64 = 600;

struct MessageState {
    next_change_time: Instant,
    message: String
}

#[get("/")]
async fn get_message(message_state: &State<Mutex<MessageState>>) -> String {
    message_state.lock().await.message.clone()
}

#[post("/update", data="<msg>")]
async fn update_message(message_state: &State<Mutex<MessageState>>, msg: String) -> String {
    let mut outer_guard = message_state.lock().await;

    let now = Instant::now();

    if now >= outer_guard.next_change_time {
        outer_guard.message = format!("\"{}\"", msg);
        outer_guard.next_change_time = now + Duration::from_secs(WAITING_DURATION);

        "Success!".to_owned()
    }
    else {
        format!("{}{}{}", "Too late! Wait ", ((outer_guard.next_change_time - now).as_secs_f32() / 60.0).ceil(), " minutes!")
    }
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .manage(Mutex::new(MessageState{next_change_time: Instant::now(), message: "\"Default quote\"".to_owned()}))
        .mount("/", FileServer::from(relative!("frontend/dist")))
        .mount("/api", routes![get_message, update_message]);

    Ok(rocket.into())
}