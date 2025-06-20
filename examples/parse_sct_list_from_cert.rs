use std::process::exit;

use openssl::x509::X509;
use sctverify::google_log_list::LogList;
use sctverify::sct;

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 2 {
        eprintln!("Expected 1 argument: chain.pem");
        exit(1);
    }
    let pem_path = args.into_iter().nth(1).unwrap();
    let chain = X509::stack_from_pem(&std::fs::read(pem_path).expect("Unable to read pem"))
        .expect("Unable to parse pem");
    if chain.len() < 2 {
        eprintln!("Expected at least 2 certs.");
        exit(1);
    }

    let sct_list = sct::SignedCertificateTimestamp::from_cert_sct_extension(
        chain[0].as_ref(),
        chain[1].as_ref(),
    )
    .expect("Unable to parse sct list");

    if sct_list.is_empty() {
        println!("Did not found any SCTs in the certificate.");
        exit(0);
    }

    let ll = LogList::get().expect("Unable to fetch log list from Google.");
    for (i, sct) in sct_list.iter().enumerate() {
        println!("SCT {}:", i + 1);

        let log = ll.find_by_id(&sct.log_id);
        if let Some(log) = log {
            println!("  log is {}", log.base_url);
            if let Err(e) =
                sct.verify(&openssl::pkey::PKey::public_key_from_der(&log.pub_key).unwrap())
            {
                println!("  Error: unable to verify SCT signature: {}", e);
            }
        } else {
            println!("  log is not known.");
        }
    }
}
