use crate::aqbanking_wrapper;
use std::ffi::CString;


/*
pub struct GWEN_ARGS {
    pub flags: u32,
    pub type_: GWEN_ARGS_TYPE,
    pub name: *const ::std::os::raw::c_char,
    pub minNum: ::std::os::raw::c_uint,
    pub maxNum: ::std::os::raw::c_uint,
    pub shortOption: *const ::std::os::raw::c_char,
    pub longOption: *const ::std::os::raw::c_char,
    pub shortDescription: *const ::std::os::raw::c_char,
    pub longDescription: *const ::std::os::raw::c_char,
}
*/

fn get_as_c_string(string: &str) -> *const i8 {
    CString::new(string).expect("CString::new failed").as_ptr()
} 

const null_ptr: i8 = 0;

pub fn list_accs(ab: aqbanking_wrapper::AB_BANKING, db_args: *mut aqbanking_wrapper::GWEN_DB_NODE, argc: usize, argv: Vec<String>) -> i32 {
    println!("Called list_accs with: ab = {:?}, db_args = {:?}, argc = {:?}, argv = {:?}", ab, db_args, argc, argv);
    let db: *mut aqbanking_wrapper::GWEN_DB_NODE;
    let al: aqbanking_wrapper::AB_ACCOUNT_SPEC_LIST;
    let acc_spec: aqbanking_wrapper::AB_ACCOUNT_SPEC; // as in origin
    let rv: i32;
    const tmplString: String = String::new();

    let args = [
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HAS_ARGUMENT,
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Int,
            name: get_as_c_string("uniqueAccountId"),
            minNum: 0,
            maxNum: 1,
            shortOption: &null_ptr,
            longOption: get_as_c_string("aid"),
            shortDescription: get_as_c_string("Specify the unique account id"),
            longDescription: get_as_c_string("Specify the unique account id"),
        },
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HAS_ARGUMENT, /* flags */
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Char,            /* type */
            name: get_as_c_string("backendName"),                     /* name */
            minNum: 0,                            /* minnum */
            maxNum: 1,                            /* maxnum */
            shortOption: &null_ptr,                          /* short option */
            longOption: get_as_c_string("backend"),                       /* long option */
            shortDescription: get_as_c_string("Specify the name of the backend for your account"),      /* short description */
            longDescription: get_as_c_string("Specify the name of the backend for your account")       /* long description */
        },
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HAS_ARGUMENT, /* flags */
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Char,            /* type */
            name: get_as_c_string("country"),                     /* name */
            minNum: 0,                            /* minnum */
            maxNum: 1,                            /* maxnum */
            shortOption: &null_ptr,                          /* short option */
            longOption: get_as_c_string("country"),                       /* long option */
            shortDescription: get_as_c_string("Specify the country for your account (e.g. \"de\")"),      /* short description */
            longDescription: get_as_c_string("Specify the country for your account (e.g. \"de\")")       /* long description */
        },
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HAS_ARGUMENT, /* flags */
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Char,            /* type */
            name: get_as_c_string("bankId"),                     /* name */
            minNum: 0,                            /* minnum */
            maxNum: 1,                            /* maxnum */
            shortOption: get_as_c_string("b"),                          /* short option */
            longOption: get_as_c_string("bank"),                       /* long option */
            shortDescription: get_as_c_string("Specify the bank code"),      /* short description */
            longDescription: get_as_c_string("Specify the bank code")       /* long description */
        },
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HAS_ARGUMENT, /* flags */
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Char,            /* type */
            name: get_as_c_string("accountId"),                  /* name */
            minNum: 0,                            /* minnum */
            maxNum: 1,                            /* maxnum */
            shortOption: get_as_c_string("a"),                          /* short option */
            longOption: get_as_c_string("account"),                    /* long option */
            shortDescription: get_as_c_string("Specify the account number"),     /* short description */
            longDescription: get_as_c_string("Specify the account number")      /* long description */
        },
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HAS_ARGUMENT, /* flags */
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Char,           /* type */
            name: get_as_c_string("subAccountId"),                /* name */
            minNum: 0,                            /* minnum */
            maxNum: 1,                            /* maxnum */
            shortOption: get_as_c_string("aa"),                          /* short option */
            longOption: get_as_c_string("subaccount"),                   /* long option */
            shortDescription: get_as_c_string("Specify the sub account id (Unterkontomerkmal)"),    /* short description */
            longDescription: get_as_c_string("Specify the sub account id (Unterkontomerkmal)")     /* long description */
        },
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HAS_ARGUMENT, /* flags */
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Char,            /* type */
            name: get_as_c_string("iban"),                        /* name */
            minNum: 0,                             /* minnum */
            maxNum: 1,                             /* maxnum */
            shortOption: get_as_c_string("A"),                           /* short option */
            longOption: get_as_c_string("iban"),                       /* long option */
            shortDescription: get_as_c_string("Specify the iban of your account"),      /* short description */
            longDescription: get_as_c_string("Specify the iban of your account")       /* long description */
        },
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HAS_ARGUMENT, /* flags */
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Char,            /* type */
            name: get_as_c_string("accountType"),                 /* name */
            minNum: 0,                             /* minnum */
            maxNum: 1,                             /* maxnum */
            shortOption: get_as_c_string("t"),                           /* short option */
            longOption: get_as_c_string("accounttype"),                       /* long option */
            shortDescription: get_as_c_string("Specify the type of your account"),      /* short description */
            longDescription: get_as_c_string("Specify the type of your account")       /* long description */
        },
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HAS_ARGUMENT, /* flags */
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Char,            /* type */
            name: get_as_c_string("template"),                    /* name */
            minNum: 0,                            /* minnum */
            maxNum: 1,                            /* maxnum */
            shortOption: get_as_c_string("T"),                          /* short option */
            longOption: get_as_c_string("template"),                       /* long option */
            shortDescription: get_as_c_string("Specify the template for the account list output"),      /* short description */
            longDescription: get_as_c_string("Specify the template for the account list output")       /* long description */
        },
        aqbanking_wrapper::GWEN_ARGS {
            flags: aqbanking_wrapper::GWEN_ARGS_FLAGS_HELP | aqbanking_wrapper::GWEN_ARGS_FLAGS_LAST, /* flags */
            type_: aqbanking_wrapper::GWEN_ARGS_TYPE_GWEN_ArgsType_Int,             /* type */
            name: get_as_c_string("help"),                       /* name */
            minNum: 0,                            /* minnum */
            maxNum: 0,                            /* maxnum */
            shortOption: get_as_c_string("h"),                          /* short option */
            longOption: get_as_c_string("help"),                       /* long option */
            shortDescription: get_as_c_string("Show this help screen"),      /* short description */
            longDescription: get_as_c_string("Show this help screen")       /* long description */
        }];

    unsafe {
        //db = aqbanking_wrapper::GWEN_DB_GetGroup(db_args, aqbanking_wrapper::GEN_DB_FLAGS_DEFAULT, get_as_c_string("local"));
    }

    15
}
