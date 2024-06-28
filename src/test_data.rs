use crate::parser::{AssertionFailedError, Dto, DtoField, ValueKind};


        pub fn  get_complicated_expected() -> AssertionFailedError<'static> {
            AssertionFailedError {
            expected: ValueKind::Dto(Dto {
                name: "Complicated",
                fields: vec![
                    DtoField {
                        name: "a",
                        value: ValueKind::String("hey"),
                    },
                    DtoField {
                        name: "b",
                        value: ValueKind::String("2"),
                    },
                    DtoField {
                        name: "c",
                        value: ValueKind::String("500"),
                    },
                    DtoField {
                        name: "d",
                        value: ValueKind::String("600"),
                    },
                    DtoField {
                        name: "e",
                        value: ValueKind::Map(vec![ValueKind::Field(Box::new(DtoField {
                            name: "eee",
                            value: ValueKind::Dto(Dto {
                                name: "Complicated",
                                fields: vec![
                                    DtoField {
                                        name: "a",
                                        value: ValueKind::String("a"),
                                    },
                                    DtoField {
                                        name: "b",
                                        value: ValueKind::String("2"),
                                    },
                                    DtoField {
                                        name: "c",
                                        value: ValueKind::String("500"),
                                    },
                                    DtoField {
                                        name: "d",
                                        value: ValueKind::String("600"),
                                    },
                                    DtoField {
                                        name: "e",
                                        value: ValueKind::Map(vec![ValueKind::String("")]),
                                    },
                                    DtoField {
                                        name: "f",
                                        value: ValueKind::Array(vec![ValueKind::String("")]),
                                    },
                                    DtoField {
                                        name: "g",
                                        value: ValueKind::Array(vec![ValueKind::String("")]),
                                    },
                                ],
                            }),
                        }))]),
                    },
                    DtoField {
                        name: "f",
                        value: ValueKind::Array(vec![ValueKind::Dto(Dto {
                            name: "Complicated",
                            fields: vec![
                                DtoField {
                                    name: "a",
                                    value: ValueKind::String("thing"),
                                },
                                DtoField {
                                    name: "b",
                                    value: ValueKind::String("2"),
                                },
                                DtoField {
                                    name: "c",
                                    value: ValueKind::String("500"),
                                },
                                DtoField {
                                    name: "d",
                                    value: ValueKind::String("600"),
                                },
                                DtoField {
                                    name: "e",
                                    value: ValueKind::Map(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "f",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "g",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                            ],
                        })]),
                    },
                    DtoField {
                        name: "g",
                        value: ValueKind::Array(vec![ValueKind::Dto(Dto {
                            name: "Complicated",
                            fields: vec![
                                DtoField {
                                    name: "a",
                                    value: ValueKind::String("hehe"),
                                },
                                DtoField {
                                    name: "b",
                                    value: ValueKind::String("2"),
                                },
                                DtoField {
                                    name: "c",
                                    value: ValueKind::String("500"),
                                },
                                DtoField {
                                    name: "d",
                                    value: ValueKind::String("600"),
                                },
                                DtoField {
                                    name: "e",
                                    value: ValueKind::Map(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "f",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "g",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                            ],
                        })]),
                    },
                ],
            }),
            real: ValueKind::Dto(Dto {
                name: "Complicated",
                fields: vec![
                    DtoField {
                        name: "a",
                        value: ValueKind::String("hey"),
                    },
                    DtoField {
                        name: "b",
                        value: ValueKind::String("2"),
                    },
                    DtoField {
                        name: "c",
                        value: ValueKind::String("500"),
                    },
                    DtoField {
                        name: "d",
                        value: ValueKind::String("600"),
                    },
                    DtoField {
                        name: "e",
                        value: ValueKind::Map(vec![ValueKind::Field(Box::new(DtoField {
                            name: "eee",
                            value: ValueKind::Dto(Dto {
                                name: "Complicated",
                                fields: vec![
                                    DtoField {
                                        name: "a",
                                        value: ValueKind::String("b"),
                                    },
                                    DtoField {
                                        name: "b",
                                        value: ValueKind::String("2"),
                                    },
                                    DtoField {
                                        name: "c",
                                        value: ValueKind::String("500"),
                                    },
                                    DtoField {
                                        name: "d",
                                        value: ValueKind::String("600"),
                                    },
                                    DtoField {
                                        name: "e",
                                        value: ValueKind::Map(vec![ValueKind::String("")]),
                                    },
                                    DtoField {
                                        name: "f",
                                        value: ValueKind::Array(vec![ValueKind::String("")]),
                                    },
                                    DtoField {
                                        name: "g",
                                        value: ValueKind::Array(vec![ValueKind::String("")]),
                                    },
                                ],
                            }),
                        }))]),
                    },
                    DtoField {
                        name: "f",
                        value: ValueKind::Array(vec![ValueKind::Dto(Dto {
                            name: "Complicated",
                            fields: vec![
                                DtoField {
                                    name: "a",
                                    value: ValueKind::String("thing"),
                                },
                                DtoField {
                                    name: "b",
                                    value: ValueKind::String("2"),
                                },
                                DtoField {
                                    name: "c",
                                    value: ValueKind::String("500"),
                                },
                                DtoField {
                                    name: "d",
                                    value: ValueKind::String("600"),
                                },
                                DtoField {
                                    name: "e",
                                    value: ValueKind::Map(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "f",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "g",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                            ],
                        })]),
                    },
                    DtoField {
                        name: "g",
                        value: ValueKind::Array(vec![ValueKind::Dto(Dto {
                            name: "Complicated",
                            fields: vec![
                                DtoField {
                                    name: "a",
                                    value: ValueKind::String("hehe"),
                                },
                                DtoField {
                                    name: "b",
                                    value: ValueKind::String("2"),
                                },
                                DtoField {
                                    name: "c",
                                    value: ValueKind::String("500"),
                                },
                                DtoField {
                                    name: "d",
                                    value: ValueKind::String("600"),
                                },
                                DtoField {
                                    name: "e",
                                    value: ValueKind::Map(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "f",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "g",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                            ],
                        })]),
                    },
                ],
            }),
        }
        }
