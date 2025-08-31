use rbx_binary::{ InnerError, Serializer, SerializerState };
use rbx_dom_weak::{ WeakDom };
use rbx_types::{ Ref, SharedString };

fn custom_add_instances<'dom, 'db>(
    dom: &'dom WeakDom,
    serializer_state: &mut SerializerState<'dom, 'db, &mut Vec<u8>>,
    refs: &[Ref]
) -> Result<(), InnerError> {
    for referent in refs {
        let instance = dom.get_by_ref(*referent).ok_or(InnerError::InvalidInstanceId {
            referent: *referent,
        })?;

        serializer_state.relevant_instances.push(*referent);
        serializer_state.collect_type_info(instance)?;
    }

    serializer_state.shared_strings.sort_by_key(SharedString::hash);
    for (id, shared_string) in serializer_state.shared_strings.iter().cloned().enumerate() {
        serializer_state.shared_string_ids.insert(shared_string, id as u32);
    }

    Ok(())
}

pub fn serialize_instance_size(dom: &WeakDom, refs: &[Ref]) -> u64 {
    let mut writer = Vec::new();
    let serializer = Serializer::new();
    let mut serializer_state = SerializerState::new(&serializer, dom, &mut writer);

    let result: Result<(), InnerError> = (|| {
        custom_add_instances(dom, &mut serializer_state, refs)?;
        serializer_state.generate_referents();
        serializer_state.serialize_shared_strings()?;
        serializer_state.serialize_instances()?;
        serializer_state.serialize_properties()?;
        Ok(())
    })();

    if result.is_ok() {
        writer.len() as u64
    } else {
        0
    }
}
