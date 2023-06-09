use serde::ser::{Serialize, SerializeStruct, Serializer};

pub enum TaskStatus {
    DONE,
    PENDING,
}
impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("Input {} not supported!", input_string),
        }
    }
}
// implementing the Serde Serialize Trait for Task Status
impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Ok(serializer.serialize_str(&self.stringify().as_str())?)
        let mut s = serializer.serialize_struct("TaskStatus", 1)?;
        s.serialize_field("status", &self.stringify())?;
        s.end()
    }
}

// impl fmt::Display for TaskStatus {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             TaskStatus::DONE => {
//                 write!(f, "DONE")
//             }
//             TaskStatus::PENDING => {
//                 write!(f, "PENDING")
//             }
//         }
//     }
// }
