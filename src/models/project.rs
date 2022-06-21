use oso::PolarClass;

#[derive(PolarClass, Queryable)]
#[diesel(table_name = projects)]
pub struct Project {
    #[polar(attribute)]
    pub id: i32,
    pub name: String,
}
