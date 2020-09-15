extern crate serde_derive;
use kube::{api::{Api, ListParams, WatchEvent}, Client};
use futures::{StreamExt, TryStreamExt};
use serde::{Serialize, Deserialize};
use kube_derive::CustomResource;
use kube::config::Config;

#[derive(CustomResource, Serialize, Deserialize, Default, Clone, Debug)]
#[kube(group = "helloworld.apimeister.com", version = "v1", kind="Member", namespaced)]
#[allow(non_snake_case)]
pub struct MemberSpec {
  pub memberOf: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), kube::Error>  {
    println!("starting hello world operator");
    let config = Config::infer().await?;
    let client: kube::Client = Client::new(config);

    let crds: Api<Member> = Api::namespaced(client, "default");
    let lp = ListParams::default();

    println!("subscribing events of type members.helloworld.apimeister.com/v1");
    let mut stream = crds.watch(&lp, "0").await?.boxed();
    while let Some(status) = stream.try_next().await? {
        match status {
            WatchEvent::Added(member) => {
              match member.spec.memberOf {
                None => println!("welcome {}",member.metadata.name.unwrap()),
                Some(member_of) => println!("welcome {} to the team {}"
                          ,member.metadata.name.unwrap()
                          ,member_of),
              }
            },
            WatchEvent::Modified(_member) => {
            },
            WatchEvent::Deleted(member) => {
              println!("sad to see you go {}",member.metadata.name.unwrap());
            },
            WatchEvent::Error(member) => println!("error: {}", member),
            _ => {}
        }
    }
    println!("done");
    Ok(())
}