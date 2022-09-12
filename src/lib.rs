#[cfg(test)]
mod tests;
use std::boxed::Box;
use bbb_parser;
use bbb_parser::protocol_defs::{self,methods::*};


pub fn generate_call(data:&[u8], index:(u8,u8))->Box<[u8]>{
    let length  = data.len().to_string();
    let mut buf:Vec<u8> = vec![protocol_defs::START,CALL,index.0,index.1,];
    buf.append(length.into_bytes().as_mut());
    buf.push(protocol_defs::CR);
    buf.extend_from_slice(data); 
    buf.push(protocol_defs::CR);
    buf.push(protocol_defs::CR);
    buf.into_boxed_slice()
}

pub fn generate_call_resp(data:&[u8],status:u8,status_code:u8)->Box<[u8]>{ 
    let length  = data.len().to_string();
    let mut buf:Vec<u8> = vec![protocol_defs::START,CALLRESP,status,status_code];
    buf.append(length.into_bytes().as_mut());
    buf.push(protocol_defs::CR);
    buf.extend_from_slice(data); 
    buf.push(protocol_defs::CR);
    buf.push(protocol_defs::CR);
    buf.into_boxed_slice()
}

pub fn generate_reg_caller(names:&[String])->Box<[u8]>{
    let mut buf:Vec<u8> = vec![protocol_defs::START,REG_CALLER];
    for name in names{
        buf.extend_from_slice(name.as_bytes());
        buf.push(protocol_defs::ETX);
    }
    buf.pop();
    buf.push(protocol_defs::CR);
    buf.push(protocol_defs::CR);
    buf.into_boxed_slice()
}


pub fn generate_stop_caller()->Box<[u8]>{
    let buf:Vec<u8> = vec![protocol_defs::START,STOP_CALLER,protocol_defs::CR,protocol_defs::CR];
    buf.into_boxed_slice()
}

pub fn generate_stop_service()->Box<[u8]>{
    let buf:Vec<u8> = vec![protocol_defs::START,STOP_SERVICE,protocol_defs::CR,protocol_defs::CR];
    buf.into_boxed_slice()
}
//                                                                                                                             
pub fn generate_reg_caller_ack(status:u8,status_code:u8,caller_id_lsb:u8,caller_id_msb:u8 , service_id_list:Vec<(u8/*lsb*/,u8/*msb*/)>)->Box<[u8]>{    
    let mut buf:Vec<u8> = vec![protocol_defs::START,REG_CALLER_ACK,status,status_code,caller_id_lsb,caller_id_msb];
    for id in service_id_list{
        buf.push(id.0);
        buf.push(id.1);
    }
    buf.push(0xFF);
    buf.push(0xFF); 
    buf.push(protocol_defs::CR);
    buf.push(protocol_defs::CR);
    buf.into_boxed_slice()
}

