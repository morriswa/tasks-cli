
use cursive::{views::{Dialog, TextView, EditView}, Cursive, view::{Nameable, Resizable}};


fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();
    
    render_home(&mut siv,"Hello World!");

    // Starts the event loop.    
    siv.run();
}

fn render_home(s: &mut Cursive, message: &str) {
    // let message:Option<String> = s.take_user_data();
    // let message:String = message.unwrap();
    
    // Creates a dialog with a single "Quit" button
    s .add_layer(Dialog::around(TextView::new(message
    ))
        .title("Welcome")
        .button("Dummy", |s| {
            s.pop_layer();
            rewrite_message(s);
        }
        )
        .button("Quit", |s| s.quit()));
}


fn rewrite_message(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.set_user_data(name.to_string());
        s.pop_layer();
        render_home(s, name);
    }

    fn bye(s: &mut Cursive) {
        s.set_user_data("Default".to_string());
        s.pop_layer();
        render_home(s, "Hello Default!");
    }

    s.add_layer(Dialog::around(EditView::new()
            .with_name("name")
            .fixed_width(20))
        .title("Enter a new message")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {bye(s)}));    

}
