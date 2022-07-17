use rcgen::generate_simple_self_signed;

fn main() {
    
    let p = pem::Pem {
        contents: vec![1,2,3,1,1,1,1,1],
        tag: String::from("PEM CONTENT!")
    };
    
    
    println!("{}", pem::encode(&p));

    let subject_alt_names = vec![ 
        "work.com".to_string(), 
        "site.bulentcoskun.dev".to_string()
    ];

    let cert = generate_simple_self_signed(subject_alt_names).expect("Something Wrong!");
    println!("{}", cert.serialize_pem().unwrap());
    println!("{}", cert.serialize_private_key_pem());
    println!("{}", cert.serialize_request_pem().unwrap());

}
