use super::parser::Cmd;

pub struct Program {
    pub cmds: Vec<Cmd>,

    pub version: String,

    pub author: String,

    pub about: String,
}

impl Program {
    pub fn new() -> Self {
        Self {
            cmds: vec![],
            version: "0.1.0".to_owned(),
            author: "".to_owned(),
            about: "".to_owned(),
        }
    }

    pub fn version(&mut self, vers: &str) -> &mut Program {
        self.version = vers.to_string();
        self
    }

    pub fn author(&mut self, auth: &str) -> &mut Program {
        self.author = auth.to_string();
        self
    }

    pub fn description(&mut self, desc: &str) -> &mut Program {
        self.about = desc.to_string();
        self
    }

    pub fn add_cmd() -> Cmd {
        Cmd::new()
    }

    pub fn parse() {}
    pub fn output_help() {}
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_prog() {
        let mut auto = Program::new();
        auto.author("me").description("a test");

        let manual = Program {
            cmds: vec![],
            version: "0.1.0".to_string(),
            author: "me".to_string(),
            about: "a test".to_string(),
        };

        assert_eq!(auto.author, manual.author);
        assert_eq!(auto.about, manual.about);
    }
}
