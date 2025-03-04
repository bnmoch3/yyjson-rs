use crate::allocator::YyjsonAllocProvider;
use crate::write::{WriteError, WriteOutput, Writer};
use crate::BasicAllocProvider;
use yyjson_sys as ffi;

pub struct List<'doc> {
    p: *mut ffi::yyjson_val,
    // this lifetime is here to ensure that Doc remains 'live' as long as
    // the allocator underlying memory is 'live'
    _doc_lifetime: std::marker::PhantomData<&'doc ()>,
}

impl std::fmt::Debug for List<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.to_val();
        std::fmt::Debug::fmt(&v, f)
    }
}

impl std::fmt::Display for List<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.to_val();
        std::fmt::Display::fmt(&v, f)
    }
}

impl<'doc> List<'doc> {
    // WARNING: only contruct List after checking the val type, from here on
    // we aren't checking if it's type list or not. TODO, add assert just in
    // case
    #[inline(always)]
    fn from_raw(
        p: *mut ffi::yyjson_val,
        doc_lifetime: std::marker::PhantomData<&'doc ()>,
    ) -> List<'doc> {
        List {
            p,
            _doc_lifetime: doc_lifetime,
        }
    }

    fn to_val(&self) -> Val<'doc> {
        Val {
            p: self.p,
            _doc_lifetime: self._doc_lifetime,
        }
    }

    pub fn len(&self) -> usize {
        let v = unsafe { &*self.p };
        let len: usize = (v.tag >> NUM_TAG_BITS) as usize; // tag bit should b 8 bits
        len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<Val<'doc>> {
        // SAFETY: yyjson doc on fn call
        let p = unsafe { ffi::yyjson_arr_get(self.p, index) };
        if p.is_null() {
            None
        } else {
            Some(Val {
                // SAFETY: we already check if is null above, no need to check
                // again
                p,
                _doc_lifetime: std::marker::PhantomData,
            })
        }
    }

    pub fn first(&self) -> Option<Val<'doc>> {
        // SAFETY: yyjson doc on fn call
        let p = unsafe { ffi::yyjson_arr_get_first(self.p) };
        if p.is_null() {
            None
        } else {
            Some(Val {
                // SAFETY: we already check if is null above, no need to check
                // again
                p,
                _doc_lifetime: std::marker::PhantomData,
            })
        }
    }

    pub fn last(&self) -> Option<Val<'doc>> {
        // SAFETY: yyjson doc on fn call
        let p = unsafe { ffi::yyjson_arr_get_last(self.p) };
        if p.is_null() {
            None
        } else {
            Some(Val {
                // SAFETY: we already check if is null above, no need to check
                // again
                p,
                _doc_lifetime: std::marker::PhantomData,
            })
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> ListIterator<'doc> {
        let iter = unsafe { ffi::yyjson_arr_iter_with(self.p) };
        ListIterator {
            iter,
            _doc_lifetime: self._doc_lifetime,
        }
    }
}

#[repr(transparent)]
pub struct ListIterator<'doc> {
    iter: ffi::yyjson_arr_iter,
    _doc_lifetime: std::marker::PhantomData<&'doc ()>,
}

impl<'doc> Iterator for ListIterator<'doc> {
    type Item = Val<'doc>;
    fn next(&mut self) -> Option<Self::Item> {
        let p = unsafe { ffi::yyjson_arr_iter_next(&mut self.iter) };
        if p.is_null() {
            None
        } else {
            Some(Val {
                p,
                _doc_lifetime: self._doc_lifetime,
            })
        }
    }
}

pub struct Obj<'doc> {
    p: *mut ffi::yyjson_val,
    // this lifetime is here to ensure that Doc remains 'live' as long as
    // the allocator underlying memory is 'live'
    _doc_lifetime: std::marker::PhantomData<&'doc ()>,
}

impl std::fmt::Debug for Obj<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.to_val();
        std::fmt::Debug::fmt(&v, f)
    }
}

impl std::fmt::Display for Obj<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.to_val();
        std::fmt::Display::fmt(&v, f)
    }
}

impl<'doc> Obj<'doc> {
    #[inline(always)]
    fn from_raw(
        p: *mut ffi::yyjson_val,
        _doc_lifetime: std::marker::PhantomData<&'doc ()>,
    ) -> Obj<'doc> {
        Obj { p, _doc_lifetime }
    }

    fn to_val(&self) -> Val<'doc> {
        Val {
            p: self.p,
            _doc_lifetime: self._doc_lifetime,
        }
    }

    #[inline(always)]
    fn key_to_str(v: &ffi::yyjson_val) -> &'doc str {
        let key = unsafe {
            let p = v.uni.str_ as *const u8;
            let len: usize = (v.tag >> NUM_TAG_BITS) as usize; // TAG bit should b 8 bits
            let buf = std::slice::from_raw_parts(p, len);
            std::str::from_utf8_unchecked(buf)
        };
        key
    }

    pub fn len(&self) -> usize {
        let v = unsafe { &*self.p };
        let len: usize = (v.tag >> NUM_TAG_BITS) as usize; // TAG bit should b 8 bits
        len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, key: &str) -> Option<Val<'doc>> {
        let p = unsafe { ffi::yyjson_obj_getn(self.p, key.as_ptr().cast(), key.len()) };
        if p.is_null() {
            None
        } else {
            Some(Val {
                // SAFETY: we already check if is null above, no need to check
                // again
                p,
                _doc_lifetime: self._doc_lifetime,
            })
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> ObjEntriesIterator<'doc> {
        ObjEntriesIterator {
            iter: unsafe { ffi::yyjson_obj_iter_with(self.p) },
            _doc_lifetime: self._doc_lifetime,
        }
    }

    #[inline(always)]
    pub fn keys(&self) -> ObjKeysIterator<'doc> {
        ObjKeysIterator {
            iter: unsafe { ffi::yyjson_obj_iter_with(self.p) },
            _doc_lifetime: self._doc_lifetime,
        }
    }

    #[inline(always)]
    pub fn ordered_getter(&self) -> ObjOrderedGetter<'doc> {
        ObjOrderedGetter {
            iter: unsafe { ffi::yyjson_obj_iter_with(self.p) },
            _doc_lifetime: self._doc_lifetime,
        }
    }
}

#[repr(transparent)]
pub struct ObjOrderedGetter<'doc> {
    iter: ffi::yyjson_obj_iter,
    _doc_lifetime: std::marker::PhantomData<&'doc ()>,
}

impl<'doc> ObjOrderedGetter<'doc> {
    // warning: this function takes a linear search time if the key is not
    // nearby
    pub fn get(&mut self, key: &str) -> Option<Val<'doc>> {
        let val_p =
            unsafe { ffi::yyjson_obj_iter_getn(&mut self.iter, key.as_ptr().cast(), key.len()) };
        if val_p.is_null() {
            None
        } else {
            Some(Val {
                p: val_p,
                _doc_lifetime: self._doc_lifetime,
            })
        }
    }
}

#[repr(transparent)]
pub struct ObjEntriesIterator<'doc> {
    iter: ffi::yyjson_obj_iter,
    _doc_lifetime: std::marker::PhantomData<&'doc ()>,
}

impl<'doc> Iterator for ObjEntriesIterator<'doc> {
    type Item = (&'doc str, Val<'doc>);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let key_p = unsafe { ffi::yyjson_obj_iter_next(&mut self.iter) };
        if key_p.is_null() {
            None
        } else {
            let key = Obj::key_to_str(unsafe { &*key_p });
            let val_p = unsafe { ffi::yyjson_obj_iter_get_val(key_p) };
            let val = Val {
                p: val_p,
                _doc_lifetime: self._doc_lifetime,
            };
            Some((key, val))
        }
    }
}

#[repr(transparent)]
pub struct ObjKeysIterator<'doc> {
    iter: ffi::yyjson_obj_iter,
    _doc_lifetime: std::marker::PhantomData<&'doc ()>,
}

impl<'doc> Iterator for ObjKeysIterator<'doc> {
    type Item = &'doc str;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let p = unsafe { ffi::yyjson_obj_iter_next(&mut self.iter) };
        if p.is_null() {
            None
        } else {
            Some(Obj::key_to_str(unsafe { &*p }))
        }
    }
}

#[repr(transparent)]
pub struct Val<'doc> {
    pub(crate) p: *mut ffi::yyjson_val,
    // this lifetime is here to ensure that Val remains 'live' as long as
    // the Doc it's contained in is 'live'
    pub(crate) _doc_lifetime: std::marker::PhantomData<&'doc ()>,
}

impl std::fmt::Debug for Val<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Val({:?}) {{ p: {:?} }}", self.get_type(), self.p)
    }
}

impl std::fmt::Display for Val<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let write_opts = crate::write::WriteOptions {
            pretty_with_two_spaces: true,
            ..Default::default()
        };
        // use yyjson's default allocator which is libc::malloc/free
        let allocator = BasicAllocProvider::default();
        let alc = allocator.get_allocator();
        let mut writer = Writer::new(alc, Some(&write_opts));
        let res = match self.write(&mut writer) {
            Ok(output) => output.fmt(f),
            // NOTE, need to provide out-of-band
            // means for checking exact error
            // that occured
            Err(_) => Err(std::fmt::Error),
        };
        res
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ValType {
    Null,
    Bool,
    UInt64,
    Int64,
    Float64,
    Str,
    List,
    Obj,
}

// CREDIT: https://github.com/ijl/orjson/blob/master/src/deserialize/yyjson.rs
const NUM_TAG_BITS: usize = 8;
const TAG_NULL: u8 = 0b00000010;
const TAG_FALSE: u8 = 0b00000011;
const TAG_TRUE: u8 = 0b00001011;
const TAG_INT64: u8 = 0b00001100;
const TAG_UINT64: u8 = 0b00000100;
const TAG_FLOAT64: u8 = 0b00010100;
const TAG_STRING: u8 = 0b00000101;
const TAG_STRING_NO_ESC: u8 = 0b00001101;
const TAG_LIST: u8 = 0b00000110;
const TAG_OBJ: u8 = 0b00000111;

impl ValType {
    #[inline(always)]
    fn from_u8(tag: u8) -> ValType {
        use ValType::*;
        match tag {
            TAG_NULL => Null,
            TAG_TRUE | TAG_FALSE => Bool,
            TAG_UINT64 => UInt64,
            TAG_INT64 => Int64,
            TAG_FLOAT64 => Float64,
            TAG_STRING | TAG_STRING_NO_ESC => Str,
            TAG_LIST => List,
            TAG_OBJ => Obj,
            _ => unreachable!(""),
        }
    }
}

impl<'doc> Val<'doc> {
    #[inline(always)]
    pub fn get_type(&self) -> ValType {
        // SAFETY: p in non-null and if tag is invalid, program panics rather
        // than initialize with invalid value. Doc constructor should and check
        // that json value is valid therefore all types should be one of the
        // enumeration values
        let tag: u8 = unsafe { (&*self.p).tag as u8 };
        ValType::from_u8(tag)
    }

    #[inline(always)]
    pub fn bool(&self) -> Option<bool> {
        let tag: u8 = unsafe { (&*self.p).tag as u8 };
        match tag {
            TAG_TRUE => Some(true),
            TAG_FALSE => Some(false),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn u64(&self) -> Option<u64> {
        if self.get_type() == ValType::UInt64 {
            Some(unsafe { (&*self.p).uni.u64_ })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn i64(&self) -> Option<i64> {
        if self.get_type() == ValType::Int64 {
            Some(unsafe { (&*self.p).uni.i64_ })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn f64(&self) -> Option<f64> {
        if self.get_type() == ValType::Float64 {
            Some(unsafe { (&*self.p).uni.f64_ })
        } else {
            None
        }
    }

    #[allow(non_snake_case)]
    #[inline(always)]
    // if value is f64, returns it as is, otherwise, returns NAN
    pub fn f64_or_NAN(&self) -> f64 {
        if self.get_type() == ValType::Float64 {
            unsafe { (&*self.p).uni.f64_ }
        } else {
            f64::NAN
        }
    }

    #[inline(always)]
    pub fn str(&self) -> Option<&'doc str> {
        if self.get_type() == ValType::Str {
            // SAFETY: everything here is pretty unsafe. As long as the type
            // checking has been done above (i.e. is ValType::Str), much of the
            // safety relies on yyjson API guarantees. For the length where
            // we're doing a right shift, num_bits in tag should remain 8
            // though and the tag should remain in the lower 8 bits region.
            // Val should only be constructed from valid values such that when
            // we're retrieving the string pointer, it should point to a valid
            // string.
            let s = unsafe {
                let v = &*self.p;
                let p = v.uni.str_ as *const u8;
                let len: usize = (v.tag >> NUM_TAG_BITS) as usize; // TAG bit should b 8 bits
                let buf = std::slice::from_raw_parts(p, len);
                std::str::from_utf8_unchecked(buf)
            };
            Some(s)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn list(&self) -> Option<List<'doc>> {
        if self.get_type() == ValType::List {
            Some(List::from_raw(self.p, self._doc_lifetime))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn obj(&self) -> Option<Obj<'doc>> {
        if self.get_type() == ValType::Obj {
            Some(Obj::from_raw(self.p, self._doc_lifetime))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn at_key(&self, key: &str) -> Option<Val<'doc>> {
        if self.get_type() == ValType::Obj {
            let obj = Obj::from_raw(self.p, self._doc_lifetime);
            obj.get(key)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn at_index(&self, index: usize) -> Option<Val<'doc>> {
        if self.get_type() == ValType::List {
            let list = List::from_raw(self.p, self._doc_lifetime);
            list.get(index)
        } else {
            None
        }
    }

    pub fn write<'a>(&self, writer: &'a mut Writer<'a>) -> Result<WriteOutput<'a>, WriteError> {
        writer.write(
            |write_flag: u32,
             alc: *mut ffi::yyjson_alc,
             len: &mut usize,
             write_err: *mut ffi::yyjson_write_err| {
                // SAFETY: it is okay if doc is null, yyjson writes out null string
                // otherwise doc should be a valid yyjson_doc
                unsafe {
                    ffi::yyjson_val_write_opts(self.p, write_flag, alc, len, write_err).cast::<u8>()
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::DocContext;

    fn eq_bool(val: &Val<'_>, v: bool) -> bool {
        if let Some(got) = val.bool() {
            got == v
        } else {
            false
        }
    }

    fn eq_u64(val: &Val<'_>, v: u64) -> bool {
        if let Some(got) = val.u64() {
            got == v
        } else {
            false
        }
    }
    fn eq_i64(val: &Val<'_>, v: i64) -> bool {
        if let Some(got) = val.i64() {
            got == v
        } else {
            false
        }
    }

    fn eq_f64(val: &Val<'_>, v: f64) -> bool {
        if let Some(got) = val.f64() {
            got == v
        } else {
            false
        }
    }

    fn eq_str(val: &Val<'_>, v: &str) -> bool {
        if let Some(got) = val.str() {
            got == v
        } else {
            false
        }
    }

    #[derive(Debug, Clone)]
    enum TestVal<'a> {
        Null,
        Bool(bool),
        UInt64(u64),
        Int64(i64),
        Float64(f64),
        Str(&'a str),
        List(Vec<TestVal<'a>>),
        Obj(Vec<(String, TestVal<'a>)>),
    }

    fn eq(got: &Val<'_>, expect: TestVal) -> bool {
        use TestVal::*;
        match expect {
            Null => got.get_type() == ValType::Null,
            Bool(v) => eq_bool(got, v),
            UInt64(v) => eq_u64(got, v),
            Int64(v) => eq_i64(got, v),
            Float64(v) => eq_f64(got, v),
            Str(v) => eq_str(got, v),
            List(l) => {
                let got_list = match got.list() {
                    Some(v) => v,
                    None => return false,
                };
                if got_list.len() != l.len() {
                    return false;
                }
                for (got_val, expect_val) in got_list.iter().zip(l.into_iter()) {
                    if !eq(&got_val, expect_val) {
                        return false;
                    }
                }
                return true;
            }
            Obj(expect_obj) => {
                let got_obj = match got.obj() {
                    Some(v) => v,
                    None => return false,
                };
                if got_obj.len() != expect_obj.len() {
                    return false;
                }
                for (key, expect_val) in expect_obj {
                    let got_val = match got_obj.get(&key) {
                        Some(v) => v,
                        None => return false,
                    };
                    if !eq(&got_val, expect_val) {
                        return false;
                    }
                }
                return true;
            }
        }
    }

    use super::*;
    #[test]
    fn is_correct_val_type() -> anyhow::Result<()> {
        use TestVal::*;
        struct TestCase<'a, 'b> {
            input: &'a [u8],
            is_type: unsafe extern "C" fn(*mut ffi::yyjson_val) -> bool,
            expected_type: ValType,
            expected_tag: u8,
            expect_val: TestVal<'b>,
        }
        let tc = |i: &'static str, f, e, t, v| TestCase {
            input: i.as_bytes(),
            is_type: f,
            expected_type: e,
            expected_tag: t,
            expect_val: v,
        };
        let test_cases = [
            tc("null", ffi::yyjson_is_null, ValType::Null, TAG_NULL, Null),
            tc(
                "true",
                ffi::yyjson_is_bool,
                ValType::Bool,
                TAG_TRUE,
                Bool(true),
            ),
            tc(
                "false",
                ffi::yyjson_is_bool,
                ValType::Bool,
                TAG_FALSE,
                Bool(false),
            ),
            tc(
                "100",
                ffi::yyjson_is_uint,
                ValType::UInt64,
                TAG_UINT64,
                UInt64(100),
            ),
            tc(
                "-100",
                ffi::yyjson_is_sint,
                ValType::Int64,
                TAG_INT64,
                Int64(-100),
            ),
            tc(
                "3.14",
                ffi::yyjson_is_real,
                ValType::Float64,
                TAG_FLOAT64,
                Float64(3.14),
            ),
            tc(
                "\"foo\"",
                ffi::yyjson_is_str,
                ValType::Str,
                TAG_STRING,
                Str("foo"),
            ),
            tc(
                "[100,200, 300]",
                ffi::yyjson_is_arr,
                ValType::List,
                TAG_LIST,
                List(vec![UInt64(100), UInt64(200), UInt64(300)]),
            ),
            tc(
                "{\"a\":100, \"b\": 200, \"c\": 300 }",
                ffi::yyjson_is_obj,
                ValType::Obj,
                TAG_OBJ,
                Obj(vec![
                    ("a".into(), UInt64(100)),
                    ("b".into(), UInt64(200)),
                    ("c".into(), UInt64(300)),
                ]),
            ),
        ];

        let dc = DocContext::default();
        for tc in test_cases {
            let doc = dc.parse(tc.input)?;
            let val = doc.root();
            assert!(
                unsafe { (tc.is_type)(val.p) },
                "yyjson fn for checking type {:?} returned false for given val: \"{}\"",
                tc.expected_type,
                std::str::from_utf8(tc.input).unwrap()
            );
            let got_type = val.get_type();
            let got_tag: u8 = unsafe { (&*val.p).tag as u8 };
            assert!(
                got_type == tc.expected_type,
                "val type enum check for \"{}\", expect {:?}, got {:?}.\nexp tag={:#08b}\ngot tag={:#08b}",
                std::str::from_utf8(tc.input).unwrap(),
                tc.expected_type,
                got_type,
                tc.expected_tag,
                got_tag
            );
            assert!(
                eq(&val, tc.expect_val.clone()),
                "on checking underlying val \"{}\", expect: {:?}",
                std::str::from_utf8(tc.input).unwrap(),
                tc.expect_val
            );
        }

        Ok(())
    }

    #[test]
    fn list_access() -> anyhow::Result<()> {
        let data = r#"[10,20,30]"#;
        let content: &[u8] = data.as_bytes();
        let dc = DocContext::default();
        let doc = dc.parse(content)?;
        let root = doc.root();
        let list = root.list().unwrap();
        assert!(list.len() == 3);
        assert!(list.first().unwrap().u64() == Some(10));
        assert!(list.get(1).unwrap().u64() == Some(20));
        assert!(list.get(3).is_none());
        assert!(list.last().unwrap().u64() == Some(30));

        let got: Vec<u64> = list.iter().filter_map(|v| v.u64()).collect();
        assert_eq!(got, vec![10, 20, 30]);
        Ok(())
    }

    #[test]
    fn empty_list_access() -> anyhow::Result<()> {
        let data = r#"[  ]"#;
        let content: &[u8] = data.as_bytes();
        let dc = DocContext::default();
        let doc = dc.parse(content)?;
        let root = doc.root();
        let list = root.list().unwrap();
        assert!(list.len() == 0);
        let got: Vec<u64> = list.iter().filter_map(|v| v.u64()).collect();
        assert!(got.len() == 0);
        Ok(())
    }

    #[test]
    fn obj_access() -> anyhow::Result<()> {
        let data = r#"{"a": 10, "b": 20, "c": 30, "d": 40, "e": 50}"#;
        let content: &[u8] = data.as_bytes();

        let dc = DocContext::default();
        let doc = dc.parse(content)?;
        let root = doc.root();
        let obj = root.obj().unwrap();
        assert_eq!(obj.len(), 5);
        assert_eq!(obj.get("a").unwrap().u64(), Some(10));
        assert_eq!(obj.get("b").unwrap().u64(), Some(20));
        assert_eq!(obj.get("c").unwrap().u64(), Some(30));
        assert_eq!(obj.get("d").unwrap().u64(), Some(40));
        assert_eq!(obj.get("e").unwrap().u64(), Some(50));
        assert!(obj.get("_not_present").is_none());

        let entries: Vec<(&str, u64)> = obj
            .iter()
            .filter_map(|(k, v)| v.u64().map(|n| (k, n)))
            .collect();

        assert_eq!(
            entries,
            vec![("a", 10), ("b", 20), ("c", 30), ("d", 40), ("e", 50)]
        );

        let keys: Vec<&str> = obj.keys().collect();
        assert_eq!(keys, vec!["a", "b", "c", "d", "e"]);

        let mut getter = obj.ordered_getter();
        assert_eq!(getter.get("a").unwrap().u64(), Some(10));
        assert_eq!(getter.get("e").unwrap().u64(), Some(50));
        assert!(getter.get("_not_present").is_none());
        assert_eq!(getter.get("b").unwrap().u64(), Some(20)); // not cool
        assert_eq!(getter.get("c").unwrap().u64(), Some(30));
        assert_eq!(getter.get("d").unwrap().u64(), Some(40));

        Ok(())
    }
}
