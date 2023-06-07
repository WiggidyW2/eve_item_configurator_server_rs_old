// Market Group, Category, Group
#[derive(Debug, Default, Clone)]
pub struct DivisionNames {
    pub indexes: Vec<u32>, // indexes into names
    pub names: Vec<String>,
}

impl DivisionNames {
    pub fn with_names(names: Vec<String>) -> Self {
        Self {
            indexes: Vec::new(),
            names: names,
        }
    }
}
