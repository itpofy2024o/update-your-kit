use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug]
pub struct Music {
	pub id:u32,
	pub name: String,
	pub author: String,
}
