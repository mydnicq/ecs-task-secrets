use aws_sdk_ecs::Error;
use aws_types::{region::Region, SdkConfig};
use aws_sdk_ecs::model::Secret as AwsSecret;
use serde::Serialize;
use std::fs::{File, remove_file};
use clap::Parser;
use std::io::Write;
use std::path::Path;

const SECRETS_FILE_NAME: &str = "secrets.json";

#[derive(Debug, Serialize)]
struct Secret {
    pub name: String,
    pub valueFrom: String,
}
#[derive(Debug, Serialize)]
struct Secrets(Vec<Secret>);

impl Secrets {
    fn new() -> Secrets {
        Secrets(Vec::new())
    }

    fn add(&mut self, elem: Secret) {
        self.0.push(elem);
    }
}

impl <'a> FromIterator<&'a AwsSecret> for Secrets {
    fn from_iter<I: IntoIterator<Item=&'a AwsSecret>>(iter: I) -> Self {
        let mut secrets = Secrets::new();
        
        for s in iter{
            secrets.add(Secret {
                name: s.name.to_owned().unwrap(),
                valueFrom: s.value_from.to_owned().unwrap()
            })
        }

        secrets
    }
}

struct GetTaskSecretParams <'a> {
    config: &'a SdkConfig,
    familiy: String
}


async fn get_task_secrets(params: GetTaskSecretParams<'_>) -> Result<(), aws_sdk_ecs::Error> {
    let client = aws_sdk_ecs::Client::new(params.config);

    let td_output = client.describe_task_definition().task_definition(params.familiy).send().await?;

    let task_definition = td_output.task_definition.unwrap();
    let container_definitions = task_definition.container_definitions().unwrap();

    let secrets: Secrets = container_definitions.first().unwrap().secrets().unwrap().into_iter().collect();
    let secrets_json = serde_json::to_string(&secrets).unwrap();

    let mut output = File::create(SECRETS_FILE_NAME).unwrap();
    write!(output, "{}", secrets_json).unwrap();

    println!("{}", "Json File with Task Definition secrets created.");

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// AWS region name (defaults to "us-east-1")
    #[clap(long, value_parser)]
    aws_region_name: Option<String>,

    /// AWS profile name
    #[clap(short = 'p', long, value_parser)]
    aws_profile_name: String,

    /// Task definition name family
    #[clap(short, long, value_parser)]
    family: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let args = Args::parse();

    let region = args.aws_region_name.unwrap_or("us-east-1".into());

    let config = aws_config::from_env()
    .credentials_provider(
        aws_config::profile::ProfileFileCredentialsProvider::builder()
        .profile_name(args.aws_profile_name)
        .build()
    ).region(Region::new(region))
    .load()
    .await;

    if Path::new(SECRETS_FILE_NAME).exists() {
        remove_file(SECRETS_FILE_NAME).expect(format!("Couldn't delete {}", SECRETS_FILE_NAME).as_str());
    }

    let params = GetTaskSecretParams {
        config : &config,
        familiy: args.family
    };
    
    get_task_secrets(params).await
}
