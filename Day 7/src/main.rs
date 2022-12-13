use std::{fs, path::Path, cell::RefCell, collections::HashMap, vec};

struct Dir {
    subdirs: Vec<String>,
    files: Vec<FileEntry>,
    pwd: String,
    parent: String,
    size: i32,
}

struct FileEntry {
    size: i32,
    _name: String,
}

impl Dir {
    fn add_dir(self: &mut Self, dir: String) {
        self.subdirs.push(dir);
    }
    
    fn add_file(self: &mut Self, file: FileEntry) {
        self.files.push(file);
    }

    fn get_size(self: &mut Self, db: &HashMap<String, RefCell<Dir>>) -> i32 {
        if self.size <= 0 {
            self.size = self.files.iter().map(|f| f.size).sum::<i32>() + 
                self.subdirs.iter().map(|s| db.get(s).unwrap().borrow_mut().get_size(db)).sum::<i32>();
        }
        self.size
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let mut directories = HashMap::new();

    directories.insert("/".to_string(), RefCell::new(Dir{subdirs: vec![], files: vec![], pwd: "/".to_string(), parent: "/".to_string(), size: -1}));

    let mut curr: String = String::from("/");

    for line in data.lines() {
        match line.get(0..4).unwrap() {
            "$ cd" => {
                let target = line.get(5..).unwrap();
                if target == ".." {
                    curr = String::from(directories.get(&curr).unwrap().borrow().parent.clone());
                } else if target == "/" {
                    curr = String::from(target);
                } else {
                    curr = String::from(curr + target + "/");
                }
            },
            "$ ls" => {},
            "dir " => {
                let name = String::from(line.get(4..).unwrap());

                let path = directories.get(&curr).unwrap().borrow().pwd.clone() + &name.clone() + "/";

                let new_dir = Dir{subdirs: vec![], files: vec![], pwd: path.clone(), parent: curr.to_string(), size: -1};
                if directories.insert(path.clone(), RefCell::new(new_dir)).is_some() {
                    panic!("Duplicate directories");
                }
                directories.get(&curr).unwrap().borrow_mut().add_dir(path.clone());
            },
            _ => {
                let size = line.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
                let name = String::from(line.split(" ").last().unwrap());
                let new_file = FileEntry{size, _name: name.clone()};
                directories.get(&curr).unwrap().borrow_mut().add_file(new_file);
            },
        }
    }

    let part_1: i32 = directories.iter().map(|(_,v)| v.borrow_mut().get_size(&directories)).filter(|i| i.clone() <= 100000).sum();

    let total_size = directories.get("/").unwrap().borrow_mut().get_size(&directories);
    let unused_space: i32 = 70000000 - total_size;
    let min_size = 30000000 - unused_space;

    let mut large_folders = directories.iter().map(|(_,v)| v.borrow_mut().get_size(&directories)).filter(|i| i.clone() >= min_size).collect::<Vec<i32>>();
    large_folders.sort();

    let part_2 = large_folders[0];

    print!("Part 1: {},\nPart 2: {}", part_1, part_2);
}
