use super::encording;

enum Section {
    Custom = 0,
    Type = 1,
    Import = 2,
    Func = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
}

enum ValType {
    I32 = 0x7f,
    F32 = 0x7d,
}

enum Opcodes {
    End = 0x0b,
    GetLocal = 0x20,
    F32Add = 0x92,
}

enum ExportType {
    Func = 0x00,
    Tabel = 0x01,
    Mem = 0x02,
    Global = 0x03,
}

const FUNCTION_TYPE: u8 = 0x60;
const EMPTY_ARRAY: u8 = 0x0;
const MAGIV_MODULE_HEADER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
const MODULE_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

pub fn emiter() -> Vec<u8> {
    let mut add_function_type: Vec<u8> = Vec::new();
    add_function_type.push(FUNCTION_TYPE);
    add_function_type.append(&mut encode_vector(&mut vec![
        ValType::F32 as u8,
        ValType::F32 as u8,
    ]));
    add_function_type.append(&mut encode_vector(&mut vec![ValType::F32 as u8]));

    let mut type_section: Vec<u8> =
        create_section(Section::Type, &mut encode_vector(&mut add_function_type));

    let mut function_section = create_section(Section::Func, &mut encode_vector(&mut vec![0x00]));

    let mut e: Vec<u8> = Vec::new();
    e.append(&mut "run".as_bytes().to_vec());
    e.push(ExportType::Func as u8);
    e.push(0x00);
    let mut export_section = create_section(Section::Export, &mut encode_vector(&mut e));

    let mut code: Vec<u8> = Vec::new();
    code.push(Opcodes::GetLocal as u8);
    code.append(&mut encording::unsigned_leb_128(0));
    code.push(Opcodes::GetLocal as u8);
    code.append(&mut encording::unsigned_leb_128(1));
    code.push(Opcodes::F32Add as u8);

    let mut f: Vec<u8> = Vec::new();
    f.push(EMPTY_ARRAY);
    f.append(&mut code);
    f.push(Opcodes::End as u8);
    let mut function_body = encode_vector(&mut f);
    let mut code_section = create_section(Section::Code, &mut encode_vector(&mut function_body));

    let mut format:Vec<u8> = Vec::new();
    format.append(&mut MAGIV_MODULE_HEADER.to_vec());
    format.append(&mut MODULE_VERSION.to_vec());
    format.append(&mut type_section);
    format.append(&mut function_section);
    format.append(&mut export_section);
    format.append(&mut code_section);
    return  format;
}

fn encode_vector(data: &mut Vec<u8>) -> Vec<u8> {
    let mut leb = encording::unsigned_leb_128(data.len() as u8);
    leb.append(data);
    return leb;
}

fn create_section(section_type: Section, data: &mut Vec<u8>) -> Vec<u8> {
    let mut type_section: Vec<u8> = Vec::new();
    type_section.push(section_type as u8);
    type_section.append(&mut encode_vector(data));
    return type_section;
}
