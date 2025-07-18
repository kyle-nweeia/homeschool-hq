#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| {
        rsx! {
            body {
                Router::<Route> {}
            }
        }
    });
}

#[derive(serde::Deserialize)]
struct FirebaseResponse {
    idToken: String,
    email: String,
    refreshToken: String,
    expiresIn: String,
    localId: String,
    registered: bool,
}

#[derive(serde::Serialize)]
struct Payload {
    email: String,
    password: String,
    returnSecureToken: bool,
}

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    Home,
    #[route("/login")]
    Login,
}

const FIREBASE_RESPONSE: GlobalSignal<Option<FirebaseResponse>> = Global::new(Option::default);

fn Home() -> Element {
    let onclick = |_| {
        use_navigator().push(Route::Login);
    };

    rsx! {
        h1 { "Homeschool HQ" }
        button { onclick, if FIREBASE_RESPONSE.read().is_some() { "Log out" } else { "Log in" } }
    }
}

fn Login() -> Element {
    let onclick = |_| {
        *FIREBASE_RESPONSE.write() = None;
        use_navigator().push(Route::Home);
    };
    let onsubmit = move |evt: FormEvent| async move {
        let payload = Payload {
            email: evt.values()["email"].as_value(),
            password: evt.values()["password"].as_value(),
            returnSecureToken: true,
        };
        let response = reqwest::Client::new()
            .post("https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key=AIzaSyCvvAGxu3nyHzYkvlxv9_UqPKTPqOJqIdk")
            .json(&payload)
            .send()
            .await
            .unwrap();

        if response.status() == 200 {
            *FIREBASE_RESPONSE.write() = Some(response.json::<FirebaseResponse>().await.unwrap());
        }
        use_navigator().push(Route::Home);
    };

    rsx! {
        h1 { "Login" }
        if FIREBASE_RESPONSE.read().is_some() {
            button { onclick, "Confirm logout" }
        } else {
            form { onsubmit,
                label { "Email" }
                br {}
                input { r#type: "text", id: "email", name: "email" }
                br {}
                label { "Password" }
                br {}
                input { r#type: "password", id: "password", name: "password" }
                br {}
                button { "Sign in" }
            }
        }
    }
}
