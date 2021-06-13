use core::fmt;
use std::{collections::HashMap, fmt::Formatter, path::PathBuf, process::Command};

use cursive::{
    align::HAlign,
    event::Key,
    views::{Button, EditView, LinearLayout, NamedView, ViewRef},
};
use cursive::{traits::*, views::ListView};
use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

#[derive(Debug)]
enum ViewNames {
    DirList,
    DirSelect,
}

impl fmt::Display for ViewNames {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

use crate::{
    db::{self, fetch_all_workspaces},
    workspace::{self, Dir, Workspace},
};

fn create_main_select() -> SelectView {
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();

    let choices = vec![
        String::from("List Workspaces"),
        String::from("Add a Workspace"),
    ];

    select.add_all_str(choices);

    select.set_on_submit(on_main_choice_select);

    return select;
}

fn open_path(path: &str) {
    let p = PathBuf::from(path);
    let output = Command::new("code.cmd").arg(p).spawn().unwrap();

    println!("{:?}", output);
}

fn make_workspace(workspaceId: i32, name: String, dirs: &Vec<(i32, String)>) -> Workspace {
    let mut w = Workspace::new(String::from(name));

    for d in dirs {
        w.add_dir(Dir::new(d.1.clone()).id(d.0));
    }

    w
}

fn open_workspace(workspace: Workspace) {
    workspace.dir_iter().for_each(|d| {
        let cmd = Command::new("code").arg(d.path.clone()).spawn();

        match cmd {
            Ok(child) => {
                print!("Code Instance Spawned for {} :> {}", d.path, child.id());
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    });
}

fn create_dir_list_view(workspaceId: i32) -> NamedView<SelectView<(usize, i32, String)>> {
    let mut select_view = SelectView::new().h_align(HAlign::Center);

    let paths = db::get_dirs_for_workspace(workspaceId).unwrap();

    for i in 0..paths.len() {
        let data = (i, paths[i].0, paths[i].1.clone());
        select_view.add_item(paths[i].1.clone(), data);
    }

    select_view.set_on_submit(|s: &mut Cursive, choice: &(usize, i32, String)| {
        println!("{:?}", choice);

        let id = choice.1;

        let path = choice.2.clone();

        s.add_layer(
            Dialog::new()
                .title(format!("Viewing {}", choice.1))
                .button("Delete", move |siv| {
                    println!("Deleting at index {}", id);

                    let res = db::remove_dir_from_workspace(id);

                    match res {
                        Err(e) => {
                            println!("Error removing dir");
                            println!("{}", e.to_string());
                        },
                        _ => {

                        }
                    }


                    siv.pop_layer();
                })
                .button("Open", move |siv| {
                    open_path(&path);
                    siv.pop_layer();
                })
                .button("Back", |siv: &mut Cursive| {
                    siv.pop_layer();
                }),
        );
    });

    let x = select_view.with_name("dir_list_view");

    return x;
}

fn on_view_workspace(workspace_hash: HashMap<String, i32>) -> Box<Fn(&mut Cursive, &str)> {
    return Box::new(move |siv: &mut Cursive, choice: &str| {
        println!("{}", workspace_hash[choice]);
        println!("{}", choice);

        let workspaceId = workspace_hash[choice];

        let work = Workspace::new(String::from("choice")).id(workspaceId);
        let paths = db::get_dirs_for_workspace(workspaceId).unwrap();
        let select_view = create_dir_list_view(workspaceId);

        let c = String::from(choice);

        siv.add_layer(Dialog::around(
            LinearLayout::vertical()
                .child(
                    ListView::new().child(
                        "New Path",
                        EditView::new()
                            .on_submit(move |s, value| {
                                println!("New Dir {}", value);

                                let res = db::insert_new_dir_for_workspace(
                                    workspaceId,
                                    String::from(value),
                                );

                                match res {
                                    Ok(id) => {
                                        // let mut x :ViewRef<SelectView<(i32, String)>> = s.find_name("dir_select_view").unwrap();
                                        s.pop_layer();
                                        // x.add_item(value, (id as i32, String::from(value)));
                                    }
                                    Err(e) => {
                                        println!("{}", e.to_string());
                                    }
                                }
                            })
                            .fixed_width(20),
                    ),
                )
                .child(
                    select_view
                        .min_size((20, 10))
                        .scrollable()
                        .with_name("dir_select_view"),
                )
                .child(Button::new("Open Workspace", move |_| {
                    let w = make_workspace(workspaceId, c.to_string(), &paths);
                    
                    open_workspace(w);
                })),
        ));
    });
}

fn on_add_workspace(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Enter the name of the workspace")
            .content(
                ListView::new().child(
                    "Name",
                    EditView::new()
                        .on_submit(|s, text| {
                            println!("{}", text);
                            let res = db::insert_new_workspace(workspace::Workspace::new(
                                String::from(text),
                            ));

                            match res {
                                Ok(_) => {
                                    s.pop_layer();
                                }
                                Err(e) => {
                                    panic!("{}", e.to_string());
                                }
                            }
                        })
                        .fixed_width(20)
                        .with_name("name"),
                ),
            ),
    );
}

fn on_main_choice_select(siv: &mut Cursive, choice: &str) {
    println!("Choice, {}", choice);
    match choice {
        "List Workspaces" => {
            println!("List Workspace");

            let mut workspace_hash: HashMap<String, i32> = HashMap::new();

            fetch_all_workspaces().unwrap().iter().for_each(|f| {
                workspace_hash.insert(f.1.clone(), f.0);
            });

            let mut select_view = SelectView::new().h_align(HAlign::Center).autojump();

            select_view.add_all_str(workspace_hash.keys());

            select_view.set_on_submit(on_view_workspace(workspace_hash));

            siv.add_layer(
                Dialog::around(select_view.scrollable().fixed_size((20, 10)))
                    .title("Your Workspaces"),
            );
        }
        "Add a Workspace" => {
            on_add_workspace(siv);
        }
        _ => {}
    }
}

pub fn run_visual() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(create_main_select().scrollable().fixed_size((20, 10)))
            .title("Choose from below"),
    );

    siv.add_global_callback('q', |s| s.quit());

    siv.add_global_callback(cursive::event::Event::Key(Key::Esc), |s| {
        s.pop_layer();
    });

    match siv.load_theme_file("assets/style.toml") {
        Ok(_) => {}
        Err(e) => {
            panic!("{:?}", e);
        }
    }

    siv.run();
}
