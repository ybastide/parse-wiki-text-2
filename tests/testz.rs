#[cfg(test)]
mod tests {
    use parse_wiki_text_2::{Configuration, FunctionParameter, Node, Output, Parameter};

    #[test]
    fn test_1() {
        let c = Configuration::default();
        let s = "{{a}}";
        let expected = Output {
            nodes: vec![Node::Template {
                start: 0,
                end: 5,
                name: vec![Node::Text {
                    start: 2,
                    end: 3,
                    value: "a",
                }],
                parameters: vec![],
            }],
            warnings: vec![],
        };
        let actual = c.parse(s).unwrap();
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }
    #[test]
    fn test_1_1() {
        let c = Configuration::default();
        let s = "{{a|}}";
        let expected = Output {
            nodes: vec![Node::Template {
                start: 0,
                end: 6,
                name: vec![Node::Text {
                    start: 2,
                    end: 3,
                    value: "a",
                }],
                parameters: vec![Parameter {
                    start: 4,
                    end: 4,
                    name: None,
                    value: vec![],
                }],
            }],
            warnings: vec![],
        };
        let actual = c.parse(s).unwrap();
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }

    #[test]
    fn test_2() {
        let c = Configuration::default();
        let s = "{{a|b}}";
        let expected = Output {
            nodes: vec![Node::Template {
                start: 0,
                end: 7,
                name: vec![Node::Text {
                    start: 2,
                    end: 3,
                    value: "a",
                }],
                parameters: vec![Parameter {
                    start: 4,
                    end: 5,
                    name: None,
                    value: vec![Node::Text {
                        start: 4,
                        end: 5,
                        value: "b",
                    }],
                }],
            }],
            warnings: vec![],
        };
        let actual = c.parse(s).unwrap();
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
    #[test]
    fn test_3() {
        let c = Configuration::default();
        let s = "{{a|b|c}}";
        let expected = Output {
            nodes: vec![Node::Template {
                start: 0,
                end: 9,
                name: vec![Node::Text {
                    start: 2,
                    end: 3,
                    value: "a",
                }],
                parameters: vec![
                    Parameter {
                        start: 4,
                        end: 5,
                        name: None,
                        value: vec![Node::Text {
                            start: 4,
                            end: 5,
                            value: "b",
                        }],
                    },
                    Parameter {
                        start: 6,
                        end: 7,
                        name: None,
                        value: vec![Node::Text {
                            start: 6,
                            end: 7,
                            value: "c",
                        }],
                    },
                ],
            }],
            warnings: vec![],
        };
        let actual = c.parse(s).unwrap();
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
    #[test]
    fn test_4() {
        let c = Configuration::default();
        let s = "{{{{{a}}}a|b|c}}";
        let expected = Output {
            nodes: vec![Node::Template {
                start: 0,
                end: 16,
                name: vec![
                    Node::Parameter {
                        start: 2,
                        end: 6,
                        name: vec![Node::Text {
                            start: 5,
                            end: 6,
                            value: "a",
                        }],
                        default: None,
                    },
                    Node::Text {
                        start: 9,
                        end: 10,
                        value: "a",
                    },
                ],
                parameters: vec![
                    Parameter {
                        start: 11,
                        end: 12,
                        name: None,
                        value: vec![Node::Text {
                            start: 11,
                            end: 12,
                            value: "b",
                        }],
                    },
                    Parameter {
                        start: 13,
                        end: 14,
                        name: None,
                        value: vec![Node::Text {
                            start: 13,
                            end: 14,
                            value: "c",
                        }],
                    },
                ],
            }],
            warnings: vec![],
        };
        let actual = c.parse(s).unwrap();
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }

    #[test]
    fn test_f1() {
        let c = Configuration::default();
        let s = "{{a:}}";
        let expected = Output {
            nodes: vec![Node::Function {
                start: 0,
                end: 6,
                name: vec![Node::Text {
                    start: 2,
                    end: 3,
                    value: "a",
                }],
                parameters: vec![FunctionParameter {
                    start: 4,
                    end: 4,
                    value: vec![],
                }],
            }],
            warnings: vec![],
        };
        let actual = c.parse(s).unwrap();
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }

    #[test]
    fn test_f2() {
        let c = Configuration::default();
        let s = "{{a:b}}";
        let expected = Output {
            nodes: vec![Node::Function {
                start: 0,
                end: 7,
                name: vec![Node::Text {
                    start: 2,
                    end: 3,
                    value: "a",
                }],
                parameters: vec![FunctionParameter {
                    start: 4,
                    end: 5,
                    value: vec![Node::Text {
                        start: 4,
                        end: 5,
                        value: "b",
                    }],
                }],
            }],
            warnings: vec![],
        };
        let actual = c.parse(s).unwrap();
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
    #[test]
    fn test_f3() {
        let c = Configuration::default();
        let s = "{{a:b|c}}";
        let expected = Output {
            nodes: vec![Node::Function {
                start: 0,
                end: 9,
                name: vec![Node::Text {
                    start: 2,
                    end: 3,
                    value: "a",
                }],
                parameters: vec![
                    FunctionParameter {
                        start: 4,
                        end: 5,
                        value: vec![Node::Text {
                            start: 4,
                            end: 5,
                            value: "b",
                        }],
                    },
                    FunctionParameter {
                        start: 6,
                        end: 7,
                        value: vec![Node::Text {
                            start: 6,
                            end: 7,
                            value: "c",
                        }],
                    },
                ],
            }],
            warnings: vec![],
        };
        let actual = c.parse(s).unwrap();
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
    #[test]
    fn test_f4() {
        let c = Configuration::default();
        let s = "{{{{{a}}}a:b|c}}";
        let expected = Output {
            nodes: vec![Node::Function {
                start: 0,
                end: 16,
                name: vec![
                    Node::Parameter {
                        start: 2,
                        end: 6,
                        name: vec![Node::Text {
                            start: 5,
                            end: 6,
                            value: "a",
                        }],
                        default: None,
                    },
                    Node::Text {
                        start: 9,
                        end: 10,
                        value: "a",
                    },
                ],
                parameters: vec![
                    FunctionParameter {
                        start: 11,
                        end: 12,
                        value: vec![Node::Text {
                            start: 11,
                            end: 12,
                            value: "b",
                        }],
                    },
                    FunctionParameter {
                        start: 13,
                        end: 14,
                        value: vec![Node::Text {
                            start: 13,
                            end: 14,
                            value: "c",
                        }],
                    },
                ],
            }],
            warnings: vec![],
        };
        let actual = c.parse(s).unwrap();
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
}
