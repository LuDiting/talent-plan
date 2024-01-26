// use std::process::exit;

// use clap::{App, AppSettings, Arg, SubCommand};

// fn main() {
//     let m = App::new(env!("CARGO_PKG_NAME"))
//         .author(env!("CARGO_PKG_AUTHORS"))
//         .version(env!("CARGO_PKG_VERSION"))
//         .about(env!("CARGO_PKG_DESCRIPTION"))
//         .setting(AppSettings::DisableHelpSubcommand)
//         .setting(AppSettings::SubcommandRequiredElseHelp)
//         .subcommands(vec![
//             SubCommand::with_name("set")
//                 .about("Set string key and value")
//                 .args(&[
//                     Arg::with_name("KEY").help("A string key").required(true),
//                     Arg::with_name("VALUE")
//                         .help("A string value")
//                         .required(true),
//                 ]),
//             SubCommand::with_name("get")
//                 .about("Get string value of a key")
//                 .arg(Arg::with_name("KEY").help("A string key").required(true)),
//             SubCommand::with_name("rm")
//                 .about("remove key and value")
//                 .arg(Arg::with_name("KEY").help("A string key").required(true)),
//         ])
//         .get_matches();

//     match m.subcommand() {
//         ("set", Some(_m)) => {
//             eprintln!("unimplemented");
//             exit(1);
//         }
//         ("get", Some(_m)) => {
//             eprintln!("unimplemented");
//             exit(1);
//         }
//         ("rm", Some(_m)) => {
//             eprintln!("unimplemented");
//             exit(1);
//         }
//         _ => unreachable!(),
//     }
// }
