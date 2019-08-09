fname: "send_sensor_data"
para: "sensor_data"
fname: "sensor_network::get_device_id"
fname: "sensor_network::init_server_post"
macro: ExprMacro {
    attrs: [],
    mac: Macro {
        path: Path {
            leading_colon: None,
            segments: [
                PathSegment {
                    ident: Ident {
                        ident: "coap",
                        span: #0 bytes(2209..2213),
                    },
                    arguments: None,
                },
            ],
        },
        bang_token: Bang,
        delimiter: Paren(
            Paren,
        ),
        tts: TokenStream [
            Punct {
                ch: '@',
                spacing: Alone,
                span: #0 bytes(2216..2217),
            },
            Ident {
                ident: "json",
                span: #0 bytes(2217..2221),
            },
            Group {
                delimiter: Brace,
                stream: TokenStream [
                    Literal { lit: Lit { kind: Str, symbol: device, suffix: None }, span: Span { lo: BytePos(2248), hi: BytePos(2256), ctxt: #0 } },
                    Punct {
                        ch: ':',
                        spacing: Alone,
                        span: #0 bytes(2256..2257),
                    },
                    Punct {
                        ch: '&',
                        spacing: Alone,
                        span: #0 bytes(2258..2259),
                    },
                    Ident {
                        ident: "device_id",
                        span: #0 bytes(2259..2268),
                    },
                    Punct {
                        ch: ',',
                        spacing: Alone,
                        span: #0 bytes(2268..2269),
                    },
                    Ident {
                        ident: "sensor_data",
                        span: #0 bytes(2286..2297),
                    },
                    Punct {
                        ch: ',',
                        spacing: Alone,
                        span: #0 bytes(2297..2298),
                    },
                ],
                span: #0 bytes(2222..2312),
            },
        ],
    },
}
path: "coap"
tts: "@ json { \"device\" : & device_id , sensor_data , }"
fname: "sensor_network::do_server_post"
fname: "Ok"
save_decls: "{\"send_sensor_data\":[[\"sensor_data\",\"_\"]]}"
successfully wrote to test.json
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
#[allow(dead_code)]
mod test_infer_type {
    extern crate macros as mynewt_macros;
    extern crate mynewt;
    use core::ptr::{
                    //null,
                    null_mut};
    use mynewt::{result::*,
                 hw::sensor::{self, sensor_type_t,
                              //sensor_data_func,
                              sensor_data_func_untyped, sensor_listener,
                              sensor_data_ptr, sensor_ptr, sensor_arg,
                              SensorValueType}, encoding::{coap_context::*},
                 libs::{sensor_network}, Strn, fill_zero, coap, d};
    use mynewt_macros::{strn, init_strn};

    /*
    const _: &str = "-------------------------------------------------------------";

    //#[mynewt_macros::infer_type(attr)] 
    fn start_sensor_listener(sensor: _, sensor_type: _, poll_time: _) -> MynewtResult<()> {
        sensor::set_poll_rate_ms(sensor, poll_time) ? ;
        let sensor_object = sensor::mgr_find_next_bydevname(sensor, null_mut()) ? ;
        if sensor_object != null_mut() {
            let listener = new_sensor_listener(sensor_type, handle_sensor_data) ? ;
            sensor::register_listener(sensor_object, listener) ? ;
        }
        Ok(())
    }        

    const _: &str = "-------------------------------------------------------------";

    //#[mynewt_macros::infer_type(attr)] 
    fn handle_sensor_data2(sensor_data: _) -> MynewtResult<()> {
        send_sensor_data(sensor_data) ? ;
        Ok(())
    }

    */
    //const _: &str = "-------------------------------------------------------------";

    fn send_sensor_data(sensor_data: _) -> MynewtResult<()> {
        let device_id = sensor_network::get_device_id()?;
        let network_ready = sensor_network::init_server_post(&DEFAULT_URI)?;
        if network_ready {
            let payload =
                //const _: &str = "-------------------------------------------------------------";

                //  TODO

                //  TODO
                //sl_arg: null_mut(),
                //sl_next: null(),

                //  TODO

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


                {
                    "begin json root";
                    {
                        "begin json coap_root";
                        unsafe {
                            mynewt::libs::sensor_network::prepare_post(mynewt::encoding::APPLICATION_JSON)?;
                        }
                        unsafe {
                            mynewt::libs::sensor_coap::json_rep_start_root_object();
                        }
                        {
                            {
                                "begin json coap_array , object : COAP_CONTEXT , key : values";
                                {
                                    "<< jarri , o: COAP_CONTEXT, k: values";
                                    let key_with_null: &str = "values\u{0}";
                                    unsafe {
                                        mynewt::libs::mynewt_rust::json_helper_set_array(COAP_CONTEXT.to_void_ptr(),
                                                                                         COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()));
                                    };
                                };
                                {
                                    " >>  >> \"device\" >> : & device_id , sensor_data ,";
                                    "add1 key : \"device\" value : $crate::parse!(@ json &device_id) to object :\nCOAP_CONTEXT";
                                    {
                                        "begin json coap_item_str , parent : COAP_CONTEXT , key : \"device\" , val :\n$crate::parse!(@ json &device_id)";
                                        {
                                            "begin json coap_item , array : COAP_CONTEXT";
                                            {
                                                "<< jitmi c: COAP_CONTEXT";
                                                let key_with_null: &str =
                                                    "COAP_CONTEXT\u{0}";
                                                unsafe {
                                                    mynewt::libs::mynewt_rust::json_helper_object_array_start_item(COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                                };
                                            };
                                            {
                                                {
                                                    "-- jtxti o: COAP_CONTEXT, k: key, v: \"device\"";
                                                    let key_strn: &Strn =
                                                        &Strn::new(b"key\x00");
                                                    let value_strn: &Strn =
                                                        &Strn::new(b"device\x00");
                                                    unsafe {
                                                        COAP_CONTEXT.json_set_text_string(key_strn,
                                                                                          value_strn)
                                                    };
                                                };
                                                {
                                                    "-- jtxti o: COAP_CONTEXT, k: value, v: $crate::parse!(@ json &device_id)";
                                                    let key_strn: &Strn =
                                                        &Strn::new(b"value\x00");
                                                    let value_strn: &Strn =
                                                        &device_id;
                                                    unsafe {
                                                        COAP_CONTEXT.json_set_text_string(key_strn,
                                                                                          value_strn)
                                                    };
                                                };
                                            };
                                            {
                                                ">>";
                                                let key_with_null: &str =
                                                    "COAP_CONTEXT\u{0}";
                                                unsafe {
                                                    mynewt::libs::mynewt_rust::json_helper_object_array_end_item(COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                                };
                                            };
                                            "end json coap_item";
                                        };
                                        "end json coap_item_str";
                                    };
                                    "--------------------";
                                    " >>  >> sensor_data >> ,";
                                    "--------------------";
                                    {
                                        "begin json coap_item_int_val , c : COAP_CONTEXT , val : sensor_data";
                                        if let SensorValueType::Uint(val) =
                                               sensor_data.val {
                                            {
                                                "begin json coap_item_int , key : sensor_data.key , value : val";
                                                {
                                                    "begin json coap_item , array : COAP_CONTEXT";
                                                    {
                                                        "<< jitmi c: COAP_CONTEXT";
                                                        let key_with_null:
                                                                &str =
                                                            "COAP_CONTEXT\u{0}";
                                                        unsafe {
                                                            mynewt::libs::mynewt_rust::json_helper_object_array_start_item(COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                                        };
                                                    };
                                                    {
                                                        {
                                                            "-- jtxte o: COAP_CONTEXT, k: \"key\", v: sensor_data.key";
                                                            let key_with_opt_null:
                                                                    &[u8] =
                                                                "key".to_bytes_optional_nul();
                                                            let value_with_opt_null:
                                                                    &[u8] =
                                                                sensor_data.key.to_bytes_optional_nul();
                                                            unsafe {
                                                                mynewt::libs::mynewt_rust::json_helper_set_text_string(COAP_CONTEXT.to_void_ptr(),
                                                                                                                       COAP_CONTEXT.key_to_cstr(key_with_opt_null),
                                                                                                                       COAP_CONTEXT.value_to_cstr(value_with_opt_null))
                                                            };
                                                        };
                                                        {
                                                            "-- jinte o: COAP_CONTEXT, k: \"value\", v: val";
                                                            let key_with_opt_null:
                                                                    &[u8] =
                                                                "value".to_bytes_optional_nul();
                                                            let value =
                                                                val as u64;
                                                            unsafe {
                                                                mynewt::libs::mynewt_rust::json_helper_set_int(COAP_CONTEXT.to_void_ptr(),
                                                                                                               COAP_CONTEXT.key_to_cstr(key_with_opt_null),
                                                                                                               value)
                                                            };
                                                        };
                                                    };
                                                    {
                                                        ">>";
                                                        let key_with_null:
                                                                &str =
                                                            "COAP_CONTEXT\u{0}";
                                                        unsafe {
                                                            mynewt::libs::mynewt_rust::json_helper_object_array_end_item(COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                                        };
                                                    };
                                                    "end json coap_item";
                                                };
                                                "end json coap_item_int";
                                            };
                                        } else {
                                            unsafe {
                                                COAP_CONTEXT.fail(CoapError::VALUE_NOT_UINT)
                                            };
                                        }
                                        "end json coap_item_int_val";
                                    };
                                    "--------------------";
                                };
                                {
                                    ">>";
                                    let key_with_null: &str = "values\u{0}";
                                    unsafe {
                                        mynewt::libs::mynewt_rust::json_helper_close_array(COAP_CONTEXT.to_void_ptr(),
                                                                                           COAP_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                    };
                                };
                                "end json coap_array";
                            };
                        };
                        unsafe {
                            mynewt::libs::sensor_coap::json_rep_end_root_object();
                        }
                        "end json coap_root";
                    };
                    "end json root";
                    ()
                };
            sensor_network::do_server_post()?;
        }
        Ok(())
    }
    const DEFAULT_URI: Strn = Strn{rep: mynewt::StrnRep::ByteStr(b"\x00"),};
    fn new_sensor_listener(sensor_type: sensor_type_t,
                           sensor_func: sensor_data_func_untyped)
     -> MynewtResult<sensor_listener> {
        Ok(sensor_listener{sl_func: Some(sensor_func),
                           sl_sensor_type:
                               sensor_type,
                                              ..unsafe {
                                                    ::core::mem::transmute::<[u8; ::core::mem::size_of::<sensor_listener>()],
                                                                             sensor_listener>([0;
                                                                                                  ::core::mem::size_of::<sensor_listener>()])
                                                }})
    }
    extern "C" fn handle_sensor_data(_sensor: sensor_ptr, _arg: sensor_arg,
                                     _sensor_data: sensor_data_ptr,
                                     _sensor_type: sensor_type_t) -> i32 {
        0
    }
}
fn main() { }
