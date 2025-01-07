
pub struct Config {
    pub project_type: u8,
    pub name: String,
    pub description: String,
    pub major_version: u8,
    pub minor_version: u8,
    pub url: String,
    pub finished: bool,
    pub author: String,
    pub extra_data: String,
}

impl Config {

    pub fn new() -> Config {
        Config{
            project_type: 0,
            name: String::from("unknown"),
            description: String::from(""),
            major_version: 0,
            minor_version: 0,
            url: String::from(""),
            finished: true,
            author: String::from(""),
            extra_data: String::from(""),
        }
    }

    pub fn construct(src: &str) -> Result<Config, String> {
        let mut config = Config::new();
        let mut state = 0;
        let mut lbuffer = String::new();
        let mut rbuffer = String::new();
        let mut extra_data = String::new();

        for c in src.chars() { match state {
            // Take in property name
            0 => match c {
                '=' => {
                    state = 1;
                },
                '\n' => {
                    extra_data.push('\n');
                    extra_data.push_str(&lbuffer);
                    lbuffer = String::new();
                }
                _ => lbuffer.push(c),
            },

            // Take in open quote
            1 => match c {
                '"' => {
                    state = 2;
                    rbuffer = String::new();
                },
                '\n' => {
                    extra_data.push('\n');
                    extra_data.push_str(&lbuffer);
                    extra_data.push('=');
                    lbuffer = String::new();
                },
                _ => {
                    rbuffer.push('=');
                    rbuffer.push(c);
                    state = 10;
                    rbuffer = String::new();
                },
            },


            // Unsure - synchronize to next line and save
            10 => match c {
                '\n' => {
                    extra_data.push('\n');
                    extra_data.push_str(&lbuffer);
                    extra_data.push_str(&rbuffer);
                    lbuffer = String::new();
                    rbuffer = String::new();
                }
                _ => rbuffer.push(c),
            },

            _ => panic!("State machine reached unknown condition"),
        } }

        Ok(config)
    }

}