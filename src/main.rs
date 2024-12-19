use editor::Editor;

mod editor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(err) = Editor::default().run(args.get(1)) {
        println!("Error: {err}");
    }
}
