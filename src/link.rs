// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_link_end<'a>(
    state: &mut crate::State<'a>,
    configuration: &crate::Configuration,
    start_position: usize,
    nodes: Vec<crate::Node<'a>>,
    namespace: Option<crate::Namespace>,
    target: &'a str,
    should_reparse: bool,
) {
    if should_reparse {
        println!()
    }
    let inner_end_position =
        state.skip_whitespace_backwards(state.scan_position);
    state.flush(inner_end_position);
    state.scan_position += 2;
    state.flushed_position = state.scan_position;
    let mut text = std::mem::replace(&mut state.nodes, nodes);
    let end = state.scan_position;
    let start = start_position;
    state.nodes.push(match namespace {
        None => {
            let mut trail_end_position = end;
            for character in state.wiki_text[end..].chars() {
                if !configuration.link_trail_character_set.contains(&character)
                {
                    break;
                }
                trail_end_position += character.len_utf8();
            }
            if trail_end_position > end {
                text.push(crate::Node::Text {
                    end: trail_end_position,
                    start: end,
                    value: &state.wiki_text[end..trail_end_position],
                });
            }
            crate::Node::Link {
                end: trail_end_position,
                start,
                target,
                text,
                reparsed: false, // TODO
            }
        }
        Some(crate::Namespace::Category) => crate::Node::Category {
            end,
            ordinal: text,
            start,
            target,
        },
        Some(crate::Namespace::File) => crate::Node::Image {
            end,
            start,
            target,
            text,
        },
    });
}

pub fn parse_link_start(
    state: &mut crate::State,
    configuration: &crate::Configuration,
) {
    if match state.stack.last() {
        Some(crate::OpenNode {
                 type_: crate::OpenNodeType::Link { namespace, .. },
                 ..
             }) => *namespace != Some(crate::Namespace::File),
        _ => false,
    } {
        let open_node = state.stack.pop().unwrap();
        state.warnings.push(crate::Warning {
            end: state.scan_position,
            message: crate::WarningMessage::InvalidLinkSyntax,
            start: open_node.start,
        });
        state.rewind(open_node.nodes, open_node.start);
        return;
    }
    let mut target_end_position;
    let target_start_position =
        state.skip_whitespace_forwards(state.scan_position + 2);
    let namespace = match configuration
        .namespaces
        .find(&state.wiki_text[target_start_position..])
    {
        Err(match_length) => {
            target_end_position = match_length + target_start_position;
            None
        }
        Ok((match_length, namespace)) => {
            target_end_position = match_length + target_start_position;
            Some(namespace)
        }
    };
    let mut n_templates = 0;
    let mut n_parameters = 0;
    let mut should_reparse = false;
    loop {
        match state.get_byte(target_end_position) {
            None | Some(b'\n') => {
                parse_unexpected_end(state, target_end_position);
                break;
            }
            Some(b'[') if n_parameters == 0 && n_templates == 0 => {
                parse_unexpected_end(state, target_end_position);
                break;
            }
            Some(b'{') => if state.get_byte(target_end_position + 1) == Some(b'{') {
                should_reparse = true;
                target_end_position += 2;
                if state.get_byte(target_end_position) == Some(b'{') {
                    target_end_position += 1;
                    n_parameters += 1;
                } else {
                    n_templates += 1;
                }
            } else {
                parse_unexpected_end(state, target_end_position);
                break;
            }
            Some(b'}') => if state.get_byte(target_end_position + 1) == Some(b'}') {
                target_end_position += 2;
                if state.get_byte(target_end_position) == Some(b'}') {
                    target_end_position += 1;
                    n_parameters -= 1;
                    if n_parameters == -1 {
                        parse_unexpected_end(state, target_end_position);
                        break;
                    }
                } else {
                    n_templates -= 1;
                    if n_templates == -1 {
                        parse_unexpected_end(state, target_end_position);
                        break;
                    }
                }
            } else {
                parse_unexpected_end(state, target_end_position);
                break;
            }
            Some(b']') if n_parameters == 0 && n_templates == 0 => {
                parse_end(
                    state,
                    configuration,
                    target_start_position,
                    target_end_position,
                    namespace,
                    should_reparse,
                );
                break;
            }
            Some(b'|') if n_parameters == 0 && n_templates == 0 => {
                if should_reparse {
                    println!()
                }
                state.push_open_node(
                    crate::OpenNodeType::Link {
                        namespace,
                        should_reparse,
                        target: &state.wiki_text
                            [target_start_position..target_end_position],
                    },
                    target_end_position + 1,
                );
                break;
            }
            _ => target_end_position += 1,
        }
    }
}

fn parse_end(
    state: &mut crate::State,
    configuration: &crate::Configuration,
    target_start_position: usize,
    target_end_position: usize,
    namespace: Option<crate::Namespace>,
    should_reparse: bool,
) {
    if state.get_byte(target_end_position + 1) != Some(b']') {
        parse_unexpected_end(state, target_end_position);
        return;
    }
    let start_position = state.scan_position;
    state.flush(start_position);
    let trail_start_position = target_end_position + 2;
    let mut trail_end_position = trail_start_position;
    let value = &state.wiki_text
        [target_start_position..target_end_position];
    match namespace {
        Some(crate::Namespace::Category) => {
            state.nodes.push(crate::Node::Category {
                end: trail_end_position,
                ordinal: vec![],
                start: state.scan_position,
                target: value.trim_end(),
            });
        }
        Some(crate::Namespace::File) => {
            state.nodes.push(crate::Node::Image {
                end: trail_end_position,
                start: state.scan_position,
                target: value.trim_end(),
                text: vec![],
            });
        }
        None => {
            for character in state.wiki_text[trail_start_position..].chars() {
                if !configuration.link_trail_character_set.contains(&character)
                {
                    break;
                }
                trail_end_position += character.len_utf8();
            }

            let target_text = crate::Node::Text {
                end: target_end_position,
                start: target_start_position,
                value,
            };
            let text = if should_reparse {
                let reparsed = crate::parse::parse(
                    configuration,
                    value,
                    std::time::Duration::ZERO,
                );
                if let Ok(reparsed) = reparsed {
                    reparsed.nodes
                } else { vec![target_text] }
            } else {
                if trail_end_position > trail_start_position {
                    vec![
                        target_text,
                        crate::Node::Text {
                            end: trail_end_position,
                            start: trail_start_position,
                            value: &state.wiki_text
                                [trail_start_position..trail_end_position],
                        },
                    ]
                } else {
                    vec![target_text]
                }
            };
            state.nodes.push(crate::Node::Link {
                end: trail_end_position,
                start: state.scan_position,
                target: &state.wiki_text
                    [target_start_position..target_end_position]
                    .trim_end(),
                text,
                reparsed: should_reparse,
            });
        }
    }
    state.flushed_position = trail_end_position;
    state.scan_position = trail_end_position;
}

fn parse_unexpected_end(state: &mut crate::State, target_end_position: usize) {
    state.warnings.push(crate::Warning {
        end: target_end_position,
        message: crate::WarningMessage::InvalidLinkSyntax,
        start: state.scan_position,
    });
    state.scan_position += 1;
}
