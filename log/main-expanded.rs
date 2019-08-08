arg_captured: ArgCaptured {
    pat: Ident(
        PatIdent {
            by_ref: None,
            mutability: None,
            ident: Ident {
                ident: "sensor",
                span: #0 bytes(0..0),
            },
            subpat: None,
        },
    ),
    colon_token: Colon,
    ty: Tuple(
        TypeTuple {
            paren_token: Paren,
            elems: [],
        },
    ),
}
pat: "sensor"
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

/// Testing safe_wrap
#[cfg(feature = "test_safe_wrap")]
mod test_safe_wrap {
    use cty::*;
    //  Import C types from cty library: https://crates.io/crates/cty

    extern crate macros as proc_macros;
    use proc_macros::{init_strn};
    //  Import Mynewt macros from `macros` library

    extern crate mynewt;
    use mynewt::{result::*,  //  Import Mynewt result and error types
                 kernel::os::{
                              //self,             //  Import Mynewt OS functions
                              os_eventq,  //  Import Mynewt OS types
                              os_task, os_stack_t, os_task_func_t, os_time_t},
                 hw::sensor::{sensor}, encoding::{coap_context::{}},
                  //  Import Mynewt JSON Encoder Context
                  //self,
                  //COAP_CONTEXT,
                  //ToBytesOptionalNull,

                 //tinycbor,         //  Import Mynewt TinyCBOR API
                 libs::{
                        //mynewt_rust,      //  Import Mynewt Rust Helper API
                        //sensor_network,   //  Import Mynewt Sensor Network API
                        sensor_coap::{}},
                  //  Import Mynewt Sensor CoAP API
                  //self,
                  //sensor_value,

                 //coap, d, fill_zero,   //  Import Mynewt macros
                 NULL, Out, Ptr, Strn};

    ///////////////////////////////////////////////////////////////////////////////
    //  Testing

    fn test_safe_wrap() -> MynewtResult<()> {
        "-------------------------------------------------------------";
        "-------------------------------------------------------------";
        //#[proc_macros::safe_wrap(attr)] 
        extern "C" {
            pub fn get_device_id() -> *const ::cty::c_char;
        }
        //#[proc_macros::safe_wrap(attr)] 
        extern "C" {
            pub static mut os_main_task: os_task;
        }
        //#[proc_macros::safe_wrap(attr)] ////
        extern "C" {
            #[doc = ""]
            pub fn start_server_transport() -> ::cty::c_int;
        }
        //#[proc_macros::safe_wrap(attr)] ////
        extern "C" {
            pub fn init_server_post(uri: *const ::cty::c_char) -> bool;
        }
        //#[proc_macros::safe_wrap(attr)] ////
        extern "C" {
            pub fn do_server_post() -> bool;
        }
        //#[proc_macros::safe_wrap(attr)] ////
        extern "C" {
            #[doc = " Set the sensor poll rate"]
            #[doc = ""]
            #[doc = " - __`devname`__: Name of the sensor"]
            #[doc = " - __`poll_rate`__: The poll rate in milli seconds"]
            pub fn sensor_set_poll_rate_ms(devname: *const ::cty::c_char,
                                           poll_rate: u32) -> ::cty::c_int;
        }
        //#[proc_macros::safe_wrap(attr)] ////
        extern "C" {
            #[doc =
                  " Search the sensor list and find the next sensor that corresponds"]
            #[doc = " to a given device name."]
            #[doc = ""]
            #[doc = " - __`devname`__: The device name to search for"]
            #[doc =
                  " - __`sensor`__: The previous sensor found with this device name"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure"]
            pub fn sensor_mgr_find_next_bydevname(devname:
                                                      *const ::cty::c_char,
                                                  prev_cursor: *mut sensor)
             -> *mut sensor;
        }
        //#[proc_macros::safe_wrap(attr)] ////
        extern "C" {
            #[doc =
                  " Pull a single item off the event queue and call it's event"]
            #[doc = " callback."]
            #[doc = ""]
            #[doc = " - __`evq`__: The event queue to pull the item off."]
            pub fn os_eventq_run(evq: *mut os_eventq);
        }
        //#[proc_macros::safe_wrap(attr)] ////
        extern "C" {
            #[doc =
                  " Retrieves the default event queue processed by OS main task."]
            #[doc = ""]
            #[doc = " Return:                      The default event queue."]
            pub fn os_eventq_dflt_get() -> *mut os_eventq;
        }
        //#[proc_macros::safe_wrap(attr)] ////
        extern "C" {
            pub fn os_task_init(arg1: *mut os_task,
                                arg2: *const ::cty::c_char,
                                arg3: os_task_func_t,
                                arg4: *mut ::cty::c_void, arg5: u8,
                                arg6: os_time_t, arg7: *mut os_stack_t,
                                arg8: u16) -> ::cty::c_int;
        }
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
        let _test_local = Strn{rep: mynewt::StrnRep::ByteStr(b"hello\x00"),};
        Ok(())
    }

    static _test_static: Strn =
        Strn{rep: mynewt::StrnRep::ByteStr(b"hello\x00"),};
}

// Note this useful idiom: importing names from outer (for mod tests) scope.
// use super::*;


//assert_eq!(1, 1);

// If `e` is an expression like `123`, `i * 2`, `func(i)`...
// Add 88 to the expression and return it.
// If `e` is an expression, `i` is an identifier like `myvar`, `blk` is a block of statements...
// Add 88 and the value of `i` to the expression.
// Execute the code block.
// Return the result.

//  This rule matches zero or more tokens.
//  For all matched tokens, convert into a string.

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

// The caller shall pass in a recipe for the soup, 
// say `salt + 88`
// We use our own salt, say Table Salt.
// We try to use our salt, but the recipe 
// actually requires a different salt, 
// like Sea Salt.
// Rust Compiler fails with a Hygiene Error.

// The caller shall pass in 2 things: 
// Which salt to use
// and the recipe (like `salt + 88`)
// We use whatever salt the caller passes in. 
// Hence we use `$salt` instead of `salt`.
// Then we use the same salt inside the recipe.
// It works!

// We try to make soup with our salt and recipe...
// If you're testing other macros, comment out the `+ salt` part or the code will never compile
// But the salt isn't the same inside the recipe.  
// Rust Compiler fails with Hygiene Error:
// `salt` not found in this scope.
 //  Return the cooked soup.

// First we tell the macro which salt we're using...
// Then we tell the macro what to do with that salt.
//  It works!
//  Return the cooked soup.

//  Import everything from outer scope.


//  Expression
//  Identifier
//  Code Block
// Shows `The value of x and y are 2 and 3`. Result is 99.




//  Start tracing macros
//  Stop tracing macros


fn main() { }
