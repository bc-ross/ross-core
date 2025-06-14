import struct

from olefile import OleFileIO


def extract_ole10native_clean(bin_path, output_file=None):
    """
    Extracts the raw payload from an Ole10Native stream.
    Handles extra temp-path headers automatically.
    """
    ole = OleFileIO(bin_path)
    if not ole.exists("\x01Ole10Native"):
        raise ValueError("No Ole10Native stream found!")

    data = ole.openstream("\x01Ole10Native").read()
    offset = 0

    # 1) Ole10Native header
    total_size = struct.unpack("<I", data[offset : offset + 4])[0]
    offset += 4

    unknown1 = struct.unpack("<H", data[offset : offset + 2])[0]
    offset += 2

    def read_cstring():
        nonlocal offset
        end = data.find(b"\x00", offset)
        s = data[offset:end]
        offset = end + 1
        return s

    filename = read_cstring().decode(errors="ignore")
    source_path = read_cstring().decode(errors="ignore")
    temp_path = read_cstring().decode(errors="ignore")

    file_size = struct.unpack("<I", data[offset : offset + 4])[0]
    offset += 4

    print(offset)
    raw_file_data = data[offset:file_size]

    # 2) Robustly strip redundant path header, if present
    def peel_extra_header(data):
        # If it starts with backslash or drive letter, likely extra path
        data_alt = data.lstrip(b"\x00")
        if not (data_alt.startswith(b"\\") or data_alt[1:3] == b":\\"):
            return data
        # Look for double null to end the path block
        double_null = data_alt.find(b"\x00\x000\x00\x00")
        if double_null == -1:
            return data  # fallback: no double null found
        payload = data_alt[double_null + 5 :]
        return payload

    clean_payload = peel_extra_header(raw_file_data)

    if output_file:
        with open(output_file, "wb") as f:
            f.write(clean_payload)
        print(f"✅ Extracted clean payload to: {output_file}")

    return {
        "filename": filename,
        "source_path": source_path,
        "temp_path": temp_path,
        "file_size": file_size,
        "raw_file_data": raw_file_data,
        "clean_payload": clean_payload,
    }


# Example usage:
result = extract_ole10native_clean("temp.db", output_file="test.db")
print("Filename in OLE:", result["filename"])
print("Source path in OLE:", result["source_path"])
print("Temp path in OLE:", result["temp_path"])
print("Raw embedded size:", len(result["clean_payload"]))
