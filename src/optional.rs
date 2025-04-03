use sea_orm::sea_query::{ArrayType, ValueType, ValueTypeErr};
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
  ActiveValue, ColIdx, ColumnType, IntoActiveValue, QueryResult, Set, TryGetError, TryGetable, Value,
};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct Optional<T>(pub Option<T>);

impl<T> Optional<T> {
  pub fn is_none(&self) -> bool {
    self.0.is_none()
  }

  pub fn is_some(&self) -> bool {
    self.0.is_some()
  }
}

impl<T> Default for Optional<T>
where
  T: Default,
{
  fn default() -> Self {
    Optional(Some(T::default()))
  }
}

impl<T> Serialize for Optional<T>
where
  T: Into<Value> + Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match &self.0 {
      Some(v) => v.serialize(serializer),
      None => serializer.serialize_none(),
    }
  }
}

impl<'de, T> Deserialize<'de> for Optional<T>
where
  T: Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(Optional(Option::deserialize(deserializer)?))
  }
}

impl<T> IntoActiveValue<T> for Optional<T>
where
  T: Into<Value>,
{
  fn into_active_value(self) -> ActiveValue<T> {
    match self.0 {
      Some(v) => Set(v),
      None => NotSet,
    }
  }
}

impl<T> From<Optional<T>> for Value
where
  T: Into<Value> + sea_orm::sea_query::Nullable,
{
  fn from(x: Optional<T>) -> Value {
    match x.0 {
      Some(v) => v.into(),
      None => T::null(),
    }
  }
}

impl<T> ValueType for Optional<T>
where
  T: ValueType + sea_orm::sea_query::Nullable,
{
  fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
    if v == T::null() {
      Ok(Optional(None))
    } else {
      Ok(Optional(Some(T::try_from(v)?)))
    }
  }

  fn type_name() -> String {
    format!("Optional<{}>", T::type_name())
  }

  fn array_type() -> ArrayType {
    T::array_type()
  }

  fn column_type() -> ColumnType {
    T::column_type()
  }
}

impl<T: TryGetable> TryGetable for Optional<T> {
  fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
    Ok(Optional(Option::<T>::try_get_by(res, index)?))
  }
}
