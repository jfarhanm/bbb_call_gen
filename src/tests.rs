use crate::*;
use bbb_parser;
#[test]
fn run_parser_tests(){
    let data:String = "Hello I am jfarhanm and this is a message to the future".to_string() ;
    
    // Call 
    let data_slice = data.into_bytes();
    let call_func:Box<[u8]> = generate_call(&data_slice.as_slice(),(20,20));
    let mut parser = bbb_parser::BBBParse::new();
    for m in 1..call_func.as_ref().len(){
        println!("{:#?}",parser.parse(&call_func.as_ref()[..m]));
    }
    
    // reg_caller 
    let name_list = vec![
        String::from("TINKYWINKY"),
        String::from("DIPSY"),
        String::from("LAALAA"),
        String::from("JEFF"),
                        ];
    let reg_caller_func:Box<[u8]> = generate_reg_caller(&name_list.as_slice());
    parser = bbb_parser::BBBParse::new();
    for m in  1..reg_caller_func.len(){
        println!("{:#?}",parser.parse(&reg_caller_func.as_ref()[..m]))
    }


    // reg_caller_ack
    let service_id_list:Vec<(u8,u8)> = vec![(0x10,0x10),(0x20,0x20),(0x30,0x30)];
    let reg_caller_ack_func:Box<[u8]> = generate_reg_caller_ack(0x33,0x33,0x22,0x22,service_id_list);
    parser = bbb_parser::BBBParse::new();
    for m in 1..reg_caller_ack_func.len(){
        println!("{:#?}",parser.parse(&reg_caller_ack_func.as_ref()[..m]))
    }

    // call_resp
    parser = bbb_parser::BBBParse::new();
    let call_resp_func:Box<[u8]> = generate_call_resp(&data_slice.as_slice(),20,20);
    for m in 1..call_resp_func.len(){
        println!("{:#?}",parser.parse(&call_resp_func.as_ref()[..m]))
    }
}

