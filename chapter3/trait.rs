struct Dove;
struct Duck;

trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Tweet!");
    }
}

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}



fn main() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();
    let bird_vec : Vec<Box<dyn Tweet>> = vec![Box::new(Dove {}), Box::new(Duck {})];
    for bird in bird_vec {
        bird.tweet();
    }
}