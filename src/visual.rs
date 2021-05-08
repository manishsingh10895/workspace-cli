use std::{
    collections::HashMap,
};

use cursive::{align::HAlign, event::Key, views::{EditView, LinearLayout}};
use cursive::{traits::*, views::ListView};
use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

use crate::{
    db::{self, fetch_all_workspaces},
    workspace,
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

fn create_dir_list_view(workspaceId: i32) -> SelectView<(i32, String)> {
    let mut select_view = SelectView::new().h_align(HAlign::Center).autojump();

    let paths = db::get_dirs_for_workspace(workspaceId).unwrap();


    for i in 0..paths.len() {
        select_view.add_item(paths[i].1.clone(), paths[i].clone());
    }

    select_view.set_on_submit(|s: &mut Cursive, choice: &(i32, String)| {
        println!("{:?}", choice);

        s.add_layer(
            Dialog::new()
                .title(format!("Viewing {}", choice.1))
                .button("Delete", |siv: &mut Cursive| {})
                .button("Open", |siv| {})
                .button("Back", |siv: &mut Cursive| {
                    siv.pop_layer();
                }),
        );
    });

    return select_view;
}

fn on_view_workspace(workspace_hash: HashMap<String, i32>) -> Box<Fn(&mut Cursive, &str)> {
    return Box::new(move |siv: &mut Cursive, choice: &str| {
        println!("{}", workspace_hash[choice]);
        println!("{}", choice);

        let workspaceId = workspace_hash[choice];   

        let select_view = create_dir_list_view(workspaceId);

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
                .child(select_view.min_size((20, 10)).scrollable().with_name("dir_select_view")),
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

    siv.load_theme_file("assets/style.toml");

    siv.run();

}
