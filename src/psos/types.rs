pub struct User {
    pub name: String,
    pub password: String,
    pub full_name: String,
    pub email: String,
    pub gpg_key_id: String,
}

pub struct InstallState {
    pub timezone: String,
    pub keyboard_layout: String,
    pub hostname: String,
    pub user: User,
}

pub static mut STATE: InstallState = InstallState {
    timezone: "".to_string(),
    keyboard_layout: "".to_string(),
    hostname: "".to_string(),
    user: User {
        name: "".to_string(),
        full_name: "".to_string(),
        password: "".to_string(),
        email: "".to_string(),
        gpg_key_id: "".to_string(),
    },
};
