mod tests {
    #[test]
    fn dummy_test(){
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn port_test(){
        let port = webserver::get_port();
        assert_eq!(port, 7878);
    }
}