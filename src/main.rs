

use cursive::{views::{Dialog, TextView, EditView, LinearLayout}, Cursive, view::{Nameable, Resizable}};

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Initialize Task Vec
    let tasks:Vec<String> = vec![];
    
    // Render Homescreen
    render_home(&mut siv, tasks);
    
    // Starts the event loop.    
    siv.run();   
}

fn render_home(s: &mut Cursive, tasks: Vec<String>) {
    let ftasks: Vec<String> = Vec::from(tasks.clone());
    let mut message = String::new();
    if (ftasks.len() <= 0) {
        message = String::from("No Tasks!");
    } else {
        message = String::from("Task List: ");
    }

    let mut list = LinearLayout::vertical();
    list.insert_child(0, TextView::new(message.clone()));
    for (i, task) in tasks.iter().enumerate() {
        let new_string = format!("{}) {}", i, task.to_string());
        list.insert_child(i+1,TextView::new(new_string));
    }

    s.pop_layer();
    s.add_layer(Dialog::around(list)
        .title("Tasks")
        .button("Add Task", move |s| {
            new_task(s, ftasks.clone());
        }
        )
        .button("Quit", move |s| {
            print_final_list(s, tasks.clone());
        })); 
}

fn print_final_list(s: &mut Cursive,tasks: Vec<String>) {
    s.pop_layer();

    let mut list = LinearLayout::vertical();
    for (i, task) in tasks.iter().enumerate() {
        let new_string = format!("{}) {}", i, task.to_string());
        list.insert_child(i,TextView::new(new_string));
    }
    
    s.add_layer(Dialog::around(list)
        .title("Tasks")
        .button("Quit", move |s| {
            s.quit();
        }));
}

fn new_task(s: &mut Cursive,tasks:Vec<String>) {
    let ftasks = tasks.clone();

    s.pop_layer();
    s.add_layer(Dialog::around(EditView::new()
            .with_name("name")
            .fixed_width(20))
        .title("Enter a new message")
        .button("Ok", move|s| {
            let mut fftasks = ftasks.clone();
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            fftasks.push(name.to_string());
            render_home(s,fftasks);
        })
        .button("Cancel", move |s| {
            render_home(s,tasks.clone());
        }));    
}