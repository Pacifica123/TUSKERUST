use sqlx::Row;

#[derive(serde::Serialize)]
pub struct Taskboard {
    pub id: i32,
    pub name: String,
    pub owner_id: i32
}

// список таскбордов
#[derive(serde::Serialize)]
pub struct ListTaskboards {
    pub success: bool,
    pub taskboards: Vec<Taskboard>
}

// конструктор таскборда
impl Taskboard {
    pub fn new(id: i32, name: String, owner_id: i32) -> Taskboard {
        Taskboard {
            id,
            name,
            owner_id
        }
    }
}