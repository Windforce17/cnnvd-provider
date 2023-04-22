pub struct lastupdate {
    pub lastupdate: i64,
}
impl lastupdate {
    pub fn create_table(){
        let create_table_stm=r#"
        create table if not exists lastupdate(
            lastupdate bigint
        );
        "#
        return sqlx::query(create_table_stm);
    }
    pub fn get_lastupdate() -> i64 {
        self.lastupdate
    }
}
