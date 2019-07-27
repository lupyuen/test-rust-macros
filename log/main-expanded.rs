#![feature(prelude_import)]
#![no_std]
#![feature(trace_macros)]
//  Enable tracing of macros
#![feature(proc_macro_hygiene)]
//  Allow proc macros to be unhygienic
#![feature(custom_attribute)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
//  Allow custom attributes like [safe_wrap]

/// Given an expression `e`, add 88. If identifier `i` and statement block `blk` are specified, add `i` to the result and execute `blk`.
macro_rules! add_88(( $ e : expr ) => { { $ e + 88 } } ; (
                    $ e : expr , $ i : ident , $ blk : block ) => {
                    { let result = $ e + $ i + 88 ; $ blk ; result } });
// If `e` is an expression like `123`, `i * 2`, `func(i)`...
// Add 88 to the expression and return it.
// If `e` is an expression, `i` is an identifier like `myvar`, `blk` is a block of statements...
// Add 88 and the value of `i` to the expression.
// Execute the code block.
// Return the result.

///  Macro to dump all tokens received as a literal string, e.g.
///  `d!(a b c)` returns `"a b c"`
macro_rules! d(( $ ( $ token : tt ) * ) => { stringify ! ( $ ( $ token ) * ) }
               ;);
//  This rule matches zero or more tokens.
//  For all matched tokens, convert into a string.

macro_rules! parse((
                   @ $ enc : ident @ object $ obj : ident [ $ ( $ key : tt ) +
                   ] ( $ value : expr ) , $ ( $ rest : tt ) * ) => {
                   d ! (
                   adding key : $ ( $ key ) + value : $ value to object : $
                   obj ) } ;);
// Helper macro to parse a JSON `key: value` entry. The entry should be followed by a trailing comma.
// For example: When parsing the JSON code `{ "device": "010203" , (omitted) }`, the macro will be called like this:
// ```
// parse!( @json @object context ["device"] ("010203") , (omitted) )
// ```

//  Append to the "values" array e.g.
//    {"key":"device", "value":"010203"},
//  $crate::coap_item_str!(@$enc $obj, $($key)+, $value);

//  Continue expanding the rest of the JSON.
//  $crate::parse!(@$enc @object $obj () ($($rest)*) ($($rest)*));

/// Let's make some soup based on a recipe...
macro_rules! bad_soup(( $ recipe : expr ) => { { let salt = 1 ; $ recipe } });
// The caller shall pass in a recipe for the soup, 
// say `salt + 88`
// We use our own salt, say Table Salt.
// We try to use our salt, but the recipe 
// actually requires a different salt, 
// like Sea Salt.
// Rust Compiler fails with a Hygiene Error.

/// Let's make soup the right way...
macro_rules! good_soup(( $ salt : ident , $ recipe : expr ) => {
                       { let $ salt = 1 ; $ recipe } });
// The caller shall pass in 2 things: 
// Which salt to use
// and the recipe (like `salt + 88`)
// We use whatever salt the caller passes in. 
// Hence we use `$salt` instead of `salt`.
// Then we use the same salt inside the recipe.
// It works!

/// Make an Unhygienic soup... 
fn make_bad_soup() -> Result<i32, ()> {
    let soup =
        {
            let salt = 1;

            // We try to make soup with our salt and recipe...
            88 + salt
        }; // If you're testing other macros, comment out the `+ salt` part or the code will never compile
           // But the salt isn't the same inside the recipe.  
           // Rust Compiler fails with Hygiene Error:
           // `salt` not found in this scope.

    Ok(soup) //  Return the cooked soup.
}

/// Make a Hygienic soup
fn make_good_soup() -> Result<i32, ()> {
    let soup =
        {
            let salt = 1;

            // First we tell the macro which salt we're using...
            // Then we tell the macro what to do with that salt.
            88 + salt
        };
    //  It works!
    Ok(soup) //  Return the cooked soup.
}

//  Import everything from outer scope.


//  Expression
//  Identifier
//  Code Block
// Shows `The value of x and y are 2 and 3`. Result is 99.




/// Let's start cooking!
fn main() {
    (); //  Start tracing macros
    let res = { 1 + 88 };
    (); //  Stop tracing macros


    ////#[cfg(test_safe_wrap_macro)]
    //  Import Mynewt macros from `macros` library

    ///////////////////////////////////////////////////////////////////////////////
    //  Testing


    //  Last byte must be 0.

    //  Last byte must be 0.

    //  Last byte must be 0.

    //  Last byte must be 0.

    //  Last byte must be 0.



    ////
    /*
    #[mynewt_macros::safe_wrap(attr)] ////
    extern "C" {
        #[doc = " Retrieves the default event queue processed by OS main task."]
        #[doc = ""]
        #[doc = " Return:                      The default event queue."]
        pub fn os_eventq_dflt_get() -> *mut os_eventq;
    }
    */
    /*
        #[mynewt_macros::safe_wrap(attr)]
        extern "C" {
            pub fn os_task_init(
                arg1: *mut os_task,
                arg2: *const ::cty::c_char,
                arg3: os_task_func_t,
                arg4: *mut ::cty::c_void,
                arg5: u8,
                arg6: os_time_t,
                arg7: *mut os_stack_t,
                arg8: u16,
            ) -> ::cty::c_int;
        }
    */
    /*
        type Out<T> = &'static mut T;
        type Ptr = *mut ::cty::c_void;
        const NULL: Ptr = 0 as Ptr;

        task_init(                      //  Create a new task and start it...
            out!( NETWORK_TASK ),       //  Task object will be saved here
            strn!( "network" ),         //  Name of task
            Some( network_task_func ),  //  Function to execute when task starts
            NULL,  //  Argument to be passed to above function
            10,    //  Task priority: highest is 0, lowest is 255 (main task is 127)
            os::OS_WAIT_FOREVER as u32,   //  Don't do sanity / watchdog checking
            out!( NETWORK_TASK_STACK ),   //  Stack space for the task
            NETWORK_TASK_STACK_SIZE       //  Size of the stack (in 4-byte units)
        )?;                               //  `?` means check for error

        pub fn OLDtask_init(
            t: Out<os_task>,  //  TODO: *mut os_task
            name: &Strn,      //  TODO: *const ::cty::c_char
            func: os_task_func_t,
            arg: Ptr,         //  TODO: *mut ::cty::c_void
            prio: u8,
            sanity_itvl: os_time_t,
            stack_bottom: Out<[os_stack_t]>,  //  TODO: *mut os_stack_t
            stack_size: usize,                //  TODO: u16
        ) -> MynewtResult<()> {               //  TODO: ::cty::c_int;
            extern "C" {
                pub fn os_task_init(
                    t: *mut os_task,
                    name: *const ::cty::c_char,
                    func: os_task_func_t,
                    arg: *mut ::cty::c_void,
                    prio: u8,
                    sanity_itvl: os_time_t,
                    stack_bottom: *mut os_stack_t,
                    stack_size: u16,
                ) -> ::cty::c_int;
            }
            Strn::validate_bytestr(name.bytestr);  //  TODO
            unsafe {
                let res = os_task_init(
                    t,
                    name.bytestr.as_ptr() as *const ::cty::c_char,  //  TODO
                    func,
                    arg,
                    prio,
                    sanity_itvl,
                    stack_bottom.as_ptr() as *mut os_stack_t,  //  TODO
                    stack_size as u16       //  TODO
                );
                if res == 0 { Ok(()) }
                else { Err(MynewtError::from(res)) }
            }
        }

        #[doc = " Initialize a task."]
        #[doc = ""]
        #[doc = " This function initializes the task structure pointed to by t,"]
        #[doc = " clearing and setting it's stack pointer, provides sane defaults"]
        #[doc = " and sets the task as ready to run, and inserts it into the operating"]
        #[doc = " system scheduler."]
        #[doc = ""]
        #[doc = " - __`t`__: The task to initialize"]
        #[doc = " - __`name`__: The name of the task to initialize"]
        #[doc = " - __`func`__: The task function to call"]
        #[doc = " - __`arg`__: The argument to pass to this task function"]
        #[doc = " - __`prio`__: The priority at which to run this task"]
        #[doc = " - __`sanity_itvl`__: The time at which this task should check in with the"]
        #[doc = "                    sanity task.  OS_WAIT_FOREVER means never check in"]
        #[doc = "                    here."]
        #[doc = " - __`stack_bottom`__: A pointer to the bottom of a task's stack"]
        #[doc = " - __`stack_size`__: The overall size of the task's stack."]
        #[doc = ""]
        #[doc = " Return: 0 on success, non-zero on failure."]
        fn dummy() {}
    */

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;


    //assert_eq!(1, 1);
    {
        ::std::io::_print(::std::fmt::Arguments::new_v1(&["test_add_88: ",
                                                          "\n"],
                                                        &match (&res,) {
                                                             (arg0,) =>
                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                          ::std::fmt::Display::fmt)],
                                                         }));
    };
    "adding key : \"device\" value : \"010203\" to object : context";
    {
        ::std::io::_print(::std::fmt::Arguments::new_v1(&["bad soup: ", "\n"],
                                                        &match (&make_bad_soup().unwrap(),)
                                                             {
                                                             (arg0,) =>
                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                          ::std::fmt::Display::fmt)],
                                                         }));
    };
    {
        ::std::io::_print(::std::fmt::Arguments::new_v1(&["good soup: ",
                                                          "\n"],
                                                        &match (&make_good_soup().unwrap(),)
                                                             {
                                                             (arg0,) =>
                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                          ::std::fmt::Display::fmt)],
                                                         }));
    };
}
/// Testing safe_wrap
mod tests {
    extern crate mynewt_macros;
    use mynewt_macros::{out, strn, init_strn};
    /// Represents a null-terminated byte string, suitable for passing to Mynewt APIs as `* const char`
    pub struct Strn {
        /// Byte string terminated with null
        pub bytestr: &'static [u8],
    }
    impl Strn {
        /// Create a new byte string. Fail if the last byte is not zero.
        /// ```
        /// Strn::new(b"network\0")
        /// strn!("network")
        /// ```
        pub fn new(bs: &'static [u8]) -> Strn {
            {
                match (&bs.last(), &Some(&0u8)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                            "`,\n right: `",
                                                                                            "`"],
                                                                                          &match (&&*left_val,
                                                                                                  &&*right_val)
                                                                                               {
                                                                                               (arg0,
                                                                                                arg1)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Debug::fmt),
                                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::std::fmt::Debug::fmt)],
                                                                                           }),
                                                           &("src/main.rs",
                                                             211u32, 13u32))
                            }
                        }
                    }
                }
            };
            let res = Strn{bytestr: bs,};
            res
        }
        /// Return the byte string as a null-terminated `* const char` C-style string.
        /// Fail if the last byte is not zero.
        pub fn as_cstr(self) -> *const ::cty::c_char {
            let bs: &'static [u8] = self.bytestr;
            {
                match (&bs.last(), &Some(&0u8)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                            "`,\n right: `",
                                                                                            "`"],
                                                                                          &match (&&*left_val,
                                                                                                  &&*right_val)
                                                                                               {
                                                                                               (arg0,
                                                                                                arg1)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Debug::fmt),
                                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::std::fmt::Debug::fmt)],
                                                                                           }),
                                                           &("src/main.rs",
                                                             221u32, 13u32))
                            }
                        }
                    }
                }
            };
            bs.as_ptr() as *const ::cty::c_char
        }
        /// Return the byte string.
        /// Fail if the last byte is not zero.
        pub fn as_bytestr(self) -> &'static [u8] {
            let bs: &'static [u8] = self.bytestr;
            {
                match (&bs.last(), &Some(&0u8)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                            "`,\n right: `",
                                                                                            "`"],
                                                                                          &match (&&*left_val,
                                                                                                  &&*right_val)
                                                                                               {
                                                                                               (arg0,
                                                                                                arg1)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Debug::fmt),
                                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::std::fmt::Debug::fmt)],
                                                                                           }),
                                                           &("src/main.rs",
                                                             230u32, 13u32))
                            }
                        }
                    }
                }
            };
            &bs
        }
        /// Fail if the last byte is not zero.
        pub fn validate(self) {
            let bs = &self.bytestr;
            {
                match (&bs.last(), &Some(&0u8)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                            "`,\n right: `",
                                                                                            "`"],
                                                                                          &match (&&*left_val,
                                                                                                  &&*right_val)
                                                                                               {
                                                                                               (arg0,
                                                                                                arg1)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Debug::fmt),
                                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::std::fmt::Debug::fmt)],
                                                                                           }),
                                                           &("src/main.rs",
                                                             238u32, 13u32))
                            }
                        }
                    }
                }
            };
        }
        /// Fail if the last byte is not zero.
        pub fn validate_bytestr(bs: &'static [u8]) {
            {
                match (&bs.last(), &Some(&0u8)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                            "`,\n right: `",
                                                                                            "`"],
                                                                                          &match (&&*left_val,
                                                                                                  &&*right_val)
                                                                                               {
                                                                                               (arg0,
                                                                                                arg1)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Debug::fmt),
                                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                            ::std::fmt::Debug::fmt)],
                                                                                           }),
                                                           &("src/main.rs",
                                                             244u32, 13u32))
                            }
                        }
                    }
                }
            };
        }
    }
    static _test_static: Strn = Strn{bytestr: b"hello\0",};
    fn test_safe_wrap() -> MynewtResult<()> {
        let _test_local = Strn{bytestr: b"hello\0",};
        "-------------------------------------------------------------";
        pub fn eventq_run(evq: *mut os_eventq) -> MynewtResult<()> {
            "----------Insert Extern Decl: `extern C { pub fn ... }`----------";
            extern "C" {
                #[doc =
                      " Pull a single item off the event queue and call it's event"]
                #[doc = " callback."]
                #[doc = ""]
                #[doc = " - __`evq`__: The event queue to pull the item off."]
                pub fn os_eventq_run(evq: *mut os_eventq);
            }
            "----------Insert Validation: `Strn::validate_bytestr(name.bytestr)`----------";
            unsafe {
                "----------Insert Call: `let result_code = os_task_init(`----------";
                let result_code = os_eventq_run(evq as *mut os_eventq);
                if result_code == 0 {
                    Ok(())
                } else { Err(MynewtError::from(result_code)) }
            }
        }
        "-------------------------------------------------------------";
        "-------------------------------------------------------------";
        "-------------------------------------------------------------";
        "-------------------------------------------------------------";
        Ok(())
    }
}
