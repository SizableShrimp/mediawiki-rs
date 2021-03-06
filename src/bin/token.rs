﻿use mediawiki::Mediawiki;

fn main() {
    let mw = Mediawiki::login_path("ftb.json").unwrap();
    println!("{:?}", mw.get_token::<mediawiki::Csrf>());
    println!("{:?}", mw.get_token::<mediawiki::Watch>());
    println!("{:?}", mw.get_token::<mediawiki::Patrol>());
    println!("{:?}", mw.get_token::<mediawiki::Rollback>());
    println!("{:?}", mw.get_token::<mediawiki::UserRights>());
}
