pub fn get_default_config() -> toml::Table {
    toml::toml! {
        root_dir = "$HOME/.local/share/bjl"
        task_db = "$HOME/.local/share/bjl/jotter.db3"
        work_dir = "$HOME/.cache/bjl"
        editor = "nvim"
    }
}
