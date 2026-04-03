use virt::connect::Connect;
use virt::domain::Domain;

fn main() {
    let conn = match Connect::open(Some("xen:///system")) {
        Ok(c) => c,
        Err(e) => panic!("Unable to connect: {}", e),
    };

    let domains = conn.list_domains().unwrap();

    for domain in &domains {
        println!("Domain: {}", domain);
    }

    let domain = Domain::lookup_by_id(&conn, domains[0]).expect("Unable to lookup domain");

    println!("Domain name: {}", domain.get_name().unwrap());
    println!("Domain ID: {}", domain.get_id().unwrap());

    let xml = domain.get_xml_desc(0).unwrap();
    println!("Domain XML: \n{}", xml);
}
