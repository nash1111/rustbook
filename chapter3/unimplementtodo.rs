enum Emotion {
    Anger,
    Happy,
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}

struct HappyPerson {
    name: String,
    emotion: Emotion,
}

impl Emotional for HappyPerson {
    fn get_happy(&mut self) -> String {
        self.emotion = Emotion::Happy;
        format!("{} is happy", self.name)
    }

    fn get_anger(&mut self) -> String {
        unimplemented!()
        // 何らかの理由で実装されていない, not yet implemented
    }

    fn tell_state(&self) -> String {
        // 今後実装しなければならない, not implemented
        todo!()
    }
}

fn main() {
    let mut p = HappyPerson {
        name: "John".to_string(),
        emotion: Emotion::Anger,
    };
    println!("{}", p.get_happy());
}