/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/. */

/*! implements [`Simple`] items */
/** a simple item, like a wiki page */
use crate::item::{Item, ItemBase, ItemBaseForSerde, ItemData, NewBaseTemplate, ShowBaseTemplate};
use crate::markdown;
use crate::shared::NullResult;
use crate::shared::{FLResult, FanlingError};
use crate::world::ActionResponse;
use crate::world::World;
use ansi_term::Colour;
use askama::Template;
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::collections::HashMap;
//#[macro_use]
use crate::fanling_error;
use taipo_git_control::ChangeList;
use taipo_git_control::Conflict;

/** data for a simple item */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simple {
    /** the name of the page */
    name: String,
    /** the text of the page in MarkDown format */
    text: String,
}
impl Simple {
    /** create a new [Simple]  */
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            text: "".to_owned(),
        }
    }
}
impl crate::item::ItemData for Simple {
    fn for_edit(
        &mut self,
        base: &mut ItemBase,
        is_for_update: bool,
        world: &mut World,
    ) -> fanling_interface::ResponseResult {
        let broken_text = self.text.replace("\n", "&#10;");
        trace(&format!("{} converted to {}", self.text, broken_text));
        let nt = NewSimpleTemplate {
            data: &self,
            base: NewBaseTemplate::from_base(base, is_for_update, world)?,
            broken_text,
        };
        let mut resp = fanling_interface::Response::new();
        resp.clear_errors(vec!["name-error".to_owned()]); // TODO: when?
        resp.add_tag("content", &(nt.render()?));
        trace(&format!("for edit {:?}", &resp));
        Ok(resp)
    }
    fn for_show(
        &mut self,
        base: &mut ItemBase,
        world: &mut World,
    ) -> fanling_interface::ResponseResult {
        let t = ShowSimpleTemplate {
            name: self.name.clone(),
            rendered_text: markdown::render(&self.text),
            base: ShowBaseTemplate::from_base(base, world)?,
        };
        let mut resp = fanling_interface::Response::new();
        resp.add_tag("content", &(t.render()?));
        trace(&format!("for show {:?}", &resp));
        Ok(resp)
    }
    fn to_yaml(&self, base: &crate::item::ItemBase) -> Result<Vec<u8>, FanlingError> {
        let for_serde = SimpleForSerde {
            base: crate::item::ItemBaseForSerde::from_base(base)?,
            data: self.clone(),
        };
        let yaml = serde_yaml::to_vec(&for_serde)?;
        trace(&format!("yaml is {}", String::from_utf8_lossy(&yaml)));
        Ok(yaml)
    }
    fn is_open(&self) -> bool {
        true
    }
    fn is_ready(&self) -> bool {
        true
    }
    /** an English-language description */
    fn description(&self) -> String {
        self.name.clone()
    }
    // /** a description that can be used in a list */
    // fn description_for_list(&self) -> String {
    //     self.name.clone()
    // }
    fn set_data(&mut self, vals: &HashMap<String, String>, _world: &mut World) -> NullResult {
        match vals.get("name") {
            Some(s) => self.name = s.to_string(),
            _ => return Err(fanling_error!("no name")),
        }
        match vals.get("text") {
            Some(s) => self.text = s.to_string(),
            _ => self.text = "".to_owned(),
        }
        Ok(())
    }
    fn try_update(
        &mut self,
        _base: &ItemBaseForSerde,
        vals: &HashMap<String, String>,
        _world: &mut World,
    ) -> ActionResponse {
        let mut ar = ActionResponse::new();
        ar.assert(
            !vals["name"].is_empty(),
            "name-error",
            "Name must be non-blank.",
        );
        ar
    }
    fn set_from_yaml(&mut self, yaml: serde_yaml::Value, _world: &mut World) -> NullResult {
        *self = serde_yaml::from_value(yaml)?;
        Ok(())
    }
    /** do action for simple -- should never get called */
    fn do_action(
        &mut self,
        _base: &mut ItemBase,
        _action: crate::Action,
        //    _json_value: serde_json::value::Value,
        _world: &mut World,
    ) -> fanling_interface::ResponseResult {
        Err(fanling_error!(
            "simple do action called, should never happen"
        ))?;
        Ok(fanling_interface::Response::new())
    }
    /** copy from another item data */
    fn fanling_clone(&self) -> FLResult<Box<dyn ItemData>> {
        Ok(Box::new(Self {
            name: self.name.clone(),
            text: self.text.clone(),
        }))
    }
}
#[derive(Serialize, Deserialize)]
struct SimpleForSerde {
    #[serde(flatten)]
    base: crate::item::ItemBaseForSerde,
    #[serde(flatten)]
    data: Simple,
}
/** template data for creating a new simple item */
#[derive(Template)]
#[template(path = "new-simple.html")]
struct NewSimpleTemplate<'a> {
    data: &'a Simple,
    base: NewBaseTemplate,
    broken_text: String,
}

/** template data for showing a simple item */
#[derive(Template)]
#[template(path = "show-simple.html")]
struct ShowSimpleTemplate {
    name: String,
    rendered_text: String,
    base: ShowBaseTemplate,
}

/** policy for the simple item type*/
#[derive(Debug)]
pub struct SimpleTypePolicy {}
impl SimpleTypePolicy {
    pub fn new() -> Self {
        Self {}
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}
impl crate::item::ItemTypePolicy for SimpleTypePolicy {
    fn kind(&self) -> crate::item::ItemKind {
        crate::item::ItemKind::Simple
    }
    fn make_raw(&self, item_type: crate::item::ItemTypeRef) -> Item {
        let item = Item::new_with_data(item_type, Box::new(Simple::new()));
        item
    }
    fn resolve_conflict(&self, conflict: &Conflict, _changes: &mut ChangeList) -> NullResult {
        trace(&format!("conflict detected {:#?}", &conflict));
        unimplemented!() /* TODO resolve_conflict */
    }
}

/** convenience function for debug traces */
fn trace(m: &str) {
    println!(
        "simple {}",
        Colour::Fixed(11).on(Colour::Fixed(233)).paint(m)
    );
}