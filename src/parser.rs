use std::{collections::HashMap};
use crate::arg::Arg;



pub struct Parser{
    args_to_parser : Vec<Arg>,
    separator : String
}

impl Parser{
    pub fn new( args: Vec<Arg>, separator : String ) -> Self {
        Parser { args_to_parser: args, separator :separator}
    }
}


pub fn get_params( 
    parser: &Parser, line_params: String
) -> HashMap<String, Option<String> >
{
    let mut map_arguments = HashMap::new();
    let mut iterator = line_params.split(parser.separator.as_str());

    loop {
        match iterator.next() {
            Some(a) => {
                match parser.args_to_parser.iter().find(|x| x.field.eq(a)) {
                    Some(args_found) => {
                        if args_found.has_value {
                            match iterator.next() {
                                Some(value_arg) => {
                                    map_arguments.insert(args_found.field.clone(), Some(value_arg.to_string()));
                                },
                                None => todo!(),
                            }
                            
                        }else{
                            map_arguments.insert(args_found.field.clone(), None);
                        }  
                    },
                    //not found in Arg params
                    None => {},
                };
                
            },
            None => break,
        }
    }

    map_arguments
}

pub fn validate_params_have_value_and_is_not_exist_required(
    parser: &Parser,
    map_validate_with_args: &HashMap<String, Option<String> >
) -> Result<(),ErrorArgsValidate> {
    
    for args in parser.args_to_parser.iter() {
        match map_validate_with_args.get_key_value(&args.field) {
            Some((key, value)) => {
                if args.has_value && value.is_none() {
                    return Err(
                        ErrorArgsValidate::FieldNotHasValue(
                            format!("Error this field {:?} required value",key)
                        )
                    );
                }
            },
            None => {
                if args.require {
                    return Err(
                        ErrorArgsValidate::FieldRequiredNotExist(
                            format!("Error this {:?} param is obligatory",&args.field.clone())
                        )
                    )
                }
            },
        }
    }

    Ok(())
}




pub enum ErrorArgsValidate{
    FieldRequiredNotExist(String),
    FieldNotHasValue(String)
}