extern crate web3;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};

fn main() {
    let matches = App::new("mango-admin")
                    .version("1.0")
                    .author("Yen")
                    .usage("Usage: $0 [command]")
                    .setting(AppSettings::SubcommandRequired)
                    .arg(Arg::with_name("repo")
                        .short("R")
                        .long("repo")
                        .value_name("REPO")
                        .help("Repository address")
                        .takes_value(true))
                    subcommand(SubCommand::with_name("status").about("Check status of repository"))
                    subcommand(SubCommand::with_name("status").about("Check status of repository"))
                    subcommand(SubCommand::with_name("status").about("Check status of repository"))
                    subcommand(SubCommand::with_name("status").about("Check status of repository"))
                    subcommand(SubCommand::with_name("status").about("Check status of repository"))
                    .get_matches();
                    /*.arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))*/


   /*  .usage('Usage: $0 [command]')
  .option('R', {
    alias: 'repo',
    describe: 'Repository address',
    type: 'string'
  })
  .option('admin', {
    describe: 'Treat as administrator',
    type: 'boolean'
  })
  .option('account', {
    describe: 'Sender account (a current administrator)',
    type: 'string'
  })
  .global([ 'repo', 'admin' ])
  .command('status', 'Check status of repository')
  .command('create', 'Create repository')
  .command('obsolete', 'Mark repository obsolete')
  .command('authorize <address>', 'Authorize account with write access')
  .command('deauthorize <address>', 'Deauthorize account')
  .strict()
  .version()
  .showHelpOnFail(false, 'Specify --help for available options')
  .help()
  .demand(1, 'Must provide a command')*/

}