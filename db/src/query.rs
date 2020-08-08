///query.rs
///Construct sqlx queries from a model and parameters of form
/// SELECT * [Operation::Select(vec[Field])] 
/// FROM Users, Records [DbQuery.condition]
/// WHERE User.id = Record.uid AND Record.permission = "public"
/// GROUP BY (count(id))

use chrono::{DateTime, Utc};
use common::models::Model;
use sqlx::prelude::*;
use sqlx::postgres::{PgPool, PgConnection, PgDone};
use std::boxed::Box;
use common::models::{User, Record, Item};
use serde::{Serialize, Deserialize};

//TODO consider making this all into some sort of macro maybe?
//Works for select
//
//
pub trait SQLQuery {}

//#[serde{Deserialize, Serialize}]
pub struct SelectQuery where {
    pool: PgPool,
    alias: Option<String>,
    with: Vec<SelectQuery>,
    select: Vec<SelectClause>,
    //#[serde(flatten)]
    from: Vec<FromClause>,
    cond: Vec<WhereClause>,
    group_by: Vec<SelectField>,
    having: Vec<Condition>,
    order_by: Vec<Order>,
    bind: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateQuery where {
    pool: PgPool,
    update: Vec<SelectField>,
    set: Vec<SelectField>,
    cond: Vec<Condition>,
    query_string: String,
}

pub struct DeleteQuery where {
    pool: PgPool,
    delete: Vec<SelectField>,
    cond: Vec<Condition>,
    bind: Option<Vec<String>>,
    query_string: String,
}

impl SelectQuery {

    pub fn new(pool: PgPool, alias: Option<String>) -> Self {
        Self { pool, alias, ..Vec::new() }
    }

    pub fn from<T: Queryable>(mut self, model: T, alias: Option<String>) -> Self {
        let table = model.table();
        let from = FromClause::from_table(model.table());
        self.from.push(from);
    }

    pub fn select<T: Queryable>(mut self, model: T, column: &str) -> SelectClause { 
        let tables = self.from.to_vec();
        let column = Column {
            table: Some(model.table()), col: column
        };
        let select = SelectClause::from_column(column);
        self.select.push(select);
        select
    }


    pub fn select_all(mut self, modif: Option<SelectModifier>) -> Self {
        let field = SelectField::Wildcard(Wildcard::All);
        let modifier = match self.select.len() {
            0 => SelectModifier::Beginning,  
            _ => modif.unwrap() 
        };
        let sel_claus = SelectClause { modifier, table: None, column: None,  alias: None };
        self.select.push(sel_claus);
        self
    }
        
    pub fn from<T: Queryable>(mut self, model: T) -> FromClause {
        let from_clause = FromClause { table: FromField::Table(model.table()), alias: None };
        from_clause
    }

    pub fn and_where(mut self, column: Column) -> WhereClause { 
        if self.cond.len() > 0 {
            let mut where_clause = WhereClause::from_modifier(self, WhereModifier::And);
            move |where_clause| {
                where_clause.with_column(column)
            }
        } else {  WhereClause::new(self) } 
    }

    pub fn and_not_where(mut self, column: Column) -> WhereClause { 
        let mut where_clause = WhereClause::from_modifier(self, WhereModifier::Not);
        move |where_clause| {
            where_clause.with_column(column)
        }
    } 

    pub fn or_where(self, column: Column) -> WhereClause { 
        let mut where_clause = WhereClause::from_modifier(self, WhereModifier::Or);
        move |where_clause| {
            where_clause.with_column(column) 
        }
    } 

    pub fn where_modifier(self, modifier: WhereModifier) -> WhereClause {
         match self.cond.len() {
            0 => WhereClause::new(),
            _ => WhereClause::with_modifier(modifier),
        }
    }

    pub async fn query_as<T: Queryable>(mut self) -> sqlx::Result<T> {
       let query = sqlx::query_as::<_, T>("SELECT $1 FROM $2 WHERE $3"); 
       self.bind.into_iter().map(*query.bind());
       let res = self.pool.execute(query).await?;
       Ok
    }
}


pub enum Existential {
    Exists(Column, SelectQuery),
    In(Column, SelectQuery),
    Between(Column, SelectQuery),
    Union(Column, SelectQuery),
    Except(Column, SelectQuery),
    Intercept(Column, SelectQuery),
}

pub enum TableField {
    Table(Table),
    Query(SelectQuery),
    Column(Column),
}


pub struct SelectClause {
    modifier: Option<SelectModifier>,
    table: Option<Table>,
    column: Option<Column>,
    alias: Option<String>,
}

impl SelectClause {

    pub fn from_model<T: Queryable>(model: T, alias: Option<String>, modifier: Option<SelectModifier>) {
        Self { modifier, table: model.table(), column: None, alias: None }
    }
}

pub struct FromClause {
    table: FromField,
    alias: String,
}

pub struct WhereClause {
    modifier: Option<WhereModifier>,
    condition: Option<Condition>,
    column: Option<Column>,
    target: Option<WhereField>,
    query: Box<dyn SQLQuery>,
}

impl WhereClause {
    pub fn new(
        query: impl SQLQuery,
        modifier: Option<WhereModifier>,
        condition: Option<Condition>,
        column: Option<Column>,
        target: Option<WhereField>
    ) { 
        Self {
            query: Box::new(query),
            modifier: WhereModifier::Beginning,
            condition: if Some(condition) { condition } else { None }, 
            column: if Some(column) { column } else { None },
            target: if Some(target) { target } else { None },
        }
    }

    pub fn from_modifier(query: impl SQLQuery, modifier: WhereModifier) {
        WhereClause {
            query: Box::new(query), modifier, ..Self::new()
        }
    }

    pub fn from_column(query: impl SQLQuery, column: Column) { 
        WhereClause {
            query: Box::new(query), column: Some(column), ..Self::new()
        }
    }

    pub fn equals<T: SQLPrimitive>(mut self, value: T) -> Self {
        self.condition = Condition::Equals(value);
        if let val = SSome(value).to_sql_value() {
            self.target = WhereField::Primitive(value.to_sql_value());
        } else {
            if let col = Some(field) {
                self.target = WhereField::Column(col);
            } 
        }
        self
    }

    pub fn does_not_equal<T: SQLPrimitive>(mut self, value: Option<T>, field: Option<Column>) -> Self {
        self.condition = Condition::DoesNotEquals(value);
        if let val = SSome(value).to_sql_value() {
            self.target = WhereField::Primitive(value.to_sql_value());
        } else {
            if let col = Some(field) {
                self.target = WhereField::Column(col);
            } 
        }
    }

    pub fn with_modifier(mut self, modifier: WhereModifier) { 
        self.modifier = modifier; self
    }

    pub fn with_olumn(mut self, column: Column) {
        self.column = column; self
    }
}

pub enum SelectModifier {
    Top,
    Distinct,
}

pub enum WhereModifier {
    Beginning, 
    And,
    Or,
    AndNot,
}

pub enum Condition {
    Equals,
    LessThan,
    GreaterThan,
    EqualLessThan,
    EqualGreaterThan,
    DoesNotEqual,
    Like(Column, String),
    Similar(Column, String),
    IsNull(Column),
    IsNotNull(Column),
    Existential(Existential),
}

pub enum SQLGroupBy {
    
}

pub enum Order {
    Ascending(FromField),
    Descending(FromField),
}

pub enum SelectField {
    Wildcard(Wildcard),
    Column(Column),
    Query(SelectQuery),
    Aggregate { col: Column, aggregate: Aggregate },
}

pub enum WhereField {
    Column(Column),
    Date(DateTime<Utc>),
    Primitive(SQLValue),
    Bind(Bind),
}

pub enum Bind {
    BindInt(i32),
    BindString(String),
    BindBool(bool),
    BindDt(DateTime<Utc>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FromField {
    Table(Table),
    Column(Column),
    Query(SelectQuery),
}
pub enum Wildcard {
    All,
    OnePlus,
    ZeroOrOne,
    NonGreedyOnePlus,
    NonGreedyZeroPlus,
    NonGreedyZeroOrOne,
    Atomic { m: String },
    AtomimcM { m: String },
    AtomimcMToN { m: String, n: String },
    NGAtomic { m: String },
    NGAtomimcM { m: String },
    NGAtomimcMToN { m: String, m: String },
}

pub enum Aggregate {
    Avg(Column),
    Count(Column),
    Max(Column),
    Min(Column),
    StdDev(Column),
    Sum(Column),
    Variance(Column),
    Every(Column),
    Regr_R2(Column, Column),
    Corr(Column, Column), //etc and on and on 
}

pub enum SQLValue {

    Int(i32),
    Text(String),
    Timestamptz(DateTime<Utc>),
    Boolean(bool),
    Null,
}

pub trait SQLPrimitive {
    fn to_sql_value(self)  -> SQLValue;
    fn to
    fn to_sql_value()  -> SQLValue;
}
impl SQLPrimitive for i32 {
    fn to_sql_value(self) -> SQLValue { SQLValue::Int(self) }
}
impl SQLPrimitive for String {
    fn to_sql_value(self) -> SQLValue { SQLValue::String(self) }
}
impl SQLPrimitive for bool {
    fn to_sql_value(self) -> SQLValue { SQLValue::Boolean(self) }
}

pub struct Table(String);
pub struct Column { table: Option<Table>, col: String }

impl From<&dyn Model> for Table {
    fn from(model: &dyn Model) -> Self {
        Self(model.table()) 
    }
}

impl Column {
    pub fn new(col: String) -> Self { Self { table: None, col} }

    pub fn to_dot_string(self)  -> String {
        format!("{:?}.{}", Some(self.table), col)
    }
    
}




pub struct Having { 
    cond: Column,
    cond: Condition,
    field: Field,
}

pub enum Join {
    CrossJoin(Table, Table),
    InnerJoin(Table, Table),
    LeftOuterJoin(Table, Table),
    RightOuterJoin(Table, Table),
    FullOuterJoin(Table, Table),
}

pub trait Queryable: Model {
    fn table(self) -> String { 
        match self {
            User => { String::from("Users") },
            Record => { String::from("Record") },
            _ => { String::from("") },
        }
    }
    fn id() -> i32 { 0 }
}

impl<T: Model> Queryable for T {
 
}


impl<T: Queryable> DbQuery<T> {

    pub async fn new(pool: PgPool) -> () {
    } 
    pub async fn from(pool: PgPool, model: T) -> () {
    } 

    pub async fn execute(self) -> sqlx::Result<u32> {
        let que = sqlx::query("");
        let res = que.execute(&self.pool).await?;
        Ok(res.rows_affected() as u32)
    }
}

impl DbQuery<User> {

}

impl DbQuery<Record> {

}

impl DbQuery<Item> {

}

impl DbQuery<Group> {

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn table_from_model_works() {
        let user = User::new("test@test.com", "test", "pass");
        let user_table = user.table();
        let table: Table = Table::from(user);
        assert_eq!(user_table table)
    }
}

