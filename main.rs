using std::collections::HashMap;

fn main() {
    let mut cap: HashMap<String, u8> = HashMap::new();

    cap.insert("Master".to_String(), "NO-EMAIL".to_String());
    println!("{#?}", cap);
}
