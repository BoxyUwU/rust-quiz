struct Data {
    name: String,
    bones: Vec<String>,
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)?;
        for bone in &self.bones {
            write!(f, " {}", bone)?;
        }
        writeln!(f, "")
    }
}

fn main() {
    let data = Data {
        name: "Boxy".into(),
        bones: vec!["fakebone1".into(), "fakebone2".into()],
    };
    dbg!(data)
}
