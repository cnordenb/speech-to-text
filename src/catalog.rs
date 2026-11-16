#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Model {
    Tiny,
    Base,
    #[default]
    Small,
}

impl Model {
    pub const ALL: [Model; 3] = [Model::Tiny, Model::Base, Model::Small];

    pub fn name(self) -> &'static str {
        match self {
            Model::Tiny => "tiny",
            Model::Base => "base",
            Model::Small => "small",
        }
    }
}
