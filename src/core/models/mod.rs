use serde::{Deserialize, Serialize};

pub mod session;
pub mod account;

#[derive(Deserialize, Serialize)]
pub enum ModelValueType {
    String  {field: &'static str},
    Number  {field: &'static str},
    Object  {field: &'static str},
    Array   {field: &'static str},
    Boolean {field: &'static str}
}