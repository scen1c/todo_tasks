use reqwest::Client;
use std::cell::RefCell;
use std::rc::Rc;

use crate::reqwest_functions as rf;
use crate::{RegisterWindow, UserWindow, WelcomeWindow};

use slint::ComponentHandle;
use slint::{ModelRc, SharedString, VecModel};

thread_local! {
    static USER_WINDOW_HOLDER: RefCell<Option<UserWindow>> = RefCell::new(None);
    static REGISTER_WINDOW_HOLDER: RefCell<Option<RegisterWindow>> = RefCell::new(None);
    static TASKS_MODEL_HOLDER: RefCell<Option<Rc<VecModel<SharedString>>>> = RefCell::new(None);
}

pub fn beginprogram(client: Client, app: &WelcomeWindow) {
    let login_weak = app.as_weak();
    let login_client = client.clone();

    app.on_login_clicked(move || {
        let Some(app) = login_weak.upgrade() else {
            return;
        };

        let name = app.get_login_text().to_string();
        let password = app.get_password_text().to_string();

        let client = login_client.clone();
        let app_weak_inner = app.as_weak();

        tokio::spawn(async move {
            let result: Result<rf::LoginResponse, String> =
                rf::login_user(client.clone(), name, password).await;

            match result {
                Ok(data) => {
                    let token = data.access_token;
                    let client_for_ui = client.clone();

                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak_inner.upgrade() {
                            app.set_status_text("Login successful".into());
                            open_user_panel(client_for_ui, token, &app);
                        }
                    });
                }
                Err(err) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak_inner.upgrade() {
                            app.set_status_text(err.into());
                        }
                    });
                }
            }
        });
    });

    let register_weak = app.as_weak();
    let register_client = client.clone();

    app.on_register_clicked(move || {
        let Some(app) = register_weak.upgrade() else {
            return;
        };

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
    let window_weak = window.as_weak();

    let tasks_model = Rc::new(VecModel::<SharedString>::default());
    window.set_tasks(ModelRc::from(tasks_model.clone()));

    TASKS_MODEL_HOLDER.with(|slot| {
        *slot.borrow_mut() = Some(tasks_model);
    });

    setup_user_window_logic(token.clone());

    let client_inner = client.clone();
    let token_inner = token.clone();
    let window_inner = window_weak.clone();

    window.on_create_task_clicked(move || {
        let app_weak = window_inner.clone();
        let token = token_inner.clone();
        let client = client_inner.clone();

        let title = {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            app.get_create_task().to_string()
        };

        tokio::spawn(async move {
            let result = rf::create_task(client, token.clone(), title).await;

            match result {
                Ok(_) => {
                    let reload_result = load_tasks_into_model(token.clone()).await;

                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak.upgrade() {
                            match reload_result {
                                Ok(_) => {
                                    app.set_status_text("Task created successfully".into());
                                    app.set_create_task("".into());
                                }
                                Err(err) => {
                                    app.set_status_text(
                                        format!("Task created, but refresh failed: {}", err).into(),
                                    );
                                }
                            }
                        }
                    });
                }
                Err(err) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak.upgrade() {
                            app.set_status_text(format!("Error: {}", err).into());
                        }
                    });
                }
            }
        });
    });

    window.show().unwrap();

    USER_WINDOW_HOLDER.with(|slot| {
        *slot.borrow_mut() = Some(window);
    });

    welcome_app.hide().unwrap();
}

pub fn setup_user_window_logic(token: String) {
    tokio::spawn(async move {
        let result = load_tasks_into_model(token).await;

        if let Err(err) = result {
            let err_text = format!("Failed to load tasks: {}", err);

            let _ = slint::invoke_from_event_loop(move || {
                USER_WINDOW_HOLDER.with(|slot| {
                    if let Some(app) = slot.borrow().as_ref() {
                        app.set_status_text(err_text.clone().into());
                    }
                });
            });
        }
    });
}

pub fn setup_register_window_logic(client: Client, register_app: &RegisterWindow) {
    let register_weak = register_app.as_weak();
    let register_client = client.clone();

    register_app.on_register_clicked(move || {
        let Some(app) = register_weak.upgrade() else {
            return;
        };

        let name = app.get_login_text().to_string();
        let password = app.get_password_text().to_string();

        let client = register_client.clone();
        let app_weak_inner = app.as_weak();

        tokio::spawn(async move {
            let result: Result<String, String> = rf::regist_user(client, name, password).await;

            match result {
                Ok(message) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak_inner.upgrade() {
                            app.set_status_text(message.into());
                            app.hide().unwrap();
                        }
                    });
                }
                Err(err) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak_inner.upgrade() {
                            app.set_status_text(err.into());
                        }
                    });
                }
            }
        });
    });
}

pub async fn load_tasks_into_model(token: String) -> Result<(), reqwest::Error> {
    let data: rf::ListTaskResponse = rf::get_tasks(&token).await?;

    let tasks: Vec<SharedString> = data
        .tasks
        .into_iter()
        .map(|t| SharedString::from(t.title))
        .collect();

    let _ = slint::invoke_from_event_loop(move || {
        TASKS_MODEL_HOLDER.with(|slot| {
            if let Some(model) = slot.borrow().as_ref() {
                model.set_vec(tasks);
            }
        });
    });

    Ok(())
}