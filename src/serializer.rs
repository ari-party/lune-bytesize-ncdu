use rbx_binary::{ InnerError, Serializer, SerializerState };
use rbx_dom_weak::{ Instance, WeakDom };
use rbx_types::{ SharedString };

fn custom_add_instances<'dom, 'db>(
    serializer_state: &mut SerializerState<'dom, 'db, &mut Vec<u8>>,
    instances: &[&'dom Instance]
) -> Result<(), InnerError> {
    for instance in instances {
        serializer_state.relevant_instances.push(instance.referent());
        serializer_state.collect_type_info(instance)?;
    }

    serializer_state.shared_strings.sort_by_key(SharedString::hash);
    for (id, shared_string) in serializer_state.shared_strings.iter().cloned().enumerate() {
        serializer_state.shared_string_ids.insert(shared_string, id as u32);
    }

    Ok(())
}

pub fn serialize_instance_size(dom: &WeakDom, instance: &Instance) -> u64 {
    let mut writer = Vec::new();
    let serializer = Serializer::new();
    let mut serializer_state = SerializerState::new(&serializer, dom, &mut writer);

    let result: Result<(), InnerError> = (|| {
        custom_add_instances(&mut serializer_state, &[instance])?;
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
