#![feature(trace_macros)]        //  Enable tracing of macros
#![feature(proc_macro_hygiene)]  //  Allow proc macros to be unhygienic
#![feature(custom_attribute)]    //  Allow custom attributes like [safe_wrap]

/// Testing safe_wrap
#[cfg(feature = "test_safe_wrap")]
mod test_safe_wrap {
    use cty::*;               //  Import C types from cty library: https://crates.io/crates/cty

    extern crate macros;
    use macros::{init_strn}; //  Import Mynewt macros from `macros` library

    extern crate mynewt;
    use mynewt::{
        result::*,            //  Import Mynewt result and error types
        Ptr, Out,
        //coap, d, fill_zero,   //  Import Mynewt macros
        kernel::os::{  
            //self,             //  Import Mynewt OS functions
            os_eventq,        //  Import Mynewt OS types
            os_task,          
            os_stack_t,
            os_task_func_t,
            os_time_t,
        },
        hw::sensor::{
            sensor,
        },
        encoding::{
            coap_context::{   //  Import Mynewt JSON Encoder Context
                //self,
                //COAP_CONTEXT,
                //ToBytesOptionalNull,
            },
            //tinycbor,         //  Import Mynewt TinyCBOR API
        },
        libs::{
            //mynewt_rust,      //  Import Mynewt Rust Helper API
            //sensor_network,   //  Import Mynewt Sensor Network API
            sensor_coap::{    //  Import Mynewt Sensor CoAP API
                //self,
                //sensor_value,
            },
        },
    };

    ///////////////////////////////////////////////////////////////////////////////
    //  Testing

    fn test_safe_wrap() -> MynewtResult<()> {
        //#[macros::safe_wrap(attr)] ////
        extern "C" {
            #[doc = " Set the sensor poll rate"]
            #[doc = ""]
            #[doc = " - __`devname`__: Name of the sensor"]
            #[doc = " - __`poll_rate`__: The poll rate in milli seconds"]
            pub fn sensor_set_poll_rate_ms(devname: *const ::cty::c_char, poll_rate: u32) -> ::cty::c_int;
        }
        #[macros::safe_wrap(attr)] ////
        extern "C" {
            #[doc = " Search the sensor list and find the next sensor that corresponds"]
            #[doc = " to a given device name."]
            #[doc = ""]
            #[doc = " - __`devname`__: The device name to search for"]
            #[doc = " - __`sensor`__: The previous sensor found with this device name"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure"]
            pub fn sensor_mgr_find_next_bydevname(
                devname: *const ::cty::c_char,
                prev_cursor: *mut sensor,
            ) -> *mut sensor;
        }
        "-------------------------------------------------------------";
        //#[macros::safe_wrap(attr)] ////
        extern "C" {
            #[doc = " Pull a single item off the event queue and call it's event"]
            #[doc = " callback."]
            #[doc = ""]
            #[doc = " - __`evq`__: The event queue to pull the item off."]
            pub fn os_eventq_run(evq: *mut os_eventq);
        }
        "-------------------------------------------------------------";
        //#[macros::safe_wrap(attr)] ////
        extern "C" {
            #[doc = " Retrieves the default event queue processed by OS main task."]
            #[doc = ""]
            #[doc = " Return:                      The default event queue."]
            pub fn os_eventq_dflt_get() -> *mut os_eventq;
        }
        "-------------------------------------------------------------";
        //#[macros::safe_wrap(attr)] ////
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
        "-------------------------------------------------------------";
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
        "-------------------------------------------------------------";
        let _test_local = init_strn!("hello");
        Ok(())
    }

    /// Represents a null-terminated byte string, suitable for passing to Mynewt APIs as `* const char`
    pub struct Strn {
        /// Byte string terminated with null
        pub bytestr: &'static [u8]
    }

    impl Strn {
        /// Create a new byte string. Fail if the last byte is not zero.
        /// ```
        /// Strn::new(b"network\0")
        /// strn!("network")
        /// ```
        pub fn new(bs: &'static [u8]) -> Strn {
            //  Last byte must be 0.
            assert_eq!(bs.last(), Some(&0u8));
            let res = Strn { bytestr: bs };
            res
        }

        /// Return the byte string as a null-terminated `* const char` C-style string.
        /// Fail if the last byte is not zero.
        pub fn as_cstr(self) -> *const ::cty::c_char {
            //  Last byte must be 0.
            let bs: &'static [u8] = self.bytestr;
            assert_eq!(bs.last(), Some(&0u8));
            bs.as_ptr() as *const ::cty::c_char
        }

        /// Return the byte string.
        /// Fail if the last byte is not zero.
        pub fn as_bytestr(self) -> &'static [u8] {
            //  Last byte must be 0.
            let bs: &'static [u8] = self.bytestr;
            assert_eq!(bs.last(), Some(&0u8));
            &bs
        }

        /// Fail if the last byte is not zero.
        pub fn validate(self) {
            //  Last byte must be 0.
            let bs = &self.bytestr;
            assert_eq!(bs.last(), Some(&0u8));
        }

        /// Fail if the last byte is not zero.
        pub fn validate_bytestr(bs: &'static [u8]) {
            //  Last byte must be 0.
            assert_eq!(bs.last(), Some(&0u8));
        }
    }

    static _test_static: Strn = init_strn!("hello");
}

/// Test CBOR encoding
#[cfg(feature = "test_cbor")]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    /// Run a block of CBOR encoding calls with error checking.
    fn test_run() {
        "-------------------------------------------------------------";
        run!({
            cbor_encode_text_string(
                encoder("Will be transformed", ""),
                key_to_cstr(""),
                cstr_len("")
            );
            encoder("Will NOT be transformed", "");
            cbor_encode_int(
                encoder("Will be transformed", ""),
                0
            );
        });
        "-------------------------------------------------------------";
    }
    fn cbor_encode_text_string(_: i8, _: i8, _: i8) -> i8 { 0 }
    fn cbor_encode_int(_: i8, _: i8) -> i8 { 0 }
    fn encoder(_: &str, _: &str) -> i8 { 0 }
    fn key_to_cstr(_: &str) -> i8 { 0 }
    fn cstr_len(_: &str) -> i8 { 0 }
    struct Context;
    impl Context {
        fn check_error(self, _: i8) {}
    }
    static context: Context = Context {};

    #[test]
    fn test1() {
        //assert_eq!(1, 1);
        test_run();
        test_safe_wrap();
    }
}

/// Test macro hygiene
#[cfg(feature = "test_macro_hygiene")]
mod test_macro_hygiene {
    /// Given an expression `e`, add 88. If identifier `i` and statement block `blk` are specified, add `i` to the result and execute `blk`.
    macro_rules! add_88 {
        // If `e` is an expression like `123`, `i * 2`, `func(i)`...
        ($e:expr) => {
            {
                // Add 88 to the expression and return it.
                $e + 88
            }
        };
        // If `e` is an expression, `i` is an identifier like `myvar`, `blk` is a block of statements...
        ($e:expr, $i:ident, $blk:block) => {
            {
                // Add 88 and the value of `i` to the expression.
                let result = $e + $i + 88;
                // Execute the code block.
                $blk;
                // Return the result.
                result
            }
        }
    }

    ///  Macro to dump all tokens received as a literal string, e.g.
    ///  `d!(a b c)` returns `"a b c"`
    macro_rules! d {
    //  This rule matches zero or more tokens.
    ($($token:tt)*) => {
        //  For all matched tokens, convert into a string.
        stringify!($($token)*)
    };
    }

    macro_rules! parse {
    // Helper macro to parse a JSON `key: value` entry. The entry should be followed by a trailing comma.
    // For example: When parsing the JSON code `{ "device": "010203" , (omitted) }`, the macro will be called like this:
    // ```
    // parse!( @json @object context ["device"] ("010203") , (omitted) )
    // ```
    (@$enc:ident @object $obj:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        d!(adding key: $($key)+ value: $value to object: $obj)

        //  Append to the "values" array e.g.
        //    {"key":"device", "value":"010203"},
        //  $crate::coap_item_str!(@$enc $obj, $($key)+, $value);

        //  Continue expanding the rest of the JSON.
        //  $crate::parse!(@$enc @object $obj () ($($rest)*) ($($rest)*));
    };
    }

    /// Let's make some soup based on a recipe...
    macro_rules! bad_soup {
        // The caller shall pass in a recipe for the soup, 
        // say `salt + 88`
        ($recipe:expr) => {
            {
                // We use our own salt, say Table Salt.
                let salt = 1;
                // We try to use our salt, but the recipe 
                // actually requires a different salt, 
                // like Sea Salt.
                $recipe
                // Rust Compiler fails with a Hygiene Error.
            }
        }
    }

    /// Let's make soup the right way...
    macro_rules! good_soup {
        // The caller shall pass in 2 things: 
        // Which salt to use
        // and the recipe (like `salt + 88`)
        ($salt:ident, $recipe:expr) => {
            {
                // We use whatever salt the caller passes in. 
                // Hence we use `$salt` instead of `salt`.
                let $salt = 1;
                // Then we use the same salt inside the recipe.
                $recipe
                // It works!
            }
        }
    }

    /// Make an Unhygienic soup... 
    fn make_bad_soup() -> Result<i32, ()> {
        let soup = bad_soup!(
            // We try to make soup with our salt and recipe...
            88 + salt  // If you're testing other macros, comment out the `+ salt` part or the code will never compile
            // But the salt isn't the same inside the recipe.  
            // Rust Compiler fails with Hygiene Error:
            // `salt` not found in this scope.
        );
        Ok(soup)  //  Return the cooked soup.
    }

    /// Make a Hygienic soup
    fn make_good_soup() -> Result<i32, ()> {
        let soup = good_soup!(
            // First we tell the macro which salt we're using...
            salt, 
            // Then we tell the macro what to do with that salt.
            88 + salt
            //  It works!
        );
        Ok(soup)  //  Return the cooked soup.
    }

    /// Let's test cooking!
    #[cfg(test)]
    mod tests {
        use super::*;  //  Import everything from outer scope.

        /// Test the simple add_88 macro with 1 parameter.
        #[test]
        fn test_add_88() { 
            assert_eq!(
                89,
                add_88!(1),
            );
        }

        /// Test the extended add_88 macro with 3 parameters.
        #[test]
        fn test_add_88_extended() { 
            let x = 2;
            let y = 3;
            assert_eq!(
                99,
                add_88!(
                    //  Expression
                    x * 4,
                    //  Identifier
                    y,
                    //  Code Block
                    { println!("The values of x and y are {} and {}", x, y); }
                )
            );
            // Shows `The value of x and y are 2 and 3`. Result is 99.
        }

        #[test]
        fn test_parse() { 
            parse! (
                @json @object context ["device"] ("010203") , omitted 
            );
        }

        #[test]
        fn test_bad_soup() { 
            assert_eq!(
                make_bad_soup(),
                Ok(89)
            );
        }

        #[test]
        fn test_good_soup() { 
            assert_eq!(
                make_good_soup(),
                Ok(89)
            );
        }
    }

    /// Let's start cooking!
    fn main() {
        trace_macros!(true);   //  Start tracing macros
        let res = add_88!(1);
        trace_macros!(false);  //  Stop tracing macros
        println!("test_add_88: {}", res);

        parse! (
            @json @object context ["device"] ("010203") , (omitted) 
        );
        println!("bad soup: {}",
            make_bad_soup().unwrap()
        );
        println!("good soup: {}",
            make_good_soup().unwrap()
        );
    }
}

fn main() {    
}