use rust_mail::EmailClientTemplate;

fn main() {
    let template = EmailClientTemplate::starter();

    match template.validate() {
        Ok(()) => println!("rust-mail starter template is ready."),
        Err(error) => eprintln!("template validation failed: {error}"),
    }
}
