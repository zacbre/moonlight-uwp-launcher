use winparsingtools::{
    date_time::{DosDateTime, FileTime},
    file_system::{FileAttributesFlags, FileReference},
    structs::shell_items::{IDList, ShellItem},
    structs::ExtraDataBlock,
    structs::StringData,
    utils::Rot13,
};

#[cfg(test)]
#[test]
fn filetime_test() {
    let a = FileTime::new(132516443881399495);
    assert!(format!("{}", a).eq("2020-12-05T12:19:48Z"));
    println!("{}", a);
}

#[cfg(test)]
#[test]
fn dos_date_time_test() {
    println!("{}\n", DosDateTime::from_u32(2583122118).unwrap());
    println!("{}\n", DosDateTime::from_u32(0).unwrap());
}

#[cfg(test)]
#[test]
fn file_attributes_flags_test() {
    println!("{:?}\n", FileAttributesFlags::from_u32(32));
}

#[cfg(test)]
#[test]
fn extra_data_block_test() {
    let extra_data_block: &[u8] = &[
        0x42, 0x00, 0x09, 0x00, 0x04, 0x00, 0xEF, 0xBE, 0x85, 0x51, 0x79, 0x62, 0x85, 0x51, 0x79,
        0x62, 0x2E, 0x00, 0x00, 0x00, 0x26, 0x74, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0xD2, 0x1B,
        0x01, 0x74, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74, 0x00, 0x2E, 0x00, 0x74, 0x00, 0x78, 0x00,
        0x74, 0x00, 0x00, 0x00, 0x18, 0x00,
    ];
    println!(
        "{:?}\n",
        ExtraDataBlock::from_buffer(&extra_data_block).unwrap()
    );
}

#[cfg(test)]
#[test]
fn file_reference_test() {
    let file_ref: &[u8] = &[0x26, 0x74, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00];
    println!("{:?}\n", FileReference::from_buffer(&file_ref).unwrap());
}

#[cfg(test)]
#[test]
fn shell_items_test() {
    let shell_item_data: &[u8] = &[
        0x5A, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x85, 0x51, 0x79, 0x62, 0x20, 0x00, 0x74,
        0x65, 0x73, 0x74, 0x2E, 0x74, 0x78, 0x74, 0x00, 0x00, 0x42, 0x00, 0x09, 0x00, 0x04, 0x00,
        0xEF, 0xBE, 0x85, 0x51, 0x79, 0x62, 0x85, 0x51, 0x79, 0x62, 0x2E, 0x00, 0x00, 0x00, 0x26,
        0x74, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0xD2, 0x1B, 0x01, 0x74, 0x00, 0x65, 0x00, 0x73,
        0x00, 0x74, 0x00, 0x2E, 0x00, 0x74, 0x00, 0x78, 0x00, 0x74, 0x00, 0x00, 0x00, 0x18, 0x00,
    ];
    println!("{:?}\n", ShellItem::from_buffer(&shell_item_data).unwrap());
}

#[cfg(test)]
#[test]
fn string_data_test() {
    let string_data: &[u8] = &[
        0x0A, 0x00, 0x2E, 0x00, 0x5C, 0x00, 0x74, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74, 0x00, 0x2E,
        0x00, 0x74, 0x00, 0x78, 0x00, 0x74, 0x00,
    ];

    println!("{}\n", StringData::from_buffer(&string_data).unwrap());
}

#[cfg(test)]
#[test]
fn id_list_test() {
    let id_list_data: &[u8] = &[
        0x14, 0x00, 0x1F, 0x50, 0xE0, 0x4F, 0xD0, 0x20, 0xEA, 0x3A, 0x69, 0x10, 0xA2, 0xD8, 0x08,
        0x00, 0x2B, 0x30, 0x30, 0x9D, 0x19, 0x00, 0x2F, 0x43, 0x3A, 0x5C, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x56, 0x00, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x51, 0x77, 0x56, 0x10, 0x00, 0x57,
        0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x00, 0x40, 0x00, 0x09, 0x00, 0x04, 0x00, 0xEF, 0xBE,
        0x73, 0x4E, 0xAC, 0x24, 0x10, 0x51, 0x77, 0x56, 0x2E, 0x00, 0x00, 0x00, 0x1C, 0x0D, 0x06,
        0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xAA, 0x8F, 0xE3, 0x00, 0x57, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x64,
        0x00, 0x6F, 0x00, 0x77, 0x00, 0x73, 0x00, 0x00, 0x00, 0x16, 0x00, 0x4E, 0x00, 0x31, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x10, 0x51, 0x40, 0x58, 0x10, 0x00, 0x54, 0x65, 0x6D, 0x70, 0x00,
        0x00, 0x3A, 0x00, 0x09, 0x00, 0x04, 0x00, 0xEF, 0xBE, 0x73, 0x4E, 0x98, 0x26, 0x10, 0x51,
        0x40, 0x58, 0x2E, 0x00, 0x00, 0x00, 0xF6, 0x1B, 0x06, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0xC3,
        0x85, 0x00, 0x54, 0x00, 0x65, 0x00, 0x6D, 0x00, 0x70, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00,
        0x00,
    ];
    println!("{:?}\n", IDList::from_buffer(&id_list_data).unwrap());
}

#[cfg(test)]
#[test]
fn rot13_test() {
    let res = Rot13::from("Uryyb phgvr :Q");
    println!("{}", res);
}