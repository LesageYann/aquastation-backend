use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use tera::Tera;
use tera::Context;

use crate::api::app_state::ServiceCredential;
use crate::data_sources::users::DataUser;
use crate::model::error::ASError;

#[derive(Deserialize, Debug)]
pub struct RequestResetBody {
    email: String
}

#[derive(Serialize, Debug)]
pub struct RequestResetResult {
    status: String
}

pub async fn request_reset_password_usecase(pool: &PgPool, payload: RequestResetBody, domain: &String, smtp_credidential: &ServiceCredential,  template_engine: &Tera) -> Result<RequestResetResult, ASError> {
    let token = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    let update_result =  DataUser::set_reset_token(pool, &payload.email, &token).await;
    match update_result {
        Some(data) => {
            let email_build= build_email(&data.username, &payload.email, &token, domain, template_engine);

            match email_build {
                Ok(email) => {
                    let creds = Credentials::new(smtp_credidential.user.to_owned(), smtp_credidential.password.to_owned());

                    let mailer = SmtpTransport::relay(&*smtp_credidential.host)
                        .unwrap()
                        .credentials(creds)
                        .build();

                    match mailer.send(&email) {
                        Ok(_) => {
                            println!("Email sent successfully!");
                            Ok(RequestResetResult { status: "Ok".to_string() })
                        },
                        Err(e) => {
                            println!("Could not send emails: {e:?}");
                            Err(ASError::InternalError)
                        },
                    }
                },
                Err(e) => Err(e)
            }

        },
        None => {
            println!("fail to insert token in database");
            Err(ASError::InternalError)
        }
    }
}

fn build_email(username: &String, email: &String, token: &String, domain: &String, template_engine: &Tera) -> Result<Message, ASError> {
    let mut context = Context::new();
    context.insert("username", username);
    context.insert("domain", domain);
    context.insert("token", token);

    match   template_engine.render("emails/partials/reset_password.html", &context){
        Ok(html) => {
            Message::builder()
                .from("Aquastation <aquastion@yann-lesage.fr>".parse().unwrap())
                .reply_to("Aquastation <aquastion@yann-lesage.fr>".parse().unwrap())
                .to(email.parse().unwrap())
                .subject("Reset password")
                .header(ContentType::TEXT_PLAIN)
                .body(html)
                .map_err(|e| {
                    println!("error during building reset password mail {}", e);
                    ASError::InternalError
                })
        }
        Err(e)=> {
            println!("can generate template reset password {}", e);
            Err(ASError::InternalError)
        }
    }

}
