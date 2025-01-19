/// Pop template, replace it with function, start function parameter.
pub(crate) fn parse_function(state: &mut crate::state::State) {
    match state.stack.pop() {
        Some(crate::OpenNode {
                 type_: crate::OpenNodeType::Template { name, .. },
                 start,
                 ..
             }) => {
            assert!(name.is_none());

            let position =
                crate::state::skip_whitespace_backwards(state.wiki_text, state.scan_position);
            crate::state::flush(
                &mut state.nodes,
                state.flushed_position,
                position,
                state.wiki_text,
            );
            state.flushed_position =
                crate::state::skip_whitespace_forwards(state.wiki_text, state.scan_position + 1);
            state.scan_position = state.flushed_position;

            let name = std::mem::take(&mut state.nodes);

            let parameters = vec![crate::FunctionParameter {
                end: 0,
                start: state.scan_position,
                value: vec![],
            }];

            state.stack.push(crate::OpenNode {
                nodes: vec![],
                start,
                type_: crate::OpenNodeType::Function { name, parameters },
            });
        }
        _ => unreachable!(),
    }
}

/// Same as template parameter without a name
pub(crate) fn parse_function_parameter_separator(state: &mut crate::state::State) {
    match state.stack.last_mut() {
        Some(crate::OpenNode {
                 type_: crate::OpenNodeType::Function { parameters, .. },
                 ..
             }) => {
            let position =
                crate::state::skip_whitespace_backwards(state.wiki_text, state.scan_position);
            crate::state::flush(
                &mut state.nodes,
                state.flushed_position,
                position,
                state.wiki_text,
            );
            state.flushed_position =
                crate::state::skip_whitespace_forwards(state.wiki_text, state.scan_position + 1);
            state.scan_position = state.flushed_position;
            // if name.is_none() {
            //     *name = Some(std::mem::take(&mut state.nodes));
            // } else
            {
                let parameters_length = parameters.len();
                let parameter = &mut parameters[parameters_length - 1];
                parameter.end = position;
                parameter.value = std::mem::take(&mut state.nodes);
            }
            parameters.push(crate::FunctionParameter {
                end: 0,
                start: state.scan_position,
                value: vec![],
            });
        }
        _ => unreachable!(),
    }
}
