use trees::{tr,Tree, TreeWalk, Node};
use clap::Parser;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;

fn calculate_hash_value<T:Hash>(x:T) -> u64{
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}
pub struct TaskDirection{
    id: u64,
    description: String,
}
impl TaskDirection {
    pub fn give_identifier(now_time:Instant, unidentified: TaskDirectionWithoutID) -> TaskDirection{
        TaskDirection{
            id: calculate_hash_value(now_time),
            description: unidentified.description
        }
    }
}


struct TaskDirectionWithoutID{
    description:String,
}
struct TaskTree{
    tree:Tree<TaskDirection>,
}
impl TaskTree {
    fn insert_sub_objective(&self, parent:Node<TaskDirection>, inserted:TaskDirectionWithoutID){
        let insertedTime = std::time::Instant::now();
        let direction = TaskDirection::give_identifier(insertedTime, inserted);
        let insertedTree = Tree::new(direction);
        parent.push_back(insertedTree);
    }
    fn delete_objective(&self, deleted:Node<TaskDirection>){
        deleted.detach()
    }
}

#[derive(Parser)]
struct Cli{
    operation: String,
    #[arg()]
    path: Option<std::path::PathBuf>,
}

fn insert_tasks(){
    
}

fn main() {
    let data = TaskDirection{id:"0".to_string(), description:String::from("abc")};
    let mut tree = tr(data);


}
