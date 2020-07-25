use crate::schema::transactions;
use crate::{db_conn, Error};
use common::chrono::{DateTime, Utc};
use diesel::{
    deserialize::FromSql,
    pg::Pg,
    prelude::*,
    serialize::{Output, ToSql},
    sql_types::Text,
};
use std::{
    fmt::{self, Debug},
    io::Write,
};

#[derive(Queryable, AsChangeset, Insertable, Debug, Associations, Clone)]
#[belongs_to(crate::Dialog)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub state: TransactionState,
    pub branch_id: String,
    pub dialog_id: i64,
}

#[derive(AsChangeset, Insertable, Debug, Default)]
#[table_name = "transactions"]
pub struct DirtyTransaction {
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub state: Option<TransactionState>,
    pub branch_id: Option<String>,
    pub dialog_id: Option<i64>,
}

pub struct LazyQuery<'a> {
    query: transactions::BoxedQuery<'a, diesel::pg::Pg>,
}

impl<'a> LazyQuery<'a> {
    pub fn new(query: transactions::BoxedQuery<'a, diesel::pg::Pg>) -> Self {
        Self { query }
    }

    pub fn branch_id(mut self, branch_id: Option<String>) -> Self {
        if let Some(branch_id) = branch_id {
            self.query = self.query.filter(transactions::branch_id.eq(branch_id));
        }
        self
    }

    pub fn paginate(mut self, page: i64, per_page: i64) -> Self {
        let offset = (page - 1) * per_page;

        self.query = self.query.offset(offset).limit(per_page);
        self
    }

    pub fn order_by_created_at(mut self) -> Self {
        self.query = self.query.order(transactions::created_at.desc());
        self
    }

    pub async fn load(self) -> Result<Vec<Transaction>, Error> {
        Ok(self.query.get_results(&db_conn()?)?)
    }

    pub async fn first(self) -> Result<Transaction, Error> {
        Ok(self.query.first(&db_conn()?)?)
    }
}

impl<'a> Transaction {
    pub fn query() -> LazyQuery<'a> {
        LazyQuery::new(transactions::table.into_boxed())
    }

    pub async fn find(id: i64) -> Result<Self, Error> {
        Ok(transactions::table.find(id).first::<Self>(&db_conn()?)?)
    }

    pub async fn create(record: DirtyTransaction) -> Result<Self, Error> {
        use diesel::insert_into;

        Ok(insert_into(transactions::table)
            .values(record)
            .get_result(&db_conn()?)?)
    }

    pub async fn update(record: DirtyTransaction, id: i64) -> Result<Self, Error> {
        Ok(
            diesel::update(transactions::table.filter(transactions::id.eq(id)))
                .set(&record)
                .get_result(&db_conn()?)?,
        )
    }

    pub async fn delete(id: i64) -> Result<Self, Error> {
        Ok(
            diesel::delete(transactions::table.filter(transactions::id.eq(id)))
                .get_result(&db_conn()?)?,
        )
    }
}

#[derive(FromSqlRow, AsExpression, Copy, Clone, PartialEq, Debug)]
#[sql_type = "Text"]
pub enum TransactionState {
    Trying,
    Proceeding,
    Completed,
    Terminated,
}

impl fmt::Display for TransactionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ToSql<Text, Pg> for TransactionState {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        <&str as ToSql<Text, Pg>>::to_sql(&self.to_string().to_lowercase().as_str(), out)
    }
}

impl FromSql<Text, Pg> for TransactionState {
    fn from_sql(bytes: Option<diesel::pg::PgValue>) -> diesel::deserialize::Result<Self> {
        use std::str::FromStr;

        Ok(TransactionState::from_str(
            <String as FromSql<Text, Pg>>::from_sql(bytes)?.as_ref(),
        )?)
    }
}

impl std::str::FromStr for TransactionState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("trying") => Ok(TransactionState::Trying),
            s if s.eq_ignore_ascii_case("proceeding") => Ok(TransactionState::Proceeding),
            s if s.eq_ignore_ascii_case("completed") => Ok(TransactionState::Completed),
            s if s.eq_ignore_ascii_case("terminated") => Ok(TransactionState::Terminated),
            s => Err(format!("invalid TransactionState `{}`", s)),
        }
    }
}