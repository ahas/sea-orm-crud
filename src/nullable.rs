use sea_orm::sea_query::{ArrayType, ValueType, ValueTypeErr};
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
  ActiveValue, ColIdx, ColumnType, IntoActiveValue, QueryResult, Set, TryGetError, TryGetable, Value,
};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Nullable<T> {
  Value(T),
  Null,
  Undefined,
}

impl<T> Default for Nullable<T> {
  fn default() -> Self {
    Nullable::<T>::Undefined
  }
}

impl<T> Serialize for Nullable<T>
where
  T: Into<Value> + Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match &self {
      Nullable::<T>::Value(v) => v.serialize(serializer),
      Nullable::<T>::Null => serializer.serialize_none(),
      Nullable::<T>::Undefined => serializer.serialize_unit(),
    }
  }
}

impl<'de, T> Deserialize<'de> for Nullable<T>
where
  T: Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct NullableVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> Visitor<'de> for NullableVisitor<T>
    where
      T: Deserialize<'de>,
    {
      type Value = Nullable<T>;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a value that could be defined, null, or undefined")
      }

      fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
      where
        D: Deserializer<'de>,
      {
        T::deserialize(deserializer).map(Nullable::Value)
      }

      fn visit_none<E>(self) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Nullable::<T>::Null)
      }

      fn visit_unit<E>(self) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Nullable::<T>::Undefined)
      }
    }

    deserializer.deserialize_option(NullableVisitor(std::marker::PhantomData))
  }
}

impl<T> IntoActiveValue<Option<T>> for Nullable<T>
where
  T: Into<Value> + sea_orm::sea_query::Nullable,
{
  fn into_active_value(self) -> ActiveValue<Option<T>> {
    match self {
      Nullable::Value(v) => Set(Some(v)),
      Nullable::Null => Set(None),
      Nullable::Undefined => NotSet,
    }
  }
}

impl<T> From<Nullable<T>> for Value
where
  T: Into<Value> + sea_orm::sea_query::Nullable,
{
  fn from(x: Nullable<T>) -> Value {
    match x {
      Nullable::Value(v) => v.into(),
      _ => T::null(),
    }
  }
}

impl<T> ValueType for Nullable<T>
where
  T: ValueType + sea_orm::sea_query::Nullable,
{
  fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
    if v == T::null() {
      Ok(Nullable::Null)
    } else {
      Ok(Nullable::Value(T::try_from(v)?))
    }
  }

  fn type_name() -> String {
    format!("Nullable<{}>", T::type_name())
  }

  fn array_type() -> ArrayType {
    T::array_type()
  }

  fn column_type() -> ColumnType {
    T::column_type()
  }
}

impl<T: TryGetable> TryGetable for Nullable<T> {
  fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
    match Option::<T>::try_get_by(res, index)? {
      Some(v) => Ok(Nullable::Value(v)),
      None => Ok(Nullable::Null),
    }
  }
}

impl<T> From<Option<T>> for Nullable<T> {
  fn from(value: Option<T>) -> Self {
    match value {
      Some(v) => Nullable::Value(v),
      None => Nullable::Null,
    }
  }
}
