use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Default, Clone)]
struct Directory {
    name: String,
    size: usize,
    files: Vec<(usize, String)>,
    subdirs: Vec<Rc<RefCell<Directory>>>
}

impl Directory {
    fn find_dir(&self, pwd: &String, mut cur_pwd: String) -> Option<Rc<RefCell<Self>>> {
        let mut ret = None;
        if cur_pwd != "/" { cur_pwd += "/"; }
        for c in self.clone().subdirs {
            if cur_pwd.to_owned() + c.borrow().name.as_str() == pwd.to_owned() {
                return Some(c.clone());
            }
            ret = c.borrow().find_dir(pwd, cur_pwd.to_owned() + c.borrow().name.as_str());
            if ret.is_some() {
                return ret;
            }
        }
        ret
    }

    fn calc_sizes(&mut self) {
        for d in &self.subdirs {
            d.borrow_mut().calc_sizes();
            self.size += d.borrow().size;
        }

        for (s, _) in &self.files {
            self.size += s;
        }
    }
}

fn process<'a>(input: String) -> Rc<RefCell<Directory>> {
    let mut pwd = String::from("/");
    let ret = Rc::new(RefCell::new(Directory {
        name: String::from("/"),
        ..Default::default()
    }));
    let mut cur_dir = ret.clone();

    for line in input.split("\n") {
        // dbg!(&ret);
        if line.starts_with("$ cd") && line.ends_with("..") {
            let tmp = pwd.split("/").collect::<Vec<&str>>();

            pwd = tmp[..tmp.len() - 1].join("/");
            if pwd.len() == 0 { 
                pwd = String::from("/");
                cur_dir = ret.clone();
            } else {
                cur_dir = ret.borrow().find_dir(&pwd, String::from("/")).unwrap();
            }
        } else if line.starts_with("$ cd") {
            let next_dir = line.split(" ").last().unwrap();

            if next_dir == "/" {
                continue;
            } else {
                if pwd.chars().last().unwrap() != '/' { pwd += "/"; }
                pwd += next_dir;
                cur_dir = ret.borrow().find_dir(&pwd, String::from("/")).unwrap();
            }
        } else if line.starts_with("$ ls") { 
            continue;
        } else if line.starts_with("dir") {
            let new_dir = Rc::new(RefCell::new(Directory {
                name: line.split(" ").last().unwrap().to_string(),
                ..Default::default()
            }));
            cur_dir.borrow_mut().subdirs.push(new_dir);
        } else if line.len() > 0 {
            let tmp = line.split(" ").collect::<Vec<&str>>();
            let size = tmp[0].parse().unwrap();

            let mut c = cur_dir.borrow_mut();
            c.files.push((size, tmp[1].to_owned()));
        }
    }

    ret.borrow_mut().calc_sizes();
    ret
}

pub fn solve1(input: String) -> usize {
    let root = process(input);

    let mut total = 0;

    let mut nodes = vec![root];
    while !nodes.is_empty() {
        let cur = nodes.pop().unwrap();
        let subdirs = cur.borrow().subdirs.clone();
        for x in subdirs {
            nodes.push(x.clone());
        }
        if cur.borrow().size < 100000 {
            total += cur.borrow().size
        }
    }

    total
}

pub fn solve2(input: String) -> usize {
    let root = process(input);
    let needed = 30000000 - (70_000_000 - root.borrow().size);
    let mut min = root.borrow().size;

    let mut nodes = vec![root];
    while !nodes.is_empty() {
        let cur = nodes.pop().unwrap();
        let subdirs = cur.borrow().subdirs.clone();
        for x in subdirs {
            nodes.push(x.clone());
        }
        if cur.borrow().size >= needed && cur.borrow().size < min {
            cur.borrow().size;
            min = cur.borrow().size;
        }
    }

    min
}