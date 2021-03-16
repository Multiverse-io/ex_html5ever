use std::borrow::Cow;
use std::collections::HashMap;
use std::default::Default;
use std::io::Read;

use html5ever::tendril::*;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{expanded_name, local_name, namespace_url, ns, parse_document};
use html5ever::{Attribute, ExpandedName, ParseOpts, QualName};

pub fn lint(input: String) -> Vec<(u64, String)> {
    return lint_from_read(&mut input.as_bytes());
}

fn lint_from_read<R: Read>(r: &mut R) -> Vec<(u64, String)> {
    let mut parse_errors = Vec::new();
    let sink = Sink {
        next_id: 1,
        names: HashMap::new(),
        parse_errors: &mut parse_errors,
        line_number: 1,
    };

    let parse_opts: ParseOpts = Default::default();

    // You can get extra detail in the error messages like this, but the extra
    // info doesn't seem particularly useful.
    //
    // let mut parse_opts: ParseOpts = Default::default();
    // parse_opts.tokenizer.exact_errors = true;
    // parse_opts.tree_builder.exact_errors = true;

    parse_document(sink, parse_opts)
        .from_utf8()
        .read_from(r)
        .unwrap();

    return parse_errors;
}

struct Sink<'a> {
    next_id: usize,
    names: HashMap<usize, QualName>,
    parse_errors: &'a mut Vec<(u64, String)>,
    line_number: u64,
}

impl<'a> Sink<'a> {
    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 2;
        id
    }
}

impl<'a> TreeSink for Sink<'a> {
    type Handle = usize;
    type Output = Self;
    fn finish(self) -> Self {
        self
    }

    fn get_document(&mut self) -> usize {
        0
    }

    fn get_template_contents(&mut self, target: &usize) -> usize {
        if let Some(expanded_name!(html "template")) = self.names.get(&target).map(|n| n.expanded())
        {
            target + 1
        } else {
            panic!("not a template element")
        }
    }

    fn same_node(&self, x: &usize, y: &usize) -> bool {
        x == y
    }

    fn elem_name(&self, target: &usize) -> ExpandedName {
        self.names.get(target).expect("not an element").expanded()
    }

    fn create_element(&mut self, name: QualName, _: Vec<Attribute>, _: ElementFlags) -> usize {
        let id = self.get_id();
        self.names.insert(id, name);
        id
    }

    fn create_comment(&mut self, _text: StrTendril) -> usize {
        self.get_id()
    }

    #[allow(unused_variables)]
    fn create_pi(&mut self, target: StrTendril, value: StrTendril) -> usize {
        unimplemented!()
    }

    fn append_before_sibling(&mut self, _sibling: &usize, _new_node: NodeOrText<usize>) {}

    fn append_based_on_parent_node(
        &mut self,
        _element: &usize,
        _prev_element: &usize,
        _new_node: NodeOrText<usize>,
    ) {
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        self.parse_errors.push((self.line_number, msg.to_string()))
    }
    fn set_quirks_mode(&mut self, _mode: QuirksMode) {}
    fn append(&mut self, _parent: &usize, _child: NodeOrText<usize>) {}

    fn append_doctype_to_document(&mut self, _: StrTendril, _: StrTendril, _: StrTendril) {}
    fn add_attrs_if_missing(&mut self, target: &usize, _attrs: Vec<Attribute>) {
        assert!(self.names.contains_key(&target), "not an element");
    }
    fn remove_from_parent(&mut self, _target: &usize) {}
    fn reparent_children(&mut self, _node: &usize, _new_parent: &usize) {}
    fn mark_script_already_started(&mut self, _node: &usize) {}

    fn set_current_line(&mut self, line_number: u64) {
        self.line_number = line_number;
    }
}
