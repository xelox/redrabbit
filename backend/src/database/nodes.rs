use std::{cell::RefCell, collections::HashMap, rc::Rc};

use diesel::{
    associations::Identifiable, deserialize::{Queryable, QueryableByName}, prelude::Insertable, query_builder::AsChangeset, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Selectable
};
use crate::{database::schema, util::id::Id};
use serde::Serialize;

#[derive(Queryable, Selectable, QueryableByName, AsChangeset, Identifiable)]
#[derive(Clone, Debug)]
#[diesel(table_name = schema::nodes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize)]
pub struct Node {
    id: Id,
    name: String,
    startdue: Option<Id>,
    deadline: Option<Id>,
    notes: String,
    done: bool,
    started: bool,
    parent_id: Option<Id>,
    is_open: bool,
}

#[derive(Insertable)]
#[diesel(table_name = schema::nodes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewNode<'a> {
    name: &'a String,
    startdue: Option<&'a Id>,
    deadline: Option<&'a Id>,
    notes: Option<&'a String>,
    parent_id: Option<&'a Id>,
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

pub enum BulkChange {
    Done(bool),
    Started(bool),
}

impl Node {
    pub fn update_many(targets: Vec<Id>, what: BulkChange) -> QueryResult<usize> {
        use schema::nodes;
        let conn = &mut crate::database::get_conn();

        #[derive(AsChangeset)]
        #[diesel(table_name = schema::nodes)]
        #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
        struct Change {
            started: Option<bool>, 
            done: Option<bool>,
        }

        let change = match what {
            BulkChange::Done(new) => Change {started: None, done: Some(new)},
            BulkChange::Started(new) => Change {started: Some(new), done: None},
        };


        diesel::update(
            nodes::table.filter(
                nodes::id.eq_any(targets)
            ))
            .set(change)
            .execute(conn)
    }

    pub fn update_self(&self) -> QueryResult<usize> {
        use schema::nodes;
        let conn = &mut crate::database::get_conn();
        
        diesel::update(
            nodes::table.filter(
                nodes::id.eq(self.id)
            ))
            .set(self)
            .execute(conn)
    }

    fn build_tree(input: Vec<Node>) -> Vec<TreeNode> {
        #[derive(Clone, Debug)]
        pub struct PreTreeNode {
            id: Id,
            name: String,
            notes: String,
            startdue: Option<Id>,
            deadline: Option<Id>,
            done: bool,
            started: bool,
            is_open: bool,
            parent_id: Option<Id>,
            children: Vec<Rc<RefCell<Self>>>,
            is_child: bool,
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
                    is_child: false,
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

        let map: HashMap<Id, Rc<RefCell<PreTreeNode>>> = HashMap::from_iter(
            input.iter().map(|node|{
                (node.id, Rc::new(RefCell::new(node.clone().into())))
            })
        );

        
        for val in map.values() {
            if let Some(parent_id) = val.borrow().parent_id {
                if let Some(parent) = map.get(&parent_id) {
                    let mut mut_parent_ref = parent.borrow_mut();
                    val.borrow_mut().is_child = true;
                    mut_parent_ref.children.push(val.clone())
                }
            }
        }

        let mut output: Vec<TreeNode> = Vec::new();

        for val in map.values() {
            let node = val.borrow();
            if node.is_child == false {
                let b = <RefCell<PreTreeNode> as Clone>::clone(val).into_inner();
                output.push(b.into())
            }
        }

        output
    }

    pub fn load(from: Option<Id>) -> QueryResult<Vec<TreeNode>> {
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

    pub fn delete(id: Id) -> Result<usize, diesel::result::Error> {
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
    id: Id,
    name: String,
    notes: String,
    startdue: Option<Id>,
    deadline: Option<Id>,
    done: bool,
    started: bool,
    is_open: bool,
    parent_id: Option<Id>,
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
        parent_id: Some(&1.into()),
        notes: None,
        startdue: None,
        deadline: None,
        is_open: Some(&true),
    }.save();

    NewNode {
        name: &"task3".to_string(),
        parent_id: Some(&2.into()),
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
    assert!(Node::delete(1.into()).is_ok());
}
