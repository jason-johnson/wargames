#[derive(Debug)]
pub enum PoliticalStatus {
    Russian,
    Teutonic,
}

#[derive(Debug)]
pub struct LocaleInfo {
    pub name: String,
    pub conquerable: bool,
    pub value: i32,
    pub political_status: PoliticalStatus,
}

#[derive(Debug)]
pub enum Ways {
    Waterway,
    Trackway,
}