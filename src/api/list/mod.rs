pub mod orgs;

pub struct List;

impl List {
    pub fn orgs(&self) -> orgs::ListOrgsBuilder {
        orgs::ListOrgsBuilder::new()
    }
}
