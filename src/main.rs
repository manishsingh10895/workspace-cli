use db::{fetch_all_workspaces, insert_new_workspace};
use structopt::StructOpt;
use exitfailure::ExitFailure;
use workspace::Workspace;

use crate::visual::run_visual;
mod visual;
mod workspace;
mod db;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    ///Path of the directory to be added in the workspace
    path: Option<std::path::PathBuf>,

    #[structopt(short = "l", long = "list")]
    ///List all the workspaces and directories
    list: bool,

    #[structopt(short="v", long="visual")]
    visual: bool,

    #[structopt(short = "w", long = "workspace")]
    ///Workspace name 
    workspace: Option<String>
}

fn main() -> Result<(), ExitFailure> {
    println!("Hello, world!");
    db::initialize_db()?;    

    let options = Options::from_args();

    println!("{:?}", options);

    if options.visual {
        run_visual();

        return Ok(());
    }

    if options.list {
        fetch_all_workspaces();

        return Ok(());
    }

    match options.workspace {
        Some(val) => {
            let w = Workspace::new(val);

            insert_new_workspace(w)?;
        },
        None => {

        }
    }


    Ok(())
}
