use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum Line {
    ChangeDirectory(String),
    ChangeDirectoryUp,
    List,
    Directory(String),
    File(File),
}

fn parse(input: &str) -> Vec<Line> {
    input.trim().split('\n')
        .map(|line|{
            let mut line = line.chars().collect::<Vec<_>>();
            if line[0] == '$'  && line[2] == 'c' && line[5] == '.'  {
                Line::ChangeDirectoryUp
            } else  if line[0] == '$'  && line[2] == 'c' {
                line.remove(0);
                line.remove(0);
                line.remove(0);
                line.remove(0);
                line.remove(0);
                Line::ChangeDirectory(line.into_iter().collect())
            } else if line[0] == '$'  && line[2] == 'l' {
                Line::List
            } else if line[0] == 'd' {
                line.remove(0);
                line.remove(0);
                line.remove(0);
                line.remove(0);
                Line::Directory(line.into_iter().collect())
            } else {
                let str = line.into_iter().collect::<String>();
                let mut split = str.split_ascii_whitespace();
                let size = split.next().unwrap().parse::<u32>().unwrap();
                let name = split.next().unwrap();

                Line::File(File{name:name.to_string(), size})
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct File {
    name:String,
    size:u32,
}

#[derive(Debug, Clone)]
struct DirWithSize {
    name: String,
    size: u32,
}

#[derive(Debug, Clone)]
enum Node {
    Dir(String, Vec<Arc<Mutex<Node>>>),
    File(String, u32),
}
impl Node {
    fn name(&self) -> String {
        match self {
            Node::Dir(n, _) => n.clone(),
            Node::File(n, _) => n.clone(),
        }
    }
    fn get_child_by_name(&self, name:String) -> Option<Arc<Mutex<Node>>> {
        match self {
            Node::Dir(_, children) => {
                children.iter().find(|n| n.lock().unwrap().name() == name).cloned()
            }
            Node::File(_, _) => None,
        }
    }
    fn mkdir(&mut self, dir:Node) {
        match self {
            Node::Dir(_, dirs) => { dirs.push(Arc::new(Mutex::new(dir))) }
            Node::File(_, _) => { panic!("cant happen") }
        }
    }
    fn create(&mut self, file:File) {
        match self {
            Node::Dir(_, dirs) => { dirs.push(Arc::new(Mutex::new(Node::File(file.name, file.size)))) }
            Node::File(_, _) => { panic!("cant happen") }
        }
    }
    fn size(&self) -> u32 {
        match self {
            Node::Dir(_, files) => {
                files.iter()
                    .map(|file|file.lock().unwrap().size())
                    .sum()
            }
            Node::File(_, size) => *size
        }
    }
    fn dirs_with_sizes(&self) -> Vec<DirWithSize>{
        if let Node::Dir(_,children) = self {
            let mut result = vec![];
            result.push(DirWithSize {name: self.name(), size: self.size()});
            for child in children {
                result.append(&mut child.lock().unwrap().dirs_with_sizes());
            }
            result
        } else {
            vec![]
        }

    }
}
#[derive(Debug)]
struct FileSystem {
    root: Arc<Mutex<Node>>,
    pwd: Vec<String>,
}

impl FileSystem {
    fn new() -> Self {
        let node = Node::Dir("/".to_string(), vec![]);
        Self {
            root: Arc::new(Mutex::new(node)),
            pwd: vec!["/".to_string()],
        }
    }
    fn mkdir(&mut self, name:String) {
        let mut node = self.root.clone();
        for dir in &self.pwd {
            if node.lock().unwrap().name() == *dir {
                continue
            }
            let qwe = node.clone();
            let guard = qwe.lock().unwrap();
            let asd = guard.get_child_by_name(dir.clone()).unwrap();
            node = asd.clone();
        }
        node.lock().unwrap().mkdir(Node::Dir(name, vec![]));
    }
    fn create(&mut self,file:File) {
        let mut node = self.root.clone();
        for dir in &self.pwd {
            if node.lock().unwrap().name() == *dir {
                continue
            }
            let qwe = node.clone();
            let guard = qwe.lock().unwrap();
            let asd = guard.get_child_by_name(dir.clone()).unwrap();
            node = asd.clone();
        }
        node.lock().unwrap().create(file);
    }
    fn cd(&mut self, dir: String) {
        self.pwd.push(dir);
    }
    fn cddotdot(&mut self) {
        self.pwd.pop();
    }
    fn size(&self) -> u32 {
        self.root.lock().unwrap().size()
    }

    fn dirs_with_sizes(&self) -> Vec<DirWithSize>{
        self.root.lock().unwrap().dirs_with_sizes()
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u32 {
    let mut lines = parse(input);
    lines.remove(0);
    let mut filesystem = FileSystem::new();
    for line in lines {
        match line {
            Line::ChangeDirectory(dir) => {
            filesystem.cd(dir);
            },
            Line::ChangeDirectoryUp => {
               filesystem.cddotdot();
            },
            Line::List => {
                //noop
            }
            Line::Directory(name) => {
                filesystem.mkdir(name);
            }
            Line::File(file) => {
                filesystem.create(file);
            }
        }
    }

    println!("total size: {}", filesystem.size());
    let dirs = filesystem.dirs_with_sizes();
    println!("sizes: {:?}", dirs);

    dirs.into_iter().map(|dir|{
        if dir.size <= 100000 {
            dir.size
        } else {
            0
        }
    }).sum()

}

#[aoc(day7, part2)]
fn part2(input: &str) -> u32 {
    let mut lines = parse(input);
    lines.remove(0);
    let mut filesystem = FileSystem::new();
    for line in lines {
        match line {
            Line::ChangeDirectory(dir) => {
                filesystem.cd(dir);
            },
            Line::ChangeDirectoryUp => {
                filesystem.cddotdot();
            },
            Line::List => {
                //noop
            }
            Line::Directory(name) => {
                filesystem.mkdir(name);
            }
            Line::File(file) => {
                filesystem.create(file);
            }
        }
    }

    let total_size = 70000000;
    let required_space = 30000000;
    let current_usage = filesystem.size() as i32;
    let missing_space = ((total_size - required_space - current_usage) *-1) as u32;
    println!("usage: {}, need to free {}", current_usage,missing_space );

    let mut candidates_for_deletion = filesystem.dirs_with_sizes();
    candidates_for_deletion.sort_unstable_by_key(|d|d.size);
    println!("candidates: {:?}", candidates_for_deletion);
    for candidate in candidates_for_deletion {
        if candidate.size > missing_space {
            return candidate.size
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn verify_part1() {
        let input = include_str!("../input/2022/day7.txt");
        assert_eq!(part1(input), 1454188);
    }

    #[test]
    fn verify_part2() {
        let input = include_str!("../input/2022/day7.txt");
        assert_eq!(part2(input), 4183246);
    }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
        );

        assert_eq!(result, 95437)
    }

    #[test]
    fn part2_provided_example() {
        let result = part2(
            r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
        );

        assert_eq!(result, 24933642)
    }
}
