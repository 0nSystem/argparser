use argparser::{
    arg::Arg,
    parser::{
        Parser,
        get_params,
        validate_params_have_value_and_is_not_exist_required,
        ErrorArgsValidate::FieldNotHasValue,
        ErrorArgsValidate::FieldRequiredNotExist
    }
};


#[test]
fn parser_options_with_get_params(){
    let line_arguments_to_parsed = "-f -param a".to_string();
    let arguments =  vec![
        Arg::new("-f".to_string(), false, false),
        Arg::new("-param".to_string(), true, true)
    ];

    let parser = Parser::new(arguments, " ".to_string());
    
    let map = get_params(&parser,line_arguments_to_parsed);


    assert_eq!(
        map.get_key_value("-f"),
        Some( (&"-f".to_string(), &None) ),
        "Error in value None is Some"
    );
    assert_eq!(
        map.get_key_value("-param"),
        Some( (&"-param".to_string(), &Some("a".to_string())) ),
        "Error not found param "
    );

}

#[test]
fn parser_options_with_validate_if_required_field(){
    let line_arguments_to_parsed = "asdasd asdasdasd".to_string();
    
    let argument_required =  vec![
        Arg::new("-param".to_string(), true, true),
    ];
    let parser = Parser::new(argument_required, " ".to_string());
    let map = get_params(&parser,line_arguments_to_parsed.clone());
    
    let result_with_required = validate_params_have_value_and_is_not_exist_required(&parser, &map);
    match result_with_required {
        Ok(()) => {
            assert!(
                false,
                "Error validate required bad"
            );
        },
        Err(args_error) => {
            match args_error {
                FieldRequiredNotExist(_a) => {
                    assert!(
                        true,
                        "Detect have field and with cant exist"
                    );
                },
                FieldNotHasValue(_a) => {
                    assert!(
                        false,
                        "Detect have cant come this error "
                    );
                },
            }
        },
    }
}

#[test]
fn parser_options_with_validate_if_not_value_and_required(){
    let line_arguments_to_parsed = "-param a".to_string();
    
    let argument_required =  vec![
        Arg::new("-param".to_string(), true, true),
    ];
    let parser = Parser::new(argument_required, " ".to_string());
    let map = get_params(&parser,line_arguments_to_parsed.clone());
    
    let result_with_required = validate_params_have_value_and_is_not_exist_required(&parser, &map);
    match result_with_required {
        Ok(()) => {
            assert!(
                true,
                "Error validate"
            );
        },
        Err(args_error) => {
            match args_error {
                FieldRequiredNotExist(_a) => {
                    assert!(
                        false,
                        "Have value and required valided with not value"
                    );
                },
                FieldNotHasValue(_a) => {
                    assert!(
                        false,
                        "Detect have field and cant exist"
                    );
                },
            }
        },
    }
}