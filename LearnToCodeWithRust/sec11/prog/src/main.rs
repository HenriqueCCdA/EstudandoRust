#[derive(Debug)]
enum DigitalConent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalConent> {

    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }

}

impl<T> ChatMessage<T> {

    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}


fn main() {
    let message = ChatMessage {
        content: "Hi, lol",
        time: String::from("2025-03-12"),
    };
    println!("{}", message.retrieve_time());

    let notification = ChatMessage {
        content: String::from("What's your favorite pizza topping?"),
        time:  String::from("2025-04-12"),
    };
    println!("{}", notification.retrieve_time());

    let audio = ChatMessage {
        content: DigitalConent::AudioFile,
        time: String::from("2025-05-12"),
    };
    audio.consume_entertainment();


}
