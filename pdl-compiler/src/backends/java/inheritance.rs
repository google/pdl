use std::collections::{HashMap, HashSet};

use crate::backends::java::Field;

#[derive(Debug, Clone)]
pub enum Constraint {
    Integral(usize),
    EnumTag(String),
}

#[derive(Debug, Clone)]
pub struct InheritanceNode {
    pub name: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub constraints: HashMap<String, Constraint>,
    pub static_field_width: usize,
    pub dyn_fields: HashSet<String>,
}

impl InheritanceNode {
    pub fn field_width(&self) -> Option<usize> {
        if self.dyn_fields.is_empty() {
            Some(self.static_field_width)
        } else {
            None
        }
    }
}

/// Every packet goes in the heirarchy, even ones with no inheritence
/// That way we can look up sizes of any packet
#[derive(Debug, Clone)]
pub struct ClassHeirarchy(HashMap<String, InheritanceNode>);

impl ClassHeirarchy {
    pub fn new() -> Self {
        ClassHeirarchy(HashMap::new())
    }

    pub fn default_child_name(parent_name: &str) -> String {
        format!("Unknown{}", parent_name)
    }

    pub fn add_class(&mut self, name: String, fields: &Vec<Field>) {
        let (static_field_width, non_static_members) = self.width_of_fields(fields);
        self.0.insert(
            name.clone(),
            InheritanceNode {
                name,
                parent: None,
                children: Vec::new(),
                constraints: HashMap::new(),
                static_field_width,
                dyn_fields: non_static_members,
            },
        );
    }

    pub fn add_child(
        &mut self,
        parent_name: String,
        child_name: String,
        constraints: HashMap<String, Constraint>,
        fields: &Vec<Field>,
    ) {
        self.0
            .get_mut(&parent_name)
            .expect(&format!("parent {} does not exist", parent_name))
            .children
            .push(child_name.clone());

        let (static_field_width, non_static_members) = self.width_of_fields(fields);
        self.0.insert(
            child_name.clone(),
            InheritanceNode {
                name: child_name,
                parent: Some(parent_name),
                children: Vec::new(),
                constraints,
                static_field_width,
                dyn_fields: non_static_members,
            },
        );
    }

    /// Get a class by name. Panics if the class does not exist in the heirarchy.
    pub fn get(&self, name: &str) -> &InheritanceNode {
        self.0.get(name).unwrap()
    }

    /// Get the classes parent if it has one.
    pub fn parent(&self, name: &str) -> Option<&InheritanceNode> {
        self.0.get(name).unwrap().parent.as_ref().map(|parent| self.0.get(parent).unwrap())
    }

    /// Get the class's children.
    pub fn children(&self, name: &str) -> Vec<&InheritanceNode> {
        self.0.get(name).unwrap().children.iter().map(|child| self.0.get(child).unwrap()).collect()
    }

    /// Get the class's field width if it can be statically determined.
    /// The field width of a class does not include any inherited fields.
    pub fn field_width(&self, name: &str) -> Option<usize> {
        self.get(name).field_width()
    }

    /// Same as `self.field_width(name)` but excludes the provided dynamic field.
    /// Provided field must by dynamic.
    pub fn field_width_without_dyn_field(&self, name: &str, exclude: &str) -> Option<usize> {
        let root = self.get(name);
        if !root.dyn_fields.contains(exclude) {
            panic!("{} is not a dyn field of {}", exclude, name);
        } else if root.dyn_fields.len() == 1 {
            Some(root.static_field_width)
        } else {
            None
        }
    }

    // Get the width of the class's static fields.
    pub fn static_field_width(&self, name: &str) -> usize {
        self.get(name).static_field_width
    }

    /// Get the class's width if it can be statically determined.
    pub fn width(&self, name: &str) -> Option<usize> {
        if !self.0.contains_key(name) {
            None
        } else if let Some(parent) = self.parent(name) {
            self.width_recurse(&parent.name)
                .zip(self.field_width(name))
                .map(|(inherited_width, field_width)| inherited_width + field_width)
        } else {
            self.field_width(name)
        }
    }

    fn width_recurse(&self, name: &str) -> Option<usize> {
        if let Some(parent) = self.parent(name) {
            self.width_recurse(&parent.name)
                .zip(self.field_width_without_dyn_field(name, "payload"))
                .map(|(inherited_width, field_width)| inherited_width + field_width)
        } else {
            self.field_width_without_dyn_field(name, "payload")
        }
    }

    fn width_of_fields(&self, fields: &Vec<Field>) -> (usize, HashSet<String>) {
        let mut static_width = 0;
        let mut non_static_fields = HashSet::new();

        for field in fields {
            match field {
                Field::Integral { width, .. }
                | Field::EnumRef { width, .. }
                | Field::Reserved { width } => static_width += width,
                Field::StructRef { name, ty } => {
                    if let Some(width) = self.width(ty) {
                        static_width += width
                    } else {
                        non_static_fields.insert(String::from(name));
                    }
                }
                Field::Payload { .. } => {
                    non_static_fields.insert(String::from("payload"));
                }
                Field::ArrayElem { val, count } => {
                    if let Some(count) = count {
                        static_width += count
                            * val.width().unwrap_or_else(|| {
                                self.width(val.class().unwrap())
                                    .expect("can't have array of non-static elements")
                            })
                    } else {
                        non_static_fields.insert(String::from(val.name()));
                    }
                }
            }
        }

        (static_width, non_static_fields)
    }
}
