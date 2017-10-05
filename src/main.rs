extern crate web3;
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use web3::futures::Future;

fn main() {
  let matches = App::new("mango-admin")
    .version("1.0")
    .author("Yen")
    .usage("Usage: $0 [command]")
    .setting(AppSettings::SubcommandRequired)
    .arg(
      Arg::with_name("repo")
        .short("R")
        .long("repo")
        .value_name("REPO")
        .help("Repository address")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("admin")
        .long("admin")
        .value_name("ADMIN")
        .help("Administrator")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("account")
        .long("account")
        .value_name("ACCOUNT")
        .help("Sender account (a current administrator)")
        .takes_value(true),
    )
    .subcommand(
      SubCommand::with_name("status").about("Check status of repository"),
    )
    .subcommand(SubCommand::with_name("create").about("Create repository"))
    .subcommand(
      SubCommand::with_name("obsolete").about("Mark repository obsolete"),
    )
    .subcommand(
      SubCommand::with_name("authorize").about("Authorize account with write access"),
    )
    .subcommand(
      SubCommand::with_name("deauthorize").about("Deauthorize account"),
    )
    .get_matches();

  let repo: Vec<&str> = matches.values_of("repo").unwrap().collect();
  let admin: Vec<&str> = matches.values_of("admin").unwrap().collect();
  let account: Vec<&str> = matches.values_of("account").unwrap().collect();

  let (_el, transport) = web3::transports::Ipc::new("./jsonrpc.ipc").unwrap();
  let web3 = web3::Web3::new(transport);

  match matches.subcommand_name() {
    Some("status") => {
      let contract = web3::contract::Contract::new(web3.eth());
    }
    Some("create") => {}
    Some("obsolete") => {}
    Some("authorize") => {}
    Some("deauthorize") => {}
    _ => {}
  }
}
