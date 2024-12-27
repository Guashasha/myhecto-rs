use editor::Editor;

mod editor;

fn main() {
    log4rs::init_file("logger_config.yml", Default::default())
        .expect("An error occurred while starting the logger");

    let args: Vec<String> = std::env::args().collect();
    let mut editor: Editor;

    match Editor::new(args.get(1)) {
        Err(err) => panic!("Error: {err}"),
        Ok(new_editor) => editor = new_editor,
    }

    if let Err(err) = editor.run() {
        panic!("Error: {err}");
    }
}
