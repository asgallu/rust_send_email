extern crate lettre;
extern crate lettre_email;

//use lettre::{EmailTransport, SmtpTransport};
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use lettre::smtp::authentication::Credentials;
//para fichero adjuntos
//use std::path::Path;

fn main() {
    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        .to(("info@spotawire.com", "Info - Spot a Wire"))
        // ... or by an address only
        .from("info@spotawire.com")
        .subject("Hi, Hello world from rust")
        .text("Hello world. Rust_1")
        .build()
        .unwrap();
    let mut mailer = SmtpClient::new_simple("ssl0.ovh.net")
        .unwrap()
        .credentials(Credentials::new("info@spotawire.com".into(), "87%L_qR$2gP".into()))
        //.credentials(Credentials::new("username".into(), "password".into()))
        .transport();
    // Send the email
    let result = mailer.send(email.into());
    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }
    assert!(result.is_ok());
}