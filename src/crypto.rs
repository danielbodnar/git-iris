pub fn install_default_crypto_provider() {
    if rustls::crypto::CryptoProvider::get_default().is_none() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .expect("aws-lc-rs crypto provider should install before TLS is used");
    }
}

#[cfg(test)]
mod tests {
    use super::install_default_crypto_provider;

    #[test]
    fn installs_process_crypto_provider() {
        install_default_crypto_provider();

        assert!(rustls::crypto::CryptoProvider::get_default().is_some());
    }
}
