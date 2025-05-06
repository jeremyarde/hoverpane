pub mod db_impl {
    pub trait DbTable {
        // fn get_table_name() -> &'static str;
        // fn get_create_table_sql() -> &'static str;
        fn get_insert_sql() -> &'static str;
    }
}
