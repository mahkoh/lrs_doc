// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[allow(unused_imports)] #[prelude_import] use lrs::prelude::*;
use lrs::io::{Write};
use lrs::string::{ByteString};

use html::{Formatter, write_abi, angle_generics, fn_in_out, where_predicates, markup,
           write_raw_type};
use markup::{Document};
use tree::*;

impl Formatter {
    pub fn function(&mut self, item: &ItemData, func: &Func) -> Result {
        let mut file = try!(self.file());

        try!(self.head(&mut file, "Function "));
        try!(self.h1(&mut file, "Function "));

        try!(markup::short(&mut file, &item.docs.parts));

        try!(syntax(&mut file, func, self.path.last().as_ref().unwrap()));
        try!(args(&mut file, &func.decl, &item.docs));
        try!(return_value(&mut file, &func.decl, &item.docs));

        try!(markup::remarks(&mut file, &item.docs.parts));
        try!(markup::examples(&mut file, &item.docs.parts));
        try!(markup::see_also(&mut file, &item.docs.parts));

        try!(self.foot(&mut file));
        Ok(())
    }
}

fn syntax<W: Write>(file: &mut W, func: &Func, name: &ByteString) -> Result {
    try!(file.write_all(b"\
        <h2>Syntax</h2>\
        <pre>\
        "));
    if func.unsaf {
        try!(file.write_all(b"unsafe "));
    }
    if try!(write_abi(file, &func.abi)) {
        try!(file.write_all(b" "));
    }
    try!(file.write_all(b"fn "));
    try!(file.write_all(name.as_ref()));

    let mut have_where_predicates = func.generics.where_predicates.len() > 0;
    have_where_predicates |= try!(angle_generics(file, &func.generics));

    try!(fn_in_out(file, &SelfTy::Static, &func.decl));

    if have_where_predicates {
        try!(file.write_all(b"\n"));
        try!(where_predicates(file, &func.generics, "    "));
    }

    try!(file.write_all(b"\
        </pre>\
        "));

    Ok(())
}

pub fn args<W: Write>(mut file: &mut W, decl: &FnDecl, docs: &Document) -> Result {
    if decl.inputs.len() == 0 {
        return Ok(());
    }

    try!(file.write_all(b"\
        <h2>Arguments</h2>\
        <table>\
            <thead>\
                <tr>\
                    <th>Name</th>\
                    <th>Description</th>\
                </tr>\
            </thead>\
            <tbody>\
                "));

    for arg in &decl.inputs {
        try!(file.write_all(b"\
            <tr>\
                <td>\
                "));
        try!(file.write_all(arg.name.as_ref()));
        try!(file.write_all(b"\
                </td>\
                <td>\
                "));
        try!(markup::arg_desc(file, &docs.parts, &arg.name));
        try!(file.write_all(b"\
                </td>\
            <tr>\
            "));
    }

    try!(file.write_all(b"\
            </tbody>\
        </table>\
        "));

    Ok(())
}

pub fn return_value<W: Write>(mut file: &mut W, decl: &FnDecl,
                              docs: &Document) -> Result {
    if let FuncRetTy::Unit = decl.output {
        return Ok(());
    }

    if !markup::has_return_value(&docs.parts) {
        return Ok(());
    }

    try!(file.write_all(b"\
        <h2>Return value</h2>\
        "));

    match decl.output {
        FuncRetTy::NoReturn | FuncRetTy::Return(Type::Bottom) => {
            try!(file.write_all(b"This function does not return."));
        },
        FuncRetTy::Return(_) => {
            try!(markup::return_value(file, &docs.parts));
        },
        FuncRetTy::Unit => { },
    };

    Ok(())
}
