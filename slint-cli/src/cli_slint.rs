use std::io::{self, Write};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::cell::RefCell;
use std::rc::Rc;
use crate::{WelcomeWindow, RegisterWindow, UserWindow};
use slint::ComponentHandle;
use slint::{ModelRc, VecModel, SharedString};
use crate::reqwest_functions as rf;

thread_local! {
    static USER_WINDOW_HOLDER: RefCell<Option<UserWindow>> = RefCell::new(None);
    static REGISTER_WINDOW_HOLDER: RefCell<Option<RegisterWindow>> = RefCell::new(None);
}


pub fn beginprogram(client: Client, app: &WelcomeWindow) {
    let login_weak = app.as_weak();
    let login_client = client.clone();

    app.on_login_clicked(move || {
        let app = login_weak.unwrap();

        let name = app.get_login_text().to_string();
        let password = app.get_password_text().to_string();

        let client = login_client.clone();
        let app_weak_inner = app.as_weak();

        tokio::spawn(async move {
            let result: Result<rf::LoginResponse, String> =
                rf::login_user(client.clone(), name.clone(), password).await;

            match result {
                Ok(data) => {
                    let token = data.access_token.clone();
                    let client_for_ui = client.clone();

                    let _ = slint::invoke_from_event_loop(move || {
                        let app = app_weak_inner.unwrap();
                        app.set_status_text("Login successful".into());

                        open_user_panel(client_for_ui, token, &app);
                    });
                }
                Err(err) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        let app = app_weak_inner.unwrap();
                        app.set_status_text(slint::SharedString::from(err));
                    });
                }
            }
        });
    });

    let register_weak = app.as_weak();
    let register_client = client.clone();

    app.on_register_clicked(move || {
        let app = register_weak.unwrap();
        open_register_window(register_client.clone(), &app);
    });
}
pub fn open_register_window(client: Client, _welcome_app: &WelcomeWindow) {
    let window = RegisterWindow::new().unwrap();

    setup_register_window_logic(client, &window);

    window.show().unwrap();

    REGISTER_WINDOW_HOLDER.with(|slot| {
        *slot.borrow_mut() = Some(window);
    });
}
pub fn open_user_panel(client: Client, token: String, welcome_app: &WelcomeWindow) {
    let window = UserWindow::new().unwrap();

    setup_user_window_logic(client, token, &window);

    window.show().unwrap();

    USER_WINDOW_HOLDER.with(|slot| {
        *slot.borrow_mut() = Some(window);
    });

    welcome_app.hide().unwrap();
}
pub fn setup_user_window_logic(client: Client, token: String, user_app: &UserWindow) {
    let _client_clone = client.clone();
    let token_clone = token.clone();
    let app_weak = user_app.as_weak();

    tokio::spawn(async move {
        let result: Result<rf::ListTaskResponse, reqwest::Error> =
            rf::get_tasks(&token_clone).await;

        match result {
            Ok(data) => {
                let tasks: Vec<SharedString> = data
                    .tasks
                    .into_iter()
                    .map(|t| SharedString::from(t.title))
                    .collect();

                let _ = slint::invoke_from_event_loop(move || {
                    let app = app_weak.unwrap();
                    let model = ModelRc::new(VecModel::from(tasks));
                    app.set_tasks(model);
                });
            }
            Err(err) => {
                eprintln!("Failed to load tasks: {}", err);
            }
        }
    });
}
pub fn setup_register_window_logic(client: Client, register_app: &RegisterWindow) {
    let register_weak = register_app.as_weak();
    let register_client = client.clone();

    register_app.on_register_clicked(move || {
        let app = register_weak.unwrap();

        let name = app.get_login_text().to_string();
        let password = app.get_password_text().to_string();

        let client = register_client.clone();
        let app_weak_inner = app.as_weak();

        tokio::spawn(async move {
            let result: Result<String, String> = rf::regist_user(client, name, password).await;

            match result {
                Ok(message) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        let app = app_weak_inner.unwrap();
                        app.set_status_text(message.into());

                        app.hide().unwrap();
                    });
                }
                Err(err) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        let app = app_weak_inner.unwrap();
                        app.set_status_text(err.into());
                    });
                }
            }
        });
    });
}

