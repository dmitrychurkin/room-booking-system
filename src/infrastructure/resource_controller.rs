use rocket_db_pools::{Connection, Database};

pub trait Index<TDatabase>
where
    TDatabase: Database,
{
    fn index(db: Connection<TDatabase>) -> &'static str;
}

pub trait Show {
    fn show() -> &'static str;
}

pub trait Create {
    fn create() -> &'static str;
}

pub trait Update {
    fn update() -> &'static str;
}

pub trait Delete {
    fn delete() -> &'static str;
}

pub trait ResourceController<TDatabase>:
    Index<TDatabase> + Show + Create + Update + Delete
where
    TDatabase: Database,
{
}
