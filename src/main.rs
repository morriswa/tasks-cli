
use cursive::{
    Cursive, 
    view::{
        Nameable, Resizable
    },
    views::{
        Dialog, TextView, EditView, LinearLayout
    }, 
};

/*
    Author:         William A. Morris
    Created:        2023-02-22
    Last Modified:  2023-02-23
    Email:          William @ MorrisWA.org
    Description:    
        A simple Rust program to keep a list of Tasks. Impl w Cursive TUI Library.
*/

const DATASTORE: (&str, &str) = (".wma/","tasks.json");

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Initialize Task Vec
    let tasks:Vec<String> = read_from_json();
    
    // Render Homescreen
    render_home(&mut siv, tasks);
    
    // Starts the event loop.    
    siv.run();   
}

fn render_home(s: &mut Cursive, tasks: Vec<String>) {
    let tasks_closure_add_task: Vec<String> = Vec::from(tasks.clone());
    let tasks_closure_del_task: Vec<String> = Vec::from(tasks.clone()); 

    let message: String;
    if tasks.len() <= 0 {
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
            new_task(s, tasks_closure_add_task.clone());
        })
        .button("Del Task", move |s| {
            del_task(s, tasks_closure_del_task.clone());
        }) 
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
        .title("Task List")
        .button("Save N Quit", move |s| {
            write_to_json(tasks.clone());
            s.quit()
        })
        .button("Quit without Save", move |s| {
            s.quit();
        }));
}

fn new_task(s: &mut Cursive, given_tasks:Vec<String>) {
    let given_tasks_move_for_callback = given_tasks.clone();

    s.pop_layer();
    s.add_layer(Dialog::around(EditView::new()
            .with_name("name")
            .fixed_width(20))
        .title("Add Task Menu")
        .button("Ok", move|s| {
            let mut tasks = given_tasks_move_for_callback.clone();
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            tasks.push(name.to_string());
            render_home(s,tasks);
        })
        .button("Cancel", move |s| {
            render_home(s,given_tasks.clone());
        }));    
}

fn del_task(s: &mut Cursive, tasks: Vec<String>) {
    let tasks_for_callback = tasks.clone();

    s.pop_layer();
    s.add_layer(Dialog::around(EditView::new()
            .with_name("todel")
            .fixed_width(2))
        .title("Delete Task Menu")
        .button("Ok", move|s| {
            let mut ftasks = tasks_for_callback.clone();
            let todel =
                s.call_on_name("todel", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            let todel: usize = str::parse::<usize>(&todel).unwrap_or(ftasks.len());
            if todel >= ftasks.len() {
                render_home(s,ftasks);
            } else {
                ftasks.remove(todel);
                render_home(s,ftasks); 
            }
            
        })
        .button("Cancel", move |s| {
            render_home(s,tasks.clone());
        }));     
}

fn write_to_json(vec: Vec<String>) {
    let home_dir = dirs::home_dir().unwrap();
    let dir = home_dir.as_path().join(DATASTORE.0);
    if !dir.is_dir() {
        if std::fs::create_dir_all(dir.clone()).is_err() {
            panic!("Stopped due to fs error"); 
        };
    }
    let json_rep = serde_json::to_vec(&vec).unwrap_or_default();
    if std::fs::write(dir.join(DATASTORE.1).as_path(), json_rep).is_err() {
        panic!("Stopped due to error serializing task vec");
    }
}

fn read_from_json() -> Vec<String> {
    let home_dir = dirs::home_dir().unwrap();
    let path = home_dir.as_path().join(DATASTORE.0).join(DATASTORE.1);
    let db_content = std::fs::read_to_string(path.as_path()).unwrap_or_default();
    let parsed: Vec<String> = serde_json::from_str(&db_content).unwrap_or_default();
    parsed
}