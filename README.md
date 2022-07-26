# Description

This project provides a CLI tool to get AWS ECS task definition secrets from a given task family name. I decided to create this tool because I was limited with my Terrafrom projects where I couldn't get task secrets out of previously created task definition revisions. (more info about this issue can be found [here](https://github.com/hashicorp/terraform-provider-aws/issues/20283)).
