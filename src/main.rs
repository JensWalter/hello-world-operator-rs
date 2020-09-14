extern crate serde_derive;
use kube::{api::{Api, ListParams, WatchEvent}, Client};
use futures::{StreamExt, TryStreamExt};
use serde::{Serialize, Deserialize};
use kube_derive::CustomResource;
use kube::config::Config;

#[derive(CustomResource, Serialize, Deserialize, Default, Clone, Debug)]
#[kube(group = "helloworld.apimeister.com", version = "v1", kind="Person", namespaced)]
#[allow(non_snake_case)]
pub struct PersonSpec {
  pub memberOf: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), kube::Error>  {
    println!("starting hello world operator");
    let config = Config::infer().await?;
    let client: kube::Client = Client::new(config);

    let crds: Api<Person> = Api::namespaced(client, "default");
    let lp = ListParams::default();

    println!("subscribing events of type person.helloworld.apimeister.com/v1");
    let mut stream = crds.watch(&lp, "0").await?.boxed();
    while let Some(status) = stream.try_next().await? {
        match status {
            WatchEvent::Added(person) => {
              match person.spec.memberOf {
                None => println!("welcome {}",person.metadata.name.unwrap()),
                Some(member) => println!("welcome {} to the team {}"
                          ,person.metadata.name.unwrap()
                          ,member),
              }
            },
            WatchEvent::Modified(_person) => {
            },
            WatchEvent::Deleted(person) => {
              println!("sad to see you go {}",person.metadata.name.unwrap());
            },
            WatchEvent::Error(person) => println!("error: {}", person),
            _ => {}
        }
    }
    println!("done");
    Ok(())
}