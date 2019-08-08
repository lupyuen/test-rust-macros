fname: "start_sensor_listener"
para: "sensor"
para: "sensor_type"
para: "poll_time"
fname: "sensor::set_poll_rate_ms"
sensor has inferred type &Strn
poll_time has inferred type u32
fname: "sensor::mgr_find_next_bydevname"
fname: "sensor::new_sensor_listener"
fname: "sensor::register_listener"
fname: "Ok"
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

/// Testing infer_type
#[cfg(feature = "test_infer_type")]
mod test_infer_type {
    extern crate macros as mynewt_macros;

    extern crate mynewt;
    //use mynewt::{
    //Strn,
    //};

    const _BEGIN: &str =
        "-------------------------------------------------------------";
    const _END: &str =
        "-------------------------------------------------------------";
}

//  use cty::*;               //  Import C types from cty library: https://crates.io/crates/cty

//  Import Mynewt macros from `macros` library

//  Import Mynewt result and error types
//self,             //  Import Mynewt OS functions
//  Import Mynewt OS types
//  Import Mynewt JSON Encoder Context
//self,
//COAP_CONTEXT,
//ToBytesOptionalNull,

//tinycbor,         //  Import Mynewt TinyCBOR API
//mynewt_rust,      //  Import Mynewt Rust Helper API
//sensor_network,   //  Import Mynewt Sensor Network API
//  Import Mynewt Sensor CoAP API
//self,
//sensor_value,

//coap, d, fill_zero,   //  Import Mynewt macros

///////////////////////////////////////////////////////////////////////////////
//  Testing

//#[proc_macros::safe_wrap(attr)] 
//#[proc_macros::safe_wrap(attr)] 
//#[proc_macros::safe_wrap(attr)] ////
//#[proc_macros::safe_wrap(attr)] ////
//#[proc_macros::safe_wrap(attr)] ////
//#[proc_macros::safe_wrap(attr)] ////
//#[proc_macros::safe_wrap(attr)] ////
//#[proc_macros::safe_wrap(attr)] ////
//#[proc_macros::safe_wrap(attr)] ////
//#[proc_macros::safe_wrap(attr)] ////
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
