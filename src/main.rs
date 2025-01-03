use editor::Editor;

mod editor;

fn main() {
    if let Err(err) = Editor::default().run() {
        println!("Error: {err}");
    }
}
