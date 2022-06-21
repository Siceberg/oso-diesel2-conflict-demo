use oso::PolarClass;

#[derive(PolarClass, Queryable)]
#[diesel(table_name = accounts)]
pub struct Account {
    #[polar(attribute)]
    pub id: i32,
    pub email: String,
}
