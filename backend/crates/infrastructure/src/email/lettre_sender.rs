use application::error::AppError;

/// Simple email sender interface — lettre integration.
pub struct EmailSender {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub from:      String,
}

impl EmailSender {
    pub async fn send(
        &self,
        to:      &str,
        subject: &str,
        body:    &str,
    ) -> Result<(), AppError> {
        use lettre::{
            message::header::ContentType,
            transport::smtp::authentication::Credentials,
            AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
        };

        let email = Message::builder()
            .from(self.from.parse().map_err(|e: lettre::address::AddressError| AppError::Internal(e.to_string()))?)
            .to(to.parse().map_err(|e: lettre::address::AddressError| AppError::Internal(e.to_string()))?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_host)
            .map_err(|e| AppError::Internal(e.to_string()))?
            .port(self.smtp_port)
            .credentials(Credentials::new(self.smtp_user.clone(), self.smtp_pass.clone()))
            .build();

        transport.send(email).await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }
}
