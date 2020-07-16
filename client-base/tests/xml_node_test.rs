use socha_client_base::xml_node::XmlNode;

#[test]
fn test_xml_display() {
    let node = XmlNode::new("a")
        .attribute("b", "c")
        .child(XmlNode::new("d"))
        .build();
    assert_eq!(format!("{}", node), "<a b=\"c\">\n  <d />\n</a>".to_owned());
}
