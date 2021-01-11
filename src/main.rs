use std::fs;
use std::process;
use std::path::Path;
use std::io;
use std::sync::mpsc;
use std::thread;
use clap::{Arg, App};
// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::time::Duration;

// static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let matches = App::new("Hunter")
    .version("1.0")
    .author("eltneg <eltneg@prjct.dev>")
    .about("Hunt for a folder")
    .arg(Arg::new("path")
        .about("Full path to hunt")
        .required(true)
        .index(1))
    .arg(Arg::new("del")
        .short('d')
        .long("delete")
        .about("Delete node modules")
        .takes_value(false))
    .arg(Arg::new("query")
        .short('q')
        .long("query")
        .about("Search query")
        .required(true)
        .takes_value(true)).get_matches();

    let mut path = String::from("");
    let mut query = String::new();
    let delt = matches.is_present("del");
    if let Some(p) = matches.value_of("path") {
        path = String::from(p);
        println!("Value for path: {}", path);
    }
    
    if let Some(q) = matches.value_of("query") {
        query = String::from(q);
        println!("Value for query: {}", query);
    }

    let mut all_files: Vec<String> = [].to_vec();
    let (tx, rx) = mpsc::channel();

    walk_dir(Path::new(&path), query, tx);



    // while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
    //     thread::sleep(Duration::from_millis(1)); 
    // }

    for recv in rx {
        all_files.push(recv);
    }

    println!("All files: {:#?}", all_files);


    if delt {
        println!("A total of {} files will be deleted. Proceed? y/n", all_files.len());
        let mut conti = String::new();
        io::stdin().read_line(&mut conti).expect("Failed to readline");
        let conti = conti.trim() == "y";
        if conti {
            println!("Deleting...");
            for folder in all_files {
                fs::remove_dir_all(Path::new(&folder)).unwrap_or_else(|err| {
                    println!("Error removing dir");
                    println!("{}", err);
                    process::exit(0) 
                })
            }
            println!("Delete completed!")
        }
    }
}

fn walk_dir(dir: &Path, query: String, tx: mpsc::Sender<std::string::String>) {
    // GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    let files = fs::read_dir(dir).unwrap_or_else(|err| {
        println!("Error reading dir");
        println!("{}", err);
        process::exit(0)
    });

    for file in files {
        let file = file.unwrap();
        if file.path().is_dir(){
            let s = String::from(file.path().to_string_lossy());
            if s.ends_with(&query){
                tx.send(s).unwrap();
            }else {
                let tx =  mpsc::Sender::clone(&tx);
                let q = query.clone();
                thread::spawn(move || {
                    walk_dir(&file.path(), q, tx);
                });
            }
        }
    }
    // GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
}
