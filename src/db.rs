use diesel::prelude::*;
use crate::models::*;

pub fn select_link(conn: &mut PgConnection, req_alias: &String) -> QueryResult<Link> {
    use crate::schema::links::dsl::*;
    links
        .filter(alias.eq(req_alias))
        .first::<Link>(conn)
}

pub fn insert_link(conn: &mut PgConnection, new_link: NewLink) -> QueryResult<Link> {
    use crate::schema::links::dsl::*;

    diesel::insert_into(links)
        .values(new_link)
        .returning(Link::as_returning())
        .get_result(conn)
}