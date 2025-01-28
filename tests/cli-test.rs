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

    let uut = bjl::frontend::cli::get_command(args);

    assert_eq!(
        uut,
        Ok(bjl::mw::ui::InputCommand::Add(bjl::mw::task::Task {
            title: "Example name".to_string(),
            due: bjl::utils::DueDate::Tomorrow,
            description: Some("Example description".to_string()),
            id: None,
            status: bjl::utils::Status::Todo,
        }))
    )
}
