# SeaORM CRUD
### Usage

```rust
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskListItem {
  pub id: Uuid,

  #[serde(default)] // <-- Required
  pub priority: Nullable<i32>,

  pub env_file: Optional<String>,
}
```
