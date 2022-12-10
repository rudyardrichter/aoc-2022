use std::{cell::RefCell, collections::HashMap, fmt::Formatter, rc::Rc};

#[derive(Debug, PartialEq)]
enum DirToken {
    Root,
    Parent,
    Dir(String),
}

impl From<&str> for DirToken {
    fn from(s: &str) -> Self {
        match s {
            "/" => DirToken::Root,
            ".." => DirToken::Parent,
            _ => DirToken::Dir(s.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct LSOutput {
    dirs: Vec<String>,
    files: HashMap<String, usize>,
}

impl TryFrom<&str> for LSOutput {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut dirs = Vec::new();
        let mut files = HashMap::new();
        for line in s.lines() {
            let (p1, p2) = line
                .split_once(' ')
                .ok_or(format!("parse error on line: {}", line))?;
            if p1 == "dir" {
                dirs.push(p2.to_string());
            } else {
                files.insert(p2.to_string(), p1.parse().map_err(|_| "parse error: size")?);
            }
        }
        Ok(Self { dirs, files })
    }
}

enum Command {
    CD(DirToken),
    LS(LSOutput),
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Some(dir) = s.strip_prefix("cd ") {
            Ok(Command::CD(dir.trim().into()))
        } else {
            Ok(Command::LS(LSOutput::try_from(
                s.split_once("\n")
                    .ok_or(format!("parse error in command: {}", s))?
                    .1,
            )?))
        }
    }
}

type TreeRef = Rc<RefCell<DirTree>>;

struct DirTree {
    children: HashMap<String, TreeRef>,
    parent: Option<TreeRef>,
    files: HashMap<String, usize>,
    size_cached: Option<usize>,
}

impl std::fmt::Debug for DirTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirTree")
            .field("children", &self.children)
            .field("files", &self.files)
            .field("size_cached", &self.size_cached)
            .finish()
    }
}

impl DirTree {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            parent: None,
            files: HashMap::new(),
            size_cached: None,
        }
    }

    fn size(&mut self) -> usize {
        if let Some(size) = self.size_cached {
            size
        } else {
            let size = self.files.values().sum::<usize>()
                + self
                    .children
                    .values()
                    .map(|c| c.borrow_mut().size())
                    .sum::<usize>();
            self.size_cached = Some(size);
            size
        }
    }
}

#[derive(Debug)]
pub struct DirTreeOwner {
    root: TreeRef,
}

impl DirTreeOwner {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(DirTree::new())),
        }
    }

    fn nodes(&self) -> Vec<TreeRef> {
        let mut stack: Vec<TreeRef> = vec![self.root.clone()];
        let mut result: Vec<TreeRef> = Vec::new();
        while let Some(node) = stack.pop() {
            result.push(node.clone());
            let node = node.borrow();
            for child in node.children.values() {
                stack.push(child.clone());
            }
        }
        result
    }

    fn size_of_small_dirs(&self) -> usize {
        self.nodes()
            .iter()
            .filter_map(|n| Some(n.borrow_mut().size()).filter(|&size| size < 100000))
            .sum()
    }

    fn smallest_dir_to_delete(&self, total: usize, need: usize) -> usize {
        let usage: usize = self.root.borrow_mut().size();
        self.nodes().iter().fold(usize::MAX, |acc, node| {
            let size = node.borrow_mut().size();
            if usage + need - size < total {
                acc.min(size)
            } else {
                acc
            }
        })
    }
}

impl TryFrom<&str> for DirTreeOwner {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let owner = DirTreeOwner::new();
        s.split("$ ")
            .skip(1) // empty item since input starts with $
            .map(Command::try_from)
            .collect::<Result<Vec<Command>, String>>()?
            .iter()
            .try_fold(owner.root.clone(), |current, cmd| match cmd {
                Command::CD(dir) => match dir {
                    DirToken::Root => Ok(owner.root.clone()),
                    DirToken::Parent => {
                        let parent = current.borrow().parent.clone();
                        if let Some(p) = parent {
                            Ok(p)
                        } else {
                            Err("cd .. from root")
                        }
                    }
                    DirToken::Dir(dir) => Ok(current
                        .borrow_mut()
                        .children
                        .entry(dir.to_string())
                        .or_insert_with(|| {
                            let mut child = DirTree::new();
                            child.parent = Some(current.clone());
                            Rc::new(RefCell::new(child))
                        })
                        .clone()),
                },
                Command::LS(output) => {
                    current.borrow_mut().files = output.files.clone();
                    Ok(current)
                }
            })?;
        Ok(owner)
    }
}

#[aoc_generator(day7)]
pub fn get_input(input: &str) -> DirTreeOwner {
    input.try_into().unwrap()
}

#[aoc(day7, part1)]
pub fn part_1(tree: &DirTreeOwner) -> usize {
    tree.size_of_small_dirs()
}

#[aoc(day7, part2)]
pub fn part_2(tree: &DirTreeOwner) -> usize {
    tree.smallest_dir_to_delete(70000000, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../test_data/day_07.txt");

    #[test]
    fn test_parsing() {
        assert_eq!(DirToken::from("/"), DirToken::Root);
        assert_eq!(DirToken::from(".."), DirToken::Parent);
        assert_eq!(
            DirToken::from("potato"),
            DirToken::Dir("potato".to_string())
        );
        let ls_in = "dir a\n14848514 b.txt\n8504156 c.dat\ndir d";
        let ls_out = LSOutput {
            dirs: vec!["a".to_string(), "d".to_string()],
            files: [
                ("b.txt".to_string(), 14848514),
                ("c.dat".to_string(), 8504156),
            ]
            .iter()
            .cloned()
            .collect(),
        };
        assert_eq!(LSOutput::try_from(ls_in).unwrap(), ls_out);
        assert!(DirTreeOwner::try_from("cd /")
            .unwrap()
            .root
            .borrow()
            .parent
            .is_none());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input(INPUT)), 95437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input(INPUT)), 24933642);
    }
}
