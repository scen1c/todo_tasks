use reqwest::Client;
use std::cell::RefCell;
use std::rc::Rc;

use crate::reqwest_functions as rf;
use crate::{RegisterWindow, UserWindow, WelcomeWindow, TaskItem};

use slint::ComponentHandle;
use slint::{ModelRc, VecModel};

thread_local! {
    static USER_WINDOW_HOLDER: RefCell<Option<UserWindow>> = RefCell::new(None);
    static REGISTER_WINDOW_HOLDER: RefCell<Option<RegisterWindow>> = RefCell::new(None);
    static TASKS_MODEL_HOLDER: RefCell<Option<Rc<VecModel<TaskItem>>>> = RefCell::new(None);
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

        let login_name = name.clone(); // для запроса
        let ui_name = name.clone();    // для окна

        let client = login_client.clone();
        let app_weak_inner = app.as_weak();

        tokio::spawn(async move {
            let result: Result<rf::LoginResponse, String> =
                rf::login_user(client.clone(), login_name, password).await;

            match result {
                Ok(data) => {
                    let token = data.access_token;
                    let client_for_ui = client.clone();

                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak_inner.upgrade() {
                            app.set_status_text("Login successful".into());
                            open_user_panel(client_for_ui, token, &app, ui_name);
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

pub fn open_user_panel(client: Client, token: String, welcome_app: &WelcomeWindow, name: String) {
    let window = UserWindow::new().unwrap();
    let window_weak = window.as_weak();

    let tasks_model = Rc::new(VecModel::<TaskItem>::default());
    window.set_tasks(ModelRc::from(tasks_model.clone()));

    TASKS_MODEL_HOLDER.with(|slot| {
        *slot.borrow_mut() = Some(tasks_model);
    });

    setup_user_window_logic(token.clone());

    let client_for_create = client.clone();
    let token_for_create = token.clone();
    let window_for_create = window_weak.clone();

    window.on_create_task_clicked(move || {
        let app_weak = window_for_create.clone();
        let token = token_for_create.clone();
        let client = client_for_create.clone();

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

    let client_for_checkbox = client.clone();
    let token_for_checkbox = token.clone();
    let window_for_checkbox = window_weak.clone();

    window.on_checkbox_clicked(move |title, checked| {
    let client = client_for_checkbox.clone();
    let token = token_for_checkbox.clone();
    let app_weak = window_for_checkbox.clone();

    tokio::spawn(async move {
        let result = rf::finish_task(client, token.clone(), title.to_string(), checked).await;

        match result {
            Ok(_) => {
                let reload_result = load_tasks_into_model(token.clone()).await;

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        match reload_result {
                            Ok(_) => app.set_status_text("Task updated".into()),
                            Err(err) => app.set_status_text(
                                format!("Updated, but refresh failed: {}", err).into()
                            ),
                        }
                    }
                });
            }
            Err(err) => {
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        app.set_status_text(format!("Failed to update task: {}", err).into());
                    }
                });
            }
        }
    });
});

    let client_for_delete = client.clone();
    let token_for_delete = token.clone();
    let window_for_delete = window_weak.clone();

    window.on_delete_task_clicked(move || {
        let client = client_for_delete.clone();
        let token = token_for_delete.clone();
        let app_weak = window_for_delete.clone();

        let id_raw = {
            let Some(app) = app_weak.upgrade() else {
                return;
            };
            app.get_delete_task().to_string()
        };

        tokio::spawn(async move {
           let result = rf::delete_task(client, token.clone(), id_raw).await; 

           match result {
                Ok(_) => {
                    let reload_result = load_tasks_into_model(token.clone()).await;
                    
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak.upgrade() {
                            match reload_result {
                                Ok(_) => {
                                    app.set_status_text("Task deleted successfuly".into());
                                    app.set_delete_task("".into());
                                },
                                Err(err) => {
                                    app.set_status_text(format!("Task deleted but couldn't refresh. {}", err).into());
                                }
                            }
                        }
                    });
                },

                Err(err) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(app) = app_weak.upgrade() {
                            app.set_status_text(format!("Error to delete task. Error code: {}", err).into());
                        }
                    });
                }
           }
        });
    });

    window.set_username(name.into());

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

    let tasks: Vec<TaskItem> = data
    .tasks
    .into_iter()
    .map(|t| TaskItem {
        id: t.id,
        title: t.title.into(),
        completed: t.completed,
    })
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