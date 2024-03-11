

#[derive(serde::Deserialize)]
pub struct Settings{
	pub database: DatabaseSettings,
	pub application_port: u16

}


#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
	pub username: String,
	pub password: String,
	pub port: u16,
	pub host: String,
	pub database_name: String

}



pub fn get_configuration() -> Result<Settings, config::ConfigError> {
	let settings = config::Config::builder()
		.add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
		.build()?;

	settings.try_deserialize::<Settings>()


}













fn main() {
	let configuration = get_configuration().expect("Failed to read configuration.");
	let address = format!("127.0.0.1:{}", configuration.application_port);



    println!("{}", address);
}
