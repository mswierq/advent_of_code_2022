use std::cell::RefCell;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::str::FromStr;
use std::{fs::File, io};

enum CdType {
    Into { name: String },
    Up,
    Root,
}

enum Command {
    CD { cd_type: CdType },
    LS,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        if Some("$") != parts.next() {
            return Err("It's not a command!".to_owned());
        }

        return match parts.next() {
            Some("cd") => match parts.next() {
                Some("/") => Ok(Command::CD {
                    cd_type: CdType::Root,
                }),
                Some("..") => Ok(Command::CD {
                    cd_type: CdType::Up,
                }),
                Some(name) => Ok(Command::CD {
                    cd_type: CdType::Into {
                        name: name.to_owned(),
                    },
                }),
                None => Err("CD command is missing an argument!".to_owned()),
            },
            Some("ls") => Ok(Command::LS),
            Some(command) => Err(format!("Couldn't recognize the command {}", command)),
            None => Err("Empty command!".to_owned()),
        };
    }
}

enum FileType {
    Dir {
        parent: Weak<RefCell<FileType>>,
        name: String,
        files: Vec<Rc<RefCell<FileType>>>,
    },
    File {
        size: usize,
        name: String,
    },
}

impl FileType {
    fn size(&self) -> usize {
        match *self {
            Self::File { size, name: _ } => size,
            Self::Dir {
                parent: _,
                name: _,
                ref files,
            } => files.iter().map(|file| file.borrow().size()).sum(),
        }
    }

    fn name(&self) -> &str {
        match *self {
            Self::File { size: _, ref name } => name,
            Self::Dir {
                parent: _,
                ref name,
                files: _,
            } => name,
        }
    }
}

struct FileSystem {
    root_dir: Rc<RefCell<FileType>>,
    current_dir: Weak<RefCell<FileType>>,
}

impl FileSystem {
    fn new() -> Self {
        let root_dir = Rc::new(RefCell::new(FileType::Dir {
            parent: Weak::new(),
            name: "/".to_owned(),
            files: vec![],
        }));
        Self {
            root_dir: Rc::clone(&root_dir),
            current_dir: Rc::downgrade(&root_dir),
        }
    }

    fn add_dir(&mut self, name: String) {
        let parent = self.current_dir.clone();
        if let FileType::Dir {
            parent: _,
            name: _,
            files,
        } = self
            .current_dir
            .upgrade()
            .expect("The current dir is broken! :(")
            .borrow_mut()
            .deref_mut()
        {
            files.push(Rc::new(RefCell::new(FileType::Dir {
                parent,
                name,
                files: vec![],
            })));
        }
    }

    fn add_file(&mut self, name: String, size: usize) {
        if let FileType::Dir {
            parent: _,
            name: _,
            files,
        } = self
            .current_dir
            .upgrade()
            .expect("The current dir is broken! :(")
            .borrow_mut()
            .deref_mut()
        {
            files.push(Rc::new(RefCell::new(FileType::File { name, size })));
        }
    }

    fn change_directory(&mut self, cd_type: &CdType) {
        if let FileType::Dir {
            parent,
            name: _,
            files,
        } = self
            .current_dir
            .upgrade()
            .expect("The current dir is broken! :(")
            .borrow()
            .deref()
        {
            match cd_type {
                CdType::Into { name } => {
                    let file = files
                        .iter()
                        .find(|f| f.borrow().deref().name() == name)
                        .expect("Couldn't find a file ");
                    self.current_dir = Rc::downgrade(file);
                }
                CdType::Root => {
                    self.current_dir = Rc::downgrade(&self.root_dir);
                }
                CdType::Up => {
                    self.current_dir = parent.clone();
                }
            }
        }
    }
}

// Finds directories that have overall size of 10000 and less.
// Returns a vector of pointers to those directories.
fn find_dirs_to_remove(dir: Rc<RefCell<FileType>>) -> Vec<Rc<RefCell<FileType>>> {
    if let FileType::Dir {
        parent: _,
        name: _,
        files,
    } = dir.borrow().deref()
    {
        let mut result: Vec<Rc<RefCell<FileType>>> = vec![];

        if dir.borrow().deref().size() <= 100000 {
            result.push(Rc::clone(&dir));
        }

        for file in files {
            result.append(&mut find_dirs_to_remove(Rc::clone(file)));
        }
        return result;
    }

    vec![]
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut file_system = FileSystem::new();
    let mut read_output = false;
    while let Some(Ok(ref line)) = lines.next() {
        if line.starts_with("$") {
            read_output = false;
            let command = Command::from_str(line).unwrap();
            match command {
                Command::CD { ref cd_type } => file_system.change_directory(cd_type),
                Command::LS {} => read_output = true,
            }
        } else if read_output {
            let mut split = line.split(" ");
            let dir_or_size = split.next();
            match dir_or_size {
                Some("dir") => {
                    file_system.add_dir(split.next().expect("DIR does not have a name!").to_owned())
                }
                Some(size) => file_system.add_file(
                    split.next().expect("FILE does not have a name!").to_owned(),
                    size.parse().expect("Unknown size!"),
                ),
                None => panic!("Unknown output: {}!", line),
            }
        } else {
            panic!("I don't know how to interpret this {}", line);
        }
    }

    let dirs_to_remove = find_dirs_to_remove(file_system.root_dir);
    let freed_up_space: usize = dirs_to_remove
        .iter()
        .map(|file| file.borrow().deref().size())
        .sum();

    println!("Freed up space: {}", freed_up_space);

    Ok(())
}
