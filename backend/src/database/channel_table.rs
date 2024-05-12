

// The complete User-Relation struct.
#[derive(Queryable, Selectable, Insertable, Identifiable)]
#[derive(Serialize)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelTable {

}
