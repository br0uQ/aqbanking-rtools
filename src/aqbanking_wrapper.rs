#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::list_accs;
use std::env;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn hello() {
    println!("Hello from the aqbanking wrapper");
}

pub fn list_accounts() {
    let mut ret: i32;
    let mut ab: AB_BANKING = AB_BANKING{_unused: []};
    let db: *mut GWEN_DB_NODE = &mut GWEN_DB_NODE{_unused: []};
    let mut argv: Vec<String> = env::args().collect();
    let mut argc: usize = argv.len();
    println!("Try to list accounts");
    
    unsafe {
        ret = GWEN_Init();
        println!("GWEN_Init returned {:?}", ret);

        ret = GWEN_I18N_BindTextDomain_Dir();
    }

    ret = list_accs::list_accs(ab, db, argc, argv);
    //ret = listAccs(ab, db, argc, argv);
    println!("list_accs returned {:?}", ret);
}
