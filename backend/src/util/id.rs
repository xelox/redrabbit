use std::fmt::Display;

use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, serialize::{IsNull, ToSql}, sql_types::Integer, sqlite::Sqlite};
use serde::{de::{Error, Unexpected, Visitor}, Deserialize, Serialize};

#[derive(Hash)]
#[derive(Clone, Debug, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Integer)]
pub struct Id {
    id: i32,
}

impl Serialize for Id
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.id.to_string().as_str())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}


impl FromSql<Integer, Sqlite> for Id {
    fn from_sql(bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        <i32 as FromSql<Integer, Sqlite>>::from_sql(bytes).map(|s| Self{id: s as i32})
    }
}

impl ToSql<Integer, Sqlite> for Id {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.id);
        Ok(IsNull::No)
    }
}


impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
where
        D: serde::Deserializer<'de> {
        struct TVisitor;
        impl<'de> Visitor<'de> for TVisitor {
            type Value = Id;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a number")    
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
            E: serde::de::Error, {
                match v.parse::<i32>() {
                    Ok(id) => Ok(Id{id}),
                    Err(_) => Err(Error::invalid_value(Unexpected::Str(&v.to_string()), &self))
                }
            }
        }

        deserializer.deserialize_str(TVisitor)
    }
}

impl From<i32> for Id {
    fn from(value: i32) -> Self {
        Id {id: value}
    }
}
