use piston::UpdateArgs;

pub trait Update {
    fn update(&mut self, updateargs: &UpdateArgs);
}