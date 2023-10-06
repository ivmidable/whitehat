pub struct Config {
    pub mode: String,
    pub message: String,
    pub recipient: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let mode = args[1].clone();
        let recipient = args[2].clone();
        let message = args[3].clone();
        Ok(Config {
            mode,
            recipient,
            message,
        })
    }
}
