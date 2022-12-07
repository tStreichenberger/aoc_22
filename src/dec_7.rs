use crate::{
    AOCResult,
    CustomError
};
use std::{
    cmp::Reverse,
    collections::{
        BinaryHeap,
        VecDeque
    },
    rc::Rc,
    sync::RwLock,
};

pub fn solution_1(input: String) -> AOCResult<u32> {
    let root_dir = build_root_dir(input)?;
    let mut total = 0;
    let mut queue = VecDeque::new();
    queue.push_back(root_dir.clone());
    while !queue.is_empty() {
        let searching = queue.pop_front().unwrap();
        let size = searching.read().unwrap().size();
        if size <= 100000 {total += size;}
        for content in searching.read().unwrap().contents.iter() {
            if let Content::Dir(child) = content {
                queue.push_back(child.clone())
            }
        }
    }
    Ok(total)
}

pub fn solution_2(input: String) -> AOCResult<u32> {
    let root_dir = build_root_dir(input)?;
    let mut heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut queue = VecDeque::new();
    queue.push_back(root_dir.clone());
    while !queue.is_empty() {
        let searching = queue.pop_front().unwrap();
        let size = searching.read().unwrap().size();
        if size >= 8381165 {heap.push(Reverse(size));}
        for content in searching.read().unwrap().contents.iter() {
            if let Content::Dir(child) = content {
                queue.push_back(child.clone())
            }
        }
    }
    Ok(heap.pop().unwrap().0)
}



fn build_root_dir(input: String) -> AOCResult<Rc<RwLock<Dir>>> {
    let root_dir = Rc::new(RwLock::new(Dir::new("/")));
    let mut curr_dir = root_dir.clone();
    for line in input.split("\n") {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        match line_split[0] {
            "$" => {
                if line_split[1] == "cd" {
                    match line_split[2] {
                        ".." => {
                            let par = curr_dir.read().unwrap().get_parent()?;
                            curr_dir = par;
                        },
                        "/" => curr_dir = root_dir.clone(),
                        dir_name => {
                            let child = curr_dir.write().unwrap().get_child_dir(dir_name)?;
                            curr_dir = child;
                        }
                    };
                }
            },
            "dir" => {
                let mut child = Dir::new(line_split[1]);
                child.add_parent(curr_dir.clone());
                let dir = Rc::new(RwLock::new(child));
                curr_dir.write().unwrap().add(Content::Dir(dir));
            },
            file_size => {
                let size = str::parse::<u32>(file_size)?;
                let file = File {name: line_split[1].to_string(), size};
                let content = Content::File(file);
                curr_dir.write().unwrap().add(content);
            }
        }
    }
    Ok(root_dir)
}


struct Dir {
    name: String,
    parent: Option<Rc<RwLock<Dir>>>,
    contents: Vec<Content>
}


struct File {
    name: String, 
    size: u32
}


enum Content {
    File(File),
    Dir(Rc<RwLock<Dir>>)
}

impl Content {
    fn name(&self) -> String {
        match self {
            Content::File(file) => file.name.clone(),
            Content::Dir(dir) => dir.read().unwrap().name.clone()
        }
    }

    fn size(&self) -> u32 {
        match self {
            Content::File(file) => file.size,
            Content::Dir(dir) => dir.read().unwrap().size()
        }
    }
}


impl Dir {
    fn new(name: &str) -> Self {
        Dir {name: name.to_string(), contents: Vec::new(), parent: None}
    }

    fn add_parent(&mut self, dir: Rc<RwLock<Dir>>) {
        self.parent = Some(dir);
    }

    fn get_parent(&self) -> AOCResult<Rc<RwLock<Dir>>> {
        match self.parent.clone() {
            Some(par) => Ok(par),
            None => Err(CustomError("Director Has No Parent".to_string()).into())
        }
    }

    // adds content if it does not already exist
    fn add(&mut self, content: Content) {
        if !self.contains(content.name()) {
            self.contents.push(content);
        }
    }

    fn contains(&mut self, name: String) -> bool {
        for content in self.contents.iter() {
            if content.name().eq(&name) {return true;}
        }
        return false
    }

    fn size(&self) -> u32 {
        let mut total = 0;
        for content in self.contents.iter() {
            total += content.size();
        };
        total
    }

    fn get_child_dir(&self, name: &str) -> AOCResult<Rc<RwLock<Dir>>> {
        for content in self.contents.iter() {
            if let Content::Dir(dir) = content {
                if dir.read().unwrap().name.eq(name) {return Ok(dir.clone())}
            }
        }
        return Err(CustomError(format!("Tried to Access Child {name} from dir {}. No Child found", self.name)).into())
    }
}