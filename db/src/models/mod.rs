pub mod channel;
pub mod message;

pub use channel::*;
use diesel::{PgConnection, result::Error as DieselError};
pub use message::*;

pub trait DieselCrud<T>
where
    Self: Sized,
{
    type Id;
    type NewItem;
    type Table;

    /// Create a new record
    fn create(conn: &mut PgConnection, item: &Self::NewItem) -> Result<Self, DieselError>;

    /// Read all records
    fn read_all(conn: &mut PgConnection) -> Result<Vec<Self>, DieselError>;

    /// Read a record by ID
    fn read_by_id(conn: &mut PgConnection, id: Self::Id) -> Result<Self, DieselError>;

    /// Update a record by ID
    fn update(
        conn: &mut PgConnection,
        id: Self::Id,
        item: &Self::NewItem,
    ) -> Result<Self, DieselError>;

    /// Delete a record by ID
    fn delete(conn: &mut PgConnection, id: Self::Id) -> Result<usize, DieselError>;
}

/// Macro to automatically implement DieselCrud trait
///
/// Usage: impl_diesel_crud!(ModelName, NewModelName, table_name, id_field, id_type);
///
/// Example: impl_diesel_crud!(User, NewUser, users, id, i32);
#[macro_export]
macro_rules! impl_diesel_crud {
    ($model:ty, $new_model:ty, $table:ident, $id_field:ident, $id_type:ty) => {
        impl DieselCrud<$model> for $model {
            type Id = $id_type;
            type NewItem = $new_model;
            type Table = crate::schema::$table::table;

            fn create(
                conn: &mut diesel::pg::PgConnection,
                item: &Self::NewItem,
            ) -> Result<Self, diesel::result::Error> {
                use crate::schema::$table;
                diesel::insert_into($table::table)
                    .values(item)
                    .get_result(conn)
            }

            fn read_all(
                conn: &mut diesel::pg::PgConnection,
            ) -> Result<Vec<Self>, diesel::result::Error> {
                use crate::schema::$table;
                $table::table.load::<$model>(conn)
            }

            fn read_by_id(
                conn: &mut diesel::pg::PgConnection,
                id: Self::Id,
            ) -> Result<Self, diesel::result::Error> {
                use crate::schema::$table;
                $table::table
                    .filter($table::$id_field.eq(id))
                    .first::<$model>(conn)
            }

            fn update(
                conn: &mut diesel::pg::PgConnection,
                id: Self::Id,
                item: &Self::NewItem,
            ) -> Result<Self, diesel::result::Error> {
                use crate::schema::$table;
                diesel::update($table::table.filter($table::$id_field.eq(id)))
                    .set(item)
                    .get_result(conn)
            }

            fn delete(
                conn: &mut diesel::pg::PgConnection,
                id: Self::Id,
            ) -> Result<usize, diesel::result::Error> {
                use crate::schema::$table;
                diesel::delete($table::table.filter($table::$id_field.eq(id))).execute(conn)
            }
        }
    };
}
