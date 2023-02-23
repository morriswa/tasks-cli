
use std::cmp::Ordering;

use cursive::{
    Cursive, 
    view::{Nameable, Resizable},
    views::{Dialog, TextView, EditView, LinearLayout}, 
    align::HAlign, 
};
use cursive_table_view::{TableView, TableViewItem};
use serde_derive::{Serialize, Deserialize};

/*
    Author:         William A. Morris
    Created:        2023-02-22
    Last Modified:  2023-02-23
    Email:          William @ MorrisWA.org
    Description:    
        A simple Rust program to keep a list of Tasks. Impl w Cursive TUI Library.
*/

const DATASTORE: (&str, &str) = (".wma/","tasks.json");

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    ID,
    Name,
    Category,
    Due,
}

#[derive(Clone, Debug)]
struct ExTask {
    id: usize,
    name: String,
    category: String,
    due: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    category: String,
    due: String,
}

impl TableViewItem<BasicColumn> for ExTask {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::ID => format!("{}",self.id),
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Category => self.category.to_string(),
            BasicColumn::Due => self.due.to_string(),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::ID => self.id.cmp(&other.id),
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::Category => self.category.cmp(&other.category),
            BasicColumn::Due => self.due.cmp(&other.due),
        }
    }
}


fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Initialize Task Vec
    let tasks:Vec<Task> = read_from_json();
 
    // Render Homescreen
    render_home_table(&mut siv, tasks);
    
    // Starts the event loop.    
    siv.run();   
}

fn quit_n_save_menu(s: &mut Cursive,tasks: Vec<Task>) {
    s.pop_layer();
    s.add_layer(Dialog::around(TextView::new("Would you like to Save?"))
        .title("Exit Menu")
        .button("Yes!", move |s| {
            write_to_json(tasks.clone());
            s.quit()
        })
        .button("No :(", move |s| {
            s.quit();
        }));
}

fn new_task(s: &mut Cursive, given_tasks:Vec<Task>) {
    let given_tasks_move_for_callback = given_tasks.clone();

    let layout = LinearLayout::vertical()
        .child(TextView::new("Task Name"))
        .child(EditView::new()
            .with_name("name")
            .fixed_width(20))
        .child(TextView::new("Category"))
        .child(EditView::new()
            .with_name("cat")
            .fixed_width(20))
        .child(TextView::new("Due Date"))
        .child(EditView::new()
            .with_name("due")
            .fixed_width(11));

    s.pop_layer();
    s.add_layer(Dialog::around(layout)
        .title("Add Task Menu")
        .button("Ok", move|s| {
            let mut tasks = given_tasks_move_for_callback.clone();
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            let cat =
                s.call_on_name("cat", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            let due =
                s.call_on_name("due", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            tasks.push(Task { 
                name: name.to_string(), 
                category: cat.to_string(), 
                due: due.to_string(),
            });
            render_home_table(s,tasks);
        })
        .button("Cancel", move |s| {
            render_home_table(s,given_tasks.clone());
        }));    
}

fn del_task(s: &mut Cursive, tasks: Vec<Task>) {
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
                render_home_table(s,ftasks);
            } else {
                ftasks.remove(todel);
                render_home_table(s,ftasks); 
            }
        })
        .button("Cancel", move |s| {
            render_home_table(s,tasks.clone());
        }));     
}

fn render_home_table(s: &mut Cursive, tasks: Vec<Task>) {
    let tasks_closure_add_task: Vec<Task> = Vec::from(tasks.clone());
    let tasks_closure_del_task: Vec<Task> = Vec::from(tasks.clone()); 

    let items = tasks.clone();
    let mut exitems: Vec<ExTask> = vec![];
    for (i, task) in items.iter().enumerate() {
        exitems.push(ExTask { id: i, name: task.name.clone(), category: task.category.clone(), due: task.due.clone() });
    }

    let table = TableView::<ExTask, BasicColumn>::new()
        .column(BasicColumn::ID, "ID", |c| c.width_percent(10))
        .column(BasicColumn::Name, "Name", |c| c.width_percent(40))
        .column(BasicColumn::Category, "Category", |c| {
            c   .align(HAlign::Center)
                .width_percent(30)            
        })
        .column(BasicColumn::Due, "Due", |c| {
            c   .ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(20)
        })
        .items(exitems)
        .min_size((100, 20));

    s.pop_layer();
    s.add_layer(Dialog::around(table)
        .title("Task Table")
        .button("Add Task", move |s| {
            new_task(s, tasks_closure_add_task.clone());
        })
        .button("Del Task", move |s| {
            del_task(s, tasks_closure_del_task.clone());
        }) 
        .button("Quit", move |s| {
            quit_n_save_menu(s, tasks.clone())
        }));
}

fn write_to_json(vec: Vec<Task>) {
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

fn read_from_json() -> Vec<Task> {
    let home_dir = dirs::home_dir().unwrap();
    let path = home_dir.as_path().join(DATASTORE.0).join(DATASTORE.1);
    let db_content = std::fs::read_to_string(path.as_path()).unwrap_or_default();
    let parsed: Vec<Task> = serde_json::from_str(&db_content).unwrap_or_default();
    parsed
}