use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use crate::*;
use crate::lang::*;

#[derive(Debug)]
pub enum RtableType {
    SYS_TABLE,
    USR_TABLE,
    TMP_TABLE,
}

#[derive(Debug)]
pub struct Rtable {
    pub tp: RtableType,
    pub tb: HashMap<String, PRtoken>,
    pub ft: Option<PRtable>,
}

#[derive(Debug)]
pub struct PRtable(pub Arc<RwLock<Rtable>>);


impl PRtable {
    pub fn new(t: RtableType)-> PRtable{
        PRtable(Arc::new(RwLock::new(Rtable{
            tp: t,
            tb: HashMap::new(),
            ft: None,
        })))
    }

    pub fn new_with_father(t: RtableType, f: PRtable)-> PRtable{
        PRtable(Arc::new(RwLock::new(Rtable{
            tp: t,
            tb: HashMap::new(),
            ft: Some(f),
        })))
    }

    pub fn clone(&self)-> PRtable{
        PRtable(self.0.clone())
    }

    pub fn room(&self)-> i64{
        ptr_raw!(self).tb.capacity() as i64
    }

    pub fn len(&self)-> i64{
        ptr_raw!(self).tb.len() as i64
    }

    pub fn fit_size(&mut self){
        ptr_raw_mut!(self).tb.shrink_to_fit()
    }

    pub fn has_father(&self)-> bool{
        if let Some(ref f) = ptr_raw!(self).ft {
            return true
        }
        return false
    }

    pub fn has_no_sys_father(&self)-> bool{
        if let Some(ref f) = ptr_raw!(self).ft {
            match ptr_raw!(f).tp {
                RtableType::SYS_TABLE => return false,
                _ => return true,
            }
        }
        return false
    }

    pub fn set_father(&mut self, f: PRtable){
        ptr_raw_mut!(self).ft = Some(f)
    }

    pub fn put_now(&mut self, k: String, v: PRtoken) {
        ptr_raw_mut!(self).tb.insert(k, v);
    }

    pub fn get_now(&self, k: String)-> PRtoken {
        if let Some(v) = ptr_raw!(self).tb.get(&k){
            return v.clone()
        }
        PRtoken::new_nil()
    }

    pub fn remove_now(&mut self, k: String){
        ptr_raw_mut!(self).tb.remove(&k);
    }

    pub fn put(&mut self, k: String, v: PRtoken) {
        let mut table = self.clone();

        loop {
            if ptr_raw!(table).tb.contains_key(&k) {
                table.put_now(k.clone(), v.clone());
                return
            }

            if let Some(ref f) = ptr_raw!(table.clone()).ft {
                match ptr_raw!(f).tp {
                    RtableType::SYS_TABLE => (),
                    _ => {
                        table = f.clone();
                        continue
                    }
                }
            }

            table.put_now(k.clone(), v.clone());
            return
        }

    }

    pub fn get(&self, k: String)-> PRtoken{
        let mut table = self.clone();
        loop {
            let v = table.get_now(k.clone());
            if v.is_nil(){
                if let Some(ref f) = ptr_raw!(table.clone()).ft {
                    table = f.clone();
                }else{
                    break
                }
            }else{
                return v
            }
        }

        return PRtoken::new_nil()
    }

    pub fn remove(&mut self, k: String){
        let mut table = self.clone();
        loop {
            if ptr_raw!(table).tb.contains_key(&k){
                table.remove_now(k.clone());
                if table.room() / table.len() > 4 {
                    table.fit_size();
                }
                return
            }

            if let Some(ref f) = ptr_raw!(table.clone()).ft {
                match ptr_raw!(f).tp {
                    RtableType::SYS_TABLE => return,
                    _ => {
                        table = f.clone();
                        continue
                    }
                }
            }

        }

    }


}














