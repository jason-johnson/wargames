mod hex;
mod ptp;

pub trait Board {
    fn get_x(&self) -> u32;

    fn mutate(&mut self, y: String);
}