// rust_email/main.rs

//https://blog.logrocket.com/email-crates-for-rust-lettre-and-imap/

//https://crates.io/crates/lettre
extern crate lettre;
extern crate lettre_email;

//use lettre::{EmailTransport, SmtpTransport};
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

//https://docs.rs/lettre/0.9.2/lettre/all.html
use lettre::smtp::authentication::Credentials;
//para fichero adjuntos
//use std::path::Path;

fn main() {
    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        //.to(("user@example.org", "Firstname Lastname"))
        .to("info@spotawire.com")
        //copia oculta
        .bcc("jgalvarez.jga@gmail.com")
        // ... or by an address only
        .from("info@spotawire.com")
        .subject("Confirmación de formulario/email enviado - Spot a Wire")
        //.text("Hello world. Rust_1")
        .html(" 
            <div>               
                Buenos días,<br><br>
                Gracias por ponerte en contacto con nosotros.<br>
                <a>En breve te contestaremos.</a><br><br>
                <a>Recibe un cordial saludos</a><br>
                <a href='www.spotawire.com'>www.spotawire.com</a>
            </div>               
            ")
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
        println!("Email enviado");
    } else {
        println!("No se puede enviar email: {:?}", result);
    }
    assert!(result.is_ok());    
}