use bjl;

#[test]
fn test_get_cli_add_correct() {
    let args = vec![
        "app_name".to_string(),
        "add".to_string(),
        "Example name".to_string(),
        "-d".to_string(),
        "Example description".to_string(),
        "-t".to_string(),
        "Tomorrow".to_string(),
    ];

    let uut = bjl::cli::get_command(args);

    assert_eq!(
        uut,
        bjl::command::Command::Add(bjl::command::Add {
            name: "Example name".to_string(),
            date: bjl::utils::DueDate::Tomorrow,
            description: "Example description".to_string(),
        })
    )
}
