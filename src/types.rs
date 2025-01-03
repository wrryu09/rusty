use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TestObject{
    pub id: i32,
    pub name: String,
    pub desc: String
}

#[derive(Clone, Debug)]
pub struct ObjectList{
    pub object: TestObject,
    pub quantity: i32
}