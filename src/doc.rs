extern crate sxd_document;

use sxd_document::dom;
use std::fmt;
use std::marker;

pub struct El<'a> {
    elem: dom::Element<'a>,
}

impl<'a> El<'a> {
    pub fn attr<A: fmt::Display + marker::Sized>(self, k: &str, v: A) -> El<'a> {
        self.elem.set_attribute_value(k, &format!("{}", v));
        self
    }
    pub fn append_to<'b>(self, other: &El<'b>) -> El<'a> {
        other.elem.append_child(self.elem);
        self
    }
}

pub fn el<'a>(doc: dom::Document<'a>, name: &str) -> El<'a> {
    El { elem: doc.create_element(name) }
}

pub fn append_child<'a, 'b>(doc: dom::Document<'a>, child: &El<'b>) {
    doc.root()
        .append_child(child.elem);
}
