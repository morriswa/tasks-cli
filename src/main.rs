
use cursive::{views::{Dialog, TextView, EditView}, Cursive, view::{Nameable, Resizable}};


fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    let message = String::from("Hello World!");
    
    siv.add_layer(Dialog::around(TextView::new(message.clone()))
        .title("Welcome")
        .button("Dummy", move |s| {
            rewrite_message(s,message.clone());
        })
        .button("Quit", |s| s.quit()));
    
        // Starts the event loop.    
    siv.run();
    
}

fn re_render_home(s: &mut Cursive, new_message: String) {
    let message:String = String::from(new_message);
    s.pop_layer();
    s.add_layer(Dialog::around(TextView::new(message.clone()))
        .title("Welcome")
        .button("Dummy", move |s| {
            rewrite_message(s, message.clone());
        }
        )
        .button("Quit", |s| s.quit())); 
}

fn rewrite_message(s: &mut Cursive,old_msg:String) {
    s.pop_layer();
    s.add_layer(Dialog::around(EditView::new()
            .with_name("name")
            .fixed_width(20))
        .title("Enter a new message")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
                re_render_home(s,name.to_string());
        })
        .button("Cancel", move |s| {
            re_render_home(s,old_msg.to_string());
        }));    
}