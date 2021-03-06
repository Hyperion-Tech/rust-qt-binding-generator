/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::{Arc, Mutex};
use std::ptr::null;

use implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .into_iter()
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1,
}

#[repr(C)]
pub struct QModelIndex {
    row: c_int,
    internal_id: usize,
}


fn to_usize(n: c_int) -> usize {
    if n < 0 {
        panic!("Cannot cast {} to usize", n);
    }
    n as usize
}


fn to_c_int(n: usize) -> c_int {
    if n > c_int::max_value() as usize {
        panic!("Cannot cast {} to c_int", n);
    }
    n as c_int
}


pub struct PersonsQObject {}

#[derive(Clone)]
pub struct PersonsEmitter {
    qobject: Arc<Mutex<*const PersonsQObject>>,
    new_data_ready: fn(*const PersonsQObject, index: usize, valid: bool),
}

unsafe impl Send for PersonsEmitter {}

impl PersonsEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn new_data_ready(&self, item: Option<usize>) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.new_data_ready)(ptr, item.unwrap_or(13), item.is_some());
        }
    }
}

pub struct PersonsTree {
    qobject: *const PersonsQObject,
    data_changed: fn(*const PersonsQObject, usize, usize),
    begin_reset_model: fn(*const PersonsQObject),
    end_reset_model: fn(*const PersonsQObject),
    begin_insert_rows: fn(*const PersonsQObject, index: usize, valid: bool, usize, usize),
    end_insert_rows: fn(*const PersonsQObject),
    begin_remove_rows: fn(*const PersonsQObject, index: usize, valid: bool, usize, usize),
    end_remove_rows: fn(*const PersonsQObject),
}

impl PersonsTree {
    pub fn data_changed(&self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&self, index: Option<usize>, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, index.unwrap_or(13), index.is_some(), first, last);
    }
    pub fn end_insert_rows(&self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&self, index: Option<usize>, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, index.unwrap_or(13), index.is_some(), first, last);
    }
    pub fn end_remove_rows(&self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait PersonsTrait {
    fn new(emit: PersonsEmitter, model: PersonsTree) -> Self;
    fn emit(&self) -> &PersonsEmitter;
    fn row_count(&self, Option<usize>) -> usize;
    fn can_fetch_more(&self, Option<usize>) -> bool {
        false
    }
    fn fetch_more(&mut self, Option<usize>) {}
    fn sort(&mut self, u8, SortOrder) {}
    fn index(&self, item: Option<usize>, row: usize) -> usize;
    fn parent(&self, index: usize) -> Option<usize>;
    fn row(&self, index: usize) -> usize;
    fn user_name(&self, index: usize) -> &str;
    fn set_user_name(&mut self, index: usize, String) -> bool;
}

#[no_mangle]
pub extern "C" fn persons_new(
    persons: *mut PersonsQObject,
    persons_new_data_ready: fn(*const PersonsQObject, index: usize, valid: bool),
    persons_data_changed: fn(*const PersonsQObject, usize, usize),
    persons_begin_reset_model: fn(*const PersonsQObject),
    persons_end_reset_model: fn(*const PersonsQObject),
    persons_begin_insert_rows: fn(*const PersonsQObject, index: usize, valid: bool, usize, usize),
    persons_end_insert_rows: fn(*const PersonsQObject),
    persons_begin_remove_rows: fn(*const PersonsQObject, index: usize, valid: bool, usize, usize),
    persons_end_remove_rows: fn(*const PersonsQObject),
) -> *mut Persons {
    let persons_emit = PersonsEmitter {
        qobject: Arc::new(Mutex::new(persons)),
        new_data_ready: persons_new_data_ready,
    };
    let model = PersonsTree {
        qobject: persons,
        data_changed: persons_data_changed,
        begin_reset_model: persons_begin_reset_model,
        end_reset_model: persons_end_reset_model,
        begin_insert_rows: persons_begin_insert_rows,
        end_insert_rows: persons_end_insert_rows,
        begin_remove_rows: persons_begin_remove_rows,
        end_remove_rows: persons_end_remove_rows,
    };
    let d_persons = Persons::new(persons_emit, model);
    Box::into_raw(Box::new(d_persons))
}

#[no_mangle]
pub unsafe extern "C" fn persons_free(ptr: *mut Persons) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn persons_row_count(
    ptr: *const Persons,
    index: usize,
    valid: bool,
) -> c_int {
    to_c_int(if valid {
        (&*ptr).row_count(Some(index))
    } else {
        (&*ptr).row_count(None)
    })
}
#[no_mangle]
pub unsafe extern "C" fn persons_can_fetch_more(
    ptr: *const Persons,
    index: usize,
    valid: bool,
) -> bool {
    if valid {
        (&*ptr).can_fetch_more(Some(index))
    } else {
        (&*ptr).can_fetch_more(None)
    }
}
#[no_mangle]
pub unsafe extern "C" fn persons_fetch_more(ptr: *mut Persons, index: usize, valid: bool) {
    if valid {
        (&mut *ptr).fetch_more(Some(index))
    } else {
        (&mut *ptr).fetch_more(None)
    }
}
#[no_mangle]
pub unsafe extern "C" fn persons_sort(
    ptr: *mut Persons,
    column: u8,
    order: SortOrder
) {
    (&mut *ptr).sort(column, order)
}
#[no_mangle]
pub unsafe extern "C" fn persons_index(
    ptr: *const Persons,
    index: usize,
    valid: bool,
    row: c_int,
) -> usize {
    if !valid {
        (&*ptr).index(None, to_usize(row))
    } else {
        (&*ptr).index(Some(index), to_usize(row))
    }
}
#[no_mangle]
pub unsafe extern "C" fn persons_parent(ptr: *const Persons, index: usize) -> QModelIndex {
    if let Some(parent) = (&*ptr).parent(index) {
        QModelIndex {
            row: to_c_int((&*ptr).row(parent)),
            internal_id: parent,
        }
    } else {
        QModelIndex {
            row: -1,
            internal_id: 0,
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn persons_row(ptr: *const Persons, index: usize) -> c_int {
    to_c_int((&*ptr).row(index))
}

#[no_mangle]
pub extern "C" fn persons_data_user_name(
    ptr: *const Persons, index: usize,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = unsafe { &*ptr };
    let data = o.user_name(index);
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub extern "C" fn persons_set_data_user_name(
    ptr: *mut Persons, index: usize,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = unsafe { &mut *ptr };
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_user_name(index, v)
}
