use rust_mail::EmailClientTemplate;

fn main() {
    let template = EmailClientTemplate::starter();

    match template.validate() {
        Ok(()) => println!("rust-mail starter template is ready."),
        Err(_) => {
            eprintln!("template validation failed; check starter configuration");
            std::process::exit(1);
        }
    }
}
