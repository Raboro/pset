use super::ci_step::CiStep;

#[derive(Clone, Default, Debug)]
pub struct Job {
    pub name: String,
    pub steps: Vec<CiStep>,
}

impl Job {
    pub fn to_ref_name(&self) -> String {
        let mut chars = self.name.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
