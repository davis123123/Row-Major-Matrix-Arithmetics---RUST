use std::io;
use std::string::String;
use std::vec::Vec;
use std::option::Option;
use std::cmp::Ordering;
use std::boxed::Box;

pub struct TreeNode<T> {
    key: T,
    l: Option<Box<TreeNode<T>>>,
    r: Option<Box<TreeNode<T>>>,
}

impl<T:Ord> TreeNode<T>{
    pub fn new(key: T) -> Self{
        TreeNode{
            key: key,
            l: None,
            r: None,
        }
    }

    pub fn insert2(tree: &mut Option<Box<TreeNode<T>>>, key: T) -> bool{
        //unimplemented!()
        match *tree{
            None => {
                let new_key = TreeNode::new(key);
                *tree = Some(Box::new(new_key));
                return true;
            },
            Some (ref mut p) =>{
                match key.cmp(&mut p.key){
                    Ordering::Greater => {
                        match p.r{
                            None => {
                                let new_key = TreeNode::new(key);
                                p.r = Some(Box::new(new_key));
                                return true;
                            },
                            Some(ref mut p) => TreeNode::insert2(&mut p.r, key),
                       }
                    }
                    Ordering::Less => {
                        match p.l{
                            None => {
                                let new_key = TreeNode::new(key);
                                p.l = Some(Box::new(new_key));
                                return true;
                            },
                            Some(ref mut p) => TreeNode::insert2(&mut p.l, key),
                        }
                    }
                    Ordering::Equal => return false,
                }
            }
        }

    }

    pub fn find2(tree: &Option<Box<TreeNode<T>>>, key: &T) -> bool{
        match *tree{
            None => return false,
            Some(ref p) => {
                match key.cmp(&p.key){
                    Ordering::Equal => return true,
                    Ordering::Less => {
                        match p.l{
                            Some(ref p) => TreeNode::find2(&p.l, &key),
                            None => return false,
                        }
                    },
                    Ordering::Greater => {
                        match p.r{
                        Some (ref p) =>TreeNode::find2(&p.r, &key),
                        None => return false,
                        }
                    },
                }
            }
        }
    }
}

pub struct Tree<T>{
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree{root:None}

    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        let mut x:bool = false;
        match self.root{
            Some(ref mut p) => {

                match key.cmp(&mut p.key){
                    Ordering::Greater => {
                        match p.r{
                            None => {
                                let new_key = TreeNode::new(key);
                                p.r = Some(Box::new(new_key));
                                x = true;
                            },
                            Some(ref mut p) => x = TreeNode::insert2(&mut p.r, key),
                       }
                    }
                    Ordering::Less => {
                        match p.l{
                            None => {
                                let new_key = TreeNode::new(key);
                                p.l = Some(Box::new(new_key));
                                x = true;
                            },
                            Some(ref mut p) => x = TreeNode::insert2(&mut p.l, key),
                        }
                    }
                    Ordering::Equal => x = false,
                }
            },
            None => {
                let new_key = TreeNode::new(key);
                self.root = Some(Box::new(new_key));
                x = true;
            },
        }
        return x;
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        let mut x:bool = false;
        match self.root{
            None => x = false,
            Some(ref p) => {
                match key.cmp(&p.key){
                    Ordering::Equal => x = true,
                    Ordering::Less => {
                        match p.l{
                            Some(ref p) => x = TreeNode::find2(&p.l, &key),
                            None => x = false,
                        }
                    },
                    Ordering::Greater => {
                        match p.r{
                            Some(ref p) => x = TreeNode::find2(&p.r, &key),
                            None => x = false,
                        }
                    },
                }
            }
        }
        return x;
    }

    pub fn preorder2(tree: &mut Option<Box<TreeNode<T>>>) -> Vec<&T>{
        unimplemented!()
    }
    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        unimplemented!()
        /*if self.root.unwrap().key == None{return}//if no more nodes// help here
        find(self.root.unwrap().key, key);// call find?
        let mut pre_o: Vector:<&T> = preorder(self.root.unwrap().left);*/
        let mut stack: Vec<&T> = Vec::new();

        match self.root{
            None => return stack,
            Some(ref p) => {
                stack.push(&p.key);
                stack.push(preorder2(p.l, &mut stack));
                stack.push(preorder2(p.r, &mut stack));
            },
        }
        return stack;
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        unimplemented!()
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        //if self.node.is_empty()
        unimplemented!()
    }
}
