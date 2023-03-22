use okclai::{OkClai, OpenAIWrapper, Settings};
use reqwest::Client;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "file",
    about = "A command line tool to help you find the right command for the job"
)]
struct CliArgs {
    #[structopt(long, help = "Weather or not to stream the rosponse from OpenAI")]
    no_stream: bool,

    #[structopt(help = "Description of the command you want to find")]
    command_description: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = CliArgs::from_args();

    let client = Client::new();
    let openai_api_key = get_api_key()?;
    let open_ai_wrapper = OpenAIWrapper::new(&openai_api_key, &client);
    let settings = Settings::new(!opt.no_stream);
    let okclai = OkClai::new(open_ai_wrapper, settings);

    let command_descripton = opt.command_description.join(" ");
    okclai.execute(&command_descripton).await?;

    Ok(())
}

fn get_api_key() -> anyhow::Result<String> {
    let key = std::env::var("OPENAI_API_KEY")?;
    Ok(key)
}
