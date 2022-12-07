use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum Line {
    ChangeDirectory(String),
    ChangeDirectoryUp,
    List,
    Directory(String),
    File(File),
}

fn parse(input: &str) -> FileSystem {
    let mut lines = input
        .trim()
        .split('\n')
        .map(|line| {
            let mut line = line.chars().collect::<Vec<_>>();
            if line[0] == '$' && line[2] == 'c' && line[5] == '.' {
                Line::ChangeDirectoryUp
            } else if line[0] == '$' && line[2] == 'c' {
                line.remove(0);
                line.remove(0);
                line.remove(0);
                line.remove(0);
                line.remove(0);
                Line::ChangeDirectory(line.into_iter().collect())
            } else if line[0] == '$' && line[2] == 'l' {
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
                let size = split.next().unwrap().parse::<i32>().unwrap();
                let name = split.next().unwrap();

                Line::File(File {
                    name: name.to_string(),
                    size,
                })
            }
        })
        .collect::<Vec<_>>();
    lines.remove(0);
    let mut filesystem = FileSystem::new();
    for line in lines {
        match line {
            Line::ChangeDirectory(dir) => {
                filesystem.cd(dir);
            }
            Line::ChangeDirectoryUp => {
                filesystem.cddotdot();
            }
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
    filesystem
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug, Clone)]
enum Node {
    Dir(String, Vec<Arc<Mutex<Node>>>),
    File(String, i32),
}
impl Node {
    fn name(&self) -> String {
        match self {
            Self::File(n, _) | Self::Dir(n, _) => n.clone(),
        }
    }
    fn get_child_by_name(&self, name: &str) -> Option<Arc<Mutex<Self>>> {
        match self {
            Self::Dir(_, children) => children
                .iter()
                .find(|n| n.lock().unwrap().name() == name)
                .cloned(),
            Self::File(_, _) => None,
        }
    }
    fn mkdir(&mut self, dir: Self) {
        match self {
            Self::Dir(_, dirs) => dirs.push(Arc::new(Mutex::new(dir))),
            Self::File(_, _) => {
                panic!("cant happen")
            }
        }
    }
    fn create(&mut self, file: File) {
        match self {
            Self::Dir(_, dirs) => dirs.push(Arc::new(Mutex::new(Self::File(file.name, file.size)))),
            Self::File(_, _) => {
                panic!("cant happen")
            }
        }
    }
    fn size(&self) -> i32 {
        match self {
            Self::Dir(_, files) => files.iter().map(|file| file.lock().unwrap().size()).sum(),
            Self::File(_, size) => *size,
        }
    }
    fn dirs_with_sizes(&self) -> Vec<i32> {
        if let Self::Dir(_, children) = self {
            let mut result = vec![];
            result.push(self.size());
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
    fn mkdir(&mut self, name: String) {
        let mut node = self.root.clone();
        for dir in &self.pwd {
            if node.lock().unwrap().name() == *dir {
                continue;
            }
            let qwe = node.clone();
            let guard = qwe.lock().unwrap();
            let asd = guard.get_child_by_name(dir.as_str()).unwrap();
            node = asd.clone();
        }
        node.lock().unwrap().mkdir(Node::Dir(name, vec![]));
    }
    fn create(&mut self, file: File) {
        let mut node = self.root.clone();
        for dir in &self.pwd {
            if node.lock().unwrap().name() == *dir {
                continue;
            }
            let qwe = node.clone();
            let guard = qwe.lock().unwrap();
            let asd = guard.get_child_by_name(dir.as_str()).unwrap();
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
    fn size(&self) -> i32 {
        self.root.lock().unwrap().size()
    }

    fn dirs_with_sizes(&self) -> Vec<i32> {
        self.root.lock().unwrap().dirs_with_sizes()
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> i32 {
    let filesystem = parse(input);

    println!("total size: {}", filesystem.size());
    let dirs = filesystem.dirs_with_sizes();
    println!("sizes: {:?}", dirs);

    dirs.into_iter()
        .map(|size| if size <= 100_000 { size } else { 0 })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i32 {
    let filesystem = parse(input);

    let total_size = 70_000_000;
    let required_space = 30_000_000;
    let current_usage = filesystem.size();
    let missing_space = 0 - (total_size - required_space - current_usage);

    println!("usage: {}, need to free {}", current_usage, missing_space);

    let mut candidates_for_deletion = filesystem.dirs_with_sizes();
    candidates_for_deletion.sort_unstable_by_key(|size| *size);
    println!("candidates: {:?}", candidates_for_deletion);
    for candidate in candidates_for_deletion {
        if candidate > missing_space {
            return candidate;
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
