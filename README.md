## Description

This project provides a CLI tool to get AWS ECS task definition secrets from a given task family name which are then output to a `secrets.json` file.

### Motivation

I decided to create this tool because I was limited with my Terrafrom projects where I couldn't get task secrets out of previously created task definition revisions. (more info about this issue can be found [here](https://github.com/hashicorp/terraform-provider-aws/issues/20283)).

## Usage

```
$ ecs-task-secret [OPTIONS] -p <AWS_PROFILE_NAME> -f <TASK_DEFINITION_FAMILY_NAME>
```

AWS account configuration is loaded from AWS Profiles, which are typically stored in `~/.aws/config` and `~/.aws/credentials`. This tool requires to have an AWS profile set up and its name is provided as an argument.

## License

Licensed under [MIT license](http://opensource.org/licenses/MIT).
