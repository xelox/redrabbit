use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

use diesel::{
    associations::Identifiable, deserialize::{Queryable, QueryableByName}, prelude::Insertable, query_builder::AsChangeset, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Selectable
};
use crate::database::schema;
use serde::Serialize;

// diesel::table! {
//     nodes (id) {
//         id -> Integer,
//         name -> Text,
//         startdue -> Nullable<Integer>,
//         deadline -> Nullable<Integer>,
//         notes -> Text,
//         done -> Bool,
//         started -> Bool,
//         parent_id -> Nullable<Integer>,
//         is_open -> Bool,
//     }
// }

#[derive(Queryable, Selectable, QueryableByName, AsChangeset, Identifiable)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::nodes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize)]
pub struct Node {
    id: i32,
    name: String,
    startdue: Option<i32>,
    deadline: Option<i32>,
    notes: String,
    done: bool,
    started: bool,
    parent_id: Option<i32>,
    is_open: bool,
}

#[derive(Insertable)]
#[diesel(table_name = schema::nodes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewNode<'a> {
    name: &'a String,
    startdue: Option<&'a i32>,
    deadline: Option<&'a i32>,
    notes: Option<&'a String>,
    parent_id: Option<&'a i32>,
    is_open: Option<&'a bool>,
}

impl<'a> NewNode<'a> {
    pub fn save(&self) -> Option<usize> {
        let conn = &mut crate::database::get_conn();
        let result = diesel::insert_into(schema::nodes::table).values(self).execute(conn);
        match result {
            Ok(r) => Some(r),
            Err(_) => None
        }
    }
}

impl Node {
    pub fn update(&self) -> QueryResult<Vec<TreeNode>> {
        use schema::nodes;
        let conn = &mut crate::database::get_conn();

        if let Err(err) = diesel::update(nodes::table.find(self.id))
            .set(self)
            .execute(conn) {
            return Err(err);
        };

        Self::load(Some(self.id))
    }

    fn build_tree(input: Vec<Node>) -> Vec<TreeNode> {
        #[derive(Clone, Debug)]
        pub struct PreTreeNode {
            id: i32,
            name: String,
            notes: String,
            startdue: Option<i32>,
            deadline: Option<i32>,
            done: bool,
            started: bool,
            is_open: bool,
            parent_id: Option<i32>,
            children: Vec<Rc<RefCell<Self>>>
        }

        impl From<Node> for PreTreeNode {
            fn from(src: Node) -> Self {
                Self {
                    id: src.id,
                    name: src.name,
                    notes: src.notes,
                    startdue: src.startdue,
                    deadline: src.deadline,
                    done: src.done,
                    started: src.started,
                    is_open: src.is_open,
                    parent_id: src.parent_id,
                    children: Vec::new(),
                }
            }
        }

        impl From<PreTreeNode> for TreeNode {
            fn from(src: PreTreeNode) -> TreeNode {
                let mut children = Vec::<TreeNode>::new();
                for w in src.children {
                    let b = <RefCell<PreTreeNode> as Clone>::clone(&w).into_inner();
                    children.push(b.into())
                }
                Self {
                    id: src.id,
                    name: src.name,
                    notes: src.notes,
                    startdue: src.startdue,
                    deadline: src.deadline,
                    done: src.done,
                    started: src.started,
                    is_open: src.is_open,
                    parent_id: src.parent_id,
                    children,
                }
            }
        }

        let map: HashMap<i32, Rc<RefCell<PreTreeNode>>> = HashMap::from_iter(
            input.iter().map(|node|{
                (node.id, Rc::new(RefCell::new(node.clone().into())))
            })
        );

        
        for val in map.values() {
            if let Some(parent_id) = val.borrow().parent_id {
                let parent = map.get(&parent_id).unwrap();
                let mut mut_parent_ref = parent.borrow_mut();
                mut_parent_ref.children.push(val.clone())
            }
        }

        let mut output: Vec<TreeNode> = Vec::new();

        for val in map.values() {
            let node = val.borrow();
            if node.parent_id.is_none() {
                let b = <RefCell<PreTreeNode> as Clone>::clone(val).into_inner();
                output.push(b.into())
            }
        }

        output
    }

    pub fn load(from: Option<i32>) -> QueryResult<Vec<TreeNode>> {
        let conn = &mut crate::database::get_conn();

        let result: Result<Vec<Node>, diesel::result::Error> = match from {
            Some(id) => {
                let query = include_str!("./sql/recur_tfi.sql");
                diesel::sql_query(query)
                    .bind::<diesel::sql_types::Integer, _>(id)
                    .load(conn)
            },
            None => {
                let query = include_str!("./sql/recur_tfr.sql");
                diesel::sql_query(query).load(conn)
            }
        };

        match result {
            Ok(nodes) => Ok(Self::build_tree(nodes)),
            Err(err) => Err(err)
        }
    }

    pub fn delete(id: i32) -> Result<usize, diesel::result::Error> {
        use schema::nodes;
        let conn = &mut crate::database::get_conn();

        diesel::delete(nodes::table)
            .filter(nodes::id.eq(id))
            .execute(conn)
    }
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct TreeNode {
    id: i32,
    name: String,
    notes: String,
    startdue: Option<i32>,
    deadline: Option<i32>,
    done: bool,
    started: bool,
    is_open: bool,
    parent_id: Option<i32>,
    children: Vec<Self>
}

impl From<Node> for TreeNode {
    fn from(src: Node) -> Self {
        Self {
            id: src.id,
            name: src.name,
            notes: src.notes,
            startdue: src.startdue,
            deadline: src.deadline,
            done: src.done,
            started: src.started,
            is_open: src.is_open,
            parent_id: src.parent_id,
            children: Vec::new(),
        }
    }
}

#[test]
fn insert() {
    NewNode {
        name: &"task1".to_string(),
        parent_id: None,
        notes: None,
        startdue: None,
        deadline: None,
        is_open: Some(&true),
    }.save();

    NewNode {
        name: &"task2".to_string(),
        parent_id: Some(&1),
        notes: None,
        startdue: None,
        deadline: None,
        is_open: Some(&true),
    }.save();

    NewNode {
        name: &"task3".to_string(),
        parent_id: Some(&2),
        notes: None,
        startdue: None,
        deadline: None,
        is_open: Some(&false),
    }.save();

    NewNode {
        name: &"task4".to_string(),
        parent_id: None,
        notes: None,
        startdue: None,
        deadline: None,
        is_open: Some(&false),
    }.save();
}

#[test]
fn delete() {
    assert!(Node::delete(1).is_ok());
}
